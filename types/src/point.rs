use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.as_ref().unwrap() == other.x.as_ref().unwrap()
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
