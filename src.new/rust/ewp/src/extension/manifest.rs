//! 扩展清单（extension.toml）解析
//!
//! 格式参考 Zed 的 `extension.toml`（schema_version = 1）。
//! 每个扩展目录下必须有一个 `extension.toml`。
//!
//! 支持的扩展类型：
//! - `themes`：纯数据（JSON 主题文件），不走 WASM
//! - `documents`：WASM 扩展，负责文档解析/序列化/布局/交互
//!   需要提供 `lib` 指向编译好的 `.wasm` 文件

use serde::Deserialize;

/// 扩展清单，从 `extension.toml` 反序列化。
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ExtensionManifest {
    /// 扩展唯一 ID（如 "my-theme" / "word-renderer"）。
    pub id: String,
    /// 人类可读名称。
    pub name: String,
    /// 语义版本号。
    pub version: String,
    /// 清单 schema 版本（当前 = 1）。
    pub schema_version: u32,
    /// 扩展描述（可选）。
    #[serde(default)]
    pub description: Option<String>,
    /// 作者列表（可选）。
    #[serde(default)]
    pub authors: Vec<String>,
    /// WASM 库路径（代码扩展才需要，主题扩展不需要）。
    /// 如 "target/wasm32-wasip2/release/word_ext.wasm"
    #[serde(default)]
    pub lib: Option<String>,
    /// 主题文件列表（相对路径）。纯数据扩展。
    #[serde(default)]
    pub themes: Vec<ThemeEntry>,
    /// 文档扩展列表。每个条目声明一种文档类型（如 text/sheet/slide）。
    /// 需要配合 `lib` 字段提供 WASM 模块。
    #[serde(default)]
    pub documents: Vec<DocumentEntry>,
}

/// 清单中的主题条目。
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ThemeEntry {
    /// 主题 JSON 文件相对路径（如 "themes/dark.json"）。
    pub path: String,
}

/// 清单中的文档扩展条目。
///
/// 声明此扩展能处理一种文档类型。
/// 对应的 Wit 接口见 `wit/extension/world.wit`。
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DocumentEntry {
    /// 文档类型标识（如 "text" / "sheet" / "slide"）。
    pub kind: String,
    /// 支持的文件扩展名列表（如 ["docx", "doc"]）。
    #[serde(default)]
    pub extensions: Vec<String>,
    /// 默认文件后缀（新建文档时使用，如 "docx"）。
    #[serde(default)]
    pub default_extension: Option<String>,
}
