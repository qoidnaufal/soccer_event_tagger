use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaggedEvent {
    pub uuid: String,
    pub match_id: String,
    pub match_teams: String,
    pub opponent_team: String,
    pub event_id: i32,
    pub time_start: f64,
    pub x_start: Option<i32>,
    pub y_start: Option<i32>,
    pub time_end: f64,
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
        self.event_id == other.event_id
    }
}

impl Eq for TaggedEvent {}

impl Ord for TaggedEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.event_id.cmp(&other.event_id)
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

    #[rustfmt::skip]
    pub fn assign_event_from_args(&mut self, event_args: &str, team_args: Option<String>, player_args: Option<String>) {
        match event_args {
            // --- pass
            "ps" => { self.event_id = 50; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pi" => { self.event_id = 51; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pb" => { self.event_id = 52; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Blocked".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "po" => { self.event_id = 53; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pc" => { self.event_id = 54; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pp" => { self.event_id = 55; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Punched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- goalkick
            "gks" => { self.event_id = 60; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gki" => { self.event_id = 61; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gko" => { self.event_id = 62; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gkc" => { self.event_id = 63; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gkp" => { self.event_id = 64; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Punched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- shot
            "son" => { self.event_id = 70; self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("On Target".to_string()); },
            "sof" => { self.event_id = 71; self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Off Target".to_string()); },
            "sb" => { self.event_id = 72; self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Blocked".to_string()); },
            "sg" => { self.event_id = 73; self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- penalty kick pass
            "pkps" => { self.event_id = 80; self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpi" => { self.event_id = 81; self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpo" => { self.event_id = 82; self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpc" => { self.event_id = 83; self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- penalty kick shot
            "pkon" => { self.event_id = 90; self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("On Target".to_string()); },
            "pkof" => { self.event_id = 91; self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Off Target".to_string()); },
            "pkg" => { self.event_id = 92; self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- carry
            "drp" => { self.event_id = 100; self.event_name = "Dribble".to_string(); self.outcome = Some("Pass".to_string()); },
            "drs" => { self.event_id = 101; self.event_name = "Dribble".to_string(); self.outcome = Some("Shot".to_string()); },
            "drlb" => { self.event_id = 102; self.event_name = "Dribble".to_string(); self.outcome = Some("Lost Ball".to_string()); },
            "drfw" => { self.event_id = 103; self.event_name = "Dribble".to_string(); self.outcome = Some("Foul Won".to_string()); },
            // --- crossing
            "crs" => { self.event_id = 110; self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cri" => { self.event_id = 111; self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "crc" => { self.event_id = 112; self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "crb" => { self.event_id = 113; self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Blocked".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cro" => { self.event_id = 114; self.event_name = "Pass".to_string(); self.event_type = Some("Out of Play".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "crp" => { self.event_id = 115; self.event_name = "Pass".to_string(); self.event_type = Some("Punched".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- throw
            "tis" => { self.event_id = 120; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tii" => { self.event_id = 121; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tic" => { self.event_id = 122; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tio" => { self.event_id = 123; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tip" => { self.event_id = 124; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Punched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- corner kick
            "cks" => { self.event_id = 130; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cki" => { self.event_id = 131; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "ckc" => { self.event_id = 132; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cko" => { self.event_id = 133; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "ckp" => { self.event_id = 134; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Punched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "ckg" => { self.event_id = 135; self.event_name = "Shot".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- direct free kick
            "fkon" => { self.event_id = 140; self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("On Target".to_string()); },
            "fkof" => { self.event_id = 141; self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Off Target".to_string()); },
            "fkb" => { self.event_id = 142; self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Blocked".to_string()); },
            "fkg" => { self.event_id = 143; self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Goal".to_string()); }
            // --- indirect free kick
            "fkps" => { self.event_id = 150; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpi" => { self.event_id = 151; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpo" => { self.event_id = 152; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpc" => { self.event_id = 153; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpp" => { self.event_id = 154; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Punched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- recovery
            "r" => { self.event_id = 160; self.event_name = "Recovery".to_string(); },
            // --- lost ball
            "lb" => { self.event_id = 170; self.event_name = "Lost Ball".to_string(); },
            // --- foul
            "fl" => { self.event_id = 180; self.event_name = "Fouled".to_string(); },
            "fw" => { self.event_id = 181; self.event_name = "Foul Won".to_string(); },
            // --- tackle
            "tw" => { self.event_id = 190; self.event_name = "Tackle".to_string(); self.outcome = Some("Won".to_string()); },
            "tl" => { self.event_id = 191; self.event_name = "Tackle".to_string(); self.outcome = Some("Lost".to_string()); },
            // --- duel
            "daw" => { self.event_id = 200; self.event_name = "Duel".to_string(); self.event_type = Some("Aerial".to_string()); self.outcome = Some("Won".to_string()); },
            "dal" => { self.event_id = 201; self.event_name = "Duel".to_string(); self.event_type = Some("Aerial".to_string()); self.outcome = Some("Lose".to_string()); },
            "dgw" => { self.event_id = 202; self.event_name = "Duel".to_string(); self.event_type = Some("Ground".to_string()); self.outcome = Some("Won".to_string()); },
            "dgl" => { self.event_id = 203; self.event_name = "Duel".to_string(); self.event_type = Some("Ground".to_string()); self.outcome = Some("Lose".to_string()); },
            // --- intercept
            "iop" => { self.event_id = 210; self.event_name = "Intercept".to_string(); self.event_source = Some("Open Play".to_string()); },
            "ifk" => { self.event_id = 211; self.event_name = "Intercept".to_string(); self.event_source = Some("Freekick".to_string()); },
            "ick" => { self.event_id = 212; self.event_name = "Intercept".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "iti" => { self.event_id = 213; self.event_name = "Intercept".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- clearance
            "clrop" => { self.event_id = 220; self.event_name = "Clearance".to_string(); self.event_source = Some("Open Play".to_string()); },
            "clrfk" => { self.event_id = 221; self.event_name = "Clearance".to_string(); self.event_source = Some("Freekick".to_string()); },
            "clrck" => { self.event_id = 222; self.event_name = "Clearance".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "clrti" => { self.event_id = 223; self.event_name = "Clearance".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- block
            "bs" => { self.event_id = 230; self.event_name = "Block".to_string(); self.event_source = Some("Shot".to_string()); },
            "bp" => { self.event_id = 231; self.event_name = "Block".to_string(); self.event_source = Some("Pass".to_string()); },
            "bcr" => { self.event_id = 232; self.event_name = "Block".to_string(); self.event_source = Some("Crossing".to_string()); },
            // --- pressure
            "pr" => { self.event_id = 240; self.event_name = "Pressure".to_string(); },
            // --- goalkeeper save
            "svs" => { self.event_id = 250; self.event_name = "Save".to_string(); self.event_source = Some("Shot".to_string()); },
            "svpk" => { self.event_id = 251; self.event_name = "Save".to_string(); self.event_source = Some("Penalty".to_string()); },
            "svfk" => { self.event_id = 252; self.event_name = "Save".to_string(); self.event_source = Some("Freekick".to_string()); },
            "svck" => { self.event_id = 253; self.event_name = "Save".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            // --- goalkeeper claim
            "gccr" => { self.event_id = 260; self.event_name = "Catch".to_string(); self.event_source = Some("Crossing".to_string()); },
            "gcp" => { self.event_id = 261; self.event_name = "Catch".to_string(); self.event_source = Some("Pass".to_string()); },
            "gcfk" => { self.event_id = 262; self.event_name = "Catch".to_string(); self.event_source = Some("Freekick".to_string()); },
            "gcck" => { self.event_id = 263; self.event_name = "Catch".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "gcti" => { self.event_id = 264; self.event_name = "Catch".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- time marker
            "kofh" => { self.event_id = 0; self.event_name = "Kick Off First Half".to_string(); },
            "kosh" => { self.event_id = 1; self.event_name = "Kick Off Second Half".to_string(); },
            "koefh" => { self.event_id = 2; self.event_name = "Kick Off Extra Time First Half".to_string(); },
            "koesh" => { self.event_id = 3; self.event_name = "Kick Off Extra Time Second Half".to_string(); },
            "eofh" => { self.event_id = 4; self.event_name = "End of First Half".to_string(); },
            "eosh" => { self.event_id = 5; self.event_name = "End of Second Half".to_string(); },
            "eoefh" => { self.event_id = 6; self.event_name = "End of Extra Time First Half".to_string(); },
            "eoesh" => { self.event_id = 7; self.event_name = "End of Extra Time Second Half".to_string(); },
            "eom" => { self.event_id = 8; self.event_name = "End of Match".to_string(); },
            // --- play
            "startgk" => { self.event_id = 11; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("GK".to_string()); },
            "startrfb" => { self.event_id = 12; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("RFB".to_string()); },
            "startlfb" => { self.event_id = 15; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("LFB".to_string()); },
            "startcb" => { self.event_id = 13; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("CB".to_string()); },
            "startmf" => { self.event_id = 18; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("MF".to_string()); },
            "startrwg" => { self.event_id = 17; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("RWG".to_string()); },
            "startlwg" => { self.event_id = 14; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("LWG".to_string()); },
            "startcf" => { self.event_id = 19; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("CF".to_string()); },
            // --- change position
            "changegk" => { self.event_id = 21; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("GK".to_string()); },
            "changerfb" => { self.event_id = 22; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("RFB".to_string()); },
            "changelfb" => { self.event_id = 25; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("LFB".to_string()); },
            "changecb" => { self.event_id = 23; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("CB".to_string()); },
            "changemf" => { self.event_id = 28; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("MF".to_string()); },
            "changerwg" => { self.event_id = 27; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("RWG".to_string()); },
            "changelwf" => { self.event_id = 24; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("LWG".to_string()); },
            "changecf" => { self.event_id = 29; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("CF".to_string()); },
            // --- subs
            "sigk" => { self.event_id = 31; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("GK".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "sirfb" => { self.event_id = 32; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("RFB".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "silfb" => { self.event_id = 35; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("LFB".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "sicb" => { self.event_id = 33; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("CB".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "simf" => { self.event_id = 38; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("MF".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "sirwg" => { self.event_id = 37; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("RWG".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "silwg" => { self.event_id = 34; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("LWG".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "sicf" => { self.event_id = 39; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("CF".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- other events
            "yc" => { self.event_id = 40; self.event_name = "Yellow Card".to_string(); },
            "syc" => { self.event_id = 41; self.event_name = "Second Yellow Card".to_string(); },
            "rc" => { self.event_id = 42; self.event_name = "Red Card".to_string(); },
            n => { self.event_id = 43; self.event_name = format!("Unregistered: {}", n); },
        }
    }
}
