use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl Point {
    pub fn new() -> Self {
        let x = None;
        let y = None;
        Self { x, y }
    }

    pub fn reset(&mut self) {
        self.x = None;
        self.y = None;
    }
}
