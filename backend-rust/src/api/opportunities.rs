use axum::{extract::{State, Query}, Json};
use std::sync::Arc;
use crate::api::handlers::AppState;
use crate::db::queries;
use crate::api::dto::{PerformanceDto, OpportunityDto, PaginatedOpportunitiesDto, PaginationQuery, VariationsDto};

pub async fn get_performance(State(state): State<Arc<AppState>>) -> Json<Vec<PerformanceDto>> {
    if let Some(pool) = &state.pool {
        if let Ok(perf) = queries::get_performance(pool).await {
            use std::str::FromStr;
            let profit = perf.total_pnl_usd.map(|d| f64::from_str(&d.to_string()).unwrap_or(0.0)).unwrap_or(0.0);
            let fees = perf.total_fees_usd.map(|d| f64::from_str(&d.to_string()).unwrap_or(0.0)).unwrap_or(0.0);
            let trades = perf.total_trades.unwrap_or(0);
            let db_discarded = perf.discarded_opportunities.unwrap_or(0);
            let mem_discarded = state.discarded_ticks.load(std::sync::atomic::Ordering::Relaxed) as i64;
            let discarded = db_discarded + mem_discarded;
            
            let volume = perf.total_volume_btc.map(|d| f64::from_str(&d.to_string()).unwrap_or(0.0)).unwrap_or(0.0);
            let spread = perf.average_spread_pct.map(|d| f64::from_str(&d.to_string()).unwrap_or(0.0)).unwrap_or(0.0);

            // Calculamos el win rate real
            let total_ops = trades + discarded;
            let win_rate = if total_ops > 0 { 
                (trades as f64 / total_ops as f64) * 100.0 
            } else { 0.0 };
            
            // Simulamos variaciones diarias (lógica mock hasta que exista historial multidiario real en BD)
            let var_pnl = if profit > 0.0 { 12.4 } else { 0.0 }; 
            let var_win_rate = 0.01;
            let var_trades = 8;
            let var_cost = -3.2;
            let var_btc_price = 1.1;

            return Json(vec![PerformanceDto {
                total_pnl_usd: profit.to_string(),
                total_fees_usd: fees.to_string(),
                total_trades: trades,
                discarded_opportunities: discarded,
                win_rate_percent: win_rate.to_string(),
                total_volume_btc: volume.to_string(),
                average_spread_pct: spread.to_string(),
                variations: VariationsDto {
                    pnl: var_pnl,
                    win_rate: var_win_rate,
                    trades: var_trades,
                    cost: var_cost,
                    btc_price: var_btc_price,
                }
            }]);
        }
    }
    Json(vec![PerformanceDto {
        total_pnl_usd: "0.0".to_string(),
        total_fees_usd: "0.0".to_string(),
        total_trades: 0,
        discarded_opportunities: 0,
        win_rate_percent: "0.0".to_string(),
        total_volume_btc: "0.0".to_string(),
        average_spread_pct: "0.0".to_string(),
        variations: VariationsDto { pnl: 0.0, win_rate: 0.0, trades: 0, cost: 0.0, btc_price: 0.0 },
    }])
}

pub async fn get_opportunities(State(state): State<Arc<AppState>>, Query(query): Query<PaginationQuery>,) -> Json<PaginatedOpportunitiesDto> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    if let Some(pool) = &state.pool {
        if let Ok(trades) = queries::get_trades(pool, page, limit).await {
            let mut results = vec![];
            for t in trades {
                use std::str::FromStr;
                results.push(OpportunityDto {
                    id: t.id.to_string(),
                    pair: "BTC/USDT".to_string(),
                    buy_exchange: t.buy_exchange,
                    sell_exchange: t.sell_exchange,
                    buy_price: f64::from_str(&t.buy_price_usd.to_string()).unwrap_or(0.0),
                    sell_price: f64::from_str(&t.sell_price_usd.to_string()).unwrap_or(0.0),
                    gross_margin: f64::from_str(&t.gross_profit_usd.to_string()).unwrap_or(0.0),
                    net_profit: f64::from_str(&t.net_profit_usd.to_string()).unwrap_or(0.0),
                    is_partial_fill: t.execution_status == "emergency_hedge",
                    status: t.execution_status,
                    timestamp: t.timestamp.and_utc().to_rfc3339()
                });
            }
            
            let total_items = queries::get_trades_count(pool).await.unwrap_or(0);

            return Json(PaginatedOpportunitiesDto {
                data: results,
                total_items,
                page,
                limit,
            });
        }
    }
    Json(PaginatedOpportunitiesDto {
        data: vec![],
        total_items: 0,
        page,
        limit,
    })
}