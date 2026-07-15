mod app_menus;
mod data;
mod editor_view;
mod ewp_actions;
mod extension;
mod model;
mod settings_view;
mod styles;

use data::AppData;
use editor_view::EditorView;
use ewp_actions::*;
use gpui::{
    App, Application, AssetSource, Bounds, ClickEvent, Context, FontWeight, KeyBinding,
    PathPromptOptions, Render, SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions,
    div, img, prelude::*, px, rgba, size, svg,
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
}

impl Render for Welcome {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = ThemeColors::current();

        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(c.window_bg)
            .overflow_hidden()
            // ═══ 左栏 ═══
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(440.))
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
                                move |_event, _window, cx| create_new_project(cx),
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
                    }),
            )
            // ═══ 右栏 ═══
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .h_full()
                    .pt(px(28.))
                    .pb(px(20.))
                    .pr(px(24.))
                    .pl(px(16.))
                    .bg(c.sidebar_bg)
                    .children({
                        let this = cx.weak_entity();
                        self.data
                            .recent_docs
                            .iter()
                            .enumerate()
                            .map(|(i, doc)| {
                                let path = doc.path.clone();
                                recent_item(self.selected == Some(i), i, doc, &c, {
                                    let this = this.clone();
                                    let path = path.clone();
                                    move |_event, _window, cx| {
                                        let this = this.clone();
                                        let path = path.clone();
                                        open_recent(this, i, path, cx)
                                    }
                                })
                            })
                            .collect::<Vec<_>>()
                    })
                    .when(self.data.recent_docs.is_empty(), |panel| {
                        panel.child(
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
}

// ──────────────────────────────────────────────
// 交互逻辑（让欢迎窗口真正可用，而非花瓶）
// ──────────────────────────────────────────────

/// 统一的「打开编辑器窗口」入口：内存文档，不写磁盘（保存弹窗以后再做）。
/// - name：窗口标题（如 "Untitled" / 文件名）。
/// - model：若提供则以其内容初始化（来自 .ewp），否则空白。
fn open_editor(
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
        move |_, app| {
            app.new(|entity_cx| {
                let n = name.clone();
                let p = path.clone();
                match &model {
                    Some(m) => EditorView::new_from_model(entity_cx, n, m.clone(), p),
                    None => EditorView::new_blank(entity_cx, n),
                }
            })
        },
    );
    cx.activate(true);
}

/// 「新建项目」：打开一个空白编辑器窗口（内存文档，不写磁盘）。
fn create_new_project(cx: &mut App) {
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
                let file_type = data::FileType::from_extension(&path);
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
    cx.on_action::<NewProject>(|_, cx| create_new_project(cx));
    cx.on_action::<NewWindow>(|_, cx| create_new_project(cx));
    cx.on_action::<OpenProject>(|_, cx| open_project(None, cx));
    cx.on_action::<CloseProject>(|_, _cx| eprintln!("[EWP] Close Project"));
    cx.on_action::<CloseWindow>(|_, _cx| eprintln!("[EWP] Close Window"));
    // Edit
    cx.on_action::<Find>(|_, _cx| eprintln!("[EWP] Find"));
    // View
    cx.on_action::<ZoomIn>(|_, _cx| eprintln!("[EWP] Zoom In"));
    cx.on_action::<ZoomOut>(|_, _cx| eprintln!("[EWP] Zoom Out"));
    cx.on_action::<ResetZoom>(|_, _cx| eprintln!("[EWP] Reset Zoom"));
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
            // 菜单栏（从 app_menus 模块构建）
            cx.set_menus(app_menus::app_menus());

            // 快捷键 & action 处理器
            setup_keybindings(cx);
            setup_actions(cx);

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
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
