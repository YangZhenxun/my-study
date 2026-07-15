//! 主题文件（JSON）解析
//!
//! 主题文件格式（仿 Zed 的 user theme）：
//! ```json
//! {
//!   "name": "EWP Dark",
//!   "colors": {
//!     "window_bg": "#1e1e1e",
//!     "sidebar_bg": "#252526",
//!     ...
//!   }
//! }
//! ```

use serde::Deserialize;
use gpui::Rgba;

use crate::styles::ThemeColors;

/// 主题文件（从 JSON 反序列化）。
#[derive(Debug, Deserialize)]
pub struct ThemeFile {
    /// 主题显示名称。
    pub name: String,
    /// 配色表（hex 字符串 → 运行时转为 Rgba）。
    pub colors: ThemeColorsData,
}

/// 主题配色数据（JSON 中的 hex 字符串形式）。
///
/// 字段名与 `ThemeColors` 一一对应，但用 `String` 存储 hex 值。
/// 通过 `to_theme_colors()` 转换为运行时使用的 `ThemeColors`。
#[derive(Debug, Default, Deserialize)]
pub struct ThemeColorsData {
    pub window_bg: String,
    pub sidebar_bg: String,
    pub content_bg: String,
    pub text_primary: String,
    pub text_muted: String,
    pub accent: String,
    pub border: String,
    pub button_bg: String,
    pub button_hover_bg: String,
    pub selected_bg: String,
    pub selected_text: String,
    pub pill_border: String,
    pub pill_text: String,
    pub nav_active_bg: String,
}

impl ThemeColorsData {
    /// 将 hex 字符串配色表转换为运行时 `ThemeColors`。
    pub fn to_theme_colors(&self) -> ThemeColors {
        ThemeColors {
            window_bg: parse_hex(&self.window_bg),
            sidebar_bg: parse_hex(&self.sidebar_bg),
            content_bg: parse_hex(&self.content_bg),
            text_primary: parse_hex(&self.text_primary),
            text_muted: parse_hex(&self.text_muted),
            accent: parse_hex(&self.accent),
            border: parse_hex(&self.border),
            button_bg: parse_hex(&self.button_bg),
            button_hover_bg: parse_hex(&self.button_hover_bg),
            selected_bg: parse_hex(&self.selected_bg),
            selected_text: parse_hex(&self.selected_text),
            pill_border: parse_hex(&self.pill_border),
            pill_text: parse_hex(&self.pill_text),
            nav_active_bg: parse_hex(&self.nav_active_bg),
        }
    }
}

/// 解析 hex 颜色字符串为 `Rgba`。
///
/// 支持格式：`"#1e1e1e"` / `"1e1e1e"` / `"#1e1e1eff"`（含 alpha）。
/// 解析失败时返回黑色。
fn parse_hex(hex: &str) -> Rgba {
    let hex = hex.trim_start_matches('#');
    let n = u32::from_str_radix(hex, 16).unwrap_or(0x000000);
    gpui::rgb(n)
}
