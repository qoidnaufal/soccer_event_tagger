use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

mod event;
mod point;
mod tagged_event;

pub use event::Event;
pub use point::Point;
pub use tagged_event::TaggedEvent;

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub payload: TaggedEvent,
}

pub struct Database {
    pub db: Arc<Mutex<Vec<TaggedEvent>>>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
