//! UI 模式持久化（复用 `data::data_dir()`，零新依赖）。
//!
//! mirrors LibreOffice: `sfx2::SfxNotebookBar` 的
//! `lcl_getNotebookbarFileName` / `lcl_setNotebookbarFileName`
//! （读/写当前激活的 notebookbar 文件名；EWP 简化为 mode id 字符串）。
//! 文件落点为 `<data_dir>/ui_mode.json`，与现有 `data::*` 持久化同款风格。

use crate::data::data_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 持久化的 UI 模式设置。
///
/// mirrors LibreOffice: `officecfg::Office::UI::ToolbarMode::ActiveWriter/Calc/Impress`
/// 的本地投影（v1 仅全局 `default`，`per_model` 槽位保留，决策④暂不启用）。
#[derive(Default, Serialize, Deserialize)]
pub struct UiModeSettings {
    /// 全局默认模式："standard" | "tabbed"。
    #[serde(default = "default_mode")]
    pub default: String,
    /// 每文档类型的默认模式（预留；键为 `ModelKind` 的 `{:?}` 字符串）。v1 为空。
    #[serde(default)]
    pub per_model: HashMap<String, String>,
}

fn default_mode() -> String {
    "standard".to_string()
}

/// 持久化文件路径：`<data_dir>/ui_mode.json`（复用 `data_dir()`，启动期绝不写相对路径）。
fn ui_mode_file() -> PathBuf {
    data_dir().join("ui_mode.json")
}

/// 读取 UI 模式设置；文件不存在或解析失败时使用默认值（"standard"）。
pub fn load_ui_mode() -> UiModeSettings {
    match fs::read_to_string(ui_mode_file()) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => UiModeSettings::default(),
    }
}

/// 写入 UI 模式设置到 `ui_mode.json`（pretty JSON，便于 diff / 调试）。
pub fn save_ui_mode(settings: &UiModeSettings) {
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(ui_mode_file(), json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::manager::load_initial_mode;
    use std::sync::Mutex;

    /// 串行化所有触碰真实 `ui_mode.json` 的用例：该路径在整个 crate 内共享，
    /// 而 `cargo test` 默认多线程并行，不串行化会互相删除/覆盖文件导致偶发失败。
    static FS_LOCK: Mutex<()> = Mutex::new(());

    /// 用例前后清理 `ui_mode.json`，避免污染项目 `data/` 目录、保证用例相互独立。
    fn nuke() {
        let _ = fs::remove_file(ui_mode_file());
    }

    /// 落点路径必须是 `<data_dir>/ui_mode.json`（验收：确认读写位置正确）。
    #[test]
    fn file_path_is_data_dir_ui_mode_json() {
        let p = ui_mode_file();
        assert_eq!(p.file_name().unwrap(), "ui_mode.json");
        let s = p.to_string_lossy();
        assert!(s.contains("data"), "ui_mode.json 必须落在 data 目录下: {s}");
    }

    /// 往返：保存 "tabbed" → 读回仍是 "tabbed"（SetUiMode 持久化的核心保证）。
    #[test]
    fn save_then_load_roundtrip_tabbed() {
        let _g = FS_LOCK.lock().unwrap();
        nuke();
        let s = UiModeSettings {
            default: "tabbed".to_string(),
            per_model: HashMap::new(),
        };
        save_ui_mode(&s);
        let back = load_ui_mode();
        assert_eq!(back.default, "tabbed", "持久化 default 必须原样读回");
        assert!(back.per_model.is_empty());
        nuke();
    }

    /// 往返：保存含 per_model 槽位的设置 → 读回一致（后续 per-model 默认启用前的固化）。
    #[test]
    fn save_then_load_roundtrip_with_per_model() {
        let _g = FS_LOCK.lock().unwrap();
        nuke();
        let s = UiModeSettings {
            default: "standard".to_string(),
            per_model: {
                let mut m = HashMap::new();
                m.insert("Sheet".to_string(), "tabbed".to_string());
                m
            },
        };
        save_ui_mode(&s);
        let back = load_ui_mode();
        assert_eq!(back.default, "standard");
        assert_eq!(
            back.per_model.get("Sheet").map(String::as_str),
            Some("tabbed"),
            "per_model 槽位必须原样读回"
        );
        nuke();
    }

    /// 文件缺失 → `load_ui_mode` 安全回退（default 为空串，由 `load_initial_mode` 归一为 "standard"），不得 panic。
    #[test]
    fn missing_file_falls_back_gracefully() {
        let _g = FS_LOCK.lock().unwrap();
        nuke();
        assert!(!ui_mode_file().exists(), "前置：ui_mode.json 不应存在");
        let s = load_ui_mode();
        // 源码 `UiModeSettings::default()` 派生出空串（serde 的 `default = "default_mode"`
        // 仅作用于「缺字段反序列化」，不影响 `#[derive(Default)]`）；真正的归一在
        // `load_initial_mode`（任意非 "tabbed" → "standard"，见 manager.rs）。
        assert_eq!(s.default, "", "缺失文件应返回空串默认（由调用方归一）");
        assert!(s.per_model.is_empty());
        nuke();
    }

    /// 文件内容为非法 JSON → 安全回退（不 panic）。
    #[test]
    fn corrupt_json_falls_back_gracefully() {
        let _g = FS_LOCK.lock().unwrap();
        nuke();
        let _ = fs::write(ui_mode_file(), "{ this is not valid json ");
        let s = load_ui_mode();
        assert_eq!(s.default, "", "损坏文件必须安全回退默认（空串）");
        nuke();
    }

    /// 启动恢复（验收核心）：无文件时 `load_initial_mode` 归一到 "standard"。
    #[test]
    fn initial_mode_defaults_to_standard_when_unset() {
        let _g = FS_LOCK.lock().unwrap();
        nuke();
        assert_eq!(load_initial_mode(), "standard");
        nuke();
    }

    /// 启动恢复（验收核心）：文件持久化为 "tabbed" 后，`load_initial_mode` 原样读回 "tabbed"
    /// —— 即 SetUiMode 触发后「跨重启保持」。
    #[test]
    fn initial_mode_restores_persisted_tabbed() {
        let _g = FS_LOCK.lock().unwrap();
        nuke();
        save_ui_mode(&UiModeSettings {
            default: "tabbed".to_string(),
            per_model: HashMap::new(),
        });
        assert_eq!(load_initial_mode(), "tabbed");
        nuke();
    }
}
