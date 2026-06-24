# ArbiBTC: Motor HFT de Arbitraje Institucional (Fase Final)

Este repositorio contiene la entrega final del **Bot de Arbitraje Institucional** para la gran final del Hackathon. El sistema ha sido diseñado desde cero bajo una arquitectura de microservicios robusta y orientada al máximo rendimiento.

## Arquitectura del Sistema

El ecosistema está construido con las siguientes tecnologías:

1. **Rust (Backend):** Motor de Alta Frecuencia (HFT) que procesa WebSockets reales de **Binance** y **Kraken V2**, aplicando heurística de riesgo y cálculos de latencia en microsegundos.
2. **Vue 3 + Pinia (Frontend):** Dashboard institucional sin frameworks CSS pesados, empleando *Glassmorphism* y Vanilla CSS, con gráficos a 60FPS de TradingView (Lightweight Charts).
3. **Redis (Pub/Sub):** Broker de mensajería ultrarrápida. El backend publica los spreads (oportunidades) y el frontend los consume vía WebSockets.
4. **PostgreSQL:** Persistencia real de configuraciones de usuario (`system_settings`) y registros de eventos contables (`rebalance_events`, `wallet_balances`).

## Instrucciones de Despliegue Rápido (Jueces)

Todo el sistema está contenerizado. Para desplegar la plataforma completa en tu máquina local, ejecuta:

```bash
docker-compose up --build -d
```

Una vez que los contenedores arranquen:
1. Abre tu navegador en `http://localhost:5173` (Frontend Vite).
2. Ve al **Dashboard** y haz clic en **"Iniciar Bot"**.

---

## Concepto Técnico: Kernel Bypass (Preparación)

El motor en Rust ha sido estructurado conceptualmente para poder integrar tecnologías de **Kernel Bypass** (como DPDK o Solarflare OpenOnload). 
- **¿Qué es?** El Kernel Bypass permite que los paquetes de red (TCP/UDP) salten directamente desde la tarjeta de red (NIC) hacia el espacio de usuario (nuestro binario de Rust), evadiendo la sobrecarga de interrupciones del Kernel de Linux.
- **¿Cómo estamos preparados?** Nuestro procesamiento de `MarketTick` y `ArbitrageEngine` están completamente desacoplados del sistema operativo, permitiendo en un futuro inyectar memoria compartida (Shared Memory) directamente de un controlador de red optimizado, llevando nuestra latencia de milisegundos a microsegundos puros.

---

## Probando el "Emergency Hedge" y Rebalanceo

1. **Slippage y Riesgo:** Modifica el control deslizante de *Slippage* en el panel de **Settings**. Observarás cómo en el Dashboard las oportunidades dejan de ejecutarse si el mercado se vuelve demasiado volátil (simulado por el `RiskManager`).
2. **Rebalanceo Triangular:** El sistema monitorea los saldos reales en PostgreSQL. Si Binance cae por debajo de `0.1 BTC`, el motor de Rust intercepta la condición, genera un *Triangular Rebalance* (puente XRP), deduce USDT de Kraken y abona BTC a Binance para proveer liquidez y evitar el *Liquidity Drain*.

*Este proyecto es la culminación de un diseño HFT desacoplado, combinando persistencia, streams reales y reactividad avanzada.*
