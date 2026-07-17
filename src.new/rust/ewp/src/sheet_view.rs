//! 电子表格视图 —— Calc / Excel 风格（可滚动网格 + 列标行号 + 底部 sheet 标签）。
//!
//! 布局（从外到内）：
//! ┌──────────────────────────────────────────────────────────┐
//! │  工具栏：文件名 · ＋工作表 · 保存                            │
//! ├──────────────────────────────────────────────────────────┤
//! │  编辑栏：A1 ▢ [ 输入框 / 选中格内容 ]  ← 在此输入即创建单元格 │
//! ├──────────────────────────────────────────────────────────┤
//! │  ┌──┬───┬───┬───┬────┐   ← 列标 A B C ...                  │
//! │  │  │ A │ B │ C │ D  │                                     │
//! │  ├──┼───┼───┼───┼────┤   ← 行号 1 2 3 ...                  │
//! │  │1 │   │   │   │    │                                     │
//! │  │2 │   │   │   │    │     单元格网格（点击选中）           │
//! │  ...                                                      │
//! ├──────────────────────────────────────────────────────────┤
//! │  [Sheet1] [Sheet2] ＋        ← 底部工作表标签              │
//! ├──────────────────────────────────────────────────────────┤
//! │  状态栏：选区 / 行数 / 脏标记                                │
//! └──────────────────────────────────────────────────────────┘
//!
//! 模型层（`model::sheet`）已定义：
//!   Workbook { sheets: Vec<Sheet> }
//!   Sheet    { name, cells: HashMap<usize, HashMap<usize, Cell>> (稀疏), cols, rows }
//!   Cell     { value: CellValue, style: TextStyle }
//!   CellValue{ Empty | Number(f64) | Text(String) | Bool | Formula(String) }
//!
//! 单元格采用**稀疏存储**：空白格在内存里不存在。本视图提供「编辑栏」作为
//! 唯一的写入入口 —— 选中格子后，在编辑栏输入并回车，`set_cell` 会按需动态
//! 创建该格（仿 Apple Numbers 的动态网格）。这就是「用户如何新建单元格」。

use gpui::{
    App, Bounds, Context, Entity, FocusHandle, Focusable, FontWeight, KeyDownEvent, MouseButton,
    MouseDownEvent, Pixels, Point, Render, ScrollHandle, SharedString, TextRun, Window, canvas, div,
    point, px, rgba, size,
};
use gpui::prelude::*;
use gpui_component::input::{Input, InputEvent, InputState};
use std::collections::HashMap;

use crate::data;
use crate::model::common::TextStyle;
use crate::model::ser::NativeFormat;
use crate::model::sheet::{Cell, CellValue, Sheet, Workbook};
use crate::model::Model;
use crate::sheet_grid::{
    VisibleWindow, col_left, col_width,
    compute_visible_window, paint_cell_background,
    paint_row_number, row_height, row_top,
};
use crate::sheet_grid_cache::GridTextCache;
use crate::styles::ThemeColors;
use std::path::PathBuf;

// 默认网格尺寸（空白工作簿）。
const DEF_COLS: usize = 26;
const DEF_ROWS: usize = 100;
// 单元格像素尺寸。
const CELL_W: f32 = 100.0;
const CELL_H: f32 = 28.0;
const HEADER_W: f32 = 56.0; // 足够容纳 5 位行号（13px 字号下 "99999" ≈ 50px）
const COL_HEADER_H: f32 = 28.0;
// 单元格内边距与文字字号（与 `sheet_grid.rs` 保持一致）。
const CELL_PAD: f32 = 4.0;
const CELL_FONT_SIZE: f32 = 13.0;

/// 把列索引转成电子表格列名（0→A, 25→Z, 26→AA）。
fn col_name(mut n: usize) -> String {
    let mut s = String::new();
    n += 1;
    while n > 0 {
        let r = (n - 1) % 26;
        s.insert(0, (b'A' + r as u8) as char);
        n = (n - 1) / 26;
    }
    s
}

/// 把单元格值格式化为**显示**文本。
fn format_cell(v: &CellValue) -> String {
    match v {
        CellValue::Empty => String::new(),
        CellValue::Number(f) => {
            if f.fract() == 0.0 {
                format!("{}", *f as i64)
            } else {
                let s = format!("{:.4}", f);
                s.trim_end_matches('0').trim_end_matches('.').to_string()
            }
        }
        CellValue::Text(t) => t.clone(),
        CellValue::Bool(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
        CellValue::Formula(s) => s.clone(),
    }
}

/// 把单元格值格式化为**可编辑**文本（用于回填编辑栏）。
/// 公式会带 `=` 前缀，数字按原值，其余即内容本身。
fn raw_cell_text(v: &CellValue) -> String {
    match v {
        CellValue::Empty => String::new(),
        CellValue::Number(f) => {
            if f.fract() == 0.0 {
                format!("{}", *f as i64)
            } else {
                format!("{}", f)
            }
        }
        CellValue::Text(t) => t.clone(),
        CellValue::Bool(b) => if *b { "TRUE".into() } else { "FALSE".into() },
        CellValue::Formula(s) => format!("={s}"),
    }
}

/// 把编辑栏中的字符串解析为 `CellValue`：
/// 空 → Empty；以 `=` 开头 → Formula；可解析为 f64 → Number；
/// true/false → Bool；其余 → Text。
fn parse_cell_value(s: &str) -> CellValue {
    let t = s.trim();
    if t.is_empty() {
        return CellValue::Empty;
    }
    if let Some(rest) = t.strip_prefix('=') {
        return CellValue::Formula(rest.to_string());
    }
    if let Ok(n) = t.parse::<f64>() {
        return CellValue::Number(n);
    }
    if t.eq_ignore_ascii_case("true") {
        return CellValue::Bool(true);
    }
    if t.eq_ignore_ascii_case("false") {
        return CellValue::Bool(false);
    }
    CellValue::Text(t.to_string())
}

/// 电子表格视图根。
pub struct SheetView {
    name: SharedString,
    model: Model,
    path: Option<PathBuf>,
    dirty: bool,

    current_sheet: usize,
    /// 选中的单元格 (col, row)。
    selected: Option<(usize, usize)>,
    /// 是否处于编辑状态（编辑栏接管输入）。
    editing: bool,
    /// 编辑目标格 (col, row)：进入编辑时锁定，提交时以此为准，
    /// 避免 Blur 延后提交时选区已切换到别的格导致写错位置。
    edit_target: Option<(usize, usize)>,
    /// 编辑栏输入框（gpui-component 单线 Input，自带光标/选区/IME）。
    edit_input: Entity<InputState>,

    /// 横向滚动手柄：列标头（#grid-hscroll）使用；数据区横向滚动时手动读取其偏移绘制。
    hscroll: ScrollHandle,
    /// 纵向滚动手柄：行号列（#row-canvas）使用，作为冻结窗格纵向滚动的「主驱动源」。
    /// 用户在行号列区域滚动时，此 handle 更新，data_vscroll 被同步跟随。
    vscroll: ScrollHandle,
    /// 数据区专用纵向滚动手柄（#data-scroll 独占）。
    ///
    /// 🔴 为什么不能和行号列共享同一个 vscroll？
    /// GPUI 0.2.2 的 ScrollHandleState 是单值 `Rc<RefCell<...>>`（bounds/max_offset/offset 均
    /// last-writer-wins）。两个 overflow_y_scroll 容器 track 同一 handle 时，后布局的容器会
    /// 接管滚动状态 → 先布局的行号列正常、后布局的数据区空白/裁剪错位。
    ///
    /// 为什么不能不 track 任何 handle（留空/匿名）？
    /// overflow_y_scroll 容器即使不显式 track_scroll，GPUI 也会创建内部匿名 ScrollHandle。
    /// 这个内部 handle 的偏移我们无法读取也无法重置 → canvas paint 坐标与容器裁剪区
    /// 原点不对齐 → 顶部空白且与行号列错位不同量（诊断数据铁证：
    ///   paint y=0 正确但视觉上数据区被裁到 ~336px 处 vs 行号列 ~84px 处）。
    ///
    /// 正确做法：#data-scroll 有自己的 data_vscroll（独立于行号列的 vscroll），每帧由
    /// vscroll_zeroed 归零 + scroll_sync 同步。这样两个容器各管各的正规滚动上下文，
    /// GPUI 对每个的平移/裁剪都正确，不会冲突也不会失控。
    data_vscroll: ScrollHandle,
    /// 首帧是否已对 vscroll 初始偏移做过一次性归零（Fix B 兜底，避免每帧重置而影响用户滚动）。
    vscroll_zeroed: bool,
    /// 双向滚动同步用的「上次已知偏移」：行号列(vscroll) 与 数据区(data_vscroll) 各自记住
    /// 上一帧的偏移，render() 内比较哪个手柄发生了变化，就把变化镜像到另一个手柄。
    /// 用 last-known 判定方向，避免两个 canvas 同帧互设导致的「滚动回滚」竞争。
    last_vscroll_off: Point<Pixels>,
    /// 数据区手柄的上次已知偏移（见 `last_vscroll_off` 说明）。
    last_data_off: Point<Pixels>,
    /// 单元格文字 `ShapedLine` 缓存（独立 Entity，paint 闭包内安全访问，避免每帧重新 shape）。
    text_cache: Entity<GridTextCache>,

    focus: FocusHandle,
}

impl SheetView {
    /// 默认空白工作簿模型（一张 26×100 的空工作表，单元格稀疏存储）。
    pub fn default_model() -> Model {
        let sheet = Sheet {
            name: "Sheet1".to_string(),
            cells: HashMap::new(),
            cols: DEF_COLS,
            rows: DEF_ROWS,
        };
        Model::Sheet(Workbook {
            sheets: vec![sheet],
            ..Default::default()
        })
    }

    #[allow(dead_code)]
    pub fn new_blank(window: &mut Window, cx: &mut Context<Self>, name: SharedString) -> Self {
        Self::build(window, cx, name, Self::default_model(), None)
    }

    pub fn new_from_model(
        window: &mut Window,
        cx: &mut Context<Self>,
        name: SharedString,
        model: Model,
        path: Option<PathBuf>,
    ) -> Self {
        Self::build(window, cx, name, model, path)
    }

    fn build(
        window: &mut Window,
        cx: &mut Context<Self>,
        name: SharedString,
        model: Model,
        path: Option<PathBuf>,
    ) -> Self {
        // 单线 Input：自带光标 / 选区 / IME 合成 / 点击定位。创建需 &mut Window。
        let edit_input = cx.new(|cx| InputState::new(window, cx).multi_line(false));

        // 失焦即提交（例如点到别的格子 / 切走焦点时，把当前编辑内容落盘到单元格）。
        // 注意：Blur 事件是在 edit_input 自身的 update 周期内 emit 的，此刻
        // 直接 update(SheetView) 或 read(edit_input) 都会触发 GPUI 重入 panic
        // （"cannot update ... while it is already being updated"）。因此用
        // `defer` 把提交延后到当前 effect cycle 末尾，届时所有 lease 已释放。
        let this_entity = cx.entity();
        cx.subscribe(&edit_input, move |_this, _state, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Blur) {
                let entity = this_entity.clone();
                cx.defer(move |cx| {
                    let _ = entity.update(cx, |v, cx| v.commit_edit(cx));
                });
            }
        })
        .detach();

        let view = Self {
            name,
            model,
            path,
            dirty: false,
            current_sheet: 0,
            selected: Some((0, 0)),
            editing: false,
            edit_target: None,
            edit_input,
            hscroll: ScrollHandle::new(),
            vscroll: ScrollHandle::new(),
            data_vscroll: ScrollHandle::new(),
            vscroll_zeroed: false,
            last_vscroll_off: Point::default(),
            last_data_off: Point::default(),
            text_cache: cx.new(|_cx| GridTextCache::default()),
            focus: cx.focus_handle(),
        };

        // 挂载即把焦点给网格根，使方向键 / 直接打字进入编辑可用。
        view.focus.focus(window);
        view
    }

    fn workbook(&self) -> &Workbook {
        match &self.model {
            Model::Sheet(b) => b,
            _ => unreachable!("SheetView 只承载 Model::Sheet"),
        }
    }

    fn workbook_mut(&mut self) -> &mut Workbook {
        match &mut self.model {
            Model::Sheet(b) => b,
            _ => unreachable!("SheetView 只承载 Model::Sheet"),
        }
    }

    fn current_sheet(&self) -> &Sheet {
        let book = self.workbook();
        &book.sheets[self.current_sheet.min(book.sheets.len().saturating_sub(1))]
    }

    fn current_sheet_index(&self) -> usize {
        self.current_sheet
            .min(self.workbook().sheets.len().saturating_sub(1))
    }

    // ── 交互 ──

    /// 读取选中格的可编辑文本（无选中 / 空白格返回空串）。
    fn selected_raw_text(&self) -> String {
        match self.selected {
            Some((col, row)) => self
                .current_sheet()
                .cells
                .get(&row)
                .and_then(|m| m.get(&col))
                .map(|c| raw_cell_text(&c.value))
                .unwrap_or_default(),
            None => String::new(),
        }
    }

    /// 进入编辑：把选中格内容回填到编辑栏输入框并聚焦它。
    /// `initial` 为 `Some` 时（用户直接敲字符进入编辑）以该字符作为初值。
    fn begin_edit(&mut self, window: &mut Window, cx: &mut Context<Self>, initial: Option<String>) {
        if self.selected.is_none() {
            return;
        }
        let raw = match initial {
            Some(s) => s,
            None => self.selected_raw_text(),
        };
        self.editing = true;
        self.edit_target = self.selected;
        self.edit_input
            .update(cx, |s, cx| s.set_value(raw, window, cx));
        self.edit_input.update(cx, |s, cx| s.focus(window, cx));
        cx.notify();
    }

    /// 提交编辑：把编辑栏文本解析后写入选中格（按需动态创建稀疏单元格）。
    fn commit_edit(&mut self, cx: &mut Context<Self>) {
        if !self.editing {
            return;
        }
        // 提交目标以进入编辑时锁定的 edit_target 为准，而非当前选区。
        let (col, row) = match self.edit_target {
            Some(s) => s,
            None => {
                self.editing = false;
                return;
            }
        };
        let value = self.edit_input.read(cx).value().to_string();
        let cv = parse_cell_value(&value);
        let idx = self.current_sheet_index();
        self.workbook_mut().sheets[idx].set_cell(
            col,
            row,
            Cell {
                value: cv,
                style: TextStyle::default(),
            },
        );
        self.editing = false;
        self.edit_target = None;
        self.dirty = true;
        self.text_cache.update(cx, |cache, _cx| cache.invalidate_for_sheet());
        cx.notify();
    }

    /// 取消编辑：清空输入框、退出编辑态、焦点回到网格。
    fn cancel_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editing {
            return;
        }
        self.edit_input
            .update(cx, |s, cx| s.set_value("", window, cx));
        self.editing = false;
        self.edit_target = None;
        self.focus.focus(window);
        cx.notify();
    }

    /// 移动选区（非编辑态下的方向键）。超出范围则忽略。
    fn move_selection(&mut self, d_row: isize, d_col: isize, cx: &mut Context<Self>) {
        let (c, r) = self.selected.unwrap_or((0, 0));
        let nc = c as isize + d_col;
        let nr = r as isize + d_row;
        if nc >= 0 && nr >= 0 {
            self.selected = Some((nc as usize, nr as usize));
            cx.notify();
        }
    }

    /// 清空选中格（Delete / Backspace）。
    fn clear_selected(&mut self, cx: &mut Context<Self>) {
        if let Some((col, row)) = self.selected {
            let idx = self.current_sheet_index();
            self.workbook_mut().sheets[idx].set_cell(
                col,
                row,
                Cell {
                    value: CellValue::Empty,
                    style: TextStyle::default(),
                },
            );
            self.dirty = true;
            self.text_cache.update(cx, |cache, _cx| cache.invalidate_for_sheet());
            cx.notify();
        }
    }

    /// 键盘事件分发（根容器捕获，因 on_key_down 会向上冒泡，即使焦点在编辑栏也能收到）。
    fn on_key(
        &mut self,
        key: &str,
        text: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match key {
            "down" => {
                if !self.editing {
                    self.move_selection(1, 0, cx);
                }
            }
            "up" => {
                if !self.editing {
                    self.move_selection(-1, 0, cx);
                }
            }
            "left" => {
                if !self.editing {
                    self.move_selection(0, -1, cx);
                }
            }
            "right" => {
                if !self.editing {
                    self.move_selection(0, 1, cx);
                }
            }
            "enter" => {
                if self.editing {
                    self.commit_edit(cx);
                    self.focus.focus(window);
                    self.move_selection(1, 0, cx);
                } else {
                    self.begin_edit(window, cx, None);
                }
            }
            "f2" => {
                if !self.editing {
                    self.begin_edit(window, cx, None);
                }
            }
            "escape" => {
                if self.editing {
                    self.cancel_edit(window, cx);
                }
            }
            "delete" | "backspace" => {
                if !self.editing {
                    self.clear_selected(cx);
                }
            }
            _ => {
                // 非编辑态下直接敲字符 → 进入编辑并以该字符作为初值。
                if !self.editing {
                    if let Some(t) = text {
                        if !t.is_empty() {
                            self.begin_edit(window, cx, Some(t));
                        }
                    }
                }
            }
        }
    }

    fn select_cell(&mut self, col: usize, row: usize, cx: &mut Context<Self>) {
        self.selected = Some((col, row));
        cx.notify();
    }

    fn select_sheet(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.workbook().sheets.len() {
            self.current_sheet = index;
            self.selected = Some((0, 0));
            cx.notify();
        }
    }

    fn add_sheet(&mut self, cx: &mut Context<Self>) {
        let n = self.workbook().sheets.len() + 1;
        let sheet = Sheet {
            name: format!("Sheet{n}"),
            cells: HashMap::new(),
            cols: DEF_COLS,
            rows: DEF_ROWS,
        };
        let idx = self.workbook_mut().sheets.len();
        self.workbook_mut().sheets.push(sheet);
        self.current_sheet = idx;
        self.selected = Some((0, 0));
        self.dirty = true;
        cx.notify();
    }

    fn save_document(&mut self, cx: &mut Context<Self>) {
        let path = self.path.clone().unwrap_or_else(|| {
            let safe = self.name.replace(['/', '\\', ':'], "_");
            data::data_dir().join(format!("{safe}.ewp"))
        });

        if let Err(e) = crate::model::ser::save(&self.model, &path, NativeFormat::Json) {
            eprintln!("[EWP] Failed to save {}: {e}", path.display());
            return;
        }

        self.path = Some(path.clone());
        self.dirty = false;

        let mut app_data = data::load();
        data::add_recent_doc(
            &mut app_data,
            data::RecentDoc {
                name: self.name.to_string(),
                path: path.to_string_lossy().to_string(),
                file_type: data::FileType::Excel,
            },
        );
        cx.notify();
    }
}

impl Focusable for SheetView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for SheetView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();

        // ── Fix B（首帧归零）──
        // vscroll（行号列）和 data_vscroll（数据区）是两个独立 ScrollHandle，GPUI 可能在
        // 首帧布局时给它们赋非零初始偏移（尤其是内容 > 视口时）。此处一次性将两者
        // 都归零，确保首帧从顶部开始显示。用一次性标志避免每帧重置影响用户滚动。
        if !self.vscroll_zeroed {
            self.vscroll_zeroed = true;
            let off = self.vscroll.offset();
            let doff = self.data_vscroll.offset();
            if off.x != px(0.) || off.y != px(0.) {
                self.vscroll.set_offset(point(px(0.), px(0.)));
            }
            if doff.x != px(0.) || doff.y != px(0.) {
                self.data_vscroll.set_offset(point(px(0.), px(0.)));
            }
            self.last_vscroll_off = point(px(0.), px(0.));
            self.last_data_off = point(px(0.), px(0.));
        }

        // ── 双向滚动同步（Fix D）──
        // 行号列(vscroll) 与 数据区(data_vscroll) 是各自独立的纵向手柄，但冻结窗格要求两者同滚。
        // 用 last-known 判定方向：比较哪个手柄相对「上次已知偏移」发生了变化，就把变化镜像到对方。
        // 这样无论用户在行号列还是数据区滚动，另一侧都跟随；且同帧内只做一次镜像，不会互设回滚。
        let voff = self.vscroll.offset();
        let doff = self.data_vscroll.offset();
        if voff != self.last_vscroll_off {
            // 行号列是输入源（或上次镜像结果）→ 数据区跟随
            self.data_vscroll.set_offset(voff);
            self.last_vscroll_off = voff;
            self.last_data_off = voff;
            cx.notify();
        } else if doff != self.last_data_off {
            // 数据区是输入源 → 行号列跟随
            self.vscroll.set_offset(doff);
            self.last_data_off = doff;
            self.last_vscroll_off = doff;
            cx.notify();
        }

        let c = ThemeColors::current();
        let book = self.workbook();
        let sheet = self.current_sheet();
        // 数据区虚拟化闭包需要 owned 副本（闭包是 'static，且不能捕获 &self）。
        let data_cells = sheet.cells.clone();
        // 网格范围 = 声明范围(cols/rows) 与 实际已写入单元格边界 的较大者，且不低于默认空白网格。
        // 旧版 .ewp 未持久化 cols/rows（反序列化后为 0），若只取 max(1) 会塌缩成 1×1，
        // 表现为「单元格不显示」。这里按已写入的最大行列 +1 兜底，保证始终有可用的网格。
        let max_col = sheet
            .cells
            .values()
            .flat_map(|row_map| row_map.keys())
            .cloned()
            .max()
            .unwrap_or(0);
        let max_row = sheet.cells.keys().cloned().max().unwrap_or(0);
        let cols = DEF_COLS.max(sheet.cols).max(max_col + 1);
        let rows = DEF_ROWS.max(sheet.rows).max(max_row + 1);
        // 整表尺寸（canvas 用整表尺寸，靠滚动容器平移视口，每帧只画可见切片）。
        let total_w = cols as f32 * CELL_W;
        let total_h = rows as f32 * CELL_H;

        let editing = self.editing;
        let selected = self.selected;
        let addr = match self.selected {
            Some((col, row)) => format!("{} {}", col_name(col), row + 1),
            None => "—".to_string(),
        };
        let display = self.selected_raw_text();

        let title = if self.dirty {
            format!("{} *", self.name)
        } else {
            self.name.to_string()
        };

        div()
            .id("sheet-root")
            .size_full()
            .flex()
            .flex_col()
            .bg(c.window_bg)
            // 焦点恒在网格根：方向键 / 直接打字进入编辑都依赖它。
            .track_focus(&self.focus)
            // 点击网格任意处都把焦点交回根（即便编辑栏正聚焦，也会先触发失焦提交）。
            .on_mouse_down(MouseButton::Left, {
                let focus = self.focus.clone();
                move |_, window, _cx| {
                    focus.focus(window);
                }
            })
            // 键盘事件（冒泡到此处）：编辑态只拦截 enter/escape，其余交给编辑栏。
            .on_key_down({
                let this = this.clone();
                move |event: &KeyDownEvent, window, cx| {
                    let key = event.keystroke.key.clone();
                    let text = event.keystroke.key_char.clone();
                    let _ = this.update(cx, |v, cx| v.on_key(&key, text, window, cx));
                }
            })
            // ═══ 顶栏 ═══
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(16.))
                    .py(px(6.))
                    .bg(c.sidebar_bg)
                    .border_b_1()
                    .border_color(c.border)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(c.text_muted)
                            .child(SharedString::from(title)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .id("sheet-add")
                                    .flex()
                                    .items_center()
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .bg(c.button_bg)
                                    .text_color(c.text_primary)
                                    .text_sm()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(c.button_hover_bg))
                                    .child(SharedString::from("＋ 工作表"))
                                    .on_click({
                                        let t = this.clone();
                                        move |_, _, cx: &mut App| {
                                            let _ = t.update(cx, |v, cx| v.add_sheet(cx));
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .id("sheet-save")
                                    .flex()
                                    .items_center()
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .bg(if self.dirty { c.accent } else { c.button_bg })
                                    .text_color(if self.dirty {
                                        rgba(0xffffffff)
                                    } else {
                                        c.text_primary
                                    })
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .child(SharedString::from("保存"))
                                    .on_click({
                                        let t = this.clone();
                                        move |_, _, cx: &mut App| {
                                            let _ = t.update(cx, |v, cx| v.save_document(cx));
                                        }
                                    }),
                            ),
                    ),
            )
            // ═══ 编辑栏（formula bar）：新建 / 编辑单元格的唯一入口 ═══
            .child(
                div()
                    .id("formula-bar")
                    .flex()
                    .flex_row()
                    .items_center()
                    .h(px(30.))
                    .px(px(8.))
                    .gap_2()
                    .bg(c.sidebar_bg)
                    .border_b_1()
                    .border_color(c.border)
                    .child(
                        div()
                            .id("cell-addr")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(c.button_bg)
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(c.text_primary)
                            .child(SharedString::from(addr)),
                    )
                    .when(editing, |bar| {
                        // 编辑态：显示真正的输入框（IME 正常）。
                        bar.child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(Input::new(&self.edit_input).appearance(true)),
                        )
                    })
                    .when(!editing, |bar| {
                        // 非编辑态：显示选中格内容，点击即进入编辑。
                        bar.child(
                            div()
                                .id("cell-display")
                                .flex_1()
                                .min_w_0()
                                .px_2()
                                .text_sm()
                                .text_color(c.text_primary)
                                .cursor_pointer()
                                .child(SharedString::from(display))
                                .on_click({
                                    let this = this.clone();
                                    move |_, window, cx| {
                                        let _ =
                                            this.update(cx, |v, cx| v.begin_edit(window, cx, None));
                                    }
                                }),
                        )
                    }),
            )
            // ═══ 网格主体（冻结窗格，参照 LibreOffice Calc 的 4 象限模型） ═══
            //   ┌─────────┬──────────────────────────┐
            //   │ 角(固定)│ 列标头（仅横向滚，冻结顶） │
            //   ├─────────┼──────────────────────────┤
            //   │行号(纵滚)│ 数据区（双轴滚，滚动渲染） │
            //   └─────────┴──────────────────────────┘
            // 滚动容器与 ScrollHandle 关系（Fix D 后，双独立纵向手柄 + 双向同步）：
            //   row-canvas   → track vscroll（行号纵向滚动，手柄 A）
            //   grid-hscroll → track hscroll（列标头横向滚动，DOM 字母）
            //   data-scroll  → track data_vscroll（数据区纵向滚动，手柄 B，独立不共享）
            //   两手柄每帧在 render() 内双向同步：任一变化都镜像到对方，使行号列与数据区始终同滚。
            //   关键修正：#data-scroll 必须 track 一个**正规** ScrollHandle（data_vscroll），
            //   不能共享 vscroll（last-writer-wins 冲突），也不能留空（GPUI 生成失控的匿名内部手柄）。
            .child(
                div()
                    .id("sheet-body")
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    .bg(c.content_bg)
                    // ── 左侧固定列：左上角 + 行号列（纵向随数据滚动，横向不动） ──
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(HEADER_W))
                            .flex_shrink_0()
                            // 左上角（双轴都不动）
                            .child(
                                div()
                                    .w(px(HEADER_W))
                                    .h(px(COL_HEADER_H))
                                    .flex_shrink_0()
                                    .border_r_1()
                                    .border_b_1()
                                    .border_color(c.border)
                                    .bg(c.sidebar_bg),
                            )
                            // 行号列：单 canvas 命令式绘制可见行号，与数据区共享 vscroll → 纵向同步
                            .child(
                                div()
                                    .id("row-canvas")
                                    .flex_1()
                                    .min_h_0()
                                    .overflow_y_scroll()
                                    .track_scroll(&self.vscroll)
                                    .child(
                                        canvas(
                                            {
                                                let v = self.vscroll.clone();
                                            move |_b, _w, _cx| {
                                                let vp = v.bounds().size;
                                                let voff = v.offset();
                                                let _viewport_w: f32 = vp.width.into();
                                                let viewport_h: f32 = vp.height.into();
                                                let scrolled_y: f32 = (-voff.y).into();
                                                compute_visible_window(
                                                    HEADER_W, viewport_h, 0.0, scrolled_y, 1, rows,
                                                )
                                            }
                                            },
                                            {
                                                let theme = c;
                                                let selected = selected;
                                                move |_b, win: VisibleWindow, window, cx| {
                                                    window.paint_quad(gpui::fill(_b, theme.sidebar_bg));
                                                    // Y 使用真实 content 坐标 row_top(r)。行号 canvas 是
                                                    // #row-canvas(track_scroll(vscroll)) 的子元素，GPUI 已通过
                                                    // with_element_offset() 把整块内容按 vscroll.offset() 平移，
                                                    // 因此 paint 无需、也不应再手动减 scroll_y（否则双重计数 →
                                                    // 顶部空白 + 点击错位）。r0..r1 裁剪由 compute_visible_window 算出。
                                                    for r in win.r0..win.r1 {
                                                        let y = row_top(r);
                                                        paint_row_number(
                                                            window,
                                                            cx,
                                                            y,
                                                            r,
                                                            selected.map(|(_, sr)| sr) == Some(r),
                                                            &theme,
                                                        );
                                                    }
                                                }
                                            },
                                        )
                                        .w(px(HEADER_W))
                                        .h(px(total_h)),
                                    ),
                            ),
                    )
                    // ── 右侧滚动区：列标头（横向滚）+ 数据区（纵向滚，横向手动） ──
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w_0()
                            // 列标头：横向滚动容器，track hscroll（DOM 字母），与数据区像素级对齐
                            .child(
                                div()
                                    .id("grid-hscroll")
                                    .h(px(COL_HEADER_H))
                                    .flex_1()
                                    .min_w_0()
                                    .flex_shrink_0()
                                    .overflow_x_scroll()
                                    .overflow_y_hidden()
                                    .track_scroll(&self.hscroll)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .w(px(cols as f32 * CELL_W))
                                            .h(px(COL_HEADER_H))
                                            .flex_shrink_0()
                                            .children((0..cols).map(|col| {
                                                let is_sel =
                                                    self.selected.map(|(sc, _)| sc) == Some(col);
                                                div()
                                                    .w(px(CELL_W))
                                                    .h(px(COL_HEADER_H))
                                                    .flex_shrink_0()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .border_r_1()
                                                    .border_b_1()
                                                    .border_color(c.border)
                                                    .bg(if is_sel {
                                                        rgba(0x0a84ff18)
                                                    } else {
                                                        c.sidebar_bg
                                                    })
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(if is_sel {
                                                        c.accent
                                                    } else {
                                                        c.text_muted
                                                    })
                                                    .child(SharedString::from(col_name(col)))
                                            })),
                                    ),
                            )
                            // 数据区：单 canvas 命令式绘制可见切片，每帧零单元格 DOM 节点
                            .child(
                                div()
                                    .id("data-scroll")
                                    .flex_1()
                                    .min_h_0()
                                    // 🔴 Fix D（根因修复）：#data-scroll 必须 track 一个**正规**的独立
                                    // ScrollHandle（data_vscroll），而不是留空产生失控的匿名内部手柄。
                                    // 有了正规手柄后，GPUI 对本容器的平移/裁剪都基于 data_vscroll.offset()，
                                    // canvas 的 paint 坐标系与裁剪原点严格对齐 → 顶部空白与行号错位消失。
                                    // 横向仍 hidden（由 #grid-hscroll 的 hscroll 管理，数据区不横向滚）。
                                    .overflow_y_scroll()
                                    .overflow_x_hidden()
                                    .track_scroll(&self.data_vscroll)
                                    .on_mouse_down(MouseButton::Left, {
                                        let this = this.clone();
                                        let h = self.hscroll.clone();
                                        // #data-scroll 现在 track 自己的 data_vscroll（独立手柄，render() 内
                                        // 与 vscroll 双向同步）。data-scroll 容器**自身**的 bounds 不被
                                        // with_element_offset 平移（平移只作用于其子 canvas），因此 on_mouse_down
                                        // 收到的 event.position 仍是视口坐标，Y 必须手动加回纵向滚动偏移
                                        // scrolled_y = -data_vscroll.offset().y（X 轴同理，横向由 #grid-hscroll
                                        // 的 hscroll 管理）。若不加则命中行会比实际偏低 → 点击错位。
                                        let v = self.data_vscroll.clone();
                                        let cols = cols;
                                        let rows = rows;
                                        move |event: &MouseDownEvent, window: &mut Window,
                                              cx: &mut App| {
                                            let scrolled_x: f32 = f32::from(-h.offset().x);
                                            let scrolled_y: f32 = f32::from(-v.offset().y);
                                            let content_x: f32 =
                                                f32::from(event.position.x) + scrolled_x;
                                            // 事件坐标是视口坐标，加 scrolled_y 还原成内容坐标。
                                            let content_y: f32 =
                                                f32::from(event.position.y) + scrolled_y;
                                            let mut c = 0usize;
                                            let mut x = 0.0;
                                            while c + 1 < cols && x + col_width(c) <= content_x {
                                                x += col_width(c);
                                                c += 1;
                                            }
                                            let mut r = 0usize;
                                            let mut y = 0.0;
                                            while r + 1 < rows && y + row_height(r) <= content_y {
                                                y += row_height(r);
                                                r += 1;
                                            }
                                            let entity = this.clone();
                                            if event.click_count >= 2 {
                                                entity.update(cx, |v, cx| {
                                                    v.begin_edit(window, cx, None)
                                                });
                                            } else {
                                                entity.update(cx, |v, cx| {
                                                    v.select_cell(c, r, cx)
                                                });
                                            }
                                        }
                                    })
                                    .child(
                                        canvas(
                                            {
                                                let h = self.hscroll.clone();
                                                let v = self.data_vscroll.clone();
                                                move |_b, _w, _cx| {
                                                    let hoff = h.offset();
                                                    let voff = v.offset();
                                                    let vp_h = v.bounds().size.height.into();
                                                    let scrolled_x: f32 = (-hoff.x).into();
                                                    let scrolled_y: f32 = (-voff.y).into();
                                                    compute_visible_window(
                                                        f32::MAX, vp_h, scrolled_x, scrolled_y, cols, rows,
                                                    )
                                                }
                                            },
                                            {
                                                // text_cache 当前未在此闭包内使用（文字走 window.text_system().shape_line），
                                                // 保留克隆以避免改动结构；前缀 _ 抑制 unused 警告。
                                                let _cache = self.text_cache.clone();
                                                let theme = c;
                                                let selected = selected;
                                                let editing = editing;
                                                let edit_target = self.edit_target;
                                                let cells = data_cells.clone();
                                                // 滚动同步所需：vscroll / data_vscroll 当前偏移 + 自身 Entity（用于 notify 整视图重绘）。
                                                let v = self.vscroll.clone();
                                                let dv = self.data_vscroll.clone();
                                                let this = this.clone();
                                                move |_b, win: VisibleWindow, window: &mut Window, cx: &mut App| {
                                                    // 滚动同步触发：#data-scroll 现 track 自己的 data_vscroll，GPUI 会在
                                                    // data_vscroll 变化时自动重绘本 canvas；但行号列滚动（vscroll 变化）或
                                                    // 数据区反向滚动不会自动双向反映。这里检测两纵向手柄是否一致，不一致就 notify，
                                                    // 真正的镜像在 render() 内用 last-known 方向检测完成，避免两 canvas 同帧互设回滚。
                                                    if v.offset() != dv.offset() {
                                                        this.update(cx, |_view, cx2| {
                                                            cx2.notify();
                                                        });
                                                    }
                                                    window.paint_quad(gpui::fill(_b, theme.content_bg));
                                                    // X: col_left(c) - win.scroll_x 抵消数据区不参与横向滚动
                                                    //   （横向由 #grid-hscroll 的 hscroll 管理）导致的手动横向偏移。
                                                    // Y: #data-scroll 现 track data_vscroll，GPUI 通过 with_element_offset 把本
                                                    //   canvas 按 data_vscroll.offset() 自动平移，故直接画内容坐标 row_top(r)
                                                    //   （绝不能手动减 win.scroll_y，否则双重计数 → 顶部空白）。
                                                    //   r0..r1 由 compute_visible_window 按 scrolled_y 算出。
                                                    for r in win.r0..win.r1 {
                                                        for c in win.c0..win.c1 {
                                                            let x = col_left(c) - win.scroll_x;
                                                            let y = row_top(r);
                                                            let cb = Bounds::new(
                                                                point(px(x), px(y)),
                                                                size(px(CELL_W), px(CELL_H)),
                                                            );
                                                            let is_sel = selected == Some((c, r));
                                                            paint_cell_background(
                                                                window, cx, cb, is_sel, &theme,
                                                            );
                                                            // 编辑中的目标格在 canvas 上留白，文字走公式栏。
                                                            let is_editing_this = editing
                                                                && edit_target == Some((c, r));
                                                            if !is_editing_this {
                                                                if let Some(text) = cells
                                                                    .get(&r)
                                                                    .and_then(|m| m.get(&c))
                                                                    .map(|cell| format_cell(&cell.value))
                                                                    .filter(|s| !s.is_empty())
                                                                {
                                                                let origin = point(
                                                                    px(x + CELL_PAD),
                                                                    px(y + (CELL_H - CELL_FONT_SIZE) / 2.0),
                                                                );
                                                                let font = window.text_style().font();
                                                                let run = TextRun {
                                                                    len: text.len(),
                                                                    font,
                                                                    color: theme.text_primary.into(),
                                                                    background_color: None,
                                                                    underline: None,
                                                                    strikethrough: None,
                                                                };
                                                                // FIX: force_width=None — GPUI's layout_line
                                                                // snaps each glyph to glyph_pos*force_width grid,
                                                                // spreading "format" across 460px (5 columns!).
                                                                // Only use force_width for tabular/mono content.
                                                                let shaped = window.text_system().shape_line(
                                                                    SharedString::from(text.to_string()),
                                                                    px(crate::sheet_grid::CELL_FONT_SIZE),
                                                                    &[run],
                                                                    None,
                                                                );
                                                                let _ = shaped.paint(
                                                                    origin, px(crate::sheet_grid::CELL_H), window, cx,
                                                                );
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                        )
                                        .w(px(total_w))
                                        .h(px(total_h)),
                                    ),
                            ),
                    ),
            )
            // ═══ 底部 sheet 标签 ═══
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px(px(8.))
                    .py(px(4.))
                    .bg(c.sidebar_bg)
                    .border_t_1()
                    .border_color(c.border)
                    .gap_1()
                    .children(book.sheets.iter().enumerate().map(|(i, s)| {
                        let is_active = i == self.current_sheet;
                        let tab = this.clone();
                        div()
                            .id(SharedString::from(format!("tab-{i}")))
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(if is_active { c.accent } else { c.button_bg })
                            .text_sm()
                            .text_color(if is_active {
                                rgba(0xffffffff)
                            } else {
                                c.text_primary
                            })
                            .cursor_pointer()
                            .hover(|d| d.bg(if is_active { c.accent } else { c.button_hover_bg }))
                            .child(SharedString::from(s.name.clone()))
                            .on_click(move |_, _, cx: &mut App| {
                                let _ = tab.update(cx, |v, cx| v.select_sheet(i, cx));
                            })
                    }))
                    .child(
                        div()
                            .id("sheet-tab-add")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_color(c.text_muted)
                            .hover(|d| d.bg(c.button_hover_bg))
                            .child(SharedString::from("＋"))
                            .on_click({
                                let t = this.clone();
                                move |_, _, cx: &mut App| {
                                    let _ = t.update(cx, |v, cx| v.add_sheet(cx));
                                }
                            }),
                    ),
            )
            // ═══ 状态栏 ═══
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px(px(16.))
                    .py(px(5.))
                    .bg(c.sidebar_bg)
                    .border_t_1()
                    .border_color(c.border)
                    .text_xs()
                    .text_color(c.text_muted)
                    .child(SharedString::from(match self.selected {
                        Some((col, row)) => format!("{} {}", col_name(col), row + 1),
                        None => "—".to_string(),
                    }))
                    .when(self.editing, |bar| {
                        bar.child(SharedString::from("  ·  编辑中"))
                    }),
            )
    }
}

