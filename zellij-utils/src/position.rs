use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct Position {
    pub line: Line,
    pub column: Column,
}

impl Position {
    pub fn new(line: i32, column: u16) -> Self {
        Self {
            line: Line(line as isize),
            column: Column(column as usize),
        }
    }

    pub fn relative_to(&self, line: usize, column: usize) -> Self {
        Self {
            line: Line(self.line.0 - line as isize),
            column: Column(self.column.0.saturating_sub(column)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, PartialOrd)]
pub struct Line(pub isize);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, PartialOrd)]
pub struct Column(pub usize);
