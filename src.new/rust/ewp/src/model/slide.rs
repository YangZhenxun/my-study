//! 演示文稿模型（类 Impress / PowerPoint）。

use serde::{Deserialize, Serialize};

use crate::model::common::{Rgb, TextStyle};
use crate::model::text::Run;

/// 形状几何（单位：逻辑像素）。
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct Rect {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub w: f32,
    #[serde(default)]
    pub h: f32,
}

/// 形状内容。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ShapeKind {
    /// 文本框：一段带样式的文本。
    Text(Vec<Run>),
    /// 图片：资源路径。
    Image(String),
    /// 矢量图形：图形类型标识（矩形 / 椭圆 / 线条…）。
    Vector(String),
}

/// 幻灯片上的一个形状。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Shape {
    #[serde(default)]
    pub geom: Rect,
    pub kind: ShapeKind,
    #[serde(default)]
    pub style: TextStyle,
}

/// 一张幻灯片。
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Slide {
    #[serde(default)]
    pub shapes: Vec<Shape>,
    #[serde(default)]
    pub background: Option<Rgb>,
}

/// 一份演示文稿。
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Presentation {
    #[serde(default)]
    pub id: crate::model::common::Id,
    #[serde(default)]
    pub slides: Vec<Slide>,
}
