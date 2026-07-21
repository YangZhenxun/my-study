# EWP 电子表格数据区 · 工程师可执行实现规格

> 架构：Bob（高见远）｜全部 API 已对照 GPUI 0.2.2 源码核实（`canvas.rs`/`window.rs`/`text_system.rs`/`div.rs`/`geometry.rs`/`color.rs`/`scene.rs`）。
> 目标：数据区 + 行号列从 `uniform_list` 改为「单 canvas 命令式绘制可见区」，每帧零单元格 DOM 节点。
> 本规格为 `system_design.md` 的代码层落地版，工程师按 T01→T05 顺序实现即可。

---

## 1. 文件清单（相对 `src/`）

| 文件 | 动作 | 职责 |
|---|---|---|
| `sheet_view.rs` | **改** | 主改造点。① `vscroll: UniformListScrollHandle` → `hscroll: ScrollHandle` + `vscroll: ScrollHandle`；② 新增 `text_cache: Entity<GridTextCache>`（`build()` 内 `cx.new` 创建）；③ `render()` 中删除两个 `uniform_list`，改为装配 3 个 `track_scroll` 容器 + 2 个 canvas（`data-canvas` / `row-num-canvas`）；④ 交互方法（`select_cell`/`begin_edit`/`commit_edit`/`cancel_edit`/`move_selection`/`clear_selected`）保留；⑤ `render_data_row`/`render_row_number` 删除（逻辑下沉到 `sheet_grid.rs`）。 |
| `sheet_grid.rs` | **新增** | 无状态绘制/计算模块：`compute_visible_window`、`col_left`/`row_top`/`col_width`/`row_height`、`paint_cell_background`、`paint_cell_text`、`paint_row_number`。所有绘制都接收 `&mut Window`。 |
| `sheet_grid_cache.rs` | **新增** | `GridTextCache` 实体 + `CellCacheKey`：`get_or_shape(row,col,text,theme,window,cx) -> ShapedLine`、`invalidate_for_sheet()`。 |
| `styles.rs` | 不改 | `ThemeColors` 取色源（`content_bg`/`border`/`text_primary`/`accent`/`sidebar_bg`/`text_muted`）。 |
| `model/sheet.rs` | 不改 | `Sheet`/`Cell`/`set_cell`/`get_cell` 不变。 |

**无新增第三方依赖**（`gpui 0.2.2` 已含 `canvas`/`ScrollHandle`/`paint_quad`/`shape_line`/`ShapedLine`/`fill`/`quad`/`Edges`；`gpui-component` 编辑栏沿用）。

---

## 2. 有序任务列表（带依赖 + 验收标准）

### T01 基础设施：滚动 handle 接入 + 模块骨架  【P0】
- **描述**：在 `SheetView::build()` 将 `vscroll: UniformListScrollHandle::new()` 替换为：
  ```rust
  use gpui::{ScrollHandle, Entity};
  // 在 struct SheetView 中：
  //   hscroll: ScrollHandle,
  //   vscroll: ScrollHandle,
  //   text_cache: Entity<GridTextCache>,
  // build() 内：
  //   hscroll: ScrollHandle::new(),
  //   vscroll: ScrollHandle::new(),
  //   text_cache: cx.new(|_cx| GridTextCache::default()),
  ```
  新建 `sheet_grid.rs`（先放常量、`col_name` 迁移、空 `compute_visible_window` 桩）、`sheet_grid_cache.rs`（`GridTextCache` 空结构 + `Default`）。保留 `DEF_COLS/DEF_ROWS/CELL_W/CELL_H/HEADER_W/COL_HEADER_H` 与网格范围兜底逻辑不动。编译通过。
- **涉及文件**：`sheet_view.rs`、`sheet_grid.rs`(新增)、`sheet_grid_cache.rs`(新增)
- **前置依赖**：无
- **验收**：`cargo build` 通过；`SheetView` 持有 `hscroll`/`vscroll`/`text_cache`；旧 `uniform_list` 暂未删（T05 删），但结构已就位。

### T02 可见窗口计算 + 列宽/行高累加（纯函数）  【P0】
- **描述**：在 `sheet_grid.rs` 实现（列宽/行高当前恒为常量，但形参化前向兼容可变宽度）：
  ```rust
  pub const CELL_W: f32 = 100.0;
  pub const CELL_H: f32 = 28.0;

  /// 第 c 列左边缘的 content 坐标（像素）。可变列宽时改为累加 col_width。
  pub fn col_left(c: usize) -> f32 {
      (0..c).map(col_width).sum()
  }
  pub fn row_top(r: usize) -> f32 {
      (0..r).map(row_height).sum()
  }
  pub fn col_width(_c: usize) -> f32 { CELL_W }   // 前向兼容：以后读 sheet 列宽表
  pub fn row_height(_r: usize) -> f32 { CELL_H }

  /// 仿 LibreOffice Calc 的 AddPixelsWhile：从 (0,0) 累加偏移，越过视口右/下边界即停。
  pub struct VisibleWindow { pub c0: usize, pub c1: usize, pub r0: usize, pub r1: usize,
                             pub scroll_x: f32, pub scroll_y: f32 }
  pub fn compute_visible_window(
      viewport_w: f32, viewport_h: f32,
      scroll_x: f32, scroll_y: f32,
      total_cols: usize, total_rows: usize,
  ) -> VisibleWindow {
      let mut c0 = 0; let mut x = 0.0;
      while c0 < total_cols && x + col_width(c0) <= scroll_x { x += col_width(c0); c0 += 1; }
      let mut c1 = c0;
      while c1 < total_cols && x < scroll_x + viewport_w { x += col_width(c1); c1 += 1; }
      let mut r0 = 0; let mut y = 0.0;
      while r0 < total_rows && y + row_height(r0) <= scroll_y { y += row_height(r0); r0 += 1; }
      let mut r1 = r0;
      while r1 < total_rows && y < scroll_y + viewport_h { y += row_height(r1); r1 += 1; }
      VisibleWindow { c0, c1: c1.max(c0 + 1), r0, r1: r1.max(r0 + 1), scroll_x, scroll_y }
  }
  ```
- **涉及文件**：`sheet_grid.rs`、`sheet_view.rs`(调用点)、`model/sheet.rs`(读 `cols/rows`，不改)
- **前置依赖**：T01
- **验收**：对常量列宽，给定 `scroll_x=0, viewport_w=1000` → `c1-c0 ≈ 10`；`scroll_x=250` → `c0=2` 且首列可见左边缘为负；单元测试或手测通过。

### T03 数据区 canvas：背景 + 网格 + 选中高亮  【P0】
- **描述**：在 `sheet_view.rs` 的 `render()` 数据区位置，用以下结构替换 `uniform_list("sheet-rows", ...)`：
  ```rust
  // 数据区：纵向滚动手柄 vscroll；横向靠 col-scroll 共享 hscroll（此处手动 translate X）
  let h = self.hscroll.clone();
  let v = self.vscroll.clone();
  let cache = self.text_cache.clone();
  let cells = sheet.cells.clone();
  let selected = self.selected;
  let theme = ThemeColors::current();
  let total_w = cols as f32 * CELL_W;
  let total_h = rows as f32 * CELL_H;
  div()
      .id("data-scroll")
      .flex_1()
      .min_h_0()
      .overflow_y_scroll()
      .overflow_x_hidden()
      .track_scroll(self.vscroll.clone())
      .child(
          canvas(
              // prepaint: 计算可见窗口，返回 T
              move |_bounds, _window, _cx| {
                  let vp = v.bounds().size;            // 视口尺寸来自滚动容器
                  let off = v.offset();                // Point<Pixels>: (hscroll 不在此用, vscroll.y)
                  let off2 = h.offset();
                  compute_visible_window(
                      vp.width.0, vp.height.0,
                      off2.x.0, off.y.0,
                      cols, rows,
                  )
              },
              // paint: 双层循环画可见格
              move |bounds, win: VisibleWindow, window, cx| {
                  // 1) 整片背景
                  window.paint_quad(gpui::fill(bounds, theme.content_bg));
                  // 2) 逐格画背景+网格+选中
                  for r in win.r0..win.r1 {
                      for c in win.c0..win.c1 {
                          let x = col_left(c) - win.scroll_x;   // 手动 X 平移
                          let y = row_top(r) - win.scroll_y;    // Y 已随 vscroll 由 GPUI 平移
                          let cb = gpui::Bounds::new(
                              gpui::point(gpui::px(x), gpui::px(y)),
                              gpui::size(gpui::px(CELL_W), gpui::px(CELL_H)),
                          );
                          let is_sel = selected == Some((c, r));
                          paint_cell_background(window, cx, cb, is_sel, &theme);
                          // 文字在 T04 加；本任务先空
                      }
                  }
              },
          )
          .size(gpui::px(total_w), gpui::px(total_h)),
      )
  ```
  `paint_cell_background`（在 `sheet_grid.rs`）实现为每格一个 `gpui::quad`：
  ```rust
  use gpui::{PaintQuad, Edges, BorderStyle, Bounds, point, px};
  pub fn paint_cell_background(
      window: &mut Window, _cx: &mut App,
      bounds: Bounds<Pixels>, is_selected: bool, c: &ThemeColors,
  ) {
      let bg = if is_selected { c.accent } else { c.content_bg };
      // 背景填充（选中用 accent 低透明，非选中用 content_bg）
      let fill_color = if is_selected {
          gpui::Rgba { .. } // 用 c.accent 调低 alpha，例如 c.accent.opacity(0.18)
      } else { c.content_bg };
      window.paint_quad(gpui::fill(bounds, fill_color));
      // 右侧 + 下侧 1px 网格线（避免与相邻格重复，仅画右/下）
      let edges = Edges { top: px(0.), right: px(1.), bottom: px(1.), left: px(0.) };
      window.paint_quad(gpui::quad(
          bounds, 0., gpui::transparent_black(),
          edges, c.border.into(), BorderStyle::default(),
      ));
      if is_selected {
          // 选中再加 2px accent 外框
          window.paint_quad(gpui::quad(
              bounds, 0., gpui::transparent_black(),
              Edges::all(px(2.)), c.accent.into(), BorderStyle::default(),
          ));
      }
  }
  ```
  > 注：`accent.opacity(0.18)` 用 `Rgba::opacity`；`transparent_black()` 为 GPUI 提供的透明色常量。若编译期不确定，直接用 `c.content_bg` 作选中底色亦可。
- **涉及文件**：`sheet_view.rs`、`sheet_grid.rs`、`styles.rs`(取色，不改)
- **前置依赖**：T01, T02
- **验收**：滚动/选择/冻结布局正确——可见区画出带 1px 网格线的单元格底纹；选中格有 accent 高亮；纵向滚轮滚动时网格平滑移动（先不显示文字）；点击/方向键选中仍工作（交互方法未动）。

### T04 ShapedLine 缓存 + 文字绘制  【P0，性能关键】
- **描述**：新建 `sheet_grid_cache.rs`：
  ```rust
  use gpui::{Entity, App, Window, SharedString, TextRun, Pixels, Hsla};
  use gpui::text_system::ShapedLine;
  use std::collections::HashMap;

  #[derive(Clone, PartialEq, Eq, Hash)]
  struct CellCacheKey { row: usize, col: usize, value: String, theme_hash: u64 }

  pub struct GridTextCache { map: HashMap<CellCacheKey, ShapedLine> }
  impl Default for GridTextCache { fn default() -> Self { Self { map: HashMap::new() } } }

  impl GridTextCache {
      pub fn get_or_shape(
          &mut self, row: usize, col: usize, text: &str,
          theme: &ThemeColors, window: &mut Window, _cx: &mut App,
      ) -> ShapedLine {
          let theme_hash = pack_theme(theme);   // accent/text_primary 的 packed u64
          let key = CellCacheKey { row, col, value: text.to_string(), theme_hash };
          if let Some(s) = self.map.get(&key) { return s.clone(); }
          let font = window.text_style().font.clone();       // GPUI 0.2.2 无 Font::default，取当前默认字体
          let run = TextRun {
              len: text.len(),
              font,
              color: theme.text_primary.into(),               // Rgba -> Hsla (From 已实现)
              background_color: None, underline: None, strikethrough: None,
          };
          let shaped = window.text_system().shape_line(
              SharedString::from(text),
              px(FONT_SIZE),                 // FONT_SIZE: f32 = 13.0
              &[run],
              Some(px(CELL_W - 2.0 * PAD)),  // 强制换行宽度 = 单元格内宽
          );
          self.map.insert(key, shaped.clone());
          shaped
      }
      pub fn invalidate_for_sheet(&mut self) { self.map.clear(); }
  }
  ```
  在 `sheet_grid.rs::paint_cell_text` 调用缓存并绘制：
  ```rust
  pub fn paint_cell_text(
      window: &mut Window, cx: &mut App,
      cache: &mut GridTextCache, origin: Point<Pixels>,
      text: &str, c: &ThemeColors,
  ) {
      if text.is_empty() { return; }
      let shaped = cache.get_or_shape(0, 0, text, c, window, cx); // row/col 仅用于 key 区分，见下
      let _ = shaped.paint(origin, px(CELL_H), window, cx);
  }
  ```
  > **key 的 row/col**：为严格按格独立缓存，`paint_cell_text` 应把实际 `r,c` 传入 `get_or_shape(r,c,text,...)`，使同内容不同格各自缓存（更准；代价是重复 shape，但命中率高）。下方 `data-canvas` paint 里直接 `cache.get_or_shape(r, c, text, &theme, window, cx)`。
- **涉及文件**：`sheet_grid_cache.rs`、`sheet_grid.rs`、`sheet_view.rs`(在 `commit_edit`/`clear_selected` 后调 `self.text_cache.update(cx, |c,_| c.invalidate_for_sheet())`)
- **前置依赖**：T03
- **验收**：可见单元格显示文字且对齐；连续滚动/多帧**不再每帧重新 shape**（可在 `get_or_shape` 打点验证命中率接近 100%）；修改某格内容后该格文字更新（失效生效）；性能明显优于原 DOM 方案。

### T05 行号列 canvas 化 + 删旧代码收尾  【P1】
- **描述**：
  1. 行号列替换 `uniform_list("row-numbers", ...)` 为窄 canvas，与数据区共享 `vscroll`（纵向天然同步）：
     ```rust
     div()
         .id("row-scroll")
         .w(px(HEADER_W)).flex_shrink_0()
         .overflow_y_scroll().track_scroll(self.vscroll.clone())
         .child(
             canvas(
                 move |_b, _w, _cx| { /* 行号窗口：读 vscroll.y */ compute_visible_window(HEADER_W, vp_h, 0., v.offset().y.0, 1, rows) },
                 move |_b, win, window, _cx| {
                     for r in win.r0..win.r1 {
                         let y = row_top(r) - win.scroll_y;   // Y 随 vscroll 由 GPUI 平移
                         paint_row_number(window, px(y), r, selected.map(|(_,sr)| sr)==Some(r), &theme);
                     }
                 },
             ).size(px(HEADER_W), px(total_h)),
         )
     ```
     `paint_row_number` 用 `window.paint_quad(gpui::fill(...))` + `window.text_system().shape_line(row_label).paint(...)`，底色 `sidebar_bg`、选中 `accent` 低透明。
  2. 删除 `render_data_row` / `render_row_number` 两个 free function（逻辑已迁到 `sheet_grid.rs`）。
  3. `sheet_view.rs` 顶部 `use gpui::{... uniform_list, UniformListScrollHandle ...}` 改为移除 `uniform_list`/`UniformListScrollHandle`，加入 `canvas`/`ScrollHandle`/`Entity`/`Bounds`/`point`/`px`/`size`/`Edges`/`BorderStyle`/`PaintQuad` 等。
  4. 确认 4 象限对齐：TL 角（DOM）、TR 列标头（`#grid-hscroll` `overflow_x_scroll().track_scroll(&hscroll)`，DOM 字母）、BR 数据 canvas、BL 行号 canvas。横向：列标头与数据区共享 `hscroll`（列标头原生横滚、数据 canvas 读 `hscroll.x` 手动平移）→ 同源对齐。纵向：`row-scroll` 与 `data-scroll` 共享 `vscroll` → 同步。
- **涉及文件**：`sheet_view.rs`、`sheet_grid.rs`
- **前置依赖**：T02, T03, T04
- **验收**：`cargo build` 通过；行号列随数据区纵向滚动完全同步；列标头横向滚动与数据区对齐；旧 `uniform_list`/`render_data_row`/`render_row_number` 无残留引用；手动验证滚动/选中/编辑（公式栏 IME）/写入/删除 全链路正常。

---

## 3. 关键代码决策（基于真实 GPUI 0.2.2 API）

### 3.1 scroll model 最终选型
**三个 `track_scroll` 滚动容器共享两个 `ScrollHandle`（推荐，已验证可行）**，而非「自维护 scroll_offset + 手势」或「单 canvas 被滚动容器 translate」。

- 为什么不用「单 canvas 被滚动容器整体 translate」：GPUI `track_scroll` 一次只能挂**一个** `ScrollHandle`；而数据区需 X 跟 `hscroll`、Y 跟 `vscroll` 做四象限冻结窗格同步，单元素无法同时 track 两个 handle。
- 为什么不用「自维护 scroll_offset + 手势」：需自己实现滚轮/触控板/滚动条/键盘，工作量大且易出 bug；GPUI 原生 `overflow_*_scroll().track_scroll()` 已免费提供这些并暴露 `offset()`。
- 最终布局（3 容器共享 2 handle）：
  - `row-scroll`（左列，宽 `HEADER_W`）：`overflow_y_scroll().track_scroll(&vscroll)` → 行号 canvas 随纵向同步。
  - `col-scroll`（`#grid-hscroll`，顶部）：`overflow_x_scroll().track_scroll(&hscroll)` → DOM 列标头横向滚动。
  - `data-scroll`（BR）：`overflow_y_scroll().overflow_x_hidden().track_scroll(&vscroll)` → 数据 canvas，Y 由 GPUI 平移、X **手动** `-hscroll.x`（因为此容器只 track `vscroll`）。
- **canvas 怎么拿到偏移**：canvas 闭包是 `'static`，我们把 `hscroll.clone()` / `vscroll.clone()`（`ScrollHandle` 内部 `Rc<RefCell>`，可 clone）**move 进 prepaint 闭包**；prepaint 内 `v.offset()` / `h.offset()` 读当前偏移，并 `v.bounds().size` 读视口尺寸，算出 `VisibleWindow` 作为 `T` 返回给 paint。滚动发生在本视图 render 树内 → GPUI 因滚动元素状态变化重跑 `SheetView::render()` → 下一帧所有 canvas 闭包用最新 offset 重画（无需手动 `cx.notify()`；如有滞后可在 scroll 容器挂 `on_scroll_wheel` 显式 `cx.notify()` 兜底）。

### 3.2 可见 (row,col) 窗口计算
算法见 T02 `compute_visible_window`（仿 Calc `AddPixelsWhile`）：从 (0,0) 按 `col_width`/`row_height` **累加**偏移，跳过完全在 `scroll_x`/`scroll_y` 左侧/上方的列/行，直到累加宽/高覆盖视口 `[scroll, scroll+viewport]`。列宽/行高目前为常量 `CELL_W/CELL_H`，但 `col_width(c)`/`row_height(r)` 已形参化，未来接可变列宽表时只改这两个函数。**不能用 `range`/固定行高假设**——必须累加。屏幕坐标：`x = col_left(c) - scroll_x`，`y = row_top(r) - scroll_y`（Y 在 data/row canvas 内已由 GPUI 随 vscroll 平移，故 paint 里对 data-canvas 的 Y 写 `row_top(r) - scroll_y` 仍正确，因为 canvas 元素本身被平移了；等价写法也可直接写 `row_top(r)` 由 GPUI 平移——两者一致，统一用减 scroll 最直观）。

### 3.3 ShapedLine 缓存结构
- **key**：`(row, col, 显示文本, theme_hash)`（`CellCacheKey`，Derive `Hash/Eq`）。`theme_hash` 由 `ThemeColors` 关键色（`accent`/`text_primary`） packed 成 `u64`，主题切换时整体失效。
- **存在哪**：独立 `Entity<GridTextCache>`（`cx.new` 创建，存 `SheetView.text_cache` 字段）。**不**直接存 `SheetView` 普通字段，因为 canvas 的 `paint` 是 `'static` 闭包、且 GPUI 不允许在 paint 期间 `update(self)` 重入；用 `Entity` handle 在 paint 内 `cache.update(cx, ...)` 访问，与 Zed 一致，安全无重入。
- **何时失效**：写入/清空单元格时调 `text_cache.update(cx, |c,_| c.invalidate_for_sheet())`（v1 粗粒度整表清，安全简单）；主题切换时因 `theme_hash` 不同自然不命中。后续可细化到单格失效。
- **取值 API 纠正（重要）**：用 `window.text_system().shape_line(text: SharedString, font_size: Pixels, runs: &[TextRun], force_width: Option<Pixels>) -> ShapedLine`（`text_system.rs:365`）。**不是** `layout_line`（`layout_line` 返回 `Arc<LineLayout>`，不可直接 paint）。`ShapedLine::paint(origin: Point<Pixels>, line_height: Pixels, &mut Window, &mut App)`（`line.rs:63`）。`Font` 用 `window.text_style().font.clone()`（GPUI 0.2.2 无 `Font::default()`）。

### 3.4 编辑态 DOM overlay
- **不引入 in-cell overlay**。现有编辑只走顶部公式栏 `Input`（`begin_edit` 聚焦 `edit_input`，公式栏 `when(editing)` 显示该 `Input`），无独立 in-cell 编辑器。
- canvas 处理：正在编辑的 `edit_target` 单元格在 paint 时**跳过文字绘制**（只画背景/网格），文字由公式栏 `Input` 显示与 IME 输入。行为与原实现完全一致，零新增 IME/光标复杂度。
- （若未来要真正 in-cell 编辑，作为独立增量：在 `data-canvas` 上叠加一个绝对定位的 `Input`，坐标 = `point(px(col_left(c)-scroll_x), px(row_top(r)-scroll_y))`，尺寸 = 单元格大小。）

### 3.5 行号列
- **用 canvas 画**（窄 canvas，宽 `HEADER_W`），与数据区共享 `vscroll` → 纵向天然同步。
- 行号 canvas 在 `row-scroll`（`overflow_y_scroll().track_scroll(&vscroll)`）内，`prepaint` 读 `vscroll.y` 算可见行号区间，`paint` 对每个可见行 `paint_row_number`（底色 `sidebar_bg`、选中 `accent` 低透明、数字 `shape_line` 后 `paint`）。

---

## 4. 要删除的旧代码
- `sheet_view.rs`：`uniform_list("sheet-rows", ...)` 整段替换为 T03 的 `data-scroll` + `canvas`。
- `sheet_view.rs`：`uniform_list("row-numbers", ...)` 整段替换为 T05 的 `row-scroll` + `canvas`。
- `sheet_view.rs`：free function `render_data_row(...)` 与 `render_row_number(...)` 删除（单元格/行号绘制逻辑迁到 `sheet_grid.rs` 的 `paint_cell_background`/`paint_cell_text`/`paint_row_number`）。
- `sheet_view.rs` 顶部 import：`UniformListScrollHandle`、`uniform_list`、`ListHorizontalSizingBehavior`（若不再用）移除；改为引入 `canvas`、`ScrollHandle`、`Entity`、`Bounds`、`point`、`px`、`size`、`Edges`、`BorderStyle`、`PaintQuad` 等。
- `SheetView.vscroll` 字段类型由 `UniformListScrollHandle` 改为 `ScrollHandle`（并新增 `hscroll`、`text_cache`）。

---

## 5. 待明确事项 · 推荐默认值
1. **选中高亮画在哪**：✅ 推荐**全部在 canvas 内**用 `paint_quad` 画（选中底色 `accent` 低透明 + 2px `accent` 外框），不留 DOM overlay，保证零单元格节点。
2. **超大表（>10000 行）是否分块 tiling**：✅ 推荐 **v1 不分块**。每帧只画可见切片（≈视口/CELL ≈ 几十~上百格），`paint_quad` 成本极低；当单帧可见格数 >~2000 再考虑按视口瓦片化。
3. **横向滚动条与 canvas 偏移精确对齐**：✅ 列标头与数据区**共用同一个 `hscroll`**（`col-scroll.track_scroll(&hscroll)` 原生横滚；`data-canvas` 读 `hscroll.offset().x` 做 `x - scroll_x`），同源 → 天然像素级对齐，规避误差。
4. **编辑态 in-cell overlay**：✅ 推荐 v1 **不做**（走公式栏），见 3.4。
5. **行号列纵向滚轮**：✅ v1 行号列本身非滚动手柄承载区，滚轮落在数据区 `data-scroll` 才滚动（已共享 `vscroll`，同步无碍）；如需行号列上也响应滚轮，给 `row-scroll` 内加透明占位即可，低成本增量。
6. **缓存失效粒度**：✅ v1 用 `invalidate_for_sheet()` 整表粗失效（安全、实现简）；后续按需细化到单格。
7. **`ScrollHandle` 重渲染时机**：✅ 依赖「滚动发生在本视图树内 → GPUI 重跑 `render()`」行为（与 `uniform_list` 一致）。实现时重点验证滚动手感；如滞后补 `on_scroll_wheel` 显式 `cx.notify()`。

---
> 附：`system_design.md`（设计总览）、`class-diagram.mermaid`、`sequence-diagram.mermaid` 为本规格的配套图。
