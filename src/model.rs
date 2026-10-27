mod run;
mod iterate;
#[allow(unused_imports)]
use run::*;
#[allow(unused_imports)]
use iterate::*;

use crate::chunker::ChunkStore;
use crate::command::{Context, Dispatcher};
use crate::config;
use crate::core::searle::CognitiveCore;
use crate::core::grounding::DefinitionGrounder;
use crate::envision::Envision;
use phiecs::phasor::facet::Facet;
use crate::memory::Memo;
use crate::persona::Styles;
use crate::storage::Storage;
use crate::tokenizer::Tokenizer;
use crate::trainer::Trainer;
use rustyline::Editor;
use rustyline::history::DefaultHistory;
use std::fs;

/// Model - the recursive learning agent.
///
/// Operates in a continuous cycle:
///
///   envision → apply → eval → iterate → scale
///
/// Each user input triggers this cycle. The model envisions what it
/// doesn't know, applies training, evaluates understanding, iterates
/// on gaps, and scales by persisting knowledge.
pub struct Model {
    /// Turns since the last automatic checkpoint.
    pub(super) turns_since_save: usize,
    /// The facet - lexicon of words mapped to complex phasors.
    pub facet: Facet,
    /// The trainer - Kuramoto phase attraction learning engine.
    pub trainer: Trainer,
    /// The memo - 16-layer memory log of all interactions.
    pub memo: Memo,
    /// The world - collection of personas for impersonation.
    pub world: Styles,
    /// The running multi-turn context wave buffer.
    pub context_buffer: crate::generate::ContextWaveBuffer,
    /// The cognitive core - 16 Searle-inspired agents.
    pub cognitive_core: CognitiveCore,
    /// Journal of taught corrections, replayed after grounding.
    pub corrections: crate::correction::CorrectionLog,
    /// Gap detection, carrying its ledger across turns.
    pub envisioner: Envision,
    /// Local dictionary, consulted before the user is asked about a gap.
    pub chunk_store: ChunkStore,
}
