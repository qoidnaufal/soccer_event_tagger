use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventOutcome {
    Other { name: String },
    PassOutcome { outcome: String, team: Option<String>, player: Option<String> }
}

impl Default for EventOutcome {
    fn default() -> Self {
        Self::Other {
            name: "Unregistered".to_string(),
        }
    }
}

impl std::fmt::Display for EventOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[rustfmt::skip]
impl EventOutcome {
    fn from_args(event_args: &str, team_outcome: Option<String>, player_outcome: Option<String>) -> Self {
        match event_args {
            // --- goal kick
            "gks" => Self::PassOutcome { outcome: "Success".to_string(), team: team_outcome, player: player_outcome, },
            "gki" => Self::PassOutcome { outcome: "Intercepted".to_string(), team: team_outcome, player: player_outcome, },
            "gko" => Self::PassOutcome { outcome: "Out of Play".to_string(), team: team_outcome, player: player_outcome, },
            "gkc" => Self::PassOutcome { outcome: "Catched".to_string(), team: team_outcome, player: player_outcome, },
            // --- pass
            "ps" => Self::PassOutcome { outcome: "Success".to_string(), team: team_outcome, player: player_outcome, },
            "pi" => Self::PassOutcome { outcome: "Intercepted".to_string(), team: team_outcome, player: player_outcome, },
            "pb" => Self::PassOutcome { outcome: "Blocked".to_string(), team: team_outcome, player: player_outcome, },
            "po" => Self::PassOutcome { outcome: "Out of Play".to_string(), team: team_outcome, player: player_outcome, },
            "pc" => Self::PassOutcome { outcome: "Catched".to_string(), team: team_outcome, player: player_outcome, },
            // --- crossing
            "crs" => Self::PassOutcome { outcome: "Success".to_string(), team: team_outcome, player: player_outcome, },
            "cri" => Self::PassOutcome { outcome: "Intercepted".to_string(), team: team_outcome, player: player_outcome, },
            "crc" => Self::PassOutcome { outcome: "Catched".to_string(), team: team_outcome, player: player_outcome, },
            "crb" => Self::PassOutcome { outcome: "Blocked".to_string(), team: team_outcome, player: player_outcome, },
            "cro" => Self::PassOutcome { outcome: "Out of Play".to_string(), team: team_outcome, player: player_outcome, },
            // --- throw
            "tis" => Self::PassOutcome { outcome: "Success".to_string(), team: team_outcome, player: player_outcome, },
            "tii" => Self::PassOutcome { outcome: "Intercepted".to_string(), team: team_outcome, player: player_outcome, },
            "tic" => Self::PassOutcome { outcome: "Catched".to_string(), team: team_outcome, player: player_outcome, },
            "tio" => Self::PassOutcome { outcome: "Out of Play".to_string(), team: team_outcome, player: player_outcome, },
            // --- corner kick
            "cks" => Self::PassOutcome { outcome: "Success".to_string(), team: team_outcome, player: player_outcome, },
            "cki" => Self::PassOutcome { outcome: "Intercepted".to_string(), team: team_outcome, player: player_outcome, },
            "ckc" => Self::PassOutcome { outcome: "Catched".to_string(), team: team_outcome, player: player_outcome, },
            "cko" => Self::PassOutcome { outcome: "Out of Play".to_string(), team: team_outcome, player: player_outcome, },
            // --- free kick
            "fkps" => Self::PassOutcome { outcome: "Success".to_string(), team: team_outcome, player: player_outcome, },
            "fkpi" => Self::PassOutcome { outcome: "Intercepted".to_string(), team: team_outcome, player: player_outcome, },
            "fkpo" => Self::PassOutcome { outcome: "Out of Play".to_string(), team: team_outcome, player: player_outcome, },
            "fkpc" => Self::PassOutcome { outcome: "Catched".to_string(), team: team_outcome, player: player_outcome, },
            // --- penalty kick
            "pkps" => Self::PassOutcome { outcome: "Success".to_string(), team: team_outcome, player: player_outcome, },
            "pkpi" => Self::PassOutcome { outcome: "Intercepted".to_string(), team: team_outcome, player: player_outcome, },
            "pkpo" => Self::PassOutcome { outcome: "Out of Play".to_string(), team: team_outcome, player: player_outcome, },
            "pkpc" => Self::PassOutcome { outcome: "Catched".to_string(), team: team_outcome, player: player_outcome, },
            // --- unregistered
            _ => Self::Other { name: "Unregistered".to_string(), },
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Other { name: String, },
    Shot { name: String, shot_type: String, outcome: String, },
    Pass { name: String, pass_type: String, outcome: EventOutcome, },
    Dribble { name: String, outcome: String, },
    LostBall { name: String, },
    Recovery { name: String, },
    Foul { name: String, },
    Tackle { name: String, outcome: String, },
    Intercept { name: String, event_source: String, },
    Duel { name: String, duel_type: String, outcome: String, },
    Clearance { name: String, event_source: String, },
    Block { name: String, event_source: String, },
    Pressure { name: String, },
    Save { name: String, event_source: String, },
    Catch { name: String, event_source: String, },
    Play { name: String, position: String, },
    Subs { name: String, team: Option<String>, subs_in: Option<String> },
    TimeMarker { name: String, }
}

impl Default for Event {
    fn default() -> Self {
        Self::Other {
            name: "unregistered".to_string(),
        }
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Event {
    #[rustfmt::skip]
    pub fn from_event_args(event_args: &str, team_args: Option<String>, player_args: Option<String>) -> Self {
        match event_args {
            // --- pass
            "ps" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "pi" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "pb" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "po" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "pc" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            // --- goal kick
            "gks" => Self::Pass { name: "Pass".to_string(), pass_type: "Goal Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "gki" => Self::Pass { name: "Pass".to_string(), pass_type: "Goal Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "gko" => Self::Pass { name: "Pass".to_string(), pass_type: "Goal Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "gkc" => Self::Pass { name: "Pass".to_string(), pass_type: "Goal Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            // --- shot
            "son" => Self::Shot { name: "Shot".to_string(), shot_type: "Open Play".to_string(), outcome: "On target".to_string() },
            "sof" => Self::Shot { name: "Shot".to_string(), shot_type: "Open Play".to_string(), outcome: "Off target".to_string() },
            "sb" => Self::Shot { name: "Shot".to_string(), shot_type: "Open Play".to_string(), outcome: "Blocked".to_string() },
            "sg" => Self::Shot { name: "Shot".to_string(), shot_type: "Open Play".to_string(), outcome: "Goal".to_string() },
            // --- penalty kick pass
            "pkps" => Self::Pass { name: "Pass".to_string(), pass_type: "Penalty Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "pkpi" => Self::Pass { name: "Pass".to_string(), pass_type: "Penalty Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "pkpo" => Self::Pass { name: "Pass".to_string(), pass_type: "Penalty Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "pkpc" => Self::Pass { name: "Pass".to_string(), pass_type: "Penalty Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            // --- penalty kick shot
            "pkon" => Self::Shot { name: "Shot".to_string(), shot_type: "Penalty Kick".to_string(), outcome: "On target".to_string() },
            "pkof" => Self::Shot { name: "Shot".to_string(), shot_type: "Penalty Kick".to_string(), outcome: "Off target".to_string() },
            "pkg" => Self::Shot { name: "Shot".to_string(), shot_type: "Penalty Kick".to_string(), outcome: "Goal".to_string() },
            // --- carry
            "drp" => Self::Dribble { name: "Dribble".to_string(), outcome: "Pass".to_string() },
            "drs" => Self::Dribble { name: "Dribble".to_string(), outcome: "Shot".to_string() },
            "drlb" => Self::Dribble { name: "Dribble".to_string(), outcome: "Lost ball".to_string() },
            "drfw" => Self::Dribble { name: "Dribble".to_string(), outcome: "Foul Won".to_string() },
            // --- crossing
            "crs" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "cri" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "crc" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "crb" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "cro" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            // --- throw
            "tis" => Self::Pass { name: "Pass".to_string(), pass_type: "Throw In".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "tii" => Self::Pass { name: "Pass".to_string(), pass_type: "Throw In".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "tic" => Self::Pass { name: "Pass".to_string(), pass_type: "Throw In".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "tio" => Self::Pass { name: "Pass".to_string(), pass_type: "Throw In".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            // --- corner kick
            "cks" => Self::Pass { name: "Pass".to_string(), pass_type: "Corner Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "cki" => Self::Pass { name: "Pass".to_string(), pass_type: "Corner Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "ckc" => Self::Pass { name: "Pass".to_string(), pass_type: "Corner Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "cko" => Self::Pass { name: "Pass".to_string(), pass_type: "Corner Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "ckg" => Self::Shot { name: "Shot".to_string(), shot_type: "Corner Kick".to_string(), outcome: "Goal".to_string() },
            // --- direct free kick
            "fkon" => Self::Shot { name: "Shot".to_string(), shot_type: "Free Kick".to_string(), outcome: "On Target".to_string() },
            "fkof" => Self::Shot { name: "Shot".to_string(), shot_type: "Free Kick".to_string(), outcome: "Off Target".to_string() },
            "fkb" => Self::Shot { name: "Shot".to_string(), shot_type: "Free Kick".to_string(), outcome: "Blocked".to_string() },
            // --- indirect free kick
            "fkps" => Self::Pass { name: "Pass".to_string(), pass_type: "Free Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "fkpi" => Self::Pass { name: "Pass".to_string(), pass_type: "Free Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "fkpo" => Self::Pass { name: "Pass".to_string(), pass_type: "Free Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            "fkpc" => Self::Pass { name: "Pass".to_string(), pass_type: "Free Kick".to_string(), outcome: EventOutcome::from_args(event_args, team_args, player_args) },
            // --- recovery
            "r" => Self::Recovery { name: "Recovery".to_string() },
            // --- lost ball
            "lb" => Self::LostBall { name: "Lost Ball".to_string() },
            // --- foul
            "fl" => Self::Foul { name: "Fouled".to_string() },
            "fw" => Self::Foul { name: "Foul Won".to_string() },
            // --- tackle
            "tw" => Self::Tackle { name: "Tackle".to_string(), outcome: "Won".to_string() },
            "tl" => Self::Tackle { name: "Tackle".to_string(), outcome: "Lost".to_string() },
            // --- duel
            "daw" => Self::Duel { name: "Duel".to_string(), duel_type: "Aerial".to_string(), outcome: "Won".to_string() },
            "dal" => Self::Duel { name: "Duel".to_string(), duel_type: "Aerial".to_string(), outcome: "Lose".to_string() },
            "dgw" => Self::Duel { name: "Duel".to_string(), duel_type: "Ground".to_string(), outcome: "Won".to_string() },
            "dgl" => Self::Duel { name: "Duel".to_string(), duel_type: "Ground".to_string(), outcome: "Lose".to_string() },
            // --- intercept
            "iop" => Self::Intercept { name: "Intercept".to_string(), event_source: "Open Play".to_string() },
            "ifk" => Self::Intercept { name: "Intercept".to_string(), event_source: "Freekick".to_string() },
            "ick" => Self::Intercept { name: "Intercept".to_string(), event_source: "Cornerkick".to_string() },
            "iti" => Self::Intercept { name: "Intercept".to_string(), event_source: "Throw In".to_string() },
            // --- clearance
            "clrop" => Self::Clearance { name: "Clearance".to_string(), event_source: "Open Play".to_string() },
            "clrfk" => Self::Clearance { name: "Clearance".to_string(), event_source: "Freekick".to_string() },
            "clrck" => Self::Clearance { name: "Clearance".to_string(), event_source: "Cornerkick".to_string() },
            "clrti" => Self::Clearance { name: "Clearance".to_string(), event_source: "Throw In".to_string() },
            // --- block
            "bs" => Self::Block { name: "Block".to_string(), event_source: "Shot".to_string() },
            "bp" => Self::Block { name: "Block".to_string(), event_source: "Pass".to_string() },
            "bcr" => Self::Block { name: "Block".to_string(), event_source: "Crossing".to_string() },
            // --- pressure
            "prop" => Self::Pressure { name: "Pressure".to_string() },
            // --- goalkeeper save
            "svs" => Self::Save { name: "Save".to_string(), event_source: "Shot".to_string() },
            "svp" => Self::Save { name: "Save".to_string(), event_source: "Penalty".to_string() },
            "svfk" => Self::Save { name: "Save".to_string(), event_source: "Freekick".to_string() },
            "svck" => Self::Save { name: "Save".to_string(), event_source: "Cornerkick".to_string() },
            "svti" => Self::Save { name: "Save".to_string(), event_source: "Throw In".to_string() },
            // --- goalkeeper claim
            "gccr" => Self::Catch { name: "Catch".to_string(), event_source: "Crossing".to_string() },
            "gcp" => Self::Catch { name: "Catch".to_string(), event_source: "Pass".to_string() },
            "gcfk" => Self::Catch { name: "Catch".to_string(), event_source: "Freekick".to_string() },
            "gcck" => Self::Catch { name: "Catch".to_string(), event_source: "Cornerkick".to_string() },
            "gcti" => Self::Catch { name: "Catch".to_string(), event_source: "Throw In".to_string() },
            // --- time marker
            "kofh" => Self::TimeMarker { name: "Kick Off First Half".to_string() },
            "kosh" => Self::TimeMarker { name: "Kick Off Second Half".to_string() },
            "koefh" => Self::TimeMarker { name: "Kick Off Extra Time First Half".to_string() },
            "koesh" => Self::TimeMarker { name: "Kick Off Extra Time Second Half".to_string() },
            "eofh" => Self::TimeMarker { name: "End of First Half".to_string() },
            "eosh" => Self::TimeMarker { name: "End of Second Half".to_string() },
            "eoefh" => Self::TimeMarker { name: "End of Extra Time First Half".to_string() },
            "eoesh" => Self::TimeMarker { name: "End of Extra Time Second Half".to_string() },
            "eom" => Self::TimeMarker { name: "End of Match".to_string() },
            // --- play
            "startgk" => Self::Play { name: "Start".to_string(), position: "GK".to_string() },
            "startrfb" => Self::Play { name: "Start".to_string(), position: "RFB".to_string() },
            "startlfb" => Self::Play { name: "Start".to_string(), position: "LFB".to_string() },
            "startcb" => Self::Play { name: "Start".to_string(), position: "CB".to_string() },
            "startmf" => Self::Play { name: "Start".to_string(), position: "MF".to_string() },
            "startrwg" => Self::Play { name: "Start".to_string(), position: "RWG".to_string() },
            "startlwg" => Self::Play { name: "Start".to_string(), position: "LWG".to_string() },
            "startcf" => Self::Play { name: "Start".to_string(), position: "CF".to_string() },
            // --- change position
            "changegk" => Self::Play { name: "Change Position".to_string(), position: "GK".to_string() },
            "changerfb" => Self::Play { name: "Change Position".to_string(), position: "RFB".to_string() },
            "changelfb" => Self::Play { name: "Change Position".to_string(), position: "LFB".to_string() },
            "changecb" => Self::Play { name: "Change Position".to_string(), position: "CB".to_string() },
            "changemf" => Self::Play { name: "Change Position".to_string(), position: "MF".to_string() },
            "changerwg" => Self::Play { name: "Change Position".to_string(), position: "RWG".to_string() },
            "changelwf" => Self::Play { name: "Change Position".to_string(), position: "LWG".to_string() },
            "changecf" => Self::Play { name: "Change Position".to_string(), position: "CF".to_string() },
            // --- subs
            "subs" => Self::Subs { name: "Subs".to_string(), team: team_args, subs_in: player_args },
            // --- other events
            "yc" => Self::Other { name: "Yellow Card".to_string() },
            "syc" => Self::Other { name: "Second Yellow Card".to_string() },
            "rc" => Self::Other { name: "Red Card".to_string() },
            _ => Self::Other { name: "Unregistered".to_string(),
            },
        }
    }
}
