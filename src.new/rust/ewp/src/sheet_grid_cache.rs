//! 单元格文字的 `ShapedLine` 缓存。
//!
//! 数据区改为单 canvas 命令式绘制后，每帧只重画可见切片（几十~上百格），
//! 但大多数单元格在相邻帧之间内容不变。把 `ShapedLine` 按
//! `(row, col, 显示文本, 主题哈希)` 缓存起来，可避免每帧重新 shape。
//!
//! 失效策略（v1 粗粒度）：写入 / 清空单元格时由调用方通过
//! `invalidate_for_sheet()` 整表清空；主题切换时因 `theme_hash` 不同自然不命中。

use gpui::ShapedLine;
use gpui::{App, Rgba, SharedString, TextRun, Window, px};
use std::collections::HashMap;

use crate::sheet_grid::{CELL_FONT_SIZE, CELL_PAD, CELL_W};
use crate::styles::ThemeColors;

/// 缓存键：行列坐标 + 显示文本 + 主题哈希。
/// 同内容不同格各自缓存，避免跨格误命中；主题切换时哈希变化自动失效。
#[derive(Clone, PartialEq, Eq, Hash)]
struct CellCacheKey {
    row: usize,
    col: usize,
    value: String,
    theme_hash: u64,
}

/// 单元格文字的 `ShapedLine` 缓存（存于独立 `Entity`，paint 闭包内安全访问，无重入风险）。
pub struct GridTextCache {
    map: HashMap<CellCacheKey, ShapedLine>,
}

impl Default for GridTextCache {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl GridTextCache {
    /// 取（或按需 shape）某单元格的 `ShapedLine`。
    /// `row` / `col` 仅用于区分缓存键；`theme` 用于文字颜色与主题失效。
    pub fn get_or_shape(
        &mut self,
        row: usize,
        col: usize,
        text: &str,
        theme: &ThemeColors,
        window: &mut Window,
        _cx: &mut App,
    ) -> ShapedLine {
        let theme_hash = pack_theme(theme);
        let key = CellCacheKey {
            row,
            col,
            value: text.to_string(),
            theme_hash,
        };
        if let Some(shaped) = self.map.get(&key).cloned() {
            return shaped;
        }

        // GPUI 0.2.2 无 Font::default()：取当前 UI 文本样式的字体即可（单元格够用）。
        let font = window.text_style().font();
        let run = TextRun {
            len: text.len(),
            font,
            color: theme.text_primary.into(), // Rgba -> Hsla
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let shaped = window.text_system().shape_line(
            SharedString::from(text.to_string()),
            px(CELL_FONT_SIZE),
            &[run],
            Some(px(CELL_W - 2.0 * CELL_PAD)),
        );
        self.map.insert(key, shaped.clone());
        shaped
    }

    /// 整表粗失效（写入 / 清空单元格后调用）。
    pub fn invalidate_for_sheet(&mut self) {
        self.map.clear();
    }
}

/// 把单个颜色打包成 u32（R8G8B8A8）。`ThemeColors` 字段为 `Rgba`，分量值域 [0,1]。
fn pack_color(color: Rgba) -> u32 {
    let r = (color.r.clamp(0.0, 1.0) * 255.0) as u32 & 0xff;
    let g = (color.g.clamp(0.0, 1.0) * 255.0) as u32 & 0xff;
    let b = (color.b.clamp(0.0, 1.0) * 255.0) as u32 & 0xff;
    let a = (color.a.clamp(0.0, 1.0) * 255.0) as u32 & 0xff;
    (r << 24) | (g << 16) | (b << 8) | a
}

/// 把主题关键色打包成 u64，用于缓存键：主题切换后整体失效。
fn pack_theme(c: &ThemeColors) -> u64 {
    ((pack_color(c.accent) as u64) << 32) | (pack_color(c.text_primary) as u64)
}
