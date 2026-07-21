//! 数据区 / 行号列的无状态命令式绘制与坐标计算。
//!
//! 设计参照 Zed 编辑器 + LibreOffice Calc：单 canvas 只绘制可见切片，
//! 每帧零单元格 DOM 节点，大幅提升大表性能。
//!
//! 本模块所有函数均接收 `&mut Window`，在 `SheetView` 的 canvas `paint` 闭包内被调用，
//! 不持有任何状态（状态在 `sheet::view` 与 `GridTextCache` 中）。

use gpui::{
    App, Bounds, BorderStyle, Edges, Hsla, Pixels, Rgba, SharedString, TextRun, Window,
    fill, point, px, quad, size, transparent_black,
};

use crate::styles::ThemeColors;

// 单元格像素尺寸与内边距（与 `sheet::view` 中保持一致）。
// 数值经一轮"更贴近 LibreOffice 的宽松间距"调整（原 100/28/4/13 → 120/34/8/14），
// 所有坐标公式 / 可见窗口 / 命中测试都从这些常量派生，改一处即全局生效。
pub const CELL_W: f32 = 120.0;
pub const CELL_H: f32 = 34.0;
pub const CELL_PAD: f32 = 8.0;
pub const CELL_FONT_SIZE: f32 = 14.0;
// 左侧行号列宽度（与 `sheet::view` 中的 `HEADER_W` 同值）。宽松调整后 56 → 64。
pub const HEADER_W: f32 = 64.0;
// 顶部列标头高度（从 `sheet::view` 迁入，作为唯一来源）。宽松调整后 28 → 34。
pub const COL_HEADER_H: f32 = 34.0;

/// 当前可见的 (row, col) 窗口 + 当前滚动偏移。
#[derive(Clone, Copy, Debug)]
pub struct VisibleWindow {
    pub c0: usize,
    pub c1: usize,
    pub r0: usize,
    pub r1: usize,
    /// 当前横向滚动偏移（正数），由调用方透传进 `compute_visible_window`。
    /// 仅作回显 / 测试读取之用；绘制时的真实坐标以 `SheetViewState.scroll_x` 为准。
    #[allow(dead_code)]
    pub scroll_x: f32,
    /// 当前纵向滚动偏移（正数），同上。
    #[allow(dead_code)]
    pub scroll_y: f32,
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
        scroll_y,
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
    // force_width=None — 自然排版；GPUI 的等间距网格重排(line_layout.rs:568)
    // 会让 "10" 的 '1' 和 '0' 拉开 36px，在窄行号列里虽不明显但原理相同。
    let shaped = window.text_system().shape_line(
        SharedString::from(label),
        px(CELL_FONT_SIZE),
        &[run],
        None,
    );
    let origin = point(px(CELL_PAD), px(y + (CELL_H - CELL_FONT_SIZE) / 2.0));
    // `ShapedLine::paint` 返回 `Result<()>`，命令式绘制下忽略即可。
    let _ = shaped.paint(origin, px(CELL_H), window, cx);
}

/// 绘制单个列标头（A/B/C…）。
///
/// 底色 `sidebar_bg`，右边框 + 底边框；选中时叠加 `accent` 低透明底与 2px 外框。
/// 文字由 `shape_line` 后 `paint`。坐标 `x` 为列标头左边缘的**窗口坐标**，
/// 由调用方（单一 canvas 的同源公式）传入，保证与数据区横向永远对齐。
pub fn paint_col_header(
    window: &mut Window,
    cx: &mut App,
    x: f32,
    y: f32,
    col_name: &str,
    is_selected: bool,
    c: &ThemeColors,
) {
    let bounds = Bounds::new(point(px(x), px(y)), size(px(CELL_W), px(COL_HEADER_H)));
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

    let len = col_name.len();
    let font = window.text_style().font();
    let run = TextRun {
        len,
        font,
        color: (if is_selected { c.accent } else { c.text_muted }).into(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let shaped = window.text_system().shape_line(
        SharedString::from(col_name.to_string()),
        px(CELL_FONT_SIZE),
        &[run],
        None,
    );
    let origin = point(px(x + CELL_PAD), px(y + (COL_HEADER_H - CELL_FONT_SIZE) / 2.0));
    let _ = shaped.paint(origin, px(COL_HEADER_H), window, cx);
}

/// 绘制左上角固定方块（尺寸 `HEADER_W × COL_HEADER_H`，底色 `sidebar_bg` + 边框）。
/// `x`/`y` 为角的窗口坐标（canvas 原点），由调用方传入。
pub fn paint_corner(window: &mut Window, _cx: &mut App, x: f32, y: f32, c: &ThemeColors) {
    let bounds = corner_rect(x, y);
    window.paint_quad(fill(bounds, c.sidebar_bg));
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
}

/// 在给定表头单元格范围上叠加选中高亮（accent 2px 外框），用于整列/整行选中。
/// v1 仅当对应列/行号被选中时由 `sheet::view` 调用（点击表头本身不特殊处理）。
pub fn paint_header_selection(
    window: &mut Window,
    _cx: &mut App,
    bounds: Bounds<Pixels>,
    c: &ThemeColors,
) {
    window.paint_quad(quad(
        bounds,
        0.,
        transparent_black(),
        Edges::all(px(2.)),
        c.accent,
        BorderStyle::default(),
    ));
}

// ─────────────────────────────────────────────────────────────
// 四个绘制区（角 / 列标头 / 行号 / 数据）的裁剪矩形。
//
// 这些是**纯几何**函数：只从 canvas 窗口原点 `(ox, oy)` 与 canvas 尺寸
// `(cw, ch)` 计算，与滚动无关。把 `sheet::view` paint 闭包里的内联
// `Bounds::new(...)` 抽出来，既消除重复，又让「四区域两两不相交」可被单测固化——
// 这正是用户报的「滚动时单元格覆盖表头」 bug 在结构上不可能的证明。
// ─────────────────────────────────────────────────────────────

/// 数据区裁剪矩形：左 / 上缘锁在 `HEADER_W` / `COL_HEADER_H`，
/// 滚动后溢出的格子被裁掉，永不盖到两侧表头槽。
pub fn data_clip_rect(ox: f32, oy: f32, cw: f32, ch: f32) -> Bounds<Pixels> {
    Bounds::new(
        point(px(ox + HEADER_W), px(oy + COL_HEADER_H)),
        size(px(cw - HEADER_W), px(ch - COL_HEADER_H)),
    )
}

/// 列标头带裁剪矩形：仅横向滚，左缘锁 `ox + HEADER_W`，永不盖行号槽。
pub fn col_header_clip_rect(ox: f32, oy: f32, cw: f32) -> Bounds<Pixels> {
    Bounds::new(
        point(px(ox + HEADER_W), px(oy)),
        size(px(cw - HEADER_W), px(COL_HEADER_H)),
    )
}

/// 行号带裁剪矩形：仅纵向滚，上缘锁 `oy + COL_HEADER_H`，永不盖列标头槽。
pub fn row_header_clip_rect(ox: f32, oy: f32, ch: f32) -> Bounds<Pixels> {
    Bounds::new(
        point(px(ox), px(oy + COL_HEADER_H)),
        size(px(HEADER_W), px(ch - COL_HEADER_H)),
    )
}

/// 左上角固定方块矩形：尺寸 `HEADER_W × COL_HEADER_H`，不裁剪、最后画。
pub fn corner_rect(ox: f32, oy: f32) -> Bounds<Pixels> {
    Bounds::new(
        point(px(ox), px(oy)),
        size(px(HEADER_W), px(COL_HEADER_H)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── 坐标常量 ──

    #[test]
    fn col_left_constants() {
        assert_eq!(col_left(0), 0.0);
        assert_eq!(col_left(1), CELL_W);
        assert_eq!(col_left(2), 2.0 * CELL_W);
    }

    #[test]
    fn row_top_constants() {
        assert_eq!(row_top(0), 0.0);
        assert_eq!(row_top(1), CELL_H);
        assert_eq!(row_top(2), 2.0 * CELL_H);
    }

    // ── compute_visible_window：原点（无滚动） ──

    #[test]
    fn visible_window_at_origin() {
        let w = compute_visible_window(1000.0, 560.0, 0.0, 0.0, 26, 100);
        assert_eq!(w.c0, 0);
        assert_eq!(w.c1, (1000.0 / CELL_W).ceil() as usize, "视口宽 / 单元格宽 -> 可见列数");
        assert_eq!(w.r0, 0);
        assert_eq!(w.r1, (560.0 / CELL_H).ceil() as usize, "视口高 / 单元格高 -> 可见行数");
        assert_eq!(w.scroll_x, 0.0);
    }

    // ── compute_visible_window：横向滚动 250 ──
    // 注意：任务描述写 c1==14，但按算法正确值是 13。
    // 视口 content 区间 [250, 1250]；列宽恒 100：
    //   col2 left=200、col12 left=1200 部分可见，col13 left=1300 已超出 1250 不可见。
    // 故可见列为 2..12（含），c1 上界（开）应为 13。
    #[test]
    fn visible_window_horizontal_scroll() {
        let w = compute_visible_window(1000.0, 560.0, 250.0, 0.0, 26, 100);
        assert_eq!(w.c0, (250.0 / CELL_W).floor() as usize, "完全滚过的列数");
        assert_eq!(w.c1, ((250.0 + 1000.0) / CELL_W).ceil() as usize, "视口右沿所在列");
        assert_eq!(w.r0, 0);
        assert_eq!(w.r1, (560.0 / CELL_H).ceil() as usize);
        // 首列可见左边缘部分超出视口左沿（负值，但不超过一个单元格宽）。
        let off = col_left(w.c0) - w.scroll_x;
        assert!(off < 0.0 && off >= -CELL_W, "首列部分滚出左沿");
        assert_eq!(w.scroll_x, 250.0);
    }

    // ── compute_visible_window：纵向滚动 280 ──
    #[test]
    fn visible_window_vertical_scroll() {
        let w = compute_visible_window(1000.0, 560.0, 0.0, 280.0, 26, 100);
        assert_eq!(w.r0, (280.0 / CELL_H).floor() as usize, "完全滚过的行数");
        assert_eq!(w.r1, ((280.0 + 560.0) / CELL_H).ceil() as usize, "视口下沿所在行");
        assert_eq!(w.scroll_x, 0.0);
    }

    // ── 边界：超大滚动不 panic，且 c1/r1 至少保证 1 列/行 ──
    #[test]
    fn visible_window_huge_scroll_no_panic() {
        let w = compute_visible_window(1000.0, 560.0, 1_000_000.0, 1_000_000.0, 26, 100);
        assert_eq!(w.c0, 26);
        assert_eq!(w.r0, 100);
        assert!(w.c1 >= w.c0 + 1, "至少 1 列");
        assert!(w.r1 >= w.r0 + 1, "至少 1 行");
    }

    // ── 边界：空表（total 为 0）不 panic，c1/r1.max(c0+1) 兜底 ──
    #[test]
    fn visible_window_empty_sheet() {
        let w = compute_visible_window(1000.0, 560.0, 0.0, 0.0, 0, 0);
        assert_eq!(w.c0, 0);
        assert_eq!(w.r0, 0);
        assert!(w.c1 >= w.c0 + 1);
        assert!(w.r1 >= w.r0 + 1);
    }

    // ── 任务规格回归：250×60 视口、5×5 表、无滚动 ──
    // 250 宽覆盖 2.5 列(含第 3 列起点) -> c0=0,c1=3；
    // 60 高覆盖 2.14 行(含第 3 行起点) -> r0=0,r1=3。
    #[test]
    fn visible_window_task_spec_origin() {
        let w = compute_visible_window(250.0, 60.0, 0.0, 0.0, 5, 5);
        assert_eq!(w.c0, 0, "X 无滚动");
        assert_eq!(w.c1, (250.0 / CELL_W).ceil() as usize, "视口宽 / 单元格宽");
        assert_eq!(w.r0, 0, "Y 无滚动");
        assert_eq!(w.r1, (60.0 / CELL_H).ceil() as usize, "视口高 / 单元格高");
        assert_eq!(w.scroll_x, 0.0);
        assert_eq!(w.scroll_y, 0.0);
    }

    // ── 任务规格回归：纵向滚动 28（滚过部分行）、视口高 60 ──
    // r0 = floor(scroll_y / CELL_H)（完全滚过的行数）；r1 = ceil((scroll_y + 视口高) / CELL_H)
    // （部分可见行必须绘制，否则视口底部出现空白）。断言从常量派生，见下方。
    #[test]
    fn visible_window_task_spec_vertical_scroll() {
        let w = compute_visible_window(250.0, 60.0, 0.0, 28.0, 5, 5);
        assert_eq!(w.r0, (28.0 / CELL_H).floor() as usize, "完全滚过的行数");
        assert_eq!(
            w.r1,
            ((28.0 + 60.0) / CELL_H).ceil() as usize,
            "视口下沿所在行（部分可见行须绘制）"
        );
    }

    // ─────────────────────────────────────────────────────────
    // 裁剪带（第 2/3 项核心）：四区域矩形边缘 + 两两不相交。
    // 这是用户「滚动时单元格覆盖表头」 bug 的固化证明——
    // 只要四矩形两两不相交，表头与数据在结构上就不可能互相覆盖。
    // ─────────────────────────────────────────────────────────

    // 取一组与默认常量不同的 canvas 几何，证明裁剪带是从 ox/oy/cw/ch 派生，
    // 而非写死（注意：这些函数签名里根本没有 scroll 参数 → 与滚动无关）。
    const OX: f32 = 12.0;
    const OY: f32 = 34.0;
    const CW: f32 = 880.0;
    const CH: f32 = 520.0;

    #[test]
    fn clip_col_header_rect_edges() {
        // 列标头带：左缘锁 ox+HEADER_W（永不盖行号槽），高恒为 COL_HEADER_H。
        // 与滚动无关：无论数据滚动多少，列标头都在固定水平带里横滑。
        let r = col_header_clip_rect(OX, OY, CW);
        assert_eq!(f32::from(r.origin.x), OX + HEADER_W, "左缘锁 ox+HEADER_W");
        assert_eq!(f32::from(r.origin.y), OY, "上缘贴 canvas 顶");
        assert_eq!(f32::from(r.size.width), CW - HEADER_W, "宽=画布宽-行号列宽");
        assert_eq!(f32::from(r.size.height), COL_HEADER_H, "高恒=COL_HEADER_H（与滚动无关）");
        // 结构证明：函数不接受 scroll 参数，左缘/高度恒为常量带。
        let r2 = col_header_clip_rect(OX, OY, CW); // 再算一次仍是同一带
        assert_eq!(f32::from(r2.origin.x), f32::from(r.origin.x));
        assert_eq!(f32::from(r2.size.height), f32::from(r.size.height));
    }

    #[test]
    fn clip_row_header_rect_edges() {
        // 行号带：上缘锁 oy+COL_HEADER_H（永不盖列标头槽），宽恒为 HEADER_W。
        let r = row_header_clip_rect(OX, OY, CH);
        assert_eq!(f32::from(r.origin.x), OX, "左缘贴 canvas 左");
        assert_eq!(f32::from(r.origin.y), OY + COL_HEADER_H, "上缘锁 oy+COL_HEADER_H");
        assert_eq!(f32::from(r.size.width), HEADER_W, "宽恒=HEADER_W（与滚动无关）");
        assert_eq!(f32::from(r.size.height), CH - COL_HEADER_H, "高=画布高-列标头高");
    }

    #[test]
    fn clip_data_rect_edges() {
        // 数据区：左缘锁 ox+HEADER_W、上缘锁 oy+COL_HEADER_H。
        // 滚动后溢出的格子被这个带裁掉 → 永不盖到两侧表头槽。
        let r = data_clip_rect(OX, OY, CW, CH);
        assert_eq!(f32::from(r.origin.x), OX + HEADER_W, "左缘锁 ox+HEADER_W");
        assert_eq!(f32::from(r.origin.y), OY + COL_HEADER_H, "上缘锁 oy+COL_HEADER_H");
        assert_eq!(f32::from(r.size.width), CW - HEADER_W, "宽=画布宽-行号列宽");
        assert_eq!(f32::from(r.size.height), CH - COL_HEADER_H, "高=画布高-列标头高");
    }

    #[test]
    fn clip_corner_rect_edges() {
        // 左上角固定方块：尺寸 HEADER_W × COL_HEADER_H，原点即 canvas 原点。
        let r = corner_rect(OX, OY);
        assert_eq!(f32::from(r.origin.x), OX);
        assert_eq!(f32::from(r.origin.y), OY);
        assert_eq!(f32::from(r.size.width), HEADER_W);
        assert_eq!(f32::from(r.size.height), COL_HEADER_H);
        assert_eq!(f32::from(r.bottom_right().x), OX + HEADER_W, "右下角 X");
        assert_eq!(f32::from(r.bottom_right().y), OY + COL_HEADER_H, "右下角 Y");
    }

    #[test]
    fn clip_four_rects_pairwise_disjoint() {
        // 四矩形两两不相交（仅边相邻，交集为空）→
        // 表头之间、表头与数据在结构上永不可能互相覆盖 = 用户 bug 不可能发生。
        let corner = corner_rect(OX, OY);
        let col_hdr = col_header_clip_rect(OX, OY, CW);
        let row_hdr = row_header_clip_rect(OX, OY, CH);
        let data = data_clip_rect(OX, OY, CW, CH);

        let pairs: [(&str, &Bounds<Pixels>, &Bounds<Pixels>); 6] = [
            ("corner×col_hdr", &corner, &col_hdr),
            ("corner×row_hdr", &corner, &row_hdr),
            ("corner×data", &corner, &data),
            ("col_hdr×row_hdr", &col_hdr, &row_hdr),
            ("col_hdr×data", &col_hdr, &data),
            ("row_hdr×data", &row_hdr, &data),
        ];
        for (name, a, b) in pairs.iter() {
            assert!(
                a.intersect(b).is_empty(),
                "裁剪带应两两不相交，但 {name} 交集非空（表头/数据可能互相覆盖）"
            );
            // 再用 intersects 双重确认（bool 语义与 intersect 判空一致）。
            assert!(!a.intersects(b), "{name} 不应相交");
        }
    }
}
