//! Central configuration constants for the Phiano chromatic resonance engine.
//!
//! All tunable parameters and mathematical constants live here.
//! The system is built on four fundamental constants:
//!
//!   α  (alpha)  - fine-structure constant (~1/137)
//!   φ  (phi)    - golden ratio (1.618...)
//!   π  (pi)     - circle constant (3.14159...)
//!   2ⁿ          - power-of-2 sector resolution (2⁶ = 64 minimum)

mod types;
mod novelty;
mod update;
mod primes;
pub use types::*;
pub use novelty::*;
pub use update::*;
pub use primes::*;

use std::f64::consts::PI;
