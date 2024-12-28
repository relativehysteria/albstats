use crate::DecodeError;

/// Type of origin that caused an effect
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum BuildingAction {
    Unknown = 0,
    BuyAndCraft,
    Repair,
    Recycle,
    Investigate = 4,
    RerollQuality = 6,
}

impl TryFrom<u8> for BuildingAction {
    type Error = DecodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match usize::from(value) < core::mem::variant_count::<Self>() {
            true  => Ok(unsafe { core::mem::transmute(value) }),
            false => Err(DecodeError::UnexpectedValue(usize::from(value))),
        }
    }
}
