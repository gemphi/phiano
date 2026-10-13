#[allow(unused_imports)]
use super::*;

pub(super) const POOL_FLOOR: u32 = 25;
pub(super) const WINDOW: usize = 6;
pub(super) const BATCH: usize = 64;

/// A decoy sampler: (target, seed) ⇒ the decoy word's index.
pub(super) type Sampler<'a> = &'a dyn Fn(usize, u64) -> usize;

/// The data an LM operator is trained and validated on.
pub(super) struct LmData<'a> {
    pub space: &'a FunctionSpace,
    pub train: &'a [Example],
    pub valid: &'a [Example],
    pub pool: &'a [usize],
}

/// `apply` needs `&mut`; this lets a comparison borrow immutably by copying
/// the operator's bytes first.
pub(super) trait CloneApply {
    fn clone_apply(&self, inputs: &[WordFunction]) -> Vec<WordFunction>;
}
