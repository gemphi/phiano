#[allow(unused_imports)]
use super::*;

/// Pairs are drawn only from words the model has actually seen this often.
///
/// A word seen four times sits at its hash seed, so its offset to anything is
/// noise, and a cluster of noise is still a cluster. The floor sweep in
/// `bin/roles` showed what happens when the untrained tail is left in.
pub(super) const COUNT_FLOOR: u32 = 25;
