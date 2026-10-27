//! Fingerprint - a style signature extracted from text examples.
//!
//! Captures how a persona's text distributes across the phase circle's
//! sectors. Each text sample is converted to a wave, mapped to a sector,
//! and the distribution is accumulated.

mod extract;
mod likelihood;
#[allow(unused_imports)]
use extract::*;
#[allow(unused_imports)]
use likelihood::*;

mod traits;

use phiecs::phasor::facet::Facet;
use crate::tokenizer::Tokenizer;
use phiecs::phasor::wave::Wave;
use std::fmt;

pub struct Fingerprint {
    /// How many text samples were used to build this fingerprint.
    pub sample_count: usize,
    /// Histogram: frequency of each sector across all samples.
    pub sector_histogram: Vec<f64>,
    /// Dominant sectors sorted by frequency (sector index, weight).
    pub dominant: Vec<(u16, f64)>,
    /// Average text length (in tokens) across samples.
    pub avg_length: f64,
    /// Sector diversity (entropy of the histogram).
    pub diversity: f64,
}

#[cfg(test)]
mod tests;
