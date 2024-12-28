use crate::albion::event::registry::{Decoder, Handler, Registry};
use crate::albion::{EventType, DecodeError};
use photon_decode::EventData;

#[derive(Debug)]
pub struct ChatMessage {
    name: String,
    msg: String,
}

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

#[derive(Copy, Clone, Debug)]
pub struct ChatHandler;

impl Handler<ChatMessage> for ChatHandler {
    fn handle(&self, msg: &ChatMessage) {
        println!("{msg:?}");
    }
}

pub fn register(registry: &mut Registry) {
    // registry.register_decoder(EventType::ChatMessage, ChatDecoder);
    // registry.register_handler(EventType::ChatMessage, ChatHandler);
    registry.register_decoder(EventType::ChatSay, ChatDecoder);
    registry.register_handler(EventType::ChatSay, ChatHandler);
}
