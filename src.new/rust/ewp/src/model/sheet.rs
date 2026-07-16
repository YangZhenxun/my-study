//! 电子表格模型（类 Calc / Excel）。

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::model::common::{Id, TextStyle};

/// 单元格值。公式以字符串保存，由计算引擎后续求值。
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Bool(bool),
    Formula(String),
}

/// 单个单元格。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cell {
    pub value: CellValue,
    #[serde(default)]
    pub style: TextStyle,
}

/// 一张工作表。
///
/// 单元格采用**稀疏存储**：`cells[row][col] -> Cell`，只有被写入过的单元格
/// 才占内存 / 落盘，空白单元格不存在于结构中 —— 这与 Apple Numbers 的
/// 「按需动态网格」一致，而不是一开始就预分配几千个空单元格。
///
/// `cols` / `rows` 仅表示当前网格的**声明范围**（用于滚动边界），
/// 它不会限制写入：调用 `set_cell` 写入越界坐标时会自动扩展范围。
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Sheet {
    #[serde(default)]
    pub name: String,
    /// 行 → 列 → 单元格（稀疏）。`{ "0": { "0": {...} } }`。
    /// 反序列化兼容旧格式（整张网格预分配为 `[[cell, ...], ...]` 序列）。
    #[serde(default, deserialize_with = "deserialize_cells")]
    pub cells: HashMap<usize, HashMap<usize, Cell>>,
    #[serde(default)]
    pub cols: usize,
    #[serde(default)]
    pub rows: usize,
}

impl Sheet {
    /// 读取单元格；未写入过则返回 `None`（即空白格）。
    #[allow(dead_code)]
    pub fn get_cell(&self, col: usize, row: usize) -> Option<&Cell> {
        self.cells.get(&row).and_then(|row_map| row_map.get(&col))
    }

    /// 写入单元格；若坐标超出当前 `cols`/`rows` 声明范围则自动扩展。
    /// 写入 `CellValue::Empty` 等价于清空该格（从稀疏表中移除）。
    #[allow(dead_code)]
    pub fn set_cell(&mut self, col: usize, row: usize, cell: Cell) {
        if cell.value == CellValue::Empty {
            if let Some(row_map) = self.cells.get_mut(&row) {
                row_map.remove(&col);
                if row_map.is_empty() {
                    self.cells.remove(&row);
                }
            }
            return;
        }
        if row + 1 > self.rows {
            self.rows = row + 1;
        }
        if col + 1 > self.cols {
            self.cols = col + 1;
        }
        self.cells.entry(row).or_default().insert(col, cell);
    }
}

/// `cells` 的反序列化：同时兼容新格式（行→列 的稀疏 map）与旧格式
/// （整张网格预分配的二维序列 `[[cell, ...], ...]`）。加载旧文件时把
/// 序列安静地转换为稀疏表，避免 schema 升级后旧 `.ewp` 打不开。
fn deserialize_cells<'de, D>(
    d: D,
) -> Result<HashMap<usize, HashMap<usize, Cell>>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(d).map_err(D::Error::custom)?;
    let mut cells: HashMap<usize, HashMap<usize, Cell>> = HashMap::new();

    // 新格式：`{ "row": { "col": <cell> } }`
    if let Some(obj) = v.as_object() {
        for (rk, rv) in obj {
            let Ok(row) = rk.parse::<usize>() else {
                continue;
            };
            let Some(colmap) = rv.as_object() else {
                continue;
            };
            let mut rmap = HashMap::new();
            for (ck, cv) in colmap {
                let Ok(col) = ck.parse::<usize>() else {
                    continue;
                };
                if let Ok(cell) = serde_json::from_value::<Cell>(cv.clone()) {
                    rmap.insert(col, cell);
                }
            }
            if !rmap.is_empty() {
                cells.insert(row, rmap);
            }
        }
        return Ok(cells);
    }

    // 旧格式：`[ [ <cell>? ] ]`（预分配网格，可能含 null / 空白）。
    if let Some(rows) = v.as_array() {
        for (r, rowv) in rows.iter().enumerate() {
            let Some(cols) = rowv.as_array() else {
                continue;
            };
            let mut rmap = HashMap::new();
            for (c, cellv) in cols.iter().enumerate() {
                if let Ok(cell) = serde_json::from_value::<Cell>(cellv.clone()) {
                    if cell.value != CellValue::Empty {
                        rmap.insert(c, cell);
                    }
                }
            }
            if !rmap.is_empty() {
                cells.insert(r, rmap);
            }
        }
        return Ok(cells);
    }

    Ok(cells)
}

/// 一本工作簿。
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Workbook {
    #[serde(default)]
    pub id: Id,
    #[serde(default)]
    pub sheets: Vec<Sheet>,
}
