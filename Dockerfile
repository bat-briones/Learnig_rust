FROM rust:1.94-slim

# Instalar dependencias básicas que suele pedir Rust para compilar crates
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

# Instalar rustlings globalmente en el contenedor
RUN cargo install rustlings

# Mantener el contenedor vivo
CMD ["tail", "-f", "/dev/null"]