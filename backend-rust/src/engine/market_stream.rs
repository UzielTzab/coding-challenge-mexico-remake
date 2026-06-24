use futures_util::StreamExt;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{error, info};
use url::Url;
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
struct BinanceTicker {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    bid_price: String,
    #[serde(rename = "a")]
    ask_price: String,
    #[serde(rename = "E")]
    event_time: u64,
}

pub async fn run_market_stream(redis_url: String) {
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    let mut redis_conn = match redis_client.get_tokio_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to connect to Redis: {:?}", e);
            return;
        }
    };

    let ws_url = Url::parse("wss://stream.binance.com:9443/ws/btcusdt@ticker").unwrap();

    loop {
        info!("Connecting to Binance WS...");
        match connect_async(ws_url.clone()).await {
            Ok((ws_stream, _)) => {
                info!("Connected to Binance WS");
                let (_, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Ok(ticker) = serde_json::from_str::<BinanceTicker>(&text) {
                                if let (Ok(bid), Ok(ask)) = (
                                    ticker.bid_price.parse::<f64>(),
                                    ticker.ask_price.parse::<f64>(),
                                ) {
                                    let tick = MarketTick {
                                        symbol: ticker.symbol.clone(),
                                        bid,
                                        ask,
                                        timestamp: ticker.event_time,
                                    };
                                    
                                    // Mock a second exchange's tick (e.g., Kraken) with a slight random price difference
                                    let mut mock_tick = tick.clone();
                                    let rand_diff = (rand::random::<f64>() - 0.5) * 10.0; // +/- 5 USD difference
                                    mock_tick.bid += rand_diff;
                                    mock_tick.ask += rand_diff;

                                    let risk_manager = RiskManager::default();
                                    let arbitrage_engine = ArbitrageEngine::new(risk_manager, 2.0); // Threshold of 2.0
                                    
                                    if let Some(result) = arbitrage_engine.detect_spread(&tick, &mock_tick) {
                                        let pnl_event = serde_json::json!({
                                            "type": "PNL",
                                            "symbol": ticker.symbol,
                                            "spread": result.spread,
                                            "net_profit": result.net_profit,
                                            "is_partial_fill": result.is_partial_fill,
                                            "timestamp": ticker.event_time
                                        });
                                        
                                        if let Ok(pnl_json) = serde_json::to_string(&pnl_event) {
                                            if let Err(e) = redis_conn.publish::<_, _, ()>("market:spreads", pnl_json).await {
                                                error!("Failed to publish PNL to Redis: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Ok(Message::Close(_)) => {
                            info!("Binance WebSocket closed, reconnecting...");
                            break;
                        }
                        Err(e) => {
                            error!("Binance WebSocket error: {:?}, reconnecting...", e);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                error!("Failed to connect to Binance: {:?}. Retrying in 5 seconds...", e);
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
