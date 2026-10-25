#[allow(unused_imports)]
use super::*;

/// SplitMix64 — a deterministic mixer used to derive sampling indices.
///
/// Training draws its randomness from the input itself rather than from a
/// mutable generator, so a run is reproducible and `Trainer` stays `Sync`.
/// splitmix64 is phinet's `mix64` (docs/159: one hash).
#[inline]
pub(super) fn splitmix(z: u64) -> u64 {
    phistd::mix64(z)
}

/// Shortest signed angular difference, wrapped into (−π, π] — phical's `wrap`.
#[inline]
pub fn wrap_signed(delta: f64) -> f64 {
    phical::wrap(delta)
}
