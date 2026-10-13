/// API router - assembles all routes from submodules.
use super::SharedModel;
use super::routes::core::*;
use super::routes::oscillator::*;
use super::routes::infinity::*;
use super::routes::wiki::*;
use super::routes::cognitive::*;
use super::routes::chat::*;
use super::routes::flow::*;
use super::routes::stream::*;
use super::routes::reasoning::*;
use axum::routing::{get, post};
use axum::Router;

pub struct ApiRouter;

impl ApiRouter {
    pub fn build(state: SharedModel) -> Router {
    Router::new()
        .route("/api/eval", post(eval))
        .route("/api/learn", post(learn))
        .route("/api/learn_multi", post(learn_multi))
        .route("/api/compose", post(compose))
        .route("/api/generate", post(generate_seq))
        .route("/api/instruct", post(instruct))
        .route("/api/reason", post(reason))
        .route("/api/layers", get(layers_info))
        .route("/api/synthetic", post(run_synthetic))
        .route("/api/phi4/learn", post(phi4_learn))
        .route("/api/ingest", post(ingest_all))
        .route("/api/oscillator/eval", post(osc_eval))
        .route("/api/oscillator/train", post(osc_train))
        .route("/api/stats", get(stats))
        .route("/api/command", post(command))
        .route("/api/infinity/visualize", post(infinity_visualize))
        .route("/api/infinity/train", post(infinity_train))
        .route("/api/wiki/learn", post(wiki_learn))
        .route("/api/wiki/search", post(wiki_search))
        .route("/api/cognitive", post(cognitive))
        .route("/api/chat", post(chat))
        .route("/api/reason_chain", post(reason_chain))
        .route("/api/define", post(define_word))
        .route("/api/dialogue/learn", post(dialogue_learn))
        .route("/api/save", post(save_manifold))
        .route("/api/generate/stream", post(generate_stream))
        .route("/api/flow", post(flow))
        .route("/api/reason/hybrid", post(hybrid_reason))
        .route("/api/reason/multi_path", post(multi_path_reason))
        .route("/api/reason/compare", post(compare_reason))
        .route("/api/benchmark", get(benchmark))
        .with_state(state)
    }
}
