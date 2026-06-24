use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::{stream::StreamExt, SinkExt};
use std::sync::Arc;
use tracing::{error, info};

use super::handlers::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.redis_url.clone()))
}

async fn handle_socket(socket: WebSocket, redis_url: String) {
    let (mut sender, mut _receiver) = socket.split();

    let client = match redis::Client::open(redis_url.as_str()) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create Redis client for WS: {:?}", e);
            return;
        }
    };

    let mut pubsub_conn = match client.get_async_connection().await {
        Ok(conn) => conn.into_pubsub(),
        Err(e) => {
            error!("Failed to get Redis pubsub connection: {:?}", e);
            return;
        }
    };

    if let Err(e) = pubsub_conn.subscribe("market:spreads").await {
        error!("Failed to subscribe to market:spreads: {:?}", e);
        return;
    }
    
    if let Err(e) = pubsub_conn.subscribe("REBALANCE_CHANNEL").await {
        error!("Failed to subscribe to REBALANCE_CHANNEL: {:?}", e);
    }

    info!("Client connected to WS and subscribed to market:spreads and REBALANCE_CHANNEL");

    let mut pubsub_stream = pubsub_conn.into_on_message();

    while let Some(msg) = pubsub_stream.next().await {
        let payload: String = match msg.get_payload() {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Err(e) = sender.send(Message::Text(payload)).await {
            error!("Error sending to WS client: {:?}", e);
            break;
        }
    }

    info!("WS client disconnected");
}
