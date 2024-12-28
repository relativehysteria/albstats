use crate::albion::{EventRegistry, DecodeError};
use crate::albion::event::chat;
use photon_decode::Message;

pub struct Registry {
    events: EventRegistry,
}

impl Registry {
    pub fn new() -> Self {
        let mut events = EventRegistry::new();
        chat::register(&mut events);
        Self { events }
    }

    pub fn handle_msg(&self, msg: &Message) -> Result<(), DecodeError> {
        match msg {
            Message::Event(data)    => self.events.process_event(data),
            Message::Request(data)  => Ok(()),
            Message::Response(data) => Ok(()),
        }
    }
}
