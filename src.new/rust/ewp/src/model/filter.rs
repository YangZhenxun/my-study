//! 过滤器边界（OOXML 接入点）。
//!
//! OOXML（docx/xlsx/pptx）通过实现这两个 trait 接入 EWP，
//! 把外部格式映射进 `Model`，核心代码完全不引用 `ooxmlsdk`。
//! 这是「原生模型 + 过滤器边界」架构的接缝。

#![allow(dead_code)]

use std::path::Path;

use crate::model::Model;

/// 导入器：外部格式 → EWP 原生模型。
pub trait Importer {
    /// 把 `path` 指向的外部文件映射为 `Model`。
    fn import(&self, path: &Path) -> Result<Model, String>;
}

/// 导出器：EWP 原生模型 → 外部格式。
pub trait Exporter {
    /// 把 `model` 映射并写入 `path`。
    fn export(&self, model: &Model, path: &Path) -> Result<(), String>;
}
