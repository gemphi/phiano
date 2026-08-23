pub mod api;
pub mod types;
pub mod routes_core;
pub mod routes_oscillator;
pub mod routes_infinity;
pub mod routes_wiki;
pub mod routes_cognitive;
pub mod routes_chat;

use crate::model::Model;
use std::sync::Arc;
use std::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

/// Shared model state — wrapped in Arc<Mutex> for thread-safe access.
pub type SharedModel = Arc<Mutex<Model>>;

/// Starts the web server on the given port.
pub async fn run(model: Model, port: u16) {
    let state: SharedModel = Arc::new(Mutex::new(model));

    let app = api::router(state)
        .layer(CorsLayer::permissive())
        .fallback_service(ServeDir::new("web/dist"));

    let addr = format!("127.0.0.1:{}", port);
    println!("  [web] Phiano web interface at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
