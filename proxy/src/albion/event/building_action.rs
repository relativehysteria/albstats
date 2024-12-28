use crate::albion::event::registry::{Decoder, Registry};
use crate::albion::EventType;
use crate::albion::types::BuildingAction;
use crate::DecodeError;
use photon_decode::EventData;

/// Decoded faction currency update
#[derive(Debug)]
pub struct BuildingActionFinished {
    /// ID of the user that caused this action
    user_id: usize,

    /// ID of the building that was acted on
    building_id: usize,

    /// Type of action that was done
    action_type: BuildingAction,
}

/// Decoder for raw photon messages that returns a `Leave` update
#[derive(Copy, Clone, Debug)]
pub struct BuildingActionDecoder;

impl Decoder for BuildingActionDecoder {
    type Output = BuildingActionFinished;

    fn decode(&self, data: &EventData) -> Result<Self::Output, DecodeError> {
        let p = &data.parameters;
        Ok(BuildingActionFinished {
            user_id: crate::ph_int!(p, 0, usize)?,
            building_id: crate::ph_int!(p, 2, usize)?,
            action_type: crate::ph_int!(p, 4, u8)?.try_into()?,
        })
    }
}

/// Registers the faction currency update decoder with the registry
pub fn register(registry: &mut Registry) {
    registry.register_decoder(
        EventType::ActionOnBuildingFinished, BuildingActionDecoder);
}
