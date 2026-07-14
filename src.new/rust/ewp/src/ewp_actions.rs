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
