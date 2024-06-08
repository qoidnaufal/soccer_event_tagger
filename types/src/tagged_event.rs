use std::hash::Hash;

use serde::{Deserialize, Serialize};

use super::{Event, Point};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedEvent {
    pub uuid: String,
    pub time_start: f64,
    pub player_name: String,
    pub team_name: String,
    pub loc_start: Point,
    pub event: Event,
    pub time_end: f64,
    pub loc_end: Point,
}

impl PartialEq for TaggedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
            && self.loc_start == other.loc_start
            && self.player_name == other.player_name
    }
}

impl Eq for TaggedEvent {}

impl Ord for TaggedEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl PartialOrd for TaggedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for TaggedEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state)
    }
}

impl Default for TaggedEvent {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            time_start: 0.,
            player_name: String::new(),
            team_name: String::new(),
            loc_start: Point::new(),
            event: Event::Other {
                name: String::new(),
            },
            time_end: 0.,
            loc_end: Point::new(),
        }
    }
}

impl TaggedEvent {
    pub fn new() -> Self {
        let uuid = "".to_string();
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
            uuid,
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
