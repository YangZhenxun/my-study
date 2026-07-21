//! 编辑器视图 — Pages 风格布局（左侧画布 + 右侧格式侧栏）
//!
//! 布局（从外到内）：
//! ┌─────────────────────────────────────────────────────┐
//! │  [框架顶部栏：文件名 · B I U 对齐 · 保存 · 侧栏开关]   │  ← UiLayoutManager 统一套 chrome
//! ├────────────────────────────┬────────────────────────┤
//! │                            │  格式侧栏               │
//! │  主画布编辑区（白底页面式）  │  [文本 ▾]              │
//! │  行号 + 文本 + 光标         │  样式 | 布局 | 更多     │
//! │                            │                        │
//! │                            │  字体 / 大小 / B I U   │
//! │                            │  对齐 / 颜色 ...        │
//! ├────────────────────────────┴────────────────────────┤
//! │  状态栏：行/列 · 总行数 · 字数 · 脏标记 ●             │
//! └─────────────────────────────────────────────────────┘
//!
//! 顶部 chrome（工具栏）由 `UiLayoutManager` 调度：标准模式 =
//! `StandardToolbar`（复刻原 `top_toolbar` 的框），标签页式 = `TabbedLayout`。
//! 本视图只负责文档内容（body）+ 把 B/I/U/≡ 按钮组塞进 `ChromeCtx::tool_group`，
//! 行为与旧版逐一对齐（见 `docs/system_design.md` §3、§4）。

// mirrors LibreOffice: 本视图对应 `sw::Writer` 的编辑区；顶部 chrome 由
// `sfx2::SfxNotebookBar` 调度的 toolbar 提供（见 `ui/layout.rs` / `ui/standard.rs`）。
use gpui::{
    anchored, deferred, AnyElement, App, ClickEvent, Context, Corner, DefiniteLength, Entity,
    FocusHandle, Focusable, FontWeight, MouseButton, Pixels, Point, Render, Rgba, SharedString,
    Window, div, px, point, rgba,
};
use gpui::prelude::*;
use gpui_component::input::{Input, InputEvent, InputState};
use rust_i18n::t;
use std::rc::Rc;

use crate::data;
use crate::model::ser::NativeFormat;
use crate::model::Model;
use crate::styles::ThemeColors;
use crate::text::model::{Block, Document, Paragraph, Run};
use crate::ui::layout::{ChromeCtx, ModelKind};
use crate::ui::manager::UiLayoutManager;
use std::path::PathBuf;

// ════════════════════════════════════════
// 枚举
// ════════════════════════════════════════

/// 格式侧栏的标签页。
#[derive(Clone, Copy, PartialEq, Eq)]
enum FormatTab {
    Style,
    Layout,
    More,
}

/// 文本对齐方式。
#[derive(Clone, Copy, PartialEq, Eq)]
enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    fn next(self) -> Self {
        match self {
            TextAlign::Left => TextAlign::Center,
            TextAlign::Center => TextAlign::Right,
            TextAlign::Right => TextAlign::Justify,
            TextAlign::Justify => TextAlign::Left,
        }
    }

    /// 返回对齐的 i18n 键。
    #[allow(dead_code)]
    fn label_key(&self) -> &'static str {
        match self {
            TextAlign::Left => "editor.align_left",
            TextAlign::Center => "editor.align_center",
            TextAlign::Right => "editor.align_right",
            TextAlign::Justify => "editor.align_justify",
        }
    }
}

// 字体候选（点击「字体」行循环切换）
const FONT_CHOICES: &[&str] = &["PingFang SC", "Helvetica", "Georgia", "Menlo", "Songti SC"];

// 可选文本颜色数量（不含索引 0 = 跟随主题）
const TEXT_COLOR_COUNT: usize = 4;

/// 返回某颜色索引对应的实际颜色（0 = 跟随主题）。
fn color_at(index: usize, c: &ThemeColors) -> Rgba {
    match index {
        0 => c.text_primary,
        1 => rgba(0xd83931ff),
        2 => rgba(0x0a84ffff),
        3 => rgba(0x34c759ff),
        _ => rgba(0xff9f0aff),
    }
}

// ════════════════════════════════════════
// EditorView
// ════════════════════════════════════════

/// 编辑器根视图 —— Pages 风格布局。
pub struct EditorView {
    /// 文本编辑核心：gpui-component 的多行 `InputState`（自带光标/选区/IME/点击定位/滚动）。
    /// 手写输入层已废弃，全部交给它。
    input_state: Entity<InputState>,
    /// 窗口标题栏显示的名字（如 "Untitled" / 文件名）。
    name: SharedString,
    /// 背后的原生模型（保存时再映射回去，本期未做富文本往返）。
    #[allow(dead_code)]
    model: Model,
    /// 已保存路径；None 表示从未保存过（保存时落到默认 data/ 目录）。
    path: Option<PathBuf>,
    /// 是否有未保存的修改（用于标题星号与状态栏标记）。
    dirty: bool,

    // ── Pages 风格新增状态 ──
    /// 格式侧栏是否展开。
    sidebar_open: bool,
    /// 当前激活的侧栏标签页。
    active_tab: FormatTab,
    /// 字号（UI 显示用，本期未实际应用）。
    font_size: u16,
    bold: bool,
    italic: bool,
    underline: bool,
    alignment: TextAlign,
    /// 字体族（UI 显示用，本期部分应用）。
    font_family: String,
    /// 文本颜色索引（0 = 跟随主题）。
    color_index: usize,
    /// 字体下拉是否展开（悬浮菜单）。
    font_dropdown_open: bool,
    /// 字体下拉锚点（窗口坐标，点击字体行时记录，用于悬浮定位）。
    font_dropdown_anchor: Point<Pixels>,
}

impl EditorView {
    pub fn new_blank(window: &mut Window, cx: &mut Context<Self>, name: SharedString) -> Self {
        Self::build(window, cx, name, Model::Text(Document::default()), None)
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
        let initial_text = extract_lines(&model).join("\n");

        // 用 gpui-component 的多行 InputState 作为文本核心：自带光标、选区、
        // IME 合成、点击定位、软换行与滚动。创建时需要 &mut Window（macOS 输入注册）。
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .soft_wrap(true)
                .placeholder(t!("editor.placeholder"))
        });
        if !initial_text.is_empty() {
            input_state.update(cx, |s, cx| s.set_value(initial_text, window, cx));
        }

        // 文本变化时置脏并刷新（状态栏字数/标题星号）。
        cx.subscribe(&input_state, |this, _state, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.dirty = true;
                cx.notify();
            }
        })
        .detach();

        // 挂载即聚焦，方便直接输入。
        input_state.update(cx, |s, cx| s.focus(window, cx));

        Self {
            input_state,
            name,
            model,
            path,
            dirty: false,

            sidebar_open: true,
            active_tab: FormatTab::Style,
            font_size: 11,
            bold: false,
            italic: false,
            underline: false,
            alignment: TextAlign::Left,
            font_family: "PingFang SC".to_string(),
            color_index: 0,
            font_dropdown_open: false,
            font_dropdown_anchor: point(px(0.), px(0.)),
        }
    }

    /// 切换格式侧栏开/关。
    fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_open = !self.sidebar_open;
        cx.notify();
    }

    /// 切换格式属性（bold/italic/underline/alignment/font/color）。
    fn toggle_format(&mut self, which: &str, cx: &mut Context<Self>) {
        match which {
            "bold" => self.bold = !self.bold,
            "italic" => self.italic = !self.italic,
            "underline" => self.underline = !self.underline,
            "align" => self.alignment = self.alignment.next(),
            "cycle-font" => self.cycle_font(),
            "font-inc" => self.adjust_font_size(1),
            "font-dec" => self.adjust_font_size(-1),
            "cycle-color" => self.cycle_color(),
            _ => {}
        }
        cx.notify();
    }

    /// 循环切换字体族（仅 UI 状态，暂不强加渲染）。
    fn cycle_font(&mut self) {
        let idx = FONT_CHOICES
            .iter()
            .position(|f| *f == self.font_family)
            .unwrap_or(0);
        let next = (idx + 1) % FONT_CHOICES.len();
        self.font_family = FONT_CHOICES[next].to_string();
    }

    /// 调整字号（夹紧 8–72）。
    fn adjust_font_size(&mut self, delta: i16) {
        let next = (self.font_size as i16 + delta).clamp(8, 72);
        self.font_size = next as u16;
    }

    /// 循环切换文本颜色（索引 0 = 跟随主题）。
    fn cycle_color(&mut self) {
        self.color_index = (self.color_index + 1) % (TEXT_COLOR_COUNT + 1);
    }

    /// 展开/收起字体下拉菜单（记录点击位置作为悬浮锚点）。
    fn toggle_font_dropdown(&mut self, anchor: Point<Pixels>, cx: &mut Context<Self>) {
        self.font_dropdown_anchor = anchor;
        self.font_dropdown_open = !self.font_dropdown_open;
        cx.notify();
    }

    /// 从下拉菜单选中字体并收起。
    fn select_font(&mut self, idx: usize, cx: &mut Context<Self>) {
        if let Some(f) = FONT_CHOICES.get(idx) {
            self.font_family = f.to_string();
        }
        self.font_dropdown_open = false;
        cx.notify();
    }

    /// 把 `InputState` 当前文本映射回原生 `Document`（按 `\n` 拆行，每行一个段落）。
    fn to_document(&self, cx: &App) -> Document {
        let text = self.input_state.read(cx).value().to_string();
        let blocks = text
            .split('\n')
            .map(|line| {
                Block::Paragraph(Paragraph {
                    runs: vec![Run {
                        text: line.to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
            })
            .collect();
        Document {
            blocks,
            ..Default::default()
        }
    }

    /// 保存到磁盘。
    fn save_document(&mut self, cx: &mut Context<Self>) {
        let doc = self.to_document(cx);
        let path = self.path.clone().unwrap_or_else(|| {
            let safe = self.name.replace(['/', '\\', ':'], "_");
            data::data_dir().join(format!("{safe}.ewp"))
        });

        if let Err(e) = crate::model::ser::save(&Model::Text(doc), &path, NativeFormat::Json) {
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
                file_type: data::FileType::Document,
            },
        );
        cx.notify();
    }
}

impl Focusable for EditorView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        // 焦点直接路由到内嵌的 InputState，让 gpui-component 的 Input 接管输入。
        self.input_state.read(cx).focus_handle(cx)
    }
}

impl Render for EditorView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();
        let name = self.name.clone();
        let c = ThemeColors::current();

        // 文本统计与光标位置从 InputState 读取（gpui-component 维护真实状态）。
        let state = self.input_state.read(cx);
        let text = state.value();
        let line_count = text.split('\n').count();
        let char_count: usize = text.chars().filter(|ch| *ch != '\n').count();
        let cursor = state.cursor_position();
        let caret_line = cursor.line as usize;
        let caret_col = cursor.character as usize;

        // 格式切换回调（顶栏与侧栏的 B/I/U/对齐按钮共用），以 Rc 形式交给 ChromeCtx。
        let on_format: Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static> = {
            let this = this.clone();
            Rc::new(move |which: &str, _: &ClickEvent, _: &mut Window, cx: &mut App| {
                let this = this.clone();
                let w = which.to_string();
                let _ = this.update(cx, |this, cx| this.toggle_format(&w, cx));
            })
        };

        // 字体下拉：点击触发器展开/收起
        let on_toggle_font_dropdown = {
            let this = this.clone();
            move |event: &ClickEvent, _: &mut Window, cx: &mut App| {
                let pos = event.position();
                let _ = this.update(cx, |this, cx| this.toggle_font_dropdown(pos, cx));
            }
        };
        // 字体下拉：选中某一项
        let on_pick_font = {
            let this = this.clone();
            move |idx: usize, _: &ClickEvent, _: &mut Window, cx: &mut App| {
                let _ = this.update(cx, |this, cx| this.select_font(idx, cx));
            }
        };

        // 顶部 chrome 回调（保存 / 侧栏 / 文档类型切换）。
        let on_save: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> = {
            let this = this.clone();
            Rc::new(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                let this = this.clone();
                let _ = this.update(cx, |this, cx| this.save_document(cx));
            })
        };
        let on_toggle_sidebar: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> = {
            let this = this.clone();
            Rc::new(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                let this = this.clone();
                let _ = this.update(cx, |this, cx| this.toggle_sidebar(cx));
            })
        };
        // 文档类型切换（Tabbed 的 tab 点击触发）：打开对应类型的新窗口（决策⑤）。
        let on_switch_model: Rc<dyn Fn(ModelKind, &mut Window, &mut App) + 'static> = {
            Rc::new(move |kind: ModelKind, _window: &mut Window, cx: &mut App| {
                // mirrors LibreOffice: 切到另一 module（Writer/Calc/Impress）即开对应文档窗口。
                let model = match kind {
                    ModelKind::Text => Model::Text(crate::text::model::Document::default()),
                    ModelKind::Sheet => Model::Sheet(crate::sheet::model::Workbook::default()),
                    ModelKind::Slide => Model::Slide(crate::slide::model::Presentation::default()),
                };
                crate::open_editor(cx, "Untitled".into(), Some(model), None);
            })
        };

        // 中间工具按钮组（B/I/U/≡），交给 StandardToolbar 嵌进统一框。
        let tool_group = format_tools(on_format.clone());

        // ── 文档内容 body（不含顶部栏，由框架套 chrome 后放在下方）──
        // 注意：焦点与键盘全部交给内嵌的 gpui-component Input（它自带 track_focus /
        // on_key_down / IME），根容器不再抢焦点，否则 Input 无法输入。
        let body = div()
            .id("editor-root")
            .flex_1()
            .min_h_0()
            .flex()
            .flex_row()
            .bg(c.window_bg)
            // ── 左区域：画布 + 状态栏 ──
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    // ── 画布外框（灰色背景模拟页面阴影） ──
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .min_h_0()
                            .px(px(32.))
                            .py(px(24.))
                            .bg(c.border) // 浅灰边框色作画布外围
                            // ── 白纸画布：内嵌 gpui-component 多行 Input（自带光标/选区/IME/点击/滚动） ──
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .size_full()
                                    .rounded_md()
                                    .bg(c.content_bg) // 白底页面
                                    .px(px(48.))
                                    .py(px(40.))
                                    .text_color(c.text_primary)
                                    .child(
                                        Input::new(&self.input_state)
                                            .h_full()
                                            .appearance(false), // 去掉 Input 自带边框/底色，融入白纸
                                    ),
                            ),
                    )
                    // ── 状态栏 ──
                    .child(status_bar(
                        caret_line,
                        caret_col,
                        line_count,
                        char_count,
                        self.dirty,
                        &c,
                    )),
            )
            // ═══ 右侧格式侧栏（条件显示） ═══
            .when(self.sidebar_open, |root| {
                root.child(format_sidebar(
                    self.active_tab,
                    self.font_size,
                    self.bold,
                    self.italic,
                    self.underline,
                    self.alignment,
                    &self.font_family,
                    self.color_index,
                    &c,
                    {
                        let f = on_format.clone();
                        move |s: &str, e: &ClickEvent, w: &mut Window, cx: &mut App| f(s, e, w, cx)
                    },
                    on_toggle_font_dropdown,
                    {
                        let this = this.clone();
                        move |tab: FormatTab, _: &ClickEvent, _: &mut Window, cx: &mut App| {
                            let this = this.clone();
                            let _ = this.update(cx, |this, cx| {
                                this.active_tab = tab;
                                cx.notify();
                            });
                        }
                    },
                ))
            })
            // ═══ 字体悬浮下拉（浮在内容上方，不撑开布局） ═══
            .when(self.font_dropdown_open, |root| {
                let c = c.clone();
                let viewport = window.viewport_size();
                let font_family_now = self.font_family.clone();
                let this2 = this.clone();
                // 遮罩：点外部关闭
                let backdrop = deferred(
                    anchored()
                        .position(point(px(0.), px(0.)))
                        .child(
                            div()
                                .id("font-dropdown-backdrop")
                                .w(viewport.width)
                                .h(viewport.height)
                                .bg(rgba(0x00000000))
                                .on_mouse_down(MouseButton::Left, {
                                    let t = this2.clone();
                                    move |_, _, cx: &mut App| {
                                        let _ = t.update(cx, |this, cx| {
                                            this.font_dropdown_open = false;
                                            cx.notify();
                                        });
                                    }
                                }),
                        ),
                );
                // 面板
                let panel = deferred(
                    anchored()
                        .anchor(Corner::TopLeft)
                        .position(self.font_dropdown_anchor)
                        .child(
                            div()
                                .id("font-dropdown-panel")
                                .flex()
                                .flex_col()
                                .min_w(px(180.))
                                .rounded_md()
                                .border_1()
                                .border_color(c.border)
                                .bg(c.content_bg)
                                .overflow_hidden()
                                .children(FONT_CHOICES.iter().enumerate().map(|(i, f)| {
                                    let selected = font_family_now == *f;
                                    let pick = on_pick_font.clone();
                                    div()
                                        .id(SharedString::from(format!("font-opt-{i}")))
                                        .px_3()
                                        .py_1p5()
                                        .text_sm()
                                        .text_color(
                                            if selected {
                                                c.accent
                                            } else {
                                                c.text_primary
                                            },
                                        )
                                        .hover(|s| s.bg(c.button_hover_bg))
                                        .child(SharedString::from(*f))
                                        .on_click(move |e, w, cx| pick(i, e, w, cx))
                                })),
                        ),
                )
                .with_priority(1);
                root.child(backdrop).child(panel)
            })
            .into_any_element();

        // 构造 chrome 上下文并交给布局管理器渲染（标准 / 标签页式 由当前模式决定）。
        let ctx = ChromeCtx {
            model_kind: ModelKind::Text,
            name,
            dirty: self.dirty,
            sidebar_open: self.sidebar_open,
            tool_group,
            on_save,
            on_toggle_sidebar,
            on_format,
            on_switch_model,
        };

        UiLayoutManager::render_chrome(cx, window, ctx, body)
    }
}

// ════════════════════════════════════════
// 组件：顶部工具按钮组（交给框架嵌入统一框）
// ════════════════════════════════════════

/// 中间格式快捷按钮组（B / I / U / ≡），作为 `AnyElement` 交给 `ChromeCtx::tool_group`。
///
/// mirrors LibreOffice: `sfx2::SfxNotebookBar` 中的 Writer 格式 toolbar 按钮组
/// （粗体/斜体/下划线/对齐），由框架画在统一框的中部。
fn format_tools(
    on_format: Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>,
) -> AnyElement {
    let c = ThemeColors::current();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1()
        .child(format_tool_btn("B", "tb-bold", "bold", &c, on_format.clone()))
        .child(format_tool_btn("I", "tb-italic", "italic", &c, on_format.clone()))
        .child(format_tool_btn("U", "tb-underline", "underline", &c, on_format.clone()))
        .child(div().w(px(4.))) // 分隔
        .child(format_tool_btn("≡", "tb-align", "align", &c, on_format.clone()))
        .into_any_element()
}

/// 单个格式快捷按钮（B / I / U / ≡）。
fn format_tool_btn(
    label: &'static str,
    id: &'static str,
    key: &'static str,
    c: &ThemeColors,
    on_format: Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>,
) -> impl IntoElement {
    let on_f = on_format;
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .w(px(26.))
        .h(px(26.))
        .rounded_md()
        .cursor_pointer()
        .hover(|s| s.bg(c.button_hover_bg))
        .text_sm()
        .font_weight(FontWeight::BOLD)
        .text_color(c.text_muted)
        .child(SharedString::from(label))
        .on_click(move |e, w, cx| on_f(key, e, w, cx))
}

// ════════════════════════════════════════
// 组件：格式侧栏
// ════════════════════════════════════════

/// 右侧格式面板（Pages 风格）。
///
/// 固定宽度 ~280 px；包含「文本」头部、标签页切换、以及当前 tab 的内容。
fn format_sidebar(
    active_tab: FormatTab,
    font_size: u16,
    bold: bool,
    italic: bool,
    underline: bool,
    alignment: TextAlign,
    font_family: &str,
    color_index: usize,
    c: &ThemeColors,
    on_format: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
    on_toggle_font_dropdown: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_tab: impl Fn(FormatTab, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w(DefiniteLength::Fraction(0.4))
        .flex_shrink_0()
        .h_full()
        .border_l_1()
        .border_color(c.border)
        .bg(c.sidebar_bg)
        .overflow_hidden()
        // 头部：「文本」+ 样式下拉
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px(px(14.))
                .py(px(10.))
                .border_b_1()
                .border_color(c.border)
                .child(
                    div()
                        .text_xs()
                        .text_color(c.text_muted)
                        .child(SharedString::from(t!("editor.text").to_string())),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_sm()
                        .text_color(c.text_primary)
                        .child(SharedString::from(
                            t!("editor.body_text").to_string(),
                        ))
                        .child(SharedString::from(" ▾")),
                ),
        )
        // 标签页栏：样式 | 布局 | 更多
        .child(sidebar_tab_bar(active_tab, c, on_tab))
        // 标签页内容（按当前 tab 渲染不同内容）
        .child({
            // 各 tab 内容统一为 AnyElement 以便在 match 里选择
            let style = sidebar_style_content(
                font_size, bold, italic, underline,
                alignment, font_family, color_index, c,
                on_format, on_toggle_font_dropdown,
            ).into_any_element();
            let layout = sidebar_layout_content(c).into_any_element();
            let more = sidebar_more_content(c).into_any_element();

            let content = match active_tab {
                FormatTab::Style => style,
                FormatTab::Layout => layout,
                FormatTab::More => more,
            };

            div()
                .id("sidebar-content")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .px(px(14.))
                .py(px(8.))
                .gap_4()
                .child(content)
        })
}

/// 标签页栏（样式 / 布局 / 更多）。
fn sidebar_tab_bar(
    active: FormatTab,
    c: &ThemeColors,
    on_tab: impl Fn(FormatTab, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
) -> impl IntoElement {
    let tabs = [
        (FormatTab::Style, "editor.style"),
        (FormatTab::Layout, "editor.layout"),
        (FormatTab::More, "editor.more"),
    ];

    div()
        .flex()
        .flex_row()
        .px(px(12.))
        .pt(px(4.))
        .border_b_1()
        .border_color(c.border)
        .children(tabs.into_iter().map(move |(tab, key)| {
            let is_active = active == tab;
            let label = t!(key).to_string();
            let bg = if is_active {
                c.accent
            } else {
                rgba(0x00000000)
            };
            let txt_color = if is_active {
                rgba(0xffffffff)
            } else {
                c.text_muted
            };

            let tab_clone = tab;
            let on_tab_clone = on_tab.clone();
            div()
                .id(key)
                .flex()
                .items_center()
                .justify_center()
                .flex_1()
                .py_1p5()
                .mt(px(4.)) // 让活跃 tab 的底部边框覆盖 container border
                .mb(px(-1.))
                .rounded_t_md()
                .cursor_pointer()
                .bg(bg)
                .text_xs()
                .font_weight(if is_active {
                    FontWeight::MEDIUM
                } else {
                    FontWeight::NORMAL
                })
                .text_color(txt_color)
                .child(SharedString::from(label))
                .on_click(move |evt, win, cx| (on_tab_clone)(tab_clone, evt, win, cx))
        }))
}

/// 「样式」标签页内容：字体、大小、B/I/U、对齐、颜色。
fn sidebar_style_content(
    font_size: u16,
    bold: bool,
    italic: bool,
    underline: bool,
    alignment: TextAlign,
    font_family: &str,
    color_index: usize,
    c: &ThemeColors,
    on_format: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
    on_toggle_font_dropdown: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    // 字体选择器行：点击展开悬浮下拉菜单，选中即应用并收起。
    let font_row = div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .id("editor-font-row")
                .cursor_pointer()
                .on_click(on_toggle_font_dropdown)
                .child(sidebar_label_row("editor.font", c))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .flex_1()
                        .text_sm()
                        .text_color(c.text_primary)
                        .child(SharedString::from(font_family.to_string()))
                        .child(SharedString::from("▾")),
                ),
        );

    // 字体大小行（- / 数字 / +，点击步进字号）
    let size_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .child(sidebar_label_row("editor.size", c))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1()
                .child(
                    div()
                        .id("editor-font-dec")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(24.))
                        .h(px(26.))
                        .rounded_md()
                        .border_1()
                        .border_color(c.border)
                        .cursor_pointer()
                        .text_base()
                        .text_color(c.text_primary)
                        .child(SharedString::from("-"))
                        .on_click({
                            let on_f = on_format.clone();
                            move |e, w, cx| on_f("font-dec", e, w, cx)
                        }),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(48.))
                        .h(px(26.))
                        .rounded_md()
                        .border_1()
                        .border_color(c.border)
                        .text_sm()
                        .text_color(c.text_primary)
                        .child(SharedString::from(format!("{}", font_size))),
                )
                .child(
                    div()
                        .id("editor-font-inc")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(24.))
                        .h(px(26.))
                        .rounded_md()
                        .border_1()
                        .border_color(c.border)
                        .cursor_pointer()
                        .text_base()
                        .text_color(c.text_primary)
                        .child(SharedString::from("+"))
                        .on_click({
                            let on_f = on_format.clone();
                            move |e, w, cx| on_f("font-inc", e, w, cx)
                        }),
                ),
        );

    // B I U S 按钮
    let fmt_btn = |label: &'static str, _key: &'static str, active: bool| -> _ {
        let on_f = on_format.clone();
        div()
            .id(_key)
            .flex()
            .items_center()
            .justify_center()
            .w(px(30.))
            .h(px(28.))
            .rounded_md()
            .cursor_pointer()
            .bg(if active { rgba(0x0a84ff18) } else { rgba(0x00000000) })
            .border_1()
            .border_color(if active { c.accent } else { c.border })
            .text_base()
            .font_weight(if active {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            })
            .text_color(if active { c.accent } else { c.text_primary })
            .child(SharedString::from(label))
            .on_click(move |e, w, cx| on_f(_key, e, w, cx))
    };

    let biu_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1p5()
        .child(fmt_btn("B", "bold", bold))
        .child(fmt_btn("I", "italic", italic))
        .child(fmt_btn("U", "underline", underline));

    // 对齐按钮
    let align_labels = [
        ("align-left", "L", TextAlign::Left),
        ("align-center", "C", TextAlign::Center),
        ("align-right", "R", TextAlign::Right),
        ("align-justify", "J", TextAlign::Justify),
    ];

    let align_on = on_format.clone();
    let align_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1p5()
        .children(align_labels.into_iter().map(move |(_key, label, ta)| {
            let active = alignment == ta;
            let align_f = align_on.clone();
            div()
                .id(_key)
                .flex()
                .items_center()
                .justify_center()
                .w(px(30.))
                .h(px(28.))
                .rounded_md()
                .cursor_pointer()
                .bg(if active { rgba(0x0a84ff18) } else { rgba(0x00000000) })
                .border_1()
                .border_color(if active { c.accent } else { c.border })
                .text_xs()
                .text_color(if active { c.accent } else { c.text_primary })
                .child(SharedString::from(label))
                .on_click(move |e, w, cx| align_f("align", e, w, cx))
        }));

    // 文本颜色行（点击循环切换颜色）
    let color_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .child(sidebar_label_row("editor.text_color", c))
        .child(
            div()
                .id("editor-color")
                .flex()
                .items_center()
                .justify_center()
                .w(px(28.))
                .h(px(28.))
                .rounded_md()
                .border_1()
                .border_color(c.border)
                .cursor_pointer()
                .bg(color_at(color_index, c))
                .on_click({
                    let on_f = on_format.clone();
                    move |e, w, cx| on_f("cycle-color", e, w, cx)
                }),
        );

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(font_row)
        .child(size_row)
        .child(biu_row)
        .child(div().h(px(4.))) // 分隔
        .child(align_row)
        .child(color_row)
}

/// 「布局」标签页内容（占位）。
fn sidebar_layout_content(c: &ThemeColors) -> impl IntoElement {
    div()
        .flex()
        .flex_1()
        .items_center()
        .justify_center()
        .text_sm()
        .text_color(c.text_muted)
        .child(SharedString::from(format!(
            "{} …",
            t!("editor.layout").to_string()
        )))
}

/// 「更多」标签页内容（占位）。
fn sidebar_more_content(c: &ThemeColors) -> impl IntoElement {
    div()
        .flex()
        .flex_1()
        .items_center()
        .justify_center()
        .text_sm()
        .text_color(c.text_muted)
        .child(SharedString::from(format!(
            "{} …",
            t!("editor.more").to_string()
        )))
}

/// 侧栏内的小节标签行（如「字体」「文本颜色」）。
fn sidebar_label_row(key: &'static str, c: &ThemeColors) -> impl IntoElement {
    div()
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(c.text_muted)
        .child(SharedString::from(t!(key).to_string()))
}

// ════════════════════════════════════════
// 组件：状态栏
// ════════════════════════════════════════

/// 底部状态栏：当前行列、总行数、字数、未保存标记。
fn status_bar(
    line: usize,
    col: usize,
    total_lines: usize,
    chars: usize,
    dirty: bool,
    c: &ThemeColors,
) -> impl IntoElement {
    let mark = if dirty { "   \u{25CF}" } else { "" }; // ●
    let text = format!(
        "Ln {}, Col {}    {} {}{}",
        line + 1,
        col + 1,
        total_lines,
        chars,
        mark
    );

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
        .child(SharedString::from(text))
}

// ════════════════════════════════════════
// 辅助函数
// ════════════════════════════════════════

/// 把模型里的文本块抽成行（段落/标题各一行，标题加 "# " 前缀）。
fn extract_lines(model: &Model) -> Vec<String> {
    if let Model::Text(doc) = model {
        doc.blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(p) => Some(
                    p.runs.iter().map(|r| r.text.clone()).collect::<String>(),
                ),
                Block::Heading(h) => Some(format!(
                    "# {}",
                    h.runs.iter().map(|r| r.text.clone()).collect::<String>()
                )),
                _ => None,
            })
            .collect()
    } else {
        vec![]
    }
}
