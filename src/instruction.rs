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
//!   └─▶ ChatResponse                      structured output
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

use crate::cognitive::CognitiveCore;
use crate::facet::Facet;
use crate::generate::{ContextWaveBuffer, Generator};
use crate::layers::HierarchicalPhaseField;
use crate::tokenizer::Tokenizer;
use crate::trainer::Trainer;
use crate::wave::Wave;
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

/// ChatResponse - clean response from the chat/instruct pipeline.
/// Separates the conversational text from cognitive metadata.
pub struct ChatResponse {
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
