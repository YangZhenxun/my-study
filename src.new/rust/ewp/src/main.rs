mod app_menus;
mod data;
mod ewp_actions;
mod extension;
mod model;
mod settings_view;
mod sheet;
mod slide;
mod styles;
mod text;
mod ui;

use data::AppData;
use ewp_actions::*;
use sheet::view::SheetView;
use slide::view::SlideView;
use text::editor_view::EditorView;
use ui::manager::{load_initial_mode, UiLayoutManager};
use gpui::{
    anchored, deferred, App, Application, AssetSource, Bounds, ClickEvent, Context, Corner,
    DefiniteLength, FontWeight, KeyBinding, MouseButton, MouseDownEvent, PathPromptOptions, Pixels,
    Point, Render, SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions, div, img,
    point, prelude::*, px, rgba, size, svg,
};
use model::Model;
use rust_i18n::t;
use settings_view::SettingsView;
use styles::ThemeColors;
use std::path::PathBuf;

// 编译期加载 locales/ 目录下的 YAML 翻译文件，fallback 到英文
rust_i18n::i18n!("locales", fallback = "en");

// ──────────────────────────────────────────────
// 常量
// ──────────────────────────────────────────────

const ICON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon-256.png");
const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");

// ──────────────────────────────────────────────
// AssetSource
// ──────────────────────────────────────────────

struct FileAssetSource;

impl AssetSource for FileAssetSource {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        let full = format!("{ASSETS_DIR}/{path}");
        match std::fs::read(&full) {
            Ok(bytes) => Ok(Some(std::borrow::Cow::Owned(bytes))),
            Err(_) => Ok(None),
        }
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        let full = format!("{ASSETS_DIR}/{path}");
        let mut entries = Vec::new();
        if let Ok(dir) = std::fs::read_dir(&full) {
            for entry in dir.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    entries.push(name.to_string().into());
                }
            }
        }
        Ok(entries)
    }
}

// ──────────────────────────────────────────────
// 视图
// ──────────────────────────────────────────────

struct Welcome {
    data: AppData,
    /// 当前选中的最近项索引（点击列表项时移动高亮）。
    selected: Option<usize>,
    /// 右键上下文菜单状态：`Some((最近项索引, 锚点位置, 路径))`。
    context_menu: Option<(usize, Point<Pixels>, String)>,
    /// 删除项目确认弹窗状态：`Some((最近项索引, 路径))`。
    confirm_delete: Option<(usize, String)>,
}

impl Render for Welcome {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = ThemeColors::current();
        let viewport = _window.viewport_size();

        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(c.window_bg)
            .overflow_hidden()
            // ═══ 左栏（60%） ═══
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(DefiniteLength::Fraction(0.6))
                    .h_full()
                    .pt(px(36.))
                    .pb_10()
                    .px(px(32.))
                    .items_center()
                    .gap_4()
                    .child(
                        img(PathBuf::from(ICON_PATH))
                            .w(px(96.))
                            .h(px(96.))
                            .rounded_2xl(),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(c.text_primary)
                            .mt(px(-4.))
                            .child(t!("welcome.title").to_string()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(c.text_muted)
                            .child(t!("welcome.version").to_string()),
                    )
                    .child({
                        let this = cx.weak_entity();
                        div()
                            .flex()
                            .flex_col()
                            .w_full()
                            .gap_2p5()
                            .mt_3()
                            .child(action_button(
                                "new",
                                "plus",
                                t!("welcome.create_new_project").to_string(),
                                &c,
                                {
                                    let this = this.clone();
                                    move |_event, _window, cx| create_new_project(Some(this.clone()), cx)
                                },
                            ))
                            .child(action_button(
                                "open",
                                "folder",
                                t!("welcome.open_project").to_string(),
                                &c,
                                {
                                    let this = this.clone();
                                    move |_event, _window, cx| {
                                        let this = this.clone();
                                        open_project(Some(this), cx)
                                    }
                                },
                            ))
                            .child(action_button(
                                "new-presentation",
                                "presentation",
                                t!("welcome.create_presentation").to_string(),
                                &c,
                                {
                                    let this = this.clone();
                                    move |_event, _window, cx| {
                                        create_new_presentation(Some(this.clone()), cx)
                                    }
                                },
                            ))
                            .child(action_button(
                                "new-spreadsheet",
                                "spreadsheet",
                                t!("welcome.create_spreadsheet").to_string(),
                                &c,
                                {
                                    let this = this.clone();
                                    move |_event, _window, cx| {
                                        create_new_spreadsheet(Some(this.clone()), cx)
                                    }
                                },
                            ))
                    }),
            )
            // ═══ 右栏（40%） ═══
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(DefiniteLength::Fraction(0.4))
                    .h_full()
                    .pt(px(28.))
                    .pb(px(20.))
                    .pr(px(24.))
                    .pl(px(16.))
                    .bg(c.sidebar_bg)
                    .child(
                        // 列表容器：填满右栏全部剩余高度，让项目卡片动态分布。
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .gap_2()
                            .children({
                                let this = cx.weak_entity();
                                self.data
                                    .recent_docs
                                    .iter()
                                    .enumerate()
                                    .map(|(i, doc)| {
                                        let path = doc.path.clone();
                                        recent_item(
                                            self.selected == Some(i),
                                            i,
                                            doc,
                                            &c,
                                            {
                                                let this = this.clone();
                                                let path = path.clone();
                                                move |_event, _window, cx| {
                                                    let this = this.clone();
                                                    let path = path.clone();
                                                    open_recent(this, i, path, cx)
                                                }
                                            },
                                            {
                                                let this = this.clone();
                                                let path = path.clone();
                                                move |event: &MouseDownEvent, _window, cx| {
                                                    let this = this.clone();
                                                    let anchor = event.position;
                                                    let path = path.clone();
                                                    let _ = this.update(cx, |this, cx| {
                                                        this.context_menu =
                                                            Some((i, anchor, path));
                                                        cx.notify();
                                                    });
                                                }
                                            },
                                        )
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .when(self.data.recent_docs.is_empty(), |list| {
                                list.child(
                                    div()
                                        .flex()
                                        .flex_1()
                                        .items_center()
                                        .justify_center()
                                        .text_sm()
                                        .text_color(c.text_muted)
                                        .child(t!("welcome.no_recent").to_string()),
                                )
                            }),
                    )
            )
            .when(self.context_menu.is_some(), |d| {
                let (i, anchor, path) = self.context_menu.clone().unwrap();
                let this = cx.weak_entity();
                // 锚点夹在视口内，避免菜单跑到屏幕外（transparent 标题栏下坐标可能偏移）。
                let mut anchor = anchor;
                let menu_w = px(200.);
                let menu_h = px(200.);
                if anchor.x + menu_w > viewport.width {
                    anchor.x = if viewport.width >= menu_w {
                        viewport.width - menu_w
                    } else {
                        px(0.)
                    };
                }
                if anchor.y + menu_h > viewport.height {
                    anchor.y = if viewport.height >= menu_h {
                        viewport.height - menu_h
                    } else {
                        px(0.)
                    };
                }
                // 注意：不要用「全屏透明 backdrop」来关闭菜单——它会和面板争抢命中，
                // 导致点菜单项时实际触发了 backdrop 的关闭 handler，菜单项 on_click 收不到。
                // 改为在面板根用 on_mouse_down_out（点面板外部才关闭），与 gpui-component 的 PopupMenu 同款做法。
                let panel = deferred(
                    anchored()
                        .anchor(Corner::TopLeft)
                        .position(anchor)
                        .child(
                            div()
                                .id("recent-ctx-panel")
                                .on_mouse_down_out({
                                    let t = this.clone();
                                    move |_, _, cx: &mut App| {
                                        let _ = t.update(cx, |this, cx| {
                                            this.context_menu = None;
                                            cx.notify();
                                        });
                                    }
                                })
                                .flex()
                                .flex_col()
                                .min_w(px(180.))
                                .rounded_md()
                                .border_1()
                                .border_color(c.border)
                                .bg(c.content_bg)
                                .overflow_hidden()
                                .child(ctx_menu_item(
                                    "ctx-open",
                                    t!("welcome.context_open").to_string(),
                                    c,
                                    {
                                        let this = this.clone();
                                        let path = path.clone();
                                        move |_, _, cx| {
                                            open_recent(this.clone(), i, path.clone(), cx);
                                            let _ = this.update(cx, |this, cx| {
                                                this.context_menu = None;
                                                cx.notify();
                                            });
                                        }
                                    },
                                ))
                                .child(ctx_menu_item(
                                    "ctx-remove",
                                    t!("welcome.context_remove").to_string(),
                                    c,
                                    {
                                        let this = this.clone();
                                        let path = path.clone();
                                        move |_, _, cx| {
                                            remove_from_recent(this.clone(), path.clone(), cx);
                                        }
                                    },
                                ))
                                .child(ctx_menu_item(
                                    "ctx-delete",
                                    t!("welcome.context_delete").to_string(),
                                    c,
                                    {
                                        let this = this.clone();
                                        let path = path.clone();
                                        move |_, _, cx| {
                                            let _ = this.update(cx, |this, cx| {
                                                this.confirm_delete = Some((i, path.clone()));
                                                this.context_menu = None;
                                                cx.notify();
                                            });
                                        }
                                    },
                                ))
                                .child(div().h(px(1.)).w_full().bg(c.border))
                                .child(ctx_menu_item(
                                    "ctx-new",
                                    t!("welcome.context_new").to_string(),
                                    c,
                                    {
                                        let this = this.clone();
                                        move |_, _, cx| {
                                            create_new_project(Some(this.clone()), cx);
                                            let _ = this.update(cx, |this, cx| {
                                                this.context_menu = None;
                                                cx.notify();
                                            });
                                        }
                                    },
                                ))
                                .child(ctx_menu_item(
                                    "ctx-new-presentation",
                                    t!("welcome.context_new_presentation").to_string(),
                                    c,
                                    {
                                        let this = this.clone();
                                        move |_, _, cx| {
                                            create_new_presentation(Some(this.clone()), cx);
                                            let _ = this.update(cx, |this, cx| {
                                                this.context_menu = None;
                                                cx.notify();
                                            });
                                        }
                                    },
                                ))
                                .child(ctx_menu_item(
                                    "ctx-new-spreadsheet",
                                    t!("welcome.context_new_spreadsheet").to_string(),
                                    c,
                                    {
                                        let this = this.clone();
                                        move |_, _, cx| {
                                            create_new_spreadsheet(Some(this.clone()), cx);
                                            let _ = this.update(cx, |this, cx| {
                                                this.context_menu = None;
                                                cx.notify();
                                            });
                                        }
                                    },
                                ))
                                .child(ctx_menu_item(
                                    "ctx-open-other",
                                    t!("welcome.context_open_other").to_string(),
                                    c,
                                    {
                                        let this = this.clone();
                                        move |_, _, cx| {
                                            open_project(Some(this.clone()), cx);
                                            let _ = this.update(cx, |this, cx| {
                                                this.context_menu = None;
                                                cx.notify();
                                            });
                                        }
                                    },
                                )),
                        ),
                )
                .with_priority(1);
                d.child(panel)
            })
            .when(self.confirm_delete.is_some(), |d| {
                let (_, path) = self.confirm_delete.clone().unwrap();
                let this = cx.weak_entity();
                let panel_w = px(340.);
                let backdrop = deferred(
                    anchored()
                        .position(point(px(0.), px(0.)))
                        .child(
                            div()
                                .id("confirm-delete-backdrop")
                                .w(viewport.width)
                                .h(viewport.height)
                                .bg(rgba(0x000000aa))
                                .flex()
                                .items_center()
                                .justify_center()
                                .on_mouse_down(MouseButton::Left, {
                                    let t = this.clone();
                                    move |_, _, cx: &mut App| {
                                        let _ = t.update(cx, |this, cx| {
                                            this.confirm_delete = None;
                                            cx.notify();
                                        });
                                    }
                                })
                                .child(
                                    div()
                                        .id("confirm-delete-panel")
                                        .flex()
                                        .flex_col()
                                        .w(panel_w)
                                        .rounded_xl()
                                        .border_1()
                                        .border_color(c.border)
                                        .bg(c.content_bg)
                                .overflow_hidden()
                                .on_mouse_down(MouseButton::Left, |_, _, _| {})
                                        .child(
                                            div()
                                                .px_5()
                                                .pt_5()
                                                .pb_3()
                                                .text_base()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(c.text_primary)
                                                .child(t!("welcome.confirm_delete_title").to_string()),
                                        )
                                        .child(
                                            div()
                                                .px_5()
                                                .pb_3()
                                                .text_sm()
                                                .text_color(c.text_muted)
                                                .child(format!(
                                                    "{} {}",
                                                    t!("welcome.confirm_delete_message").to_string(),
                                                    path.clone()
                                                )),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .justify_end()
                                                .gap_2()
                                                .px_5()
                                                .pb_4()
                                                .pt_1()
                                                .child(confirm_button(
                                                    "confirm-cancel",
                                                    t!("welcome.cancel").to_string(),
                                                    c,
                                                    false,
                                                    {
                                                        let t = this.clone();
                                                        move |_, _, cx| {
                                                            let _ = t.update(cx, |this, cx| {
                                                                this.confirm_delete = None;
                                                                cx.notify();
                                                            });
                                                        }
                                                    },
                                                ))
                                                .child(confirm_button(
                                                    "confirm-delete",
                                                    t!("welcome.delete").to_string(),
                                                    c,
                                                    true,
                                                    {
                                                        let t = this.clone();
                                                        let path = path.clone();
                                                        move |_, _, cx| {
                                                            delete_project(t.clone(), path.clone(), cx);
                                                        }
                                                    },
                                                )),
                                        ),
                                ),
                        ),
                );
                d.child(backdrop)
            })
    }
}

// ──────────────────────────────────────────────
// 辅助组件
// ──────────────────────────────────────────────

fn icon_path(name: &str) -> String {
    format!("icons/{name}.svg")
}

fn action_button(
    id: &'static str,
    icon_name: &'static str,
    label: String,
    c: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .gap_3()
        .px_4()
        .py_2p5()
        .rounded_lg()
        .bg(c.button_bg)
        .cursor_pointer()
        .text_base()
        .text_color(c.text_primary)
        .hover(|s| s.bg(c.button_hover_bg))
        .child(
            svg()
                .path(icon_path(icon_name))
                .w(px(18.))
                .h(px(18.))
                .text_color(c.text_muted),
        )
        .child(div().flex_1().child(SharedString::from(label)))
        .on_click(on_click)
}


fn recent_item(
    is_selected: bool,
    id: usize,
    doc: &data::RecentDoc,
    c: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_context: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let bg = if is_selected {
        c.selected_bg
    } else {
        rgba(0x00000000)
    };
    let name_color = if is_selected {
        c.selected_text
    } else {
        c.text_primary
    };
    let path_color = c.text_muted;
    let icon_color = if is_selected {
        c.selected_text
    } else {
        c.accent
    };

    div()
        .id(id)
        .flex()
        .flex_row()
        .items_start()
        .gap_3()
        .px_3()
        .py_2p5()
        .rounded_md()
        .cursor_pointer()
        .bg(bg)
        .child(
            svg()
                .path(icon_path(doc.file_type.icon_name()))
                .w(px(20.))
                .h(px(20.))
                .text_color(icon_color),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .min_w_0()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(name_color)
                        .child(doc.name.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(path_color)
                        .truncate()
                        .child(doc.path.clone()),
                ),
        )
        .on_click(on_click)
        .on_mouse_down(MouseButton::Right, on_context)
}

/// 右键上下文菜单的单个条目（悬浮高亮 + 点击触发）。
fn ctx_menu_item(
    id: &'static str,
    label: String,
    c: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .w_full()
        .px_3()
        .py_1p5()
        .text_sm()
        .text_color(c.text_primary)
        .cursor_pointer()
        .hover(|s| s.bg(c.button_hover_bg))
        .child(SharedString::from(label))
        .on_click(on_click)
}

/// 确认弹窗的按钮（danger=true 用红色表示危险操作）。
fn confirm_button(
    id: &'static str,
    label: String,
    c: ThemeColors,
    danger: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let (bg, hover, fg) = if danger {
        (rgba(0xe54848ff), rgba(0xce3b3bff), rgba(0xffffffff))
    } else {
        (c.button_bg, c.button_hover_bg, c.text_primary)
    };
    div()
        .id(id)
        .px_4()
        .py_1p5()
        .rounded_md()
        .bg(bg)
        .text_sm()
        .text_color(fg)
        .cursor_pointer()
        .hover(|s| s.bg(hover))
        .child(SharedString::from(label))
        .on_click(on_click)
}

// ──────────────────────────────────────────────
// 交互逻辑（让欢迎窗口真正可用，而非花瓶）
// ──────────────────────────────────────────────

/// 统一的「打开编辑器窗口」入口：内存文档，不写磁盘（保存弹窗以后再做）。
/// - name：窗口标题（如 "Untitled" / 文件名）。
/// - model：若提供则以其内容初始化（来自 .ewp），否则空白。
///
/// `pub` 以便各视图在「文档类型切换 tab」（`ChromeCtx::on_switch_model`）里复用，
/// 打开对应类型的新窗口（决策⑤）。
pub fn open_editor(
    cx: &mut App,
    name: SharedString,
    model: Option<Model>,
    path: Option<PathBuf>,
) {
    let bounds = Bounds::centered(None, size(px(900.), px(620.)), cx);
    let _ = cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(name.clone()),
                appears_transparent: true,
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        },
        move |window, app| {
            // gpui-component 的 Input 文字颜色恒取 `cx.theme().foreground`（忽略父级
            // text_color），且其 init 默认跟随系统明暗。EWP 编辑器的白纸背景来自自身
            // 主题（content_bg），与系统明暗无关——若系统处于暗色，Input 会用浅色字画
            // 在白纸上而看不见。这里把 gpui-component 的主题模式绑定到 EWP 纸张背景的
            // 亮度：纸张亮 → Light（暗字），纸张暗 → Dark（亮字），确保始终有对比。
            let paper = crate::styles::ThemeColors::current().content_bg;
            let luminance = 0.299 * paper.r + 0.587 * paper.g + 0.114 * paper.b;
            let mode = if luminance >= 0.5 {
                gpui_component::ThemeMode::Light
            } else {
                gpui_component::ThemeMode::Dark
            };
            gpui_component::Theme::change(mode, Some(window), app);

            // gpui-component 的 Input 要求窗口根 view 是 `Root`（其 paint 里 Root::read
            // 会 panic）。这里按模型类型选对应的视图实体，统一转成 AnyView 再包进 Root。
            let view_entity: gpui::AnyView = match model {
                Some(m @ Model::Text(_)) => app
                    .new(|entity_cx| {
                        EditorView::new_from_model(window, entity_cx, name.clone(), m, path.clone())
                    })
                    .into(),
                Some(m @ Model::Slide(_)) => app
                    .new(|entity_cx| {
                        SlideView::new_from_model(window, entity_cx, name.clone(), m, path.clone())
                    })
                    .into(),
                Some(m @ Model::Sheet(_)) => app
                    .new(|entity_cx| {
                        SheetView::new_from_model(window, entity_cx, name.clone(), m, path.clone())
                    })
                    .into(),
                None => app
                    .new(|entity_cx| EditorView::new_blank(window, entity_cx, name.clone()))
                    .into(),
            };
            app.new(|cx| gpui_component::Root::new(view_entity, window, cx))
        },
    );
    cx.activate(true);
}

/// 「新建演示文稿」：打开一个空白演示窗口。
/// 注意：**不**把未保存的文稿加入最近列表 —— 只有真正落盘（保存）后
/// 才会通过视图的 `save_document` 进入最近列表。
fn create_new_presentation(_this: Option<gpui::WeakEntity<Welcome>>, cx: &mut App) {
    open_editor(cx, "Untitled".into(), Some(SlideView::default_model()), None);
}

/// 「新建电子表格」：打开一个空白表格窗口。同样不进入最近列表。
fn create_new_spreadsheet(_this: Option<gpui::WeakEntity<Welcome>>, cx: &mut App) {
    open_editor(cx, "Untitled".into(), Some(SheetView::default_model()), None);
}

/// 「新建项目」：打开一个空白编辑器窗口（内存文档，不写磁盘）。
/// 同样不进入最近列表。
fn create_new_project(_this: Option<gpui::WeakEntity<Welcome>>, cx: &mut App) {
    open_editor(cx, "Untitled".into(), None, None);
}

/// 「打开项目」：弹出系统文件选择器，选中后打开编辑器（.ewp 会真正加载模型）。
/// `this` 为欢迎窗口的弱引用；来自菜单时传 `None`（不更新最近列表）。
fn open_project(this: Option<gpui::WeakEntity<Welcome>>, cx: &mut App) {
    let options = PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: None,
    };
    let rx = cx.prompt_for_paths(options);
    cx.spawn(async move |async_cx| {
        if let Ok(Ok(Some(paths))) = rx.await {
            if let Some(path) = paths.into_iter().next() {
                let name: SharedString = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default()
                    .into();
                let model: Option<Model> = if path.extension().map(|e| e == "ewp").unwrap_or(false)
                {
                    match model::ser::load::<Model>(&path, model::ser::NativeFormat::Json) {
                        Ok(m) => Some(m),
                        Err(e) => {
                            eprintln!("[EWP] Failed to open {path:?}: {e}");
                            None
                        }
                    }
                } else {
                    None
                };
                // 类型图标必须按「实际加载到的模型」判定，而不是按扩展名：
                // 所有文稿都是 .ewp，若按 from_extension 一律会得到 Document，
                // 导致表格/演示文件在最近列表里显示成文档图标（标 Doc 打开 Excel）。
                let file_type = match &model {
                    Some(Model::Sheet(_)) => data::FileType::Excel,
                    Some(Model::Slide(_)) => data::FileType::PowerPoint,
                    Some(Model::Text(_)) | None => data::FileType::Document,
                };
                let doc = data::RecentDoc {
                    name: name.to_string(),
                    path: path.to_string_lossy().to_string(),
                    file_type,
                };
                let _ = async_cx.update(|app| {
                    if let Some(this) = this {
                        let id = this.entity_id();
                        let _ = this.update(app, |this, _cx| {
                            data::add_recent_doc(&mut this.data, doc);
                            this.selected = Some(0);
                        });
                        app.notify(id);
                    }
                    let save_path = if model.is_some() {
                        Some(path.clone())
                    } else {
                        None
                    };
                    open_editor(app, name.clone(), model, save_path);
                });
            }
        }
    })
    .detach();
}

/// 点击最近列表项：移动高亮，并尝试打开对应编辑器（.ewp 加载模型）。
fn open_recent(this: gpui::WeakEntity<Welcome>, index: usize, path: String, cx: &mut App) {
    let id = this.entity_id();
    let _ = this.update(cx, |this, _cx| {
        this.selected = Some(index);
    });
    cx.notify(id);
    let pathbuf = std::path::PathBuf::from(&path);
    let name: SharedString = pathbuf
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default()
        .into();
    let model: Option<Model> = if pathbuf.extension().map(|e| e == "ewp").unwrap_or(false) {
        match model::ser::load::<Model>(&pathbuf, model::ser::NativeFormat::Json) {
            Ok(m) => Some(m),
            Err(e) => {
                eprintln!("[EWP] Failed to open {path}: {e}");
                None
            }
        }
    } else {
        None
    };
    let save_path = if model.is_some() {
        Some(pathbuf.clone())
    } else {
        None
    };
    open_editor(cx, name, model, save_path);
}

/// 「从最近移除」：仅把该项从最近列表删除（不碰磁盘文件），并关闭右键菜单。
fn remove_from_recent(this: gpui::WeakEntity<Welcome>, path: String, cx: &mut App) {
    let id = this.entity_id();
    let _ = this.update(cx, |this, _cx| {
        data::remove_recent_doc(&mut this.data, &path);
        // 选中项前移，避免指向越界
        if let Some(sel) = this.selected {
            if sel > 0 {
                this.selected = Some(sel - 1);
            }
        }
        this.context_menu = None;
    });
    cx.notify(id);
}

/// 「删除项目」：从最近列表移除并删除磁盘上的文件，然后关闭确认弹窗。
fn delete_project(this: gpui::WeakEntity<Welcome>, path: String, cx: &mut App) {
    let id = this.entity_id();
    let _ = this.update(cx, |this, _cx| {
        let _ = std::fs::remove_file(&path);
        data::remove_recent_doc(&mut this.data, &path);
        this.confirm_delete = None;
    });
    cx.notify(id);
}

/// 「设置」：打开独立设置窗口。
fn open_settings(cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(560.), px(460.)), cx);
    let _ = cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(t!("settings.title").to_string().into()),
                appears_transparent: true,
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        },
        |_, cx| cx.new(|entity_cx| SettingsView::new(entity_cx)),
    );
    cx.activate(true);
}

// ──────────────────────────────────────────────
// 快捷键 & Action 处理器
// ──────────────────────────────────────────────

fn setup_keybindings(cx: &mut App) {
    cx.bind_keys(vec![
        // File
        KeyBinding::new("cmd-n", NewProject, None),
        KeyBinding::new("cmd-shift-n", NewWindow, None),
        KeyBinding::new("cmd-o", OpenProject, None),
        KeyBinding::new("cmd-shift-w", CloseWindow, None),
        KeyBinding::new("cmd-w", CloseProject, None),
        // Edit
        KeyBinding::new("cmd-f", Find, None),
        // View
        KeyBinding::new("cmd-=", ZoomIn, None),
        KeyBinding::new("cmd--", ZoomOut, None),
        KeyBinding::new("cmd-0", ResetZoom, None),
        KeyBinding::new("ctrl-cmd-f", ToggleFullScreen, None),
        // App
        KeyBinding::new("cmd-comma", Settings, None),
        KeyBinding::new("cmd-q", Quit, None),
        // Window
        KeyBinding::new("cmd-m", Minimize, None),
        KeyBinding::new("cmd-h", Hide, None),
    ]);
}

fn setup_actions(cx: &mut App) {
    // File
    cx.on_action::<NewProject>(|_, cx| create_new_project(None, cx));
    cx.on_action::<NewWindow>(|_, cx| create_new_project(None, cx));
    cx.on_action::<OpenProject>(|_, cx| open_project(None, cx));
    cx.on_action::<CloseProject>(|_, _cx| eprintln!("[EWP] Close Project"));
    cx.on_action::<CloseWindow>(|_, _cx| eprintln!("[EWP] Close Window"));
    // Edit
    cx.on_action::<Find>(|_, _cx| eprintln!("[EWP] Find"));
    // View
    cx.on_action::<ZoomIn>(|_, _cx| eprintln!("[EWP] Zoom In"));
    cx.on_action::<ZoomOut>(|_, _cx| eprintln!("[EWP] Zoom Out"));
    cx.on_action::<ResetZoom>(|_, _cx| eprintln!("[EWP] Reset Zoom"));
    // View —— 界面模式切换（标准工具栏 / 标签页式）
    cx.on_action::<SetUiMode>(|action, cx| {
        // mirrors LibreOffice: SfxNotebookBar::ExecMethod（保存激活模式 + 通知重绘）。
        // action.0 是运行时 String，菜单只传 "standard"/"tabbed" 两字面量，安全映射回 &'static str。
        let id: &'static str = match action.0.as_str() {
            "tabbed" => "tabbed",
            _ => "standard",
        };
        UiLayoutManager::global_mut(cx).set_mode(id);
    });
    cx.on_action::<ToggleFullScreen>(|_, cx| {
        if let Some(handle) = cx.active_window() {
            let _ = handle.update(cx, |_, window, _| window.toggle_fullscreen());
        }
    });
    // App
    cx.on_action::<Quit>(|_, cx| cx.quit());
    cx.on_action::<Settings>(|_, cx| open_settings(cx));
    cx.on_action::<Languages>(|_, cx| open_settings(cx));
    cx.on_action::<About>(|_, _cx| eprintln!("[EWP] About (TODO)"));
    // Window —— 交给 OS 默认行为（最小/最大化/全屏）。
    cx.on_action::<Minimize>(|_, cx| {
        if let Some(handle) = cx.active_window() {
            let _ = handle.update(cx, |_, window, _| window.minimize_window());
        }
    });
    cx.on_action::<Zoom>(|_, cx| {
        if let Some(handle) = cx.active_window() {
            let _ = handle.update(cx, |_, window, _| window.zoom_window());
        }
    });
    // Help
    cx.on_action::<EwpHelp>(|_, _cx| eprintln!("[EWP] Help (TODO)"));
    cx.on_action::<OpenDocumentation>(|_, _cx| eprintln!("[EWP] Documentation (TODO)"));
    cx.on_action::<ReportIssue>(|_, _cx| eprintln!("[EWP] Report Issue (TODO)"));
    // macOS 应用菜单：隐藏 / 隐藏其他 / 全部显示 —— 交给 OS。
    #[cfg(target_os = "macos")]
    {
        cx.on_action::<Hide>(|_, cx| cx.hide());
        cx.on_action::<HideOthers>(|_, cx| cx.hide_other_apps());
        cx.on_action::<ShowAll>(|_, cx| cx.unhide_other_apps());
    }
}

// ──────────────────────────────────────────────
// 系统语言检测
// ──────────────────────────────────────────────

fn detect_locale() -> &'static str {
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
    match locale.as_str() {
        l if l.starts_with("zh-CN") || l.starts_with("zh-Hans") => "zh-CN",
        l if l.starts_with("zh-TW") || l.starts_with("zh-HK") || l.starts_with("zh-Hant") => {
            "zh-TW"
        }
        l if l.starts_with("zh") => "zh-CN",
        _ => "en",
    }
}

// ──────────────────────────────────────────────
// 入口
// ──────────────────────────────────────────────

fn main() {
    // 初始语言：优先用已保存的设置（这样重启后保持上次选择），否则跟随系统语言。
    let saved = data::load_settings();
    let locale: String = if saved.locale.is_empty() {
        detect_locale().to_string()
    } else {
        saved.locale
    };
    rust_i18n::set_locale(&locale);

    let app_data = data::load();

    eprintln!("[EWP] Data directory: {}", data::data_dir().display());

    Application::new()
        .with_assets(FileAssetSource)
        .run(move |cx: &mut App| {
            // gpui-component 一次性初始化（主题 / 输入 / 弹层等全局状态）。
            // 必须在打开任何使用其组件的窗口之前调用，否则 Input 的 Root::read 会 panic。
            gpui_component::init(cx);

            // 菜单栏（从 app_menus 模块构建）
            cx.set_menus(app_menus::app_menus());

            // 快捷键 & action 处理器
            setup_keybindings(cx);
            setup_actions(cx);

            // 装载 UI 布局管理器（多套 UI 框架：standard / tabbed），并恢复已持久化的模式。
            // mirrors LibreOffice: SfxNotebookBar 注册布局 + 读取激活模式
            // （lcl_getNotebookbarFileName）。
            cx.set_global(UiLayoutManager::new_with_defaults());
            UiLayoutManager::global_mut(cx).set_mode(load_initial_mode());

            let bounds = Bounds::centered(None, size(px(800.), px(480.)), cx);
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_min_size: Some(size(px(700.), px(420.))),
                    ..Default::default()
                },
                |_window, cx| {
                    cx.new(|_| Welcome {
                        data: app_data.clone(),
                        selected: if app_data.recent_docs.is_empty() {
                            None
                        } else {
                            Some(0)
                        },
                        context_menu: None,
                        confirm_delete: None,
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
