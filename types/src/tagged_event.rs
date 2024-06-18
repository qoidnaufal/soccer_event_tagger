use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaggedEvent {
    pub match_id: String,
    pub match_teams: String,
    pub opponent_team: String,
    pub uuid: String,
    pub time_start: f64,
    pub x_start: Option<i32>,
    pub y_start: Option<i32>,
    pub time_end: f64,
    pub x_end: Option<i32>,
    pub y_end: Option<i32>,
    pub player_name: String,
    pub team_name: String,
    pub event_name: String,
    pub event_type: Option<String>,
    pub event_source: Option<String>,
    pub outcome: Option<String>,
    pub team_end: Option<String>,
    pub player_end: Option<String>,
    pub play_position: Option<String>,
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
        let uuid = uuid::Uuid::now_v7().as_simple().to_string();
        let uuid = format!("{}", uuid);
        self.uuid = uuid;
    }

    #[rustfmt::skip]
    pub fn assign_event_from_args(&mut self, event_args: &str, team_args: Option<String>, player_args: Option<String>) {
        match event_args {
            // --- pass
            "ps" => { self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pi" => { self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pb" => { self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Blocked".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "po" => { self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pc" => { self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- goalkick
            "gks" => { self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gki" => { self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gko" => { self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gkc" => { self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- shot
            "son" => { self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("On Target".to_string()); },
            "sof" => { self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Off Target".to_string()); },
            "sb" => { self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Blocked".to_string()); },
            "sg" => { self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- penalty kick pass
            "pkps" => { self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpi" => { self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpo" => { self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpc" => { self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- penalty kick shot
            "pkon" => { self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("On Target".to_string()); },
            "pkof" => { self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Off Target".to_string()); },
            "pkg" => { self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- carry
            "drp" => { self.event_name = "Dribble".to_string(); self.outcome = Some("Pass".to_string()); },
            "drs" => { self.event_name = "Dribble".to_string(); self.outcome = Some("Shot".to_string()); },
            "drlb" => { self.event_name = "Dribble".to_string(); self.outcome = Some("Lost Ball".to_string()); },
            "drfw" => { self.event_name = "Dribble".to_string(); self.outcome = Some("Foul Won".to_string()); },
            // --- crossing
            "crs" => { self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cri" => { self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "crc" => { self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "crb" => { self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Blocked".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cro" => { self.event_name = "Pass".to_string(); self.event_type = Some("Out of Play".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- throw
            "tis" => { self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tii" => { self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tic" => { self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tio" => { self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- corner kick
            "cks" => { self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cki" => { self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "ckc" => { self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cko" => { self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "ckg" => { self.event_name = "Shot".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- direct free kick
            "fkon" => { self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("On Target".to_string()); },
            "fkof" => { self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Off Target".to_string()); },
            "fkb" => { self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Blocked".to_string()); },
            "fkg" => { self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Goal".to_string()); }
            // --- indirect free kick
            "fkps" => { self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpi" => { self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpo" => { self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpc" => { self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- recovery
            "r" => { self.event_name = "Recovery".to_string(); },
            // --- lost ball
            "lb" => { self.event_name = "Lost Ball".to_string(); },
            // --- foul
            "fl" => { self.event_name = "Fouled".to_string(); },
            "fw" => { self.event_name = "Foul Won".to_string(); },
            // --- tackle
            "tw" => { self.event_name = "Tackle".to_string(); self.outcome = Some("Won".to_string()); },
            "tl" => { self.event_name = "Tackle".to_string(); self.outcome = Some("Lost".to_string()); },
            // --- duel
            "daw" => { self.event_name = "Duel".to_string(); self.event_type = Some("Aerial".to_string()); self.outcome = Some("Won".to_string()); },
            "dal" => { self.event_name = "Duel".to_string(); self.event_type = Some("Aerial".to_string()); self.outcome = Some("Lose".to_string()); },
            "dgw" => { self.event_name = "Duel".to_string(); self.event_type = Some("Ground".to_string()); self.outcome = Some("Won".to_string()); },
            "dgl" => { self.event_name = "Duel".to_string(); self.event_type = Some("Ground".to_string()); self.outcome = Some("Lose".to_string()); },
            // --- intercept
            "iop" => { self.event_name = "Intercept".to_string(); self.event_source = Some("Open Play".to_string()); },
            "ifk" => { self.event_name = "Intercept".to_string(); self.event_source = Some("Freekick".to_string()); },
            "ick" => { self.event_name = "Intercept".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "iti" => { self.event_name = "Intercept".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- clearance
            "clrop" => { self.event_name = "Clearance".to_string(); self.event_source = Some("Open Play".to_string()); },
            "clrfk" => { self.event_name = "Clearance".to_string(); self.event_source = Some("Freekick".to_string()); },
            "clrck" => { self.event_name = "Clearance".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "clrti" => { self.event_name = "Clearance".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- block
            "bs" => { self.event_name = "Block".to_string(); self.event_source = Some("Shot".to_string()); },
            "bp" => { self.event_name = "Block".to_string(); self.event_source = Some("Pass".to_string()); },
            "bcr" => { self.event_name = "Block".to_string(); self.event_source = Some("Crossing".to_string()); },
            // --- pressure
            "pr" => { self.event_name = "Pressure".to_string(); },
            // --- goalkeeper save
            "svs" => { self.event_name = "Save".to_string(); self.event_source = Some("Shot".to_string()); },
            "svp" => { self.event_name = "Save".to_string(); self.event_source = Some("Penalty".to_string()); },
            "svfk" => { self.event_name = "Save".to_string(); self.event_source = Some("Freekick".to_string()); },
            "svck" => { self.event_name = "Save".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "svti" => { self.event_name = "Save".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- goalkeeper claim
            "gccr" => { self.event_name = "Catch".to_string(); self.event_source = Some("Crossing".to_string()); },
            "gcp" => { self.event_name = "Catch".to_string(); self.event_source = Some("Pass".to_string()); },
            "gcfk" => { self.event_name = "Catch".to_string(); self.event_source = Some("Freekick".to_string()); },
            "gcck" => { self.event_name = "Catch".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "gcti" => { self.event_name = "Catch".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- time marker
            "kofh" => { self.event_name = "Kick Off First Half".to_string(); },
            "kosh" => { self.event_name = "Kick Off Second Half".to_string(); },
            "koefh" => { self.event_name = "Kick Off Extra Time First Half".to_string(); },
            "koesh" => { self.event_name = "Kick Off Extra Time Second Half".to_string(); },
            "eofh" => { self.event_name = "End of First Half".to_string(); },
            "eosh" => { self.event_name = "End of Second Half".to_string(); },
            "eoefh" => { self.event_name = "End of Extra Time First Half".to_string(); },
            "eoesh" => { self.event_name = "End of Extra Time Second Half".to_string(); },
            "eom" => { self.event_name = "End of Match".to_string(); },
            // --- play
            "startgk" => { self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("GK".to_string()); },
            "startrfb" => { self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("RFB".to_string()); },
            "startlfb" => { self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("LFB".to_string()); },
            "startcb" => { self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("CB".to_string()); },
            "startmf" => { self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("MF".to_string()); },
            "startrwg" => { self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("RWG".to_string()); },
            "startlwg" => { self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("LWG".to_string()); },
            "startcf" => { self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("CF".to_string()); },
            // --- change position
            "changegk" => { self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("GK".to_string()); },
            "changerfb" => { self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("RFB".to_string()); },
            "changelfb" => { self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("LFB".to_string()); },
            "changecb" => { self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("CB".to_string()); },
            "changemf" => { self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("MF".to_string()); },
            "changerwg" => { self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("RWG".to_string()); },
            "changelwf" => { self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("LWG".to_string()); },
            "changecf" => { self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("CF".to_string()); },
            // --- subs
            "so" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs Out".to_string()); },
            "sigk" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("GK".to_string()); },
            "sirfb" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("RFB".to_string()); },
            "silfb" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("LFB".to_string()); },
            "sicb" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("CB".to_string()); },
            "simf" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("MF".to_string()); },
            "sirwg" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("RWG".to_string()); },
            "silwg" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("LWG".to_string()); },
            "sicf" => { self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("CF".to_string()); },
            // --- other events
            "yc" => { self.event_name = "Yellow Card".to_string(); },
            "syc" => { self.event_name = "Second Yellow Card".to_string(); },
            "rc" => { self.event_name = "Red Card".to_string(); },
            _ => { self.event_name = "Unregistered".to_string(); },
        }
    }
}
