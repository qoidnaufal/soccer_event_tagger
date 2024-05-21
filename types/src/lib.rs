use serde::{Deserialize, Serialize};

mod event;
mod point;
mod tagged_event;

pub type Event = event::Event;
pub type TaggedEvent = tagged_event::TaggedEvent;
pub type Point = point::Point;

#[derive(Serialize, Deserialize)]
pub struct Dummy {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub payload: TaggedEvent,
}
