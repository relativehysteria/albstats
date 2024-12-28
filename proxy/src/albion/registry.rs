use crate::albion::EventRegistry;
use crate::DecodeError;
use photon_decode::Message;

/// Registry for packet decoders and their handlers
pub struct Registry {
    events: EventRegistry,
}

impl Registry {
    /// Creates a new registry with registered decoders and handlers
    pub fn new() -> Self {
        let events = EventRegistry::new();
        Self { events }
    }

    /// Handles a raw photon message packate
    pub fn handle_msg(&self, msg: &Message) -> Result<(), DecodeError> {
        match msg {
            Message::Event(data)     => self.events.process_event(data),
            Message::Request(_data)  => Ok(()),
            Message::Response(_data) => Ok(()),
        }
    }
}
