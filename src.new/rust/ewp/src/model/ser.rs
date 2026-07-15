//! 原生格式序列化。
//!
//! v1 用 JSON（可读、可 diff、好调试）。同一 serde 层后续可加
//! `Cbor` 变体（ciborium / serde_cbor），无需改动模型。
//!
//! 磁盘形态：一个 `.ewp` 文件 = 带 `format` 标签的信封，包住 `Model`。
//! 后续若要"包"（manifest + 多文档 + 媒体），把信封换成 zip 即可。

use std::path::Path;

use serde::Serialize;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::model::Model;

/// v1 原生格式标签，写在信封里做版本识别。
pub const FORMAT_TAG: &str = "ewp.doc.v1";

/// 支持的落盘格式。
#[derive(Clone, Copy, Debug)]
pub enum NativeFormat {
    Json,
    // Cbor, // TODO: 后续用 ciborium/serde_cbor 实现，同一 serde 层无缝切换
}

#[derive(Serialize, Deserialize)]
struct Envelope<T> {
    format: String,
    model: T,
}

/// 把模型序列化到磁盘。
pub fn save<T: Serialize>(model: &T, path: &Path, fmt: NativeFormat) -> Result<(), String> {
    match fmt {
        NativeFormat::Json => {
            let env = Envelope {
                format: String::from(FORMAT_TAG),
                model,
            };
            let json =
                serde_json::to_string_pretty(&env).map_err(|e| e.to_string())?;
            std::fs::write(path, json).map_err(|e| e.to_string())
        }
    }
}

/// 从磁盘读回模型。
pub fn load<T: DeserializeOwned>(path: &Path, fmt: NativeFormat) -> Result<T, String> {
    match fmt {
        NativeFormat::Json => {
            let txt = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            let env: Envelope<T> =
                serde_json::from_str(&txt).map_err(|e| e.to_string())?;
            Ok(env.model)
        }
    }
}

/// 开发期手动 round-trip 测试：构造一个空 `Document`，序列化到磁盘再读回。
/// 不在启动时自动调用（避免相对路径 `data/` 不存在时刷屏 ENOENT 报错）；
/// 需要时手动调用即可。用 `data::data_dir()` 而非相对路径，确保目录存在。
#[allow(dead_code)]
pub fn demo() {
    use crate::model::text::Document;

    let doc = Document::default();
    let model = Model::Text(doc);
    let path = crate::data::data_dir().join("sample_document.ewp");

    match save(&model, &path, NativeFormat::Json) {
        Ok(()) => println!("[EWP] model saved -> {}", path.display()),
        Err(e) => eprintln!("[EWP] model save failed: {e}"),
    }

    match load::<Model>(&path, NativeFormat::Json) {
        Ok(_) => println!("[EWP] model round-trip OK"),
        Err(e) => eprintln!("[EWP] model load failed: {e}"),
    }
}
