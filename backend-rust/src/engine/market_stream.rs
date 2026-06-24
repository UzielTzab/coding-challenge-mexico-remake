use futures_util::{StreamExt, SinkExt};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{error, info};
use url::Url;
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::PgPool;

use crate::engine::arbitrage::ArbitrageEngine;
use crate::engine::risk::RiskManager;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketTick {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct BinanceBookTicker {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    bid_price: String,
    #[serde(rename = "a")]
    ask_price: String,
}

#[derive(Debug, Deserialize)]
struct KrakenTickerData {
    symbol: String,
    bid: f64,
    ask: f64,
}

#[derive(Debug, Deserialize)]
struct KrakenTickerMessage {
    channel: String,
    #[serde(rename = "type")]
    msg_type: String,
    data: Vec<KrakenTickerData>,
}

#[derive(Clone)]
struct SharedState {
    binance_tick: Option<MarketTick>,
    kraken_tick: Option<MarketTick>,
}

pub async fn run_market_stream(redis_url: String, pool: Option<PgPool>) {
    let state = Arc::new(RwLock::new(SharedState {
        binance_tick: None,
        kraken_tick: None,
    }));

    let redis_url_clone = redis_url.clone();
    let state_binance = state.clone();
    let pool_binance = pool.clone();

    // Tarea 1: Binance
    tokio::spawn(async move {
        let redis_client = redis::Client::open(redis_url_clone).expect("Failed to create Redis client");
        let mut redis_conn = redis_client.get_tokio_connection().await.unwrap();
        let ws_url = Url::parse("wss://stream.binance.com:9443/ws/btcusdt@bookTicker").unwrap();

        loop {
            info!("Connecting to Binance WS...");
            if let Ok((ws_stream, _)) = connect_async(ws_url.clone()).await {
                info!("Connected to Binance WS");
                let (_, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        if let Ok(ticker) = serde_json::from_str::<BinanceBookTicker>(&text) {
                            if let (Ok(bid), Ok(ask)) = (
                                ticker.bid_price.parse::<f64>(),
                                ticker.ask_price.parse::<f64>(),
                            ) {
                                let tick = MarketTick {
                                    symbol: ticker.symbol.clone(),
                                    bid,
                                    ask,
                                    timestamp: chrono::Utc::now().timestamp_millis() as u64,
                                };
                                
                                {
                                    let mut w = state_binance.write().await;
                                    w.binance_tick = Some(tick.clone());
                                }
                                
                                let binance_update = serde_json::json!({
                                    "type": "market_update",
                                    "exchange": "Binance",
                                    "symbol": ticker.symbol,
                                    "best_bid": tick.bid,
                                    "best_ask": tick.ask,
                                    "bid_volume": 1.5,
                                    "ask_volume": 1.2,
                                    "latency_ms": 12
                                });
                                if let Ok(j) = serde_json::to_string(&binance_update) {
                                    let _ = redis_conn.publish::<_, _, ()>("market:spreads", j).await;
                                }

                                check_arbitrage(state_binance.clone(), &mut redis_conn, pool_binance.clone()).await;
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    let state_kraken = state.clone();
    let pool_kraken = pool.clone();

    // Tarea 2: Kraken
    tokio::spawn(async move {
        let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");
        let mut redis_conn = redis_client.get_tokio_connection().await.unwrap();
        let ws_url = Url::parse("wss://ws.kraken.com/v2").unwrap();

        loop {
            info!("Connecting to Kraken WS...");
            if let Ok((ws_stream, _)) = connect_async(ws_url.clone()).await {
                info!("Connected to Kraken WS");
                let (mut write, mut read) = ws_stream.split();
                
                // Suscribirse
                let sub_msg = serde_json::json!({
                    "method": "subscribe",
                    "params": {
                        "channel": "ticker",
                        "symbol": ["BTC/USD"]
                    }
                });
                let _ = write.send(Message::Text(sub_msg.to_string())).await;

                while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        if let Ok(message) = serde_json::from_str::<KrakenTickerMessage>(&text) {
                            if message.channel == "ticker" && message.msg_type == "update" {
                                if let Some(ticker) = message.data.first() {
                                    let tick = MarketTick {
                                        symbol: "BTCUSDT".to_string(), // normalizar
                                        bid: ticker.bid,
                                        ask: ticker.ask,
                                        timestamp: chrono::Utc::now().timestamp_millis() as u64,
                                    };
                                    
                                    {
                                        let mut w = state_kraken.write().await;
                                        w.kraken_tick = Some(tick.clone());
                                    }
                                    
                                    let kraken_update = serde_json::json!({
                                        "type": "market_update",
                                        "exchange": "Kraken",
                                        "symbol": "BTCUSDT",
                                        "best_bid": tick.bid,
                                        "best_ask": tick.ask,
                                        "bid_volume": 0.8,
                                        "ask_volume": 1.1,
                                        "latency_ms": 15
                                    });
                                    if let Ok(j) = serde_json::to_string(&kraken_update) {
                                        let _ = redis_conn.publish::<_, _, ()>("market:spreads", j).await;
                                    }

                                    check_arbitrage(state_kraken.clone(), &mut redis_conn, pool_kraken.clone()).await;
                                }
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    // Mantener la tarea principal viva
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

async fn check_arbitrage(
    state: Arc<RwLock<SharedState>>,
    redis_conn: &mut redis::aio::Connection,
    pool: Option<PgPool>,
) {
    let (binance_tick, kraken_tick) = {
        let r = state.read().await;
        if r.binance_tick.is_none() || r.kraken_tick.is_none() {
            return;
        }
        (r.binance_tick.clone().unwrap(), r.kraken_tick.clone().unwrap())
    };

    let risk_manager = RiskManager::default();
    let arbitrage_engine = ArbitrageEngine::new(risk_manager, 2.0);
    
    if let Some(result) = arbitrage_engine.detect_spread(&binance_tick, &kraken_tick) {
        let opp_status = if result.is_partial_fill { "emergency_hedge" } else { "executed" };
        
        // REBALANCEO TRIANGULAR (Ejecución Simulada con DB)
        if opp_status == "executed" {
            if let Some(p) = &pool {
                // Verificar si el bot está activo y saldos
                if let Ok(settings) = crate::db::queries::get_settings(p).await {
                    let mut is_active = false;
                    for s in settings {
                        if s.is_bot_active {
                            is_active = true;
                        }
                    }

                    if is_active {
                        // Leer saldo binance btc
                        if let Ok(balances) = crate::db::queries::get_wallet_balances(p).await {
                            let mut binance_btc = rust_decimal::Decimal::new(0, 0);
                            let mut kraken_usdt = rust_decimal::Decimal::new(0, 0);

                            for b in &balances {
                                if b.exchange == "Binance" && b.asset == "BTC" {
                                    binance_btc = b.available_balance;
                                }
                                if b.exchange == "Kraken" && b.asset == "USDT" {
                                    kraken_usdt = b.available_balance;
                                }
                            }

                            use std::str::FromStr;
                            let threshold = rust_decimal::Decimal::from_str("0.1").unwrap();

                            if binance_btc < threshold {
                                // Ejecutar rebalanceo: Restar 100 USDT de kraken, sumar 0.05 BTC a Binance
                                let new_kraken_usdt = kraken_usdt - rust_decimal::Decimal::from_str("100.0").unwrap();
                                let new_binance_btc = binance_btc + rust_decimal::Decimal::from_str("0.05").unwrap();

                                let _ = crate::db::queries::update_wallet_balance(p, "Kraken", "USDT", new_kraken_usdt).await;
                                let _ = crate::db::queries::update_wallet_balance(p, "Binance", "BTC", new_binance_btc).await;

                                let evt = crate::db::models::RebalanceEvent {
                                    id: uuid::Uuid::new_v4(),
                                    source_exchange: "Kraken".to_string(),
                                    target_exchange: "Binance".to_string(),
                                    asset: "USDT_TO_BTC".to_string(),
                                    amount: rust_decimal::Decimal::from_str("100.0").unwrap(),
                                    routing_method: "XRP_BRIDGE".to_string(),
                                    network_fee_usd: rust_decimal::Decimal::from_str("0.25").unwrap(),
                                };
                                let _ = crate::db::queries::save_rebalance_event(p, &evt).await;
                                info!("Triangular Rebalance Executed! XRP_BRIDGE used.");
                            }
                        }
                    }
                }
            }
        }

        let opp_event = serde_json::json!({
            "type": "opportunity_detected",
            "opportunity": {
                "id": uuid::Uuid::new_v4().to_string(),
                "pair": binance_tick.symbol,
                "buy_exchange": if binance_tick.ask < kraken_tick.ask { "Binance" } else { "Kraken" },
                "sell_exchange": if binance_tick.bid > kraken_tick.bid { "Binance" } else { "Kraken" },
                "buy_price": if binance_tick.ask < kraken_tick.ask { binance_tick.ask } else { kraken_tick.ask },
                "sell_price": if binance_tick.bid > kraken_tick.bid { binance_tick.bid } else { kraken_tick.bid },
                "gross_margin": result.spread,
                "net_profit": result.net_profit,
                "is_partial_fill": result.is_partial_fill,
                "status": opp_status,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });
        
        if let Ok(opp_json) = serde_json::to_string(&opp_event) {
            let _ = redis_conn.publish::<_, _, ()>("market:spreads", opp_json).await;
        }
    }
}
