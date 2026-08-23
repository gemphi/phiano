/// Shared request/response types for all API routes.

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TextRequest {
    pub text: String,
    pub epochs: Option<usize>,
    pub warmup: Option<usize>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f64>,
}

#[derive(Deserialize)]
pub struct DefineRequest {
    pub word: String,
}

#[derive(Serialize)]
pub struct DefineResponse {
    pub word: String,
    pub definition: String,
    pub source: String,
    pub phase: Option<f64>,
    pub amplitude: Option<f64>,
    pub vocabulary: usize,
}

#[derive(Serialize)]
pub struct EvalResponse {
    pub coherence: f64,
    pub novelty: f64,
    pub resonance: f64,
    pub overall: f64,
    pub verdict: String,
    pub vocabulary: usize,
}

#[derive(Serialize)]
pub struct LearnResponse {
    pub tokens: usize,
    pub vocabulary: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct MultiLearnResponse {
    pub epochs: usize,
    pub tokens: usize,
    pub converged: bool,
    pub vocabulary: usize,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub prompt: String,
    pub generated: String,
    pub vocabulary: usize,
    pub context_phase: f64,
    pub context_amplitude: f64,
}

#[derive(Serialize)]
pub struct InstructResponse {
    pub prompt: String,
    pub output: String,
    pub vocabulary: usize,
}

#[derive(Serialize)]
pub struct ReasoningResponse {
    pub problem: String,
    pub converged: bool,
    pub steps_count: usize,
    pub final_answer: String,
}

#[derive(Serialize)]
pub struct LayersResponse {
    pub layers_count: usize,
    pub layer_summaries: Vec<LayerSummaryItem>,
}

#[derive(Serialize)]
pub struct LayerSummaryItem {
    pub level: usize,
    pub sector_count: u16,
    pub clusters_count: usize,
}

#[derive(Serialize)]
pub struct SyntheticResponse {
    pub accepted_count: usize,
    pub vocabulary: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct Phi4LearnResponse {
    pub vocab_tokens_loaded: usize,
    pub merges_trained: usize,
    pub doc_sentences_trained: usize,
    pub final_vocabulary_size: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub vocabulary: usize,
    pub memory_entries: usize,
}

#[derive(Serialize)]
pub struct CommandResponse {
    pub output: String,
}

#[derive(Deserialize)]
pub struct WikiRequest {
    pub topic: String,
    pub epochs: Option<usize>,
}

#[derive(Serialize)]
pub struct WikiLearnResponse {
    pub topic: String,
    pub title: String,
    pub extract: String,
    pub tokens_trained: usize,
    pub vocabulary_before: usize,
    pub vocabulary_after: usize,
    pub coherence: f64,
    pub novelty: f64,
    pub resonance: f64,
    pub verdict: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct WikiSearchResult {
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct WikiSearchResponse {
    pub query: String,
    pub results: Vec<WikiSearchResult>,
}

#[derive(Deserialize)]
pub struct ReasonChainRequest {
    pub text: String,
    #[serde(default = "default_max_steps")]
    pub max_steps: usize,
}

fn default_max_steps() -> usize { 4 }

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
    pub felicity_conditions: crate::cognitive::FelicityConditions,
    pub perlocutionary_effect: String,
    pub propositional_content: String,
    pub speaker_meaning: String,
    pub literal_meaning: String,
}
