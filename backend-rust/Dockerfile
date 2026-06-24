FROM rust:latest
WORKDIR /app

# Instalar dependencias del sistema requeridas para tokio-tungstenite y ssl
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copiar el código
COPY . .

# Exponer el puerto
EXPOSE 8000

# Para entornos de desarrollo usamos cargo run.
CMD ["cargo", "run"]
