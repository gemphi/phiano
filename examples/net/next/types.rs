#[allow(unused_imports)]
use super::*;

pub(super) const POOL_FLOOR: u32 = 25;
pub(super) const WINDOW: usize = 6;
pub(super) const BATCH: usize = 64;
/// Proximal pull of the word table toward its trained start, per step.
pub(super) const JOINT_TETHER: f64 = 0.02;
