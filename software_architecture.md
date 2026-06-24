# Arquitectura de Software: Motor de Arbitraje HFT (Fase 2)

Este documento detalla la arquitectura institucional diseñada para lograr latencias de microsegundos, procesar datos de mercado en tiempo real y asegurar la fiabilidad del sistema en condiciones adversas.

## 1. El Ecosistema de Contenedores (Docker)

El sistema completo está orquestado mediante `docker-compose`, dividiendo las responsabilidades en microservicios aislados:

*   **`backend-core` (Rust):** El motor principal. Construido en Rust utilizando `Axum` y `Tokio`. Se encarga de conectarse a los WebSockets de los exchanges, ejecutar los modelos matemáticos en memoria y disparar las órdenes de compra/venta.
*   **`database` (PostgreSQL):** Base de datos relacional para almacenamiento persistente en frío. Guarda el historial de operaciones (`Trades`), movimientos de billetera (`WalletMovements`) y las configuraciones dinámicas (`SystemSettings`).
*   **`message-broker` (Redis):** Actúa como el puente de ultra-baja latencia. El backend de Rust publica (`Pub`) los spreads detectados aquí, y otros servicios o el frontend se suscriben (`Sub`) para recibirlos en tiempo real sin sobrecargar el motor de Rust.
*   **`frontend-proxy` (Nginx):** Contenedor ultraligero que sirve los archivos estáticos compilados de Vue 3. Además, actúa como un Proxy Inverso (Reverse Proxy) para enrutar las peticiones al backend.

## 2. Dominio Dinámico (`nip.io`)

Para facilitar el despliegue y las pruebas del jurado sin necesidad de configurar DNS complejos, utilizaremos `nip.io`. 
*   **Beneficio:** Permite mapear IPs locales o públicas directamente a un nombre de dominio válido.
*   **Ejemplo:** El frontend apuntará sus peticiones de API y WebSockets a `api.127.0.0.1.nip.io` (en desarrollo) o `api.<EC2_IP>.nip.io` (en producción). Esto soluciona problemas de CORS y ruteo en el Nginx de manera instantánea.

## 3. Despliegue en AWS EC2 (Estrategia de Baja Latencia)

Para demostrar ambición, el despliegue no será un simple "subir código a la nube".
*   **Instancias Optimizadas:** Utilizaremos instancias EC2 de la familia Computación Optimizada (ej. `c6i.large`) que ofrecen redes de alto rendimiento (hasta 12.5 Gbps).
*   **Colocación Geográfica:** Desplegaremos la instancia en la región `ap-northeast-1` (Tokio) si apuntamos a Binance, o `eu-west-2` (Londres) dependiendo del exchange objetivo, reduciendo la latencia de red de 60ms a < 5ms.
*   **Beneficio para el Jurado:** Verán que entendemos que el código es solo la mitad de la batalla; la infraestructura de red es la otra mitad.

## 4. El "Secret Sauce": Kernel Bypass (Explicación para el Jurado)

En la justificación arquitectónica, explicaremos por qué elegimos Rust y el concepto de *Kernel Bypass*:

*   **El Problema:** En sistemas normales (como Python/Django), cuando llega un paquete de datos de Binance por la tarjeta de red, el Sistema Operativo (Linux) lo intercepta, lo pasa por el Kernel, revisa reglas de firewall, lo copia a la memoria del usuario, y finalmente se lo da a Python. Esto toma milisegundos invaluables.
*   **La Solución Teórica (Kernel Bypass):** Técnicas como `DPDK` (Data Plane Development Kit) o `AF_XDP` permiten que la tarjeta de red entregue los datos de precios *directamente* a la memoria de nuestro programa en Rust, saltándose el Kernel de Linux por completo.
*   **Impacto:** Reduce la latencia de recepción de datos de milisegundos a nanosegundos. Aunque no implementemos DPDK puro en este sprint por tiempo, justificar que *Rust fue elegido por su capacidad futura para integrarse con C y ejecutar Kernel Bypass* dejará al jurado sin palabras.
