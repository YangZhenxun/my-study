//! 设置界面
//!
//! 由菜单「设置」打开。当前提供两项真实可交互的设置：
//! - 语言（en / zh-CN / zh-TW）：选择后立即 `rust_i18n::set_locale`，界面文字随之切换。
//! - 主题（浅色 / 深色）：选择后持久化，并应用到本窗口。
//!
//! 设置持久化到 `data/settings.json`（见 `data.rs`）。

use gpui::{
    App, ClickEvent, Context, FontWeight, Render, Rgba, SharedString, Window, div, prelude::*, px,
    rgb,
};
use rust_i18n::t;

use crate::data::{load_settings, save_settings, Settings, Theme};

/// 支持的语言列表（标签用其本族语言显示）。
const LOCALES: &[(&str, &str)] = &[
    ("en", "English"),
    ("zh-CN", "简体中文"),
    ("zh-TW", "繁體中文"),
];

/// 设置窗口根视图。
pub struct SettingsView {
    settings: Settings,
}

impl SettingsView {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let settings = load_settings();
        // 进入设置时，让界面语言与已保存设置保持一致。
        rust_i18n::set_locale(&settings.locale);
        Self { settings }
    }

    /// 应用并保存语言选择（通知由调用方统一触发）。
    fn choose_locale(&mut self, code: &str) {
        self.settings.locale = code.to_string();
        rust_i18n::set_locale(code);
        save_settings(&self.settings);
    }

    /// 应用并保存主题选择（通知由调用方统一触发）。
    fn choose_theme(&mut self, theme: Theme) {
        self.settings.theme = theme;
        save_settings(&self.settings);
    }
}

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.settings.theme;
        let (bg, fg, panel, accent) = theme_colors(theme);
        let locale = self.settings.locale.clone();
        let current_theme = self.settings.theme;

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(bg)
            .text_color(fg)
            .px(px(28.))
            .py(px(22.))
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .child(t!("settings.title").to_string()),
            )
            .child(div().h(px(16.)))
            // ─── 语言 ───
            .child(section_title(t!("settings.language").to_string(), fg))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .children(LOCALES.iter().map(|(code, label)| {
                                let this = cx.entity();
                                let code = *code;
                                let selected = locale == code;
                                option_row(
                                    SharedString::from(*label),
                                    selected,
                                    panel,
                                    accent,
                                    fg,
                                    move |_, _, cx: &mut App| {
                                        cx.update_entity::<SettingsView, ()>(
                                            &this,
                                            |this, _ctx| this.choose_locale(code),
                                        );
                                        cx.notify(this.entity_id());
                                    },
                                )
                            })),
                    ),
            )
            .child(div().h(px(20.)))
            // ─── 主题 ───
            .child(section_title(t!("settings.theme").to_string(), fg))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(option_row(
                        t!("settings.light").to_string().into(),
                        current_theme == Theme::Light,
                        panel,
                        accent,
                        fg,
                        {
                            let this = cx.entity();
                                move |_, _, cx: &mut App| {
                                    cx.update_entity::<SettingsView, ()>(
                                        &this,
                                        |this, _ctx| this.choose_theme(Theme::Light),
                                    );
                                    cx.notify(this.entity_id());
                                }
                        },
                    ))
                    .child(option_row(
                        t!("settings.dark").to_string().into(),
                        current_theme == Theme::Dark,
                        panel,
                        accent,
                        fg,
                        {
                            let this = cx.entity();
                                move |_, _, cx: &mut App| {
                                    cx.update_entity::<SettingsView, ()>(
                                        &this,
                                        |this, _ctx| this.choose_theme(Theme::Dark),
                                    );
                                    cx.notify(this.entity_id());
                                }
                        },
                    )),
            )
            .child(div().h(px(20.)))
            .child(
                div()
                    .text_xs()
                    .child(t!("settings.apply_hint").to_string()),
            )
    }
}

/// 一段小标题。
fn section_title(label: String, fg: Rgba) -> impl IntoElement {
    div()
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .text_color(fg)
        .child(label)
}

/// 一个可点击的选项行（选中时高亮）。
fn option_row(
    label: SharedString,
    selected: bool,
    panel: Rgba,
    accent: Rgba,
    fg: Rgba,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let bg = if selected { accent } else { panel };
    let text = if selected { rgb(0xffffff) } else { fg };
    let mut el = div()
        .w_full()
        .px_4()
        .py_2p5()
        .rounded_md()
        .bg(bg)
        .text_color(text)
        .cursor_pointer()
        .child(label);
    // 注意：普通 `Div` 不实现 `StatefulInteractiveElement`（只有 `Stateful<Div>` 才实现
    // 带 `on_click` 的流式 API）。这里走 `Interactivity` 的原生 `on_click`，无需先 `.id()`。
    el.interactivity().on_click(on_click);
    el
}

/// 按主题返回配色（背景 / 前景 / 面板 / 强调色）。
fn theme_colors(theme: Theme) -> (Rgba, Rgba, Rgba, Rgba) {
    match theme {
        Theme::Light => (rgb(0xffffff), rgb(0x1d1d1f), rgb(0xf0f0f2), rgb(0x007aff)),
        Theme::Dark => (rgb(0x1e1e20), rgb(0xf2f2f5), rgb(0x2c2c2e), rgb(0x0a84ff)),
    }
}
