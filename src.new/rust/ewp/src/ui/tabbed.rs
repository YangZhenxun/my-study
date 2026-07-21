//! 标签页式布局：v1 紧凑双行栏（文档类型 tab + 固定常用动作行）。
//!
//! mirrors LibreOffice: `sfx2::NotebookbarTabControl`
//!（`sfx2/source/notebookbar/NotebookbarTabControl.cxx`：NotebookBar 内的标签页控件，
//! 点 tab 切到不同 notebookbar / 应用模块）。
//!
//! 上行：Text / Sheet / Slide 三个 tab（点 → `ctx.on_switch_model(kind)`，
//!   打开对应类型新窗口，决策⑤）；右侧显示当前文档名。
//! 下行：保存 / 粗体 / 斜体 / 下划线 / 侧栏（固定常用动作行；
//!   侧栏对 sheet/slide 为 no-op，决策⑥）。

use gpui::{AnyElement, App, ClickEvent, FontWeight, SharedString, Window, div, px, rgba};
use gpui::prelude::*;
use rust_i18n::t;
use std::rc::Rc;

use crate::styles::ThemeColors;
use crate::ui::layout::{ChromeCtx, ModelKind, UiLayout};

/// 标签页式布局（v1 紧凑双行栏）。
///
/// mirrors LibreOffice: `sfx2::NotebookbarTabControl`（NotebookBar 内的文档类型标签页）。
pub struct TabbedLayout;

impl UiLayout for TabbedLayout {
    fn id(&self) -> &'static str {
        "tabbed"
    }
    fn label(&self) -> String {
        t!("ui.mode_tabbed").to_string()
    }

    fn render_top(
        &self,
        ctx: ChromeCtx,
        body: AnyElement,
        _window: &mut Window,
        _cx: &mut App,
    ) -> AnyElement {
        let c = ThemeColors::current();
        // 解构 ctx：tabbed 模式用文档类型 tab + 固定常用动作行，不用 tool_group / sidebar_open。
        // mirrors LibreOffice: NotebookbarTabControl 自带 tab，底部固定动作区。
        let ChromeCtx {
            model_kind,
            name,
            dirty,
            sidebar_open: _,
            tool_group: _,
            on_save,
            on_toggle_sidebar,
            on_format,
            on_switch_model,
        } = ctx;

        // 上行：文档类型 tab + 当前名。
        // mirrors LibreOffice: NotebookbarTabControl 的 tab 行。
        let tab_row = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px(px(10.))
            .h(px(34.))
            .bg(c.sidebar_bg)
            .border_b_1()
            .border_color(c.border)
            .child(model_tabs(model_kind, &c, on_switch_model))
            .child(
                div()
                    .text_xs()
                    .text_color(c.text_muted)
                    .child(SharedString::from(name.to_string())),
            );

        // 下行：固定常用动作行（保存 / B / I / U / 侧栏）。
        // mirrors LibreOffice: NotebookBar 底部固定 toolbar 区（保存 / 格式 / 侧栏）。
        let action_row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .px(px(10.))
            .h(px(34.))
            .bg(c.sidebar_bg)
            .border_b_1()
            .border_color(c.border)
            .child(action_btn(
                "tb-save",
                t!("editor.save").to_string(),
                &c,
                dirty,
                on_save,
            ))
            .child(action_btn(
                "tb-bold",
                "B".to_string(),
                &c,
                false,
                Rc::new({
                    let f = on_format.clone();
                    move |e, w, cx| f("bold", e, w, cx)
                }),
            ))
            .child(action_btn(
                "tb-italic",
                "I".to_string(),
                &c,
                false,
                Rc::new({
                    let f = on_format.clone();
                    move |e, w, cx| f("italic", e, w, cx)
                }),
            ))
            .child(action_btn(
                "tb-underline",
                "U".to_string(),
                &c,
                false,
                Rc::new({
                    let f = on_format.clone();
                    move |e, w, cx| f("underline", e, w, cx)
                }),
            ))
            .child(action_btn(
                "tb-sidebar",
                "☰".to_string(),
                &c,
                false,
                on_toggle_sidebar,
            ));

        div()
            .flex_col()
            .size_full()
            .child(tab_row)
            .child(action_row)
            .child(body)
            .into_any_element()
    }
}

/// 文档类型 tab 行：Text / Sheet / Slide，当前类型高亮，点 → 打开对应类型新窗口。
///
/// mirrors LibreOffice: `NotebookbarTabControl` 内的各 module tab（Writer/Calc/Impress）。
fn model_tabs(
    active: ModelKind,
    c: &ThemeColors,
    on_switch_model: Rc<dyn Fn(ModelKind, &mut Window, &mut App) + 'static>,
) -> impl IntoElement {
    let tabs = [
        (ModelKind::Text, t!("ui.doc_text").to_string()),
        (ModelKind::Sheet, t!("ui.doc_sheet").to_string()),
        (ModelKind::Slide, t!("ui.doc_slide").to_string()),
    ];
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1()
        .children(tabs.into_iter().map(move |(kind, label)| {
            let is_active = active == kind;
            let bg = if is_active { c.accent } else { rgba(0x00000000) };
            let txt = if is_active {
                rgba(0xffffffff)
            } else {
                c.text_primary
            };
            let sw = on_switch_model.clone();
            div()
                .id(match kind {
                    ModelKind::Text => "tab-text",
                    ModelKind::Sheet => "tab-sheet",
                    ModelKind::Slide => "tab-slide",
                })
                .px_3()
                .py_1()
                .rounded_md()
                .cursor_pointer()
                .bg(bg)
                .text_sm()
                .font_weight(if is_active {
                    FontWeight::MEDIUM
                } else {
                    FontWeight::NORMAL
                })
                .text_color(txt)
                .child(SharedString::from(label))
                .on_click(move |_, w, cx| sw(kind, w, cx))
        }))
}

/// 固定动作行里的单个按钮（保存 / B / I / U / 侧栏）。
fn action_btn(
    id: &'static str,
    label: String,
    c: &ThemeColors,
    active: bool,
    on_click: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
) -> impl IntoElement {
    let bg = if active { c.accent } else { rgba(0x00000000) };
    let fg = if active {
        rgba(0xffffffff)
    } else {
        c.text_primary
    };
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .h(px(26.))
        .px(px(8.))
        .rounded_md()
        .cursor_pointer()
        .hover(|s| s.bg(c.button_hover_bg))
        .bg(bg)
        .text_sm()
        .font_weight(FontWeight::BOLD)
        .text_color(fg)
        .child(SharedString::from(label))
        .on_click(move |e, w, cx| on_click(e, w, cx))
}
