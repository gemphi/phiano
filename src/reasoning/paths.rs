//! Multi-path reasoning: explores multiple reasoning paths from different sectors.
//! Also provides depth-controlled reasoning with effort levels.

mod effort;
mod solve;
#[allow(unused_imports)]
use effort::*;
#[allow(unused_imports)]
use solve::*;

use crate::config::TWO_PI;
use phiecs::phasor::facet::Facet;
use crate::generate::ContextWaveBuffer;
use crate::reasoning::diagnostics::Diagnostic;
use crate::reasoning::pathfinding::{ReasoningChain, ReasoningEngine, ReasoningStep, REASONING_MAX_STEPS};
use crate::tokenizer::Tokenizer;
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum EffortLevel {
    Instant,
    Quick,
    Standard,
    Deep,
    Exhaustive,
}

#[derive(Debug, Default)]
pub struct MultiPath;
