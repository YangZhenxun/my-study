use gpui::actions;

// ──────────────────────────────────────────────
// Actions —— 菜单项触发的动作
// ──────────────────────────────────────────────

actions!(
    ewp,
    [
        // File
        NewProject,
        NewWindow,
        OpenProject,
        CloseProject,
        CloseWindow,
        // Edit
        Cut,
        Copy,
        Paste,
        SelectAll,
        Undo,
        Redo,
        Find,
        // View
        ZoomIn,
        ZoomOut,
        ResetZoom,
        ToggleFullScreen,
        // App
        Settings,
        Languages,
        Quit,
        About,
        Hide,
        HideOthers,
        ShowAll,
        // Window
        Minimize,
        Zoom,
        // Help
        EwpHelp,
        OpenDocumentation,
        ReportIssue,
    ]
);

/// 切换 UI 模式（标准工具栏 / 标签页式）。
///
/// 携带目标模式 id（"standard" / "tabbed"），由 `UiLayoutManager::set_mode` 处理。
/// mirrors LibreOffice: `sfx2::SfxNotebookBar::ExecMethod`（经 action 触发切换 notebookbar）。
#[derive(Clone, PartialEq, Eq, Debug, gpui::Action)]
#[action(namespace = ewp, no_json)]
pub struct SetUiMode(pub String);
