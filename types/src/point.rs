use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {:?}, y: {:?}", self.x, self.y)
    }
}

impl Default for Point {
    fn default() -> Self {
        let x = None;
        let y = None;
        Self { x, y }
    }
}

impl Point {
    pub fn reset(&mut self) {
        self.x = None;
        self.y = None;
    }
}
