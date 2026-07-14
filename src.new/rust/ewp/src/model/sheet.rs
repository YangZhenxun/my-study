//! 电子表格模型（类 Calc / Excel）。

use serde::{Deserialize, Serialize};

use crate::model::common::{Id, TextStyle};

/// 单元格值。公式以字符串保存，由计算引擎后续求值。
#[derive(Serialize, Deserialize, Clone, Debug)]
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

/// 一张工作表（二维单元格网格）。
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Sheet {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub cells: Vec<Vec<Cell>>,
    #[serde(default)]
    pub cols: usize,
    #[serde(default)]
    pub rows: usize,
}

/// 一本工作簿。
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Workbook {
    #[serde(default)]
    pub id: Id,
    #[serde(default)]
    pub sheets: Vec<Sheet>,
}
