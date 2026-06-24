// Operaciones SQLx (Insertar Trades, Cargar Configuraciones)

use sqlx::PgPool;

pub async fn save_trade(_pool: &PgPool) -> Result<(), sqlx::Error> {
    // TODO: Insertar log de P&L
    Ok(())
}
