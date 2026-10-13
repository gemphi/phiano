#[allow(unused_imports)]
use super::*;

/// Mean and sample standard deviation of a metric across seeds.
///
/// Sample sd (n-1), because these runs are a sample of the seeds that could
/// have been drawn, not the population of them. With n = 5 the difference is
/// not cosmetic.
#[derive(Clone, Copy, Default)]
pub(super) struct Stat {
    pub(super) mean: f64,
    pub(super) sd: f64,
    pub(super) n: usize,
}

pub(super) struct Row {
    pub(super) name: String,
    pub(super) pair_vs_random: f64,
    pub(super) neighbour_top10: f64,
    pub(super) analogy_top1: f64,
    pub(super) analogy_mrr: f64,
    pub(super) dispersion: f64,
    pub(super) valid_ppl: f64,
    pub(super) usable_pairs: usize,
}
