# ArbiBTC: Motor HFT de Arbitraje Institucional (Fase Final)

Este repositorio contiene la entrega final del **Bot de Arbitraje Institucional** para la gran final del Hackathon. El sistema ha sido diseñado desde cero bajo una arquitectura de microservicios robusta, orientada al máximo rendimiento, gestión de riesgo de grado Wall-Street y neutralidad de portafolio.

## 🚀 Arquitectura y Stack Tecnológico

El ecosistema está construido con las siguientes tecnologías:

1. **Rust (Backend):** Motor de Alta Frecuencia (HFT) que procesa WebSockets reales de **Binance** y **Kraken V2**, aplicando heurística de riesgo estocástico y cálculos de latencia en microsegundos.
2. **Vue 3 + Pinia (Frontend):** Dashboard institucional sin frameworks CSS pesados, empleando *Glassmorphism* y Vanilla CSS oscuro, con gráficos a 60FPS de TradingView (Lightweight Charts).
3. **Redis (Pub/Sub):** Broker de mensajería ultrarrápida. El backend publica los spreads (oportunidades) y eventos de riesgo, y el frontend los consume vía WebSockets sin bloquear el DOM.
4. **PostgreSQL:** Persistencia real de configuraciones de usuario (`system_settings`) y registros de eventos contables y trades ejecutados (`rebalance_events`, `wallet_balances`, `trades`).

---

## 🛡️ Características Avanzadas (Nivel Institucional)

A diferencia de un bot tradicional, este motor incorpora protecciones de una mesa de *Prop Trading*:

- **Protección contra Phantom Liquidity (Cooldown):** Evita la saturación y las ejecuciones ficticias sobre el mismo spread al imponer un estado refractario en el par ejecutado.
- **Delta Neutrality (Cobertura):** Si el balance neto de BTC (Binance + Kraken) supera los 2.0 BTC, el motor detecta el riesgo direccional y simula instantáneamente la apertura de una posición corta (Short) en Futuros Perpetuos para cubrir el portafolio.
- **Legging Risk Management:** Motor estocástico que inyecta una probabilidad del 3% de sufrir un "Fallo en la Pata 2". Al ocurrir, ejecuta un *Market Dump* para salir de la posición, protegiendo el capital a costa de un *slippage* severo.
- **Enrutamiento Triangular & Liquidity Drain:** Si un exchange se queda sin saldo (ej. Binance cae por debajo de 0.1 BTC), el bot intercepta la condición y simula un rebalanceo transfronterizo usando XRP como puente.
- **Órdenes IOC (Immediate Or Cancel):** Todas las órdenes son procesadas bajo el paradigma de ejecución inmediata o cancelación, visualizadas en la interfaz.

---

## 💻 Instrucciones de Instalación y Despliegue (Jueces)

Todo el sistema está contenerizado. Para desplegar la plataforma completa en tu máquina local, ejecuta:

```bash
docker-compose down -v
docker-compose up --build -d
```

Una vez que los contenedores arranquen:
1. Abre tu navegador en [http://localhost:5173](http://localhost:5173).
2. Ve al panel principal y haz clic en el botón de **"Iniciar Bot"** (Esquina superior).
3. Podrás observar en tiempo real la detección de spreads, las alertas de *Delta Neutrality* y las ejecuciones *IOC*.

---

## 📸 Capturas de Pantalla

A continuación, se muestra el sistema en acción:

### Dashboard Principal y Detección HFT
*(Panel general mostrando los Ticks en tiempo real, KPIs y Órdenes IOC)*
![Dashboard Principal](./docs/assets/dashboard.png)

### Gestión de Riesgo (Delta Neutrality & Legging)
*(El sistema detectando exceso de exposición BTC y protegiéndose)*
![Delta Neutrality Alertas](./docs/assets/delta_hedge.png)

*(Nota: Agrega tus propias capturas de pantalla en la carpeta `docs/assets/` con estos nombres para que se visualicen correctamente en Github/Gitlab).*

---

## ⚡ Concepto Técnico Futuro: Kernel Bypass

El motor en Rust ha sido estructurado conceptualmente para poder integrar tecnologías de **Kernel Bypass** (como DPDK o Solarflare OpenOnload). 
- **¿Qué es?** El Kernel Bypass permite que los paquetes de red (TCP/UDP) salten directamente desde la tarjeta de red (NIC) hacia el espacio de usuario (nuestro binario de Rust), evadiendo la sobrecarga de interrupciones del Kernel de Linux.
- **¿Cómo estamos preparados?** Nuestro procesamiento de `MarketTick` y `ArbitrageEngine` están completamente desacoplados del sistema operativo, permitiendo en un futuro inyectar memoria compartida (Shared Memory) directamente de un controlador de red optimizado, llevando nuestra latencia de milisegundos a microsegundos puros.
