// Operaciones SQLx (Insertar Trades, Cargar Configuraciones)

use sqlx::PgPool;
use super::models::SystemSettings;

pub async fn save_trade(_pool: &PgPool) -> Result<(), sqlx::Error> {
    // TODO: Insertar log de P&L
    Ok(())
}

pub async fn get_settings(pool: &PgPool) -> Result<Vec<SystemSettings>, sqlx::Error> {
    let settings = sqlx::query_as!(
        SystemSettings,
        "SELECT id, key, value FROM system_settings"
    )
    .fetch_all(pool)
    .await?;
    Ok(settings)
}

pub async fn update_setting(pool: &PgPool, key: &str, value: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE system_settings SET value = $1 WHERE key = $2",
        value,
        key
    )
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
