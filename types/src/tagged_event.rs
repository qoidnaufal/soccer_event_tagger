use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaggedEvent {
    pub uuid: String,
    pub match_id: String,
    pub player_id: String,
    pub match_date: String,
    pub match_teams: String,
    pub opponent_team: String,
    pub time_start: f64,
    pub x_start: Option<i32>,
    pub y_start: Option<i32>,
    pub time_end: Option<f64>,
    pub x_end: Option<i32>,
    pub y_end: Option<i32>,
    pub play_position: Option<String>,
    pub player_name: String,
    pub team_name: String,
    pub event_name: String,
    pub event_type: Option<String>,
    pub event_source: Option<String>,
    pub outcome: Option<String>,
    pub team_end: Option<String>,
    pub player_end: Option<String>,
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

impl TaggedEvent {
    pub fn assign_uuid(&mut self) {
        self.uuid = uuid::Uuid::now_v7().as_simple().to_string();
    }

    pub fn assign_event_from_args(
        &mut self,
        event_args: &str,
        team_args: Option<String>,
        player_args: Option<String>,
    ) {
        match event_args {
            // --- pass
            "ps" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "pi" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "pb" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Blocked".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "po" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "pc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "pp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "pr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- goalkick
            "gks" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Goal Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "gki" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Goal Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "gko" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Goal Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "gkc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Goal Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "gkp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Goal Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "gkr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Goal Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- shot
            "son" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("On Target".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "sof" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Off Target".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "sb" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Blocked".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "sg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- own goal
            "ogop" => {
                self.event_name = "Own Goal".to_string();
                self.event_type = None;
                self.event_source = Some("Open Play".to_string());
                self.outcome = Some("Own Goal".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "ogsp" => {
                self.event_name = "Own Goal".to_string();
                self.event_type = None;
                self.event_source = Some("Set Piece".to_string());
                self.outcome = Some("Own Goal".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            // --- penalty kick pass
            "pkps" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Penalty Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "pkpi" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Penalty Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "pkpo" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Penalty Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "pkpc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Penalty Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "pkpr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Penalty Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- penalty kick shot
            "pkon" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Penalty Kick".to_string());
                self.event_source = None;
                self.outcome = Some("On Target".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "pkof" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Penalty Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Off Target".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "pkg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Penalty Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- carry
            "drp" => {
                self.event_name = "Dribble".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = Some("Pass".to_string());
                self.player_end = None;
                self.team_end = None;
            }
            "drs" => {
                self.event_name = "Dribble".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = Some("Shot".to_string());
                self.player_end = None;
                self.team_end = None;
            }
            "drlb" => {
                self.event_name = "Dribble".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = Some("Lost Ball".to_string());
                self.player_end = None;
                self.team_end = None;
            }
            "drfw" => {
                self.event_name = "Dribble".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = Some("Foul Won".to_string());
                self.player_end = None;
                self.team_end = None;
            }
            // --- crossing
            "crs" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Crossing".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "cri" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Crossing".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "crc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Crossing".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "crb" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Crossing".to_string());
                self.event_source = None;
                self.outcome = Some("Blocked".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "cro" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Crossing".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "crp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Crossing".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "crr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Crossing".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- throw
            "tis" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Throw In".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "tii" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Throw In".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "tic" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Throw In".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "tio" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Throw In".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "tip" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Throw In".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "tir" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Throw In".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- corner kick
            "cks" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Corner Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "cki" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Corner Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "ckc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Corner Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "cko" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Corner Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "ckp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Corner Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "ckr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Corner Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "ckg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Corner Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- direct free kick
            "fkon" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("On Target".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "fkof" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Off Target".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "fkb" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Blocked".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "fkg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- indirect free kick
            "fkps" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "fkpi" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "fkpo" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "fkpc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "fkpp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "fkpr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- recovery
            "r" => {
                self.event_name = "Recovery".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- lost ball
            "lb" => {
                self.event_name = "Lost Ball".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- foul
            "fl" => {
                self.event_name = "Fouled".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "fw" => {
                self.event_name = "Foul Won".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "ofsop" => {
                self.event_name = "Offside".to_string();
                self.event_type = Some("Open Play".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "ofsfk" => {
                self.event_name = "Offside".to_string();
                self.event_type = Some("Free Kick".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            // --- tackle
            "tw" => {
                self.event_name = "Tackle".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = Some("Won".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "tl" => {
                self.event_name = "Tackle".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = Some("Lost".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            // --- duel
            "daw" => {
                self.event_name = "Duel".to_string();
                self.event_type = Some("Aerial".to_string());
                self.event_source = None;
                self.outcome = Some("Won".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "dal" => {
                self.event_name = "Duel".to_string();
                self.event_type = Some("Aerial".to_string());
                self.event_source = None;
                self.outcome = Some("Lose".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "dgw" => {
                self.event_name = "Duel".to_string();
                self.event_type = Some("Ground".to_string());
                self.event_source = None;
                self.outcome = Some("Won".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "dgl" => {
                self.event_name = "Duel".to_string();
                self.event_type = Some("Ground".to_string());
                self.event_source = None;
                self.outcome = Some("Lose".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            // --- clearance
            "clr" => {
                self.event_name = "Clearance".to_string();
                self.event_type = None;
                self.event_source = Some("Open Play".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            // --- pressure
            "prs" => {
                self.event_name = "Pressure".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            // --- save
            "svop" => {
                self.event_name = "Save".to_string();
                self.event_type = None;
                self.event_source = Some("Open Play".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "svpk" => {
                self.event_name = "Save".to_string();
                self.event_type = None;
                self.event_source = Some("Penalty".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "svfk" => {
                self.event_name = "Save".to_string();
                self.event_type = None;
                self.event_source = Some("Freekick".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "svck" => {
                self.event_name = "Save".to_string();
                self.event_type = None;
                self.event_source = Some("Cornerkick".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "svot" => {
                self.event_name = "Save".to_string();
                self.event_type = None;
                self.event_source = Some("Other".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            // --- conceded
            "cdop" => {
                self.event_name = "Conceded".to_string();
                self.event_type = None;
                self.event_source = Some("Open Play".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cdpk" => {
                self.event_name = "Conceded".to_string();
                self.event_type = None;
                self.event_source = Some("Penalty".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cdfk" => {
                self.event_name = "Conceded".to_string();
                self.event_type = None;
                self.event_source = Some("Freekick".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cdck" => {
                self.event_name = "Conceded".to_string();
                self.event_type = None;
                self.event_source = Some("Cornerkick".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cdot" => {
                self.event_name = "Conceded".to_string();
                self.event_type = None;
                self.event_source = Some("Other".to_string());
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            // --- kickoff first half
            "kofhps" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "kofhpi" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "kofhpc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "kofhpp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "kofho" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "kofhr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "kofhg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Kick Off First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- kickoff second half
            "koshps" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koshpi" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koshpc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koshpp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "kosho" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "koshr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koshg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Kick Off Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- kickoff extra time first half
            "koefhps" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koefhpi" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koefhpc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koefhpp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koefho" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "koefhr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koefhg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Kick Off Extra Time First Half".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- kickoff extra time second half
            "koeshps" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koeshpi" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koeshpc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koeshpp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koesho" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "koeshr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Extra Time Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koeshg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Kick Off Extra Time Second Half".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- kickoff after conceding
            "korps" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Restart".to_string());
                self.event_source = None;
                self.outcome = Some("Success".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "korpi" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Restart".to_string());
                self.event_source = None;
                self.outcome = Some("Intercepted".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "korpc" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Restart".to_string());
                self.event_source = None;
                self.outcome = Some("Catched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "korpp" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Restart".to_string());
                self.event_source = None;
                self.outcome = Some("Punched".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "koro" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Restart".to_string());
                self.event_source = None;
                self.outcome = Some("Out of Play".to_string());
                self.team_end = None;
                self.player_end = None;
            }
            "korr" => {
                self.event_name = "Pass".to_string();
                self.event_type = Some("Kick Off Restart".to_string());
                self.event_source = None;
                self.outcome = Some("Recovered".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            "korg" => {
                self.event_name = "Shot".to_string();
                self.event_type = Some("Kick Off Restart".to_string());
                self.event_source = None;
                self.outcome = Some("Goal".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
            }
            // --- time marker
            "eofh" => {
                self.event_name = "End of First Half".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "eosh" => {
                self.event_name = "End of Second Half".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "eoefh" => {
                self.event_name = "End of Extra Time First Half".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "eoesh" => {
                self.event_name = "End of Extra Time Second Half".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "eom" => {
                self.event_name = "End of Match".to_string();
                self.event_type = None;
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            // --- change position
            "cpgk" => {
                self.event_name = "Change Position".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("GK".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cprfb" => {
                self.event_name = "Change Position".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("RFB".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cplfb" => {
                self.event_name = "Change Position".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("LFB".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cpcb" => {
                self.event_name = "Change Position".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("CB".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cpmf" => {
                self.event_name = "Change Position".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("MF".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cprwg" => {
                self.event_name = "Change Position".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("RWG".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cplwf" => {
                self.event_name = "Change Position".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("LWG".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            "cpcf" => {
                self.event_name = "Change Position".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("CF".to_string());
                self.event_source = None;
                self.outcome = None;
                self.team_end = None;
                self.player_end = None;
            }
            // --- subs
            "subsgk" => {
                self.event_name = "Subs Out".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("GK".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
            "subsrfb" => {
                self.event_name = "Subs Out".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("RFB".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
            "subslfb" => {
                self.event_name = "Subs Out".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("LFB".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
            "subscb" => {
                self.event_name = "Subs Out".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("CB".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
            "subsmf" => {
                self.event_name = "Subs Out".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("MF".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
            "subsrwg" => {
                self.event_name = "Subs Out".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("RWG".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
            "subslwg" => {
                self.event_name = "Subs Out".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("LWG".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
            "subscf" => {
                self.event_name = "Subs Out".to_string();
                self.event_type = Some("Play".to_string());
                self.play_position = Some("CF".to_string());
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
            // --- other events
            "yc" => {
                self.event_name = "Yellow Card".to_string();
                self.event_type = None;
                self.play_position = None;
                self.team_end = None;
                self.player_end = None;
                self.event_source = None;
                self.outcome = None;
            }
            "syc" => {
                self.event_name = "Second Yellow Card".to_string();
                self.event_type = None;
                self.play_position = None;
                self.team_end = None;
                self.player_end = None;
                self.event_source = None;
                self.outcome = None;
            }
            "rc" => {
                self.event_name = "Red Card".to_string();
                self.event_type = None;
                self.play_position = None;
                self.team_end = None;
                self.player_end = None;
                self.event_source = None;
                self.outcome = None;
            }
            unregistered => {
                self.event_name = format!("Unregistered: {}", unregistered);
                self.event_type = None;
                self.play_position = None;
                self.team_end = team_args;
                self.player_end = player_args;
                self.event_source = None;
                self.outcome = None;
            }
        }
    }
}
