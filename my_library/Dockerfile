FROM node:22-slim

# Instalar dependencias básicas útiles para desarrollo
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Instalar Rust toolchain de forma desatendida
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspace

# Mantener el contenedor vivo
CMD ["tail", "-f", "/dev/null"]
