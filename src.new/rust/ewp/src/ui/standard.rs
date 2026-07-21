//! 标准工具栏布局：复刻原 `editor_view::top_toolbar` 的框（标题 | tool_group | 保存 + 侧栏）。
//!
//! mirrors LibreOffice: `sfx2::SfxNotebookBar` 的「默认 toolbar 模式」——
//! 一行式工具栏，左标题、中格式按钮组、右常用动作（保存 / 侧栏开关）。
//!
//! 本布局**只画统一框**，视图把现有按钮组塞进 `ChromeCtx::tool_group`，从而行为不变。

use gpui::{AnyElement, App, ClickEvent, FontWeight, SharedString, Window, div, px, rgba};
use gpui::prelude::*;
use rust_i18n::t;
use std::rc::Rc;

use crate::styles::ThemeColors;
use crate::ui::layout::{ChromeCtx, UiLayout};

/// 标准工具栏布局（= 当前默认行为）。
///
/// mirrors LibreOffice: 默认 Writer/Calc/Impress 工具栏（SfxNotebookBar 的标准模式）。
pub struct StandardToolbar;

impl UiLayout for StandardToolbar {
    fn id(&self) -> &'static str {
        "standard"
    }
    fn label(&self) -> String {
        t!("ui.mode_standard").to_string()
    }

    fn render_top(
        &self,
        ctx: ChromeCtx,
        body: AnyElement,
        _window: &mut Window,
        _cx: &mut App,
    ) -> AnyElement {
        let c = ThemeColors::current();
        // 解构 `ctx`：按值取出 tool_group 与各回调（无需 Clone）。
        let ChromeCtx {
            model_kind: _,
            name,
            dirty,
            sidebar_open,
            tool_group,
            on_save,
            on_toggle_sidebar,
            on_format: _,
            on_switch_model: _,
        } = ctx;

        let title = if dirty {
            format!("{name} *")
        } else {
            name.to_string()
        };

        div()
            .flex_col()
            .size_full()
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
                    // 左：文件名（脏标记）
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(c.text_muted)
                            .child(SharedString::from(title)),
                    )
                    // 中：视图提供的格式/动作按钮组（B/I/U/≡ 或 ＋工作表 或 ＋幻灯片/＋文本框）
                    .child(tool_group)
                    // 右：保存 + 侧栏开关
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(save_button(&c, dirty, on_save))
                            .child(sidebar_button(&c, sidebar_open, on_toggle_sidebar)),
                    ),
            )
            // 下方：文档内容（body）
            .child(body)
            .into_any_element()
    }
}

/// 保存按钮（脏时高亮），点击触发 `on_save`。
fn save_button(
    c: &ThemeColors,
    dirty: bool,
    on_save: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
) -> impl IntoElement {
    div()
        .id("toolbar-save")
        .flex()
        .items_center()
        .gap_1p5()
        .px_3()
        .py_1()
        .rounded_md()
        .bg(if dirty { c.accent } else { c.button_bg })
        .text_color(if dirty { rgba(0xffffffff) } else { c.text_primary })
        .cursor_pointer()
        .hover(|s| s.opacity(0.85))
        .child(SharedString::from(t!("editor.save").to_string()))
        .on_click(move |e, w, cx| on_save(e, w, cx))
}

/// 侧栏开关按钮：展开显示 ◧，收起显示 ☰；点击触发 `on_toggle_sidebar`。
fn sidebar_button(
    c: &ThemeColors,
    open: bool,
    on_toggle: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
) -> impl IntoElement {
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
        .bg(if open {
            rgba(0x00000011)
        } else {
            rgba(0x00000000)
        })
        .text_sm()
        .font_weight(FontWeight::BOLD)
        .text_color(c.text_muted)
        .child(SharedString::from(if open { "◧" } else { "☰" }))
        .on_click(move |e, w, cx| on_toggle(e, w, cx))
}
