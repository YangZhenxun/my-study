//! 文字处理模型（类 Writer / Word）。
//!
//! `Document` 是一棵块的序列：段落、标题、表格、图片、列表。
//! 段落由 `Run`（带样式的连续文本片段）组成，便于保留局部格式。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::common::{Id, TextStyle};

/// 带样式的连续文本片段。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Run {
    pub text: String,
    #[serde(default)]
    pub style: TextStyle,
    /// 超链接目标（如有）。
    #[serde(default)]
    pub link: Option<String>,
}

/// 普通段落。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Paragraph {
    pub runs: Vec<Run>,
    #[serde(default)]
    pub style: TextStyle,
}

/// 标题（按层级区分，便于生成大纲）。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Heading {
    pub level: u8,
    pub runs: Vec<Run>,
}

/// 表格单元格。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TableCell {
    pub runs: Vec<Run>,
}

/// 表格。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Table {
    pub rows: Vec<Vec<TableCell>>,
}

/// 行内图片。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageBlock {
    pub src: String,
    #[serde(default)]
    pub alt: String,
}

/// 列表项。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListItem {
    pub runs: Vec<Run>,
}

/// 有序 / 无序列表。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct List {
    pub ordered: bool,
    pub items: Vec<ListItem>,
}

/// 文档顶层块。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Block {
    Paragraph(Paragraph),
    Heading(Heading),
    Table(Table),
    Image(ImageBlock),
    List(List),
}

/// 一篇文档。
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Document {
    #[serde(default)]
    pub id: Id,
    #[serde(default)]
    pub blocks: Vec<Block>,
    /// 命名样式表（段落/字符样式引用）。
    #[serde(default)]
    pub styles: HashMap<Id, TextStyle>,
}
