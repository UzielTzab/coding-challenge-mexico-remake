mod api;
mod config;
mod db;
mod engine;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar el sistema de logging
    tracing_subscriber::fmt::init();

    tracing::info!("Inicializando Engine HFT de Arbitraje...");

    // 1. Cargar configuración
    let settings = config::settings::Settings::load();

    // 2. Conectar a PostgreSQL asíncronamente
    tracing::info!("Conectando a PostgreSQL...");
    // No conectamos realmente si no está disponible, manejamos el error sutilmente
    // ya que Docker no está encendido en la máquina host según instrucciones
    let pool = match db::establish_postgres_connection(&settings.database_url).await {
        Ok(pool) => {
            tracing::info!("Conexión a PostgreSQL establecida exitosamente.");
            Some(pool)
        }
        Err(e) => {
            tracing::warn!("No se pudo conectar a PostgreSQL (¿Docker apagado?): {}", e);
            None
        }
    };

    // 3. Conectar a Redis
    tracing::info!("Conectando a Redis Pub/Sub...");
    match redis::Client::open(settings.redis_url.clone()) {
        Ok(client) => {
            // Intentamos obtener una conexión asíncrona solo si es posible
            if let Ok(_conn) = client.get_tokio_connection().await {
                tracing::info!("Conexión a Redis establecida exitosamente.");
            } else {
                tracing::warn!("No se pudo establecer la conexión asíncrona a Redis.");
            }
        }
        Err(e) => {
            tracing::warn!("No se pudo parsear la URL de Redis: {}", e);
        }
    }

    // 4. Crear las rutas base de Axum
    let app = api::handlers::router(settings.redis_url.clone(), pool);

    // 5. Iniciar la conexión a Binance y publicación a Redis en background
    let redis_url_for_stream = settings.redis_url.clone();
    tokio::spawn(async move {
        engine::market_stream::run_market_stream(redis_url_for_stream).await;
    });

    // 6. Iniciar el Engine de Rebalanceo (si hay BD)
    if let Some(p) = pool.clone() {
        let redis_url_for_rebalancer = settings.redis_url.clone();
        tokio::spawn(async move {
            let rebalancer = engine::rebalancer::Rebalancer::new(p, redis_url_for_rebalancer);
            rebalancer.run_worker().await;
        });
    }

    // Dirección IP 0.0.0.0 es necesaria para exponer el puerto fuera de Docker
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server_port));
    tracing::info!("Servidor REST/WS (Axum) escuchando en {}", addr);

    // Iniciar el servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
