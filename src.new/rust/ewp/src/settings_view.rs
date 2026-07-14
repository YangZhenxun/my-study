//! 设置界面（Safari 风格顶栏 Tab 布局）
//!
//! 布局仿照 macOS Safari 的偏好设置窗口：
//! - 顶部水平图标标签栏（General / Appearance），选中项蓝色高亮 + 下划线。
//! - 下方内容区显示当前分类的设置行（label + description + control）。
//!
//! 设置行布局参考 Zed 源码的 `render_settings_item_layout()`（h_flex + justify_between）。
//!
//! 当前提供两项可交互设置：
//! - **General** 分类：语言（en / zh-CN / zh-TW），切换后全局生效。
//! - **Appearance** 分类：主题（浅色 / 深色），持久化到 `data/settings.json`。

use gpui::{
    App, ClickEvent, Context, FontWeight, Render, SharedString, Window, div, rgb,
};
use gpui::prelude::*;
use rust_i18n::t;

use crate::data::{load_settings, save_settings, Settings, Theme};

// ──── 分类（顶部标签栏） ──────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
enum SettingsCategory {
    General,
    Appearance,
}

impl SettingsCategory {
    /// 所有分类及其 i18n 标签名（按显示顺序）。
    const ALL: [(Self, &'static str); 2] = [
        (Self::General, "settings.general"),
        (Self::Appearance, "settings.appearance"),
    ];

    /// 图标（Unicode 占位，后续可替换为 SVG 资源）。
    fn icon(self) -> &'static str {
        match self {
            Self::General => "\u{2699}",   // ⚙
            Self::Appearance => "\u{1F3A8}", // 🎨
        }
    }

    fn label(self) -> String {
        match self {
            Self::General => t!("settings.general").to_string(),
            Self::Appearance => t!("settings.appearance").to_string(),
        }
    }
}

// ──── 语言列表 ────────────────────────────────────────────────

const LOCALES: &[(&str, &str)] = &[
    ("en", "English"),
    ("zh-CN", "简体中文"),
    ("zh-TW", "繁體中文"),
];

/// 返回某 locale code 对应的展示名。
fn locale_display_name(code: &str) -> &'static str {
    LOCALES
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, name)| *name)
        .unwrap_or("Unknown")
}

// ──── 主视图 ──────────────────────────────────────────────────

pub struct SettingsView {
    settings: Settings,
    selected_category: SettingsCategory,
}

impl SettingsView {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let settings = load_settings();
        Self {
            settings,
            selected_category: SettingsCategory::General,
        }
    }

    // ── 状态修改 ──

    fn choose_locale(&mut self, code: &str) {
        self.settings.locale = code.to_string();
        save_settings(&self.settings);
    }

    fn choose_theme(&mut self, theme: Theme) {
        self.settings.theme = theme;
        save_settings(&self.settings);
    }

    fn select_category(&mut self, cat: SettingsCategory) {
        self.selected_category = cat;
    }

    // ── 子区域渲染 ──

    /// 渲染「General」内容：语言选择行。
    fn render_general(
        &self,
        this: gpui::Entity<SettingsView>,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let locale = self.settings.locale.clone();
        let current_name = locale_display_name(&locale);

        div()
            .flex_col()
            .gap_3()
            // 小节标题
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x888888))
                    .child(t!("settings.general_settings").to_string()),
            )
            // 语言行
            .child(setting_row(
                t!("settings.language").to_string(),
                t!("settings.language_desc").to_string(),
                // 控件：当前语言名 + 下拉箭头（点击循环切换）
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_3()
                    .py_1p5()
                    .rounded_md()
                    .bg(rgb(0x0a84ff))
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .child(SharedString::from(current_name))
                    .child(" \u{25BE}") // ▾
                    .into_any_element(),
                {
                    let this = this.clone();
                    move |_, _, cx: &mut App| {
                        // 循环切换到下一个语言
                        let current_idx = LOCALES
                            .iter()
                            .position(|(c, _)| *c == locale)
                            .unwrap_or(0);
                        let next_idx = (current_idx + 1) % LOCALES.len();
                        let next_code = LOCALES[next_idx].0;

                        cx.update_entity::<SettingsView, ()>(&this, |this, _| {
                            this.choose_locale(next_code);
                        });
                        apply_locale(next_code, cx);
                        cx.notify(this.entity_id());
                    }
                },
            ))
    }

    /// 渲染「Appearance」内容：主题选择行。
    fn render_appearance(
        &self,
        this: gpui::Entity<SettingsView>,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_light = self.settings.theme == Theme::Light;

        div()
            .flex_col()
            .gap_3()
            // 小节标题
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x888888))
                    .child(t!("settings.appearance_settings").to_string()),
            )
            // 主题行
            .child(setting_row(
                t!("settings.theme").to_string(),
                t!("settings.theme_desc").to_string(),
                // 控件：Light / Dark 双按钮
                {
                    let light_this = this.clone();
                    let dark_this = this.clone();
                    div()
                        .flex()
                        .gap_2()
                        .child(theme_pill(
                            t!("settings.light").to_string(),
                            is_light,
                            {
                                let this = light_this;
                                move |_, _, cx: &mut App| {
                                    cx.update_entity::<SettingsView, ()>(&this, |this, _| {
                                        this.choose_theme(Theme::Light);
                                    });
                                    cx.notify(this.entity_id());
                                }
                            },
                        ))
                        .child(theme_pill(
                            t!("settings.dark").to_string(),
                            !is_light,
                            {
                                let this = dark_this;
                                move |_, _, cx: &mut App| {
                                    cx.update_entity::<SettingsView, ()>(&this, |this, _| {
                                        this.choose_theme(Theme::Dark);
                                    });
                                    cx.notify(this.entity_id());
                                }
                            },
                        ))
                        .into_any_element()
                },
                // 点击整行时也切换主题（辅助操作）
                {
                    let this = this.clone();
                    move |_, _, cx: &mut App| {
                        let target = if is_light { Theme::Dark } else { Theme::Light };
                        cx.update_entity::<SettingsView, ()>(&this, |this, _| {
                            this.choose_theme(target);
                        });
                        cx.notify(this.entity_id());
                    }
                },
            ))
    }
}

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();
        let selected = self.selected_category;

        // ── 配色（浅色基底，匹配 Safari 截图风格） ──
        let bg_window = rgb(0xf6f6f6);       // 窗口背景（浅灰）
        let bg_tab_bar = rgb(0xf6f6f6);      // 标签栏背景（同窗口）
        let fg_primary = rgb(0x1d1d1f);      // 主文字（深黑）
        let _fg_muted = rgb(0x86868b);        // 次要文字（灰，预留）
        let accent = rgb(0x0066cc);          // 强调蓝（Safari 风格）
        let _border_color = rgb(0xd2d2d7);    // 分隔线（浅灰）
        let tab_selected_fg = rgb(0x0066cc); // 选中标签文字蓝
        let tab_inactive_fg = rgb(0x555555); // 未选中标签灰

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(bg_window)
            .text_color(fg_primary)
            // ═══ 顶部标签栏 ═══
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .items_end()             // 标签底部对齐（下划线对齐）
                    .gap_6()
                    .px_6()
                    .pt_4()
                    .pb_0()
                    .bg(bg_tab_bar)
                    .border_b_1()
                    .border_color(rgb(0xd2d2d7))
                    // 各分类标签
                    .children(SettingsCategory::ALL.iter().map(|(cat, _key)| {
                        let is_active = *cat == selected;
                        let cat_label = cat.label();
                        let cat_icon = cat.icon();
                        let cat_value = *cat;
                        let tab_this = this.clone();

                        let mut tab = div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_1()
                            .pb_2()
                            .cursor_pointer();

                        if is_active {
                            tab = tab
                                .text_color(tab_selected_fg)
                                .font_weight(FontWeight::MEDIUM)
                                .border_b_2()
                                .border_color(accent);
                        } else {
                            tab = tab
                                .text_color(tab_inactive_fg);
                        }

                        // 图标 + 标签（链式调用避免 move）
                        tab = tab
                            .child(
                                div()
                                    .text_2xl()
                                    .child(cat_icon),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .child(cat_label),
                            );

                        // 点击切换分类（先注册 on_click 再返回完整元素）
                        {
                            let mut tab_with_click = tab;
                            tab_with_click.interactivity().on_click(move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                                cx.update_entity::<SettingsView, ()>(
                                    &tab_this,
                                    |this, _| this.select_category(cat_value),
                                );
                                cx.notify(tab_this.entity_id());
                            });
                            tab_with_click
                        }
                    })),
            )
            // ═══ 内容区 ═══
            .child(
                div()
                    .flex_1()
                    .px_6()
                    .py_5()
                    .flex()
                    .flex_col()
                    // 根据 selected_category 渲染不同内容
                    .child(match selected {
                        SettingsCategory::General => self.render_general(this.clone(), cx).into_any_element(),
                        SettingsCategory::Appearance => self.render_appearance(this.clone(), cx).into_any_element(),
                    }),
            )
    }
}

// ──── 全局 locale 广播 ──────────────────────────────────────────

fn apply_locale(code: &str, cx: &mut App) {
    rust_i18n::set_locale(code);
    cx.set_menus(crate::app_menus::app_menus());
    for handle in cx.windows() {
        let _ = handle.update(cx, |_, window, _| window.refresh());
    }
}

// ──── 通用 UI 组件 ─────────────────────────────────────────────

/// 一条设置行：标签 + 描述文字（左）+ 控件（右）。
///
/// 布局参考 Zed 源码的 `render_settings_item_layout()`：
/// `h_flex().justify_between()`，左侧 `v_flex()` 放标题+描述，右侧放控件。
fn setting_row(
    label: String,
    desc: String,
    control: impl IntoElement,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let mut el = div()
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .py_3()
        .border_b_1()
        .border_color(rgb(0xd2d2d7))
        .cursor_pointer()
        // 左侧：标签 + 描述
        .child(
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::MEDIUM)
                        .child(label),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x888888))
                        .child(desc),
                ),
        )
        // 右侧：控件
        .child(control);

    el.interactivity().on_click(on_click);
    el
}

/// 主题选择的小药丸按钮（选中=实心蓝，未选中=描边）。
fn theme_pill(
    label: String,
    selected: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let mut el = div()
        .px_4()
        .py_1p5()
        .rounded_md()
        .cursor_pointer()
        .text_sm()
        .child(label);

    if selected {
        el = el.bg(rgb(0x0066cc)).text_color(rgb(0xffffff));
    } else {
        el = el
            .border_1()
            .border_color(rgb(0xcccccc))
            .text_color(fg_pill_inactive());
    }

    el.interactivity().on_click(on_click);
    el
}

fn fg_pill_inactive() -> gpui::Rgba {
    rgb(0x333333)
}
