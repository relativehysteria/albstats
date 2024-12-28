use crate::albion::event::registry::{Decoder, Registry};
use crate::albion::EventType;
use crate::DecodeError;
use photon_decode::EventData;

/// Decoded faction currency update
#[derive(Debug)]
pub struct UpdateSilver {
    /// The current amount of silver the player has
    current: usize,
}

/// Decoder for raw photon messages that returns a `Leave` update
#[derive(Copy, Clone, Debug)]
pub struct UpdateSilverDecoder;

impl Decoder for UpdateSilverDecoder {
    type Output = UpdateSilver;

    fn decode(&self, data: &EventData) -> Result<Self::Output, DecodeError> {
        let p = &data.parameters;
        Ok(UpdateSilver {
            current: crate::ph_int!(p, 1, usize)? / 10_000,
        })
    }
}

/// Registers the faction currency update decoder with the registry
pub fn register(registry: &mut Registry) {
    registry.register_decoder(EventType::UpdateMoney, UpdateSilverDecoder);
}
