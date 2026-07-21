//! UI 框架抽象：文档类型维度、Chrome 上下文、可渲染的 UI 单元 trait。
//!
//! mirrors LibreOffice:
//!   - `UiLayout`  ← `framework::UIElement`（`include/framework/uielement.hxx`，
//!      用 name/type/visibility 描述一个可渲染 UI 元素；此处用 trait 方法直接产出元素树）
//!   - `ChromeCtx` ← `framework::UIElement` 的上下文 + `sfx2::SfxNotebookBar` 经
//!      `Bindings` 绑定的动作（保存 / 侧栏 / 格式 / 文档类型切换）
//!   - `ModelKind` ← `vcl::EnumContext::Application`（Writer/Calc/Impress）

use gpui::{AnyElement, App, ClickEvent, SharedString, Window};
use std::rc::Rc;

use crate::model::Model;

/// 文档类型维度（驱动 tab 内容、per-model 默认、坐标无关）。
///
/// mirrors LibreOffice: `vcl::EnumContext::Application`（Writer / Calc / Impress / Draw）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ModelKind {
    Text,
    Sheet,
    Slide,
}

impl ModelKind {
    /// 从模型实例推导文档类型。
    ///
    /// mirrors LibreOffice: `EnumContext::Application` 由当前 module 决定。
    #[allow(dead_code)] // v1 保留：启用 per_model_default 时按文档类型选默认 UI 用。
    pub fn from_model(m: &Model) -> Self {
        match m {
            Model::Text(_) => ModelKind::Text,
            Model::Sheet(_) => ModelKind::Sheet,
            Model::Slide(_) => ModelKind::Slide,
        }
    }
}

/// 视图 → 布局的上下文：视图把自己能提供的标题、脏标记、按钮组与动作回调交给布局，
/// 布局只负责「套最外层 chrome」（顶部栏 + 把 body 放下方），不包含任何文档内部状态。
///
/// mirrors LibreOffice: `framework::UIElement`（m_aName/m_aType/m_bVisible 的语义）
/// + `sfx2::SfxNotebookBar` 经 `Bindings` 绑定的动作。
///
/// 注意：`ctx` 由视图每帧构造并**移动**给 `UiLayout::render_top`（见 `UiLayoutManager::render_chrome`），
/// 因此 `tool_group`（`AnyElement`）与回调可按值取出，无需 `Clone`。
#[allow(dead_code)]
pub struct ChromeCtx {
    /// 当前文档类型（驱动 Tabbed 的高亮 tab 与切换目标）。
    pub model_kind: ModelKind,
    /// 窗口标题（如 "Untitled" / 文件名）。
    pub name: SharedString,
    /// 是否有未保存修改（用于标题星号与保存按钮高亮）。
    pub dirty: bool,
    /// 侧栏是否展开（Standard 工具栏的「侧栏开关」据此显示 ◧ / ☰）。
    pub sidebar_open: bool,
    /// 视图自身的中间工具按钮组（保留各视图现有按钮：文本 B/I/U/≡，表格 ＋工作表，演示 新建）。
    /// Tabbed 布局不使用它（改用下方的固定动作行），但为统一 `ChromeCtx` 形状仍保留。
    pub tool_group: AnyElement,
    /// 保存动作（写磁盘 + 进最近列表）。
    pub on_save: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    /// 侧栏开关动作（文本为格式侧栏开/关；表格/演示为 no-op）。
    pub on_toggle_sidebar: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    /// 格式切换动作（"bold" / "italic" / "underline" / "align" / ...）；表格/演示为 no-op。
    /// 用 `Rc` 以便 Tabbed 布局的多个按钮共享同一回调（Box 不可 Clone）。
    pub on_format: Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>,
    /// 文档类型切换动作（Tabbed 的 tab 点击触发，打开对应类型新窗口）。同样用 `Rc`。
    pub on_switch_model: Rc<dyn Fn(ModelKind, &mut Window, &mut App) + 'static>,
}

/// 一个可渲染的 UI 单元（顶部 chrome）。每种 UI 模式（standard / tabbed）实现本 trait，
/// 由 `UiLayoutManager` 注册并按当前模式调度。
///
/// mirrors LibreOffice: `framework::UIElement`（结构体 + XUIElement 接口；
/// LO 用 name/type/visibility 描述元素，EWP 用 trait 方法直接产出元素树）。
///
/// `render_top` 返回 `AnyElement` 以支持 `dyn` 分发（`Box<dyn UiLayout>` 存入注册表），
/// 视图先把 `body` 建成 `AnyElement` 再交给管理器（见 `docs/system_design.md` §1.3、§7）。
pub trait UiLayout {
    /// 布局唯一标识（"standard" / "tabbed"），用作持久化 key 与注册表 key。
    fn id(&self) -> &'static str;
    /// 菜单显示名（"标准工具栏" / "标签页式"）。返回 `String` 以支持 i18n。
    #[allow(dead_code)] // v1 保留：菜单用 action 触发，label 供后续设置页/动态菜单使用。
    fn label(&self) -> String;
    /// 渲染顶部 chrome，`body` 为文档内容（不含顶部栏），置于下方。
    /// 返回 `AnyElement` 以便 `dyn` 分发。
    fn render_top(
        &self,
        ctx: ChromeCtx,
        body: AnyElement,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement;
}
