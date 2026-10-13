#[allow(unused_imports)]
use super::*;

pub(super) struct Path {
    pub(super) name: &'static str,
    pub(super) samples: Vec<Duration>,
    pub(super) note: String,
}
