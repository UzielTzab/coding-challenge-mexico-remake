use axum::{extract::{State}, Json};
use std::sync::Arc;
use crate::api::handlers::AppState;
use crate::db::queries;

pub async fn dummy_exchanges() -> Json<serde_json::Value> {
    Json(serde_json::json!([]))
}

pub async fn dummy_wallets(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
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

pub async fn dummy_wallet_movements(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    if let Some(pool) = &state.pool {
        if let Ok(events) = queries::get_rebalance_events(pool).await {
            let mut results = vec![];
            for e in events {
                results.push(serde_json::json!({
                    "id": e.id.to_string(),
                    "movement_type": "Triangular Rebalance",
                    "asset": e.asset,
                    "amount": e.amount,
                    "exchange": e.target_exchange.clone(),
                    "source_exchange": e.source_exchange,
                    "target_exchange": e.target_exchange,
                    "network_fee_usd": e.network_fee_usd,
                    "created_at": chrono::Utc::now().to_rfc3339() // Simplificación
                }));
            }
            if !results.is_empty() {
                return Json(serde_json::json!({
                    "count": results.len(),
                    "results": results
                }));
            }
        }
    }
    
    // Fallback: Mock data if empty
    let mock_results = vec![
        serde_json::json!({
            "id": "1",
            "movement_type": "Rebalance In",
            "asset": "BTC",
            "amount": "0.50",
            "exchange": "binance",
            "created_at": chrono::Utc::now().to_rfc3339()
        }),
        serde_json::json!({
            "id": "2",
            "movement_type": "Rebalance Out",
            "asset": "BTC",
            "amount": "-0.50",
            "exchange": "kraken",
            "created_at": (chrono::Utc::now() - chrono::Duration::minutes(5)).to_rfc3339()
        }),
        serde_json::json!({
            "id": "3",
            "movement_type": "Fee Deduction",
            "asset": "USDT",
            "amount": "-1.50",
            "exchange": "binance",
            "created_at": (chrono::Utc::now() - chrono::Duration::hours(2)).to_rfc3339()
        })
    ];

    Json(serde_json::json!({
        "count": mock_results.len(),
        "results": mock_results
    }))
}