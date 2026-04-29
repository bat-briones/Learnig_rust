FROM rust:1.94-slim

# Instalar dependencias básicas y Node.js
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

# Configurar Rust
ENV RUST_BACKTRACE=1
RUN cargo install rustlings

# Verificar versiones instaladas
RUN rustc --version && cargo --version && node --version && npm --version

# Mantener el contenedor vivo
CMD ["bash"]
