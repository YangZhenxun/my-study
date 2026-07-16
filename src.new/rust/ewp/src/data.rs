use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// ──────────────────────────────────────────────
// 设置（持久化到 data/settings.json）
// ──────────────────────────────────────────────

/// 内置主题 ID 常量。
#[allow(dead_code)]
pub const THEME_LIGHT: &str = "light";
#[allow(dead_code)]
pub const THEME_DARK: &str = "dark";

/// 应用设置。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    /// 当前语言代码（与 rust-i18n locale 对应：en / zh-CN / zh-TW）。
    #[serde(default = "default_locale")]
    pub locale: String,
    /// 当前主题 ID（如 "light" / "dark" / 自定义扩展 ID）。
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_locale() -> String {
    "en".to_string()
}

fn default_theme() -> String {
    THEME_LIGHT.to_string()
}

/// 设置文件路径：`<data_dir>/settings.json`
fn settings_file_path() -> PathBuf {
    data_dir().join("settings.json")
}

/// 加载设置；文件不存在或解析失败时使用默认值。
pub fn load_settings() -> Settings {
    let path = settings_file_path();
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

/// 保存设置到磁盘。
pub fn save_settings(settings: &Settings) {
    let path = settings_file_path();
    match serde_json::to_string_pretty(settings) {
        Ok(json) => {
            if let Err(e) = fs::write(&path, json) {
                eprintln!("[EWP] Warning: Failed to write settings file {}: {e}", path.display());
            }
        }
        Err(e) => eprintln!("[EWP] Warning: Failed to serialize settings: {e}"),
    }
}

// ──────────────────────────────────────────────
// 数据模型
// ──────────────────────────────────────────────

/// 文件类型 —— 决定最近列表中显示的图标。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Document,
    Excel,
    PowerPoint,
    PDF,
}

impl FileType {
    /// 返回对应的 SVG 图标名称（不含扩展名）。
    pub fn icon_name(&self) -> &'static str {
        match self {
            FileType::Document => "document",
            FileType::Excel => "spreadsheet",
            FileType::PowerPoint => "presentation",
            FileType::PDF => "pdf",
        }
    }

    /// 按文件扩展名推断类型（用于「打开」时自动选图标）。
    /// 注意：本项目文稿统一为 `.ewp`，其真实类型须按加载到的 `Model`
    /// 变体判定（见 `open_project`）；此函数保留给未来直接打开
    /// `.docx` / `.xlsx` / `.pptx` 等原生格式时使用。
    #[allow(dead_code)]
    pub fn from_extension(path: &std::path::Path) -> FileType {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase());
        match ext.as_deref() {
            Some("docx") | Some("doc") => FileType::Document,
            Some("xlsx") | Some("xls") => FileType::Excel,
            Some("pptx") | Some("ppt") => FileType::PowerPoint,
            Some("pdf") => FileType::PDF,
            Some("ewp") => FileType::Document,
            _ => FileType::Document,
        }
    }
}

/// 最近打开的文档/项目。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentDoc {
    pub name: String,
    pub path: String,
    pub file_type: FileType,
}

/// 应用持久化数据 —— 整个 JSON 文件的反序列化目标。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    #[serde(default)]
    pub recent_docs: Vec<RecentDoc>,
}

// ──────────────────────────────────────────────
// 数据目录
// ──────────────────────────────────────────────

/// 返回应用数据目录路径。
///
/// 当前使用项目内 `data/` 目录（开发阶段）。
/// 生产环境可切换为平台标准目录：
/// - macOS: `~/Library/Application Support/EWP/`
/// - Linux: `~/.local/share/ewp/`
/// - Windows: `%APPDATA%\EWP\`
///
/// 目录不存在时自动创建。
pub fn data_dir() -> PathBuf {
    // 开发阶段：项目根目录下的 data/
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("[EWP] Warning: Failed to create data dir {}: {e}", dir.display());
        }
    }
    dir
}

/// 数据文件路径：`<data_dir>/data.json`
fn data_file_path() -> PathBuf {
    data_dir().join("data.json")
}

// ──────────────────────────────────────────────
// 加载 / 保存
// ──────────────────────────────────────────────

/// 从磁盘加载应用数据。文件不存在时返回默认值。
///
/// 加载后会做一次迁移清理：移除 `path` 为 `"(unsaved)"` 的残留项
/// （这些是曾经「新建但未保存」的文稿被错误地写进了最近列表，
/// 它们没有真实磁盘文件，既不该显示也不该持久化）。
pub fn load() -> AppData {
    let path = data_file_path();
    let mut data = match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppData::default(),
    };
    let before = data.recent_docs.len();
    data.recent_docs.retain(|d| d.path != "(unsaved)");
    if data.recent_docs.len() != before {
        save(&data);
    }
    data
}

/// 保存应用数据到磁盘。
pub fn save(data: &AppData) {
    let path = data_file_path();
    match serde_json::to_string_pretty(data) {
        Ok(json) => {
            if let Err(e) = fs::write(&path, json) {
                eprintln!("[EWP] Warning: Failed to write data file {}: {e}", path.display());
            }
        }
        Err(e) => eprintln!("[EWP] Warning: Failed to serialize data: {e}"),
    }
}

/// 将一个文档添加到最近列表头部（去重）。
pub fn add_recent_doc(data: &mut AppData, doc: RecentDoc) {
    // 去重：如果路径已存在，先移除旧的
    data.recent_docs.retain(|d| d.path != doc.path);
    // 插入头部
    data.recent_docs.insert(0, doc);
    // 限制最多 20 条
    data.recent_docs.truncate(20);
    save(data);
}

/// 从最近列表移除指定路径的条目（仅改列表，不碰磁盘文件）。
pub fn remove_recent_doc(data: &mut AppData, path: &str) {
    data.recent_docs.retain(|d| d.path != path);
    save(data);
}
