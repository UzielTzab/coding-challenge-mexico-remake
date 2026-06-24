// Estructuras de tablas (P&L Histórico, Settings Editables por API)

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TradeInfo {
    pub id: uuid::Uuid,
    pub symbol: String,
    pub pnl: rust_decimal::Decimal,
    // Otras propiedades
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
