//! 扩展宿主：发现、加载、注册扩展
//!
//! 加载顺序：
//! 1. 内置主题（编译进二进制，`include_str!`）
//! 2. 用户扩展（扫描 `data/extensions/` 下的 `extension.toml`）

use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::data;
use crate::styles::ThemeColors;
use crate::extension::manifest::ExtensionManifest;
use crate::extension::theme_file::ThemeFile;

/// 全局缓存的 ExtensionHost（进程级单例，首次访问时加载）。
static HOST: OnceLock<ExtensionHost> = OnceLock::new();

impl ExtensionHost {
    /// 获取全局 ExtensionHost 实例（进程级单例）。
    pub fn shared() -> &'static ExtensionHost {
        HOST.get_or_init(ExtensionHost::load)
    }
}

/// 已加载的主题。
pub struct LoadedTheme {
    /// 主题 ID（如 "light" / "dark" / "my-custom"）。
    pub id: String,
    /// 主题显示名称（如 "EWP Dark"）。
    pub name: String,
    /// 运行时配色。
    pub colors: ThemeColors,
}

/// 扩展宿主，管理所有已加载的扩展。
pub struct ExtensionHost {
    /// 所有已加载的主题（内置 + 用户）。
    themes: Vec<LoadedTheme>,
}

impl ExtensionHost {
    /// 加载所有扩展：内置主题 + 用户扩展目录。
    pub fn load() -> Self {
        let mut themes = Vec::new();

        // 1. 内置主题（编译进二进制）
        themes.push(load_builtin_theme("light", include_str!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/themes/light.json")
        )));
        themes.push(load_builtin_theme("dark", include_str!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/themes/dark.json")
        )));

        // 2. 用户扩展（扫描 data/extensions/）
        let ext_dir = data::data_dir().join("extensions");
        if ext_dir.exists() {
            if let Ok(entries) = fs::read_dir(&ext_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(theme) = load_extension_theme(&path) {
                            themes.push(theme);
                        }
                    }
                }
            }
        }

        Self { themes }
    }

    /// 按 ID 查找主题。
    pub fn get_theme(&self, id: &str) -> Option<&LoadedTheme> {
        self.themes.iter().find(|t| t.id == id)
    }

    /// 返回所有主题的 (id, name) 列表（用于 UI 选择器）。
    pub fn theme_list(&self) -> Vec<(&str, &str)> {
        self.themes.iter().map(|t| (t.id.as_str(), t.name.as_str())).collect()
    }

    /// 获取当前选中主题的配色。若未找到则回退到第一个主题。
    pub fn colors_for(&self, theme_id: &str) -> ThemeColors {
        self.get_theme(theme_id)
            .map(|t| t.colors)
            .unwrap_or_else(|| self.themes.first().map(|t| t.colors).unwrap_or_default())
    }
}

/// 从 JSON 字符串加载内置主题。
fn load_builtin_theme(id: &str, json: &str) -> LoadedTheme {
    let theme_file: ThemeFile = serde_json::from_str(json).unwrap_or_else(|e| {
        eprintln!("[EWP] Failed to parse built-in theme '{id}': {e}");
        ThemeFile {
            name: id.to_string(),
            colors: Default::default(),
        }
    });
    LoadedTheme {
        id: id.to_string(),
        name: theme_file.name,
        colors: theme_file.colors.to_theme_colors(),
    }
}

/// 从扩展目录加载主题（读 extension.toml → 读 theme JSON）。
fn load_extension_theme(dir: &PathBuf) -> Option<LoadedTheme> {
    let manifest_path = dir.join("extension.toml");
    let manifest_str = fs::read_to_string(&manifest_path).ok()?;
    let manifest: ExtensionManifest = toml::from_str(&manifest_str).ok()?;

    // 加载第一个主题文件（一个扩展目前只支持一个主题）
    let theme_entry = manifest.themes.first()?;
    let theme_path = dir.join(&theme_entry.path);
    let theme_str = fs::read_to_string(&theme_path).ok()?;
    let theme_file: ThemeFile = serde_json::from_str(&theme_str).ok()?;

    Some(LoadedTheme {
        id: manifest.id,
        name: theme_file.name,
        colors: theme_file.colors.to_theme_colors(),
    })
}
