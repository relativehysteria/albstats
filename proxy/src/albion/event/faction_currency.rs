use crate::albion::event::registry::{Decoder, Registry};
use crate::albion::EventType;
use crate::albion::types::CityFaction;
use crate::DecodeError;
use photon_decode::EventData;

/// Decoded faction currency update
#[derive(Debug)]
pub struct CurrencyUpdate {
    /// The faction for which this currency has been updated
    faction: CityFaction,

    /// The amount of faction coins that has been gained
    coins_gained: f32,

    /// Total player faction coins
    coins_total: f32,

    /// Base rank points gained
    rp: f32,

    /// Rank points gained due to population bonus
    rp_population: f32,

    /// Rank points gained due to premium bonus
    rp_premium: f32,
}

/// Decoder for raw photon messages that returns a `Leave` update
#[derive(Copy, Clone, Debug)]
pub struct CurrencyUpdateDecoder;

impl Decoder for CurrencyUpdateDecoder {
    type Output = CurrencyUpdate;

    fn decode(&self, data: &EventData) -> Result<Self::Output, DecodeError> {
        let p = &data.parameters;
        Ok(CurrencyUpdate {
            faction: crate::ph_int!(p, 2, u8)?.try_into()?,
            coins_gained: crate::ph_int!(p, 3, usize)? as f32 / 10_000.,
            coins_total: crate::ph_int!(p, 9, usize)? as f32 / 10_000.,
            rp: crate::ph_int!(p, 4, usize)? as f32 / 10_000.,
            rp_population: crate::ph_int!(p, 6, usize)? as f32 / 10_000.,
            rp_premium: crate::ph_int!(p, 8, usize)? as f32 / 10_000.,
        })
    }
}

struct Dummy;

impl crate::albion::event::registry::Handler<CurrencyUpdate> for Dummy {
    fn handle(&self, update: &CurrencyUpdate) {
        println!("{update:?}");
    }
}

/// Registers the faction currency update decoder with the registry
pub fn register(registry: &mut Registry) {
    registry.register_decoder(EventType::UpdateCurrency, CurrencyUpdateDecoder);
    registry.register_handler(EventType::UpdateCurrency, Dummy);
}
