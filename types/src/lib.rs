use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

mod event;
mod point;
mod tagged_event;

pub use event::Event;
pub use point::Point;
pub use tagged_event::TaggedEvent;

#[derive(Serialize, Deserialize)]
pub struct Payload<T: Clone + 'static> {
    pub payload: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, ThisError)]
pub enum AppError {
    #[error("Database Error: {0}")]
    DatabaseError(String),
}
