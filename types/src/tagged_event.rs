use serde::{Deserialize, Serialize};

use super::{Event, Point};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedEvent {
    pub match_id: String,
    pub match_teams: String,
    pub opponent_team: String,
    pub uuid: String,
    pub time_start: f64,
    pub loc_start: Point,
    pub time_end: f64,
    pub loc_end: Point,
    pub player_name: String,
    pub team_name: String,
    pub event: Event,
}

impl PartialEq for TaggedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
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

impl Default for TaggedEvent {
    fn default() -> Self {
        Self {
            match_id: String::new(),
            match_teams: String::new(),
            opponent_team: String::new(),
            uuid: String::new(),
            time_start: 0.,
            player_name: String::new(),
            team_name: String::new(),
            loc_start: Point::default(),
            event: Event::default(),
            time_end: 0.,
            loc_end: Point::default(),
        }
    }
}

impl TaggedEvent {
    pub fn assign_uuid(&mut self) {
        let uuid = uuid::Uuid::now_v7().as_simple().to_string();
        let uuid = format!("{}", uuid);
        self.uuid = uuid;
    }
}
