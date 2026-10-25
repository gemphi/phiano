mod sentence;
mod ray;
mod sector;
#[cfg(test)]
mod tests;
#[allow(unused_imports)]
use sentence::*;
#[allow(unused_imports)]
use ray::*;
#[allow(unused_imports)]
use sector::*;

use crate::config;
use crate::config::{GOLDEN_ANGLE, PHASE_CHANNELS, TWO_PI};
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use num_complex::Complex64;
use rayon::prelude::*;
use std::cmp::Ordering;

/// Type alias for a complex number with f64 real and imaginary parts.
#[allow(non_camel_case_types)]
pub type c64 = Complex64;

/// Wave - operations on complex wave representations of text.
pub struct Wave;

/// Coarse spatial index over the phase circle.
///
/// Retrieval was O(V) per query, and generation issues one per token. On a
/// circle, nearest-neighbour search does not need a full scan: bucket words by
/// sector, then scan the query's own sector and its neighbours outward. The
/// expected cost is V/`sector_count` rather than V.
///
/// This is the one place a one-dimensional representation is an advantage, and
/// it was going unused.
pub struct SectorIndex {
    pub(super) buckets: Vec<Vec<String>>,
}
