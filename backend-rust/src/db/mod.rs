pub mod models;
pub mod queries;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn establish_postgres_connection(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(50)
        .connect(database_url)
        .await
}
