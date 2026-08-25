/// Cognitive, curriculum, and reasoning chain route handlers.

use super::SharedModel;
use super::types::*;
use crate::curriculum::{ChildCurriculum, CurriculumResult};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

pub async fn cognitive(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<CognitiveResponse>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut ctx_buf = crate::generate::ContextWaveBuffer::new(4096);
    let result = guard.cognitive_core.process(&guard.facet, &mut ctx_buf, &req.text);

    let agent_outputs: Vec<CognitiveAgentOutput> = result.agent_outputs.iter()
        .map(|c| CognitiveAgentOutput {
            agent_name: c.agent_name.to_string(),
            agent_role: c.agent_role.to_string(),
            confidence: c.confidence,
            output: c.output.clone(),
        })
        .collect();

    Ok(Json(CognitiveResponse {
        prompt: result.prompt,
        synthesized_output: result.synthesized_output,
        coherence: result.coherence,
        intentionality_phase: result.intentionality_phase,
        speech_act: result.speech_act,
        direction_of_fit: result.direction_of_fit,
        satisfaction: result.satisfaction,
        agent_outputs,
        vocabulary: guard.facet.vocabulary_size(),
        intentional_states: result.intentional_states,
        felicity_conditions: result.felicity_conditions,
        perlocutionary_effect: result.perlocutionary_effect,
        propositional_content: result.propositional_content,
        speaker_meaning: result.speaker_meaning,
        literal_meaning: result.literal_meaning,
    }))
}

#[allow(dead_code)]
pub async fn run_curriculum(
    State(state): State<SharedModel>,
) -> Result<Json<CurriculumResult>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let curriculum = ChildCurriculum::new();
    let chunk_store = crate::chunker::ChunkStore::new("data/chunks");
    let result = curriculum.run(&mut model.facet, &model.trainer, &chunk_store);
    Ok(Json(result))
}

pub async fn reason_chain(
    State(state): State<SharedModel>,
    Json(req): Json<ReasonChainRequest>,
) -> Result<Json<crate::cognitive::ReasoningResult>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut ctx_buf = crate::generate::ContextWaveBuffer::new(4096);
    let result = guard.cognitive_core.reason(
        &guard.facet, &mut ctx_buf, &req.text, req.max_steps,
    );
    Ok(Json(result))
}
