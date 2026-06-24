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

async fn get_settings_dummy(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
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

async fn dummy_wallets(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    if let Some(pool) = &state.pool {
        if let Ok(balances) = queries::get_wallet_balances(pool).await {
            use std::collections::HashMap;
            let mut map: HashMap<String, (rust_decimal::Decimal, rust_decimal::Decimal)> = HashMap::new();
            for b in balances {
                let e = map.entry(b.exchange.clone()).or_insert((rust_decimal::Decimal::new(0, 0), rust_decimal::Decimal::new(0, 0)));
                if b.asset == "BTC" {
                    e.0 = b.available_balance;
                } else if b.asset == "USDT" {
                    e.1 = b.available_balance;
                }
            }
            
            let mut results = vec![];
            for (exchange, (btc, usdt)) in map {
                use std::str::FromStr;
                let btc_f = f64::from_str(&btc.to_string()).unwrap_or(0.0);
                let usdt_f = f64::from_str(&usdt.to_string()).unwrap_or(0.0);
                results.push(serde_json::json!({
                    "exchange": exchange,
                    "btc_available": btc_f,
                    "usdt_available": usdt_f,
                    "total_value_usd": usdt_f + (btc_f * 60000.0) // Estimado simple
                }));
            }
            if !results.is_empty() {
                return Json(serde_json::json!(results));
            }
        }
    }
    Json(serde_json::json!([]))
}

async fn dummy_wallet_movements(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    if let Some(pool) = &state.pool {
        if let Ok(events) = queries::get_rebalance_events(pool).await {
            let mut results = vec![];
            for e in events {
                results.push(serde_json::json!({
                    "id": e.id.to_string(),
                    "movement_type": "Triangular Rebalance",
                    "asset": e.asset,
                    "amount": e.amount,
                    "source_exchange": e.source_exchange,
                    "target_exchange": e.target_exchange,
                    "network_fee_usd": e.network_fee_usd,
                    "created_at": chrono::Utc::now().to_rfc3339() // Simplificación
                }));
            }
            return Json(serde_json::json!({
                "count": results.len(),
                "results": results
            }));
        }
    }
    Json(serde_json::json!({
        "count": 0,
        "results": []
    }))
}

#[derive(serde::Deserialize)]
pub struct UpdateSettingRequest {
    pub is_running: Option<bool>,
    pub value: Option<String>,
}

async fn update_setting(
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
