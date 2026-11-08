//! Config module - re-exports constants and functions.
//!
//! Constants live in `constants.rs`, derived functions in `functions.rs`.
//! This separation ensures config values are never mixed with logic.

mod constant;
mod function;

pub use constant::*;
pub use function::*;
