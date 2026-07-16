//! 演示文稿视图 —— Impress / PowerPoint 风格（左侧缩略图 + 右侧主画布）。
//!
//! 布局（从外到内）：
//! ┌──────────────────────────────────────────────────────────┐
//! │  工具栏：文件名 · ＋幻灯片 · ＋文本框 · 保存                │
//! ├──────────────┬───────────────────────────────────────────┤
//! │  缩略图导航   │  主画布（16:9 白纸，绝对定位渲染各 shape）   │
//! │  [1]         │  ┌─────────────────────────────────────┐   │
//! │  [2]         │  │  (shape)  (shape)                    │   │
//! │  ...         │  └─────────────────────────────────────┘   │
//! ├──────────────┴───────────────────────────────────────────┤
//! │  状态栏：幻灯片 X / N                                       │
//! └──────────────────────────────────────────────────────────┘
//!
//! 模型层（`model::slide`）已定义：
//!   Presentation { slides: Vec<Slide> }
//!   Slide       { shapes: Vec<Shape>, background: Option<Rgb> }
//!   Shape       { geom: Rect, kind: ShapeKind, style: TextStyle }
//!   ShapeKind   { Text(Vec<Run>) | Image(String) | Vector(String) }
//! 这里只负责渲染与轻量交互（选择、新增幻灯片/文本框），不做富文本编辑。

use gpui::{
    App, ClickEvent, Context, FocusHandle, Focusable, FontWeight, Render, SharedString, Window,
    div, px, rgba,
};
use gpui::prelude::*;

use crate::data;
use crate::model::ser::NativeFormat;
use crate::model::slide::{Presentation, Shape, ShapeKind, Slide};
use crate::model::text::Run;
use crate::model::Model;
use crate::styles::ThemeColors;
use std::path::PathBuf;

// 幻灯片逻辑尺寸（设计单位，shape.geom 以此为坐标系）。16:9。
const SLIDE_W: f32 = 960.0;
const SLIDE_H: f32 = 540.0;
// 主画布显示尺寸（保持 16:9）。
const CANVAS_W: f32 = 760.0;
const CANVAS_H: f32 = SLIDE_H * (CANVAS_W / SLIDE_W);

fn scale() -> f32 {
    CANVAS_W / SLIDE_W
}

/// 演示文稿视图根。
pub struct SlideView {
    name: SharedString,
    /// 背后原生模型（保存时再回填）。
    model: Model,
    path: Option<PathBuf>,
    dirty: bool,

    /// 当前显示的幻灯片索引。
    current: usize,
    /// 当前幻灯片中被选中的形状索引（用于高亮）。
    selected: Option<usize>,

    focus: FocusHandle,
}

impl SlideView {
    /// 默认空白演示文稿模型（含一张带标题占位符的幻灯片）。
    pub fn default_model() -> Model {
        Model::Slide(Presentation {
            slides: vec![Slide {
                shapes: vec![Shape {
                    geom: crate::model::slide::Rect {
                        x: 80.0,
                        y: 200.0,
                        w: 800.0,
                        h: 140.0,
                    },
                    kind: ShapeKind::Text(vec![Run {
                        text: "点击此处编辑标题".to_string(),
                        ..Default::default()
                    }]),
                    style: Default::default(),
                }],
                ..Default::default()
            }],
            ..Default::default()
        })
    }

    #[allow(dead_code)]
    pub fn new_blank(window: &mut Window, cx: &mut Context<Self>, name: SharedString) -> Self {
        Self::build(window, cx, name, Self::default_model(), None)
    }

    pub fn new_from_model(
        window: &mut Window,
        cx: &mut Context<Self>,
        name: SharedString,
        model: Model,
        path: Option<PathBuf>,
    ) -> Self {
        Self::build(window, cx, name, model, path)
    }

    fn build(
        window: &mut Window,
        cx: &mut Context<Self>,
        name: SharedString,
        model: Model,
        path: Option<PathBuf>,
    ) -> Self {
        // 进 current 之前先 clamp，避免越界。
        let _ = window;
        Self {
            name,
            model,
            path,
            dirty: false,
            current: 0,
            selected: None,
            focus: cx.focus_handle(),
        }
    }

    fn presentation(&self) -> &Presentation {
        match &self.model {
            Model::Slide(p) => p,
            _ => unreachable!("SlideView 只承载 Model::Slide"),
        }
    }

    fn presentation_mut(&mut self) -> &mut Presentation {
        match &mut self.model {
            Model::Slide(p) => p,
            _ => unreachable!("SlideView 只承载 Model::Slide"),
        }
    }

    // ── 交互 ──

    fn select_slide(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.presentation().slides.len() {
            self.current = index;
            self.selected = None;
            cx.notify();
        }
    }

    fn select_shape(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected = Some(index);
        cx.notify();
    }

    fn add_slide(&mut self, cx: &mut Context<Self>) {
        let blank = Slide {
            shapes: vec![Shape {
                geom: crate::model::slide::Rect {
                    x: 80.0,
                    y: 200.0,
                    w: 800.0,
                    h: 140.0,
                },
                kind: ShapeKind::Text(vec![Run {
                    text: "新幻灯片".to_string(),
                    ..Default::default()
                }]),
                style: Default::default(),
            }],
            ..Default::default()
        };
        let insert_at = self.current + 1;
        self.presentation_mut().slides.insert(insert_at, blank);
        self.current = insert_at;
        self.selected = None;
        self.dirty = true;
        cx.notify();
    }

    fn add_text_shape(&mut self, cx: &mut Context<Self>) {
        let cur = self.current;
        let idx = {
            let slide = match self.presentation_mut().slides.get_mut(cur) {
                Some(s) => s,
                None => return,
            };
            let n = slide.shapes.len();
            slide.shapes.push(Shape {
                geom: crate::model::slide::Rect {
                    x: 120.0,
                    y: 120.0 + n as f32 * 70.0,
                    w: 600.0,
                    h: 60.0,
                },
                kind: ShapeKind::Text(vec![Run {
                    text: "文本框".to_string(),
                    ..Default::default()
                }]),
                style: Default::default(),
            });
            n
        };
        self.selected = Some(idx);
        self.dirty = true;
        cx.notify();
    }

    fn save_document(&mut self, cx: &mut Context<Self>) {
        let path = self.path.clone().unwrap_or_else(|| {
            let safe = self.name.replace(['/', '\\', ':'], "_");
            data::data_dir().join(format!("{safe}.ewp"))
        });

        if let Err(e) = crate::model::ser::save(&self.model, &path, NativeFormat::Json) {
            eprintln!("[EWP] Failed to save {}: {e}", path.display());
            return;
        }

        self.path = Some(path.clone());
        self.dirty = false;

        let mut app_data = data::load();
        data::add_recent_doc(
            &mut app_data,
            data::RecentDoc {
                name: self.name.to_string(),
                path: path.to_string_lossy().to_string(),
                file_type: data::FileType::PowerPoint,
            },
        );
        cx.notify();
    }
}

impl Focusable for SlideView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for SlideView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();
        let c = ThemeColors::current();
        let pres = self.presentation();
        let total = pres.slides.len();
        let current = self.current.min(total.saturating_sub(1));

        let title = if self.dirty {
            format!("{} *", self.name)
        } else {
            self.name.to_string()
        };

        div()
            .id("slide-root")
            .size_full()
            .flex()
            .flex_col()
            .bg(c.window_bg)
            // ═══ 顶栏 ═══
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(16.))
                    .py(px(6.))
                    .bg(c.sidebar_bg)
                    .border_b_1()
                    .border_color(c.border)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(c.text_muted)
                            .child(SharedString::from(title)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(tool_btn(
                                "slide-add",
                                "＋ 幻灯片",
                                &c,
                                {
                                    let t = this.clone();
                                    move |_, _, cx: &mut App| {
                                        let _ = t.update(cx, |v, cx| v.add_slide(cx));
                                    }
                                },
                            ))
                            .child(tool_btn(
                                "slide-add-text",
                                "＋ 文本框",
                                &c,
                                {
                                    let t = this.clone();
                                    move |_, _, cx: &mut App| {
                                        let _ = t.update(cx, |v, cx| v.add_text_shape(cx));
                                    }
                                },
                            ))
                            .child(
                                div()
                                    .id("slide-save")
                                    .flex()
                                    .items_center()
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .bg(if self.dirty { c.accent } else { c.button_bg })
                                    .text_color(if self.dirty {
                                        rgba(0xffffffff)
                                    } else {
                                        c.text_primary
                                    })
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .child(SharedString::from("保存"))
                                    .on_click({
                                        let t = this.clone();
                                        move |_, _, cx: &mut App| {
                                            let _ = t.update(cx, |v, cx| v.save_document(cx));
                                        }
                                    }),
                            ),
                    ),
            )
            // ═══ 主体：左缩略图 + 右画布 ═══
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    // ── 左：缩略图导航 ──
                    .child(
                        div()
                            .id("slide-thumbs")
                            .flex()
                            .flex_col()
                            .w(px(160.))
                            .flex_shrink_0()
                            .h_full()
                            .bg(c.sidebar_bg)
                            .border_r_1()
                            .border_color(c.border)
                            .overflow_y_scroll()
                            .py(px(10.))
                            .gap_2()
                            .children(pres.slides.iter().enumerate().map(|(i, s)| {
                                let is_active = i == current;
                                let thumb = this.clone();
                                let bg = if is_active { c.accent } else { c.content_bg };
                                let border = if is_active {
                                    c.accent
                                } else {
                                    c.border
                                };
                                div()
                                    .id(SharedString::from(format!("thumb-{i}")))
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_1()
                                    .px(px(8.))
                                    .cursor_pointer()
                                    .child(
                                        div()
                                            .w(px(132.))
                                            .h(px(74.)) // 16:9
                                            .rounded_sm()
                                            .border_1()
                                            .border_color(border)
                                            .bg(bg)
                                            .overflow_hidden()
                                            .child(thumb_shapes(s, &c)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(if is_active {
                                                c.accent
                                            } else {
                                                c.text_muted
                                            })
                                            .child(SharedString::from(format!("{}", i + 1))),
                                    )
                                    .on_click(move |_, _, cx: &mut App| {
                                        let _ = thumb.update(cx, |v, cx| v.select_slide(i, cx));
                                    })
                            })),
                    )
                    // ── 右：主画布 ──
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .min_w_0()
                            .items_center()
                            .justify_center()
                            .bg(c.border)
                            .child(
                                        div()
                                            .relative()
                                            .w(px(CANVAS_W))
                                            .h(px(CANVAS_H))
                                            .rounded_md()
                                            .bg(c.content_bg)
                                            .shadow_md()
                                            .children(
                                                pres
                                                    .slides
                                                    .get(current)
                                                    .map(|slide| {
                                                        slide
                                                            .shapes
                                                            .iter()
                                                            .enumerate()
                                                            .map(|(i, shape)| {
                                                                let selected = self.selected == Some(i);
                                                                let on_sel = this.clone();
                                                                render_shape(
                                                                    i,
                                                                    shape,
                                                                    selected,
                                                                    c.clone(),
                                                                    move |_, _, cx: &mut App| {
                                                                        let _ = on_sel.update(
                                                                            cx,
                                                                            |v, cx| v.select_shape(i, cx),
                                                                        );
                                                                    },
                                                                )
                                                            })
                                                            .collect::<Vec<_>>()
                                                    })
                                                    .unwrap_or_default(),
                                            ),
                            ),
                    ),
            )
            // ═══ 状态栏 ═══
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px(px(16.))
                    .py(px(5.))
                    .bg(c.sidebar_bg)
                    .border_t_1()
                    .border_color(c.border)
                    .text_xs()
                    .text_color(c.text_muted)
                    .child(SharedString::from(format!(
                        "幻灯片 {} / {}",
                        current + 1,
                        total
                    ))),
            )
    }
}

/// 渲染单个 shape（绝对定位到画布坐标系）。
fn render_shape(
    id: usize,
    shape: &Shape,
    selected: bool,
    c: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let s = scale();
    let x = shape.geom.x * s;
    let y = shape.geom.y * s;
    let w = (shape.geom.w * s).max(8.0);
    let h = (shape.geom.h * s).max(8.0);

    let border = if selected { c.accent } else { rgba(0x00000000) };

    div()
        .id(SharedString::from(format!("shape-{id}")))
        .absolute()
        .left(px(x))
        .top(px(y))
        .w(px(w))
        .h(px(h))
        .border_2()
        .border_color(border)
        .rounded_sm()
        .overflow_hidden()
        .cursor_pointer()
        .on_click(on_click)
        .child(match &shape.kind {
            ShapeKind::Text(runs) => {
                let text: String = runs.iter().map(|r| r.text.clone()).collect();
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .items_center()
                    .px(px(4.))
                    .text_color(c.text_primary)
                    .text_sm()
                    .child(SharedString::from(text))
            }
            ShapeKind::Image(path) => {
                div()
                    .w_full()
                    .h_full()
                    .child(SharedString::from(format!("[图片 {}]", path)))
            }
            ShapeKind::Vector(kind) => {
                let is_ellipse = kind.eq_ignore_ascii_case("ellipse");
                div()
                    .w_full()
                    .h_full()
                    .bg(c.accent)
                    .when(is_ellipse, |d| d.rounded_full())
            }
        })
}

/// 缩略图里的迷你 shape 预览（只画色块占位，不渲染文字细节）。
fn thumb_shapes(slide: &Slide, c: &ThemeColors) -> impl IntoElement {
    let s = scale() * (132.0 / CANVAS_W); // 缩放到 132 宽缩略图
    div()
        .relative()
        .size_full()
        .overflow_hidden()
        .children(slide.shapes.iter().enumerate().map(|(i, shape)| {
            let x = shape.geom.x * s;
            let y = shape.geom.y * s;
            let w = (shape.geom.w * s).max(2.0);
            let h = (shape.geom.h * s).max(2.0);
            div()
                .id(SharedString::from(format!("t-shape-{i}")))
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(px(w))
                .h(px(h))
                .rounded_sm()
                .bg(if matches!(shape.kind, ShapeKind::Text(_)) {
                    c.text_muted
                } else {
                    c.accent
                })
        }))
}

/// 顶栏工具按钮。
fn tool_btn(
    id: &'static str,
    label: &'static str,
    c: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .px_3()
        .py_1()
        .rounded_md()
        .bg(c.button_bg)
        .text_color(c.text_primary)
        .text_sm()
        .cursor_pointer()
        .hover(|s| s.bg(c.button_hover_bg))
        .child(SharedString::from(label))
        .on_click(on_click)
}
