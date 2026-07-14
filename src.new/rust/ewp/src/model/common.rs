//! 公共原语：颜色、长度、文本样式、ID。
//! 所有文档类型（text/sheet/slide）共享这些基础类型。

use serde::{Deserialize, Serialize};

/// 文档内实体标识符（段落样式、主题等）。
pub type Id = String;

/// 24-bit RGB 颜色。
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// 长度单位。排版引擎按上下文解析。
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Length {
    /// 磅（point），1pt = 1/72 英寸。
    Pt(f32),
    /// 像素。
    Px(f32),
    /// 相对字号（em）。
    Em(f32),
}

/// 一段文本的通用样式。所有字段默认缺失，由渲染层继承上下文。
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TextStyle {
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub size: Option<Length>,
    #[serde(default)]
    pub color: Option<Rgb>,
    #[serde(default)]
    pub font: Option<String>,
}
