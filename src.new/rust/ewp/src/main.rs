mod app_menus;
mod data;
mod ewp_actions;

use data::AppData;
use ewp_actions::*;
use gpui::{
    App, Application, AssetSource, Bounds, Context, FontWeight, KeyBinding, Render, SharedString,
    TitlebarOptions, Window, WindowBounds, WindowOptions, div, img, prelude::*, px, rgb, rgba,
    size, svg,
};
use rust_i18n::t;
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
}

impl Render for Welcome {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0xffffff))
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
                            .text_color(rgb(0x1d1d1f))
                            .mt(px(-4.))
                            .child(t!("welcome.title").to_string()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x86868b))
                            .child(t!("welcome.version").to_string()),
                    )
                    .child(
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
                            ))
                            .child(action_button(
                                "clone",
                                "arrow-down",
                                t!("welcome.clone_repository").to_string(),
                            ))
                            .child(action_button(
                                "open",
                                "folder",
                                t!("welcome.open_project").to_string(),
                            )),
                    ),
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
                    .bg(rgb(0xf8f9fa))
                    .children(
                        self.data
                            .recent_docs
                            .iter()
                            .enumerate()
                            .map(|(i, doc)| recent_item(i == 0, i, doc)),
                    )
                    .when(self.data.recent_docs.is_empty(), |panel| {
                        panel.child(
                            div()
                                .flex()
                                .flex_1()
                                .items_center()
                                .justify_center()
                                .text_sm()
                                .text_color(rgb(0x86868b))
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

fn action_button(id: &'static str, icon_name: &'static str, label: String) -> impl IntoElement {
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
        .bg(rgb(0xf0f0f2))
        .cursor_pointer()
        .text_base()
        .text_color(rgb(0x1d1d1f))
        .child(
            svg()
                .path(icon_path(icon_name))
                .w(px(18.))
                .h(px(18.))
                .text_color(rgb(0x86868b)),
        )
        .child(div().flex_1().child(SharedString::from(label)))
        .on_click(|_, _, _| {})
}

fn recent_item(is_selected: bool, id: usize, doc: &data::RecentDoc) -> impl IntoElement {
    let bg = if is_selected {
        rgb(0x007aff)
    } else {
        rgba(0xffffff00)
    };
    let name_color = if is_selected {
        rgb(0xffffff)
    } else {
        rgb(0x1d1d1f)
    };
    let path_color = rgb(0x86868b);
    let icon_color = if is_selected {
        rgb(0xffffff)
    } else {
        rgb(0x007aff)
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
        .on_click(|_, _, _| {})
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
        KeyBinding::new("cmd-shift-u", CloneRepository, None),
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
    ]);
}

fn setup_actions(cx: &mut App) {
    // File
    cx.on_action::<NewProject>(|_, _cx| eprintln!("[EWP] New Project"));
    cx.on_action::<NewWindow>(|_, _cx| eprintln!("[EWP] New Window"));
    cx.on_action::<OpenProject>(|_, _cx| eprintln!("[EWP] Open Project"));
    cx.on_action::<CloneRepository>(|_, _cx| eprintln!("[EWP] Clone Repository"));
    cx.on_action::<CloseProject>(|_, _cx| eprintln!("[EWP] Close Project"));
    cx.on_action::<CloseWindow>(|_, _cx| eprintln!("[EWP] Close Window"));
    // Edit
    cx.on_action::<Find>(|_, _cx| eprintln!("[EWP] Find"));
    // View
    cx.on_action::<ZoomIn>(|_, _cx| eprintln!("[EWP] Zoom In"));
    cx.on_action::<ZoomOut>(|_, _cx| eprintln!("[EWP] Zoom Out"));
    cx.on_action::<ResetZoom>(|_, _cx| eprintln!("[EWP] Reset Zoom"));
    cx.on_action::<ToggleFullScreen>(|_, _cx| eprintln!("[EWP] Toggle Full Screen"));
    // App
    cx.on_action::<Quit>(|_, cx| cx.quit());
    cx.on_action::<Settings>(|_, _cx| eprintln!("[EWP] Settings (TODO)"));
    cx.on_action::<About>(|_, _cx| eprintln!("[EWP] About (TODO)"));
    cx.on_action::<Languages>(|_, _cx| eprintln!("[EWP] Languages (TODO)"));
    // Window
    cx.on_action::<Minimize>(|_, _cx| eprintln!("[EWP] Minimize (TODO)"));
    cx.on_action::<Zoom>(|_, _cx| eprintln!("[EWP] Zoom (TODO)"));
    // Help
    cx.on_action::<EwpHelp>(|_, _cx| eprintln!("[EWP] Help (TODO)"));
    cx.on_action::<OpenDocumentation>(|_, _cx| eprintln!("[EWP] Documentation (TODO)"));
    cx.on_action::<ReportIssue>(|_, _cx| eprintln!("[EWP] Report Issue (TODO)"));
    // macOS window menu
    #[cfg(target_os = "macos")]
    {
        cx.on_action::<Hide>(|_, _cx| eprintln!("[EWP] Hide (TODO)"));
        cx.on_action::<HideOthers>(|_, _cx| eprintln!("[EWP] Hide Others (TODO)"));
        cx.on_action::<ShowAll>(|_, _cx| eprintln!("[EWP] Show All (TODO)"));
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
    let locale = detect_locale();
    rust_i18n::set_locale(locale);

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
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
