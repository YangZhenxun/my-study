//! 底部状态栏组件（自绘，≈ LibreOffice `StatusBar` + `ScTabView::CreateStatusArea`/`UpdateStatusBar`）。
//!
//! LibreOffice 的状态栏由多个 `StatusBarItem` 组成（地址、单元格内容、Sheet 名/计数、
//! 缩放、选区统计 `CalcWnd` 的 Sum/Average/Count、插入模式、语言等）。EWP 用纯 `div`+`text`
//! 自绘同样的字段条。**本组件是纯展示、只读派生**：数据全部由 `SheetView::derive_status_bar`
//! 一次性计算后传入 `StatusBarModel`，组件本身不持有状态、不触发任何 `cx.notify`（红线：只经
//! `SheetViewState` 驱动 canvas；状态栏零副作用）。
//!
//! 字段清单（含 WebFetch `tabview.cxx` `CreateStatusArea`/`UpdateStatusBar` 核对后采用的子集）：
//! 地址 / 单元格内容预览 / 当前 Sheet 名 + 总 Sheet 数 / 缩放百分比 / 选区统计
//! (Sum·Avg·Count) / 选区描述 / 插入模式(INS 占位) / 语言(中文(中国) 占位)。

use gpui::{Div, SharedString, div, px};
use gpui::prelude::*;

use crate::styles::ThemeColors;

/// 状态栏只读派生数据（全部由 `SheetView::derive_status_bar` 计算后传入）。
/// 组件本身不写任何状态。
pub struct StatusBarModel {
    /// 当前选中格地址（A1）。
    pub cell_addr: String,
    /// 单元格内容预览（≈ InfoWnd）。
    pub cell_preview: String,
    /// 当前 sheet 名。
    pub sheet_name: String,
    /// 总 sheet 数（≈ "Sheet1 / 3"）。
    pub sheet_count: usize,
    /// 缩放百分比（state.zoom * 100）。
    pub zoom_pct: u32,
    /// 选区 Sum（≈ CalcWnd）。单格数值时即该值；否则 `None`。
    pub sum: Option<f64>,
    /// 选区 Average。单格数值时即该值；否则 `None`。
    pub avg: Option<f64>,
    /// 选区 Count（含非数值格计数；空选 0）。
    pub count: usize,
    /// 选区描述（如 "A1" 或 "A1:B3"）。
    pub selection_label: String,
    /// 插入模式（"INS" / "OVR"）。
    pub insert_mode: &'static str,
    /// 语言（占位 "中文(中国)"）。
    pub language: &'static str,
}

// mirrors LibreOffice: sc/source/ui/view/tabview.cxx — ScTabView::CreateStatusArea / UpdateStatusBar / FillStatusBar
//   C++（设计还原 + WebFetch 字段核查）：状态栏拼接多个 StatusBarItem 文本段：
//     "A1"  |  <单元格内容>  |  "Sheet1 / 3"  |  "100%"  |  "Sum: …  Avg: …  Count: …"  |  "INS"  |  "中文(中国)"
//   Rust 逐行对应（每个段 = 一个 div().child(text(...))，段间用分隔条）：
//     for field in [cell_addr, cell_preview, sheet, zoom, stats, insert_mode, language] { div().child(field) }
/// 渲染状态栏：纯展示，按 LibreOffice 字段顺序拼字段条。无状态写入、无 notify。
pub fn render_status_bar(model: &StatusBarModel) -> impl IntoElement {
    let c = ThemeColors::current();
    let sheet_text = format!("{} / {}", model.sheet_name, model.sheet_count);
    let zoom_text = format!("{}%", model.zoom_pct);
    let stats_text = match (model.sum, model.avg) {
        (Some(s), Some(a)) => format!(
            "求和 {}  平均 {:.2}  计数 {}",
            format_num(s),
            a,
            model.count
        ),
        _ => format!("计数 {}", model.count),
    };
    let insert_text = model.insert_mode;
    let lang_text = model.language;

    let seg = |label: &str| -> Div {
        div()
            .px_2()
            .text_xs()
            .text_color(c.text_muted)
            .child(SharedString::from(label.to_string()))
    };
    let spacer = || -> Div {
        div()
            .px_1()
            .text_xs()
            .text_color(c.border)
            .child(SharedString::from("│"))
    };

    div()
        .flex()
        .flex_row()
        .items_center()
        .px(px(8.))
        .py(px(5.))
        .bg(c.sidebar_bg)
        .border_t_1()
        .border_color(c.border)
        .child(seg(&model.cell_addr))
        .child(spacer())
        .child(seg(model.cell_preview.trim()))
        .child(spacer())
        .child(seg(&sheet_text))
        .child(spacer())
        .child(seg(&zoom_text))
        .child(spacer())
        .child(seg(&stats_text))
        .child(spacer())
        .child(seg(insert_text))
        .child(spacer())
        .child(seg(lang_text))
        .when(model.selection_label != model.cell_addr, |bar| {
            // 若未来支持多格选区，额外展示选区范围（本期单格，通常不显示）。
            bar.child(spacer()).child(seg(&model.selection_label))
        })
}

/// 数字格式化（整数不显示小数，浮点保留合理精度）。
fn format_num(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        format!("{:.4}", v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 状态栏数字格式化（≈ LibreOffice CalcWnd Sum/Avg 展示）：
    // 整数无小数；浮点保留 4 位；超大整数走 i64 不丢精度。
    #[test]
    fn format_num_integer_no_decimal() {
        assert_eq!(format_num(100.0), "100");
        assert_eq!(format_num(0.0), "0");
        assert_eq!(format_num(-42.0), "-42");
    }

    #[test]
    fn format_num_float_keeps_precision() {
        assert_eq!(format_num(2.5), "2.5000");
        assert_eq!(format_num(2.5), "2.5000");
        assert_eq!(format_num(-0.001), "-0.0010");
    }

    #[test]
    fn format_num_large_integer_no_precision_loss() {
        // 1e15 以下整数走 i64 精确格式，不出现科学计数/精度丢失。
        assert_eq!(format_num(1234567890123.0), "1234567890123");
    }
}
