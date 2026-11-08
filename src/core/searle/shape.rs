//! Core types for the cognitive system - Searle's intentional state model.
//!
//! Searle's key insight: mental states have:
//! - Content (what they're about - the propositional content)
//! - Direction of fit (mind→world or world→mind)
//! - Satisfaction conditions (when the state is fulfilled)
//! - Psychological mode (belief, desire, intention, fear, hope, etc.)

use serde::Serialize;

// The classifier's own types live in phiton; the agents here read them.
pub use phiton::p64::{DirectionOfFit, FelicityCondition, SpeechAct};

/// A single agent's contribution to the cognitive process.
#[derive(Debug, Clone, Serialize)]
pub struct AgentContribution {
    pub agent_name: &'static str,
    pub agent_role: &'static str,
    pub confidence: f64,
    pub output: String,
    pub phase_contribution: f64,
}

/// The result of a full cognitive cycle.
#[derive(Debug, Clone, Serialize)]
pub struct CognitiveResult {
    pub prompt: String,
    pub agent_outputs: Vec<AgentContribution>,
    pub synthesized_output: String,
    pub coherence: f64,
    pub intentionality_phase: f64,
    pub speech_act: String,
    pub direction_of_fit: String,
    pub satisfaction: f64,
    pub intentional_states: Vec<IntentionalState>,
    pub felicity_conditions: FelicityCondition,
    pub perlocutionary_effect: String,
    pub propositional_content: String,
    pub speaker_meaning: String,
    pub literal_meaning: String,
}

/// Searle's intentional state - the building block of mentality.
/// Each state has a mode, content, direction of fit, and satisfaction conditions.
#[derive(Debug, Clone, Serialize)]
pub struct IntentionalState {
    pub mode: PsychologicalMode,
    pub content: String,
    pub direction_of_fit: DirectionOfFit,
    pub satisfaction_condition: String,
    pub sincerity: f64,
}

/// Psychological modes - the type of mental state.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PsychologicalMode {
    Belief,
    Desire,
    Intention,
    Fear,
    Hope,
    Perception,
}

impl PsychologicalMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Belief => "belief",
            Self::Desire => "desire",
            Self::Intention => "intention",
            Self::Fear => "fear",
            Self::Hope => "hope",
            Self::Perception => "perception",
        }
    }

    #[allow(dead_code)]
    pub fn direction_of_fit(&self) -> DirectionOfFit {
        match self {
            Self::Belief | Self::Perception => DirectionOfFit::MindToWorld,
            Self::Desire | Self::Intention => DirectionOfFit::WorldToMind,
            Self::Fear | Self::Hope => DirectionOfFit::WorldToMind,
        }
    }
}
