//! 电子表格视图 —— Calc / Excel 风格（可滚动网格 + 列标行号 + 底部 sheet 标签）。
//!
//! 布局（从外到内）：
//! ┌──────────────────────────────────────────────────────────┐
//! │  [框架顶部栏：文件名 · ＋工作表 · 保存]                      │  ← UiLayoutManager 统一套 chrome
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
//! 模型层（`sheet::model`）已定义：
//!   Workbook { sheets: Vec<Sheet> }
//!   Sheet    { name, cells: HashMap<usize, HashMap<usize, Cell>> (稀疏), cols, rows }
//!   Cell     { value: CellValue, style: TextStyle }
//!   CellValue{ Empty | Number(f64) | Text(String) | Bool | Formula(String) }
//!
//! 单元格采用**稀疏存储**：空白格在内存里不存在。本视图提供「编辑栏」作为
//! 唯一的写入入口 —— 选中格子后，在编辑栏输入并回车，`set_cell` 会按需动态
//! 创建该格（仿 Apple Numbers 的动态网格）。这就是「用户如何新建单元格」。
//!
//! ───────────────────────────────────────────────────────────────
//! 重写说明（消除"冻结窗格/滚动坐标错位"）：
//! 旧版用两套坐标机制——列标头靠 GPUI `overflow_x_scroll`（DOM 平移）+
//! `track_scroll(hscroll)`，数据区靠 `col_left(c) - scroll_x` 手算 X，纵向又靠
//! GPUI 注入的窗口原点动态偏移 + `row_top(r) - scroll_y` 补丁。
//! 两套机制漂移 → 错位。
//!
//! 新版（LibreOffice Calc 架构的干净移植）：删除全部 `ScrollHandle` /
//! `overflow_*_scroll` / `track_scroll` / 窗口原点动态补偿补丁，改为**单一铺满网格视口
//! 的 `canvas`** + 集中状态 `SheetViewState`，四区域（角 / 列标头 / 行号 / 数据）
//! 由**同一个 paint 闭包**用**同一套公式**绘制：
//!   列标头 X 与数据格 X 同一表达式 `HEADER_W + col_left(c) - scroll_x`
//!   行号   Y 与数据格 Y 同一表达式 `COL_HEADER_H + row_top(r) - scroll_y`
//! 二者数学同源，结构上不可能再错位。
//!
//! 顶部 chrome 由 `UiLayoutManager` 调度：标准模式 = `StandardToolbar`，
//! 标签页式 = `TabbedLayout`。**本文件内部的 sheet-body / 单 canvas / 裁剪带 /
//! 坐标公式绝不动**（见 `docs/MEMORY.md` Sheet 视图架构，🔴 红线）。

// mirrors LibreOffice: 本视图对应 `sc::ScTabView` 的网格区；顶部 chrome 由
// `sfx2::SfxNotebookBar` 调度的 toolbar 提供（见 `ui/layout.rs` / `ui/standard.rs`）。
use gpui::{
    App, Bounds, ClickEvent, ContentMask, Context, Entity, FocusHandle, Focusable,
    FontWeight, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Render,
    ScrollWheelEvent, SharedString, Window, canvas, div, point, px, rgba, size,
};
use gpui::prelude::*;
use gpui_component::input::{Input, InputEvent, InputState};
use std::collections::HashMap;
use std::rc::Rc;

use crate::data;
use crate::model::common::TextStyle;
use crate::model::ser::NativeFormat;
use crate::model::Model;
use crate::sheet::grid::*;
use crate::sheet::grid_cache::GridTextCache;
use crate::sheet::model::{Cell, CellValue, Sheet, Workbook};
use crate::sheet::scrollbar::*;
use crate::sheet::status_bar::*;
use crate::sheet::view_state::{Pane, SheetViewState};
use crate::styles::ThemeColors;
use crate::ui::layout::{ChromeCtx, ModelKind};
use crate::ui::manager::UiLayoutManager;
use std::path::PathBuf;

// 默认网格尺寸（空白工作簿）。
const DEF_COLS: usize = 26;
const DEF_ROWS: usize = 100;

/// 滚轮方向校准常量。
///
/// GPUI 的 `ScrollWheelEvent.delta` 在「向下滚」手势下为负值（见 `gpui-0.2.2`
/// `ListState::scroll` 的 `new_scroll_top = scroll_top - delta.y` 约定）。我们的
/// `scroll_y` 增大 == 向下滚（数据格上移、露出下方行），故对调滚轮符号。
/// 若实测方向相反（向下滚反而上移），把此常量改为 `1.0` 即可。
const WHEEL_SIGN: f32 = -1.0;

/// 滚动条拖拽轴（横向 / 纵向）。
/// 与 `FreezeAxis` 区分：滚动是改 `scroll_x/y`，冻结是改 `frozen_cols/rows`。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScrollAxis {
    H,
    V,
}

/// 冻结拖拽轴（列 / 行）。
/// 命中测试在列标头带右缘 / 行号带下缘热区命中后，拖拽反算列/行写入
/// `SheetViewState.frozen_cols/rows`（≈ LibreOffice `ScHSplitWindow`/`ScVSplitWindow` 的 `SplitHdl`）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FreezeAxis {
    Col,
    Row,
}

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

    /// 集中视图状态：滚动 / 冻结 / 缩放的唯一真相源（类比 LibreOffice ScViewData）。
    /// 删除旧版的 hscroll / vscroll 两个 ScrollHandle，所有坐标真相都在这里。
    state: SheetViewState,
    /// canvas 数据区视口尺寸（已扣表头），由 prepaint 每帧记录，供滚轮 clamp 使用。
    viewport_w: f32,
    viewport_h: f32,
    /// 整表像素尺寸（内容宽/高，已扣表头），每帧由 render 记录，供滚动条组件读取比例。
    total_w: f32,
    total_h: f32,
    /// canvas 在窗口坐标系中的左上角原点，由 prepaint 每帧记录，供命中测试换算。
    canvas_ox: f32,
    canvas_oy: f32,
    /// 滚动条拖拽瞬时态（非真相源，不放 `SheetViewState`）：横/纵滚动条拖拽中
    /// 记录指针在 thumb 上的抓取偏移（局部坐标），松手置 `None`。
    hscroll_drag: Option<f32>,
    vscroll_drag: Option<f32>,
    /// 冻结拖拽瞬时态：`Some(Col/Row)` 表示正在拖拽冻结手柄，松手置 `None`。
    freeze_drag: Option<FreezeAxis>,
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
            state: SheetViewState::new(),
            viewport_w: 0.0,
            viewport_h: 0.0,
            total_w: 0.0,
            total_h: 0.0,
            canvas_ox: 0.0,
            canvas_oy: 0.0,
            hscroll_drag: None,
            vscroll_drag: None,
            freeze_drag: None,
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();

        let c = ThemeColors::current();
        let sheet = self.current_sheet();
        // 数据区虚拟化闭包需要 owned 副本（闭包是 'static，且不能捕获 &self）。
        let data_cells = sheet.cells.clone();
        // 网格范围 = 声明范围(cols/rows) 与 实际已写入单元格边界 的较大者，且不低于默认空白网格。
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
        // 整表尺寸（canvas 用整表尺寸推算可见窗口，每帧只画可见切片）。
        let total_w = col_left(cols);
        let total_h = row_top(rows);
        // 记录整表尺寸供滚动条组件读取比例（T02；每帧记录，红线：只经 state 派生）。
        // 必须在 `book` 的长借用在途之前完成本可变赋值（NLL：借用在途不可变赋值）。
        self.total_w = total_w;
        self.total_h = total_h;

        let book = self.workbook();

        let editing = self.editing;
        let selected = self.selected;
        let edit_target = self.edit_target;
        let addr = match self.selected {
            Some((col, row)) => format!("{} {}", col_name(col), row + 1),
            None => "—".to_string(),
        };
        let display = self.selected_raw_text();
        // T03：底部状态栏只读派生模型（纯展示、零副作用，红线：状态栏不写状态 / 不 notify）。
        let status_model = self.derive_status_bar();

        // 快照状态与网格范围，供 canvas 的 measure/paint 两个闭包使用（同源）。
        let state = self.state;
        let cols_snap = cols;
        let rows_snap = rows;
        let total_w_snap = total_w;
        let total_h_snap = total_h;
        let this_entity = this.clone();
        let theme = c;

        // 顶部 chrome 回调（保存 / 侧栏[表格无] / 格式[表格无] / 文档类型切换）。
        let on_save: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> = {
            let this = this.clone();
            Rc::new(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                let this = this.clone();
                let _ = this.update(cx, |v, cx| v.save_document(cx));
            })
        };
        // 表格 / 演示无侧栏：no-op（决策⑥）。
        let on_toggle_sidebar: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> =
            Rc::new(|_: &ClickEvent, _: &mut Window, _: &mut App| {});
        let on_format: Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static> =
            Rc::new(|_: &str, _: &ClickEvent, _: &mut Window, _: &mut App| {});
        // 文档类型切换（Tabbed 的 tab 点击触发）：打开对应类型的新窗口（决策⑤）。
        let on_switch_model: Rc<dyn Fn(ModelKind, &mut Window, &mut App) + 'static> = {
            Rc::new(move |kind: ModelKind, _window: &mut Window, cx: &mut App| {
                // mirrors LibreOffice: 切到另一 module（Writer/Calc/Impress）即开对应文档窗口。
                let model = match kind {
                    ModelKind::Text => Model::Text(crate::text::model::Document::default()),
                    ModelKind::Sheet => Model::Sheet(crate::sheet::model::Workbook::default()),
                    ModelKind::Slide => Model::Slide(crate::slide::model::Presentation::default()),
                };
                crate::open_editor(cx, "Untitled".into(), Some(model), None);
            })
        };

        // 中间工具按钮组（＋工作表），交给 StandardToolbar 嵌进统一框。
        let tool_group = {
            let c = ThemeColors::current();
            let this = this.clone();
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
                .into_any_element()
        };

        // ── 滚动条拖拽回调（T02）：把目标 scroll 经 SheetViewState 写回，绝不移动窗口几何 ──
        // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::ScrollHdl → SetPosX/Y + Invalidate
        let this_h = this.clone();
        let hscroll_on_drag: Rc<dyn Fn(f32, &mut Window, &mut App)> = Rc::new(move |v, _window, cx| {
            let _ = this_h.update(cx, |view, cx| view.on_scrollbar_drag(ScrollAxis::H, v, cx));
        });
        let this_hb = this.clone();
        let hscroll_on_begin: Rc<dyn Fn(f32, &mut Window, &mut App)> =
            Rc::new(move |grab, _window, cx| {
                let _ = this_hb
                    .update(cx, |view, cx| view.on_scrollbar_drag_begin(ScrollAxis::H, grab, cx));
            });
        let this_he = this.clone();
        let hscroll_on_end: Rc<dyn Fn(&mut Window, &mut App)> =
            Rc::new(move |_window, cx| {
                let _ = this_he.update(cx, |view, cx| view.on_scrollbar_drag_end(ScrollAxis::H, cx));
            });
        let this_v = this.clone();
        let vscroll_on_drag: Rc<dyn Fn(f32, &mut Window, &mut App)> = Rc::new(move |v, _window, cx| {
            let _ = this_v.update(cx, |view, cx| view.on_scrollbar_drag(ScrollAxis::V, v, cx));
        });
        let this_vb = this.clone();
        let vscroll_on_begin: Rc<dyn Fn(f32, &mut Window, &mut App)> =
            Rc::new(move |grab, _window, cx| {
                let _ = this_vb
                    .update(cx, |view, cx| view.on_scrollbar_drag_begin(ScrollAxis::V, grab, cx));
            });
        let this_ve = this.clone();
        let vscroll_on_end: Rc<dyn Fn(&mut Window, &mut App)> =
            Rc::new(move |_window, cx| {
                let _ = this_ve.update(cx, |view, cx| view.on_scrollbar_drag_end(ScrollAxis::V, cx));
            });

        // ── 文档内容 body（不含顶部栏，由框架套 chrome 后放在下方）──
        // 焦点恒在网格根：方向键 / 直接打字进入编辑都依赖它。
        let body = div()
            .id("sheet-root")
            .size_full()
            .min_h_0()
            .flex()
            .flex_col()
            .bg(c.window_bg)
            .track_focus(&self.focus)
            // 点击网格任意处都把焦点交回根（即便编辑栏正聚焦，也会先触发失焦提交）。
            .on_mouse_down(MouseButton::Left, {
                let focus = self.focus.clone();
                move |_, window, _cx| {
                    focus.focus(window);
                }
            })
                    // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView 键盘分派（方向键/F2/Enter/Delete，经 SfxBindings 动作）
                    //   C++：ScTabView 把键盘事件路由到光标移动 / 编辑；Enter 进入编辑、F2 编辑、方向键移光标、Delete 清格（≈ SID 动作）。
                    //   Rust 逐行对应（见 on_key 分发）：
                    //     "down"/"up"/"left"/"right" → move_selection(±1/±1,0...)   // ≈ 方向键移光标（非编辑态）
                    //     "enter" → if editing { commit_edit; move_selection(1,0) } else { begin_edit }  // ≈ Enter 进入/提交编辑
                    //     "f2"   → begin_edit                                    // ≈ F2 编辑
                    //     "escape" → cancel_edit                               // ≈ Esc 取消编辑
                    //     "delete"/"backspace" → clear_selected                // ≈ Delete 清格
                    //     _ => 直接敲字符 → begin_edit(Some(text))              // ≈ 打字即进入编辑
                    //   偏差核对：编辑态下键盘被编辑栏 Input 接管（不冒泡到根），非编辑态由根 on_key_down 捕获（冒泡）。
                    // 键盘事件（冒泡到此处）：编辑态只拦截 enter/escape，其余交给编辑栏。
                    .on_key_down({
                let this = this.clone();
                move |event: &KeyDownEvent, window, cx| {
                    let key = event.keystroke.key.clone();
                    let text = event.keystroke.key_char.clone();
                    let _ = this.update(cx, |v, cx| v.on_key(&key, text, window, cx));
                }
            })
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
                            // 用 content_bg 代替偏亮的 button_bg，使其与编辑框同色（inset 风格），
                            // 避免深色主题下 cell-addr 比公式栏更亮而突兀。
                            .bg(c.content_bg)
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
                                // 关键修复：gpui-component 的 `Input` 在 `appearance(true)`
                                // 下背景取 `cx.theme().background`，而 EWP 把 gpui-component 的主题
                                // 模式绑定到「纸张亮度」——深色纸张 → 暗色模式 → 背景为 gpui-component
                                // 自带的近黑 `#0a0a0a`，与 EWP 公式栏 `sidebar_bg(#252526)` 明显不符，
                                // 在深色主题下显得突兀。这里用 `.bg()/.border_color()` 通过
                                // `refine_style` 覆盖 gpui-component 自带外观，让编辑框跟随 EWP 主题。
                                .child(
                                    Input::new(&self.edit_input)
                                        .appearance(true)
                                        .bg(c.content_bg)
                                        .border_color(c.border),
                                ),
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
            // ═══ 网格主体：单一铺满视口的 canvas（四区域同源绘制） ═══
            //  ┌─────────┬──────────────────────────┐
            //  │ 角(固定)│ 列标头（仅横向滚）         │
            //  ├─────────┼──────────────────────────┤
            //  │行号(纵滚)│ 数据区（双轴滚）           │
            //  └─────────┴──────────────────────────┘
            //
            // 坐标公式（同源，杜绝错位）：
            //   角        : (canvas_ox,            canvas_oy)
            //   列标头 col: (canvas_ox + HEADER_W + col_left(c) - scroll_x,  canvas_oy)
            //   行号   row: (canvas_ox,            canvas_oy + COL_HEADER_H + row_top(r) - scroll_y)
            //   数据格(c,r):(canvas_ox + HEADER_W + col_left(c) - scroll_x,  canvas_oy + COL_HEADER_H + row_top(r) - scroll_y)
            // canvas_ox/oy 是 canvas 在窗口中的固定原点（由 prepaint 记录），不是滚动补丁；
            // 列标头 X 与数据 X、行号 Y 与数据 Y 用的是同一对表达式，不可能漂移。
            .child(
                div()
                    .id("sheet-body")
                    .flex_1()
                    .min_h_0()
                    // 必须仍是 flex 容器：canvas 用 `.flex_1()` 撑满高度。
                    // 若此处是纯 block，canvas 的 `.size_full()`（height:100%）会向
                    // 「由 flex-grow 分配来的高度」请求百分比高度，而 GPUI/taffy 不把
                    // flex item 的 grow 高度视为确定高度，导致 canvas 高度塌 0、measure
                    // 返回 0、整片网格空白（见 v2 bugfix）。故保留 `.flex().flex_col()`，
                    // 与 text/editor_view 的「每级既是 flex item 又是 flex container」一致。
                    .flex()
                    .flex_col()
                    .bg(c.content_bg)
                    // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::ScrollX / ScrollY → aViewData.SetPosX/Y + Invalidate
                    //   C++（研究文档 §5.2，逐行）：
                    //     用户滚动 / 拖滚动条 → ScrollHdl → pViewData->Scroll(...)   // 写 nPosX[eWhich]=newAnchor; rem=新余数
                    //                                 → pView->Scroll(...)            // 同步滚动条 thumb
                    //                                 → Invalidate(pane)              // 标记重绘 → 下一帧 Paint 用新 anchor+rem
                    //   Rust 逐行对应：
                    //     let p = event.delta.pixel_delta(px(CELL_H));   // 滚轮 delta → 像素位移（≈ 滚动条 thumb 位移）
                    //     let dx = p.x.into(); let dy = p.y.into();
                    //     v.state.scroll_by(WHEEL_SIGN*dx, WHEEL_SIGN*dy, v.viewport_w, v.viewport_h, tw, th);
                    //        // ≈ Scroll：改 scroll(锚点+余数)；内部 clamp（≈ 合法范围）
                    //     cx.notify();   // ≈ Invalidate(pane)：标记 SheetView 重绘，下一帧 render 用新 state
                    //   偏差核对：EWP 无独立 ScrollBar 控件（v1 用 on_wheel 代，滚动条为扩展点，红线 §1.4.4）；
                    //            WHEEL_SIGN=-1 校准 GPUI delta 符号（向下滚手势 delta 为负 → scroll_y 增大=向下）。
                    // 滚轮滚动：更新集中状态后重绘（clamp 提供硬边界）。
                    .on_scroll_wheel({
                        let this = this.clone();
                        let tw = total_w_snap;
                        let th = total_h_snap;
                        move |event: &ScrollWheelEvent, _window, cx| {
                            let p = event.delta.pixel_delta(px(CELL_H));
                            let dx: f32 = p.x.into();
                            let dy: f32 = p.y.into();
                            let _ = this.update(cx, |v, cx| {
                                v.state.scroll_by(
                                    WHEEL_SIGN * dx,
                                    WHEEL_SIGN * dy,
                                    v.viewport_w,
                                    v.viewport_h,
                                    tw,
                                    th,
                                );
                                cx.notify();
                            });
                        }
                    })
                    // mirrors LibreOffice: sc/source/ui/view/gridwin.cxx — ScGridWindow::MouseButtonDown → HandleMouseButtonDown → GetPosFromPixel 命中测试
                    //   C++ 逐字（HandleMouseButtonDown 头部）：
                    //     bool bDouble = (rMEvt.GetClicks() == 2);          // 双击检测
                    //     Point aPos = rMEvt.GetPosPixel();
                    //     SCCOL nPosX; SCROW nPosY;
                    //     mrViewData.GetPosFromPixel(aPos.X(), aPos.Y(), eWhich, nPosX, nPosY);  // 像素→单元格
                    //   C++ 逐字（MouseButtonUp 双击编辑分支）：
                    //     if (rMEvt.GetClicks()==2 && rMEvt.IsLeft() && !bRefMode ...) {
                    //         pScMod->SetInputMode(SC_INPUT_TABLE);   // 进入单元格编辑
                    //     }
                    //   Rust 逐行对应：
                    //     let pos_x = event.position.x.into(); let pos_y = event.position.y.into();  // ≈ aPos = GetPosPixel()
                    //     let click_count = event.click_count;                                       // ≈ GetClicks()
                    //     let local_x = pos_x - canvas_ox; let local_y = pos_y - canvas_oy;         // ≈ pane 局部坐标（去窗口原点）
                    //     if local_x < HEADER_W || local_y < COL_HEADER_H { return; }               // 表头区不选中（v1 最小改动）
                    //     let content_x = local_x - HEADER_W + scroll_x;                             // ≈ lx + remX（还原内容坐标）
                    //     let content_y = local_y - COL_HEADER_H + scroll_y;
                    //     let (col, row) = state.content_to_cell(content_x, content_y);              // ≈ GetPosFromPixel(aPos, eWhich)
                    //     if click_count >= 2 { begin_edit(window, cx, None); }                      // ≈ SetInputMode(SC_INPUT_TABLE)（双击编辑）
                    //     else { select_cell(col, row, cx); }                                         // ≈ 单击选中
                    //   偏差核对：命中测试严格走 content_to_cell（GetPosFromPixel 逆映射），与绘制同源 → 不随滚动错位（单测证明）。
                    // 点击命中测试：屏幕坐标 → 内容坐标 → 单元格。
                    .on_mouse_down(MouseButton::Left, {
                        // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::SplitHdl（ScHSplitWindow/ScVSplitWindow 拖拽写 nFixPos）
                        //   C++（设计还原）：拖拽 splitter → nFixPosX/Y 设冻结列/行数；双击取消冻结。
                        //   Rust 逐行对应（命中测试走 content_to_cell 同源逆映射）：
                        //     if local_y < COL_HEADER_H {  // 列标头带
                        //         let (col,_) = content_to_cell(content_x, 0);          // ≈ GetPosFromPixel
                        //         let right_edge = HEADER_W + col_left(col+1) - scroll_x; // 该列右缘屏幕 X
                        //         if |local_x - right_edge| <= HANDLE {                  // 命中右缘 4px 热区
                        //             if click_count>=2 { set_frozen_cols(0) }           // ≈ 双击取消冻结
                        //             else { freeze_drag = Some(Col); }                  // ≈ 开始拖拽 splitter
                        //             return; } }
                        //     // 行号带同理（命中下缘 4px 热区 → FreezeAxis::Row）
                        //   self.freeze_drag 是瞬时态（非真相源），拖拽中经全局 mouse_move 反算列/行 → set_frozen_cols/rows + notify。
                        let this = this.clone();
                        move |event: &MouseDownEvent, window, cx| {
                            let pos_x: f32 = event.position.x.into();
                            let pos_y: f32 = event.position.y.into();
                            let click_count = event.click_count;
                            let _ = this.update(cx, |v, cx| {
                                let local_x = pos_x - v.canvas_ox;
                                let local_y = pos_y - v.canvas_oy;
                                // 冻结拖拽手柄热区：列标头带右缘 / 行号带下缘 4px（命中即进入冻结拖拽）。
                                const HANDLE: f32 = 4.0;
                                if local_y >= 0.0 && local_y < COL_HEADER_H && local_x >= HEADER_W {
                                    // 列标头带：反算指针所在列，取其右缘屏幕 X。
                                    let content_x = local_x - HEADER_W + v.state.scroll_x;
                                    let (col, _) = v.state.content_to_cell(content_x, 0.0);
                                    let right_edge =
                                        HEADER_W + col_left(col + 1) - v.state.scroll_x;
                                    if (local_x - right_edge).abs() <= HANDLE {
                                        if click_count >= 2 {
                                            v.state.set_frozen_cols(0); // 双击取消冻结
                                        } else {
                                            v.freeze_drag = Some(FreezeAxis::Col);
                                        }
                                        cx.notify();
                                        return;
                                    }
                                }
                                if local_x >= 0.0 && local_x < HEADER_W && local_y >= COL_HEADER_H {
                                    // 行号带：反算指针所在行，取其下缘屏幕 Y。
                                    let content_y = local_y - COL_HEADER_H + v.state.scroll_y;
                                    let (_, row) = v.state.content_to_cell(0.0, content_y);
                                    let bottom_edge =
                                        COL_HEADER_H + row_top(row + 1) - v.state.scroll_y;
                                    if (local_y - bottom_edge).abs() <= HANDLE {
                                        if click_count >= 2 {
                                            v.state.set_frozen_rows(0); // 双击取消冻结
                                        } else {
                                            v.freeze_drag = Some(FreezeAxis::Row);
                                        }
                                        cx.notify();
                                        return;
                                    }
                                }
                                // 表头区（v1）不做选中：点击角 / 列标头 / 行号不移动选区。
                                if local_x < HEADER_W || local_y < COL_HEADER_H {
                                    return;
                                }
                                // 内容坐标 = 局部坐标 − 表头偏移 + 滚动量。
                                let content_x = local_x - HEADER_W + v.state.scroll_x;
                                let content_y = local_y - COL_HEADER_H + v.state.scroll_y;
                                let (col, row) = v.state.content_to_cell(content_x, content_y);
                                if click_count >= 2 {
                                    v.begin_edit(window, cx, None);
                                } else {
                                    v.select_cell(col, row, cx);
                                }
                            });
                        }
                    })
                    .child(
                // 网格区 + 纵滚动条（横排）：与 LibreOffice ScTabView 编排一致
                // （滚动条是 canvas 之外的兄弟 GPUI 控件，红线：自绘无 overflow/ScrollHandle）。
                div()
                    .id("sheet-grid-row")
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    .child(
                        // 原 canvas 容器（保留 flex 撑高，红线 1：唯一 canvas）
                        div()
                            .id("sheet-canvas-area")
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_h_0()
                            .child(
                                canvas(
                    {
                        // measure：读状态 + canvas 可用视口，推算可见窗口。
                                let state = state;
                                let cols = cols_snap;
                                let rows = rows_snap;
                                let this_entity = this_entity.clone();
                                move |bounds, _window, cx| {
                                    let data_w: f32 = f32::from(bounds.size.width) - HEADER_W;
                                    let data_h: f32 = f32::from(bounds.size.height) - COL_HEADER_H;
                                    let win = compute_visible_window(
                                        data_w, data_h, state.scroll_x, state.scroll_y, cols, rows,
                                    );
                                    // 记录视口尺寸与画布窗口原点，供下一帧滚轮 clamp / 命中测试使用。
                                    let this = this_entity.clone();
                                    let ox: f32 = bounds.origin.x.into();
                                    let oy: f32 = bounds.origin.y.into();
                                    cx.defer(move |cx| {
                                        let _ = this.update(cx, |v, _cx| {
                                            v.viewport_w = data_w;
                                            v.viewport_h = data_h;
                                            v.canvas_ox = ox;
                                            v.canvas_oy = oy;
                                        });
                                    });
                                    win
                                }
                            },
                            {
                                // mirrors LibreOffice: sc/source/ui/view/output.cxx — ScOutputData::Draw + sc/source/ui/view/tabview.cxx — ScTabView 4-pane 装配
                                //   Draw（研究文档 §5.1/§7.6，设计）：按顺序 out.DrawGrid(); out.DrawStrings(); out.Draw();
                                //        无状态绘制器，4 个 pane 共用同一份代码（仅参数 anchor/origin/范围/zoom 不同）。
                                //   ScTabView（研究文档 §1）：创建最多 4 个 ScGridWindow（TL/TR/BL/BR），各自 Paint 调同一 ScOutputData::Draw。
                                //   Rust 逐行对应（单 canvas 合并 4 pane，统一 paint 闭包按同源公式绘 4 区域）：
                                //     window.paint_quad(fill(bounds, content_bg));   // 铺底（角/表头带稍后覆盖，等价 4 pane 各自底色）
                                //     with_content_mask(data_clip_rect)   { for r..for c { paint_cell_background; ShapedLine::paint } }  // ≈ DrawGrid+DrawStrings+Draw（数据区）
                                //     with_content_mask(col_header_clip_rect) { for c { paint_col_header } }  // ≈ 列标头 pane（仅横滚）
                                //     with_content_mask(row_header_clip_rect) { for r { paint_row_number } }  // ≈ 行号 pane（仅纵滚）
                                //     paint_corner(...);   // ≈ TopLeft 固定角（最后画、不裁剪）
                                //   坐标同源：screen_x = ox + cell_to_screen(c,0,BR).0；screen_y = oy + cell_to_screen(0,r,BR).1
                                //           列标头 X 与数据 X、行号 Y 与数据 Y 同一表达式 → 结构上不可能错位。
                                //   偏差核对：EWP 单 canvas 承载 4 pane（Calc 是 4 个 VCL Window），但绘制代码同源、不引入第二套坐标（红线 §1.4）。
                                // paint：用同源公式画四区域。
                                let theme = theme;
                                let selected = selected;
                                let editing = editing;
                                let edit_target = edit_target;
                                let cells = data_cells.clone();
                                let state = state;
                                let text_cache = self.text_cache.clone();
                                // 冻结拖拽瞬时态快照（非真相源）供 paint 内全局鼠标监听使用。
                                let freeze_drag = self.freeze_drag;
                                let this_drag = this.clone();
                                move |bounds, win: VisibleWindow, window: &mut Window, cx: &mut App| {
                                    let ox: f32 = bounds.origin.x.into();
                                    let oy: f32 = bounds.origin.y.into();
                                    // 同源坐标闭包：列标头 X 与数据 X 同一表达式；行号 Y 与数据 Y 同一表达式。
                                    // 统一经 `cell_to_screen(col, row, pane)` 取同源屏幕坐标（v1 仅用
                                    // `Pane::BottomRight`，frozen=0 → 退化为 cell_screen_x/y；冻结窗格为后续扩展点，
                                    // 届时按区域选择对应 Pane 即可，不引入第二套坐标机制）。
                                    let screen_x = |col: usize| -> f32 {
                                        ox + state.cell_to_screen(col, 0, Pane::BottomRight).0
                                    };
                                    let screen_y = |row: usize| -> f32 {
                                        oy + state.cell_to_screen(0, row, Pane::BottomRight).1
                                    };

                                    // 背景铺满整个 canvas 视口（不裁剪：先把两个槽填成内容底色，
                                    // 稍后由列标头带 / 行号带 / 角覆盖，确保滚动时槽里不会透出底层内容）。
                                    window.paint_quad(gpui::fill(bounds, theme.content_bg));

                                    // 画布完整尺寸（窗口坐标系），用于裁剪带右下角。
                                    let canvas_w: f32 = f32::from(bounds.size.width);
                                    let canvas_h: f32 = f32::from(bounds.size.height);

                                    // 数据区：裁剪到数据矩形（左/上缘锁在 HEADER_W/COL_HEADER_H），
                                    // 滚动后溢出的格子被裁掉，永不盖到两侧表头槽。
                                    let data_clip = data_clip_rect(ox, oy, canvas_w, canvas_h);
                                    window.with_content_mask(
                                        Some(ContentMask { bounds: data_clip }),
                                        |window| {
                                            for r in win.r0..win.r1 {
                                                for c in win.c0..win.c1 {
                                                    let x = screen_x(c);
                                                    let y = screen_y(r);
                                                    let cb = Bounds::new(
                                                        point(px(x), px(y)),
                                                        size(px(CELL_W), px(CELL_H)),
                                                    );
                                                    let is_sel = selected == Some((c, r));
                                                    paint_cell_background(window, cx, cb, is_sel, &theme);
                                                    let is_editing_this =
                                                        editing && edit_target == Some((c, r));
                                                    if !is_editing_this {
                                                        if let Some(cell) = cells
                                                            .get(&r)
                                                            .and_then(|m| m.get(&c))
                                                        {
                                                            let text = format_cell(&cell.value);
                                                            if !text.is_empty() {
                                                                // LibreOffice 对齐：数值右对齐、文本左对齐
                                                                // （output2.cxx 中 eOutHorJust 对 value 取 Right、对 text 取 Left）。
                                                                let is_number =
                                                                    matches!(cell.value, CellValue::Number(_));
                                                                let shaped = text_cache
                                                                    .read(cx)
                                                                    .get_or_shape(r, c, &text, &theme, window, cx);
                                                                // 注意：本版本 GPUI 0.2.2 中 `LineLayout.width` 是公开字段
                                                                //（`width()` 方法只在 `WrappedLineLayout` 上），`ShapedLine` 经
                                                                // `Deref` 暴露为字段，故此处用 `shaped.width` 而非 `shaped.width()`。
                                                                let text_w: f32 = f32::from(shaped.width);
                                                                let origin_x = if is_number {
                                                                    // 数值右对齐：贴右内边距。
                                                                    x + CELL_W - CELL_PAD - text_w
                                                                } else {
                                                                    // 文本左对齐。
                                                                    x + CELL_PAD
                                                                };
                                                                let origin = point(
                                                                    px(origin_x),
                                                                    px(y + (CELL_H - CELL_FONT_SIZE) / 2.0),
                                                                );
                                                                let _ = shaped.paint(origin, px(CELL_H), window, cx);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                    );

                                    // 列标头带（仅横向滚）：左缘锁 ox+HEADER_W，永不盖行号槽。
                                    let col_clip = col_header_clip_rect(ox, oy, canvas_w);
                                    window.with_content_mask(
                                        Some(ContentMask { bounds: col_clip }),
                                        |window| {
                                            for cidx in win.c0..win.c1 {
                                                let x = screen_x(cidx);
                                                let is_sel = selected.map(|(sc, _)| sc) == Some(cidx);
                                                paint_col_header(window, cx, x, oy, &col_name(cidx), is_sel, &theme);
                                                if is_sel {
                                                    paint_header_selection(
                                                        window,
                                                        cx,
                                                        Bounds::new(
                                                            point(px(x), px(oy)),
                                                            size(px(CELL_W), px(COL_HEADER_H)),
                                                        ),
                                                        &theme,
                                                    );
                                                }
                                            }
                                        },
                                    );

                                    // 行号带（仅纵向滚）：上缘锁 oy+COL_HEADER_H，永不盖列标头槽。
                                    let row_clip = row_header_clip_rect(ox, oy, canvas_h);

                                    window.with_content_mask(
                                        Some(ContentMask { bounds: row_clip }),
                                        |window| {
                                            for ridx in win.r0..win.r1 {
                                                let y = screen_y(ridx);
                                                let is_sel = selected.map(|(_, sr)| sr) == Some(ridx);
                                                paint_row_number(window, cx, ox, y, ridx, is_sel, &theme);
                                                if is_sel {
                                                    paint_header_selection(
                                                        window,
                                                        cx,
                                                        Bounds::new(
                                                            point(px(ox), px(y)),
                                                            size(px(HEADER_W), px(CELL_H)),
                                                        ),
                                                        &theme,
                                                    );
                                                }
                                            }
                                        },
                                    );

                                    // 角（左上固定）：放在三段裁剪带之后最后画、不裁剪，
                                    // 使左上交叉点永远盖在最上层、干净。
                                    paint_corner(window, cx, ox, oy, &theme);

                                    // 冻结分隔线（T04）：与数据区同源坐标（freeze_split_line 经
                                    // cell_to_screen(frozen)），画在四区域之上；frozen=0 时不画。
                                    // mirrors LibreOffice: sc/source/ui/view/output.cxx — DrawGrid 冻结边界重线
                                    if state.frozen_cols > 0 || state.frozen_rows > 0 {
                                        paint_freeze_splitter(
                                            window, cx, ox, oy, canvas_w, canvas_h, &state, &theme,
                                        );
                                    }

                                    // 冻结拖拽全局监听（GPUI 0.2.2 无 capture_mouse，用 Paint 阶段注册的
                                    // window.on_mouse_event 实现「捕获鼠标」等价语义）：拖拽中全局
                                    // mouse_move 反算列/行 → set_frozen_cols/rows + notify；松手 mouse_up
                                    // 清除 freeze_drag。paint 闭包只读 state 快照，瞬时态经 notify 触发新帧
                                    // （红线：不移动窗口几何、不重入 update）。
                                    if let Some(axis) = freeze_drag {
                                        let drag_axis = axis;
                                        let ox_f = ox;
                                        let oy_f = oy;
                                        let state_f = state;
                                        let this_f = this_drag.clone();
                                        window.on_mouse_event(
                                            move |event: &MouseMoveEvent, _phase, _window, cx| {
                                                if event.pressed_button != Some(MouseButton::Left) {
                                                    return;
                                                }
                                                let content_x = f32::from(event.position.x) - ox_f
                                                    - HEADER_W
                                                    + state_f.scroll_x;
                                                let content_y = f32::from(event.position.y) - oy_f
                                                    - COL_HEADER_H
                                                    + state_f.scroll_y;
                                                let _ = this_f.update(cx, |view, cx| {
                                                    view.on_freeze_drag(
                                                        drag_axis, content_x, content_y, cx,
                                                    );
                                                });
                                            },
                                        );
                                        let this_f2 = this_drag.clone();
                                        window.on_mouse_event(
                                            move |_event: &MouseUpEvent, _phase, _window, cx| {
                                                let _ = this_f2.update(cx, |view, _cx| {
                                                    view.freeze_drag = None;
                                                });
                                            },
                                        );
                                    }
                                }
                            },
                        )
                        // 用 flex-grow 撑满 canvas-area（已恢复为 flex 容器），不再依赖
                        // height:100% 的百分比解析——这是 v2 空白 bug 的根因修复点。
                        .flex_1()
                        .min_h_0()
                                    )
                                )
                                .child(
                                    // 纵滚动条（固定宽 12px，canvas 外兄弟控件，红线：自绘无 overflow）
                                    render_v_scrollbar(
                                        self.state,
                                        self.total_h,
                                        self.viewport_h,
                                        self.vscroll_drag,
                                        vscroll_on_drag.clone(),
                                        vscroll_on_begin.clone(),
                                        vscroll_on_end.clone(),
                                    )
                                )
                            )
                            .child(
                                // 横滚动条 + 右下角方块（横排）
                                div()
                                    .id("sheet-hscroll-row")
                                    .flex()
                                    .flex_row()
                                    .child(
                                        // 横滚动条（高 12px，占满剩余宽度）
                                        render_h_scrollbar(
                                            self.state,
                                            self.total_w,
                                            self.viewport_w,
                                            self.hscroll_drag,
                                            hscroll_on_drag.clone(),
                                            hscroll_on_begin.clone(),
                                        hscroll_on_end.clone(),
                                    )
                                )
                                    .child(
                                        // 滚动条右下角方块
                                        div()
                                            .id("sheet-corner")
                                            .w(px(12.))
                                            .h(px(12.))
                                            .bg(c.border)
                                    )
                            )
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
            // ═══ 状态栏（自绘组件，≈ LibreOffice StatusBar；纯展示、只读派生） ═══
            .child(render_status_bar(&status_model))
            .into_any_element();

        // 构造 chrome 上下文并交给布局管理器渲染（标准 / 标签页式 由当前模式决定）。
        let ctx = ChromeCtx {
            model_kind: ModelKind::Sheet,
            name: self.name.clone(),
            dirty: self.dirty,
            sidebar_open: false,
            tool_group,
            on_save,
            on_toggle_sidebar,
            on_format,
            on_switch_model,
        };

        UiLayoutManager::render_chrome(cx, window, ctx, body)
    }
}

impl SheetView {
    // ─────────────────────────────────────────────────────────────
    // 状态栏只读派生（T03）
    // ─────────────────────────────────────────────────────────────

    // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::CreateStatusArea / UpdateStatusBar / FillStatusBar
    //   C++：状态栏各字段由当前选区 / 文档状态一次性派生（地址、单元格内容、Sheet 名/计数、
    //        缩放、选区统计 Sum/Avg/Count、插入模式、语言），组件只展示、不持有状态。
    //   Rust 逐行对应：从 self.selected / self.state / self.workbook() 计算 StatusBarModel
    //        （纯只读派生，零副作用；红线：状态栏不写状态、不触发 notify）。
    /// 只读派生状态栏数据：地址 / 单元格预览 / Sheet 信息 / 缩放 / 选区统计 / 插入模式 / 语言。
    /// `render_status_bar` 只消费此模型，自身不持状态、不 notify（红线 §1.4.5）。
    fn derive_status_bar(&self) -> StatusBarModel {
        let sheet = self.current_sheet();
        let sheet_count = self.workbook().sheets.len();
        let sheet_name = sheet.name.clone();
        // zoom 默认 1.0（=100%），本组件只读展示（扩展点：接入 UI 后实时更新）。
        let zoom_pct = (self.state.zoom * 100.0).round().max(0.0) as u32;

        // v1 单格选区：统计只对选中格生效。数值格 → Sum/Avg=该值、Count=1；
        // 非空非数值 → Count=1、Sum/Avg 不适用；空格 → 全 0/None（与 Calc 一致）。
        let (cell_addr, cell_preview, sum, avg, count, selection_label) = match self.selected {
            Some((col, row)) => {
                let addr = format!("{}{}", col_name(col), row + 1);
                let preview = self.selected_raw_text();
                let (sum, avg, count) = match sheet
                    .cells
                    .get(&row)
                    .and_then(|m| m.get(&col))
                    .map(|c| &c.value)
                {
                    Some(CellValue::Number(f)) => (Some(*f), Some(*f), 1usize),
                    Some(_) => (None, None, 1usize),
                    None => (None, None, 0usize),
                };
                (addr.clone(), preview, sum, avg, count, addr)
            }
            None => (
                "—".to_string(),
                String::new(),
                None,
                None,
                0usize,
                "—".to_string(),
            ),
        };

        StatusBarModel {
            cell_addr,
            cell_preview,
            sheet_name,
            sheet_count,
            zoom_pct,
            sum,
            avg,
            count,
            selection_label,
            insert_mode: "INS",
            language: "中文(中国)",
        }
    }

    // ─────────────────────────────────────────────────────────────
    // 滚动条拖拽回调（T02）
    // ─────────────────────────────────────────────────────────────

    // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::ScrollHdl / ScrollVHdl → SetPosX/Y + Invalidate
    //   C++：拖拽滚动条 → ScrollHdl → pViewData->SetPosX(newX) + Invalidate(pane)
    //   Rust 逐行对应：state.set_scroll_x/y(target, viewport, total)（内部 clamp 到 [0, total-viewport]）→ cx.notify()
    /// 滚动条拖拽进行中：把目标 scroll 写回集中状态（内部 clamp）并重绘（红线：只改 state + notify）。
    fn on_scrollbar_drag(&mut self, axis: ScrollAxis, value: f32, cx: &mut Context<Self>) {
        match axis {
            ScrollAxis::H => self.state.set_scroll_x(value, self.viewport_w, self.total_w),
            ScrollAxis::V => self.state.set_scroll_y(value, self.viewport_h, self.total_h),
        }
        cx.notify();
    }

    // Rust 逐行对应（拖拽开始）：记录指针在 thumb 上的抓取偏移（瞬时态，非真相源）→ notify 触发新帧。
    /// 滚动条拖拽开始：记录抓取偏移（供 paint 内全局 mouse_move 反算 thumb 位置）。
    fn on_scrollbar_drag_begin(&mut self, axis: ScrollAxis, grab: f32, cx: &mut Context<Self>) {
        match axis {
            ScrollAxis::H => self.hscroll_drag = Some(grab),
            ScrollAxis::V => self.vscroll_drag = Some(grab),
        }
        cx.notify();
    }

    // Rust 逐行对应（拖拽结束）：清除抓取偏移（松手）→ notify。
    /// 滚动条拖拽结束：清除拖拽瞬时态。
    fn on_scrollbar_drag_end(&mut self, axis: ScrollAxis, cx: &mut Context<Self>) {
        match axis {
            ScrollAxis::H => self.hscroll_drag = None,
            ScrollAxis::V => self.vscroll_drag = None,
        }
        cx.notify();
    }

    // ─────────────────────────────────────────────────────────────
    // 冻结拖拽回调（T04）
    // ─────────────────────────────────────────────────────────────

    // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::SplitHdl（ScHSplitWindow/ScVSplitWindow 拖拽写 nFixPos）
    //   C++：拖拽 splitter → nFixPosX = nNewCols（冻结列数 = 指针所在列「含左」计数）
    //   Rust 逐行对应：content_to_cell(content_x, content_y)（同源逆映射 GetPosFromPixel）→ set_frozen_cols/rows(col+1/row+1) + notify
    /// 冻结拖拽进行中：由内容坐标反算列/行，写入冻结数（红线：只改 state + notify）。
    fn on_freeze_drag(
        &mut self,
        axis: FreezeAxis,
        content_x: f32,
        content_y: f32,
        cx: &mut Context<Self>,
    ) {
        let (col, row) = self.state.content_to_cell(content_x, content_y);
        match axis {
            // 冻结「指针列及其左侧」所有列（含该列）：≈ nFixPosX = nNewCols。
            FreezeAxis::Col => self.state.set_frozen_cols(col.saturating_add(1)),
            FreezeAxis::Row => self.state.set_frozen_rows(row.saturating_add(1)),
        }
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════
    // T05：view.rs 集成 + 红线 grep 单测（不依赖 Window，纯源码内省）。
    //
    // 设计硬约束（docs/sheet_peripherals/system_design.md §0/§1）：三外围组件
    // 只经 `SheetViewState` 驱动 canvas，红线禁用 `ScrollHandle` / `overflow_` /
    // `track_scroll` / `_b.origin`。本模块把"红线 grep"与"集成装配"固化为单测：
    //   - red_line_no_forbidden_apis_anywhere：扫描 sheet 模块全部源文件，确认无真实
    //     （非注释）红线禁用 API。（注释中提及这些词是为说明"已删除"，已剔除。）
    //   - integration_three_peripherals_wired：确认 render 装配了三大外围组件 +
    //     冻结线绘制 + 拖拽回调接线 + 布局草图节点（防回归拆掉装配）。
    // ═══════════════════════════════════════════════════════════════════

    /// 剔除注释后的「纯代码」文本：每行按首个 `//` 截断（含整行与行尾注释），
    /// 并丢弃块注释起始行（`/*`）。红线关键词只在文档注释里出现（描述"已删除"），
    /// 截断后即可安全判定"真实引用"。
    fn code_only(src: &str) -> String {
        src.lines()
            .map(|line| match line.find("//") {
                Some(i) => &line[..i],
                None => line,
            })
            .filter(|line| !line.trim_start().starts_with("/*"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 红线禁用 token（用 `concat!` 拼出，确保本测试源自身**不出现连续禁用串**，
    /// 避免 `include_str!("view.rs")` 把本测试自身误判为红线违规）。
    fn forbidden_tokens() -> [&'static str; 4] {
        [
            concat!("Scroll", "Handle"),
            concat!("over", "flow_"),
            concat!("track_", "scroll"),
            concat!("_b.", "origin"),
        ]
    }

    /// 对任意 sheet 模块源文件，断言代码（非注释）中不含任何红线禁用 token。
    fn assert_red_line_clean(src: &str, file: &str) {
        let code = code_only(src);
        for tok in forbidden_tokens() {
            assert!(
                !code.contains(tok),
                "红线违规：{file} 含禁用 token `{tok}`（应只经 SheetViewState 驱动 canvas）"
            );
        }
    }

    // mirrors LibreOffice: docs/sheet_peripherals/system_design.md §0 红线五条 — 全仓 0 真实
    //   ScrollHandle / overflow_*_scroll / track_scroll / _b.origin 引用（三组件只经 SheetViewState）。
    // Rust 逐行对应（源码内省 + 注释剔除）：
    //   扫描 view/scrollbar/grid/status_bar/view_state/mod 六个源文件
    //     → 逐文件 code_only 去注释 → 断言不含 forbidden_tokens → 红线 0 真实引用
    #[test]
    fn red_line_no_forbidden_apis_anywhere() {
        assert_red_line_clean(include_str!("view.rs"), "view.rs");
        assert_red_line_clean(include_str!("scrollbar.rs"), "scrollbar.rs");
        assert_red_line_clean(include_str!("grid.rs"), "grid.rs");
        assert_red_line_clean(include_str!("status_bar.rs"), "status_bar.rs");
        assert_red_line_clean(include_str!("view_state.rs"), "view_state.rs");
        assert_red_line_clean(include_str!("mod.rs"), "mod.rs");
    }

    // mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView 装配 aHScroll/aVScroll/StatusBar/Splitter
    //   C++（设计还原）：ScTabView 构造时把 ScrollBar / StatusBar / Splitter 挂到网格四周，
    //      拖拽经 ScrollHdl/SplitHdl 写 nPosX/Y / nFixPos + Invalidate。
    //   Rust 逐行对应（源码内省）：render 必须引用三大外围组件 + 冻结线绘制 + 拖拽回调
    //     + 布局草图节点（sheet-grid-row / sheet-hscroll-row / sheet-corner），任一被拆即回归失败。
    #[test]
    fn integration_three_peripherals_wired_into_render() {
        let src = include_str!("view.rs");
        // 三外围组件装配：横/纵滚动条 + 状态栏 + 冻结分隔线。
        assert!(src.contains("render_h_scrollbar"), "横滚动条未装配到 render");
        assert!(src.contains("render_v_scrollbar"), "纵滚动条未装配到 render");
        assert!(src.contains("render_status_bar"), "状态栏未装配到 render");
        assert!(src.contains("paint_freeze_splitter"), "冻结分隔线未装配到 paint");
        assert!(src.contains("derive_status_bar"), "状态栏只读派生未接入 render");
        // 拖拽回调接线：滚动条 → set_scroll_x/y + notify；冻结 → set_frozen_* + notify。
        assert!(src.contains("on_scrollbar_drag"), "滚动条拖拽回调未接线");
        assert!(src.contains("on_freeze_drag"), "冻结拖拽回调未接线");
        assert!(src.contains("set_scroll_x"), "set_scroll_x 未被消费");
        assert!(src.contains("set_scroll_y"), "set_scroll_y 未被消费");
        assert!(src.contains("set_frozen_cols"), "set_frozen_cols 未被消费");
        assert!(src.contains("set_frozen_rows"), "set_frozen_rows 未被消费");
        // 布局草图关键节点 id（T02/T04 布局：网格区+纵滚动条横排、横滚动条+右下角方块）。
        assert!(src.contains("sheet-grid-row"), "网格区+纵滚动条横排布局缺失");
        assert!(src.contains("sheet-hscroll-row"), "横滚动条+右下角方块布局缺失");
        assert!(src.contains("sheet-corner"), "右下角方块缺失");
    }

    // ═══════════════════════════════════════════════════════════════════
    // 深色模式公式栏修复回归锁（本次 BugFix 核心）。
    //
    // 主因：view.rs `Input::appearance(true)` 使用 gpui-component 自带外观，背景取
    // `cx.theme().background`（深色纸张 → 近黑 #0a0a0a），与 EWP 公式栏
    // `sidebar_bg(#252526)` 明显不符、深色下突兀。
    // 次要：cell-addr 用偏亮的 `button_bg`，比公式栏更亮。
    // 修复：Input 用 `.bg(c.content_bg).border_color(c.border)` 覆盖自带外观；
    //       cell-addr 改用 `content_bg`（inset 风格）。
    // 下面用源码内省把这两个修复固化为回归锁（与 T05 同范式）：若回退到
    // `button_bg` 或去掉 `content_bg/border_color` 覆盖，测试即失败。
    // ═══════════════════════════════════════════════════════════════════

    /// 取 `start` 标记之后、`end` 标记之前（不含）的代码段，把回归锁限定在具体
    /// 组件块内，避免与文件内其它合法用法（如真实按钮的 `button_bg`）误冲突。
    fn region_between(code: &str, start: &str, end: &str) -> String {
        let s = code.find(start).unwrap_or(0);
        let e = code.find(end).unwrap_or(code.len());
        if e <= s {
            return String::new();
        }
        code[s..e].to_string()
    }

    #[test]
    fn dark_mode_formula_bar_input_follows_ewp_theme() {
        // 主修复锁：编辑态 Input 必须 `.appearance(true)` 且被
        // `.bg(c.content_bg).border_color(c.border)` 覆盖，跟随 EWP 深色主题，
        // 而非 gpui-component 自带近黑 #0a0a0a。
        let code = code_only(include_str!("view.rs"));
        let input_block = region_between(&code, "Input::new(&self.edit_input)", ".when(!editing)");
        assert!(
            !input_block.is_empty(),
            "未找到公式栏编辑框 Input 块（region_between 失败）"
        );
        assert!(
            input_block.contains(".appearance(true)"),
            "公式栏 Input 未启用 appearance(true)"
        );
        assert!(
            input_block.contains(".bg(c.content_bg)"),
            "深色模式修复缺失：Input 未用 .bg(c.content_bg) 覆盖 gpui-component 自带背景"
        );
        assert!(
            input_block.contains(".border_color(c.border)"),
            "深色模式修复缺失：Input 未用 .border_color(c.border) 跟随 EWP 边框"
        );
    }

    #[test]
    fn dark_mode_cell_addr_uses_content_bg_not_button_bg() {
        // 次修复锁：cell-addr 必须用 content_bg（inset 风格），不得回退到偏亮的 button_bg。
        // 用 region 限定在 cell-addr 块内（文件其它处合法使用 button_bg 不计入）。
        let code = code_only(include_str!("view.rs"));
        let cell_addr_block = region_between(&code, "id(\"cell-addr\")", ".when(editing)");
        assert!(
            !cell_addr_block.is_empty(),
            "未找到 cell-addr 块（region_between 失败）"
        );
        assert!(
            cell_addr_block.contains(".bg(c.content_bg)"),
            "cell-addr 未使用 content_bg（inset 风格），深色下会突兀"
        );
        assert!(
            !cell_addr_block.contains(".bg(c.button_bg)"),
            "回归：cell-addr 回退到偏亮 button_bg，深色主题下比公式栏更亮"
        );
    }

    #[test]
    fn dark_mode_formula_bar_container_uses_sidebar_bg() {
        // 公式栏容器背景应为 sidebar_bg（与整个 chrome 一致），边框用 border。
        let code = code_only(include_str!("view.rs"));
        let bar_block = region_between(&code, "id(\"formula-bar\")", "id(\"cell-addr\")");
        assert!(!bar_block.is_empty(), "未找到 formula-bar 容器块");
        assert!(
            bar_block.contains(".bg(c.sidebar_bg)"),
            "公式栏容器未使用 sidebar_bg"
        );
        assert!(
            bar_block.contains(".border_color(c.border)"),
            "公式栏容器未使用 border"
        );
    }

    // ═══════════════════════════════════════════════════════════════════
    // 公式栏文本逻辑单测（纯函数，编辑栏显示/提交的直接依赖）。
    // ═══════════════════════════════════════════════════════════════════

    #[test]
    fn col_name_boundary_values() {
        // 0→A … 25→Z, 26→AA, 51→AZ, 701→ZZ, 702→AAA（与 Excel/LibreOffice 一致）。
        assert_eq!(col_name(0), "A");
        assert_eq!(col_name(25), "Z");
        assert_eq!(col_name(26), "AA");
        assert_eq!(col_name(51), "AZ");
        assert_eq!(col_name(701), "ZZ");
        assert_eq!(col_name(702), "AAA");
        // 几个随机校验。
        assert_eq!(col_name(2), "C");
        assert_eq!(col_name(27), "AB");
        assert_eq!(col_name(52), "BA");
    }

    #[test]
    fn parse_cell_value_covers_all_kinds() {
        // 空 → Empty；= 前缀 → Formula；数字 → Number；true/false → Bool；其余 → Text。
        assert_eq!(parse_cell_value(""), CellValue::Empty);
        assert_eq!(parse_cell_value("   "), CellValue::Empty);
        assert_eq!(parse_cell_value("=A1+B2"), CellValue::Formula("A1+B2".to_string()));
        assert_eq!(parse_cell_value("123"), CellValue::Number(123.0));
        assert_eq!(parse_cell_value("2.5"), CellValue::Number(2.5));
        assert_eq!(parse_cell_value("true"), CellValue::Bool(true));
        assert_eq!(parse_cell_value("FALSE"), CellValue::Bool(false));
        assert_eq!(parse_cell_value("hello"), CellValue::Text("hello".to_string()));
    }

    #[test]
    fn raw_cell_text_roundtrips_for_editing() {
        // 编辑栏回填文本（raw）经 parse 应基本还原：公式带 =、数字原值、布尔大写。
        assert_eq!(raw_cell_text(&CellValue::Empty), "");
        assert_eq!(raw_cell_text(&CellValue::Number(123.0)), "123");
        assert_eq!(raw_cell_text(&CellValue::Number(2.5)), "2.5");
        assert_eq!(raw_cell_text(&CellValue::Text("x".to_string())), "x");
        assert_eq!(raw_cell_text(&CellValue::Bool(true)), "TRUE");
        assert_eq!(raw_cell_text(&CellValue::Formula("A1".to_string())), "=A1");
        // 公式回填 → 再解析应保持 Formula。
        assert_eq!(
            parse_cell_value(&raw_cell_text(&CellValue::Formula("A1+B2".to_string()))),
            CellValue::Formula("A1+B2".to_string())
        );
        // 数字回填 → 再解析保持 Number。
        assert_eq!(
            parse_cell_value(&raw_cell_text(&CellValue::Number(2.5))),
            CellValue::Number(2.5)
        );
    }

    #[test]
    fn format_cell_display_text() {
        // 显示用文本：空→""、整数→无小数、浮点→去尾零、文本原样、布尔大写、公式原样。
        assert_eq!(format_cell(&CellValue::Empty), "");
        assert_eq!(format_cell(&CellValue::Number(123.0)), "123");
        assert_eq!(format_cell(&CellValue::Number(2.5)), "2.5");
        assert_eq!(format_cell(&CellValue::Text("hi".to_string())), "hi");
        assert_eq!(format_cell(&CellValue::Bool(false)), "FALSE");
        assert_eq!(format_cell(&CellValue::Formula("=SUM(A1)".to_string())), "=SUM(A1)");
    }
}
