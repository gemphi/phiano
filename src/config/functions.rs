//! Phase-space configuration functions.
//!
//! Provides derived values and utilities computed from the raw constants
//! defined in [`constants`]. Every function is an associated method on
//! [`PhiConfig`], eliminating bare standalone functions in accordance
//! with the Diem convention that all public symbols belong to a named type.
//!
//! # Thread Safety
//!
//! All methods are stateless and safe to call from any thread. The stop-word
//! set is lazily initialised via [`OnceLock`] and cached for the process lifetime.
//!
//! # Invariants
//!
//! - `SECTOR_RESOLUTION` is always a power of two with exponent ≥ `SECTOR_EXPONENT_MIN`.
//! - `sector_prime()` returns a value coprime to `SECTOR_RESOLUTION`, preventing
//!   aliasing in phase-to-sector mapping.
//! - Weight sums are normalised: `total_compose_weight()` equals the sum of all
//!   `WEIGHT_*` constants.

mod phi;
mod kernel;
#[allow(unused_imports)]
use phi::*;
pub use kernel::*;

use super::constants::*;

/// Central configuration façade for the Phiano phase-resonance engine.
///
/// All methods are stateless associated functions (`PhiConfig::method_name()`).
/// No instances are ever created—this struct exists solely to namespace
/// configuration logic, following the Diem convention.
///
/// # Architecture
///
/// ```text
/// constants.rs         functions.rs (this file)
/// ┌────────────┐       ┌───────────────────────┐
/// │ ALPHA, PHI │──────▶│ PhiConfig::sector_*()  │
/// │ SECTOR_RES │       │ PhiConfig::eval_*()    │
/// │ WEIGHT_*   │       │ PhiConfig::stop_word*()│
/// │ PRIME_*    │       │ PhiConfig::print_*()   │
/// └────────────┘       └───────────────────────┘
/// ```
pub struct PhiConfig;
