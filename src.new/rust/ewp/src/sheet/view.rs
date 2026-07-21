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
    FontWeight, KeyDownEvent, MouseButton, MouseDownEvent, Render, ScrollWheelEvent, SharedString,
    Window, canvas, div, point, px, rgba, size,
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
    /// canvas 在窗口坐标系中的左上角原点，由 prepaint 每帧记录，供命中测试换算。
    canvas_ox: f32,
    canvas_oy: f32,
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
            canvas_ox: 0.0,
            canvas_oy: 0.0,
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
        let book = self.workbook();
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

        let editing = self.editing;
        let selected = self.selected;
        let edit_target = self.edit_target;
        let addr = match self.selected {
            Some((col, row)) => format!("{} {}", col_name(col), row + 1),
            None => "—".to_string(),
        };
        let display = self.selected_raw_text();

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

        // ── 文档内容 body（不含顶部栏，由框架套 chrome 后放在下方）──
        // 焦点恒在网格根：方向键 / 直接打字进入编辑都依赖它。
        let body = div()
            .id("sheet-root")
            .flex_1()
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
                    // 点击命中测试：屏幕坐标 → 内容坐标 → 单元格。
                    .on_mouse_down(MouseButton::Left, {
                        let this = this.clone();
                        move |event: &MouseDownEvent, window, cx| {
                            let pos_x: f32 = event.position.x.into();
                            let pos_y: f32 = event.position.y.into();
                            let click_count = event.click_count;
                            let _ = this.update(cx, |v, cx| {
                                let local_x = pos_x - v.canvas_ox;
                                let local_y = pos_y - v.canvas_oy;
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
                                // paint：用同源公式画四区域。
                                let theme = theme;
                                let selected = selected;
                                let editing = editing;
                                let edit_target = edit_target;
                                let cells = data_cells.clone();
                                let state = state;
                                let text_cache = self.text_cache.clone();
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
                                                paint_row_number(window, cx, y, ridx, is_sel, &theme);
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
                                }
                            },
                        )
                        // 用 flex-grow 撑满 sheet-body（已恢复为 flex 容器），不再依赖
                        // height:100% 的百分比解析——这是 v2 空白 bug 的根因修复点。
                        .flex_1()
                        .min_h_0()
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
