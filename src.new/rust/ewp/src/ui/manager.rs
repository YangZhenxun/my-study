//! UI 布局管理器：注册表 + 当前激活模式 + 渲染调度 + 持久化接线。
//!
//! mirrors LibreOffice:
//!   - `framework::ToolbarLayoutManager`（`framework/source/layoutmanager/toolbarlayoutmanager.cxx`，
//!       持有已装载布局集合、当前激活模式，负责 doLayout + createUIElement）
//!   - `framework::UIElementFactoryManager`（`:uielementfactorymanager.cxx`，
//!       `registerFactory` / `getFactory` / `createUIElement`）→ 本结构的 `registry` +
//!       `register_layout` / `current_layout`
//!   - `framework::ModuleUIConfigurationManager`（`:moduleuiconfigurationmanager.cxx`，
//!       按 module 装载 UI 配置）→ 本结构的 `per_model_default` + `default_mode_for`
//!   - `sfx2::SfxNotebookBar`（`ExecMethod` 保存 + 通知重绘）→ 本结构的 `set_mode` +
//!        `ui::persistence`

use std::collections::HashMap;

use gpui::{AnyElement, App, Global, Window};

use crate::ui::layout::{ChromeCtx, ModelKind, UiLayout};
use crate::ui::persistence::{load_ui_mode, save_ui_mode, UiModeSettings};
use crate::ui::standard::StandardToolbar;
use crate::ui::tabbed::TabbedLayout;

/// 多 UI 模式管理器：持有已注册布局、当前激活模式、每文档类型的默认模式槽位。
///
/// 作为 GPUI global 注册（`cx.set_global(UiLayoutManager::new_with_defaults())`），
/// 运行时通过 `UiLayoutManager::global(cx)` / `global_mut(cx)` 访问（与 `ThemeColors::current()`
/// 同款「全局单例」风格，见 `docs/system_design.md` §1.3）。
pub struct UiLayoutManager {
    /// 模式注册表：id → 布局实例。mirrors LibreOffice: UIElementFactoryManager 的工厂 map。
    registry: HashMap<&'static str, Box<dyn UiLayout>>,
    /// 当前激活模式 id。mirrors LibreOffice: SfxNotebookBar 的「Active」状态。
    current: &'static str,
    /// 每文档类型的默认模式（v1 全局默认，槽位保留，决策④暂不启用）。
    /// mirrors LibreOffice: ModuleUIConfigurationManager（Writer/Calc/Impress 各自默认 toolbar 集合）。
    per_model_default: HashMap<ModelKind, &'static str>,
}

impl Global for UiLayoutManager {}

impl UiLayoutManager {
    /// 构造并注册全部内置布局（standard / tabbed）。
    pub fn new_with_defaults() -> Self {
        let mut m = Self {
            registry: HashMap::new(),
            current: "standard",
            per_model_default: HashMap::new(),
        };
        // mirrors LibreOffice: UIElementFactoryManager::registerFactory
        m.register_layout(Box::new(StandardToolbar));
        m.register_layout(Box::new(TabbedLayout));
        m
    }

    /// 注册一个布局实现（键为其 `id()`）。
    pub fn register_layout(&mut self, layout: Box<dyn UiLayout>) {
        self.registry.insert(layout.id(), layout);
    }

    /// 当前激活的布局（不可变引用）。
    /// 保留为公共 API（mirrors LibreOffice: ToolbarLayoutManager 取当前 UIElement）；
    /// 当前 `render_chrome` 直接按 `current_mode()` 分派，故本方法暂未内部调用。
    #[allow(dead_code)]
    pub fn current_layout(&self) -> &dyn UiLayout {
        // 注册表恒含 current（注册或 set_mode 时保证），unwrap 安全。
        self.registry[self.current].as_ref()
    }

    /// 当前激活模式 id（"standard" / "tabbed"）。
    pub fn current_mode(&self) -> &'static str {
        self.current
    }

    /// 全部可用模式：(id, label) 列表，供菜单/设置枚举。
    #[allow(dead_code)] // v1 保留：设置页/动态菜单将枚举模式，当前仅菜单 action 触发切换。
    pub fn available_modes(&self) -> Vec<(&'static str, String)> {
        self.registry.values().map(|l| (l.id(), l.label())).collect()
    }

    /// 切换模式 + 持久化（mirrors LibreOffice: SfxNotebookBar::ExecMethod）。
    ///
    /// 未知 id 回退 "standard"。切换后写入 `ui_mode.json`（含全局 default 与 per_model 槽位）。
    pub fn set_mode(&mut self, id: &'static str) {
        let id = if self.registry.contains_key(id) {
            id
        } else {
            "standard"
        };
        self.current = id;
        let settings = UiModeSettings {
            default: id.to_string(),
            per_model: self
                .per_model_default
                .iter()
                .map(|(k, v)| (format!("{:?}", k), v.to_string()))
                .collect(),
        };
        save_ui_mode(&settings);
    }

    /// 按文档类型取默认模式（v1 回退当前全局模式；per_model_default 槽位预留）。
    ///
    /// mirrors LibreOffice: ModuleUIConfigurationManager（Writer/Calc/Impress 各自默认）。
    /// 决策④：v1 先全局默认，本方法保留供后续 per-model 默认启用。
    #[allow(dead_code)]
    pub fn default_mode_for(&self, kind: ModelKind) -> &'static str {
        self.per_model_default.get(&kind).copied().unwrap_or(self.current)
    }

    /// 渲染 chrome：按当前模式委托对应布局（mirrors LibreOffice:
    /// ToolbarLayoutManager::doLayout + createUIElement）。
    ///
    /// 设计为关联函数（不放 `&self`）以避免「持有 `&UiLayoutManager`（借 `cx` 不可变）
    /// 同时把 `cx` 作为 `&mut App` 传入布局」的借用冲突：先以 `current_mode()`
    /// 取出与 `'static` 等长的模式 id（释放对 `cx` 的借用），再按 id 分派到布局的
    /// `render_top`（`StandardToolbar` / `TabbedLayout` 均为零尺寸类型，随用随构）。
    /// `ctx` 与 `body` 按值移动给布局。
    pub fn render_chrome(
        cx: &mut App,
        window: &mut Window,
        ctx: ChromeCtx,
        body: AnyElement,
    ) -> AnyElement {
        let mode = Self::global(cx).current_mode();
        match mode {
            "tabbed" => TabbedLayout.render_top(ctx, body, window, cx),
            _ => StandardToolbar.render_top(ctx, body, window, cx),
        }
    }

    /// 全局只读访问（与 `ThemeColors::current()` 同款风格）。
    pub fn global(cx: &App) -> &UiLayoutManager {
        cx.global::<UiLayoutManager>()
    }

    /// 全局可变访问（切换模式、注册布局用）。
    pub fn global_mut(cx: &mut App) -> &mut UiLayoutManager {
        cx.global_mut::<UiLayoutManager>()
    }
}

/// 启动时装载已持久化的模式（无效 id 回退 "standard"）。
///
/// mirrors LibreOffice: `lcl_getNotebookbarFileName`（读取激活模式）。
pub fn load_initial_mode() -> &'static str {
    let settings = load_ui_mode();
    match settings.default.as_str() {
        "tabbed" => "tabbed",
        _ => "standard",
    }
}
