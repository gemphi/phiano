#[allow(unused_imports)]
use super::*;

/// The floor the word-level scores are read at.
///
/// Zero is the whole vocabulary, most of which the model saw fewer than five
/// times; those words sit at their hash seeds and every average over them is
/// dominated by initialisation. The floor sweep in `bin/roles` showed the
/// project's relational headline was carried entirely by that tail.
pub(super) const FLOOR: CountFloor = 25;

pub(super) struct Arm {
    pub(super) label: String,
    pub(super) facet: Facet,
    pub(super) report: Option<DescentReport>,
}
