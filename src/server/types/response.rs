#[allow(unused_imports)]
use super::*;

#[derive(Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub speech_act: String,
    pub direction_of_fit: String,
    pub words_learned: usize,
    pub definitions_learned: usize,
    pub wiki_learned: Option<String>,
    pub vocabulary: usize,
    pub coherence: f64,
}

#[derive(Serialize)]
pub struct CognitiveAgentOutput {
    pub agent_name: String,
    pub agent_role: String,
    pub confidence: f64,
    pub output: String,
}

#[derive(Serialize)]
pub struct CognitiveResponse {
    pub prompt: String,
    pub synthesized_output: String,
    pub coherence: f64,
    pub intentionality_phase: f64,
    pub speech_act: String,
    pub direction_of_fit: String,
    pub satisfaction: f64,
    pub agent_outputs: Vec<CognitiveAgentOutput>,
    pub vocabulary: usize,
    pub intentional_states: Vec<crate::cognitive::IntentionalState>,
    pub felicity_conditions: crate::cognitive::FelicityCondition,
    pub perlocutionary_effect: String,
    pub propositional_content: String,
    pub speaker_meaning: String,
    pub literal_meaning: String,
}
