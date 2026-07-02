use rand::Rng;
use crate::engine::market_stream::MarketTick;

pub struct RiskManager {
    pub partial_fill_probability: f64,
    pub slippage_punishment: f64,
}

impl Default for RiskManager {
    fn default() -> Self {
        Self {
            partial_fill_probability: 0.05, // 5% probability
            slippage_punishment: 0.01, // 1% slippage punishment
        }
    }
}

pub struct RiskSimulationResult {
    pub is_partial_fill: bool,
    pub is_legging_hedge: bool,
    pub adjusted_profit: f64,
}

impl RiskManager {
    pub fn new(partial_fill_probability: f64, slippage_punishment: f64) -> Self {
        Self {
            partial_fill_probability,
            slippage_punishment,
        }
    }

    pub fn simulate_trade(&self, tick: &MarketTick, theoretical_profit: f64) -> RiskSimulationResult {
        let mut rng = rand::thread_rng();
        let is_partial_fill = rng.gen_bool(self.partial_fill_probability);
        // Introducimos probabilidad de "Legging Risk" (ej. 3% de que la pata 2 falle)
        let is_legging_hedge = !is_partial_fill && rng.gen_bool(0.03);
        
        let adjusted_profit = if is_partial_fill {
            self.emergency_hedge(tick, theoretical_profit)
        } else if is_legging_hedge {
            self.market_dump(tick, theoretical_profit)
        } else {
            theoretical_profit
        };

        RiskSimulationResult {
            is_partial_fill,
            is_legging_hedge,
            adjusted_profit,
        }
    }

    fn emergency_hedge(&self, _tick: &MarketTick, theoretical_profit: f64) -> f64 {
        // Execute emergency hedge
        // Apply slippage punishment on profit
        let loss = theoretical_profit * self.slippage_punishment;
        theoretical_profit - loss
    }
    
    fn market_dump(&self, _tick: &MarketTick, theoretical_profit: f64) -> f64 {
        // En caso de Legging Risk, se hace un volcado a mercado
        // La penalización es severa, simularemos perder el 80% del profit esperado
        // o incluso quedar en negativo
        let loss = theoretical_profit * 1.5; // Penalidad fuerte
        theoretical_profit - loss
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_fill_trigger() {
        // Force 100% partial fill probability to guarantee triggering
        let risk = RiskManager::new(1.0, 0.50); // 50% loss on partial fill
        let tick = MarketTick {
            symbol: "BTCUSDT".to_string(),
            bid: 100.0,
            ask: 100.0,
            timestamp: 0,
        };
        
        let result = risk.simulate_trade(&tick, 10.0);
        assert!(result.is_partial_fill);
        // 10.0 profit - 50% loss = 5.0
        assert_eq!(result.adjusted_profit, 5.0);
    }
    
    #[test]
    fn test_no_partial_fill() {
        // Force 0% probability
        let risk = RiskManager::new(0.0, 0.50);
        let tick = MarketTick {
            symbol: "BTCUSDT".to_string(),
            bid: 100.0,
            ask: 100.0,
            timestamp: 0,
        };
        
        let result = risk.simulate_trade(&tick, 10.0);
        assert!(!result.is_partial_fill);
        assert_eq!(result.adjusted_profit, 10.0);
    }
}
