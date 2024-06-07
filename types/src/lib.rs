use serde::{Deserialize, Serialize};

mod event;
mod point;
mod tagged_event;

pub use event::Event;
pub use point::Point;
pub use tagged_event::TaggedEvent;

#[derive(Serialize, Deserialize)]
pub struct Dummy {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub payload: TaggedEvent,
}
