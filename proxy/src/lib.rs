#![feature(variant_count)]

/// Definitions for handling albion networking
pub mod albion;

mod error;
pub use error::DecodeError;

#[macro_use] pub mod ph_extract;
