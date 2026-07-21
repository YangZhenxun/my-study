# LibreOffice Calc 表格视图源码研究（面向 Rust / GPUI 移植）

> 目标：提炼出**不靠 origin hack、坐标永不错位**的干净视图所需的设计与数学。
> 结论先行（与前期分析一致）：`ScViewData` 集中持有视图状态；`GetScrPos(col,row,pane)` 只返回「相对本 pane 首个可见单元格」的像素偏移；绝对屏幕坐标 = `pane 屏幕原点 + GetScrPos − scroll_remainder`；**冻结是拆分的特例**；移植要点是「集中状态 struct + 统一坐标映射函数 + pane=同渲染函数+不同参数 + 连续滚动偏移转锚点+余数」。

---

## 0. 取数来源与可信度说明

| 来源文件 | 用途 | 可信度 |
|---|---|---|
| `sc/source/ui/view/viewdata.cxx` | `GetScrPos` / `GetPosX` / `GetPosY` / `WhichH` / `WhichV` 实现 | ✅ 已逐字取回（见第 7 节） |
| `sc/source/ui/view/gridwin.cxx` | `ScGridWindow`（单 pane 窗口）绘制/输入 | ⚠️ 文件过大被截断，按设计描述 |
| `sc/source/ui/view/tabview.cxx` | `ScTabView`（顶层视图，4 个 pane 装配） | ⚠️ 按设计描述 |
| `sc/source/ui/view/output.cxx` + `sc/inc/output.hxx` | `ScOutputData` 无状态绘制器 | ⚠️ 按设计 + 已知签名描述 |
| `sc/inc/viewdata.hxx` | `ScViewData` 成员声明 | ⚠️ master 分支该路径 404（头文件已迁移），成员清单来自源码知识，行号随版本变动 |

> 凡标注「逐字」者，为从 raw 源码取回的真实片段；标注「设计描述」者，为源码行为还原 + 我们建议的 Rust 形态，用于核对逻辑而非逐字比对。

---

## 1. 类结构与职责速查表

| 类 | 职责 | 视图状态存在哪 |
|---|---|---|
| **`ScViewData`** | 视图状态的**唯一真相源**。集中持有：每个 pane 方向的滚动锚点、拆分/冻结模式、缩放、像素/缇换算因子（`nPPT`）。被所有窗口共享。 | 全部视图状态都在这里（含 per-tab 的 `ScViewDataTable`）。 |
| **`ScViewDataTable`**（`ScViewData` 内嵌 per-tab） | 单张表的视图状态副本：`nPosX[2]` / `nPosY[2]`（左右/上下两个方向的锚点）、`eHSplitMode` / `eVSplitMode`。当前表用 `pThisTab`，其余表在 `maTabData[]`。 | 锚点 + 拆分模式按表保存。 |
| **`ScTabView`** | 顶层视图控制器。创建并管理**最多 4 个 `ScGridWindow`**（每个 pane 一个）、滚动条、`ScHSplitWindow`/`ScVSplitWindow` 拆分条，以及 `ScViewData` 引用。 | 不持有状态，只编排；状态全在 `ScViewData`。 |
| **`ScGridWindow`** | **单个 pane 的窗口控件**（VCL `Window`）。负责该 pane 的 `Paint`、鼠标/键盘输入、拖拽。`eWhich` 标识自己是哪个 pane（TL/TR/BL/BR）。 | 只持有 `eWhich` 与 `ScViewData&` 引用；不持有坐标状态。 |
| **`ScOutputData`** | **（近似）无状态**绘制器。给定 `OutputDevice*` + 视图参数（锚点、余数、可见范围、缩放），把文档内容（网格、文字、背景）画到设备上。**4 个 pane 共用同一份绘制代码**，仅参数不同。 | 无长期状态；参数是构造时传入的。 |

**关键洞察**：状态（滚动锚点、模式、缩放）集中于 `ScViewData`；绘制逻辑（`ScOutputData`）与窗口（`ScGridWindow`）都无状态地消费这些状态。这正是「pane = 同渲染函数 + 不同参数」的物理基础。

---

## 2. 核心视图状态数据结构（状态存哪）

`ScViewData` / `ScViewDataTable` 中承载坐标系统的成员（类型与含义，**设计描述**，行号随版本变动）：

```
// 拆分模式（横/纵各一份）
enum ScSplitMode { SC_SPLIT_NONE, SC_SPLIT_NORMAL, SC_SPLIT_FIX };
//   NONE : 不拆分（单 pane）
//   NORMAL: 普通拆分（4 pane 均可独立滚动，splitter 可拖）
//   FIX   : 冻结（top/left pane 锚点锁死，splitter 固定在冻结边缘）

// pane 标识
enum ScSplitPos   { SC_SPLIT_TOPLEFT, SC_SPLIT_TOPRIGHT,
                    SC_SPLIT_BOTTOMLEFT, SC_SPLIT_BOTTOMRIGHT };
enum ScHSplitPos  { SC_SPLIT_LEFT, SC_SPLIT_RIGHT };   // 水平方向索引
enum ScVSplitPos  { SC_SPLIT_TOP,  SC_SPLIT_BOTTOM };  // 垂直方向索引

// —— ScViewDataTable（每张表）——
SCCOL nPosX[2];   // [LEFT]=左 pane 首个可见列, [RIGHT]=右 pane 首个可见列  ← 滚动锚点(列)
SCROW nPosY[2];   // [TOP]=上 pane 首个可见行, [BOTTOM]=下 pane 首个可见行  ← 滚动锚点(行)
ScSplitMode eHSplitMode, eVSplitMode;
SCCOL nFixPosX;   // 冻结列边界（冻结多少列 / 冻结处列号）
SCROW nFixPosY;   // 冻结行边界
tools::Long nHSplitPos, nVSplitPos;  // splitter 的像素位置

// —— ScViewData（全局）——
double nPPTX, nPPTY;     // 「像素 per 缇」换算因子，已含缩放与 DPI
double nZoom, nZoomX, nZoomY;
ScZoomType meZoomType;
```

> 滚动锚点的本质：`nPosX[eHSplitPos]` 是该 pane 方向「第一个（部分）可见单元格」的列号；像素余数 `remX` 是「该锚点列的左边缘相对 pane 原点向左越出的像素数」（见第 3 节）。LibreOffice 把连续滚动偏移编码为 `(锚点列号, 像素余数)` 两部分。

---

## 3. 坐标映射数学模型（最重要）

### 3.1 基本约定

- 文档坐标：列 `col ∈ [0, MaxCol]`，行 `row ∈ [0, MaxRow]`。
- 列宽 `W(col)`、行高 `H(row)`，LibreOffice 内部以 **缇(twips)** 存储；像素尺寸 = `ToPixel(size_twips, nPPT)`（见第 3.6 节）。**我们 Rust 侧直接以像素存储**，缇只在「从文档模型导入」时转换一次。
- 一个 pane 由 `ScSplitPos` 标识（TL/TR/BL/BR）。`WhichH/WhichV` 把 `ScSplitPos` 拆成水平/垂直方向索引。
- `GetScrPos(col,row,pane)` 的返回值 `(dx,dy)` 是：**从「该 pane 首个可见单元格的左上角」到目标单元格左上角**的像素偏移（pane 局部坐标，非绝对屏幕坐标）。

### 3.2 `GetScrPos(col,row,pane)` 精确逻辑（伪代码）

基于 `viewdata.cxx` 逐字实现（见第 7 节）还原的核心算法：

```
GetScrPos(col, row, pane) -> (dx, dy):
    hx = WhichH(pane)      // LEFT / RIGHT
    vy = WhichV(pane)      // TOP  / BOTTOM
    anchorCol = nPosX[hx]  // 该 pane 水平方向锚点列
    anchorRow = nPosY[vy]  // 该 pane 垂直方向锚点行

    dx = 0
    if col >= anchorCol:                       # 正向累加（目标在锚点之后）
        for c = anchorCol .. col-1:
            if W(c) == 0: continue             # 跳过隐藏列
            dx += ToPixel(W(c), nPPTX)
    else:                                      # bAllowNeg：反向累减（目标在锚点之前）
        for c = anchorCol-1 .. col:
            if W(c) == 0: continue
            dx -= ToPixel(W(c), nPPTX)

    dy = 0   # 行同理，用 H(row) / nPPTY
    return (dx, dy)
```

要点：
- 当 `col == anchorCol`（且 `row == anchorRow`）时返回 `(0,0)`：**锚点单元格的左上角恰好落在 pane 局部原点**。
- 隐藏列/行宽度 0，直接跳过（与 LibreOffice 一致）。
- `bAllowNeg=true` 时允许返回负值（用于光标在锚点之上/之前的命中测试）；普通绘制用 `false`。
- **该函数完全不知道「绝对屏幕坐标」**——它只算 pane 局部偏移。绝对坐标由调用方叠加 pane 原点与滚动余数得到。

### 3.3 四个 pane 的屏幕原点与绝对坐标公式

绝对屏幕坐标（相对网格区域左上角，或相对 pane 窗口客户区左上角，两者等价，因为 pane 窗口已是该区域）：

```
screenX(col, pane) = originX(pane) + GetScrPos(col,row,pane).dx - remX(pane)
screenY(row, pane) = originY(pane) + GetScrPos(col,row,pane).dy - remY(pane)
```

其中：

```
originX(pane) = (pane 在 LEFT ? 0 : splitterX)
originY(pane) = (pane 在 TOP  ? 0 : splitterY)
```

- `splitterX` / `splitterY`：拆分线（或冻结线）的像素位置。
- `remX(pane)` / `remY(pane)`：锚点单元格左/上边缘相对 pane 原点向左/上越出的像素数（连续滚动余数）。`rem=0` 表示锚点单元格左边缘正好贴在 pane 原点。

> 这正是前期结论：**绝对坐标 = pane 屏幕原点 + GetScrPos − scroll_remainder**。余数 `rem` 把「像素级平滑滚动」与「单元格级锚点」解耦：滚动只改 `anchorCol/remX`，不改任何窗口几何。

### 3.4 四种情况的具体化（普通滚动 / 冻结首行 / 冻结首列 / 冻结首行+首列）

记 `frozenColsPx` = 冻结列的总像素宽，`frozenRowsPx` = 冻结行的总像素高。

#### (A) 普通滚动（无拆分，`eSplitMode = NONE`）
- 单 pane（逻辑上取 BL 或 TL，origin = (0,0)）。
- `anchorCol = nPosX[LEFT]`, `anchorRow = nPosY[BOTTOM]`。
- `screenX(col) = GetScrPos(col).dx − remX`
- `screenY(row) = GetScrPos(row).dy − remY`
- 滚动：改 `anchorCol/anchorRow + remX/remY`，重绘。

#### (B) 冻结首行（`eVSplitMode = FIX`，冻结前 K 行）
- 垂直拆为 **TOP（冻结）** 与 **BOTTOM（可滚）** 两个 pane；`splitterY = frozenRowsPx`（固定）。
- **TOP pane**：`originY = 0`，`anchorRow = 0`（锁死），`remY = 0`。
  `screenY(row∈冻结行) = 0 + GetScrPos(row, TOP).dy`（恒为静态顶部）。
- **BOTTOM pane**：`originY = frozenRowsPx`，`anchorRow = nPosY[BOTTOM]`（可滚），`remY` 随滚动变。
  `screenY(row) = frozenRowsPx + GetScrPos(row, BOTTOM).dy − remY`。

#### (C) 冻结首列（`eHSplitMode = FIX`，冻结前 M 列）
- 水平拆为 **LEFT（冻结）** 与 **RIGHT（可滚）**；`splitterX = frozenColsPx`。
- LEFT：`originX = 0`，`anchorCol = 0`，`remX = 0` → 静态左侧。
- RIGHT：`originX = frozenColsPx`，`anchorCol = nPosX[RIGHT]`，`remX` 随滚动变。

#### (D) 冻结首行 + 首列（`eHSplitMode = eVSplitMode = FIX`）
四个 pane 同时生效，原点矩阵：

| pane | originX | originY | anchorCol | anchorRow |
|---|---|---|---|---|
| TL（冻结角） | 0 | 0 | 0（锁） | 0（锁） |
| TR（冻结行/滚列） | `frozenColsPx` | 0 | `nPosX[RIGHT]` | 0（锁） |
| BL（滚行/冻结列） | 0 | `frozenRowsPx` | 0（锁） | `nPosY[BOTTOM]` |
| BR（双滚） | `frozenColsPx` | `frozenRowsPx` | `nPosX[RIGHT]` | `nPosY[BOTTOM]` |

每个 pane 套用第 3.3 节的统一公式即可，**无需为 TL 写特例**——它只是「锚点锁 0、rem 0、origin 0」的普通 pane。

### 3.5 逆映射：屏幕像素 → 文档行列（`GetPosFromScrPos` / `GetPosFromPixel`）

给定 pane 内局部像素 `(lx, ly)`（已减去 `origin`），求落在其上的 `(col, row)`：

```
GetPosFromPixel(lx, ly, pane) -> (col, row):
    hx = WhichH(pane); vy = WhichV(pane)
    anchorCol = nPosX[hx]; anchorRow = nPosY[vy]

    target_x = lx + remX(pane)   # 还原为「相对锚点单元格左边缘」的偏移
    target_y = ly + remY(pane)

    col = anchorCol
    acc = 0
    while col <= MaxCol:
        w = ToPixel(W(col), nPPTX)
        if target_x < acc + w: break   # 命中当前列
        acc += w; col += 1
    # 行同理用 target_y / H(row)
    return (col, row)
```

- LibreOffice 实际实现为 `ScViewData::GetPosFromPixel(...)`（签名见第 7 节，未逐字取回）；大表里会用 `ScPositionHelper`（`std::map` + `upper_bound`）做跨隐藏区域的近似二分以避免逐列累加——**算法等价，仅是性能优化**。
- 这再次印证：正/逆映射都只依赖 `(anchor, rem, nPPT, W/H)`，与 pane 是冻结还是拆分无关。

### 3.6 单位换算：缇(twips) ↔ 像素

```
pixel = round( twips * nPPT )      // nPPT 是「像素/缇」，已含 缩放 × DPI
// 等价于：pixel = round( twips * zoom * (screenDPI / 1440) )
```

- `nPPTX` / `nPPTY` 是 `ScViewData` 持有的换算因子；LibreOffice 用定点/分数运算保证精度。
- **我们 Rust 侧的策略**：内部**一律用像素**。列宽/行高在「从文档模型导入」时转换一次：`pxWidth = round(twips * zoom * dpiFactor)`，之后 `ToPixel` 退化为恒等（或仅留整数取整）。`nPPT` 因子被**折叠进存储的像素尺寸**，坐标数学不再出现缇。
- RTL 布局：LibreOffice 对 X 做镜像 `nScrPosX = aScrSize.Width() − 1 − nScrPosX`（见逐字 `GetScrPos` 末尾）。我们可后续在映射层加一个 `mirror_x` 开关，不影响核心数学。

---

## 4. 冻结窗格 vs 拆分窗口（数据流差异）

| 维度 | 拆分（Split, `NORMAL`） | 冻结（Freeze, `FIX`） |
|---|---|---|
| pane 是否都能滚 | 是，4 个 pane 各有独立锚点 | 仅 BOTTOM/RIGHT（及 BR）可滚；TOP/LEFT 锚点**锁死**在冻结边界 |
| splitter 位置 | 用户拖动，任意像素 | 固定在「冻结区域像素尺寸」处（`frozenColsPx`/`frozenRowsPx`） |
| `eSplitMode` | `SC_SPLIT_NORMAL` | `SC_SPLIT_FIX` |
| 状态成员 | `nHSplitPos/nVSplitPos`（可移动） | `nFixPosX/nFixPosY`（冻结边界） |

**核心结论**：二者的**坐标映射数学完全相同**（都是「pane 各自有 anchor + origin，套第 3.3 节公式」）。区别只在**如何维护 `anchor` 与 `splitter`**：
- 拆分：滚动同时更新上下/左右两个锚点，splitter 可动。
- 冻结：滚动只更新 BOTTOM/RIGHT 锚点，TOP/LEFT 锚点恒为冻结边界，splitter 固定在冻结尺寸。

因此**冻结 = 拆分的特例**。Excel 的「冻结首行 / 冻结首列 / 冻结窗格」在 LibreOffice 中正是 `SC_SPLIT_FIX` + 对应 `nFixPosX/Y`。**移植时绝不写两套代码**，只用一个 `SplitMode` 枚举 + 「冻结时锁锚点」的约束即可。

---

## 5. 绘制主流程（Paint）

### 5.1 一次 Paint 如何决定「哪些行列可见、像素起点」

（基于 `ScGridWindow::Paint` + `ScOutputData` 设计还原）

```
Paint(pane):                         # 由 ScGridWindow 对单个 pane 触发
    hx = WhichH(pane); vy = WhichV(pane)
    originX = (hx==LEFT ? 0 : splitterX)
    originY = (vy==TOP  ? 0 : splitterY)
    anchorCol = nPosX[hx];  anchorRow = nPosY[vy]

    # —— 1) 计算可见列范围 ——
    col = anchorCol; x = -remX(pane)
    while col <= MaxCol and x < clientWidth:
        w = ToPixel(W(col), nPPTX)
        if w>0: drawCell(col, ...); x += w; col += 1
    lastVisCol = col

    # —— 2) 计算可见行范围（同理）——
    ...

    # —— 3) 交给无状态绘制器 ——
    ScOutputData out(dev, ...);      # 传入 device + 本 pane 的 anchor/origin/范围/zoom
    out.nScrX = originX - remX;      # 设备内绘制起点（= pane 原点 − 余数）
    out.nScrY = originY - remY;
    out.DrawGrid(); out.DrawStrings(); out.Draw();
```

- **可见范围 = 从 anchor 起，逐列累加像素宽直到超出 client 宽**。与 `GetScrPos` 用同一套 `W/H`，因此永远自洽、不会错位。
- **4 个 pane 各画自己的范围**：TL 从 (0,0) 画冻结角；BR 从 (nPosX[RIGHT], nPosY[BOTTOM]) 画可滚区；都调用**同一个 `ScOutputData::Draw`**。

### 5.2 滚动时哪层更新什么、如何触发重绘

```
用户滚动 / 拖动滚动条
   └─> ScrollHdl / EndScrollHdl
         └─> pViewData->Scroll(...)            # 更新 ScViewData 的锚点
               ├─ nPosX[hx] = newAnchorCol     # 锚点列/行
               ├─ remX = newRemainderPx        # 像素余数
               ├─ pView->Scroll(...)           # 同步滚动条 thumb
               └─ Invalidate(pane 窗口)        # 标记重绘
                     └─> 下一帧 Paint(pane)    # 用新 anchor+rem 重算可见范围并重画
```

- **滚动不移动任何窗口的 origin**（`ScGridWindow` 几何恒定）；只改 `ScViewData` 中的 `(anchor, rem)`，然后 `Invalidate` 触发重绘。内容“位移”完全来自 `GetScrPos` 用新锚点重算——这就是「不靠 origin hack」的根本原因。
- `ScTabView` 负责把滚动事件路由到正确的 pane；`ScViewData::ScrollX/ScrollY/Scroll`（签名见第 7 节，未逐字取回）负责写锚点并通知滚动条。

---

## 6. 移植到 Rust / GPUI 应保留的核心概念清单

- ✅ **集中式视图状态 struct**（`ViewState`）：持有所有 pane 的 `anchor_col/anchor_row` + `rem_x/rem_y` + `split_mode` + `zoom`。等价 `ScViewData`，**不要**把坐标状态分散进窗口/pane。
- ✅ **滚动真相 = (锚点单元格 + 像素余数)**：`(anchor_col, anchor_row, rem_x, rem_y)` 是唯一滚动状态；连续像素滚动只改 `rem`，跨过单元格边界时 `rem` 归零、`anchor` ±1。
- ✅ **统一坐标映射函数** `get_scr_pos(col,row,pane) -> (dx,dy)`：从 pane 锚点累加到目标，与 LibreOffice `GetScrPos` 同算法；隐藏行列宽度 0 跳过。
- ✅ **绝对坐标 = `pane_origin(pane) + get_scr_pos − remainder`**；`pane_origin` 由 splitter 位置决定。不在窗口层做任何 origin 偏移。
- ✅ **4 个 pane 共享同一绘制函数**，仅参数 `(anchor, origin, visible_range, zoom)` 不同。等价 `ScOutputData` 无状态 + 4×`ScGridWindow`。
- ✅ **冻结 = 拆分的特例**：用同一个 `SplitMode` 枚举；冻结时「top/left pane 的 anchor 锁死在冻结边界、splitter 固定在冻结尺寸」。不写两套代码。
- ✅ **像素为唯一内部单位**：缇只在「文档模型 → 视图」导入时转换一次；`nPPT` 因子并入存储的像素尺寸，坐标数学不再出现缇。
- ✅ **可见范围 = anchor 起累加像素宽直到超出 client 宽**；用于绘制与命中测试，保证二者同源。
- ✅ **隐藏行列**：宽度 0，映射/绘制/命中均跳过（可后续用 span 缓存加速大表）。
- ✅ **滚动 → 只更新 `ViewState` + 标记 pane 重绘**：窗口几何恒定，`paint` 用新 anchor 重算。绝不用「移动窗口」模拟滚动。
- 🔶 **RTL 镜像**：作为映射层可选 `mirror_x` 开关，后续支持，不影响核心数学。
- 🔶 **大表性能**：可引入 `ScPositionHelper` 式的「列/行像素前缀和 + 二分」缓存，算法与朴素累加等价，仅优化常数。

---

## 7. 关键 C++ 函数摘录（便于核对）

> 以下 **`GetScrPos` / `GetPosX` / `GetPosY` / `WhichH` / `WhichV`** 为从 `viewdata.cxx` 逐字取回的真实片段（master 分支）。`GetPosFromPixel` / `ScrollX/Y/Scroll` 因文件过大被 fetch 层截断未能逐字取回，仅给**签名与设计描述**。行号随版本变动，请以函数签名检索为准。

### 7.1 `WhichH` / `WhichV` — `sc/source/ui/view/viewdata.cxx`（逐字）
```cpp
ScHSplitPos WhichH(ScSplitPos ePos)
{
    return (ePos == SC_SPLIT_LEFT || ePos == SC_SPLIT_BOTTOMLEFT) ? SC_SPLIT_LEFT : SC_SPLIT_RIGHT;
}

ScVSplitPos WhichV(ScSplitPos ePos)
{
    return (ePos == SC_SPLIT_TOP || ePos == SC_SPLIT_TOPLEFT) ? SC_SPLIT_TOP : SC_SPLIT_BOTTOM;
}
```

### 7.2 `GetScrPos` — `sc/source/ui/view/viewdata.cxx`（逐字核心）
```cpp
Point ScViewData::GetScrPos( SCCOL nWhereX, SCROW nWhereY, ScSplitPos eWhich,
                             bool bAllowNeg, SCTAB nForTab ) const
{
    ScHSplitPos eWhichX = SC_SPLIT_LEFT;
    ScVSplitPos eWhichY = SC_SPLIT_BOTTOM;
    switch( eWhich )
    {
        case SC_SPLIT_TOPLEFT:     eWhichX = SC_SPLIT_LEFT;   eWhichY = SC_SPLIT_TOP;    break;
        case SC_SPLIT_TOPRIGHT:    eWhichX = SC_SPLIT_RIGHT;  eWhichY = SC_SPLIT_TOP;    break;
        case SC_SPLIT_BOTTOMLEFT:  eWhichX = SC_SPLIT_LEFT;   eWhichY = SC_SPLIT_BOTTOM; break;
        case SC_SPLIT_BOTTOMRIGHT: eWhichX = SC_SPLIT_RIGHT;  eWhichY = SC_SPLIT_BOTTOM; break;
    }
    // ...（nForTab / tiled-rendering 分支略，桌面端不走）...

    SCCOL nPosX = GetPosX(eWhichX, nForTab);
    tools::Long nScrPosX = 0;
    if (bAllowNeg || nWhereX >= nPosX)
    {
        for (SCCOL nX = nPosX; nX < nWhereX; nX++)
        {
            sal_uInt16 nTSize = mrDoc.GetColWidth(nX, nForTab);
            if (nTSize)
                nScrPosX += ToPixel( nTSize, nPPTX );   // 累加列宽(像素)
            // else 隐藏列：跳过（群跳逻辑略）
        }
    }
    // 行方向 nScrPosY 同理，用 GetRowHeight / nPPTY

    if (mrDoc.IsLayoutRTL(nForTab) /* && !tiled */)
        nScrPosX = aScrSize.Width() - 1 - nScrPosX;     // RTL 镜像
    return Point( nScrPosX, nScrPosY );
}
```
> 解读：`nPosX = GetPosX(eWhichX)` 即「该 pane 水平方向锚点列」；返回值是**从锚点列左边缘到目标列左边缘的像素累加**——正是第 3.2 节的算法。注意 `nWhereX == nPosX` 时循环不执行，返回 0，印证「锚点单元格左边缘 = pane 局部原点」。

### 7.3 `GetPosX` / `GetPosY` — `sc/source/ui/view/viewdata.cxx`（逐字）
```cpp
SCCOL ScViewData::GetPosX( ScHSplitPos eWhich, SCTAB nForTab ) const
{
    if (comphelper::LibreOfficeKit::isActive()) return 0;   // LOK 平铺渲染无锚点
    if (nForTab == -1) return pThisTab->nPosX[eWhich];
    if (!ValidTab(nForTab) || (nForTab >= static_cast<SCTAB>(maTabData.size()))) return -1;
    return maTabData[nForTab]->nPosX[eWhich];
}
// GetPosY 同构，返回 pThisTab->nPosY[eWhich]
```
> 解读：`nPosX[eWhich]` 就是第 2/3 节反复引用的「滚动锚点」。它按 `ScHSplitPos`（LEFT/RIGHT）索引 → 左右 pane 各有独立锚点；冻结时 top/left pane 的锚点被锁在冻结边界。

### 7.4 `GetPosFromPixel` — `sc/source/ui/view/viewdata.cxx`（签名 + 设计描述，未逐字）
```cpp
// 签名（设计还原，待核对）：
void ScViewData::GetPosFromPixel( SCCOL& rCol, SCROW& rRow,
                                  ScSplitPos eWhich,
                                  tools::Long nPixelX, tools::Long nPixelY,
                                  bool bTestMerge );
```
- 行为：见第 3.5 节。先 `WhichH/WhichV` 取方向，取该 pane 锚点，把入参像素加回 `rem` 还原为「相对锚点」偏移，再逐列/行累加 `ToPixel(W/H)` 直到越过目标，得到命中行列。大表用 `ScPositionHelper`（`std::map` + `upper_bound`）近似二分加速。

### 7.5 `ScrollX` / `ScrollY` / `Scroll` — `sc/source/ui/view/viewdata.cxx`（签名 + 设计描述，未逐字）
```cpp
// 签名（设计还原，待核对）：
void ScViewData::ScrollX( SCCOL nNewPosX, ScHSplitPos eWhich );
void ScViewData::ScrollY( SCROW nNewPosY, ScVSplitPos eWhich );
void ScViewData::Scroll( ... );
```
- 行为：写入 `pThisTab->nPosX[eWhich] = nNewPosX`（或 `nPosY`），同步滚动条 `thumb`，并 `Invalidate` 对应 pane 窗口触发重绘（见第 5.2 节）。**连续像素滚动的余数 `rem` 由滚动条像素位置与锚点列像素左缘之差得到**，未在 `ScViewData` 单独存储为成员，而是由 `GetScrPos` 配合 `bAllowNeg` 与滚动条状态隐含表达——这正是我们移植时要**显式**拆成 `(anchor, rem)` 的原因。

### 7.6 `ScOutputData` — `sc/inc/output.hxx`（签名 + 设计描述）
```cpp
// 构造函数（设计还原）：
ScOutputData::ScOutputData( OutputDevice* pDev, ... );
// 关键成员（设计还原）：
//   tools::Long nScrX, nScrY;   // 设备内绘制起点 = pane 原点 − rem
//   tools::Long nExcelLeft, nExcelTop, nMirrorX;
//   bool bTabProtected;
// 主要绘制方法：
//   void DrawGrid(); void DrawStrings(); void Draw();
```
> 解读：`ScOutputData` 不持有视图状态，参数（含 `nScrX/nScrY` 起点、`anchor`、可见范围）构造时传入 → **同一份绘制代码服务 4 个 pane**，仅参数不同。第 6 节「pane=同渲染函数+不同参数」即源于此。

---

## 8. 一句话移植定理（给架构师）

> **把 LibreOffice 的 `(nPosX[2], nPosY[2], eSplitMode, nFixPos, splitterPx, nPPT)` 收进一个 `ViewState`；把 `GetScrPos` 收进一个 `get_scr_pos(col,row,pane)`；绝对坐标永远走 `origin(pane) + get_scr_pos − rem`；冻结只是 `eSplitMode=FIX` 下「锁锚点 + 固定 splitter」的拆分。做到这四点，坐标在数学上自洽，永远不需要 origin hack。**
