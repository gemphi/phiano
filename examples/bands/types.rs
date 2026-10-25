#[allow(unused_imports)]
use super::*;

/// Channels per band. Eight bands of eight over the 64-channel torus.
pub(super) const BAND: usize = 8;

/// Sentences of context before the prediction point.
pub(super) const CONTEXT: usize = 3;
/// Candidates ranked per item.
pub(super) const CANDIDATES: usize = 50;
/// Items scored. Enough for a stable mean, small enough to finish.
pub(super) const ITEMS: usize = 600;
/// Random partners drawn per relation pair.
pub(super) const DRAWS: usize = 32;

/// Words below this were barely trained and sit at their hash seed.
pub(super) const COUNT_FLOOR: u32 = 25;

pub(super) struct Item {
    pub(super) context: Vec<Vec<String>>,
    pub(super) candidates: Vec<Vec<String>>,
    pub(super) truth: usize,
}
