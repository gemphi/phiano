use super::SharedModel;
use crate::phase_flow::PhaseFlow;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FlowRequest {
    pub text: String,
    pub steps: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct FlowResponse {
    pub nodes: Vec<crate::phase_flow::FlowNode>,
    pub edges: Vec<crate::phase_flow::FlowEdge>,
    pub trajectory: Vec<crate::phase_flow::FlowStep>,
    pub collective_phase: f64,
    pub momentum: f64,
    pub order_parameter: f64,
    pub novelty: f64,
    pub node_count: usize,
    pub edge_count: usize,
}

pub async fn phase_flow(
    State(state): State<SharedModel>,
    Json(req): Json<FlowRequest>,
) -> Json<FlowResponse> {
    let model = state.lock().expect("model lock");
    let steps = req.steps.unwrap_or(10);
    let mut flow = PhaseFlow::build(&model.facet, &req.text);
    flow.propagate(steps);
    let novelty = flow.novelty();

    Json(FlowResponse {
        node_count: flow.nodes.len(),
        edge_count: flow.edges.len(),
        collective_phase: flow.collective_phase,
        momentum: flow.momentum,
        order_parameter: flow.order_parameter,
        novelty,
        nodes: flow.nodes,
        edges: flow.edges,
        trajectory: flow.trajectory,
    })
}
