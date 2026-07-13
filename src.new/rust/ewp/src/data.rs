use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
pub fn load() -> AppData {
    let path = data_file_path();
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppData::default(),
    }
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
