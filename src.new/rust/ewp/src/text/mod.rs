//! 文本文档类型目录（类 LibreOffice Writer）。
//!
//! 把该文档类型的「主视图」与「模型」在此 re-export，方便根模块与
//! 其它文档类型以 `crate::text::EditorView` / `crate::text::model::Document`
//! 的方式访问（见 `docs/system_design.md` §2、§7）。

pub mod editor_view;
pub mod model;

