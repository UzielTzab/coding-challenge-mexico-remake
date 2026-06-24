use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub redis_url: String,
    pub server_port: u16,
}

impl Settings {
    pub fn load() -> Self {
        // Ignoramos el error en caso de que no exista el archivo .env (ej: en Docker usando variables de entorno)
        let _ = dotenv();

        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/hft_db".into()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379/".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8000".into())
                .parse()
                .unwrap_or(8000),
        }
    }
}
