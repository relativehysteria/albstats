use crate::albion::event::registry::{Decoder, Registry};
use crate::albion::EventType;
use crate::DecodeError;
use crate::albion::types::{EffectType, EffectOrigin};
use photon_decode::EventData;

/// Decoded health update
#[derive(Debug)]
pub struct HealthUpdate {
    /// The ID of the object that caused the update
    pub src_id: usize,

    /// The ID of the object whose health was updated
    pub dst_id: usize,

    /// The delta by which the health has been changed
    pub delta: f64,

    /// New health value of the object
    pub new_hp: f64,

    /// What effect exactly caused the health update
    pub effect_origin: EffectOrigin,

    /// Type of the effect that caused this health update
    pub effect_type: EffectType,

    /// Index of the spell that caused this health update
    pub causing_spell_idx: u16,
}

/// Decoder for raw photon messages that returns a `HealthUpdate`
#[derive(Copy, Clone, Debug)]
pub struct HealthDecoder;

impl Decoder for HealthDecoder {
    type Output = HealthUpdate;

    fn decode(&self, data: &EventData) -> Result<Self::Output, DecodeError> {
        let p = &data.parameters;
        Ok(HealthUpdate {
            dst_id: crate::ph_int!(p, 0, usize)?,
            // 1 is timestamp -- useless
            delta: crate::ph_float!(p, 2, f64)?,
            new_hp: crate::ph_float!(p, 3, f64)?,
            effect_type: crate::ph_int!(p, 4, u8)?.try_into()?,
            effect_origin: crate::ph_int!(p, 5, u8)?.try_into()?,
            src_id: crate::ph_int!(p, 6, usize)?,
            causing_spell_idx: crate::ph_int!(p, 7, u16)?,
        })
    }
}

/// Registers the health update decoder with the registry
pub fn register(registry: &mut Registry) {
    registry.register_decoder(EventType::HealthUpdate, HealthDecoder);
}
