#[allow(unused_imports)]
use super::*;

/// Words seen fewer times than this sit near their seed and carry no relation.
pub(super) const COUNT_FLOOR: u32 = 25;

/// A pair must occur in at least this many *different* sentences to be scored.
///
/// The entire point. A pair seen once has one frame, and one frame cannot show
/// that the frame matters — which is the dictionary's situation for nearly every
/// pair it produces.
pub(super) const MIN_FRAMES: usize = 4;

/// Distance within a sentence for two words to count as a pair.
///
/// Adjacent-ish rather than any-two-words: a 21-token sentence has 210 unordered
/// pairs, most of which stand in no relation at all, and they would swamp the
/// ones that do.
pub(super) const WINDOW: usize = 4;

/// Rows scored, most-recurring pairs first. Keeps the run affordable and spends
/// the budget on pairs that actually have several frames.
pub(super) const MAX_ROWS: usize = 40_000;

/// Frames scored per pair.
///
/// Without this the sample is eaten by a handful of pairs: sorting by recurrence
/// puts `of the` at the top with thousands of occurrences, and the first run
/// spent 40,000 rows on a few hundred distinct pairs — 590 million same-pair
/// comparisons, when 40,000 rows over 10,000 pairs should give a few hundred
/// thousand. The headline finding is true either way, but every other number on
/// the page was describing `of the`.
pub(super) const MAX_ROWS_PER_PAIR: usize = 8;
