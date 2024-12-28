use crate::albion::event::registry::{Decoder, Registry};
use crate::albion::EventType;
use crate::DecodeError;
use photon_decode::EventData;

/// Decoded leave update
#[derive(Debug)]
pub struct Leave {
    /// ID of the object that has left the cluster
    id: usize,
}

/// Decoder for raw photon messages that returns a `Leave` update
#[derive(Copy, Clone, Debug)]
pub struct LeaveDecoder;

impl Decoder for LeaveDecoder {
    type Output = Leave;

    fn decode(&self, data: &EventData) -> Result<Self::Output, DecodeError> {
        let p = &data.parameters;
        Ok(Leave {
            id: crate::ph_int!(p, 0, usize)?,
        })
    }
}

/// Registers the leave update decoder with the registry
pub fn register(registry: &mut Registry) {
    registry.register_decoder(EventType::Leave, LeaveDecoder);
}
