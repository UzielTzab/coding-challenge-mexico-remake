use axum::{routing::{get, put}, Router};
use crate::api::wallets::{dummy_exchanges, dummy_wallet_movements, dummy_wallets};
use crate::api::opportunities::{get_opportunities, get_performance};
use crate::api::settings::{get_settings_dummy, update_setting};
use tower_http::cors::{Any, CorsLayer};
use super::ws::ws_handler;
use std::sync::Arc;
use sqlx::PgPool;

pub struct AppState {
    pub redis_url: String,
    pub pool: Option<PgPool>,
    pub discarded_ticks: Arc<std::sync::atomic::AtomicU64>,
}

pub fn router(redis_url: String, pool: Option<PgPool>, discarded_ticks: Arc<std::sync::atomic::AtomicU64>) -> Router {
    let state = Arc::new(AppState { redis_url, pool, discarded_ticks });
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_check))
        .route("/ws/market", get(ws_handler))
        .route("/api/settings", get(get_settings_dummy))
        .route("/api/settings/", get(get_settings_dummy))
        .route("/api/settings/:key", put(update_setting).patch(update_setting))
        .route("/api/settings/:key/", put(update_setting).patch(update_setting))
        .route("/api/analytics/performance/", get(get_performance))
        .route("/api/exchanges/", get(dummy_exchanges))
        .route("/api/opportunities/", get(get_opportunities))
        .route("/api/trades", get(get_opportunities))
        .route("/api/trades/", get(get_opportunities))
        .route("/api/wallets/", get(dummy_wallets))
        .route("/api/wallets", get(dummy_wallets))
        .route("/api/wallets/movements/", get(dummy_wallet_movements))
        .route("/api/wallets/movements", get(dummy_wallet_movements))
        .layer(cors)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK - HFT Engine is alive"
}