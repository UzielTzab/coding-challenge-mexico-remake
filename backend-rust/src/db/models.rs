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
    pub key: String,
    pub value: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct WalletBalance {
    pub id: i32,
    pub exchange: String,
    pub asset: String,
    pub available_balance: rust_decimal::Decimal,
    pub locked_balance: rust_decimal::Decimal,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RebalanceEvent {
    pub id: uuid::Uuid,
    pub source_exchange: String,
    pub target_exchange: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub routing_method: String,
    pub network_fee_usd: rust_decimal::Decimal,
}
