use crate::albion::event::registry::{Decoder, Registry};
use crate::albion::EventType;
use crate::DecodeError;
use photon_decode::EventData;

/// Decoded faction currency update
#[derive(Debug)]
pub struct SilverUpdate {
    /// The current amount of silver the player has
    current: usize,
}

/// Decoder for raw photon messages that returns a `Leave` update
#[derive(Copy, Clone, Debug)]
pub struct SilverUpdateDecoder;

impl Decoder for SilverUpdateDecoder {
    type Output = SilverUpdate;

    fn decode(&self, data: &EventData) -> Result<Self::Output, DecodeError> {
        let p = &data.parameters;
        Ok(SilverUpdate {
            current: crate::ph_int!(p, 1, usize)? / 10_000,
        })
    }
}

struct Dummy;

impl crate::albion::event::registry::Handler<SilverUpdate> for Dummy {
    fn handle(&self, update: &SilverUpdate) {
        println!("{update:?}");
    }
}

/// Registers the faction currency update decoder with the registry
pub fn register(registry: &mut Registry) {
    registry.register_decoder(EventType::UpdateMoney, SilverUpdateDecoder);
    registry.register_handler(EventType::UpdateMoney, Dummy);
}
