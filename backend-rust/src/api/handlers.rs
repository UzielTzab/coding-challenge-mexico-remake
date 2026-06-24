use axum::{
    routing::{get, put, patch},
    Router,
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;
use sqlx::PgPool;
use super::ws::ws_handler;
use crate::db::{models::SystemSettings, queries};

pub struct AppState {
    pub redis_url: String,
    pub pool: Option<PgPool>,
}

pub fn router(redis_url: String, pool: Option<PgPool>) -> Router {
    let state = Arc::new(AppState { redis_url, pool });
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
        .route("/api/analytics/performance/", get(dummy_performance))
        .route("/api/exchanges/", get(dummy_exchanges))
        .route("/api/opportunities/", get(dummy_opportunities))
        .route("/api/trades", get(dummy_opportunities))
        .route("/api/trades/", get(dummy_opportunities))
        .layer(cors)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK - HFT Engine is alive"
}

async fn get_settings_dummy() -> Json<serde_json::Value> {
    Json(serde_json::json!([{
        "id": "demo-config",
        "min_net_profit_usd": 5.0,
        "max_trade_volume_btc": 0.01,
        "emergency_slippage_penalty_pct": 2.0,
        "rebalance_threshold_pct": 20.0,
        "is_running": false
    }]))
}

async fn dummy_performance() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "total_profit_usd": 1540.50,
        "active_trades": 3,
        "success_rate": 95.5
    }))
}

async fn dummy_exchanges() -> Json<serde_json::Value> {
    Json(serde_json::json!([]))
}

async fn dummy_opportunities() -> Json<serde_json::Value> {
    Json(serde_json::json!([]))
}

#[derive(serde::Deserialize)]
pub struct UpdateSettingRequest {
    pub is_running: Option<bool>,
    pub value: Option<String>,
}

async fn update_setting(
    Path(_key): Path<String>,
    Json(_payload): Json<UpdateSettingRequest>,
) -> Result<StatusCode, StatusCode> {
    // Simular guardado exitoso para la demo
    Ok(StatusCode::OK)
}
