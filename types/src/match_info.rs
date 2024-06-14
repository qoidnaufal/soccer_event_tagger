use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatchInfo {
    pub match_date: String,
    pub match_id: String,
    pub team_home: TeamInfo,
    pub team_away: TeamInfo,
}

impl MatchInfo {
    pub fn assign_id(&mut self) {
        let uuid = uuid::Uuid::now_v7().as_simple().to_string();
        let match_id = format!("{}{}", self.match_date.clone(), uuid);
        self.match_id = match_id;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamInfo {
    pub team_state: String,
    pub team_name: String,
    pub players: Vec<PlayerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerInfo {
    pub team_name: String,
    pub number: String,
    pub player_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerQuery {
    pub match_id: String,
    pub team_name: String,
    pub number: String,
}
