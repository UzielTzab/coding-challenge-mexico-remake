use futures_util::{StreamExt, SinkExt};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::info;
use url::Url;
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::PgPool;
use rust_decimal::prelude::FromPrimitive;

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
    last_trade_time: Option<u64>,
}

pub async fn run_market_stream(redis_url: String, pool: Option<PgPool>, discarded_ticks: Arc<std::sync::atomic::AtomicU64>) {
    info!("Iniciando WebSocket HFT hacia Binance y Kraken...");

    // Canales MPMC (multi-producer multi-consumer) internos
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<MarketTick>();

    let state = Arc::new(RwLock::new(SharedState {
        binance_tick: None,
        kraken_tick: None,
        last_trade_time: None,
    }));

    let redis_url_clone = redis_url.clone();
    let state_binance = state.clone();
    let pool_binance = pool.clone();
    let discarded_ticks_binance = discarded_ticks.clone();

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

                                check_arbitrage(state_binance.clone(), &mut redis_conn, pool_binance.clone(), discarded_ticks_binance.clone()).await;
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
    let discarded_ticks_kraken = discarded_ticks.clone();

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

                                    check_arbitrage(state_kraken.clone(), &mut redis_conn, pool_kraken.clone(), discarded_ticks_kraken.clone()).await;
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
    discarded_ticks: Arc<std::sync::atomic::AtomicU64>,
) {
    let (binance_tick, kraken_tick) = {
        let r = state.read().await;
        if r.binance_tick.is_none() || r.kraken_tick.is_none() {
            return;
        }
        (r.binance_tick.clone().unwrap(), r.kraken_tick.clone().unwrap())
    };

    let risk_manager = RiskManager::new(0.05, 0.01);
    let arbitrage_engine = ArbitrageEngine::new(risk_manager, 2.0);
    
    if let Some(result) = arbitrage_engine.detect_spread(&binance_tick, &kraken_tick) {
        {
            let mut w = state.write().await;
            let now = chrono::Utc::now().timestamp_millis() as u64;
            if let Some(last_time) = w.last_trade_time {
                if now - last_time < 2000 {
                    // PHANTOM LIQUIDITY PROTECTION: 2 seconds cooldown
                    return; 
                }
            }
            w.last_trade_time = Some(now);
        }

        let mut opp_status = if result.is_partial_fill { 
            "emergency_hedge" 
        } else if result.is_legging_hedge {
            "legging_hedge"
        } else { 
            "executed" 
        };


        let mut is_active = false;
        if let Some(p) = &pool {
            if let Ok(settings) = crate::db::queries::get_settings(p).await {
                for s in settings {
                    if s.is_bot_active {
                        is_active = true;
                    }
                }
            }
        }

        if !is_active {
            opp_status = "discarded";
            // Incrementamos ticks descartados porque el bot está en pausa pero evaluó un tick
            discarded_ticks.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        
        // REBALANCEO TRIANGULAR (Ejecución Simulada con DB) Y DELTA NEUTRALITY
        if opp_status == "executed" {
            if let Some(p) = &pool {
                if is_active {
                    // Leer saldos
                    if let Ok(balances) = crate::db::queries::get_wallet_balances(p).await {
                        let mut binance_btc = rust_decimal::Decimal::new(0, 0);
                        let mut kraken_usdt = rust_decimal::Decimal::new(0, 0);
                        let mut kraken_btc = rust_decimal::Decimal::new(0, 0);

                        for b in &balances {
                            if b.exchange == "Binance" && b.asset == "BTC" {
                                binance_btc = b.available_balance;
                            }
                            if b.exchange == "Kraken" && b.asset == "USDT" {
                                kraken_usdt = b.available_balance;
                            }
                            if b.exchange == "Kraken" && b.asset == "BTC" {
                                kraken_btc = b.available_balance;
                            }
                        }

                        use std::str::FromStr;
                        
                        // DELTA NEUTRALITY HEDGE
                        let net_btc = binance_btc + kraken_btc;
                        if net_btc > rust_decimal::Decimal::from_str("2.0").unwrap() {
                            let evt = serde_json::json!({
                                "type": "DELTA_HEDGE",
                                "asset": "BTC",
                                "net_exposure": net_btc,
                                "message": "High Net BTC Exposure. Simulating Perpetual Short Position."
                            });
                            if let Ok(json) = serde_json::to_string(&evt) {
                                let _ = redis_conn.publish::<_, _, ()>("market:spreads", json).await;
                            }
                        }

                        // REBALANCEO TRIANGULAR
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

        // SAVE TRADE TO DB
        if opp_status == "executed" || opp_status == "emergency_hedge" || opp_status == "legging_hedge" {
            if let Some(p) = &pool {
                use std::str::FromStr;
                let trade = crate::db::models::Trade {
                    id: uuid::Uuid::new_v4(),
                    timestamp: chrono::Utc::now().naive_utc(),
                    buy_exchange: if binance_tick.ask < kraken_tick.ask { "Binance".to_string() } else { "Kraken".to_string() },
                    sell_exchange: if binance_tick.bid > kraken_tick.bid { "Binance".to_string() } else { "Kraken".to_string() },
                    volume_btc: rust_decimal::Decimal::from_str("0.01").unwrap(), // Dummy volume
                    buy_price_usd: rust_decimal::Decimal::from_f64(if binance_tick.ask < kraken_tick.ask { binance_tick.ask } else { kraken_tick.ask }).unwrap_or_default(),
                    sell_price_usd: rust_decimal::Decimal::from_f64(if binance_tick.bid > kraken_tick.bid { binance_tick.bid } else { kraken_tick.bid }).unwrap_or_default(),
                    gross_profit_usd: rust_decimal::Decimal::from_f64(result.spread).unwrap_or_default(),
                    net_profit_usd: rust_decimal::Decimal::from_f64(result.net_profit).unwrap_or_default(),
                    execution_status: opp_status.to_string(),
                    latency_ms: 15,
                };
                let _ = crate::db::queries::save_trade(p, &trade).await;
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
                "is_legging_hedge": result.is_legging_hedge,
                "order_type": "IOC",
                "status": opp_status,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });
        
        if let Ok(opp_json) = serde_json::to_string(&opp_event) {
            let _ = redis_conn.publish::<_, _, ()>("market:spreads", opp_json).await;
        }
    } else {
        // El spread no fue rentable (ArbitrageResult == None)
        discarded_ticks.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binance_json_parse_success() {
        let json = r#"{"s":"BTCUSDT","b":"60100.50","a":"60101.50"}"#;
        let ticker: Result<BinanceBookTicker, _> = serde_json::from_str(json);
        assert!(ticker.is_ok());
        let t = ticker.unwrap();
        assert_eq!(t.symbol, "BTCUSDT");
        assert_eq!(t.bid_price, "60100.50");
        assert_eq!(t.ask_price, "60101.50");
    }

    #[test]
    fn test_kraken_json_parse_success() {
        let json = r#"{"channel":"ticker","type":"update","data":[{"bid":60100.50,"ask":60101.50}]}"#;
        let msg: Result<KrakenTickerMessage, _> = serde_json::from_str(json);
        assert!(msg.is_ok());
        let m = msg.unwrap();
        assert_eq!(m.channel, "ticker");
        assert_eq!(m.msg_type, "update");
        assert_eq!(m.data.len(), 1);
        assert_eq!(m.data[0].bid, 60100.50);
        assert_eq!(m.data[0].ask, 60101.50);
    }
}
