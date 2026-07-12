# Wall-Street HFT Arbitrage Engine 🚀

Este proyecto es la solución definitiva para el Challenge Final de México. En lugar de construir un bot estándar, hemos diseñado una arquitectura institucional de **Alta Frecuencia (HFT)** capaz de mitigar los riesgos inherentes al arbitraje cripto en tiempo real.

## 🏗 Arquitectura del Sistema
El motor está completamente desacoplado (Decoupling puro).
* **Backend (Rust):** Despacha la lógica matemática, controla WebSockets y enruta órdenes. Es un sistema multi-hilo impulsado por Tokio.
* **Frontend (Vue 3 + Pinia):** Recibe un chorro constante de ticks y lo procesa mediante un canvas reactivo y CSS nativo sin saturar el DOM, logrando visualizaciones de mercado a 60 FPS.
* **Middleware (Redis Pub/Sub):** El puente ultra-rápido que comunica el backend y el frontend, evitando la sobrecarga en PostgreSQL. PostgreSQL se reserva **únicamente** para auditoría y persistencia asíncrona.

## 🧠 Características Institucionales (HFT Edge)

A diferencia de bots simples de arbitraje que compran y venden ciegamente, nuestro sistema está preparado para escenarios adversos:

### 1. Delta Neutrality (Cobertura de Riesgo Direccional)
Si un exchange se queda sin saldo por un evento de mercado y el inventario neto del bot supera los 2.0 BTC, la exposición direccional al precio de Bitcoin es peligrosa. El bot automáticamente despacha un `DELTA_HEDGE`, emulando la apertura de una posición SHORT en futuros perpetuos para cubrir la cartera de una posible caída de precio mientras rebalancea su saldo.

### 2. Mitigación de Legging Risk
En escenarios reales, una de las dos "patas" del arbitraje puede fallar (latencia, falta de liquidez instantánea). El motor de riesgo en Rust (RiskManager) evalúa y simula constantemente el *Legging Risk*. Si este evento asíncrono ocurre en medio de una operación, el bot ejecuta un *Market Dump* inmediato, sacrificando el margen de ganancia de la operación para liquidar el activo y no quedarse atrapado con el token físico.

### 3. Phantom Liquidity Protection
En libros de órdenes volátiles, el spread a veces es un espejismo creado por un solo market maker (Phantom Liquidity). El motor tiene un cooldown de hardware a nivel de milisegundos para evitar disparar 100 órdenes seguidas hacia una liquidez falsa, asegurando solo trades consolidados.

### 4. Contadores In-Memory (Zero Latency)
Para medir cuántas oportunidades detecta el bot vs. cuántas desecha (por spread negativo), **no hacemos `INSERT` en PostgreSQL** por cada evento, ya que eso destruiría el rendimiento. En su lugar, utilizamos un contador atómico en memoria (`std::sync::atomic::AtomicU64`) dentro del núcleo de Rust. Esto incrementa millones de ticks descartados en 0 milisegundos y los envía al dashboard en tiempo real, demostrando el altísimo throughput del motor.

### 5. Rebalanceo Triangular
Contamos con un Worker Asíncrono de Rebalanceo. Constantemente evalúa si los balances locales (Kraken vs Binance) se desestabilizan de manera crítica, ejecutando transacciones on-chain rápidas (usando un bridge simulado como XRP) para reinyectar capital y seguir tradeando indefinidamente sin intervención humana.

---

## 💻 Tech Stack
- **Rust (Tokio, Axum, SQLx)**
- **Vue 3 (Composition API, Pinia)**
- **PostgreSQL 15 (Auditoría/Persistencia)**
- **Redis (Mensajería Pub/Sub de baja latencia)**
- **Docker & Docker Compose**

## 🏁 Instalación y Uso (Jurado)
Todo corre bajo contenedores Docker listos para despliegue:

```bash
# Levantar todo el ecosistema (DB, Redis, Rust, Vue)
docker-compose up -d --build
```
Una vez levantado, la plataforma estará viva y recibiendo los Websockets de mercado en `http://localhost:80`.
¡Disfruta del motor!
