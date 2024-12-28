use crate::DecodeError;

/// Type of effect
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum EffectType {
    None = 0,
    Physical,
    Magical = 2,
}

impl TryFrom<u8> for EffectType {
    type Error = DecodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match usize::from(value) < core::mem::variant_count::<Self>() {
            true  => Ok(unsafe { core::mem::transmute(value) }),
            false => Err(DecodeError::UnexpectedValue(usize::from(value))),
        }
    }
}
