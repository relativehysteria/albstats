use crate::DecodeError;

/// Type of origin that caused an effect
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum EffectOrigin {
    MeleeAttack = 0,
    RangedAttack,
    SpellDirect,
    SpellArea,
    SpellPassive,
    OverTimeEffect,
    Reflected,
    SpellCost,
    ServerAuthority,
    Unknown = 10,
}

impl TryFrom<u8> for EffectOrigin {
    type Error = DecodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match usize::from(value) < core::mem::variant_count::<Self>() {
            true  => Ok(unsafe { core::mem::transmute(value) }),
            false => Err(DecodeError::UnexpectedValue(usize::from(value))),
        }
    }
}
