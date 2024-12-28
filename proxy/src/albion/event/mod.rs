#![allow(dead_code)]

pub mod health;
pub mod leave;
pub mod faction_currency;
pub mod silver;
pub mod building_action;

/// Event decoder and handler registry
pub mod registry;

mod types;

pub use types::Type as EventType;
pub use registry::Registry as EventRegistry;
