#![allow(dead_code)]
//! Instruction parsing, formatting, and phase-guided execution.
//!
//! Implements the instruction pipeline from prompt classification through
//! grounded definition lookup, cognitive synthesis, and torus-attractor
//! generation.  All public functions are associated methods on
//! [`InstructionEngine`], following the Diem convention.
//!
//! # Architecture
//!
//! ```text
//! User Prompt
//!   │
//!   ├─▶ InstructionKind::parse()          classify intent
//!   ├─▶ InstructionEngine::extract_topic_words()  topic extraction
//!   ├─▶ InstructionEngine::generate_response()    cognitive pipeline
//!   │       ├─▶ lookup_definitions()       grounded knowledge
//!   │       ├─▶ compose_reply()            torus attractor
//!   │       └─▶ CognitiveCore::process()  intentional states
//!   └─▶ Answer                      structured output
//! ```

mod kind;
mod engine;
#[cfg(test)]
mod tests;
#[allow(unused_imports)]
use kind::*;
#[allow(unused_imports)]
use engine::*;
#[allow(unused_imports)]

use crate::core::searle::CognitiveCore;
use phiecs::phasor::facet::Facet;
use phient::generate::{ContextWaveBuffer, Generator};
use phiecs::phasor::layers::HierarchicalPhaseField;
use phiton::p64::words::Tokenizer;
use phient::train::trainer::Trainer;
use phiecs::phasor::wave::Wave;
use std::fmt;

/// InstructionKind - type of instruction issued by user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionKind {
    Code,
    Explain,
    Creative,
    Analyze,
    Command,
}

/// The answer the instruction engine gives: the words said, apart from
/// what the cognitive cycle read in them.
///
/// It was `ChatResponse`, the name the chat route's wire shape carries
/// (`server::shape::response::ChatResponse`), which holds other fields
/// entirely - the words learned, the definitions, the vocabulary - and is
/// never this one. Two types of one name in one crate, sharing two fields
/// of eight; the wire keeps the name its family reads by (2026-11-25).
pub struct Answer {
    /// The main response text shown to the user.
    pub text: String,
    /// The cognitive synthesis output (intentional state-driven).
    pub cognitive_synthesis: String,
    /// Detected intent type.
    pub intent: InstructionKind,
    /// Speech act classification.
    pub speech_act: String,
    /// Direction of fit (mind→world, world→mind, etc.).
    pub direction_of_fit: String,
    /// Satisfaction score [0, 1].
    pub satisfaction: f64,
    /// Phase-guided generation trace.
    pub phase_trace: String,
}

/// InstructionEngine - handles instruction parsing, formatting, and phase-guided execution (Phase 4).
pub struct InstructionEngine {
    pub context_buffer: ContextWaveBuffer,
    pub phase_field: HierarchicalPhaseField,
    pub generator: Generator,
}
