//! 扩展系统
//!
//! 仿 Zed 的扩展架构：
//! - 主题扩展 = 纯数据文件（JSON），不走 WASM
//! - extension.toml 清单描述扩展元数据
//! - 内置主题编译进二进制（include_str!），用户扩展放 data/extensions/
//! - WASM 接口预留（wit/ 目录），后续接入 wasmtime

pub mod host;
pub mod manifest;
pub mod theme_file;

#[allow(unused_imports)]
pub use host::{ExtensionHost, LoadedTheme};
#[allow(unused_imports)]
pub use manifest::ExtensionManifest;
#[allow(unused_imports)]
pub use theme_file::{ThemeFile, ThemeColorsData};
