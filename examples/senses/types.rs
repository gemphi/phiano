#[allow(unused_imports)]
use super::*;

/// Words seen fewer times than this are not split: a handful of occurrences
/// cannot support several roles, and splitting them only dilutes.
pub(super) const SPLIT_FLOOR: u32 = 60;

/// Tokens either side of an occurrence that describe it.
pub(super) const WINDOW: usize = 3;

/// RoleTable per word, overridable from the command line.
///
/// Bounded, in the spirit of the rest of the project — and small, because the
/// evidence per role falls as it rises and the split has to pay for itself.
pub(super) static VARIANTS_CELL: std::sync::OnceLock<usize> = std::sync::OnceLock::new();

/// Laplace smoothing on the bigram tables.
pub(super) const ALPHA: f64 = 0.1;

/// Per-word role centroids, learned from the training half only.
pub(super) struct RoleTable {
    pub(super) centroids: HashMap<String, Vec<Vec<f64>>>,
}

/// How a token is keyed in the bigram table's *condition* side.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Scheme {
    Unsplit,
    Sense,
    Random,
}
