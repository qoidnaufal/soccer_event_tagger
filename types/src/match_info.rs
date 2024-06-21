use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq)]
pub struct MatchInfo {
    pub match_id: String,
    pub match_date: String,
    pub team_home: String,
    pub team_away: String,
}

impl std::fmt::Display for MatchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.match_id, self.team_home, self.team_away)
    }
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
        self.player_id = format!("{}{}", self.team_name, self.number);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInfoQuery {
    pub match_id: String,
    pub team_state: String,
}

#[derive(Debug, Clone, Default, Eq)]
pub struct MatchData {
    pub match_id: String,
    pub team_home: String,
    pub team_away: String,
}

impl PartialEq for MatchData {
    fn eq(&self, other: &Self) -> bool {
        self.match_id == other.match_id
    }
}

impl MatchData {
    pub fn get_from_str(input: String) -> Self {
        let mut data = input.split('/').map(|s| s.to_string());

        Self {
            match_id: data.next().unwrap_or_default(),
            team_home: data.next().unwrap_or_default(),
            team_away: data.next().unwrap_or_default(),
        }
    }
}
