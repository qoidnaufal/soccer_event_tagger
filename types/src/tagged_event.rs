use serde::{Deserialize, Serialize};

use super::{Event, Point};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedEvent {
    pub time_start: f64,
    pub player_name: String,
    pub team_name: String,
    pub loc_start: Point,
    pub event: Event,
    pub time_end: f64,
    pub loc_end: Point,
}

impl TaggedEvent {
    pub fn new() -> Self {
        let time_start = 0.0;
        let player_name = "".to_string();
        let team_name = "".to_string();
        let loc_start = Point::new();
        let event = Event::Other {
            name: "No Event".to_string(),
        };
        let time_end = 0.0;
        let loc_end = Point::new();
        Self {
            time_start,
            player_name,
            team_name,
            loc_start,
            event,
            time_end,
            loc_end,
        }
    }
}
