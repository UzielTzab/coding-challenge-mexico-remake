use crate::engine::market_stream::MarketTick;
use crate::engine::risk::RiskManager;

pub struct ArbitrageEngine {
    pub risk_manager: RiskManager,
    pub threshold: f64,
}

pub struct ArbitrageResult {
    pub spread: f64,
    pub net_profit: f64,
    pub is_partial_fill: bool,
}

impl ArbitrageEngine {
    pub fn new(risk_manager: RiskManager, threshold: f64) -> Self {
        Self {
            risk_manager,
            threshold,
        }
    }

    pub fn detect_spread(&self, tick_a: &MarketTick, tick_b: &MarketTick) -> Option<ArbitrageResult> {
        // Simple triangular or direct arbitrage spread:
        // Buy on A (ask), sell on B (bid)
        let spread_buy_a_sell_b = tick_b.bid - tick_a.ask;
        // Buy on B (ask), sell on A (bid)
        let spread_buy_b_sell_a = tick_a.bid - tick_b.ask;

        let (max_spread, _buy_exchange, _sell_exchange) = if spread_buy_a_sell_b > spread_buy_b_sell_a {
            (spread_buy_a_sell_b, "A", "B")
        } else {
            (spread_buy_b_sell_a, "B", "A")
        };

        if max_spread > self.threshold {
            // Apply risk math
            let sim_result = self.risk_manager.simulate_trade(tick_a, max_spread);

            Some(ArbitrageResult {
                spread: max_spread,
                net_profit: sim_result.adjusted_profit,
                is_partial_fill: sim_result.is_partial_fill,
            })
        } else {
            None
        }
    }
}
