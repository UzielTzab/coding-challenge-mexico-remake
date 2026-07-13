# 🚀 Guía Definitiva de Infraestructura y Despliegue (Bot HFT)

Esta guía documenta toda la arquitectura, configuración, comandos operativos y, lo más importante, las lecciones aprendidas tras enfrentarnos a las limitaciones de los entornos Cloud durante el hackathon.

---

## 🏗️ 1. Arquitectura y Configuración del Servidor (EC2)

Para este proyecto utilizamos una instancia **AWS EC2 (m7i-flex.large)** con **8 GB de RAM** y **8 GB de Disco Duro**.
El sistema operativo base es **Ubuntu**.

### Ingresar al servidor
Desde tu terminal en Windows (PowerShell), utiliza tu llave `.pem`:
```powershell
ssh -o StrictHostKeyChecking=no -i "C:\Users\uzieltzab\Documents\My_Security\challenge_practice_api.pem" ubuntu@18.188.187.255
```
*(Nota: Si reinicias la instancia desde AWS, la IP Pública cambiará y deberás actualizar este comando y tu dominio en No-IP).*

### Requisitos previos instalados en el servidor
1. **Docker y Docker Compose**: Para contenerizar toda la aplicación.
2. **Nginx**: Como proxy inverso para enrutar el tráfico HTTP/HTTPS.
3. **Certbot (Let's Encrypt)**: Para proveer certificados SSL y permitir conexiones WebSockets Seguras (WSS).

---

## 🐳 2. Comandos Docker Esenciales (Operación Diaria)

Una vez dentro del servidor (`ssh`), navega a la carpeta del proyecto:
```bash
cd ~/coding-challenge-mexico-remake
```

### Ver el estado del sistema
- `docker ps`: Lista todos los contenedores encendidos y sus puertos.
- `docker-compose ps`: Muestra el estado específico de los servicios de esta app.

### Apagar y Encender
- `docker-compose down`: Apaga y destruye los contenedores actuales (no borra los volúmenes de base de datos).
- `docker-compose up -d`: Levanta toda la aplicación en segundo plano (Frontend, Backend, Redis, Postgres).
- `docker-compose restart backend`: Reinicia únicamente el motor de Rust.

### Leer los Logs (Crucial para monitoreo)
- `docker-compose logs -f backend`: Observa en tiempo real los logs del bot de arbitraje (presiona `Ctrl+C` para salir).
- `docker-compose logs --tail 50 frontend`: Ve las últimas 50 líneas de error del frontend.

---

## 🚨 3. Catálogo de Errores Críticos y Soluciones (Lecciones Aprendidas)

Durante el despliegue nos enfrentamos a desafíos técnicos severos. Aquí está documentado exactamente qué pasó y cómo se resolvió:

### ❌ Error 1: EC2 Congelado (Out Of Memory / Loop de la Muerte)
- **Síntoma**: El servidor dejaba de responder al SSH y el uso de CPU/RAM llegaba al 100%.
- **Causa**: Inicialmente el servidor tenía 1 GB de RAM. Al intentar compilar Rust (`cargo run` descarga más de 300 librerías), el proceso se quedaba sin memoria, crasheaba, y Docker intentaba reiniciarlo infinitamente.
- **Solución Definitiva**: Subimos a una instancia **m7i-flex.large (8 GB RAM)** y convertimos el `Dockerfile` de Rust a un "Multi-Stage Build" para separar el compilador del runtime.

### ❌ Error 2: `No space left on device` (Disco Lleno)
- **Síntoma**: La compilación llegaba al 99% y fallaba arrojando error al escribir dependencias `.rlib`.
- **Causa**: Aunque teníamos 8 GB de RAM, el disco duro seguía siendo de 8 GB. La compilación local en AWS generaba una caché gigantesca de gigabytes.
- **Solución Definitiva (Cross-Deploy)**: En lugar de compilar el backend en AWS, lo compilamos en Windows, lo exportamos como un `.tar` de 70 MB y lo subimos por SSH.

### ❌ Error 3: `502 Bad Gateway` y `address already in use`
- **Síntoma**: El Frontend no encendía en AWS.
- **Causa**: El Nginx nativo de Ubuntu estaba usando el Puerto 80 para la web. Nuestro contenedor de Vue también quería usar el Puerto 80 (`80:80`). Chocaron.
- **Solución Definitiva**: Cambiamos el puerto del frontend en `docker-compose.yml` a `8080:80` y configuramos Nginx para que reenviara el tráfico web ahí.

### ❌ Error 4: Bloqueo de WebSockets de Binance
- **Síntoma**: El Backend marcaba conexión rechazada a `stream.binance.com`.
- **Causa**: AWS está en EE.UU. y Binance Global bloquea IPs estadounidenses por regulaciones.
- **Solución Definitiva**: Cambiamos el endpoint en el código de Rust a `wss://stream.binance.us:9443` (Binance US), que maneja la misma estructura de datos pero sí permite conexiones desde AWS.

---

## ⚡ 4. Guía de Actualización en Producción (Cross-Deploy)

Si haces modificaciones al código fuente en el futuro, **no uses `docker-compose build` para el backend dentro de AWS**, o llenarás el disco. Sigue este flujo exacto desde Windows:

### A. Para actualizar el Frontend (Vue 3)
El frontend es muy ligero y no llena el disco, así que sí puedes compilarlo directamente en AWS:
1. Haz tus cambios, haz commit y `git push` desde Windows.
2. Ejecuta este comando mágico que entra al servidor, descarga los cambios y compila:
```powershell
ssh -o StrictHostKeyChecking=no -i "C:\Users\uzieltzab\Documents\My_Security\challenge_practice_api.pem" ubuntu@18.188.187.255 "cd ~/coding-challenge-mexico-remake && git pull && docker-compose build frontend && docker-compose up -d frontend"
```

### B. Para actualizar el Backend (Rust)
Utiliza la técnica de Cross-Deploy para evadir el límite del disco duro:

**1. Compila la imagen localmente en Windows:**
```powershell
docker build --platform linux/amd64 -t coding-challenge-mexico-remake-backend:latest ./backend-rust
```

**2. Empaqueta la imagen a un archivo `.tar`:**
```powershell
docker save -o backend-image.tar coding-challenge-mexico-remake-backend:latest
```

**3. Transfiere el `.tar` al servidor AWS:**
```powershell
scp -o StrictHostKeyChecking=no -i "C:\Users\uzieltzab\Documents\My_Security\challenge_practice_api.pem" backend-image.tar ubuntu@18.188.187.255:~
```

**4. Entra a AWS, limpia la basura, carga el .tar y reinicia el backend:**
```powershell
ssh -o StrictHostKeyChecking=no -i "C:\Users\uzieltzab\Documents\My_Security\challenge_practice_api.pem" ubuntu@18.188.187.255 "docker builder prune -a -f && docker system prune -a -f --volumes && sudo journalctl --vacuum-time=1s && docker load -i backend-image.tar && cd ~/coding-challenge-mexico-remake && docker-compose up -d backend"
```

> **¡Felicidades! Tienes un sistema de Alta Frecuencia con una arquitectura Cloud robusta y a prueba de balas.**
