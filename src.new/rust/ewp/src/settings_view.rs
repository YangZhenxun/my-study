//! 设置界面（Zed 风格侧边栏布局）
//!
//! 布局参考 Zed 的设置窗口（`crates/settings_ui/src/settings_ui.rs`）：
//! - 左侧导航栏：分类列表（icon + label），选中项高亮。
//! - 右侧内容区：分类标题 + 设置行（label + description + control）。
//!
//! 所有配色通过 `crate::styles::ThemeColors` 获取，跟随当前主题（浅色/深色）。
//!
//! 当前提供两项可交互设置：
//! - **General** 分类：语言（en / zh-CN / zh-TW），切换后全局生效。
//! - **Appearance** 分类：主题（浅色 / 深色），持久化到 `data/settings.json`。

use gpui::{
    anchored, deferred, App, ClickEvent, Context, Corner, FontWeight, MouseButton, Pixels, Point,
    Render, SharedString, Window, div, px, point, rgba,
};
use gpui::prelude::*;
use rust_i18n::t;

use crate::data::{load_settings, save_settings, Settings};
use crate::extension;
use crate::styles::ThemeColors;

// ──── 分类（侧边栏导航） ──────────────────────────────────

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
    /// 语言下拉是否展开（悬浮菜单）。
    locale_dropdown_open: bool,
    /// 语言下拉锚点（窗口坐标，点击语言行时记录）。
    locale_dropdown_anchor: Point<Pixels>,
}

impl SettingsView {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let settings = load_settings();
        Self {
            settings,
            selected_category: SettingsCategory::General,
            locale_dropdown_open: false,
            locale_dropdown_anchor: point(px(0.), px(0.)),
        }
    }

    // ── 状态修改 ──

    /// 展开/收起语言下拉菜单（记录点击位置作为悬浮锚点）。
    fn toggle_locale_dropdown(&mut self, anchor: Point<Pixels>, cx: &mut Context<Self>) {
        self.locale_dropdown_anchor = anchor;
        self.locale_dropdown_open = !self.locale_dropdown_open;
        cx.notify();
    }

    /// 从下拉菜单选中语言并收起。
    fn select_locale(&mut self, code: &str) {
        self.settings.locale = code.to_string();
        save_settings(&self.settings);
        self.locale_dropdown_open = false;
    }

    fn choose_theme(&mut self, theme: &str) {
        self.settings.theme = theme.to_string();
        save_settings(&self.settings);
    }

    fn select_category(&mut self, cat: SettingsCategory) {
        self.selected_category = cat;
    }

    // ── 子区域渲染 ──

    /// 渲染「General」内容：语言选择行（点击展开悬浮下拉菜单）。
    fn render_general(
        &self,
        this: gpui::Entity<SettingsView>,
        c: &ThemeColors,
    ) -> impl IntoElement {
        let locale = self.settings.locale.clone();
        let current_name = locale_display_name(&locale);

        // 点击语言行：切换悬浮下拉展开
        let on_toggle = {
            let this = this.clone();
            move |event: &ClickEvent, _: &mut Window, cx: &mut App| {
                let pos = event.position();
                let _ = this.update(cx, |this, cx| this.toggle_locale_dropdown(pos, cx));
            }
        };

        div()
            .flex_col()
            .gap_3()
            // 小节标题
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(c.text_muted)
                    .child(t!("settings.general_settings").to_string()),
            )
            // 语言项（点击展开悬浮下拉）
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .py_3()
                            .border_b_1()
                            .border_color(c.border)
                            .cursor_pointer()
                            .id("setting-language")
                            .on_click(on_toggle)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(t!("settings.language").to_string()),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(c.text_muted)
                                            .child(t!("settings.language_desc").to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_3()
                                    .py_1p5()
                                    .rounded_md()
                                    .bg(c.accent)
                                    .text_color(c.selected_text)
                                    .child(SharedString::from(current_name))
                                    .child(SharedString::from(" \u{25BE}")),
                            ),
                    ),
            )
    }

    /// 渲染「Appearance」内容：主题选择行。
    fn render_appearance(
        &self,
        this: gpui::Entity<SettingsView>,
        c: &ThemeColors,
    ) -> impl IntoElement {
        let current_theme = self.settings.theme.clone();
        let theme_list = extension::ExtensionHost::shared().theme_list();

        // 动态构建主题药丸按钮列表
        let mut pills = div().flex().gap_2();
        for (theme_id, theme_name) in &theme_list {
            let is_selected = *theme_id == current_theme;
            let tid = theme_id.to_string();
            let pill_this = this.clone();

            pills = pills.child(theme_pill(
                theme_id,
                theme_name.to_string(),
                is_selected,
                c,
                move |_, _, cx: &mut App| {
                    cx.update_entity::<SettingsView, ()>(&pill_this, |this, _| {
                        this.choose_theme(&tid);
                    });
                    apply_theme(cx);
                    cx.notify(pill_this.entity_id());
                },
            ));
        }

        div()
            .flex_col()
            .gap_3()
            // 小节标题
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(c.text_muted)
                    .child(t!("settings.appearance_settings").to_string()),
            )
            // 主题行
            .child(setting_row(
                "setting-theme",
                t!("settings.theme").to_string(),
                t!("settings.theme_desc").to_string(),
                c,
                pills.into_any_element(),
                // 点击整行时不做操作（多主题时无法简单切换）
                move |_, _, _| {},
            ))
    }
}

impl Render for SettingsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();
        let selected = self.selected_category;
        let c = ThemeColors::current();

        div()
            .size_full()
            .flex()
            .flex_row()
            .bg(c.window_bg)
            .text_color(c.text_primary)
            // ═══ 左侧导航栏 ═══
            .child(
                div()
                    .w(px(200.))
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(c.sidebar_bg)
                    .border_r_1()
                    .border_color(c.border)
                    .pt_3()
                    // 分类列表
                    .child({
                        let mut list = div().flex().flex_col().gap_0p5().px_2();
                        for (cat, _key) in SettingsCategory::ALL.iter() {
                            let is_active = *cat == selected;
                            let cat_label = cat.label();
                            let cat_icon = cat.icon();
                            let cat_value = *cat;
                            let row_this = this.clone();

                            let mut item = div()
                                .w_full()
                                .px_3()
                                .py_2()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .rounded_md()
                                .cursor_pointer();

                            if is_active {
                                item = item.bg(c.nav_active_bg).text_color(c.selected_text);
                            } else {
                                item = item.text_color(c.text_muted);
                            }

                            item = item
                                .child(
                                    div()
                                        .text_lg()
                                        .child(cat_icon),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .child(cat_label),
                                );

                            {
                                let nav_id: &'static str = match cat_value {
                                    SettingsCategory::General => "nav-general",
                                    SettingsCategory::Appearance => "nav-appearance",
                                };
                                let clickable = item.id(nav_id).on_click(
                                    move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                                        cx.update_entity::<SettingsView, ()>(
                                            &row_this,
                                            |this, _| this.select_category(cat_value),
                                        );
                                        cx.notify(row_this.entity_id());
                                    },
                                );
                                list = list.child(clickable);
                            }
                        }
                        list
                    })
                    // 底部弹性填充
                    .child(div().flex_1()),
            )
            // ═══ 右侧内容区 ═══
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(c.content_bg)
                    .child(
                        div()
                            .flex_1()
                            .px_8()
                            .py_6()
                            .flex()
                            .flex_col()
                            .gap_1()
                            // 分类大标题
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .mb_4()
                                    .child(selected.label()),
                            )
                            // 根据 selected_category 渲染不同内容
                            .child(match selected {
                                SettingsCategory::General => self.render_general(this.clone(), &c).into_any_element(),
                                SettingsCategory::Appearance => self.render_appearance(this.clone(), &c).into_any_element(),
                            }),
                    ),
            )
            // ═══ 语言悬浮下拉（浮在内容上方，不撑开布局） ═══
            .when(self.locale_dropdown_open, |root| {
                let c = c.clone();
                let viewport = window.viewport_size();
                let locale_now = self.settings.locale.clone();
                let this2 = this.clone();
                // 遮罩：点外部关闭
                let backdrop = deferred(
                    anchored()
                        .position(point(px(0.), px(0.)))
                        .child(
                            div()
                                .id("locale-dropdown-backdrop")
                                .w(viewport.width)
                                .h(viewport.height)
                                .bg(rgba(0x00000000))
                                .on_mouse_down(MouseButton::Left, {
                                    let t = this2.clone();
                                    move |_, _, cx: &mut App| {
                                        let _ = t.update(cx, |this, cx| {
                                            this.locale_dropdown_open = false;
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
                        .position(self.locale_dropdown_anchor)
                        .child(
                            div()
                                .id("locale-dropdown-panel")
                                .flex()
                                .flex_col()
                                .min_w(px(180.))
                                .rounded_md()
                                .border_1()
                                .border_color(c.border)
                                .bg(c.content_bg)
                                .overflow_hidden()
                                .children(LOCALES.iter().copied().map(|(code, name)| {
                                    let selected = locale_now == code;
                                    let t = this2.clone();
                                    let code_str = code.to_string();
                                    div()
                                        .id(SharedString::from(format!("locale-opt-{code}")))
                                        .px_3()
                                        .py_1p5()
                                        .text_sm()
                                        .text_color(if selected { c.accent } else { c.text_primary })
                                        .hover(|s| s.bg(c.button_hover_bg))
                                        .child(SharedString::from(name))
                                        .on_click(move |_, _, cx: &mut App| {
                                            let _ = t.update(cx, |this, _| this.select_locale(&code_str));
                                            apply_locale(&code_str, cx);
                                            cx.notify(t.entity_id());
                                        })
                                })),
                        ),
                )
                .with_priority(1);
                root.child(backdrop).child(panel)
            })
    }
}

// ──── 全局广播 ──────────────────────────────────────────────

/// 切换语言后广播：改全局 locale + 重建菜单 + 刷新所有窗口。
fn apply_locale(code: &str, cx: &mut App) {
    rust_i18n::set_locale(code);
    cx.set_menus(crate::app_menus::app_menus());
    refresh_all_windows(cx);
}

/// 切换主题后广播：刷新所有窗口（让 Welcome 等其他窗口也跟随新主题）。
fn apply_theme(cx: &mut App) {
    refresh_all_windows(cx);
}

/// 遍历所有已开窗口，逐个 refresh（触发重渲染）。
fn refresh_all_windows(cx: &mut App) {
    for handle in cx.windows() {
        let _ = handle.update(cx, |_, window, _| window.refresh());
    }
}

// ──── 通用 UI 组件 ─────────────────────────────────────────────

/// 一条设置行：标签 + 描述文字（左）+ 控件（右）。
///
/// 布局参考 Zed 源码的 `render_settings_item_layout()`：
/// `h_flex().justify_between()`，左侧 `v_flex()` 放标题+描述，右侧放控件。
///
/// 注意：`id` 必须唯一且为 `'static`，因为 GPUI 0.2.2 的 `on_click` 派发依赖
/// element 拥有 `element_id`（否则命中测试拿不到 `InteractiveElementState`，点击永不触发）。
fn setting_row(
    id: &'static str,
    label: String,
    desc: String,
    c: &ThemeColors,
    control: impl IntoElement,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .py_3()
        .border_b_1()
        .border_color(c.border)
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
                        .text_color(c.text_muted)
                        .child(desc),
                ),
        )
        // 右侧：控件
        .child(control)
        .id(id)
        .on_click(on_click)
}

/// 主题选择的小药丸按钮（选中=实心蓝，未选中=描边）。
fn theme_pill(
    theme_id: &str,
    label: String,
    selected: bool,
    c: &ThemeColors,
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
        el = el.bg(c.accent).text_color(c.selected_text);
    } else {
        el = el
            .border_1()
            .border_color(c.pill_border)
            .text_color(c.pill_text);
    }

    el.id(SharedString::from(format!("theme-{theme_id}")))
        .on_click(on_click)
}
