//! 数据区 / 行号列的无状态命令式绘制与坐标计算。
//!
//! 设计参照 Zed 编辑器 + LibreOffice Calc：单 canvas 只绘制可见切片，
//! 每帧零单元格 DOM 节点，大幅提升大表性能。
//!
//! 本模块所有函数均接收 `&mut Window`，在 `SheetView` 的 canvas `paint` 闭包内被调用，
//! 不持有任何状态（状态在 `sheet_view.rs` 与 `GridTextCache` 中）。

use gpui::{
    App, Bounds, BorderStyle, Edges, Hsla, Pixels, Point, Rgba, SharedString, TextRun, Window,
    fill, point, px, quad, size, transparent_black,
};

use crate::sheet_grid_cache::GridTextCache;
use crate::styles::ThemeColors;

// 单元格像素尺寸与内边距（与 `sheet_view.rs` 中保持一致）。
pub const CELL_W: f32 = 100.0;
pub const CELL_H: f32 = 28.0;
pub const CELL_PAD: f32 = 4.0;
pub const CELL_FONT_SIZE: f32 = 13.0;
// 左侧行号列宽度（与 `sheet_view.rs` 中的 `HEADER_W` 同值）。
pub const HEADER_W: f32 = 44.0;

/// 当前可见的 (row, col) 窗口 + 当前滚动偏移。
#[derive(Clone, Copy, Debug)]
pub struct VisibleWindow {
    pub c0: usize,
    pub c1: usize,
    pub r0: usize,
    pub r1: usize,
    /// 已向左滚动的横向距离（正数）。
    /// 纵向滚动由 GPUI 随 `vscroll` 自动平移 canvas 子元素，无需在此记录。
    pub scroll_x: f32,
}

/// 第 `c` 列左边缘的 content 坐标（像素）。列宽恒定时即 `c * CELL_W`。
pub fn col_left(c: usize) -> f32 {
    (0..c).map(col_width).sum()
}

/// 第 `r` 行上边缘的 content 坐标（像素）。
pub fn row_top(r: usize) -> f32 {
    (0..r).map(row_height).sum()
}

/// 第 `c` 列宽（前向兼容可变列宽表：未来读 sheet 列宽表即可，无需改调用方）。
pub fn col_width(_c: usize) -> f32 {
    CELL_W
}

/// 第 `r` 行高（前向兼容可变行高）。
pub fn row_height(_r: usize) -> f32 {
    CELL_H
}

/// 计算当前可见的 (row, col) 窗口（仿 Calc 的 `AddPixelsWhile`）。
///
/// 从 (0,0) 按 `col_width` / `row_height` 累加偏移，跳过完全在 `scroll_x` / `scroll_y`
/// 左侧 / 上方的列 / 行，直到累加宽 / 高覆盖视口 `[scroll, scroll+viewport]`。
/// `scroll_x` / `scroll_y` 为**已滚动的正距离**（向下 / 向左为正）。
pub fn compute_visible_window(
    viewport_w: f32,
    viewport_h: f32,
    scroll_x: f32,
    scroll_y: f32,
    total_cols: usize,
    total_rows: usize,
) -> VisibleWindow {
    let mut c0 = 0usize;
    let mut x = 0.0;
    while c0 < total_cols && x + col_width(c0) <= scroll_x {
        x += col_width(c0);
        c0 += 1;
    }
    let mut c1 = c0;
    while c1 < total_cols && x < scroll_x + viewport_w {
        x += col_width(c1);
        c1 += 1;
    }
    let mut r0 = 0usize;
    let mut y = 0.0;
    while r0 < total_rows && y + row_height(r0) <= scroll_y {
        y += row_height(r0);
        r0 += 1;
    }
    let mut r1 = r0;
    while r1 < total_rows && y < scroll_y + viewport_h {
        y += row_height(r1);
        r1 += 1;
    }
    VisibleWindow {
        c0,
        c1: c1.max(c0 + 1),
        r0,
        r1: r1.max(r0 + 1),
        scroll_x,
    }
}

/// 绘制单个单元格的底纹 + 网格线 + 选中高亮。
///
/// 背景：选中格用 `accent` 低透明，非选中用 `content_bg`。
/// 网格线：仅画右 / 下 1px（避免与相邻格重复绘制）。
/// 选中额外加 2px `accent` 外框。全部用 `paint_quad`，不留 DOM 节点。
pub fn paint_cell_background(
    window: &mut Window,
    _cx: &mut App,
    bounds: Bounds<Pixels>,
    is_selected: bool,
    c: &ThemeColors,
) {
    let fill_color: Rgba = if is_selected {
        Rgba::from(Hsla::from(c.accent).opacity(0.18))
    } else {
        c.content_bg
    };
    window.paint_quad(fill(bounds, fill_color));

    let edges = Edges {
        top: px(0.),
        right: px(1.),
        bottom: px(1.),
        left: px(0.),
    };
    window.paint_quad(quad(
        bounds,
        0.,
        transparent_black(),
        edges,
        c.border,
        BorderStyle::default(),
    ));

    if is_selected {
        window.paint_quad(quad(
            bounds,
            0.,
            transparent_black(),
            Edges::all(px(2.)),
            c.accent,
            BorderStyle::default(),
        ));
    }
}

/// 绘制单元格文字（经 `GridTextCache` 缓存 `ShapedLine`，避免每帧重新 shape）。
///
/// 调用方需先把 `self.text_cache` 经 `Entity::update` 取出 `&mut GridTextCache` 后传入。
pub fn paint_cell_text(
    window: &mut Window,
    cx: &mut App,
    cache: &mut GridTextCache,
    origin: Point<Pixels>,
    row: usize,
    col: usize,
    text: &str,
    c: &ThemeColors,
) {
    if text.is_empty() {
        return;
    }
    let shaped = cache.get_or_shape(row, col, text, c, window, cx);
    // `ShapedLine::paint` 返回 `Result<()>`，命令式绘制下忽略即可。
    let _ = shaped.paint(origin, px(CELL_H), window, cx);
}

/// 绘制左侧行号列的一个行号。
///
/// 底色 `sidebar_bg`（选中 `accent` 低透明），数字由 `shape_line` 后 `paint`。
pub fn paint_row_number(
    window: &mut Window,
    cx: &mut App,
    y: f32,
    row: usize,
    is_selected: bool,
    c: &ThemeColors,
) {
    let bounds = Bounds::new(point(px(0.), px(y)), size(px(HEADER_W), px(CELL_H)));
    let bg: Rgba = if is_selected {
        Rgba::from(Hsla::from(c.accent).opacity(0.18))
    } else {
        c.sidebar_bg
    };
    window.paint_quad(fill(bounds, bg));

    let edges = Edges {
        top: px(0.),
        right: px(1.),
        bottom: px(1.),
        left: px(0.),
    };
    window.paint_quad(quad(
        bounds,
        0.,
        transparent_black(),
        edges,
        c.border,
        BorderStyle::default(),
    ));
    if is_selected {
        window.paint_quad(quad(
            bounds,
            0.,
            transparent_black(),
            Edges::all(px(2.)),
            c.accent,
            BorderStyle::default(),
        ));
    }

    let label = format!("{}", row + 1);
    let font = window.text_style().font();
    let run = TextRun {
        len: label.len(),
        font,
        color: (if is_selected { c.accent } else { c.text_muted }).into(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let shaped = window.text_system().shape_line(
        SharedString::from(label),
        px(CELL_FONT_SIZE),
        &[run],
        Some(px(HEADER_W - 2.0 * CELL_PAD)),
    );
    let origin = point(px(CELL_PAD), px(y + (CELL_H - CELL_FONT_SIZE) / 2.0));
    // `ShapedLine::paint` 返回 `Result<()>`，命令式绘制下忽略即可。
    let _ = shaped.paint(origin, px(CELL_H), window, cx);
}
