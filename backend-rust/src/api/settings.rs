use axum::{extract::{State, Path}, Json, http::StatusCode};
use std::sync::Arc;
use crate::api::handlers::AppState;
use crate::db::queries;

pub async fn get_settings_dummy(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let config = {
        if let Ok(c) = state.dyn_config.read() {
            c.clone()
        } else {
            crate::api::handlers::DynamicConfig::default()
        }
    };

    Json(serde_json::json!({
        "id": "demo-config",
        "min_spread_percent": config.min_spread_usd / 10.0, // Reverse dummy conversion
        "max_trade_usd": config.max_trade_volume_btc * 60000.0,
        "strategy_type": "triangular",
        "daily_loss_limit": 50,
        "circuit_breaker_pause": 15,
        "latency_alert_ms": 200,
        "is_running": config.is_running,
        "fees": [
            { "exchange": "binance", "maker_fee": config.binance_fee_pct, "taker_fee": config.binance_fee_pct, "withdrawal_fee": 10 },
            { "exchange": "kraken", "maker_fee": config.kraken_fee_pct, "taker_fee": config.kraken_fee_pct, "withdrawal_fee": 15 }
        ]
    }))
}

#[derive(serde::Deserialize, Debug)]
pub struct FeeSetting {
    pub exchange: String,
    pub taker_fee: f64,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateSettingRequest {
    pub is_running: Option<bool>,
    pub min_spread_percent: Option<f64>,
    pub max_trade_usd: Option<f64>,
    pub fees: Option<Vec<FeeSetting>>,
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

    if let Ok(mut config) = state.dyn_config.write() {
        if let Some(r) = payload.is_running { config.is_running = r; }
        if let Some(spread_pct) = payload.min_spread_percent {
            config.min_spread_usd = spread_pct * 10.0;
        }
        if let Some(vol) = payload.max_trade_usd {
            config.max_trade_volume_btc = vol / 60000.0;
        }
        if let Some(fees) = payload.fees {
            for f in fees {
                if f.exchange.to_lowercase() == "binance" {
                    config.binance_fee_pct = f.taker_fee;
                } else if f.exchange.to_lowercase() == "kraken" {
                    config.kraken_fee_pct = f.taker_fee;
                }
            }
        }
    }

    Ok(StatusCode::OK)
}