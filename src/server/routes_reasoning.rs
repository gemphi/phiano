/// Reasoning API routes: hybrid reasoning, multi-path, comparison, and benchmark.

use super::SharedModel;
use crate::reasoning;
use crate::metrics;
use crate::lifelong;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HybridReasonRequest {
    pub text: String,
}

#[derive(Serialize)]
pub struct HybridReasonResponse {
    pub final_answer: String,
    pub confidence: f64,
    pub analogies: Vec<(String, f64)>,
    pub structural_matches: Vec<String>,
    pub pathfinding_answer: String,
    pub pathfinding_converged: bool,
}

pub async fn hybrid_reason(
    State(state): State<SharedModel>,
    Json(req): Json<HybridReasonRequest>,
) -> Result<Json<HybridReasonResponse>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let reasoner = reasoning::HybridReasoner::new();
    let result = reasoner.solve_hybrid(&guard.facet, &req.text);

    Ok(Json(HybridReasonResponse {
        final_answer: result.final_answer,
        confidence: result.confidence,
        analogies: result.analogies,
        structural_matches: result.structural_matches,
        pathfinding_answer: result.pathfinding_chain.final_answer,
        pathfinding_converged: result.pathfinding_chain.converged,
    }))
}

#[derive(Deserialize)]
pub struct MultiPathRequest {
    pub text: String,
    pub n_paths: Option<usize>,
}

#[derive(Serialize)]
pub struct MultiPathResponse {
    pub paths: Vec<reasoning::ReasoningChain>,
    pub best_path_index: usize,
    pub best_confidence: f64,
}

pub async fn multi_path_reason(
    State(state): State<SharedModel>,
    Json(req): Json<MultiPathRequest>,
) -> Result<Json<MultiPathResponse>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let n = req.n_paths.unwrap_or(4);
    let paths = reasoning::MultiPath::solve_multi_path(&guard.facet, &req.text, n);

    let mut best_idx = 0;
    let mut best_conf = 0.0;
    for (i, path) in paths.iter().enumerate() {
        let conf = reasoning::Diagnostics::confidence(path);
        if conf > best_conf {
            best_conf = conf;
            best_idx = i;
        }
    }

    Ok(Json(MultiPathResponse {
        paths,
        best_path_index: best_idx,
        best_confidence: best_conf,
    }))
}

#[derive(Deserialize)]
pub struct CompareReasonRequest {
    pub text: String,
}

pub async fn compare_reason(
    State(state): State<SharedModel>,
    Json(req): Json<CompareReasonRequest>,
) -> Result<Json<reasoning::ReasoningComparison>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let comparison = reasoning::ReasoningComparison::compare(&guard.facet, &req.text);
    Ok(Json(comparison))
}

pub async fn benchmark(
    State(state): State<SharedModel>,
) -> Result<Json<metrics::benchmark_runner::BenchmarkReport>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let report = metrics::benchmark_runner::BenchmarkRunner::run_all(&mut model.facet, &model.trainer);

    let mut history = lifelong::history::BenchmarkHistory::load("data/benchmark_history.json");
    history.record(report.clone());
    history.save("data/benchmark_history.json");

    Ok(Json(report))
}
