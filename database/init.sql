-- 1. TABLA DE CONFIGURACIÓN MAESTRA
CREATE TABLE system_settings (
    id SERIAL PRIMARY KEY,
    min_net_profit_usd DECIMAL(10,2) NOT NULL DEFAULT 5.00,
    max_trade_volume_btc DECIMAL(10,4) NOT NULL DEFAULT 0.01,
    emergency_slippage_penalty_pct DECIMAL(5,2) NOT NULL DEFAULT 2.00,
    rebalance_threshold_pct DECIMAL(5,2) NOT NULL DEFAULT 20.00,
    is_bot_active BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insertar configuración por defecto
INSERT INTO system_settings (id) VALUES (1);

-- 2. TABLA DE CARTERAS (INVENTARIO BASE)
CREATE TABLE wallet_balances (
    id SERIAL PRIMARY KEY,
    exchange VARCHAR(50) NOT NULL,
    asset VARCHAR(20) NOT NULL,
    available_balance DECIMAL(18,8) NOT NULL,
    locked_balance DECIMAL(18,8) NOT NULL DEFAULT 0,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(exchange, asset)
);

INSERT INTO wallet_balances (exchange, asset, available_balance) VALUES 
('Binance', 'BTC', 1.25),
('Binance', 'USDT', 50000.00),
('Kraken', 'BTC', 0.8),
('Kraken', 'USDT', 35000.00);

-- 3. TABLA DE OPERACIONES (TRADES)
CREATE TABLE trades (
    id UUID PRIMARY KEY,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    buy_exchange VARCHAR(50) NOT NULL,
    sell_exchange VARCHAR(50) NOT NULL,
    volume_btc DECIMAL(18,8) NOT NULL,
    buy_price_usd DECIMAL(18,2) NOT NULL,
    sell_price_usd DECIMAL(18,2) NOT NULL,
    gross_profit_usd DECIMAL(10,2) NOT NULL,
    net_profit_usd DECIMAL(10,2) NOT NULL,
    execution_status VARCHAR(20) NOT NULL,
    latency_ms INTEGER NOT NULL
);

-- 4. TABLA DE REBALANCEO
CREATE TABLE rebalance_events (
    id UUID PRIMARY KEY,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    source_exchange VARCHAR(50) NOT NULL,
    target_exchange VARCHAR(50) NOT NULL,
    asset VARCHAR(20) NOT NULL,
    amount DECIMAL(18,8) NOT NULL,
    routing_method VARCHAR(20) NOT NULL,
    network_fee_usd DECIMAL(10,2) NOT NULL
);
