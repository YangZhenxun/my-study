//! 编辑器视图
//!
//! 由「新建项目」或「打开 .ewp」打开。当前是一个**最小但真实可用**的文本编辑器：
//! - 内存中的文档，不在磁盘上创建任何文件（保存弹窗以后再做）。
//! - 自建文本输入（GPUI 0.2.2 没有内置 TextInput 控件）：靠 `FocusHandle` + `on_key_down`
//!   捕获按键，自己维护多行缓冲与光标。
//! - 打开 .ewp 时，把 `Document` 的块抽成行显示出来，证明“打开即见”。

use gpui::{
    App, Context, FocusHandle, Focusable, FontWeight, KeyDownEvent, MouseButton, Render,
    SharedString, Window, div, px, rgb,
};
use gpui::prelude::*;

use crate::model::text::{Block, Document};
use crate::model::Model;

/// 编辑器根视图。
pub struct EditorView {
    focus: FocusHandle,
    /// 窗口标题栏显示的名字（如 "Untitled" / 文件名）。
    name: SharedString,
    /// 多行文本缓冲，最后一行即正在编辑的行。
    lines: Vec<String>,
    /// 当前光标所在行。
    caret_line: usize,
    /// 当前光标在该行的列。
    caret_col: usize,
    /// 背后的原生模型（保存时再映射回去，本期未做）。
    #[allow(dead_code)]
    model: Model,
}

impl EditorView {
    /// 新建一个空白编辑器。
    pub fn new_blank(cx: &mut Context<Self>, name: SharedString) -> Self {
        Self::build(cx, name, Model::Text(Document::default()))
    }

    /// 从已加载的模型打开编辑器（用于“打开 .ewp”）。
    pub fn new_from_model(cx: &mut Context<Self>, name: SharedString, model: Model) -> Self {
        Self::build(cx, name, model)
    }

    fn build(cx: &mut Context<Self>, name: SharedString, model: Model) -> Self {
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
        }
    }

    /// 处理一次按键：可打印字符插入，Enter/Backspace/方向键移动光标。
    fn handle_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let ks = &event.keystroke;

        // 组合键（⌘ / Ctrl / Alt）留给快捷键，不当作输入。
        if ks.modifiers.platform || ks.modifiers.control || ks.modifiers.alt {
            return;
        }

        // 可打印字符优先用 key_char（含 Shift / 输入法结果）。
        if let Some(ch) = &ks.key_char {
            if ch.chars().count() == 1 {
                let c = ch.chars().next().unwrap();
                let line = &mut self.lines[self.caret_line];
                if self.caret_col >= line.chars().count() {
                    line.push(c);
                    self.caret_col += c.len_utf8();
                } else {
                    let byte = char_byte_index(line, self.caret_col);
                    line.insert(byte, c);
                    self.caret_col += c.len_utf8();
                }
                cx.notify();
                return;
            }
        }

        // 不可打印键靠 key 字符串判断。
        match ks.key.as_str() {
            "enter" => {
                let cur = self.lines[self.caret_line].clone();
                let byte = char_byte_index(&cur, self.caret_col);
                let (left, right) = cur.split_at(byte);
                self.lines[self.caret_line] = left.to_string();
                self.lines.insert(self.caret_line + 1, right.to_string());
                self.caret_line += 1;
                self.caret_col = 0;
            }
            "backspace" => {
                let col = self.caret_col;
                if col > 0 {
                    let line = &mut self.lines[self.caret_line];
                    let byte = char_byte_index(line, col - 1);
                    line.remove(byte);
                    self.caret_col -= 1;
                } else if self.caret_line > 0 {
                    let cur = self.lines.remove(self.caret_line);
                    self.caret_line -= 1;
                    self.caret_col = self.lines[self.caret_line].chars().count();
                    self.lines[self.caret_line].push_str(&cur);
                }
            }
            "left" => {
                if self.caret_col > 0 {
                    self.caret_col -= 1;
                }
            }
            "right" => {
                let len = self.lines[self.caret_line].chars().count();
                if self.caret_col < len {
                    self.caret_col += 1;
                }
            }
            "up" => {
                if self.caret_line > 0 {
                    self.caret_line -= 1;
                    self.clamp_caret();
                }
            }
            "down" => {
                if self.caret_line + 1 < self.lines.len() {
                    self.caret_line += 1;
                    self.clamp_caret();
                }
            }
            _ => return,
        }
        cx.notify();
    }

    /// 把光标列限制在当前行长度内。
    fn clamp_caret(&mut self) {
        let len = self.lines[self.caret_line].chars().count();
        if self.caret_col > len {
            self.caret_col = len;
        }
    }
}

impl Focusable for EditorView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for EditorView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();
        let focused = self.focus.is_focused(window);
        let name = self.name.clone();
        let caret_line = self.caret_line;
        let caret_col = self.caret_col;

        // 每行都渲染成统一类型的 div；光标行把 "|" 插到对应列。
        let display_lines: Vec<SharedString> = self
            .lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                if focused && i == caret_line {
                    insert_caret(line, caret_col)
                } else {
                    SharedString::from(line.clone())
                }
            })
            .collect();

        div()
            .id("editor-root")
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x1d1d1f))
            .on_mouse_down(MouseButton::Left, {
                let focus = self.focus.clone();
                move |_, window, _cx| window.focus(&focus)
            })
            .on_key_down({
                let this = this.clone();
                move |event: &KeyDownEvent, _window, cx| {
                    let _ = this.update(cx, |this, cx| this.handle_key(event, cx));
                }
            })
            // 标题栏
            .child(
                div()
                    .px(px(24.))
                    .py(px(14.))
                    .bg(rgb(0xf5f5f7))
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .child(name.clone()),
                    ),
            )
            // 文本区
            .child(
                div()
                    .flex_1()
                    .px(px(24.))
                    .py(px(12.))
                    .text_base()
                    .children(display_lines.iter().map(|line| {
                        div().child(if line.is_empty() {
                            SharedString::from(" ")
                        } else {
                            line.clone()
                        })
                    })),
            )
    }
}

/// 在光标列后插入一个 "|" 当作光标。
fn insert_caret(line: &str, caret_col: usize) -> SharedString {
    let (before, after) = split_at_char(line, caret_col);
    SharedString::from(format!("{before}|{after}"))
}

/// 把模型里的文本块抽成行（段落/标题各一行，标题加 "# " 前缀）。
fn extract_lines(model: &Model) -> Vec<String> {
    if let Model::Text(doc) = model {
        doc.blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(p) => {
                    Some(p.runs.iter().map(|r| r.text.clone()).collect::<String>())
                }
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
