use axum::{extract::{State, Path}, Json, http::StatusCode};
use std::sync::Arc;
use crate::api::handlers::AppState;
use crate::db::queries;

pub async fn get_settings_dummy(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    if let Some(pool) = &state.pool {
        if let Ok(settings) = queries::get_settings(pool).await {
            if let Some(s) = settings.first() {
                return Json(serde_json::json!([{
                    "id": "demo-config",
                    "min_net_profit_usd": s.min_net_profit_usd,
                    "max_trade_volume_btc": s.max_trade_volume_btc,
                    "emergency_slippage_penalty_pct": s.emergency_slippage_penalty_pct,
                    "rebalance_threshold_pct": s.rebalance_threshold_pct,
                    "is_running": s.is_bot_active
                }]));
            }
        }
    }
    Json(serde_json::json!([{
        "id": "demo-config",
        "min_net_profit_usd": 5.0,
        "max_trade_volume_btc": 0.01,
        "emergency_slippage_penalty_pct": 2.0,
        "rebalance_threshold_pct": 20.0,
        "is_running": false
    }]))
}

#[derive(serde::Deserialize)]
pub struct UpdateSettingRequest {
    pub is_running: Option<bool>,
}

pub async fn update_setting(
    State(state): State<Arc<AppState>>,
    Path(_key): Path<String>,
    Json(payload): Json<UpdateSettingRequest>,
) -> Result<StatusCode, StatusCode> {
    if let Some(is_running) = payload.is_running {
        if let Some(pool) = &state.pool {
            let _ = queries::update_setting(pool, is_running).await;
        }
    }
    Ok(StatusCode::OK)
}