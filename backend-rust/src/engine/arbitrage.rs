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
    pub is_legging_hedge: bool,
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
                is_legging_hedge: sim_result.is_legging_hedge,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_tick(bid: f64, ask: f64) -> MarketTick {
        MarketTick {
            symbol: "BTCUSDT".to_string(),
            bid,
            ask,
            timestamp: 0,
        }
    }

    #[test]
    fn test_detect_profitable_spread_binance_buy() {
        let risk = RiskManager::new(0.0, 0.0);
        let engine = ArbitrageEngine::new(risk, 5.0);
        
        let binance = mock_tick(60000.0, 60010.0); // Buy at 60010
        let kraken = mock_tick(60020.0, 60030.0);  // Sell at 60020
        
        let result = engine.detect_spread(&binance, &kraken);
        assert!(result.is_some());
        let opp = result.unwrap();
        // Buy binance (60010), sell kraken (60020). Spread = 10.0
        assert_eq!(opp.spread, 10.0);
    }

    #[test]
    fn test_reject_low_profit_spread() {
        let risk = RiskManager::new(0.0, 0.0);
        let engine = ArbitrageEngine::new(risk, 5.0);
        
        let binance = mock_tick(60000.0, 60010.0);
        let kraken = mock_tick(60012.0, 60020.0);
        
        // Spread is 60012 - 60010 = 2.0. Threshold is 5.0. Should reject.
        let result = engine.detect_spread(&binance, &kraken);
        assert!(result.is_none());
    }
}
