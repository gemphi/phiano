mod dispatch;
#[cfg(test)]
mod tests;
#[allow(unused_imports)]
use dispatch::*;

pub mod benchmark;
pub mod chat;
pub mod cognitive;
pub mod compose;
pub mod define;
pub mod eval;
pub mod generate;
pub mod help;
pub mod instruction;
pub mod layers;
pub mod learn;
pub mod oscillator;
pub mod persona;
pub mod reasoning;
pub mod resonance;
pub mod save;
pub mod stats;
pub mod synthetic;
pub mod synonym;
pub mod wave;

use crate::cognitive::CognitiveCore;
use phiecs::phasor::facet::Facet;
use crate::generate::ContextWaveBuffer;
use crate::memory::Memo;
use crate::persona::Styles;
use crate::trainer::Trainer;

/// Shared context passed to every command handler.
///
/// Provides mutable access to the facet, memory, world, and multi-turn context buffer.
pub struct Context<'a> {
    /// The facet - mutable so commands can train or modify the lexicon.
    pub manifold: &'a mut Facet,
    /// The trainer - shared across commands for consistent learning.
    pub trainer: &'a Trainer,
    /// The memo - mutable so commands can record interactions.
    pub memory: &'a mut Memo,
    /// The world - mutable so persona commands can add/modify personas.
    pub world: &'a mut Styles,
    /// Multi-turn context wave buffer for conversational continuity.
    pub context_buffer: &'a mut ContextWaveBuffer,
    /// The cognitive core - 16 Searle-inspired agents.
    pub cognitive_core: &'a CognitiveCore,
    /// Persisted journal of taught corrections, so they survive a re-ingest.
    pub corrections: &'a mut crate::correction::CorrectionLog,
    /// Outstanding knowledge gaps, when the caller tracks them.
    pub gaps: Option<&'a crate::envision::GapLedger>,
    /// The argument string after the command name (trimmed).
    pub arg: &'a str,
    /// The full raw input line.
    pub line: &'a str,
}

/// Command - core REPL commands only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Help,
    Learn,
    Define,
    Eval,
    Compose,
    Generate,
    Instruct,
    Chat,
    Reason,
    Layers,
    Synthetic,
    Oscillator,
    Persona,
    Cognitive,
    Synonym,
    Resonance,
    Wave,
    Save,
    Load,
    Stat,
    Benchmark,
    Exit,
    Unknown,
}

/// Dispatcher - routes input lines to the appropriate command handler.
pub struct Dispatcher;

/// Parser - utility for parsing command arguments.
pub struct Parser;
