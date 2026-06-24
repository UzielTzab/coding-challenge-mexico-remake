// ============= EXCHANGES =============
export interface Exchange {
  id: number;
  name: string;
  code: string;
  is_active: boolean;
  websocket_url?: string;
  rest_url?: string;
  latency_ms: number;
  status: string; // 'connected' | 'disconnected'
}

export interface ExchangeFeeProfile {
  id: number;
  exchange: number; // FK to Exchange
  trading_fee_percent: number;
  withdrawal_fee_btc: number;
}

// ============= MARKET DATA =============
export interface MarketData {
  exchange: string;
  pair: string;
  bid: number;
  ask: number;
  bidVolume?: number;
  askVolume?: number;
  timestamp: string;
  latency_ms?: number;
}

export interface MarketSnapshot {
  id: number;
  exchange: number; // FK to Exchange
  symbol: string;
  best_bid: number;
  best_ask: number;
  bid_volume: number;
  ask_volume: number;
  spread: number;
  latency_ms: number;
  received_at: string;
  raw_payload?: Record<string, any>;
  levels?: OrderBookLevel[];
}

export interface OrderBookLevel {
  id: number;
  snapshot: number; // FK to MarketSnapshot
  side: 'buy' | 'sell';
  price: number;
  quantity: number;
  level_index: number;
}

// ============= ARBITRAGE =============
export interface ArbitrageOpportunity {
  id: string;
  pair?: string;
  symbol?: string;
  buy_exchange: string;
  sell_exchange: string;
  buy_exchange_name?: string;
  sell_exchange_name?: string;
  ask_price: number;
  bid_price: number;
  volume_available: number;
  gross_spread: number;
  gross_spread_percent: number;
  estimated_fees: number;
  estimated_slippage: number;
  withdrawal_cost: number;
  latency_penalty: number;
  net_profit: number;
  net_profit_percent: number;
  profit_usd?: number; // alias
  profit_percent?: number; // alias
  spread_percent?: number; // alias
  status: string;
  decision_reason?: string;
  timestamp: string;
  detected_at: string;
  cost_breakdown?: OpportunityCostBreakdown;
}

export interface OpportunityCostBreakdown {
  id: number;
  opportunity: number; // FK to ArbitrageOpportunity
  buy_fee_usd: number;
  sell_fee_usd: number;
  withdrawal_fee_usd: number;
  slippage_usd: number;
  latency_penalty_usd: number;
}

// ============= TRADING =============
export interface SimulatedTrade {
  id: string;
  opportunity_id: string;
  symbol: string;
  buy_exchange: string;
  sell_exchange: string;
  quantity_btc: number;
  buy_cost: number;
  sell_revenue: number;
  total_fees: number;
  slippage_cost: number;
  latency_cost: number;
  net_profit: number;
  status: string;
  executed_at: string;
  legs?: TradeLeg[];
}

export interface TradeLeg {
  id: number;
  trade: number; // FK to SimulatedTrade
  side: 'buy' | 'sell';
  exchange: number; // FK to Exchange
  price: number;
  quantity: number;
  fee: number;
  status: string;
  executed_at: string;
}

export interface Trade {
  id: string;
  opportunity_id: string;
  status: string;
  net_profit: number;
  executed_at: string;
}

// ============= WALLETS =============
export interface Wallet {
  id: number;
  exchange: number; // FK to Exchange
  btc_available: number;
  usdt_available: number;
  btc_locked: number;
  usdt_locked: number;
  total_value_usd: number;
  updated_at: string;
}

export interface WalletMovement {
  id: number;
  wallet: number; // FK to Wallet
  movement_type: string;
  asset: string;
  amount: number;
  balance_before: number;
  balance_after: number;
  reference_type?: string;
  reference_id?: string;
  created_at: string;
}

// ============= ANALYTICS =============
export interface PerformanceSnapshot {
  id: number;
  total_pnl_usd: number;
  total_trades: number;
  profitable_trades: number;
  discarded_opportunities: number;
  failed_trades: number;
  total_fees_usd: number;
  average_execution_time_ms: number;
  win_rate_percent: number;
  created_at: string;
}

// ============= SYSTEM LOGS =============
export interface SystemLog {
  id: number;
  level: 'info' | 'success' | 'warn' | 'error';
  source: string;
  message: string;
  metadata?: Record<string, any>;
  created_at: string;
}

export interface BotRuntimeState {
  id: number;
  is_running: boolean;
  mode: string;
  started_at?: string;
  stopped_at?: string;
  last_heartbeat_at?: string;
  circuit_breaker_active: boolean;
  created_at: string;
}
