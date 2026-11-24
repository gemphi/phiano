mod run;
mod iterate;

use phient::train::chunker::ChunkStore;
use crate::command::{Context, Dispatcher};
use crate::config;
use crate::core::searle::CognitiveCore;
use phient::train::grounding::DefinitionGrounder;
use phient::train::envision::Envision;
use phiecs::phasor::facet::Facet;
use phient::memory::Memo;
use crate::persona::Styles;
use phiecs::phasor::facet::storage::Storage;
use phiton::p64::words::Tokenizer;
use phient::train::trainer::Trainer;
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
    pub context_buffer: phient::generate::ContextWaveBuffer,
    /// The cognitive core - 16 Searle-inspired agents.
    pub cognitive_core: CognitiveCore,
    /// Journal of taught corrections, replayed after grounding.
    pub corrections: phient::train::correction::CorrectionLog,
    /// Gap detection, carrying its ledger across turns.
    pub envisioner: Envision,
    /// Local dictionary, consulted before the user is asked about a gap.
    pub chunk_store: ChunkStore,
}

impl Model {
    /// The command context: a view of the model's own parts under the line
    /// being answered. Built here once, so a part added to the model
    /// reaches every caller - the prompt loop, the one-shot help and the
    /// server's command route each held their own copy of this literal.
    pub fn context<'a>(&'a mut self, line: &'a str, arg: &'a str) -> Context<'a> {
        Context {
            manifold: &mut self.facet,
            trainer: &self.trainer,
            memory: &mut self.memo,
            world: &mut self.world,
            context_buffer: &mut self.context_buffer,
            cognitive_core: &self.cognitive_core,
            corrections: &mut self.corrections,
            gaps: Some(&self.envisioner.ledger),
            arg,
            line,
        }
    }
}
