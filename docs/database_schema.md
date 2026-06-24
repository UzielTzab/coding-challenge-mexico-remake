# Diseño de Base de Datos (PostgreSQL)

Para asegurar la velocidad, Rust manejará el 90% de la lógica en memoria y solo escribirá en la base de datos de PostgreSQL para persistencia histórica (Trades) y configuración del sistema (Settings).

## Diagrama Visual de Tablas

| Tabla | Propósito Principal |
| :--- | :--- |
| **`system_settings`** | Singleton. Guarda la configuración dinámica (Thresholds, Fees, Slippage) que el usuario mueve en el Frontend. |
| **`trades`** | Bitácora histórica de todas las operaciones realizadas (Exitosas, Fallidas y Emergency Hedges). |
| **`wallet_balances`** | Estado actual del dinero disponible en cada exchange. |
| **`rebalance_events`** | Historial de transferencias en la red Blockchain para cuadrar las carteras. |

---

## Esquema SQL y Relaciones

```sql
-- 1. TABLA DE CONFIGURACIÓN MAESTRA
-- Solo debe existir 1 fila en esta tabla (ID = 1).
CREATE TABLE system_settings (
    id SERIAL PRIMARY KEY,
    min_net_profit_usd DECIMAL(10,2) NOT NULL DEFAULT 5.00,
    max_trade_volume_btc DECIMAL(10,4) NOT NULL DEFAULT 0.01,
    emergency_slippage_penalty_pct DECIMAL(5,2) NOT NULL DEFAULT 2.00,
    rebalance_threshold_pct DECIMAL(5,2) NOT NULL DEFAULT 20.00,
    is_bot_active BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 2. TABLA DE CARTERAS (INVENTARIO BASE)
CREATE TABLE wallet_balances (
    id SERIAL PRIMARY KEY,
    exchange VARCHAR(50) NOT NULL, -- 'BINANCE' o 'KRAKEN'
    asset VARCHAR(20) NOT NULL,    -- 'BTC' o 'USDT'
    available_balance DECIMAL(18,8) NOT NULL,
    locked_balance DECIMAL(18,8) NOT NULL DEFAULT 0,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(exchange, asset)
);

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
    -- Gestión de Riesgo (IOC/FOK)
    execution_status VARCHAR(20) NOT NULL, -- 'PERFECT', 'PARTIAL_FILL', 'HEDGED', 'FAILED'
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
    routing_method VARCHAR(20) NOT NULL, -- 'DIRECT_ERC20', 'TRIANGULAR_XRP'
    network_fee_usd DECIMAL(10,2) NOT NULL
);
```

## Justificación Arquitectónica para el Jurado

*   **¿Por qué estas tablas?** El modelo relacional está diseñado estrictamente para *auditoría y control de mando*. 
*   **Optimizaciones:** No guardamos las "Oportunidades" o los "Market Snapshots" en PostgreSQL. Guardar miles de filas por segundo destruiría el disco duro. Todos los datos de precios efímeros viven y mueren en **Redis** y en la memoria RAM de **Rust**. PostgreSQL se usa exclusivamente para responder a la pregunta: *"¿Cuánto dinero gané hoy y qué parámetros de riesgo estaban activos?"*.
