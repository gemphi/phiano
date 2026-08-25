/// Core types for the cognitive system - Searle's intentional state model.
///
/// Searle's key insight: mental states have:
/// - Content (what they're about - the propositional content)
/// - Direction of fit (mind→world or world→mind)
/// - Satisfaction conditions (when the state is fulfilled)
/// - Psychological mode (belief, desire, intention, fear, hope, etc.)

use serde::Serialize;

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
    pub felicity_conditions: FelicityConditions,
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

/// Direction of fit - Searle's distinction.
/// Mind→World: the mind should match the world (beliefs, assertions).
/// World→Mind: the world should change to match the mind (desires, commands).
/// None: no direction of fit (expressives - just express a state).
/// Both: both directions (declaratives - create a fact by representing it).
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum DirectionOfFit {
    MindToWorld,
    WorldToMind,
    None,
    Both,
}

impl DirectionOfFit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MindToWorld => "mind→world",
            Self::WorldToMind => "world→mind",
            Self::None => "none (expressive)",
            Self::Both => "both (declarative)",
        }
    }
}

/// Searle's felicity conditions for speech acts.
/// For an illocutionary act to be successful, these must be met.
#[derive(Debug, Clone, Serialize, Default)]
pub struct FelicityConditions {
    /// The propositional content must be appropriate for the act type.
    /// E.g., promises require a future act; assertions require a proposition.
    pub propositional_content_rule: String,
    /// The preparatory condition - what must be true for the act to make sense.
    /// E.g., for a command, the speaker must have authority.
    pub preparatory_condition: String,
    /// The sincerity condition - the speaker must have the appropriate psychological state.
    /// E.g., for a promise, the speaker must intend to act.
    pub sincerity_condition: String,
    /// The essential condition - what the act counts as.
    /// E.g., a promise counts as an undertaking of an obligation.
    pub essential_condition: String,
    /// Whether all conditions are met.
    pub satisfied: bool,
}

/// Searle's 5 speech act categories with their felicity conditions.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum SpeechActType {
    Assertive,
    Directive,
    Commissive,
    Expressive,
    Declarative,
}

impl SpeechActType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Assertive => "assertive",
            Self::Directive => "directive",
            Self::Commissive => "commissive",
            Self::Expressive => "expressive",
            Self::Declarative => "declarative",
        }
    }

    pub fn direction_of_fit(&self) -> DirectionOfFit {
        match self {
            Self::Assertive => DirectionOfFit::MindToWorld,
            Self::Directive => DirectionOfFit::WorldToMind,
            Self::Commissive => DirectionOfFit::WorldToMind,
            Self::Expressive => DirectionOfFit::None,
            Self::Declarative => DirectionOfFit::Both,
        }
    }
}
