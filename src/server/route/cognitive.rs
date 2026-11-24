//! Cognitive, curriculum, and reasoning chain route handlers.

use super::super::SharedModel;
use super::super::shape::*;
use phient::train::curriculum::{ChildCurriculum, CurriculumResult};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

pub async fn cognitive(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<CognitiveResponse>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut ctx_buf = phient::generate::ContextWaveBuffer::new(4096);
    let result = guard.cognitive_core.process(&guard.facet, &mut ctx_buf, &req.text);

    Ok(Json(CognitiveResponse { result, vocabulary: guard.facet.vocabulary_size() }))
}

#[allow(dead_code)]
pub async fn run_curriculum(
    State(state): State<SharedModel>,
) -> Result<Json<CurriculumResult>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let curriculum = ChildCurriculum::new();
    let chunk_store = phient::train::chunker::ChunkStore::new(&crate::data::path("chunks"));
    let result = curriculum.run(&mut model.facet, &model.trainer, &chunk_store);
    Ok(Json(result))
}

pub async fn reason_chain(
    State(state): State<SharedModel>,
    Json(req): Json<ReasonChainRequest>,
) -> Result<Json<crate::core::searle::Chain<crate::core::searle::Thought>>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut ctx_buf = phient::generate::ContextWaveBuffer::new(4096);
    let result = guard.cognitive_core.reason(
        &guard.facet, &mut ctx_buf, &req.text, req.max_steps,
    );
    Ok(Json(result))
}
