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
