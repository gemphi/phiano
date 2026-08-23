/// Oscillator and compose route handlers.

use super::SharedModel;
use super::types::*;
use crate::compose::Composition;
use crate::oscillator::{OscillatorField, OscillatorEval, OscillatorTrainer};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct OscillatorEvalResponse {
    pub coherence: f64,
    pub sync: f64,
    pub entropy: f64,
    pub word_count: usize,
    pub dominant_colors: Vec<(String, f64)>,
}

#[derive(Serialize)]
pub struct OscillatorTrainResponse {
    pub epochs: usize,
    pub coherence_before: f64,
    pub coherence_after: f64,
    pub sync_before: f64,
    pub sync_after: f64,
    pub converged: bool,
}

#[derive(Serialize)]
pub struct ComposeResponse {
    pub text: String,
    pub winning_sector: u16,
    pub winning_color: String,
    pub rounds: usize,
    pub coherence: f64,
    pub novelty: f64,
    pub resonance: f64,
    pub overall: f64,
    pub verdict: String,
}

pub async fn osc_eval(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<OscillatorEvalResponse>, StatusCode> {
    let model = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let field = OscillatorField::from_facet(&model.facet);
    let result = OscillatorEval::evaluate(&field, &req.text);
    Ok(Json(OscillatorEvalResponse {
        coherence: result.coherence,
        sync: result.sync,
        entropy: result.entropy,
        word_count: result.word_count,
        dominant_colors: result.dominant_colors,
    }))
}

pub async fn osc_train(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<OscillatorTrainResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let text = req.text.clone();
    let epochs = req.epochs.unwrap_or(10);
    let trainer = OscillatorTrainer::new();
    let result = trainer.train(&mut model.facet, &text, epochs);
    Ok(Json(OscillatorTrainResponse {
        epochs: result.epochs,
        coherence_before: result.coherence_before,
        coherence_after: result.coherence_after,
        sync_before: result.sync_before,
        sync_after: result.sync_after,
        converged: result.converged,
    }))
}

pub async fn compose(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<ComposeResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let result = Composition::compose(
        &mut model.facet, &model.trainer, &req.text, &[], 1,
    );
    Ok(Json(ComposeResponse {
        text: result.text,
        winning_sector: result.winning_sector,
        winning_color: result.winning_color,
        rounds: result.rounds,
        coherence: result.eval.coherence,
        novelty: result.eval.novelty,
        resonance: result.eval.resonance,
        overall: result.eval.overall,
        verdict: format!("{}", result.eval.verdict),
    }))
}
