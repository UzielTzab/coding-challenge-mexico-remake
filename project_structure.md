# Estructura del Proyecto (Ecosistema Completo)

Para la Fase 2, el repositorio evolucionará de un proyecto monolítico de Django a un ecosistema de microservicios. Esta será la estructura de carpetas:

```text
coding_challenge_mexico/
├── docker-compose.yml          # Orquestador maestro de los 4 contenedores
├── .env                        # Variables secretas (NO SE SUBE A GITHUB)
├── README.md                   # Documentación principal para el jurado
│
├── backend-rust/               # [CONTENEDOR 1: Backend HFT]
│   ├── Cargo.toml              # Dependencias de Rust (axum, tokio, sqlx, redis)
│   ├── Dockerfile              # Instrucciones para compilar Rust en Alpine Linux
│   └── src/
│       ├── main.rs             # Punto de entrada. Inicializa Tokio y Axum
│       ├── config/
│       │   └── settings.rs     # Carga variables de entorno
│       ├── api/                # Endpoints REST y manejadores WebSocket
│       │   ├── mod.rs
│       │   ├── handlers.rs     # Controladores de rutas
│       │   └── ws.rs           # Motor de WebSockets hacia el Frontend
│       ├── engine/             # Lógica core de alta frecuencia
│       │   ├── mod.rs
│       │   ├── market_stream.rs# Conexión a WSS de Binance/Kraken
│       │   ├── arbitrage.rs    # Detección matemática de Spreads
│       │   ├── risk.rs         # Simulación de Slippage, FOK/IOC
│       │   └── rebalancer.rs   # Lógica de Thresholds y Triangular Routing
│       └── db/                 # Conexión a Postgres y Redis
│           ├── mod.rs
│           ├── models.rs       # Estructuras de tablas
│           └── queries.rs      # SQLx operaciones
│
├── frontend-vue/               # [CONTENEDOR 2: Dashboard Frontend]
│   ├── package.json
│   ├── vite.config.ts
│   ├── Dockerfile              # Construye la app y la sirve con Nginx
│   ├── nginx.conf              # Configuración de ruteo para nip.io y WebSockets
│   └── src/
│       ├── main.ts
│       ├── App.vue
│       ├── stores/             # Pinia: Manejo de estado hiper-rápido
│       │   ├── market.ts
│       │   └── settings.ts
│       ├── components/         # Componentes visuales
│       │   ├── Dashboard/
│       │   │   ├── LightweightChart.vue # Gráfico de alto rendimiento
│       │   │   └── OrderBookTable.vue
│       │   └── Settings/
│       │       └── RiskSliders.vue
│       ├── styles/
│       │   └── variables.css   # Vanilla CSS Variables (Tema oscuro/Glass)
│       └── views/
│           ├── DashboardView.vue
│           └── ConfigurationView.vue
│
├── database/                   # [CONTENEDOR 3: PostgreSQL]
│   └── init.sql                # Script de creación de tablas iniciales
│
└── cache/                      # [CONTENEDOR 4: Redis]
    └── redis.conf              # Configuración para pub/sub de baja latencia
```
