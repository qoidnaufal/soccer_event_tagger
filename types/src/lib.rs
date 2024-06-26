use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

mod match_info;
mod point;
mod tagged_event;

pub use match_info::{MatchInfo, PlayerInfo, TeamInfoQuery};
pub use point::Point;
pub use tagged_event::TaggedEvent;

#[derive(Clone, Serialize, Deserialize)]
pub struct Payload<T: Clone + 'static> {
    pub payload: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, ThisError, Eq, PartialEq)]
pub enum AppError {
    #[error("Database Error: {0}")]
    DatabaseError(String),
    #[error("Error Writing CSV File: {0}")]
    CsvWriteError(String),
}
