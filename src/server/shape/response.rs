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

/// The cognitive cycle's own result, on the wire as it stands, with the
/// vocabulary it was read against. The fourteen fields were declared here
/// a second time and copied across one by one, so a field added to the
/// cycle never reached the route; flattened, the wire carries whatever the
/// cycle holds (each agent's `phase_contribution` among it now).
#[derive(Serialize)]
pub struct CognitiveResponse {
    #[serde(flatten)]
    pub result: crate::core::searle::CognitiveResult,
    pub vocabulary: usize,
}
