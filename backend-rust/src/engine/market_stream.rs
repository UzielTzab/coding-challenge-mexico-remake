use futures_util::StreamExt;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{error, info};
use url::Url;

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
                                        symbol: ticker.symbol,
                                        bid,
                                        ask,
                                        timestamp: ticker.event_time,
                                    };
                                    
                                    if let Ok(tick_json) = serde_json::to_string(&tick) {
                                        if let Err(e) = redis_conn.publish::<_, _, ()>("market:spreads", tick_json).await {
                                            error!("Failed to publish to Redis: {:?}", e);
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
