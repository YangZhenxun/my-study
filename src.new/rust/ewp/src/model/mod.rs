//! EWP 原生文档模型
//!
//! 设计原则（见 docs / 架构讨论）：
//! 1. serde 贯穿 —— 原生格式与 OOXML 过滤器共享同一套 struct。
//! 2. 格式无关 —— 编辑/渲染核心只认本模型，永不直接碰 OOXML。
//! 3. 可替换序列化 —— v1 用 JSON，同一 serde 层后续可换 CBOR / MessagePack。
//!
//! OOXML（docx/xlsx/pptx）通过 `filter` 模块的 `Importer` / `Exporter`
//! trait 在边界接入，把外部格式映射进 `Model` 枚举。
//!
//! 各文档类型的模型已按文档类型分目录（`text::model` / `sheet::model` /
//! `slide::model`），此处仅保留共享胶水（common / filter / ser）并从子目录
//! re-export，保持 `crate::model::{Document, Workbook, Presentation, ...}` 旧路径
//! 仍可用（向后兼容）。

pub mod common;
pub mod filter;
pub mod ser;

use serde::{Deserialize, Serialize};

// 从按文档类型分目录的新位置 re-export（见 docs/system_design.md §2、§7）。
pub use crate::sheet::model::Workbook;
pub use crate::slide::model::Presentation;
pub use crate::text::model::Document;

/// 一个 EWP 文档的三种形态。`.ewp` 文件落盘的就是这个枚举的序列化结果。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Text(Document),
    Sheet(Workbook),
    Slide(Presentation),
}
