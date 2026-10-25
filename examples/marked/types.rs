#[allow(unused_imports)]
use super::*;

/// A word must be seen this often to be a pair member or a distractor.
pub(super) const COUNT_FLOOR: u32 = 25;

/// Distinct pairs a marker needs before its channel can be tested.
pub(super) const MIN_PAIRS: usize = 16;

/// Pairs kept per marker, most frequent first.
pub(super) const MAX_PAIRS: usize = 32;

/// Markers tested, in channel order — the most general relations first.
pub(super) const MARKERS_TESTED: usize = 12;

/// Rival channels each probe is also scored on.
pub(super) const RIVALS: usize = 4;

/// Ranking pool. Smaller than the full vocabulary because the all-channels
/// scorer costs sixty-four times the others, and a rank is a rank.
pub(super) const POOL: usize = 1500;
