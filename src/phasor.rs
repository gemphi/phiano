mod types;
mod torus;
mod widen;
mod spectral;
mod count;
mod nudge;
mod dither;
mod spectrum;
#[cfg(test)]
mod tests;
pub use types::*;
pub use torus::*;
pub use widen::*;
#[allow(unused_imports)]
use spectral::*;
#[allow(unused_imports)]
use count::*;
#[allow(unused_imports)]
use nudge::*;
pub use dither::Rounding;
#[allow(unused_imports)]
use spectrum::*;

use crate::config::{
    TWO_PI, GOLDEN_ANGLE, TORUS_HARMONICS_COUNT,
    PHASE_CHANNELS, CHANNEL_QUANTA, BAND_N_EFFECTIVE_MAX, AMPLITUDE_LOG_SCALE,
    AMPLITUDE_MAX,
};
use crate::physics::Phical;
use crate::phiton::{LightQuantum, PhitonColor, PhitonSpectrum};
use crate::wave::c64;
use serde::{Deserialize, Serialize};
