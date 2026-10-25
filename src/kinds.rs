#![doc = include_str!("kinds/README.md")]

mod types;
mod estimator;
mod table;
mod modes;
mod relation;
mod saturation;
mod permute;
#[cfg(test)]
mod tests;
pub use types::*;
#[allow(unused_imports)]
use estimator::*;
#[allow(unused_imports)]
use table::*;
pub use modes::*;
#[allow(unused_imports)]
use relation::*;
#[allow(unused_imports)]
use saturation::*;
#[allow(unused_imports)]
use permute::*;

use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use crate::roles::RoleDiscovery;
use crate::config::PHASE_CHANNELS;
use crate::wave::c64;
use std::collections::HashMap;
