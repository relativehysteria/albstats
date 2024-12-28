use crate::albion::{EventRegistry, DecodeError};
use crate::albion::event::chat;
use photon_decode::Message;

/// Registry for packet decoders and their handlers
pub struct Registry {
    events: EventRegistry,
}

impl Registry {
    /// Creates a new registry with registered decoders and handlers
    pub fn new() -> Self {
        let mut events = EventRegistry::new();
        chat::register(&mut events);
        Self { events }
    }

    /// Handles a raw photon message packate
    pub fn handle_msg(&self, msg: &Message) -> Result<(), DecodeError> {
        match msg {
            Message::Event(data)    => self.events.process_event(data),
            Message::Request(data)  => Ok(()),
            Message::Response(data) => Ok(()),
        }
    }
}
