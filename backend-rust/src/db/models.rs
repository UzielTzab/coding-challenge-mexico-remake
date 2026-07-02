// Estructuras de tablas (P&L Histórico, Settings Editables por API)

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Trade {
    pub id: uuid::Uuid,
    pub timestamp: chrono::NaiveDateTime,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub volume_btc: rust_decimal::Decimal,
    pub buy_price_usd: rust_decimal::Decimal,
    pub sell_price_usd: rust_decimal::Decimal,
    pub gross_profit_usd: rust_decimal::Decimal,
    pub net_profit_usd: rust_decimal::Decimal,
    pub execution_status: String,
    pub latency_ms: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TradePerformance {
    pub total_pnl_usd: Option<rust_decimal::Decimal>,
    pub total_trades: Option<i64>,
    pub discarded_opportunities: Option<i64>,
    pub total_fees_usd: Option<rust_decimal::Decimal>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct SystemSettings {
    pub id: i32,
    pub min_net_profit_usd: rust_decimal::Decimal,
    pub max_trade_volume_btc: rust_decimal::Decimal,
    pub emergency_slippage_penalty_pct: rust_decimal::Decimal,
    pub rebalance_threshold_pct: rust_decimal::Decimal,
    pub is_bot_active: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct WalletBalance {
    pub id: i32,
    pub exchange: String,
    pub asset: String,
    pub available_balance: rust_decimal::Decimal,
    pub locked_balance: rust_decimal::Decimal,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct RebalanceEvent {
    pub id: uuid::Uuid,
    pub source_exchange: String,
    pub target_exchange: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub routing_method: String,
    pub network_fee_usd: rust_decimal::Decimal,
}
