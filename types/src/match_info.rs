use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq)]
pub struct MatchInfo {
    pub match_id: String,
    pub match_date: String,
    pub team_home: String,
    pub team_away: String,
}

impl PartialEq for MatchInfo {
    fn eq(&self, other: &Self) -> bool {
        self.match_id == other.match_id
    }
}

impl MatchInfo {
    pub fn assign_id(&mut self) {
        let uuid = uuid::Uuid::new_v4().as_simple().to_string();
        self.match_id = uuid;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq)]
pub struct PlayerInfo {
    pub player_id: String,
    pub match_id: String, // use this as table name
    pub team_name: String,
    pub team_state: String,
    pub number: i32,
    pub player_name: String,
    pub start: bool,
    pub play: bool,
    pub play_position: Vec<String>,
}

impl PartialEq for PlayerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.player_id == other.player_id
    }
}

impl PlayerInfo {
    pub fn assign_id(&mut self) {
        let team_code = self
            .team_name
            .as_bytes()
            .iter()
            .map(|n| n.to_string())
            .collect::<String>();
        self.player_id = format!("{}_{:03}", team_code, self.number);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInfoQuery {
    pub match_id: String,
    pub team_state: String,
}
