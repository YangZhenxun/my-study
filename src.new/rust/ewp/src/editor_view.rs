//! 编辑器视图 — Pages 风格布局（左侧画布 + 右侧格式侧栏）
//!
//! 布局（从外到内）：
//! ┌─────────────────────────────────────────────────────┐
//! │  工具栏：文件名 · B I U 对齐按钮 · 保存 · 侧栏开关    │
//! ├────────────────────────────┬────────────────────────┤
//! │                            │  格式侧栏               │
//! │  主画布编辑区（白底页面式）  │  [文本 ▾]              │
//! │  行号 + 文本 + 光标         │  样式 | 布局 | 更多     │
//! │                            │                        │
//! │                            │  字体 / 大小 / B I U   │
//! │                            │  对齐 / 颜色 ...        │
//! ├────────────────────────────┴────────────────────────┤
//! │  状态栏：行/列 · 总行数 · 字数 · 脏标记 ●             │
//! └─────────────────────────────────────────────────────┘

use gpui::{
    anchored, deferred, AnyElement, App, Bounds, ClickEvent, Context, Corner, Element, ElementId,
    ElementInputHandler, Entity, EntityInputHandler, FocusHandle, Focusable, FontWeight,
    GlobalElementId,
    InspectorElementId, KeyDownEvent, LayoutId, MouseButton, Pixels, Point, Render, Rgba,
    ScrollHandle, SharedString, Size, TextRun, UTF16Selection, Window, div, px, point, rgba,
};
use gpui::prelude::*;
use std::ops::Range;
use rust_i18n::t;

use crate::data;
use crate::model::ser::NativeFormat;
use crate::model::text::{Block, Document, Paragraph, Run};
use crate::model::Model;
use crate::styles::ThemeColors;
use std::path::PathBuf;

// ════════════════════════════════════════
// 枚举
// ════════════════════════════════════════

/// 格式侧栏的标签页。
#[derive(Clone, Copy, PartialEq, Eq)]
enum FormatTab {
    Style,
    Layout,
    More,
}

/// 文本对齐方式。
#[derive(Clone, Copy, PartialEq, Eq)]
enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    fn next(self) -> Self {
        match self {
            TextAlign::Left => TextAlign::Center,
            TextAlign::Center => TextAlign::Right,
            TextAlign::Right => TextAlign::Justify,
            TextAlign::Justify => TextAlign::Left,
        }
    }

    /// 返回对齐的 i18n 键。
    #[allow(dead_code)]
    fn label_key(&self) -> &'static str {
        match self {
            TextAlign::Left => "editor.align_left",
            TextAlign::Center => "editor.align_center",
            TextAlign::Right => "editor.align_right",
            TextAlign::Justify => "editor.align_justify",
        }
    }
}

// 字体候选（点击「字体」行循环切换）
const FONT_CHOICES: &[&str] = &["PingFang SC", "Helvetica", "Georgia", "Menlo", "Songti SC"];

// 可选文本颜色数量（不含索引 0 = 跟随主题）
const TEXT_COLOR_COUNT: usize = 4;

/// 返回某颜色索引对应的实际颜色（0 = 跟随主题）。
fn color_at(index: usize, c: &ThemeColors) -> Rgba {
    match index {
        0 => c.text_primary,
        1 => rgba(0xd83931ff),
        2 => rgba(0x0a84ffff),
        3 => rgba(0x34c759ff),
        _ => rgba(0xff9f0aff),
    }
}

// ════════════════════════════════════════
// EditorView
// ════════════════════════════════════════

/// 编辑器根视图 —— Pages 风格布局。
pub struct EditorView {
    focus: FocusHandle,
    /// 窗口标题栏显示的名字（如 "Untitled" / 文件名）。
    name: SharedString,
    /// 多行文本缓冲，最后一行即正在编辑的行。
    lines: Vec<String>,
    /// 当前光标所在行。
    caret_line: usize,
    /// 当前光标在该行的列（字符索引）。
    caret_col: usize,
    /// 背后的原生模型（保存时再映射回去，本期未做富文本往返）。
    #[allow(dead_code)]
    model: Model,
    /// 已保存路径；None 表示从未保存过（保存时落到默认 data/ 目录）。
    path: Option<PathBuf>,
    /// 是否有未保存的修改（用于标题星号与状态栏标记）。
    dirty: bool,

    // ── Pages 风格新增状态 ──
    /// 格式侧栏是否展开。
    sidebar_open: bool,
    /// 当前激活的侧栏标签页。
    active_tab: FormatTab,
    /// 字号（UI 显示用，本期未实际应用）。
    font_size: u16,
    bold: bool,
    italic: bool,
    underline: bool,
    alignment: TextAlign,
    /// 字体族（UI 显示用，本期部分应用）。
    font_family: String,
    /// 文本颜色索引（0 = 跟随主题）。
    color_index: usize,
    /// 字体下拉是否展开（悬浮菜单）。
    font_dropdown_open: bool,
    /// 字体下拉锚点（窗口坐标，点击字体行时记录，用于悬浮定位）。
    font_dropdown_anchor: Point<Pixels>,
    /// 输入法合成态（marked text）范围，以「当前行」的 UTF-16 偏移表示。
    /// `None` 表示非合成态（普通输入）。仅当焦点在当前行时才有意义。
    marked_range: Option<Range<usize>>,
    /// 主画布滚动句柄（#canvas 用 `track_scroll` 绑定）。
    /// `bounds_for_range` 据此扣除滚动偏移，使 IME 候选窗随画布滚动正确对齐。
    scroll_handle: ScrollHandle,
}

impl EditorView {
    pub fn new_blank(cx: &mut Context<Self>, name: SharedString) -> Self {
        Self::build(cx, name, Model::Text(Document::default()), None)
    }

    pub fn new_from_model(
        cx: &mut Context<Self>,
        name: SharedString,
        model: Model,
        path: Option<PathBuf>,
    ) -> Self {
        Self::build(cx, name, model, path)
    }

    fn build(
        cx: &mut Context<Self>,
        name: SharedString,
        model: Model,
        path: Option<PathBuf>,
    ) -> Self {
        let lines = extract_lines(&model);
        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };
        Self {
            focus: cx.focus_handle(),
            name,
            lines,
            caret_line: 0,
            caret_col: 0,
            model,
            path,
            dirty: false,

            sidebar_open: true,
            active_tab: FormatTab::Style,
            font_size: 11,
            bold: false,
            italic: false,
            underline: false,
            alignment: TextAlign::Left,
            font_family: "PingFang SC".to_string(),
            color_index: 0,
            font_dropdown_open: false,
            font_dropdown_anchor: point(px(0.), px(0.)),
            marked_range: None,
            scroll_handle: ScrollHandle::new(),
        }
    }

    // ── 键盘处理（保留原有逻辑） ──

    /// 处理键盘事件。返回 `true` 表示已消费（调用方应 `stop_propagation`）。
    ///
    /// 设计要点（GPUI 0.2.2 macOS 输入路径）：
    /// - 可打印字符（含空格/字母/中文候选）**不在此处插入**，而是交给输入上下文
    ///   `handleEvent → insertText / setMarkedText → EntityInputHandler`，避免与合成态冲突。
    /// - Enter/Tab 是结构键，必须在此处理并阻止进入输入法（否则会被当成换行/制表符字符插入）。
    /// - 退格/方向/Home/End 等没有 `key_char`，会先经 `handleEvent → doCommand` 再被派发回此处。
    fn handle_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        let ks = &event.keystroke;

        // ⌘/Ctrl + S：保存
        if (ks.modifiers.platform || ks.modifiers.control)
            && (ks.key == "s" || ks.key_char.as_deref() == Some("s"))
        {
            self.save_document(cx);
            return true;
        }

        // 带 ctrl/cmd/alt 的组合键：交给快捷键系统，不消费
        if ks.modifiers.platform || ks.modifiers.control || ks.modifiers.alt {
            return false;
        }

        // 结构键：在 on_key_down 直接处理并阻止进入输入法
        match ks.key.as_str() {
            "enter" => {
                self.split_line();
                self.dirty = true;
                cx.notify();
                return true;
            }
            "tab" => {
                self.insert_tab();
                self.dirty = true;
                cx.notify();
                return true;
            }
            _ => {}
        }

        // 其余可打印字符：交给输入上下文（handleEvent → insertText / IME 合成），
        // 不在此处插入，避免与输入法合成态冲突导致重复插入。
        if ks.key_char.is_some() {
            return false;
        }

        // 导航 / 编辑键（退格、方向、Home/End 等）
        match ks.key.as_str() {
            "backspace" => {
                let col = self.caret_col;
                if col > 0 {
                    let line = &mut self.lines[self.caret_line];
                    let byte = char_byte_index(line, col - 1);
                    line.remove(byte);
                    self.caret_col -= 1;
                    self.dirty = true;
                } else if self.caret_line > 0 {
                    let cur = self.lines.remove(self.caret_line);
                    self.caret_line -= 1;
                    self.caret_col = self.lines[self.caret_line].chars().count();
                    self.lines[self.caret_line].push_str(&cur);
                    self.dirty = true;
                }
                cx.notify();
                return true;
            }
            "home" => {
                self.caret_col = 0;
                cx.notify();
                return true;
            }
            "end" => {
                self.caret_col = self.lines[self.caret_line].chars().count();
                cx.notify();
                return true;
            }
            "left" => {
                if self.caret_col > 0 {
                    self.caret_col -= 1;
                }
                cx.notify();
                return true;
            }
            "right" => {
                let len = self.lines[self.caret_line].chars().count();
                if self.caret_col < len {
                    self.caret_col += 1;
                }
                cx.notify();
                return true;
            }
            "up" => {
                if self.caret_line > 0 {
                    self.caret_line -= 1;
                    self.clamp_caret();
                }
                cx.notify();
                return true;
            }
            "down" => {
                if self.caret_line + 1 < self.lines.len() {
                    self.caret_line += 1;
                    self.clamp_caret();
                }
                cx.notify();
                return true;
            }
            _ => return false,
        }
    }

    /// 在光标处拆行（Enter）。
    fn split_line(&mut self) {
        let cur = self.lines[self.caret_line].clone();
        let byte = char_byte_index(&cur, self.caret_col);
        let (left, right) = cur.split_at(byte);
        self.lines[self.caret_line] = left.to_string();
        self.lines.insert(self.caret_line + 1, right.to_string());
        self.caret_line += 1;
        self.caret_col = 0;
    }

    /// 在光标处插入两个空格（Tab）。
    fn insert_tab(&mut self) {
        let line = &mut self.lines[self.caret_line];
        let byte = char_byte_index(line, self.caret_col);
        line.insert(byte, ' ');
        line.insert(byte + 1, ' ');
        self.caret_col += 2;
    }

    fn clamp_caret(&mut self) {
        let len = self.lines[self.caret_line].chars().count();
        if self.caret_col > len {
            self.caret_col = len;
        }
    }

    /// 在「当前行」文本空间内以 UTF-16 坐标替换/插入文本，并把光标移到插入点之后。
    ///
    /// `range = None` 表示替换当前选择：合成态下=标记范围，否则=光标处的空范围。
    /// 这是 `EntityInputHandler` 两个写入方法的统一落点（提交 / 合成）。
    fn replace_range(&mut self, range: Option<Range<usize>>, text: &str) {
        let caret_line = self.caret_line;
        let cur_marked = self.marked_range.clone();
        let caret_utf16 = utf16_len_up_to(&self.lines[caret_line], self.caret_col);
        let (start_u, end_u) = match range {
            Some(r) => (r.start, r.end),
            None => match &cur_marked {
                Some(m) => (m.start, m.end),
                None => (caret_utf16, caret_utf16),
            },
        };

        let line = &mut self.lines[caret_line];
        let s = utf16_to_byte(line, start_u);
        let e = utf16_to_byte(line, end_u);
        line.replace_range(s..e, text);

        let new_caret_utf16 = start_u + text.chars().map(|c| c.len_utf16()).sum::<usize>();
        self.caret_col = utf16_to_char(line, new_caret_utf16);
        self.dirty = true;
    }

    /// 切换格式侧栏开/关。
    fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_open = !self.sidebar_open;
        cx.notify();
    }

    /// 切换格式属性（bold/italic/underline/alignment/font/color）。
    fn toggle_format(&mut self, which: &str, cx: &mut Context<Self>) {
        match which {
            "bold" => self.bold = !self.bold,
            "italic" => self.italic = !self.italic,
            "underline" => self.underline = !self.underline,
            "align" => self.alignment = self.alignment.next(),
            "cycle-font" => self.cycle_font(),
            "font-inc" => self.adjust_font_size(1),
            "font-dec" => self.adjust_font_size(-1),
            "cycle-color" => self.cycle_color(),
            _ => {}
        }
        cx.notify();
    }

    /// 循环切换字体族（仅 UI 状态，暂不强加渲染）。
    fn cycle_font(&mut self) {
        let idx = FONT_CHOICES
            .iter()
            .position(|f| *f == self.font_family)
            .unwrap_or(0);
        let next = (idx + 1) % FONT_CHOICES.len();
        self.font_family = FONT_CHOICES[next].to_string();
    }

    /// 调整字号（夹紧 8–72）。
    fn adjust_font_size(&mut self, delta: i16) {
        let next = (self.font_size as i16 + delta).clamp(8, 72);
        self.font_size = next as u16;
    }

    /// 循环切换文本颜色（索引 0 = 跟随主题）。
    fn cycle_color(&mut self) {
        self.color_index = (self.color_index + 1) % (TEXT_COLOR_COUNT + 1);
    }

    /// 展开/收起字体下拉菜单（记录点击位置作为悬浮锚点）。
    fn toggle_font_dropdown(&mut self, anchor: Point<Pixels>, cx: &mut Context<Self>) {
        self.font_dropdown_anchor = anchor;
        self.font_dropdown_open = !self.font_dropdown_open;
        cx.notify();
    }

    /// 从下拉菜单选中字体并收起。
    fn select_font(&mut self, idx: usize, cx: &mut Context<Self>) {
        if let Some(f) = FONT_CHOICES.get(idx) {
            self.font_family = f.to_string();
        }
        self.font_dropdown_open = false;
        cx.notify();
    }

    /// 把当前多行缓冲映射回原生 `Document`（每行一个段落）。
    fn to_document(&self) -> Document {
        let blocks = self
            .lines
            .iter()
            .map(|line| {
                Block::Paragraph(Paragraph {
                    runs: vec![Run {
                        text: line.clone(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
            })
            .collect();
        Document {
            blocks,
            ..Default::default()
        }
    }

    /// 保存到磁盘。
    fn save_document(&mut self, cx: &mut Context<Self>) {
        let doc = self.to_document();
        let path = self.path.clone().unwrap_or_else(|| {
            let safe = self.name.replace(['/', '\\', ':'], "_");
            data::data_dir().join(format!("{safe}.ewp"))
        });

        if let Err(e) = crate::model::ser::save(&Model::Text(doc), &path, NativeFormat::Json) {
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
                file_type: data::FileType::Document,
            },
        );
        cx.notify();
    }
}

impl Focusable for EditorView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

// ════════════════════════════════════════
// EntityInputHandler — 中文/IME 输入
// ════════════════════════════════════════
//
// GPUI 0.2.2 把文本输入抽象成「文档 + UTF-16 坐标」。我们把「当前正在编辑的那一行」
// 当作这个文档：所有 range 都是相对于 `self.lines[self.caret_line]` 的 UTF-16 偏移。
// 合成态（marked text）用 `marked_range` 记录，渲染时加下划线并据此定位候选窗。

impl EntityInputHandler for EditorView {
    fn text_for_range(
        &mut self,
        range: Range<usize>,
        _adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let line = &self.lines[self.caret_line];
        let s = utf16_to_byte(line, range.start);
        let e = utf16_to_byte(line, range.end);
        Some(line[s..e].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let caret_utf16 = utf16_len_up_to(&self.lines[self.caret_line], self.caret_col);
        let range = self
            .marked_range
            .clone()
            .unwrap_or_else(|| caret_utf16..caret_utf16);
        Some(UTF16Selection { range, reversed: false })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range.clone()
    }

    fn unmark_text(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(m) = self.marked_range.take() {
            let line = &self.lines[self.caret_line];
            self.caret_col = utf16_to_char(line, m.end);
            self.dirty = true;
            cx.notify();
        }
    }

    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 提交：替换 range（None = 当前选择/合成态），清除合成态。
        self.replace_range(range, text);
        self.marked_range = None;
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 合成：替换 range 并标记为 marked text。
        let caret_line = self.caret_line;
        let cur_marked = self.marked_range.clone();
        let caret_utf16 = utf16_len_up_to(&self.lines[caret_line], self.caret_col);
        let ins_utf16 = match range {
            Some(ref r) => r.start,
            None => match &cur_marked {
                Some(m) => m.start,
                None => caret_utf16,
            },
        };

        self.replace_range(range, new_text);

        let new_len_utf16 = new_text.chars().map(|c| c.len_utf16()).sum::<usize>();
        self.marked_range = Some(ins_utf16..ins_utf16 + new_len_utf16);

        // 光标定位到 IME 给出的选中段末尾，否则定位到合成段末尾。
        let caret_utf16 = match new_selected_range {
            Some(sel) => sel.end,
            None => ins_utf16 + new_len_utf16,
        };
        let line = &self.lines[self.caret_line];
        self.caret_col = utf16_to_char(line, caret_utf16);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        // 参照 Zed editor（crates/editor/src/input.rs:3032）的 bounds_for_range：
        // 返回「窗口 content 坐标」下的光标矩形——macOS 的 firstRectForCharacterRange 会自行
        // 叠加窗口 frame 原点并翻转 y 轴。所以最终 origin = element_bounds.origin + 元素内局部坐标。
        //
        // 这里把「当前正在编辑的那一行」当作 IME 文档，光标位置由 self.caret_line / self.caret_col 给出。
        // 用 window.text_system().shape_line 测量光标之前的文本像素宽，从而得到精确的 x 偏移；
        // 行高用同一字体的空格度量（ascent + descent），与画布渲染一致。
        //
        // 画布（#canvas）可滚动，其滚动偏移由 self.scroll_handle 跟踪：
        //   scroll.y 向下为负，scroll.x 向右为负（ScrollHandle::offset()）。
        // 局部坐标 local_* 是按「内容顶部」度量的，屏幕上还要叠加滚动偏移才是视口坐标，
        // 故 local_y 最终 + scroll.y、local_x + scroll.x（与 Zed scroll_position() 减法的语义一致）。
        // 零 panic 守卫：本函数由 macOS IME 的 firstRectForCharacterRange（extern "C"）
        // 同步回调，任何 unwrap / 索引越界都会跨 FFI 边界触发 abort 直接崩溃。
        // 状态异常时一律返回 None（GPUI 映射为零矩形，候选窗退到默认位置），绝不 panic。
        let line = self.lines.get(self.caret_line)?;

        let font_size_px = px(self.font_size as f32);
        let font = gpui::font(self.font_family.clone());

        // 当前行中、光标之前的前缀文本（按字符切分），用于测量 x 偏移。
        // 锚点优先用 IME 请求的 range 起点（合成态=合成段起点、选区=选区起点），否则用当前光标。
        let caret_char = if !_range_utf16.is_empty() {
            let total = line.chars().map(|c| c.len_utf16()).sum::<usize>();
            if _range_utf16.start <= total {
                utf16_to_char(line, _range_utf16.start)
            } else {
                self.caret_col
            }
        } else {
            self.caret_col
        };
        // 用真实 ShapedLine 的 x_for_index 取光标 x（与渲染完全一致，不再手工估算宽度）。
        // 整行作为一个 run 测量，run.len == 整行字节长，绝不会触发 layout_line 切片越界。
        let line_str: SharedString = SharedString::from(line.to_string());
        let line_run = TextRun {
            len: line_str.len(),
            font: font.clone(),
            color: gpui::black(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let shaped = window
            .text_system()
            .shape_line(line_str, font_size_px, &[line_run], None);
        let caret_byte = char_byte_index(line, caret_char);
        let mut prefix_w = shaped.x_for_index(caret_byte);
        // 当前行渲染时在光标处插了一个 2px 宽的光标竖线 div，x 需补上。
        if caret_char > 0 {
            prefix_w += px(2.);
        }

        // 行高：render() 的每一行都用同一个 line_height() 显式定高，
        // 因此这里算出的 y 与真实渲染逐行对齐（不再用 ascent+descent 估算）。
        let lh = line_height(self.font_size);

        // 画布内部布局常量（必须与 render() 中 #canvas 的 padding 和 行号 gutter 严格一致）：
        let canvas_pad_x = px(48.0); // canvas .px(px(48.))
        let canvas_pad_y = px(40.0); // canvas .py(px(40.))
        let gutter_w = px(36.0); // gutter .w(px(36.))；pr_2() 是 gutter 内部右内边距，不额外占行间距
        let text_origin_x = canvas_pad_x + gutter_w;

        // 叠加滚动偏移：把「内容坐标」换算回「视口坐标」（scroll.y/x 向下/向右为负，直接相加即抵消）。
        let scroll = self.scroll_handle.offset();
        let local_x = text_origin_x + prefix_w + scroll.x;
        let local_y = canvas_pad_y + lh * (self.caret_line as f32) + scroll.y;

        Some(Bounds {
            origin: Point::new(
                element_bounds.origin.x + local_x,
                element_bounds.origin.y + local_y,
            ),
            size: Size::new(px(self.font_size as f32), lh),
        })
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        None
    }
}

// ════════════════════════════════════════
// Render — Pages 风格布局
// ════════════════════════════════════════

impl Render for EditorView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();
        let focused = self.focus.is_focused(window);
        let name = self.name.clone();
        let caret_line = self.caret_line;
        let caret_col = self.caret_col;
        let c = ThemeColors::current();

        let line_count = self.lines.len();
        let char_count: usize = self.lines.iter().map(|l| l.chars().count()).sum();

        // 预渲染所有行视图；合成态（marked）只在当前行生效。
        let line_views: Vec<_> = self
            .lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
            let is_caret = focused && i == caret_line;
            build_line(
                i,
                line,
                is_caret,
                caret_col,
                &c,
                self.font_size,
                self.color_index,
                &self.font_family,
                if is_caret { self.marked_range.clone() } else { None },
            )
        })
            .collect();

        // 格式切换回调（顶栏与侧栏的 B/I/U/对齐按钮共用）
        let on_format = {
            let this = this.clone();
            move |which: &str, _: &ClickEvent, _: &mut Window, cx: &mut App| {
                let this = this.clone();
                let w = which.to_string();
                let _ = this.update(cx, |this, cx| {
                    this.toggle_format(&w, cx)
                });
            }
        };

        // 字体下拉：点击触发器展开/收起
        let on_toggle_font_dropdown = {
            let this = this.clone();
            move |event: &ClickEvent, _: &mut Window, cx: &mut App| {
                let pos = event.position();
                let _ = this.update(cx, |this, cx| this.toggle_font_dropdown(pos, cx));
            }
        };
        // 字体下拉：选中某一项
        let on_pick_font = {
            let this = this.clone();
            move |idx: usize, _: &ClickEvent, _: &mut Window, cx: &mut App| {
                let _ = this.update(cx, |this, cx| this.select_font(idx, cx));
            }
        };

        // ── 根容器：横向排列（左画布区 + 右侧栏） ──
        div()
            .id("editor-root")
            .track_focus(&self.focus) // 关键：让 FocusHandle 与 DOM 节点关联，否则 on_key_down 收不到按键
            .size_full()
            .flex()
            .flex_row()
            .bg(c.window_bg)
            .on_mouse_down(MouseButton::Left, {
                let focus = self.focus.clone();
                move |_, window, _| window.focus(&focus)
            })
            .on_key_down({
                let this = this.clone();
                move |event: &KeyDownEvent, _window, cx| {
                    let consumed = this.update(cx, |this, cx| this.handle_key(event, cx));
                    if consumed {
                        cx.stop_propagation();
                    }
                }
            })
            // ═══ 左侧区域：工具栏 + 画布 + 状态栏 ═══
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    // ── 顶栏 ──
                    .child(top_toolbar(
                        &name,
                        self.dirty,
                        self.sidebar_open,
                        &c,
                        {
                            let this = this.clone();
                            move |_, _, cx: &mut App| {
                                let this = this.clone();
                                let _ =
                                    this.update(cx, |this, cx| this.save_document(cx));
                            }
                        },
                        {
                            let this = this.clone();
                            move |_, _, cx: &mut App| {
                                let this = this.clone();
                                let _ = this.update(cx, |this, cx| {
                                    this.toggle_sidebar(cx)
                                });
                            }
                        },
                        on_format.clone(),
                    ))
                    // ── 画布外框（灰色背景模拟页面阴影） ──
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .px(px(32.))
                            .py(px(24.))
                            .bg(c.border) // 浅灰边框色作画布外围
                            // ── 白纸画布（可滚动） ──
                            // 用 InputRegion 包一层：在 paint 阶段向窗口注册 EntityInputHandler，
                            // 从而让 macOS 输入法把合成文本（中文拼音→汉字）投递到本视图。
                            .child(
                                InputRegion {
                                    view: this.clone(),
                                    focus: self.focus.clone(),
                                    child: div()
                                        .id("canvas")
                                        .track_scroll(&self.scroll_handle)
                                        .flex()
                                        .flex_col()
                                        .size_full()
                                        .overflow_y_scroll()
                                        .rounded_md()
                                        .bg(c.content_bg) // 白底页面
                                        .px(px(48.))
                                        .py(px(40.))
                                        .text_base()
                                        .text_color(c.text_primary)
                                        .children(line_views)
                                        .into_any_element(),
                                },
                            ),
                    )
                    // ── 状态栏 ──
                    .child(status_bar(
                        caret_line,
                        caret_col,
                        line_count,
                        char_count,
                        self.dirty,
                        &c,
                    )),
            )
            // ═══ 右侧格式侧栏（条件显示） ═══
            .when(self.sidebar_open, |root| {
                root.child(format_sidebar(
                    self.active_tab,
                    self.font_size,
                    self.bold,
                    self.italic,
                    self.underline,
                    self.alignment,
                    &self.font_family,
                    self.color_index,
                    &c,
                    on_format,
                    on_toggle_font_dropdown,
                    {
                        let this = this.clone();
                        move |tab: FormatTab, _: &ClickEvent, _: &mut Window, cx: &mut App| {
                            let this = this.clone();
                            let _ = this.update(cx, |this, cx| {
                                this.active_tab = tab;
                                cx.notify();
                            });
                        }
                    },
                ))
            })
            // ═══ 字体悬浮下拉（浮在内容上方，不撑开布局） ═══
            .when(self.font_dropdown_open, |root| {
                let c = c.clone();
                let viewport = window.viewport_size();
                let font_family_now = self.font_family.clone();
                let this2 = this.clone();
                // 遮罩：点外部关闭
                let backdrop = deferred(
                    anchored()
                        .position(point(px(0.), px(0.)))
                        .child(
                            div()
                                .id("font-dropdown-backdrop")
                                .w(viewport.width)
                                .h(viewport.height)
                                .bg(rgba(0x00000000))
                                .on_mouse_down(MouseButton::Left, {
                                    let t = this2.clone();
                                    move |_, _, cx: &mut App| {
                                        let _ = t.update(cx, |this, cx| {
                                            this.font_dropdown_open = false;
                                            cx.notify();
                                        });
                                    }
                                }),
                        ),
                );
                // 面板
                let panel = deferred(
                    anchored()
                        .anchor(Corner::TopLeft)
                        .position(self.font_dropdown_anchor)
                        .child(
                            div()
                                .id("font-dropdown-panel")
                                .flex()
                                .flex_col()
                                .min_w(px(180.))
                                .rounded_md()
                                .border_1()
                                .border_color(c.border)
                                .bg(c.content_bg)
                                .overflow_hidden()
                                .children(FONT_CHOICES.iter().enumerate().map(|(i, f)| {
                                    let selected = font_family_now == *f;
                                    let pick = on_pick_font.clone();
                                    div()
                                        .id(SharedString::from(format!("font-opt-{i}")))
                                        .px_3()
                                        .py_1p5()
                                        .text_sm()
                                        .text_color(
                                            if selected {
                                                c.accent
                                            } else {
                                                c.text_primary
                                            },
                                        )
                                        .hover(|s| s.bg(c.button_hover_bg))
                                        .child(SharedString::from(*f))
                                        .on_click(move |e, w, cx| pick(i, e, w, cx))
                                })),
                        ),
                )
                .with_priority(1);
                root.child(backdrop).child(panel)
            })
    }
}

// ════════════════════════════════════════
// 组件：顶栏
// ════════════════════════════════════════

/// 顶部工具栏：文件名（脏标记）+ 格式快捷按钮 + 保存 + 侧栏开关。
fn top_toolbar(
    name: &SharedString,
    dirty: bool,
    sidebar_open: bool,
    c: &ThemeColors,
    on_save: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_toggle_sidebar: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_format: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
) -> impl IntoElement {
    let title = if dirty {
        format!("{name} *")
    } else {
        name.to_string()
    };

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
        // 左：文件名
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(c.text_muted)
                .child(SharedString::from(title)),
        )
        // 中：格式快捷按钮组
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1()
                .child(format_tool_btn("B", "tb-bold", "bold", c, on_format.clone()))
                .child(format_tool_btn("I", "tb-italic", "italic", c, on_format.clone()))
                .child(format_tool_btn("U", "tb-underline", "underline", c, on_format.clone()))
                .child(div().w(px(4.))) // 分隔
                .child(format_tool_btn("≡", "tb-align", "align", c, on_format.clone())),
        )
        // 右：保存 + 侧栏开关
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .id("toolbar-save")
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .px_3()
                        .py_1()
                        .rounded_md()
                        .bg(if dirty { c.accent } else { c.button_bg })
                        .text_color(
                            if dirty {
                                rgba(0xffffffff)
                            } else {
                                c.text_primary
                            },
                        )
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .child(SharedString::from(t!("editor.save").to_string()))
                        .on_click(on_save),
                )
                .child(
                    div()
                        .id("toolbar-sidebar-toggle")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(28.))
                        .h(px(28.))
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|s| s.bg(c.button_hover_bg))
                        .bg(if sidebar_open {
                            rgba(0x00000011)
                        } else {
                            rgba(0x00000000)
                        })
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(c.text_muted)
                        .child(SharedString::from(
                            if sidebar_open { "◧" } else { "☰" },
                        ))
                        .on_click(on_toggle_sidebar),
                ),
        )
}

/// 单个格式快捷按钮（B / I / U / ≡）。
fn format_tool_btn(
    label: &'static str,
    id: &'static str,
    key: &'static str,
    c: &ThemeColors,
    on_format: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
) -> impl IntoElement {
    let on_f = on_format.clone();
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .w(px(26.))
        .h(px(26.))
        .rounded_md()
        .cursor_pointer()
        .hover(|s| s.bg(c.button_hover_bg))
        .text_sm()
        .font_weight(FontWeight::BOLD)
        .text_color(c.text_muted)
        .child(SharedString::from(label))
        .on_click(move |e, w, cx| on_f(key, e, w, cx))
}

// ════════════════════════════════════════
// 组件：格式侧栏
// ════════════════════════════════════════

/// 右侧格式面板（Pages 风格）。
///
/// 固定宽度 ~280 px；包含「文本」头部、标签页切换、以及当前 tab 的内容。
fn format_sidebar(
    active_tab: FormatTab,
    font_size: u16,
    bold: bool,
    italic: bool,
    underline: bool,
    alignment: TextAlign,
    font_family: &str,
    color_index: usize,
    c: &ThemeColors,
    on_format: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
    on_toggle_font_dropdown: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_tab: impl Fn(FormatTab, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w(px(280.))
        .h_full()
        .border_l_1()
        .border_color(c.border)
        .bg(c.sidebar_bg)
        .overflow_hidden()
        // 头部：「文本」+ 样式下拉
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px(px(14.))
                .py(px(10.))
                .border_b_1()
                .border_color(c.border)
                .child(
                    div()
                        .text_xs()
                        .text_color(c.text_muted)
                        .child(SharedString::from(t!("editor.text").to_string())),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_sm()
                        .text_color(c.text_primary)
                        .child(SharedString::from(
                            t!("editor.body_text").to_string(),
                        ))
                        .child(SharedString::from(" ▾")),
                ),
        )
        // 标签页栏：样式 | 布局 | 更多
        .child(sidebar_tab_bar(active_tab, c, on_tab))
        // 标签页内容（按当前 tab 渲染不同内容）
        .child({
            // 各 tab 内容统一为 AnyElement 以便在 match 里选择
            let style = sidebar_style_content(
                    font_size, bold, italic, underline,
                    alignment, font_family, color_index, c,
                    on_format, on_toggle_font_dropdown,
                ).into_any_element();
            let layout = sidebar_layout_content(c).into_any_element();
            let more = sidebar_more_content(c).into_any_element();

            let content = match active_tab {
                FormatTab::Style => style,
                FormatTab::Layout => layout,
                FormatTab::More => more,
            };

            div()
                .id("sidebar-content")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .px(px(14.))
                .py(px(8.))
                .gap_4()
                .child(content)
        })
}

/// 标签页栏（样式 / 布局 / 更多）。
fn sidebar_tab_bar(
    active: FormatTab,
    c: &ThemeColors,
    on_tab: impl Fn(FormatTab, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
) -> impl IntoElement {
    let tabs = [
        (FormatTab::Style, "editor.style"),
        (FormatTab::Layout, "editor.layout"),
        (FormatTab::More, "editor.more"),
    ];

    div()
        .flex()
        .flex_row()
        .px(px(12.))
        .pt(px(4.))
        .border_b_1()
        .border_color(c.border)
        .children(tabs.into_iter().map(move |(tab, key)| {
            let is_active = active == tab;
            let label = t!(key).to_string();
            let bg = if is_active {
                c.accent
            } else {
                rgba(0x00000000)
            };
            let txt_color = if is_active {
                rgba(0xffffffff)
            } else {
                c.text_muted
            };

            let tab_clone = tab;
            let on_tab_clone = on_tab.clone();
            div()
                .id(key)
                .flex()
                .items_center()
                .justify_center()
                .flex_1()
                .py_1p5()
                .mt(px(4.)) // 让活跃 tab 的底部边框覆盖 container border
                .mb(px(-1.))
                .rounded_t_md()
                .cursor_pointer()
                .bg(bg)
                .text_xs()
                .font_weight(if is_active {
                    FontWeight::MEDIUM
                } else {
                    FontWeight::NORMAL
                })
                .text_color(txt_color)
                .child(SharedString::from(label))
                .on_click(move |evt, win, cx| (on_tab_clone)(tab_clone, evt, win, cx))
        }))
}

/// 「样式」标签页内容：字体、大小、B/I/U、对齐、颜色。
fn sidebar_style_content(
    font_size: u16,
    bold: bool,
    italic: bool,
    underline: bool,
    alignment: TextAlign,
    font_family: &str,
    color_index: usize,
    c: &ThemeColors,
    on_format: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + Clone + 'static,
    on_toggle_font_dropdown: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    // 字体选择器行：点击展开悬浮下拉菜单，选中即应用并收起。
    let font_row = div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .id("editor-font-row")
                .cursor_pointer()
                .on_click(on_toggle_font_dropdown)
                .child(sidebar_label_row("editor.font", c))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .flex_1()
                        .text_sm()
                        .text_color(c.text_primary)
                        .child(SharedString::from(font_family.to_string()))
                        .child(SharedString::from("▾")),
                ),
        );

    // 字体大小行（- / 数字 / +，点击步进字号）
    let size_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .child(sidebar_label_row("editor.size", c))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1()
                .child(
                    div()
                        .id("editor-font-dec")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(24.))
                        .h(px(26.))
                        .rounded_md()
                        .border_1()
                        .border_color(c.border)
                        .cursor_pointer()
                        .text_base()
                        .text_color(c.text_primary)
                        .child(SharedString::from("-"))
                        .on_click({
                            let on_f = on_format.clone();
                            move |e, w, cx| on_f("font-dec", e, w, cx)
                        }),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(48.))
                        .h(px(26.))
                        .rounded_md()
                        .border_1()
                        .border_color(c.border)
                        .text_sm()
                        .text_color(c.text_primary)
                        .child(SharedString::from(format!("{}", font_size))),
                )
                .child(
                    div()
                        .id("editor-font-inc")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(24.))
                        .h(px(26.))
                        .rounded_md()
                        .border_1()
                        .border_color(c.border)
                        .cursor_pointer()
                        .text_base()
                        .text_color(c.text_primary)
                        .child(SharedString::from("+"))
                        .on_click({
                            let on_f = on_format.clone();
                            move |e, w, cx| on_f("font-inc", e, w, cx)
                        }),
                ),
        );

    // B I U S 按钮
    let fmt_btn = |label: &'static str, _key: &'static str, active: bool| -> _ {
        let on_f = on_format.clone();
        div()
            .id(_key)
            .flex()
            .items_center()
            .justify_center()
            .w(px(30.))
            .h(px(28.))
            .rounded_md()
            .cursor_pointer()
            .bg(if active { rgba(0x0a84ff18) } else { rgba(0x00000000) })
            .border_1()
            .border_color(if active { c.accent } else { c.border })
            .text_base()
            .font_weight(if active {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            })
            .text_color(if active { c.accent } else { c.text_primary })
            .child(SharedString::from(label))
            .on_click(move |e, w, cx| on_f(_key, e, w, cx))
    };

    let biu_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1p5()
        .child(fmt_btn("B", "bold", bold))
        .child(fmt_btn("I", "italic", italic))
        .child(fmt_btn("U", "underline", underline));

    // 对齐按钮
    let align_labels = [
        ("align-left", "L", TextAlign::Left),
        ("align-center", "C", TextAlign::Center),
        ("align-right", "R", TextAlign::Right),
        ("align-justify", "J", TextAlign::Justify),
    ];

    let align_on = on_format.clone();
    let align_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1p5()
        .children(align_labels.into_iter().map(move |(_key, label, ta)| {
            let active = alignment == ta;
            let align_f = align_on.clone();
            div()
                .id(_key)
                .flex()
                .items_center()
                .justify_center()
                .w(px(30.))
                .h(px(28.))
                .rounded_md()
                .cursor_pointer()
                .bg(if active { rgba(0x0a84ff18) } else { rgba(0x00000000) })
                .border_1()
                .border_color(if active { c.accent } else { c.border })
                .text_xs()
                .text_color(if active { c.accent } else { c.text_primary })
                .child(SharedString::from(label))
                .on_click(move |e, w, cx| align_f("align", e, w, cx))
        }));

    // 文本颜色行（点击循环切换颜色）
    let color_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .child(sidebar_label_row("editor.text_color", c))
        .child(
            div()
                .id("editor-color")
                .flex()
                .items_center()
                .justify_center()
                .w(px(28.))
                .h(px(28.))
                .rounded_md()
                .border_1()
                .border_color(c.border)
                .cursor_pointer()
                .bg(color_at(color_index, c))
                .on_click({
                    let on_f = on_format.clone();
                    move |e, w, cx| on_f("cycle-color", e, w, cx)
                }),
        );

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(font_row)
        .child(size_row)
        .child(biu_row)
        .child(div().h(px(4.))) // 分隔
        .child(align_row)
        .child(color_row)
}

/// 「布局」标签页内容（占位）。
fn sidebar_layout_content(c: &ThemeColors) -> impl IntoElement {
    div()
        .flex()
        .flex_1()
        .items_center()
        .justify_center()
        .text_sm()
        .text_color(c.text_muted)
        .child(SharedString::from(format!(
            "{} …",
            t!("editor.layout").to_string()
        )))
}

/// 「更多」标签页内容（占位）。
fn sidebar_more_content(c: &ThemeColors) -> impl IntoElement {
    div()
        .flex()
        .flex_1()
        .items_center()
        .justify_center()
        .text_sm()
        .text_color(c.text_muted)
        .child(SharedString::from(format!(
            "{} …",
            t!("editor.more").to_string()
        )))
}

/// 侧栏内的小节标签行（如「字体」「文本颜色」）。
fn sidebar_label_row(key: &'static str, c: &ThemeColors) -> impl IntoElement {
    div()
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(c.text_muted)
        .child(SharedString::from(t!(key).to_string()))
}

// ════════════════════════════════════════
// 组件：状态栏
// ════════════════════════════════════════

/// 底部状态栏：当前行列、总行数、字数、未保存标记。
fn status_bar(
    line: usize,
    col: usize,
    total_lines: usize,
    chars: usize,
    dirty: bool,
    c: &ThemeColors,
) -> impl IntoElement {
    let mark = if dirty { "   \u{25CF}" } else { "" }; // ●
    let text = format!(
        "Ln {}, Col {}    {} {}{}",
        line + 1,
        col + 1,
        total_lines,
        chars,
        mark
    );

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
        .child(SharedString::from(text))
}

// ════════════════════════════════════════
// 组件：行渲染
// ════════════════════════════════════════

/// 渲染一行：行号 + 文本；当前光标行把竖线光标插到对应列并高亮当前行。
///
/// `marked` 为输入法合成态范围（UTF-16，仅对光标行有意义）。有值时把该段加下划线，
/// 并把光标放在合成段末尾（跟随 IME 候选选择）。
fn build_line(
    i: usize,
    line: &str,
    is_caret_line: bool,
    caret_col: usize,
    c: &ThemeColors,
    font_size: u16,
    color_index: usize,
    font_family: &str,
    marked: Option<Range<usize>>,
) -> impl IntoElement {
    let gutter = div()
        .w(px(36.))
        .flex_none()
        .pr_2()
        .text_right()
        .text_xs()
        .text_color(c.text_muted)
        .opacity(0.5)
        .child(SharedString::from(format!("{}", i + 1)));

    let text = if is_caret_line {
        // 把当前行切成片段，并决定光标插入位置。
        let (segments, caret_after): (Vec<(&str, bool)>, usize) = match &marked {
            Some(m) => {
                let ms = utf16_to_char(line, m.start);
                let me = utf16_to_char(line, m.end);
                let (b, rest) = split_at_char(line, ms);
                let (mk, a) = split_at_char(rest, me.saturating_sub(ms));
                (vec![(b, false), (mk, true), (a, false)], 1)
            }
            None => {
                let (b, a) = split_at_char(line, caret_col);
                (vec![(b, false), (a, false)], 0)
            }
        };

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .flex_1()
            .min_h(px(20.))
            .text_size(px(font_size as f32))
            .font_family(font_family.to_string())
            .text_color(color_at(color_index, c));

        for (idx, (seg, underlined)) in segments.iter().enumerate() {
            if idx == caret_after {
                row = row.child(
                    div()
                        .flex_none()
                        .w(px(2.))
                        .h(px(font_size as f32 + 4.))
                        .bg(c.accent),
                );
            }
            row = row.child(
                div()
                    .when(*underlined, |s| s.underline())
                    .child(SharedString::from(if seg.is_empty() {
                        "\u{00A0}".to_string()
                    } else {
                        seg.to_string()
                    })),
            );
        }
        if caret_after >= segments.len() {
            row = row.child(
                div()
                    .flex_none()
                    .w(px(2.))
                    .h(px(font_size as f32 + 4.))
                    .bg(c.accent),
            );
        }
        row
    } else {
        div()
            .flex_1()
            .min_h(px(20.))
            .text_size(px(font_size as f32))
            .font_family(font_family.to_string())
            .text_color(color_at(color_index, c))
            .child(if line.is_empty() {
                SharedString::from("\u{00A0}") // 不换行空格保持行高
            } else {
                SharedString::from(line.to_string())
            })
    };

    let lh = line_height(font_size);
    let mut row = div()
        .flex()
        .flex_row()
        .w_full()
        .h(lh)
        .items_center()
        .child(gutter)
        .child(text);

    if is_caret_line {
        row = row.bg(rgba(0x0a84ff0d)); // 极浅蓝高亮
    }
    row
}

// ════════════════════════════════════════
// 辅助函数
// ════════════════════════════════════════

/// 把模型里的文本块抽成行（段落/标题各一行，标题加 "# " 前缀）。
fn extract_lines(model: &Model) -> Vec<String> {
    if let Model::Text(doc) = model {
        doc.blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(p) => Some(
                    p.runs.iter().map(|r| r.text.clone()).collect::<String>(),
                ),
                Block::Heading(h) => Some(format!(
                    "# {}",
                    h.runs.iter().map(|r| r.text.clone()).collect::<String>()
                )),
                _ => None,
            })
            .collect()
    } else {
        vec![]
    }
}

/// 返回第 `char_idx` 个字符在字符串中的字节下标。
fn char_byte_index(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

/// 在字符边界切分字符串。
fn split_at_char(s: &str, char_idx: usize) -> (&str, &str) {
    let byte = char_byte_index(s, char_idx);
    (&s[..byte], &s[byte..])
}

/// UTF-16 偏移 → 字节偏移（在当前行字符串内）。
fn utf16_to_byte(s: &str, utf16: usize) -> usize {
    let mut u = 0;
    for (byte, c) in s.char_indices() {
        if u >= utf16 {
            return byte;
        }
        u += c.len_utf16();
    }
    s.len()
}

/// UTF-16 偏移 → 字符索引（在当前行字符串内，用于把 IME 坐标映射回 `caret_col`）。
fn utf16_to_char(s: &str, utf16: usize) -> usize {
    let mut u = 0;
    let mut chars = 0;
    for c in s.chars() {
        if u >= utf16 {
            return chars;
        }
        u += c.len_utf16();
        chars += 1;
    }
    chars
}

/// 字符索引 → 当前行中该位置之前的 UTF-16 长度。
fn utf16_len_up_to(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .take(char_idx)
        .map(|(_, c)| c.len_utf16())
        .sum()
}

/// 编辑器每行的统一行高（px）。`render()` 的每一行与 `bounds_for_range` 都用它，
/// 保证 IME 候选窗的 y 坐标与真实渲染逐行对齐（之前用 `ascent+descent` 估算会与
/// GPUI 默认行高不符，行号越大偏移越大 → 候选窗"不跟手"）。
fn line_height(font_size: u16) -> Pixels {
    px((font_size as f32 * 1.6).max(20.0))
}

// ════════════════════════════════════════
// InputRegion — 把画布包成可承载输入处理器的元素
// ════════════════════════════════════════
//
// GPUI 0.2.2 没有「把 TextInput 挂到现成 div 上」的 API；必须用一个自定义 `Element`
// 在 `paint` 阶段调用 `window.handle_input(...)` 来注册 `EntityInputHandler`。
// 这里把画布 `child` 原样转发，只在 paint 时顺手注册输入处理器。

struct InputRegion {
    child: AnyElement,
    view: Entity<EditorView>,
    focus: FocusHandle,
}

impl Element for InputRegion {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let layout_id = self.child.request_layout(window, cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut (),
        _prepaint: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        // 先正常绘制画布，再注册输入处理器（handle_input 只能在 paint 阶段调用，
        // 且只在 focus 处于本视图时才会真正生效）。
        self.child.paint(window, cx);
        window.handle_input(
            &self.focus,
            ElementInputHandler::new(bounds, self.view.clone()),
            cx,
        );
    }
}

impl IntoElement for InputRegion {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
