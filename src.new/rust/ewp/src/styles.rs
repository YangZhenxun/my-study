//! 主题色集中管理
//!
//! 所有界面（Welcome / Settings / Editor）统一通过 `ThemeColors` 获取配色。
//! 配色来自扩展系统（`crate::extension`）加载的主题文件，不再硬编码。
//!
//! 用法：在 `render()` 顶部调用 `ThemeColors::current()` 获取当前主题配色，
//! 然后传给子组件。

use gpui::Rgba;

/// 一套完整的主题配色。
///
/// 所有颜色都是 `Rgba`（GPUI 的颜色类型），可直接传给 `.bg()` / `.text_color()` 等。
#[derive(Clone, Copy)]
pub struct ThemeColors {
    // ── 背景 ──
    pub window_bg: Rgba,
    pub sidebar_bg: Rgba,
    pub content_bg: Rgba,

    // ── 文字 ──
    pub text_primary: Rgba,
    pub text_muted: Rgba,

    // ── 强调色 ──
    pub accent: Rgba,

    // ── 边框/分隔线 ──
    pub border: Rgba,

    // ── 交互元素 ──
    pub button_bg: Rgba,
    pub button_hover_bg: Rgba,
    pub selected_bg: Rgba,
    pub selected_text: Rgba,

    // ── 未选中药丸按钮 ──
    pub pill_border: Rgba,
    pub pill_text: Rgba,

    // ── 选中分类背景（侧边栏） ──
    pub nav_active_bg: Rgba,
}

impl Default for ThemeColors {
    /// 安全回退：全黑配色（仅在主题文件解析失败时使用）。
    fn default() -> Self {
        Self {
            window_bg: gpui::rgb(0x000000),
            sidebar_bg: gpui::rgb(0x000000),
            content_bg: gpui::rgb(0x000000),
            text_primary: gpui::rgb(0xffffff),
            text_muted: gpui::rgb(0x888888),
            accent: gpui::rgb(0x0a84ff),
            border: gpui::rgb(0x333333),
            button_bg: gpui::rgb(0x222222),
            button_hover_bg: gpui::rgb(0x333333),
            selected_bg: gpui::rgb(0x0a84ff),
            selected_text: gpui::rgb(0xffffff),
            pill_border: gpui::rgb(0x555555),
            pill_text: gpui::rgb(0xbbbbbb),
            nav_active_bg: gpui::rgb(0x093d60),
        }
    }
}

impl ThemeColors {
    /// 从磁盘设置加载当前主题并返回配色。
    /// 通过扩展系统（`ExtensionHost`）查找主题文件。
    pub fn current() -> Self {
        let theme_id = crate::data::load_settings().theme;
        crate::extension::ExtensionHost::shared().colors_for(&theme_id)
    }
}
