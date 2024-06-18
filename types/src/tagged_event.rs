use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaggedEvent {
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
    #[rustfmt::skip]
    pub fn assign_event_from_args(&mut self, event_args: &str, team_args: Option<String>, player_args: Option<String>) {
        match event_args {
            // --- pass
            "ps" => { self.event_id = 37; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pi" => { self.event_id = 38; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pb" => { self.event_id = 39; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Blocked".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "po" => { self.event_id = 40; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pc" => { self.event_id = 41; self.event_name = "Pass".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- goalkick
            "gks" => { self.event_id = 42; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gki" => { self.event_id = 43; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gko" => { self.event_id = 44; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "gkc" => { self.event_id = 45; self.event_name = "Pass".to_string(); self.event_type = Some("Goal Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- shot
            "son" => { self.event_id = 46; self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("On Target".to_string()); },
            "sof" => { self.event_id = 47; self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Off Target".to_string()); },
            "sb" => { self.event_id = 48; self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Blocked".to_string()); },
            "sg" => { self.event_id = 49; self.event_name = "Shot".to_string(); self.event_type = Some("Open Play".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- penalty kick pass
            "pkps" => { self.event_id = 50; self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpi" => { self.event_id = 51; self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpo" => { self.event_id = 52; self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "pkpc" => { self.event_id = 53; self.event_name = "Pass".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- penalty kick shot
            "pkon" => { self.event_id = 54; self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("On Target".to_string()); },
            "pkof" => { self.event_id = 55; self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Off Target".to_string()); },
            "pkg" => { self.event_id = 56; self.event_name = "Shot".to_string(); self.event_type = Some("Penalty Kick".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- carry
            "drp" => { self.event_id = 57; self.event_name = "Dribble".to_string(); self.outcome = Some("Pass".to_string()); },
            "drs" => { self.event_id = 58; self.event_name = "Dribble".to_string(); self.outcome = Some("Shot".to_string()); },
            "drlb" => { self.event_id = 59; self.event_name = "Dribble".to_string(); self.outcome = Some("Lost Ball".to_string()); },
            "drfw" => { self.event_id = 60; self.event_name = "Dribble".to_string(); self.outcome = Some("Foul Won".to_string()); },
            // --- crossing
            "crs" => { self.event_id = 61; self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cri" => { self.event_id = 62; self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "crc" => { self.event_id = 63; self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "crb" => { self.event_id = 64; self.event_name = "Pass".to_string(); self.event_type = Some("Crossing".to_string()); self.outcome = Some("Blocked".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cro" => { self.event_id = 65; self.event_name = "Pass".to_string(); self.event_type = Some("Out of Play".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- throw
            "tis" => { self.event_id = 66; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tii" => { self.event_id = 67; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tic" => { self.event_id = 68; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "tio" => { self.event_id = 69; self.event_name = "Pass".to_string(); self.event_type = Some("Throw In".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- corner kick
            "cks" => { self.event_id = 70; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cki" => { self.event_id = 71; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "ckc" => { self.event_id = 72; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "cko" => { self.event_id = 73; self.event_name = "Pass".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "ckg" => { self.event_id = 74; self.event_name = "Shot".to_string(); self.event_type = Some("Corner Kick".to_string()); self.outcome = Some("Goal".to_string()); },
            // --- direct free kick
            "fkon" => { self.event_id = 75; self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("On Target".to_string()); },
            "fkof" => { self.event_id = 76; self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Off Target".to_string()); },
            "fkb" => { self.event_id = 77; self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Blocked".to_string()); },
            "fkg" => { self.event_id = 78; self.event_name = "Shot".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Goal".to_string()); }
            // --- indirect free kick
            "fkps" => { self.event_id = 79; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Success".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpi" => { self.event_id = 80; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Intercepted".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpo" => { self.event_id = 81; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Out of Play".to_string()); self.team_end = team_args; self.player_end = player_args; },
            "fkpc" => { self.event_id = 82; self.event_name = "Pass".to_string(); self.event_type = Some("Free Kick".to_string()); self.outcome = Some("Catched".to_string()); self.team_end = team_args; self.player_end = player_args; },
            // --- recovery
            "r" => { self.event_id = 83; self.event_name = "Recovery".to_string(); },
            // --- lost ball
            "lb" => { self.event_id = 84; self.event_name = "Lost Ball".to_string(); },
            // --- foul
            "fl" => { self.event_id = 85; self.event_name = "Fouled".to_string(); },
            "fw" => { self.event_id = 86; self.event_name = "Foul Won".to_string(); },
            // --- tackle
            "tw" => { self.event_id = 87; self.event_name = "Tackle".to_string(); self.outcome = Some("Won".to_string()); },
            "tl" => { self.event_id = 88; self.event_name = "Tackle".to_string(); self.outcome = Some("Lost".to_string()); },
            // --- duel
            "daw" => { self.event_id = 89; self.event_name = "Duel".to_string(); self.event_type = Some("Aerial".to_string()); self.outcome = Some("Won".to_string()); },
            "dal" => { self.event_id = 90; self.event_name = "Duel".to_string(); self.event_type = Some("Aerial".to_string()); self.outcome = Some("Lose".to_string()); },
            "dgw" => { self.event_id = 91; self.event_name = "Duel".to_string(); self.event_type = Some("Ground".to_string()); self.outcome = Some("Won".to_string()); },
            "dgl" => { self.event_id = 92; self.event_name = "Duel".to_string(); self.event_type = Some("Ground".to_string()); self.outcome = Some("Lose".to_string()); },
            // --- intercept
            "iop" => { self.event_id = 93; self.event_name = "Intercept".to_string(); self.event_source = Some("Open Play".to_string()); },
            "ifk" => { self.event_id = 94; self.event_name = "Intercept".to_string(); self.event_source = Some("Freekick".to_string()); },
            "ick" => { self.event_id = 95; self.event_name = "Intercept".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "iti" => { self.event_id = 96; self.event_name = "Intercept".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- clearance
            "clrop" => { self.event_id = 97; self.event_name = "Clearance".to_string(); self.event_source = Some("Open Play".to_string()); },
            "clrfk" => { self.event_id = 98; self.event_name = "Clearance".to_string(); self.event_source = Some("Freekick".to_string()); },
            "clrck" => { self.event_id = 99; self.event_name = "Clearance".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "clrti" => { self.event_id = 100; self.event_name = "Clearance".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- block
            "bs" => { self.event_id = 101; self.event_name = "Block".to_string(); self.event_source = Some("Shot".to_string()); },
            "bp" => { self.event_id = 102; self.event_name = "Block".to_string(); self.event_source = Some("Pass".to_string()); },
            "bcr" => { self.event_id = 103; self.event_name = "Block".to_string(); self.event_source = Some("Crossing".to_string()); },
            // --- pressure
            "pr" => { self.event_id = 104; self.event_name = "Pressure".to_string(); },
            // --- goalkeeper save
            "svs" => { self.event_id = 105; self.event_name = "Save".to_string(); self.event_source = Some("Shot".to_string()); },
            "svp" => { self.event_id = 106; self.event_name = "Save".to_string(); self.event_source = Some("Penalty".to_string()); },
            "svfk" => { self.event_id = 107; self.event_name = "Save".to_string(); self.event_source = Some("Freekick".to_string()); },
            "svck" => { self.event_id = 108; self.event_name = "Save".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "svti" => { self.event_id = 109; self.event_name = "Save".to_string(); self.event_source = Some("Throw In".to_string()); },
            // --- goalkeeper claim
            "gccr" => { self.event_id = 110; self.event_name = "Catch".to_string(); self.event_source = Some("Crossing".to_string()); },
            "gcp" => { self.event_id = 111; self.event_name = "Catch".to_string(); self.event_source = Some("Pass".to_string()); },
            "gcfk" => { self.event_id = 112; self.event_name = "Catch".to_string(); self.event_source = Some("Freekick".to_string()); },
            "gcck" => { self.event_id = 113; self.event_name = "Catch".to_string(); self.event_source = Some("Cornerkick".to_string()); },
            "gcti" => { self.event_id = 114; self.event_name = "Catch".to_string(); self.event_source = Some("Throw In".to_string()); },
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
            "startgk" => { self.event_id = 9; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("GK".to_string()); },
            "startrfb" => { self.event_id = 10; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("RFB".to_string()); },
            "startlfb" => { self.event_id = 11; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("LFB".to_string()); },
            "startcb" => { self.event_id = 12; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("CB".to_string()); },
            "startmf" => { self.event_id = 13; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("MF".to_string()); },
            "startrwg" => { self.event_id = 14; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("RWG".to_string()); },
            "startlwg" => { self.event_id = 15; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("LWG".to_string()); },
            "startcf" => { self.event_id = 16; self.event_name = "Play".to_string(); self.event_type = Some("Start".to_string()); self.play_position = Some("CF".to_string()); },
            // --- change position
            "changegk" => { self.event_id = 17; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("GK".to_string()); },
            "changerfb" => { self.event_id = 18; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("RFB".to_string()); },
            "changelfb" => { self.event_id = 19; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("LFB".to_string()); },
            "changecb" => { self.event_id = 20; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("CB".to_string()); },
            "changemf" => { self.event_id = 21; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("MF".to_string()); },
            "changerwg" => { self.event_id = 22; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("RWG".to_string()); },
            "changelwf" => { self.event_id = 23; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("LWG".to_string()); },
            "changecf" => { self.event_id = 24; self.event_name = "Play".to_string(); self.event_type = Some("Change Position".to_string()); self.play_position = Some("CF".to_string()); },
            // --- subs
            "so" => { self.event_id = 25; self.event_name = "Play".to_string(); self.event_type = Some("Subs Out".to_string()); },
            "sigk" => { self.event_id = 26; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("GK".to_string()); },
            "sirfb" => { self.event_id = 27; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("RFB".to_string()); },
            "silfb" => { self.event_id = 28; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("LFB".to_string()); },
            "sicb" => { self.event_id = 29; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("CB".to_string()); },
            "simf" => { self.event_id = 30; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("MF".to_string()); },
            "sirwg" => { self.event_id = 31; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("RWG".to_string()); },
            "silwg" => { self.event_id = 32; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("LWG".to_string()); },
            "sicf" => { self.event_id = 33; self.event_name = "Play".to_string(); self.event_type = Some("Subs In".to_string()); self.play_position = Some("CF".to_string()); },
            // --- other events
            "yc" => { self.event_id = 34; self.event_name = "Yellow Card".to_string(); },
            "syc" => { self.event_id = 35; self.event_name = "Second Yellow Card".to_string(); },
            "rc" => { self.event_id = 36; self.event_name = "Red Card".to_string(); },
            _ => { self.event_id = 115; self.event_name = "Unregistered".to_string(); },
        }
    }
}
