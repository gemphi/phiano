#[allow(unused_imports)]
use super::*;

pub(super) const COUNT_FLOOR: u32 = 25;
pub(super) const DRAWS: usize = 64;

/// Pairs sampled per channel when asking whether a channel knows anything.
pub(super) const MAX_PAIRS_PER_CHANNEL: usize = 20_000;

/// Below this a channel has not seen enough to be tested either way.
pub(super) const MIN_PAIRS_FOR_A_KIND: usize = 200;

/// The gap over the shuffled null a channel must clear to count as a kind.
pub(super) const KIND_MIN_GAP: f64 = 0.02;

/// Pairs a channel must cover before it can be a family's best channel.
pub(super) const MIN_COVERAGE: usize = 6;
