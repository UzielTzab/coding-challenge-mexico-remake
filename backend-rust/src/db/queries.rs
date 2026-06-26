// Operaciones SQLx (Insertar Trades, Cargar Configuraciones)

use sqlx::PgPool;
use super::models::SystemSettings;

pub async fn save_trade(pool: &PgPool, trade: &super::models::Trade) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO trades (id, buy_exchange, sell_exchange, volume_btc, buy_price_usd, sell_price_usd, gross_profit_usd, net_profit_usd, execution_status, latency_ms)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
    )
    .bind(trade.id)
    .bind(&trade.buy_exchange)
    .bind(&trade.sell_exchange)
    .bind(trade.volume_btc)
    .bind(trade.buy_price_usd)
    .bind(trade.sell_price_usd)
    .bind(trade.gross_profit_usd)
    .bind(trade.net_profit_usd)
    .bind(&trade.execution_status)
    .bind(trade.latency_ms)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_trades(pool: &PgPool, page: u32, limit: u32) -> Result<Vec<super::models::Trade>, sqlx::Error> {
    let offset = (page.saturating_sub(1)) * limit;
    let trades = sqlx::query_as::<_, super::models::Trade>(
        "SELECT id, timestamp, buy_exchange, sell_exchange, volume_btc, buy_price_usd, sell_price_usd, gross_profit_usd, net_profit_usd, execution_status, latency_ms FROM trades ORDER BY timestamp DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;
    Ok(trades)
}

pub async fn prune_old_trades(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM trades WHERE timestamp < NOW() - INTERVAL '24 hours'")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_trades_count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM trades")
        .fetch_one(pool)
        .await?;
    Ok(count.0)
}

pub async fn get_performance(pool: &PgPool) -> Result<super::models::TradePerformance, sqlx::Error> {
    let perf = sqlx::query_as::<_, super::models::TradePerformance>(
        "SELECT SUM(net_profit_usd) as total_profit_usd, COUNT(*) as active_trades FROM trades"
    )
    .fetch_one(pool)
    .await?;
    Ok(perf)
}

pub async fn get_settings(pool: &PgPool) -> Result<Vec<SystemSettings>, sqlx::Error> {
    let settings = sqlx::query_as::<_, SystemSettings>(
        "SELECT id, min_net_profit_usd, max_trade_volume_btc, emergency_slippage_penalty_pct, rebalance_threshold_pct, is_bot_active FROM system_settings WHERE id = 1"
    )
    .fetch_all(pool)
    .await?;
    Ok(settings)
}

pub async fn update_setting(pool: &PgPool, is_active: bool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE system_settings SET is_bot_active = $1 WHERE id = 1"
    )
    .bind(is_active)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_wallet_balances(pool: &PgPool) -> Result<Vec<super::models::WalletBalance>, sqlx::Error> {
    let balances = sqlx::query_as::<_, super::models::WalletBalance>(
        "SELECT id, exchange, asset, available_balance, locked_balance FROM wallet_balances"
    )
    .fetch_all(pool)
    .await?;
    Ok(balances)
}

pub async fn update_wallet_balance(
    pool: &PgPool,
    exchange: &str,
    asset: &str,
    new_balance: rust_decimal::Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE wallet_balances SET available_balance = $1 WHERE exchange = $2 AND asset = $3"
    )
    .bind(new_balance)
    .bind(exchange)
    .bind(asset)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn save_rebalance_event(
    pool: &PgPool,
    event: &super::models::RebalanceEvent,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO rebalance_events (id, source_exchange, target_exchange, asset, amount, routing_method, network_fee_usd)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(event.id)
    .bind(&event.source_exchange)
    .bind(&event.target_exchange)
    .bind(&event.asset)
    .bind(event.amount)
    .bind(&event.routing_method)
    .bind(event.network_fee_usd)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_rebalance_events(pool: &PgPool) -> Result<Vec<super::models::RebalanceEvent>, sqlx::Error> {
    let events = sqlx::query_as::<_, super::models::RebalanceEvent>(
        "SELECT id, source_exchange, target_exchange, asset, amount, routing_method, network_fee_usd FROM rebalance_events ORDER BY timestamp DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(events)
}
