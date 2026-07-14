use crate::ewp_actions;
use gpui::{Menu, MenuItem, OsAction, SystemMenuType};
use rust_i18n::t;

/// 构建应用菜单栏，返回菜单列表。
/// 调用方用 `cx.set_menus(app_menus())` 设置。
///
/// 结构参考 Zed 的 app_menus.rs，适配 EWP Studio 的功能。
pub fn app_menus() -> Vec<Menu> {
    vec![
        // ═══ EWP Studio（应用菜单，macOS 第一个菜单）═══
        Menu {
            name: "EWP Studio".into(),
            items: vec![
                MenuItem::action(t!("menu.app.about").to_string(), ewp_actions::About),
                MenuItem::separator(),
                // Settings 子菜单
                MenuItem::submenu(Menu {
                    name: t!("menu.app.settings").to_string().into(),
                    items: vec![
                        MenuItem::action(
                            t!("menu.app.open_settings").to_string(),
                            ewp_actions::Settings,
                        ),
                        MenuItem::action(
                            t!("menu.app.language").to_string(),
                            ewp_actions::Languages,
                        ),
                    ],
                }),
                MenuItem::separator(),
                // macOS: Services 系统子菜单
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu(
                    t!("menu.app.services").to_string(),
                    SystemMenuType::Services,
                ),
                #[cfg(target_os = "macos")]
                MenuItem::separator(),
                // macOS: Hide / Hide Others / Show All
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("menu.app.hide").to_string(), ewp_actions::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action(
                    t!("menu.app.hide_others").to_string(),
                    ewp_actions::HideOthers,
                ),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("menu.app.show_all").to_string(), ewp_actions::ShowAll),
                #[cfg(target_os = "macos")]
                MenuItem::separator(),
                MenuItem::action(t!("menu.app.quit").to_string(), ewp_actions::Quit),
            ],
        },
        // ═══ File ═══
        Menu {
            name: t!("menu.file.name").to_string().into(),
            items: vec![
                MenuItem::action(
                    t!("menu.file.new_project").to_string(),
                    ewp_actions::NewProject,
                ),
                MenuItem::action(
                    t!("menu.file.new_window").to_string(),
                    ewp_actions::NewWindow,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("menu.file.open_project").to_string(),
                    ewp_actions::OpenProject,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("menu.file.close_project").to_string(),
                    ewp_actions::CloseProject,
                ),
                MenuItem::action(
                    t!("menu.file.close_window").to_string(),
                    ewp_actions::CloseWindow,
                ),
            ],
        },
        // ═══ Edit ═══
        Menu {
            name: t!("menu.edit.name").to_string().into(),
            items: vec![
                MenuItem::os_action(
                    t!("menu.edit.undo").to_string(),
                    ewp_actions::Undo,
                    OsAction::Undo,
                ),
                MenuItem::os_action(
                    t!("menu.edit.redo").to_string(),
                    ewp_actions::Redo,
                    OsAction::Redo,
                ),
                MenuItem::separator(),
                MenuItem::os_action(
                    t!("menu.edit.cut").to_string(),
                    ewp_actions::Cut,
                    OsAction::Cut,
                ),
                MenuItem::os_action(
                    t!("menu.edit.copy").to_string(),
                    ewp_actions::Copy,
                    OsAction::Copy,
                ),
                MenuItem::os_action(
                    t!("menu.edit.paste").to_string(),
                    ewp_actions::Paste,
                    OsAction::Paste,
                ),
                MenuItem::separator(),
                MenuItem::os_action(
                    t!("menu.edit.select_all").to_string(),
                    ewp_actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::separator(),
                MenuItem::action(t!("menu.edit.find").to_string(), ewp_actions::Find),
            ],
        },
        // ═══ View ═══
        Menu {
            name: t!("menu.view.name").to_string().into(),
            items: vec![
                MenuItem::action(t!("menu.view.zoom_in").to_string(), ewp_actions::ZoomIn),
                MenuItem::action(t!("menu.view.zoom_out").to_string(), ewp_actions::ZoomOut),
                MenuItem::action(
                    t!("menu.view.reset_zoom").to_string(),
                    ewp_actions::ResetZoom,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("menu.view.toggle_fullscreen").to_string(),
                    ewp_actions::ToggleFullScreen,
                ),
            ],
        },
        // ═══ Window ═══
        Menu {
            name: t!("menu.window.name").to_string().into(),
            items: vec![
                MenuItem::action(
                    t!("menu.window.minimize").to_string(),
                    ewp_actions::Minimize,
                ),
                MenuItem::action(t!("menu.window.zoom").to_string(), ewp_actions::Zoom),
            ],
        },
        // ═══ Help ═══
        Menu {
            name: t!("menu.help.name").to_string().into(),
            items: vec![
                MenuItem::action(t!("menu.help.ewp_help").to_string(), ewp_actions::EwpHelp),
                MenuItem::separator(),
                MenuItem::action(
                    t!("menu.help.documentation").to_string(),
                    ewp_actions::OpenDocumentation,
                ),
                MenuItem::action(
                    t!("menu.help.report_issue").to_string(),
                    ewp_actions::ReportIssue,
                ),
            ],
        },
    ]
}
