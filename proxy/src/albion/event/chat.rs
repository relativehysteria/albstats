use crate::albion::event::registry::{Decoder, Handler, Registry};
use crate::albion::{EventType, DecodeError};
use photon_decode::EventData;

/// Decoded chat message
#[derive(Debug)]
pub struct ChatMessage {
    /// Name of the player that sent the message
    pub name: String,

    /// The message
    pub msg: String,
}

/// Decoder for photon messages
#[derive(Copy, Clone, Debug)]
pub struct ChatDecoder;

impl Decoder for ChatDecoder{
    type Output = ChatMessage;

    fn decode(&self, data: &EventData) -> Result<Self::Output, DecodeError> {
        let p = &data.parameters;
        Ok(ChatMessage {
            name: p.get(&0).ok_or(DecodeError::ParameterMissing)?.to_string(),
            msg:  p.get(&1).ok_or(DecodeError::ParameterMissing)?.to_string(),
        })
    }
}

/// Handler for chat messages
#[derive(Copy, Clone, Debug)]
pub struct ChatHandler;

impl Handler<ChatMessage> for ChatHandler {
    fn handle(&self, msg: &ChatMessage) {
        println!("{msg:?}");
    }
}

/// Registers the chat decoder and handlers with the registry
pub fn register(registry: &mut Registry) {
    // registry.register_decoder(EventType::ChatMessage, ChatDecoder);
    // registry.register_handler(EventType::ChatMessage, ChatHandler);
    registry.register_decoder(EventType::ChatSay, ChatDecoder);
    registry.register_handler(EventType::ChatSay, ChatHandler);
}
