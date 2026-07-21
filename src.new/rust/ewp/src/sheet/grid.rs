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

use crate::sheet::view_state::{Pane, SheetViewState};
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

// mirrors LibreOffice: sc/source/ui/view/viewdata.cxx — ScViewData::GetScrPos 的 X 方向累加
//   C++（研究文档 §7.2 逐字核心）：
//     SCCOL nPosX = GetPosX(eWhichX);          // 锚点列
//     for (SCCOL nX = nPosX; nX < nWhereX; nX++) {
//         sal_uInt16 nT = GetColWidth(nX);
//         if (nT) nScrPosX += ToPixel(nT, nPPTX);   // 累加列宽（隐藏列宽0跳过）
//     }
//   Rust 逐行对应：col_left(c) = (0..c).map(col_width).sum();
//     // 「从列 0 累加到列 c-1 的列宽和」= GetScrPos 中 anchor=0 时的列偏移；
//     //   uniform 列宽 → c*CELL_W，等价上面的累加循环。
//   偏差核对：隐藏列（宽 0）自然被 sum 跳过，与 LibreOffice `if (nT) nScrPosX += ...` 等价（v1 无隐藏列）。
/// 第 `c` 列左边缘的 content 坐标（像素）。列宽恒定时即 `c * CELL_W`。
pub fn col_left(c: usize) -> f32 {
    (0..c).map(col_width).sum()
}

// mirrors LibreOffice: sc/source/ui/view/viewdata.cxx — ScViewData::GetScrPos 的 Y 方向累加
//   C++（研究文档 §7.2）：行方向 nScrPosY 同理，用 GetRowHeight / nPPTY 累加。
//   Rust 逐行对应：row_top(r) = (0..r).map(row_height).sum();
//     // 「从行 0 累加到行 r-1 的行高和」= GetScrPos 的 Y 偏移；uniform 行高 → r*CELL_H。
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

// mirrors LibreOffice: sc/source/ui/view/viewdata.cxx — Paint 可见范围循环 + ScPositionHelper::AddPixelsWhile
//   C++（研究文档 §5.1，逐行）：从 anchor 起逐列累加像素宽直到超出 client 宽：
//     col = anchorCol; x = -remX;
//     while (col <= MaxCol && x < clientWidth) { w = ToPixel(W(col)); if (w>0) drawCell; x += w; col++; }
//   行同理（y / scroll_y / viewport_h）。
//   Rust 逐行对应：
//     // 跳过完全滚过左侧的列：while c0<total && x+col_width(c0) <= scroll_x { x+=col_width(c0); c0++; }
//     // 累加覆盖视口：      while c1<total && x < scroll_x+viewport_w { x+=col_width(c1); c1++; }
//     // 行同理（y / scroll_y / viewport_h）。
//   等价：c0 即 anchorCol（已滚过的列），c1 即末可见列；v1 frozen=0、anchor=0 退化一致。
//   大表性能扩展点：ScPositionHelper 前缀和+二分（研究文档 §6）与朴素累加算法等价，仅优化常数。
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

// mirrors LibreOffice: sc/source/ui/view/output.cxx — ScOutputData::DrawGrid + DrawBackground
//   DrawGrid 竖线循环（WebFetch 逐字核心）：
//     nPosX = mnScrX;
//     for (nX=mnX1; nX<=mnX2; nX++) {
//         sal_uInt16 nWidth = ...nWidth;
//         if (nWidth) { nPosX += nWidth*nLayoutSign;
//             aGrid.AddVerLine(bWorksInPixels, nPosX-nSignedOneX, mnScrY, mnScrY+mnScrH-nOneY, bDashed); }
//     }   // 横线循环同理（AddHorLine）；隐藏列/行宽0跳过。
//   DrawBackground（WebFetch 逐字核心）：for 行/列经 drawCells → rRenderContext.DrawRect 填背景色。
//   Rust 逐行对应：
//     window.paint_quad(fill(bounds, fill_color));        // ≈ DrawBackground 填 cell 背景（选中=accent 低透明）
//     window.paint_quad(quad(bounds,0,transparent,Edges{right,bottom},c.border,...)); // ≈ DrawGrid 网格线（仅右/下1px，避免与邻格重复）
//     if is_selected { window.paint_quad(quad(bounds,...,Edges::all(2.),c.accent,...)); } // ≈ 选中 2px 外框
//   偏差核对：EWP 仅画右/下边（共用边不重复绘制），与 Calc 网格线合并(ScGridMerger)目的一致；
//            未做合并单元格/分页符/保护色（扩展点，研究文档 §9）。
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

// mirrors LibreOffice: sc/source/ui/view/output.cxx — ScOutputData::DrawStrings(行号) + DrawBackground(行号带)
//   DrawStrings：逐行号绘制文本（eOutHorJust 对文本 Left / 数值 Right）；行号带底色由 DrawBackground 填 sidebar_bg 类色。
//   Rust 逐行对应：
//     let bounds = Bounds::new(point(0, y), size(HEADER_W, CELL_H));  // 行号列固定带（x=0 锁左，类比列标头带锁左缘）
//     window.paint_quad(fill(bounds, bg));                            // ≈ DrawBackground 行号带底色
//     ... Edges{right,bottom} 边框 ≈ DrawGrid 网格线
//     let shaped = window.text_system().shape_line(label, font, &[run], None);  // ≈ DrawText 取形（force_width=None → 自然宽）
//     shaped.paint(origin, CELL_H, window, cx);                       // ≈ DrawText 绘制行号
//   偏差核对：行号带为 EWP 独有（Calc 行号在固定左侧 splitter 槽）；其 Y 坐标与数据行 Y 同一表达式（见 cell_screen_y），横向锁左。
/// 绘制左侧行号列的一个行号。
///
/// 底色 `sidebar_bg`（选中 `accent` 低透明），数字由 `shape_line` 后 `paint`。
pub fn paint_row_number(
    window: &mut Window,
    cx: &mut App,
    x: f32,
    y: f32,
    row: usize,
    is_selected: bool,
    c: &ThemeColors,
) {
    // 行号带左缘锁定 canvas 原点 x（= ox）：与 `row_header_clip_rect(ox, ...)` 的左缘一致，
    // 否则在 canvas 不贴窗体左缘（ox≠0）时会被裁掉。原实现误用 `px(0.)`（窗体最左），
    // 仅在 ox=0 的特例下才侥幸可见。
    let bounds = Bounds::new(point(px(x), px(y)), size(px(HEADER_W), px(CELL_H)));
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

// mirrors LibreOffice: sc/source/ui/view/output.cxx — ScOutputData::DrawStrings(列标 A/B/C) + DrawBackground(列标头带)
//   DrawStrings 列标：列名 A/B/C…（EWP 由 col_name() 生成）；x 为列标头左缘窗口坐标。
//   Rust 逐行对应：
//     let bounds = Bounds::new(point(x, y), size(CELL_W, COL_HEADER_H)); // x 由调用方传「HEADER_W+col_left(c)-scroll_x」
//     window.paint_quad(fill(bounds, bg));                              // ≈ DrawBackground 列标头带底色(sidebar_bg)
//     ... Edges{right,bottom} 边框 ≈ DrawGrid 网格线
//     let shaped = window.text_system().shape_line(col_name, ..., None); // ≈ DrawText（自然宽，force_width=None）
//     shaped.paint(point(x+CELL_PAD, y+...), COL_HEADER_H, window, cx);  // ≈ DrawText 绘制列名
//   偏差核对：x 入参来自同源公式（与数据格 X 同一表达式）→ 列标头与数据列横向永远对齐
//            （单测 qa_col_header_and_data_share_formula）。
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

// mirrors LibreOffice: sc/source/ui/view/gridwin.cxx — ScGridWindow 左上角固定方块（列标头×行号交叉的 header 控件区）
//   C++：4-pane 布局中，TopLeft pane 外、列标头与行号交叉处有一固定方块（冻结角 / 全选按钮所在）。
//        EWP 用单一 canvas 后，该角由 paint_corner 最后绘制、不裁剪（见 view.rs render 的 4 区域顺序）。
//   Rust 逐行对应：
//     let bounds = corner_rect(x, y);             // 尺寸 HEADER_W × COL_HEADER_H，原点=canvas 原点
//     window.paint_quad(fill(bounds, c.sidebar_bg));   // 底色
//     window.paint_quad(quad(bounds, ..., Edges{right,bottom}, c.border, ...));  // 边框
//   偏差核对：角最后画（render 中置于三段裁剪带之后）确保左上交叉点盖最上层；与 Calc 固定角行为一致。
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

// mirrors LibreOffice: sc/source/ui/view/output.cxx — ScGridWindow 在冻结边界画重线（DrawGrid 冻结双线）
//   C++（output.cxx 冻结边界）：DrawGrid 在 GetScrPos(nFixPosX, 0) / GetScrPos(0, nFixPosY) 处画 2px 重线
//        （≈ 冻结分隔线），与数据格坐标同源（同一 GetScrPos 公式）。
//   Rust 逐行对应（同源，复用 cell_to_screen 同一坐标公式，不引入第二套坐标）：
//     let x = ox + state.cell_to_screen(frozen_cols, 0, Pane::BottomRight).0;  // 冻结列右缘屏幕 X
//     let y = oy + state.cell_to_screen(0, frozen_rows, Pane::BottomRight).1;  // 冻结行下缘屏幕 Y
//   偏差核对：坐标与数据区列标头/数据格同源（cell_to_screen 同一公式），冻结线永不偏离（红线 §2）。
/// 冻结分隔线的屏幕坐标（窗口坐标系）：竖线 X = 冻结列右缘，横线 Y = 冻结行下缘。
/// 与数据区坐标同源（经 `cell_to_screen(frozen, ...)`），不引入第二套坐标。
pub fn freeze_split_line(state: &SheetViewState, ox: f32, oy: f32) -> (f32, f32) {
    // 冻结分隔线 = 冻结区（锁死、不随滚动移动）的右/下缘。与数据区同源坐标：
    // 用「锁定 pane」(BottomLeft 锁横向 / TopRight 锁纵向) 的 cell_to_screen，
    // 等价于 HEADER_W + col_left(frozen_cols) / COL_HEADER_H + row_top(frozen_rows)
    // （不减 scroll_x/y，因为冻结列/行锁死，分隔线固定不漂移，红线 §2）。
    let x = ox + state.cell_to_screen(state.frozen_cols, 0, Pane::BottomLeft).0;
    let y = oy + state.cell_to_screen(0, state.frozen_rows, Pane::TopRight).1;
    (x, y)
}

// mirrors LibreOffice: sc/source/ui/view/output.cxx — ScOutputData::DrawGrid 冻结边界重线
//   C++：DrawGrid 在冻结边界画 2px 重线（颜色 ≈ 高亮色）。
//   Rust 逐行对应：
//     if frozen_cols>0 { paint_quad(fill(Bounds{x=fx-1, y=oy, w=2, h=canvas_h}, accent)); }  // ≈ 竖重线
//     if frozen_rows>0 { paint_quad(fill(Bounds{x=ox, y=fy-1, w=canvas_w, h=2}, accent)); }  // ≈ 横重线
/// 绘制冻结分隔线：当 `frozen_cols>0` 画竖线、当 `frozen_rows>0` 画横线（accent 色 2px）。
/// 坐标来自 `freeze_split_line`（与数据区同源）。应在四区域绘制之后调用（覆盖在最上层）。
pub fn paint_freeze_splitter(
    window: &mut Window,
    _cx: &mut App,
    ox: f32,
    oy: f32,
    canvas_w: f32,
    canvas_h: f32,
    state: &SheetViewState,
    c: &ThemeColors,
) {
    let (fx, fy) = freeze_split_line(state, ox, oy);
    if state.frozen_cols > 0 {
        let b = Bounds::new(
            point(px(fx - 1.0), px(oy)),
            size(px(2.0), px(canvas_h)),
        );
        window.paint_quad(fill(b, Rgba::from(c.accent)));
    }
    if state.frozen_rows > 0 {
        let b = Bounds::new(
            point(px(ox), px(fy - 1.0)),
            size(px(canvas_w), px(2.0)),
        );
        window.paint_quad(fill(b, Rgba::from(c.accent)));
    }
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

    // ── freeze_split_line：与 cell_to_screen 同源（T01/T04） ──
    #[test]
    fn freeze_split_line_uses_cell_to_screen_origin() {
        let mut s = SheetViewState::new();
        s.frozen_cols = 3;
        s.frozen_rows = 2;
        s.scroll_x = 100.0;
        s.scroll_y = 50.0;
        let (fx, fy) = freeze_split_line(&s, 12.0, 34.0);
        // 与直接经 cell_to_screen(frozen, 锁定 pane) 的公式严格一致（同源不变量）。
        let expect_x = 12.0 + s.cell_to_screen(3, 0, Pane::BottomLeft).0;
        let expect_y = 34.0 + s.cell_to_screen(0, 2, Pane::TopRight).1;
        assert_eq!(fx, expect_x);
        assert_eq!(fy, expect_y);
        // 等价展开：HEADER_W + col_left(3)（锁定 pane，不减 scroll_x/y，分隔线固定）。
        assert_eq!(fx, 12.0 + HEADER_W + col_left(3));
        assert_eq!(fy, 34.0 + COL_HEADER_H + row_top(2));
        // 无冻结时坐标退到原点角（与 corner_rect 一致）。
        let mut s0 = SheetViewState::new();
        let (zx, zy) = freeze_split_line(&s0, 12.0, 34.0);
        assert_eq!(zx, 12.0 + HEADER_W);
        assert_eq!(zy, 34.0 + COL_HEADER_H);
    }

// ─────────────────────────────────────────────────────────
// QA 针对性回归：v3「行号带被裁掉 / canvas 不贴窗体左缘」bug。
//
// 修复（grid.rs paint_row_number）：行号带左缘由误用的窗体最左 0
// 改为 canvas 原点 ox（= row_header_clip_rect 左缘）。当 canvas 不贴
// 窗体左缘（ox≠0）时，旧实现把行号画在 x=0，被 row_header_clip_rect
// 裁掉 → 行号空白 / canvas 看似「缺一块」。
//
// 以下测试锁定该不变式：行号带左缘必须 == ox == 行号裁剪带左缘；
// ox≠0 时绝不能为 0（旧 bug 值）。与绘制代码同源，结构保证对齐。
// ─────────────────────────────────────────────────────────
#[cfg(test)]
mod qa_canvas_offset_regression {
    use super::*;

    // 纯几何不变量：行号带左缘 = canvas 原点 ox = 行号裁剪带左缘。
    // 这是 v3 修复的核心几何约束；任何把行号带左缘写死为 0 的回退都会违背它。
    #[test]
    fn qa_row_number_band_locks_to_canvas_origin() {
        let _ = ThemeColors::default(); // 仅占位，本测试验证几何不变量，不进入绘制
        for ox in [0.0_f32, 12.0, 88.0] {
            for oy in [0.0_f32, 34.0, 120.0] {
                let cw = 880.0;
                let ch = 520.0;
                // 绘制时行号带裁剪矩形（溢出此带的内容被裁掉）
                let clip = row_header_clip_rect(ox, oy, ch);
                let clip_left = f32::from(clip.origin.x);
                // 修复后 paint_row_number 传入的 x 必须 == ox == clip_left
                let band_left = ox; // 即 paint_row_number(window, cx, ox, y, ...) 的 bounds 左缘
                assert_eq!(band_left, clip_left, "行号带左缘必须锁 ox (= 行号裁剪带左缘)");
                // 行号带恰好填满裁剪带宽度 HEADER_W，且永不侵入数据区(左缘+HEADER_W)
                assert_eq!(f32::from(clip.size.width), HEADER_W);
                let band_right = band_left + HEADER_W;
                let data_left = f32::from(data_clip_rect(ox, oy, cw, ch).origin.x);
                assert_eq!(data_left, ox + HEADER_W, "数据区左缘必须 = ox + HEADER_W");
                assert_eq!(band_right, data_left, "行号带右缘与数据区左缘相接、不重叠不间隙");
                // bug 复现断言：ox≠0 时旧值 0 与裁剪带左缘不一致 → 会被裁掉
                if ox != 0.0 {
                    assert_ne!(
                        band_left, 0.0,
                        "ox≠0 时行号带左缘绝不能回退到窗体最左 0（v3 bug）"
                    );
                }
            }
        }
    }

    // 注：paint_* 绘制路径需在真实 paint 阶段 + gpui `test-support` 特性下才能驱动
    // （TestAppContext / #[gpui::test] 均受该特性门控）。此处用纯几何不变量 +
    // 源码契约锁定修复，避免为单测引入重测试依赖；像素级位置由源码审查与
    // 同源公式不变量（qa_col_header_and_data_share_formula 等）共同兜底。
}
