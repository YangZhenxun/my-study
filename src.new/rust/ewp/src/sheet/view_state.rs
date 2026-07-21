//! 集中视图状态（类比 LibreOffice `ScViewData`）—— 滚动 / 冻结 / 缩放的唯一真相源。
//!
//! 本模块持有所有与「网格视图」相关的可变状态（滚动偏移、冻结行列、缩放），
//! 由 `sheet::view` 的单一 canvas 无状态地消费。所有绘制与命中测试都从
//! 同一组公式派生（见 `docs/sheet-view-refactor-design.md`），从结构上杜绝
//! 「两套坐标机制漂移导致的错位」。

use crate::sheet::grid::{
    col_left, col_width, compute_visible_window, row_height, row_top, HEADER_W, COL_HEADER_H,
};

/// 4 个 pane 标识（冻结 = 拆分的特例；v1 仅用 `BottomRight`）。
///
/// 变体目前仅在冻结相关测试中被构造；非测试构建下不被引用，故标注
/// `#[allow(dead_code)]`（冻结 UI 开关与 splitter 拖拽为后续任务）。
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pane {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// 集中视图状态 —— 唯一真相源（类比 LibreOffice `ScViewData`）。
///
/// 全部坐标状态都在这里；canvas / 绘制助手 / `SheetView` 持有或消费，但另不存坐标真相。
#[derive(Clone, Copy, Default, Debug)]
pub struct SheetViewState {
    /// 横向已滚动像素（向右滚为正）。
    pub scroll_x: f32,
    /// 纵向已滚动像素（向下滚为正）。
    pub scroll_y: f32,
    /// 冻结列数（0 = 不冻结）。
    pub frozen_cols: usize,
    /// 冻结行数（0 = 不冻结）。
    pub frozen_rows: usize,
    /// 缩放（1.0 = 100%）。v1 固定 1.0，未接入 UI（扩展点）。
    #[allow(dead_code)]
    pub zoom: f32,
}

impl SheetViewState {
    /// 构造默认视图状态。`zoom` 默认 100%（§3.1：缩放 1.0 = 100%）。
    ///
    /// 注意：结构体已 `derive(Default)`，而 `f32::default()` 为 `0.0`；为避免
    /// 「默认缩放 0% → 接入 zoom 后整表不可见」的潜在 bug，此处显式恢复 `zoom = 1.0`。
    /// `Default::default()` 仍可用（给出 `zoom = 0.0`），本构造函数是带正确缩放的规范入口。
    pub fn new() -> Self {
        let mut s = Self::default();
        s.zoom = 1.0;
        s
    }

    // —— 冻结几何（派生）——
    fn frozen_cols_px(&self) -> f32 {
        col_left(self.frozen_cols)
    }
    fn frozen_rows_px(&self) -> f32 {
        row_top(self.frozen_rows)
    }

    // —— pane 原点（拆分线像素位置），对应研究文档 3.3 节 originX/originY ——
    fn pane_origin(&self, pane: Pane) -> (f32, f32) {
        let ox = if matches!(pane, Pane::TopRight | Pane::BottomRight) {
            self.frozen_cols_px()
        } else {
            0.0
        };
        let oy = if matches!(pane, Pane::BottomLeft | Pane::BottomRight) {
            self.frozen_rows_px()
        } else {
            0.0
        };
        (ox, oy)
    }

    // 该 pane 方向的滚动贡献：冻结(上/左) pane 锁死 0，可滚(下/右) pane 用 state 滚动量。
    fn pane_scroll_x(&self, pane: Pane) -> f32 {
        if matches!(pane, Pane::TopLeft | Pane::BottomLeft) {
            0.0
        } else {
            self.scroll_x
        }
    }
    fn pane_scroll_y(&self, pane: Pane) -> f32 {
        if matches!(pane, Pane::TopLeft | Pane::TopRight) {
            0.0
        } else {
            self.scroll_y
        }
    }

    /// 增量滚动并 clamp。data_w/data_h = 数据区视口尺寸（已扣表头）；
    /// total_w/total_h = 整表像素尺寸。
    pub fn scroll_by(
        &mut self,
        dx: f32,
        dy: f32,
        data_w: f32,
        data_h: f32,
        total_w: f32,
        total_h: f32,
    ) {
        self.scroll_x += dx;
        self.scroll_y += dy;
        self.clamp(data_w, data_h, total_w, total_h);
    }

    /// 把 scroll 夹到 [0, max(0, total - data)]。
    pub fn clamp(&mut self, data_w: f32, data_h: f32, total_w: f32, total_h: f32) {
        let max_x = (total_w - data_w).max(0.0);
        let max_y = (total_h - data_h).max(0.0);
        self.scroll_x = self.scroll_x.clamp(0.0, max_x);
        self.scroll_y = self.scroll_y.clamp(0.0, max_y);
    }

    /// 通用（支持冻结 pane）。v1 调用均传 `Pane::BottomRight`（frozen=0 → origin/scroll 退化为单 pane）。
    /// 公式：绝对屏幕坐标 = origin(pane) + get_scr_pos − rem（见研究文档第 3.3 节）。
    #[allow(dead_code)]
    pub fn cell_to_screen(&self, col: usize, row: usize, pane: Pane) -> (f32, f32) {
        let (ox, oy) = self.pane_origin(pane);
        let sx = self.pane_scroll_x(pane);
        let sy = self.pane_scroll_y(pane);
        let x = HEADER_W + ox + col_left(col) - sx;
        let y = COL_HEADER_H + oy + row_top(row) - sy;
        (x, y)
    }

    /// 数据区（v1 唯一 pane）便捷方法，已含表头偏移。
    /// 等价于 `cell_to_screen(col, row, Pane::BottomRight)` 的退化形式（frozen=0）。
    /// v1 主绘制路径统一走 `cell_to_screen(col, row, pane)`（见 §3.4 / T04），
    /// 本方法作为同一坐标公式的便捷入口保留，供命中测试 / 外部调用使用。
    #[allow(dead_code)]
    pub fn cell_screen_x(&self, col: usize) -> f32 {
        HEADER_W + col_left(col) - self.scroll_x
    }
    #[allow(dead_code)]
    pub fn cell_screen_y(&self, row: usize) -> f32 {
        COL_HEADER_H + row_top(row) - self.scroll_y
    }

    /// 屏幕(内容)坐标 → 单元格（命中测试）。
    /// x/y 为已减去表头偏移后的「数据区内容坐标」（即 screen - HEADER_W + scroll_x）。
    pub fn content_to_cell(&self, x: f32, y: f32) -> (usize, usize) {
        let mut c = 0usize;
        let mut acc = 0.0;
        while acc + col_width(c) <= x {
            acc += col_width(c);
            c += 1;
        }
        let mut r = 0usize;
        let mut acy = 0.0;
        while acy + row_height(r) <= y {
            acy += row_height(r);
            r += 1;
        }
        (c, r)
    }

    /// 可见范围（复用无状态 `compute_visible_window`）。
    #[allow(dead_code)]
    pub fn visible_cols(&self, data_w: f32, total_cols: usize) -> (usize, usize) {
        let w = compute_visible_window(data_w, f32::MAX, self.scroll_x, 0.0, total_cols, 1);
        (w.c0, w.c1)
    }
    #[allow(dead_code)]
    pub fn visible_rows(&self, data_h: f32, total_rows: usize) -> (usize, usize) {
        let w = compute_visible_window(f32::MAX, data_h, 0.0, self.scroll_y, 1, total_rows);
        (w.r0, w.r1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sheet::grid::{col_left, row_top, COL_HEADER_H, HEADER_W, CELL_W, CELL_H};

    #[test]
    fn default_state_is_zeroed() {
        let s = SheetViewState::new();
        assert_eq!(s.scroll_x, 0.0);
        assert_eq!(s.scroll_y, 0.0);
        assert_eq!(s.frozen_cols, 0);
        assert_eq!(s.frozen_rows, 0);
        assert_eq!(s.zoom, 1.0);
    }

    #[test]
    fn clamp_bounds_when_total_smaller_than_data() {
        let mut s = SheetViewState::new();
        s.scroll_x = 500.0;
        s.scroll_y = 500.0;
        // 整表 100×56，数据区视口 1000×560 → max = 0，应夹到 0。
        s.clamp(1000.0, 560.0, 100.0, 56.0);
        assert_eq!(s.scroll_x, 0.0);
        assert_eq!(s.scroll_y, 0.0);
    }

    #[test]
    fn clamp_bounds_within_range() {
        let mut s = SheetViewState::new();
        // 总宽 1000，数据区视口 200 → max_x = 800。
        s.scroll_x = 300.0;
        s.clamp(200.0, 560.0, 1000.0, 560.0);
        assert_eq!(s.scroll_x, 300.0);
        // 总高 280，数据区视口 200 → max_y = 80。
        s.scroll_y = 50.0;
        s.clamp(200.0, 200.0, 1000.0, 280.0);
        assert_eq!(s.scroll_y, 50.0);
    }

    #[test]
    fn scroll_by_clamps_to_max() {
        let mut s = SheetViewState::new();
        // 向右滚 5000，但 max_x=800 → 夹到 800。
        s.scroll_by(5000.0, 0.0, 200.0, 200.0, 1000.0, 280.0);
        assert_eq!(s.scroll_x, 800.0);
        assert_eq!(s.scroll_y, 0.0);
        // 再向左滚 10000 → 夹到 0。
        s.scroll_by(-10000.0, 0.0, 200.0, 200.0, 1000.0, 280.0);
        assert_eq!(s.scroll_x, 0.0);
        // 纵向向下滚 9999，max_y=80 → 夹到 80。
        s.scroll_by(0.0, 9999.0, 200.0, 200.0, 1000.0, 280.0);
        assert_eq!(s.scroll_y, 80.0);
    }

    #[test]
    fn cell_screen_includes_header_offset() {
        let s = SheetViewState::new();
        assert_eq!(s.cell_screen_x(0), HEADER_W);
        assert_eq!(s.cell_screen_y(0), COL_HEADER_H);
        assert_eq!(s.cell_screen_x(1), HEADER_W + CELL_W);
        assert_eq!(s.cell_screen_y(1), COL_HEADER_H + CELL_H);
        // 滚动后应同步偏移。
        let mut s2 = SheetViewState::new();
        s2.scroll_x = 50.0;
        s2.scroll_y = 14.0;
        assert_eq!(s2.cell_screen_x(0), HEADER_W - 50.0);
        assert_eq!(s2.cell_screen_y(0), COL_HEADER_H - 14.0);
    }

    #[test]
    fn cell_to_screen_bottom_right_matches_helpers() {
        let mut s = SheetViewState::new();
        s.scroll_x = 123.0;
        s.scroll_y = 45.0;
        let (x, y) = s.cell_to_screen(2, 3, Pane::BottomRight);
        assert_eq!(x, s.cell_screen_x(2));
        assert_eq!(y, s.cell_screen_y(3));
    }

    #[test]
    fn cell_to_screen_pane_origin_for_frozen() {
        let mut s = SheetViewState::new();
        s.frozen_cols = 2;
        s.frozen_rows = 1;
        s.scroll_x = 100.0;
        s.scroll_y = 60.0;
        // TL：原点(0,0)、滚动锁死 → 数据区公式的退化。
        let (tl_x, tl_y) = s.cell_to_screen(5, 5, Pane::TopLeft);
        assert_eq!(tl_x, HEADER_W + col_left(5));
        assert_eq!(tl_y, COL_HEADER_H + row_top(5));
        // BR：原点(冻结像素)、滚动用 state 量。
        let (br_x, br_y) = s.cell_to_screen(5, 5, Pane::BottomRight);
        assert_eq!(br_x, HEADER_W + col_left(2) + col_left(5) - 100.0);
        assert_eq!(br_y, COL_HEADER_H + row_top(1) + row_top(5) - 60.0);
        // TR / BL 混合。
        let (tr_x, tr_y) = s.cell_to_screen(5, 5, Pane::TopRight);
        assert_eq!(tr_x, HEADER_W + col_left(2) + col_left(5) - 100.0);
        assert_eq!(tr_y, COL_HEADER_H + row_top(5));
        let (bl_x, bl_y) = s.cell_to_screen(5, 5, Pane::BottomLeft);
        assert_eq!(bl_x, HEADER_W + col_left(5));
        assert_eq!(bl_y, COL_HEADER_H + row_top(1) + row_top(5) - 60.0);
    }

    #[test]
    fn content_to_cell_inverse_of_left_top() {
        // 对任意 (c, r)，content_to_cell(col_left(c), row_top(r)) 应回 (c, r)。
        for c in 0..10usize {
            for r in 0..10usize {
                let (gc, gr) = SheetViewState::new().content_to_cell(col_left(c), row_top(r));
                assert_eq!((gc, gr), (c, r), "c={c} r={r}");
            }
        }
    }

    #[test]
    fn content_to_cell_offsets_into_cell() {
        let s = SheetViewState::new();
        // col_left(2) 内部 +10px 应归 col2；row_top(4) 内部 +10px 应归 row4（从常量派生，换间距仍成立）。
        assert_eq!(s.content_to_cell(col_left(2) + 10.0, row_top(4) + 10.0), (2, 4));
        // 行号 col0 在 x=10 应归 col0。
        assert_eq!(s.content_to_cell(10.0, 10.0), (0, 0));
        // content 坐标空间与 scroll 无关：同一 content 坐标不论滚动多少，逆向映射
        // 都应命中同一格。这正是「单一坐标真相源」要保障的——点击命中测试不会因
        // 滚动量而错位。这里在 col3 内 +10px、row4 内 +8px，期望恒为 (3,4)。
        for sx in [0.0_f32, 40.0, 123.0] {
            for sy in [0.0_f32, 20.0, 77.0] {
                let mut s2 = SheetViewState::new();
                s2.scroll_x = sx;
                s2.scroll_y = sy;
                // 注意：content_to_cell 的入参是「已扣表头偏移 + 已加滚动量」的内容坐标
                // （见方法文档）。内容坐标空间本身与 scroll 无关，故直接取 col_left/row_top。
                let px = col_left(3) + 10.0;
                let py = row_top(4) + 8.0;
                assert_eq!(s2.content_to_cell(px, py), (3, 4), "scroll=({sx},{sy})");
            }
        }
        // 真·往返（round-trip）：经由 cell_screen_x/cell_screen_y 构造 content 坐标，
        // 验证 `content = screen - HEADER_W + scroll_x` 的反向公式在任意 scroll 下可逆。
        let mut s3 = SheetViewState::new();
        s3.scroll_x = 40.0;
        s3.scroll_y = 20.0;
        let sx = s3.cell_screen_x(3);
        let sy = s3.cell_screen_y(4);
        let px = sx - HEADER_W + s3.scroll_x;
        let py = sy - COL_HEADER_H + s3.scroll_y;
        assert_eq!(s3.content_to_cell(px, py), (3, 4));
    }

    // ═══════════════════════════════════════════════════════════════════
    // QA 针对性测试（严过关）：证明"冻结窗格/滚动坐标错位" bug 已根除。
    // 用「同源不变式 + 严格往返 + 边界 clamp + 冻结原点数学」证明，而非确认。
    // ═══════════════════════════════════════════════════════════════════

    // 同源不变式：cell_screen_x / cell_screen_y 必须恒等于设计规定的唯一表达式。
    // 任何"第二套手算坐标"的回归都会让此测试失败（这正是旧 bug 的根因）。
    #[test]
    fn qa_cell_screen_equals_canonical_formula() {
        let scrolls = [0.0_f32, 40.0, 123.0, 1199.0, 1_000_000.0];
        for &sx in scrolls.iter() {
            for &sy in scrolls.iter() {
                let mut s = SheetViewState::new();
                s.scroll_x = sx;
                s.scroll_y = sy;
                for c in 0..12usize {
                    assert_eq!(
                        s.cell_screen_x(c),
                        HEADER_W + col_left(c) - sx,
                        "cell_screen_x 必须恒等于 HEADER_W + col_left(c) - scroll_x (scroll=({sx},{sy}), c={c})"
                    );
                }
                for r in 0..12usize {
                    assert_eq!(
                        s.cell_screen_y(r),
                        COL_HEADER_H + row_top(r) - sy,
                        "cell_screen_y 必须恒等于 COL_HEADER_H + row_top(r) - scroll_y (scroll=({sx},{sy}), r={r})"
                    );
                }
            }
        }
    }

    // 列标头 X 与数据格 X 同源：二者都经同一表达式 cell_screen_x，故对任意列恒等。
    // 行号 Y 与数据格 Y 同理。这是"错位在结构上不可能"的直接不变式。
    #[test]
    fn qa_col_header_and_data_share_formula() {
        let mut s = SheetViewState::new();
        s.scroll_x = 321.0;
        s.scroll_y = 88.0;
        for c in 0..16usize {
            // paint 中：列标头 x = HEADER_W + col_left(c) - scroll_x；数据格 x = cell_screen_x(c)。
            let header_x = HEADER_W + col_left(c) - s.scroll_x;
            let data_x = s.cell_screen_x(c);
            assert_eq!(header_x, data_x, "列标头与数据列横向必须同源 (c={c})");
        }
        for r in 0..24usize {
            let header_y = COL_HEADER_H + row_top(r) - s.scroll_y;
            let data_y = s.cell_screen_y(r);
            assert_eq!(header_y, data_y, "行号与数据行纵向必须同源 (r={r})");
        }
    }

    // 多列/多行累加：cell_screen_x/y 严格单调、相邻间距恒为 CELL_W/CELL_H（uniform），
    // 保证列标头与数据列、行号与数据行的间距一致、不漂移。
    #[test]
    fn qa_multi_axis_accumulation_monotonic_pitch() {
        let mut s = SheetViewState::new();
        s.scroll_x = 137.0;
        let mut prev = s.cell_screen_x(0);
        for c in 1..16usize {
            let cur = s.cell_screen_x(c);
            assert!(cur > prev, "cell_screen_x 必须严格单调递增 (c={c})");
            assert_eq!(cur - prev, CELL_W, "相邻列间距必须恒为 CELL_W (c={c})");
            assert_eq!(
                cur - prev,
                col_left(c) - col_left(c - 1),
                "间距必须等于列左缘之差 (c={c})"
            );
            prev = cur;
        }
        let mut s2 = SheetViewState::new();
        s2.scroll_y = 51.0;
        let mut prev_y = s2.cell_screen_y(0);
        for r in 1..30usize {
            let cur = s2.cell_screen_y(r);
            assert!(cur > prev_y, "cell_screen_y 必须严格单调递增 (r={r})");
            assert_eq!(cur - prev_y, CELL_H, "相邻行间距必须恒为 CELL_H (r={r})");
            prev_y = cur;
        }
    }

    // 严格往返（命中测试逆映射）：对多组 scroll（含 0、正数、边界附近、很大值）与
    // 多组 (col,row)，验证
    //   content_to_cell(cell_screen_x(c)-HEADER_W+scroll_x, cell_screen_y(r)-COL_HEADER_H+scroll_y) == (c,r)
    // 这正是"列标头/数据区同源 + 命中测试正确"的数学证明：滚动量在往返中精确抵消，
    // 命中结果与滚动多少无关 —— 错位在结构上不可能发生。
    #[test]
    fn qa_roundtrip_inverse_mapping_across_scroll() {
        let total_w = col_left(20); // 2000
        let data_w = 800.0_f32;
        let max_x = total_w - data_w; // 1200
        let total_h = row_top(20); // 560
        let data_h = 200.0_f32;
        let max_y = total_h - data_h; // 360
        // 0、正数、边界附近、超大值。
        let scrolls_x = [0.0_f32, 40.0, 123.0, max_x - 1.0, max_x, 1_000_000.0];
        let scrolls_y = [0.0_f32, 20.0, 77.0, max_y - 1.0, max_y, 1_000_000.0];
        for &sx in scrolls_x.iter() {
            for &sy in scrolls_y.iter() {
                let mut s = SheetViewState::new();
                s.scroll_x = sx;
                s.scroll_y = sy;
                for c in 0..10usize {
                    for r in 0..10usize {
                        // 经由 cell_screen_x 构造 painted 屏幕位置（不含 canvas_ox）。
                        let px = s.cell_screen_x(c) - HEADER_W + s.scroll_x;
                        let py = s.cell_screen_y(r) - COL_HEADER_H + s.scroll_y;
                        // 关键不变量：无论 scroll 多少，px/py 恒为 col_left(c)/row_top(r)。
                        assert_eq!(
                            px, col_left(c),
                            "往返后内容 X 必须等于 col_left(c)，与滚动无关 (scroll=({sx},{sy}), c={c})"
                        );
                        assert_eq!(
                            py, row_top(r),
                            "往返后内容 Y 必须等于 row_top(r)，与滚动无关 (scroll=({sx},{sy}), r={r})"
                        );
                        let (gc, gr) = s.content_to_cell(px, py);
                        assert_eq!(
                            (gc, gr),
                            (c, r),
                            "命中测试往返必须回到 (c,r) (scroll=({sx},{sy}), (c,r)=({c},{r}))"
                        );
                    }
                }
            }
        }
    }

    // clamp 边界：scroll_by 超过合法区间 [0, total-data] 时，scroll 被夹住不越界
    // —— 防滚动错位/空白条。
    #[test]
    fn qa_clamp_boundary_scroll_by_exceeds() {
        let total_w = col_left(20); // = 20 * CELL_W（从常量派生，换尺寸仍正确）
        let data_w = 800.0_f32;
        let max_x = total_w - data_w; // 上界 = total - data
        let total_h = row_top(20); // = 20 * CELL_H
        let data_h = 200.0_f32;
        let max_y = total_h - data_h; // 上界 = total - data

        let mut s = SheetViewState::new();
        // 向右/下狂滚，远超 max。
        s.scroll_by(1_000_000.0, 1_000_000.0, data_w, data_h, total_w, total_h);
        assert_eq!(s.scroll_x, max_x, "远超上界应夹到 max_x={max_x}");
        assert_eq!(s.scroll_y, max_y, "远超上界应夹到 max_y={max_y}");
        assert!(s.scroll_x >= 0.0 && s.scroll_x <= max_x);
        assert!(s.scroll_y >= 0.0 && s.scroll_y <= max_y);

        // 从 max 处再向左/上狂滚，远超下界。
        s.scroll_by(-1_000_000.0, -1_000_000.0, data_w, data_h, total_w, total_h);
        assert_eq!(s.scroll_x, 0.0, "远超下界应夹到 0");
        assert_eq!(s.scroll_y, 0.0, "远超下界应夹到 0");

        // 从非零起点部分越界：先滚到 max 的一半，再滚 +max 必越过上界 → 夹到 max_x/max_y。
        // 滚动量由 max_x/max_y 派生（非硬编码 1000/500），换单元格尺寸仍成立。
        let mut s2 = SheetViewState::new();
        s2.scroll_by(max_x / 2.0, max_y / 2.0, data_w, data_h, total_w, total_h);
        s2.scroll_by(max_x, max_y, data_w, data_h, total_w, total_h);
        assert_eq!(s2.scroll_x, max_x, "部分越界上界仍夹到 max_x");
        assert_eq!(s2.scroll_y, max_y, "部分越界上界仍夹到 max_y");

        // total < data 退化：max=0，任何滚动都被夹到 0（内容小于视口，不应出现空白越界）。
        let mut s3 = SheetViewState::new();
        s3.scroll_by(999.0, 999.0, 5000.0, 5000.0, 100.0, 100.0);
        assert_eq!(s3.scroll_x, 0.0);
        assert_eq!(s3.scroll_y, 0.0);
    }

    // 冻结 pane 原点数学：cell_to_screen(c,r,Pane) 的 origin 必须与 frozen_*_px 一致，
    // 冻结 pane（TopLeft）scroll 锁死为 0；可滚 pane（BottomRight）用 state 滚动量。
    // v1 frozen=0 走退化分支，但此测试锁定冻结数学，防后续接入冻结时回归。
    #[test]
    fn qa_frozen_pane_origin_math() {
        let mut s = SheetViewState::new();
        s.frozen_cols = 2;
        s.frozen_rows = 1;
        s.scroll_x = 250.0;
        s.scroll_y = 60.0;
        let fp_x = col_left(s.frozen_cols); // 200
        let fp_y = row_top(s.frozen_rows); // 28

        // TopLeft（冻结）：origin(0,0)、scroll 锁 0 —— 与 state.scroll 无关。
        for &sx in [0.0_f32, 250.0, 1_000_000.0].iter() {
            for &sy in [0.0_f32, 60.0, 1_000_000.0].iter() {
                let mut st = SheetViewState::new();
                st.frozen_cols = 2;
                st.frozen_rows = 1;
                st.scroll_x = sx;
                st.scroll_y = sy;
                let (x, y) = st.cell_to_screen(7, 9, Pane::TopLeft);
                assert_eq!(x, HEADER_W + col_left(7), "TL pane X 锁死，与 scroll 无关");
                assert_eq!(y, COL_HEADER_H + row_top(9), "TL pane Y 锁死，与 scroll 无关");
            }
        }

        // BottomRight（可滚）：origin=(frozen px)、scroll 用 state 量。
        let (br_x, br_y) = s.cell_to_screen(7, 9, Pane::BottomRight);
        assert_eq!(br_x, HEADER_W + fp_x + col_left(7) - s.scroll_x);
        assert_eq!(br_y, COL_HEADER_H + fp_y + row_top(9) - s.scroll_y);

        // TopRight（滚列/冻行）：x 带 origin+scroll，y 锁死。
        let (tr_x, tr_y) = s.cell_to_screen(7, 9, Pane::TopRight);
        assert_eq!(tr_x, HEADER_W + fp_x + col_left(7) - s.scroll_x);
        assert_eq!(tr_y, COL_HEADER_H + row_top(9));

        // BottomLeft（冻列/滚行）：x 锁死，y 带 origin+scroll。
        let (bl_x, bl_y) = s.cell_to_screen(7, 9, Pane::BottomLeft);
        assert_eq!(bl_x, HEADER_W + col_left(7));
        assert_eq!(bl_y, COL_HEADER_H + fp_y + row_top(9) - s.scroll_y);

        // v1 退化（frozen=0）：BottomRight 必须等价于 cell_screen_x/y。
        let mut v1 = SheetViewState::new();
        v1.scroll_x = 99.0;
        v1.scroll_y = 33.0;
        let (x, y) = v1.cell_to_screen(4, 6, Pane::BottomRight);
        assert_eq!(x, v1.cell_screen_x(4));
        assert_eq!(y, v1.cell_screen_y(6));
    }
}
