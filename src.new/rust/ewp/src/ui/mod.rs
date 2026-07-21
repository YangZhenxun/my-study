//! 多套 UI 界面框架（类 LibreOffice 的 UI 框架抽象）。
//!
//! 本模块实现「可扩展的多 UI 模式」：一个模式注册表（standard / tabbed，可扩展）、
//! 一个 `UiLayout` trait（渲染顶部 chrome）、一个 `UiLayoutManager`（按当前模式 +
//! 持久化偏好切换、组合布局）。设计映射见 `docs/system_design.md` §1.2。

pub mod layout;
pub mod manager;
pub mod persistence;
pub mod standard;
pub mod tabbed;
