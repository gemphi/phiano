/// API router — assembles all routes from submodules.
use super::SharedModel;
use super::routes_core::*;
use super::routes_oscillator::*;
use super::routes_infinity::*;
use super::routes_wiki::*;
use super::routes_cognitive::*;
use super::routes_chat::*;
use axum::routing::{get, post};
use axum::Router;

pub fn router(state: SharedModel) -> Router {
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
        .route("/api/oscillator/eval", post(osc_eval))
        .route("/api/oscillator/train", post(osc_train))
        .route("/api/stats", get(stats))
        .route("/api/command", post(command))
        .route("/api/infinity/visualize", post(infinity_visualize))
        .route("/api/infinity/train", post(infinity_train))
        .route("/api/wiki/learn", post(wiki_learn))
        .route("/api/wiki/search", post(wiki_search))
        .route("/api/cognitive", post(cognitive))
        .route("/api/curriculum", post(run_curriculum))
        .route("/api/chat", post(chat))
        .route("/api/reason_chain", post(reason_chain))
        .route("/api/define", post(define_word))
        .with_state(state)
}
