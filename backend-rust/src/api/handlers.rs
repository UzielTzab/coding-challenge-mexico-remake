use axum::{
    routing::{get, put},
    Router,
    extract::{State, Path},
    Json,
    http::StatusCode,
};
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
    Router::new()
        .route("/health", get(health_check))
        .route("/ws/market", get(ws_handler))
        .route("/api/settings", get(get_settings))
        .route("/api/settings/:key", put(update_setting))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK - HFT Engine is alive"
}

async fn get_settings(State(state): State<Arc<AppState>>) -> Result<Json<Vec<SystemSettings>>, StatusCode> {
    if let Some(pool) = &state.pool {
        match queries::get_settings(pool).await {
            Ok(settings) => Ok(Json(settings)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateSettingRequest {
    pub value: String,
}

async fn update_setting(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(payload): Json<UpdateSettingRequest>,
) -> Result<StatusCode, StatusCode> {
    if let Some(pool) = &state.pool {
        match queries::update_setting(pool, &key, &payload.value).await {
            Ok(_) => Ok(StatusCode::OK),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
