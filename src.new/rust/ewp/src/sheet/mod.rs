//! 表格文档类型目录（类 LibreOffice Calc）。
//!
//! 把该文档类型的「主视图」「模型」与全部 Sheet 内部模块在此 re-export。
//! 注意：Sheet 内部（grid / grid_cache / view_state）是 LibreOffice Calc 视图
//! 架构移植，保持单 canvas + 同源坐标公式，本目录只做模块聚合，不引入新状态。

pub mod grid;
pub mod grid_cache;
pub mod model;
pub mod scrollbar;
pub mod status_bar;
pub mod view;
pub mod view_state;

