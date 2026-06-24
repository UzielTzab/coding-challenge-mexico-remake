use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Inicializar el sistema de logging
    tracing_subscriber::fmt::init();

    // Crear las rutas base de Axum
    let app = Router::new()
        .route("/health", get(health_check));

    // Dirección IP 0.0.0.0 es necesaria para exponer el puerto fuera de Docker
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("Engine de Arbitraje (Rust) escuchando en {}", addr);

    // Iniciar el servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Endpoint básico de prueba
async fn health_check() -> &'static str {
    "OK - Engine is running"
}
