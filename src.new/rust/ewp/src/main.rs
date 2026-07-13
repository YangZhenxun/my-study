mod data;

use data::{AppData, FileType, RecentDoc};
use gpui::{
    App, Application, AssetSource, Bounds, Context, FontWeight, Render, SharedString,
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
// AssetSource —— 让 GPUI 的 svg() 能加载本地 SVG 文件
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
            // ═══ 右栏：最近文档列表 ═══
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
                    ),
            )
    }
}

// ──────────────────────────────────────────────
// 辅助组件
// ──────────────────────────────────────────────

fn icon_path(name: &str) -> String {
    format!("icons/{name}.svg")
}

/// 全宽操作按钮：左侧 SVG 图标 + 右侧文字，浅灰圆角背景。
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

/// 最近文档列表项。图标根据 file_type 切换。
fn recent_item(is_selected: bool, id: usize, doc: &RecentDoc) -> impl IntoElement {
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

    // 加载持久化数据；首次运行（无数据文件）时写入示例数据
    let mut app_data = data::load();
    if app_data.recent_docs.is_empty() {
        app_data.recent_docs = vec![
            RecentDoc {
                name: "EWP".into(),
                path: "~/Documents/oldEWP".into(),
                file_type: FileType::Document,
            },
            RecentDoc {
                name: "neoEWP".into(),
                path: "~/Documents".into(),
                file_type: FileType::Document,
            },
        ];
        data::save(&app_data);
    }

    // 打印数据目录路径（方便调试）
    eprintln!("[EWP] Data directory: {}", data::data_dir().display());

    Application::new()
        .with_assets(FileAssetSource)
        .run(move |cx: &mut App| {
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
                    cx.new(|_| Welcome { data: app_data.clone() })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
