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
use std::cell::RefCell;
use std::collections::HashMap;

use crate::sheet::grid::CELL_FONT_SIZE;
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
///
/// 内部用 `RefCell` 提供内部可变性：在 paint 闭包内可通过 `Entity::read`（只读借用，
/// 不会像 `Entity::update` 那样在绘制阶段重入 `App::update` 而脱离画布的滚动/坐标上下文）
/// 取到 `&GridTextCache`，再由 `get_or_shape` 临时借用可变来增删缓存。
pub struct GridTextCache {
    map: RefCell<HashMap<CellCacheKey, ShapedLine>>,
}

impl Default for GridTextCache {
    fn default() -> Self {
        Self {
            map: RefCell::new(HashMap::new()),
        }
    }
}

impl GridTextCache {
    /// 取（或按需 shape）某单元格的 `ShapedLine`。
    /// `row` / `col` 仅用于区分缓存键；`theme` 用于文字颜色与主题失效。
    ///
    /// 接受 `&self`：通过内部 `RefCell` 临时可变借用完成增删，从而可在 paint 闭包内
    /// 经 `Entity::read` 以只读方式安全调用，避免重入 `Entity::update` 破坏绘制坐标上下文。
    pub fn get_or_shape(
        &self,
        row: usize,
        col: usize,
        text: &str,
        theme: &ThemeColors,
        window: &mut Window,
        _cx: &App,
    ) -> ShapedLine {
        let theme_hash = pack_theme(theme);
        let key = CellCacheKey {
            row,
            col,
            value: text.to_string(),
            theme_hash,
        };
        if let Some(shaped) = self.map.borrow().get(&key).cloned() {
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
        // 第 4 参数是 GPUI 0.2.2 `shape_line` 的 `force_width`：一旦传入，
        // `line_layout.rs` 会把每个字形强制等距推到 `0, fw, 2fw, ...`，
        // 导致单元格文字在 x 轴被等宽拉伸散开。LibreOffice 普通字符串绘制
        // 用的是自然宽度（`output2.cxx` 的 `DrawText`，不撑开），仅靠
        // `eOutHorJust` 定位，所以这里传 `None`，让文字按自然宽度排版。
        let shaped = window.text_system().shape_line(
            SharedString::from(text.to_string()),
            px(CELL_FONT_SIZE),
            &[run],
            None,
        );
        self.map.borrow_mut().insert(key, shaped.clone());
        shaped
    }

    /// 整表粗失效（写入 / 清空单元格后调用）。
    pub fn invalidate_for_sheet(&self) {
        self.map.borrow_mut().clear();
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
