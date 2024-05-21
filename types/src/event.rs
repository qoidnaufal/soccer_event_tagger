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

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
