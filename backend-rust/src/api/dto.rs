use serde::Serialize;

#[derive(Serialize)]
pub struct PerformanceDto {
    pub total_pnl_usd: String,
    pub total_fees_usd: String,
    pub total_trades: i64,
    pub discarded_opportunities: i64,
    pub win_rate_percent: String,
}

#[derive(Serialize)]
pub struct OpportunityDto {
    pub id: String,
    pub pair: String,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub gross_margin: f64,
    pub net_profit: f64,
    pub is_partial_fill: bool,
    pub status: String,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct PaginatedOpportunitiesDto {
    pub data: Vec<OpportunityDto>,
    pub total_items: i64,
    pub page: u32,
    pub limit: u32,
}
