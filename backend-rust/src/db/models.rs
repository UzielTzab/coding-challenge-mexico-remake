// Estructuras de tablas (P&L Histórico, Settings Editables por API)

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TradeInfo {
    pub id: uuid::Uuid,
    pub symbol: String,
    pub pnl: rust_decimal::Decimal,
    // Otras propiedades
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SystemSettings {
    pub id: i32,
    pub key: String,
    pub value: String,
}
