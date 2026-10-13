#[allow(unused_imports)]
use super::*;

pub(super) const COUNT_FLOOR: u32 = 25;
pub(super) const DRAWS: usize = 64;

/// A word's position, as complex numbers per channel.
pub(super) type Point = Vec<(f64, f64)>;
