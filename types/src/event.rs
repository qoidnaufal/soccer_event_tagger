use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Other {
        name: String,
    },
    Shot {
        name: String,
        shot_type: String,
        outcome: String,
    },
    Pass {
        name: String,
        pass_type: String,
        outcome: String,
    },
    Dribble {
        name: String,
        outcome: String,
    },
    Tackle {
        name: String,
        outcome: String,
    },
    Intercept {
        name: String,
        event_source: String,
    },
    Recovery {
        name: String,
    },
    Duel {
        name: String,
        duel_type: String,
        outcome: String,
    },
    Clearance {
        name: String,
        event_source: String,
    },
    Block {
        name: String,
        event_source: String,
    },
    Pressure {
        name: String,
        outcome: String,
    },
    Save {
        name: String,
        event_source: String,
    },
    Catch {
        name: String,
        event_source: String,
    },
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
    pub fn from_event_args(args: &str) -> Self {
        match args {
            // --- kick off
            "kos" => Self::Pass { name: "Pass".to_string(), pass_type: "Kick Off".to_string(), outcome: "Success".to_string() },
            "koo" => Self::Pass { name: "Pass".to_string(), pass_type: "Kick Off".to_string(), outcome: "Out of Play".to_string() },
            "koi" => Self::Pass { name: "Pass".to_string(), pass_type: "Kick Off".to_string(), outcome: "Intercepted".to_string() },
            // --- pass
            "ps" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: "Success".to_string() },
            "pi" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: "Intercepted".to_string() },
            "pb" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: "Blocked".to_string() },
            "po" => Self::Pass { name: "Pass".to_string(), pass_type: "Open Play".to_string(), outcome: "Out of Play".to_string() },
            // --- goal kick
            "gks" => Self::Pass { name: "Pass".to_string(), pass_type: "Goalkick".to_string(), outcome: "Success".to_string() },
            "gki" => Self::Pass { name: "Pass".to_string(), pass_type: "Goalkick".to_string(), outcome: "Intercepted".to_string() },
            "gko" => Self::Pass { name: "Pass".to_string(), pass_type: "Goalkick".to_string(), outcome: "Out of Play".to_string() },
            // --- shot
            "son" => Self::Shot { name: "Shot".to_string(), shot_type: "Open Play".to_string(), outcome: "On target".to_string() },
            "sof" => Self::Shot { name: "Shot".to_string(), shot_type: "Open Play".to_string(), outcome: "Off target".to_string() },
            "sb" => Self::Shot { name: "Shot".to_string(), shot_type: "Open Play".to_string(), outcome: "Blocked".to_string() },
            "sg" => Self::Shot { name: "Shot".to_string(), shot_type: "Open Play".to_string(), outcome: "Goal".to_string() },
            // --- penalty
            "pkon" => Self::Shot { name: "Shot".to_string(), shot_type: "Penalty".to_string(), outcome: "On target".to_string() },
            "pkof" => Self::Shot { name: "Shot".to_string(), shot_type: "Penalty".to_string(), outcome: "Off target".to_string() },
            "pkg" => Self::Shot { name: "Shot".to_string(), shot_type: "Penalty".to_string(), outcome: "Goal".to_string() },
            // --- carry
            "drp" => Self::Dribble { name: "Dribble".to_string(), outcome: "Pass".to_string() },
            "drs" => Self::Dribble { name: "Dribble".to_string(), outcome: "Shot".to_string() },
            "drt" => Self::Dribble { name: "Dribble".to_string(), outcome: "Tackled".to_string() },
            "drl" => Self::Dribble { name: "Dribble".to_string(), outcome: "Lost ball".to_string() },
            // --- crossing
            "crs" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: "Success".to_string() },
            "cri" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: "Intercepted".to_string() },
            "crc" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: "Claimed".to_string() },
            "crb" => Self::Pass { name: "Pass".to_string(), pass_type: "Cross".to_string(), outcome: "Blocked".to_string() },
            // --- throw
            "tis" => Self::Pass { name: "Pass".to_string(), pass_type: "Throw In".to_string(), outcome: "Success".to_string() },
            "tii" => Self::Pass { name: "Pass".to_string(), pass_type: "Throw In".to_string(), outcome: "Intercepted".to_string() },
            "tio" => Self::Pass { name: "Pass".to_string(), pass_type: "Throw In".to_string(), outcome: "Out of Play".to_string() },
            // --- Direct cornerkick
            "ckds" => Self::Pass { name: "Pass".to_string(), pass_type: "Direct Cornerkick".to_string(), outcome: "Success".to_string() },
            "ckdi" => Self::Pass { name: "Pass".to_string(), pass_type: "Direct Cornerkick".to_string(), outcome: "Intercepted".to_string() },
            "ckdc" => Self::Pass { name: "Pass".to_string(), pass_type: "Direct Cornerkick".to_string(), outcome: "Claimed".to_string() },
            "ckdo" => Self::Pass { name: "Pass".to_string(), pass_type: "Direct Cornerkick".to_string(), outcome: "Out of Play".to_string() },
            "cksg" => Self::Shot { name: "Shot".to_string(), shot_type: "Direct Cornerkick".to_string(), outcome: "Goal".to_string() },
            "ckson" => Self::Shot { name: "Shot".to_string(), shot_type: "Direct Cornerkick".to_string(), outcome: "On Target".to_string() },
            // --- Short cornerkick
            "ckps" => Self::Pass { name: "Pass".to_string(), pass_type: "Short Cornerkick".to_string(), outcome: "Success".to_string() },
            "ckpi" => Self::Pass { name: "Pass".to_string(), pass_type: "Short Cornerkick".to_string(), outcome: "Intercepted".to_string() },
            "ckpo" => Self::Pass { name: "Pass".to_string(), pass_type: "Short Cornerkick".to_string(), outcome: "Out of Play".to_string() },
            // --- Direct freekick
            "fkon" => Self::Shot { name: "Shot".to_string(), shot_type: "Freekick".to_string(), outcome: "On Target".to_string() },
            "fkof" => Self::Shot { name: "Shot".to_string(), shot_type: "Freekick".to_string(), outcome: "Off Target".to_string() },
            "fkb" => Self::Shot { name: "Shot".to_string(), shot_type: "Freekick".to_string(), outcome: "Blocked".to_string() },
            // --- Indirect freekick
            "fkps" => Self::Pass { name: "Pass".to_string(), pass_type: "Freekick".to_string(), outcome: "Success".to_string() },
            "fkpi" => Self::Pass { name: "Pass".to_string(), pass_type: "Freekick".to_string(), outcome: "Intercepted".to_string() },
            "fkpo" => Self::Pass { name: "Pass".to_string(), pass_type: "Freekick".to_string(), outcome: "Out of Play".to_string() },
            "fkpc" => Self::Pass { name: "Pass".to_string(), pass_type: "Freekick".to_string(), outcome: "Claimed".to_string() },
            // --- tackle
            "tw" => Self::Tackle { name: "Tackle".to_string(), outcome: "Won".to_string() },
            "tl" => Self::Tackle { name: "Tackle".to_string(), outcome: "Lost".to_string() },
            // --- recovery
            "r" => Self::Recovery { name: "Recovery".to_string() },
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
            "prop" => Self::Pressure { name: "Pressure".to_string(), outcome: "Opponent Pass".to_string() },
            "prod" => Self::Pressure { name: "Pressure".to_string(), outcome: "Opponent Dribble".to_string() },
            "prtw" => Self::Pressure { name: "Pressure".to_string(), outcome: "Tackle Won".to_string() },
            "prtl" => Self::Pressure { name: "Pressure".to_string(), outcome: "Tackle Lost".to_string() },
            "probl" => Self::Pressure { name: "Pressure".to_string(), outcome: "Opponent Ball Lost".to_string() },
            // --- goalkeeper save
            "svs" => Self::Save { name: "Save".to_string(), event_source: "Shot".to_string() },
            "svp" => Self::Save { name: "Save".to_string(), event_source: "Penalty".to_string() },
            "svfk" => Self::Save { name: "Save".to_string(), event_source: "Freekick".to_string() },
            "svck" => Self::Save { name: "Save".to_string(), event_source: "Cornerkick".to_string() },
            "svti" => Self::Save { name: "Save".to_string(), event_source: "Throw In".to_string() },
            // --- goalkeeper claim
            "ctcr" => Self::Catch { name: "Catch".to_string(), event_source: "Crossing".to_string() },
            "ctp" => Self::Catch { name: "Catch".to_string(), event_source: "Pass".to_string() },
            "ctfk" => Self::Catch { name: "Catch".to_string(), event_source: "Freekick".to_string() },
            "ctck" => Self::Catch { name: "Catch".to_string(), event_source: "Cornerkick".to_string() },
            "ctti" => Self::Catch { name: "Catch".to_string(), event_source: "Throw In".to_string() },
            // --- other events
            "f" => Self::Other { name: "Foul".to_string() },
            "yc" => Self::Other { name: "Yellow Card".to_string() },
            "rc" => Self::Other { name: "Red Card".to_string() },
            "eofh" => Self::Other { name: "End of First Half".to_string() },
            "eosh" => Self::Other { name: "End of Second Half".to_string() },
            "eom" => Self::Other { name: "End of Match".to_string() },
            _ => Self::Other { name: "Unregistered".to_string(),
            },
        }
    }
}
