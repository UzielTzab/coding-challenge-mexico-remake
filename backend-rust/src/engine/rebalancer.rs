use crate::db::{models::RebalanceEvent, queries};
use redis::AsyncCommands;
use rust_decimal::prelude::*;
use sqlx::PgPool;
use std::time::Duration;
use tokio::time::sleep;

use std::str::FromStr;

// Lógica de Thresholds y Triangular Routing
// Rebalanceo dinámico de inventario para mantener Neutralidad Delta

pub struct Rebalancer {
    pool: PgPool,
    redis_url: String,
}

impl Rebalancer {
    pub fn new(pool: PgPool, redis_url: String) -> Self {
        Self { pool, redis_url }
    }

    pub async fn run_worker(&self) {
        tracing::info!("Iniciando worker asíncrono de Rebalancer...");
        
        // Conexión a Redis
        let mut redis_conn = match redis::Client::open(self.redis_url.clone()) {
            Ok(client) => match client.get_tokio_connection().await {
                Ok(conn) => Some(conn),
                Err(e) => {
                    tracing::warn!("Rebalancer no pudo conectar asincronamente a Redis: {}", e);
                    None
                }
            },
            Err(e) => {
                tracing::warn!("Rebalancer: URL de Redis inválida: {}", e);
                None
            }
        };

        let btc_threshold = Decimal::from_str("0.1").unwrap();
        let usdt_threshold = Decimal::from_str("1000.0").unwrap();

        loop {
            // Evaluamos la exposición cada 5 segundos
            sleep(Duration::from_secs(5)).await;
            self.evaluate_exposure(&mut redis_conn, btc_threshold, usdt_threshold).await;
        }
    }

    async fn evaluate_exposure(
        &self,
        redis_conn: &mut Option<redis::aio::Connection>,
        btc_threshold: Decimal,
        usdt_threshold: Decimal,
    ) {
        // 1. Obtener saldos
        let balances = match queries::get_wallet_balances(&self.pool).await {
            Ok(b) => b,
            Err(e) => {
                tracing::error!("Error al obtener balances para rebalanceo: {}", e);
                return;
            }
        };

        // 2. Detectar drenaje de liquidez y encontrar contraparte
        let mut drain_detected = false;
        let mut source_exchange = String::new();
        let mut target_exchange = String::new();
        let mut asset_needed = String::new();
        let mut amount_needed = Decimal::ZERO;

        // Por simplicidad, simularemos que si a Kraken le falta USDT, lo traemos de Binance
        // Y si a Binance le falta BTC, lo traemos de Kraken
        for balance in &balances {
            if balance.asset == "BTC" && balance.available_balance < btc_threshold {
                drain_detected = true;
                target_exchange = balance.exchange.clone();
                asset_needed = "BTC".to_string();
                amount_needed = btc_threshold * Decimal::from_str("2").unwrap(); // Traemos el doble del umbral
                source_exchange = if target_exchange == "Binance" { "Kraken".to_string() } else { "Binance".to_string() };
                break;
            } else if balance.asset == "USDT" && balance.available_balance < usdt_threshold {
                drain_detected = true;
                target_exchange = balance.exchange.clone();
                asset_needed = "USDT".to_string();
                amount_needed = usdt_threshold * Decimal::from_str("2").unwrap();
                source_exchange = if target_exchange == "Binance" { "Kraken".to_string() } else { "Binance".to_string() };
                break;
            }
        }

        if drain_detected {
            tracing::warn!(
                "DRENAJE DE LIQUIDEZ DETECTADO en {}. Faltan {}. Iniciando Triangular Routing con XRP...",
                target_exchange,
                asset_needed
            );

            // 3. Simular Triangular Routing con XRP
            // Costo de red simulado y log de evento
            let network_fee_usd = Decimal::from_str("0.25").unwrap(); // 0.25 USD fee
            let event = RebalanceEvent {
                id: uuid::Uuid::new_v4(),
                source_exchange: source_exchange.clone(),
                target_exchange: target_exchange.clone(),
                asset: asset_needed.clone(),
                amount: amount_needed,
                routing_method: "Triangular XRP".to_string(),
                network_fee_usd,
            };

            if let Err(e) = queries::save_rebalance_event(&self.pool, &event).await {
                tracing::error!("No se pudo guardar el evento de rebalanceo: {}", e);
            }

            // 4. Actualizar balances en BD para reflejar la transferencia
            // Deduct from source
            if let Some(source_balance) = balances.iter().find(|b| b.exchange == source_exchange && b.asset == asset_needed) {
                let new_source_balance = source_balance.available_balance - amount_needed;
                let _ = queries::update_wallet_balance(&self.pool, &source_exchange, &asset_needed, new_source_balance).await;
            }
            // Add to target
            if let Some(target_balance) = balances.iter().find(|b| b.exchange == target_exchange && b.asset == asset_needed) {
                let new_target_balance = target_balance.available_balance + amount_needed;
                let _ = queries::update_wallet_balance(&self.pool, &target_exchange, &asset_needed, new_target_balance).await;
            }

            // 5. Emitir evento a Redis para que el Frontend lo consuma
            if let Some(conn) = redis_conn {
                let payload = serde_json::json!({
                    "type": "REBALANCE",
                    "event_id": event.id,
                    "source": event.source_exchange,
                    "target": event.target_exchange,
                    "asset": event.asset,
                    "amount": event.amount,
                    "method": event.routing_method,
                    "fee_usd": event.network_fee_usd,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });
                
                let payload_str = payload.to_string();
                let res: redis::RedisResult<()> = conn.publish("REBALANCE_CHANNEL", payload_str).await;
                if let Err(e) = res {
                    tracing::error!("Error publicando evento de rebalanceo a Redis: {}", e);
                } else {
                    tracing::info!("Evento de rebalanceo emitido a Redis (REBALANCE_CHANNEL).");
                }
            }
        }
    }
}
