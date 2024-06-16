use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq)]
pub struct MatchInfo {
    pub match_date: String,
    pub match_id: String,
    pub team_home: TeamInfo,
    pub team_away: TeamInfo,
}

impl PartialEq for MatchInfo {
    fn eq(&self, other: &Self) -> bool {
        self.match_id == other.match_id
    }
}

impl MatchInfo {
    pub fn assign_id(&mut self) {
        let uuid = uuid::Uuid::now_v7().as_simple().to_string();
        let match_id = format!("{}-{}", self.match_date.clone(), uuid);
        self.match_id = match_id;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq)]
pub struct TeamInfo {
    pub team_state: String,
    pub team_name: String,
    pub players: Vec<PlayerInfo>,
}

impl PartialEq for TeamInfo {
    fn eq(&self, other: &Self) -> bool {
        self.team_name == other.team_name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct PlayerInfo {
    pub team_name: String,
    pub number: String,
    pub player_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInfoQuery {
    pub match_id: String,
    pub team_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerQuery {
    pub match_id: String,
    pub team_name: String,
    pub number: String,
}
