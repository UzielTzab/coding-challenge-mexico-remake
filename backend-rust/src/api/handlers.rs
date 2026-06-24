use axum::{routing::get, Router};
use std::sync::Arc;
use super::ws::ws_handler;

pub fn router(redis_url: String) -> Router {
    let state = Arc::new(redis_url);
    Router::new()
        .route("/health", get(health_check))
        .route("/ws/market", get(ws_handler))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK - HFT Engine is alive"
}
