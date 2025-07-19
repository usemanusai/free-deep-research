# Dockerfile for Tauri Application Testing
FROM rust:1.75-slim-bullseye

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    libpq-dev \
    curl \
    git \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js for frontend testing
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs

# Install Rust testing tools
RUN cargo install cargo-tarpaulin cargo-audit cargo-outdated

# Create app directory
WORKDIR /app

# Create non-root user
RUN useradd -m -u 1000 tester && \
    chown -R tester:tester /app

USER tester

# Copy package files for caching
COPY --chown=tester:tester package*.json ./
COPY --chown=tester:tester src-tauri/Cargo.toml src-tauri/Cargo.lock ./src-tauri/

# Install Node.js dependencies
RUN npm ci

# Create dummy main.rs for dependency caching
RUN mkdir -p src-tauri/src && echo "fn main() {}" > src-tauri/src/main.rs

# Build dependencies
WORKDIR /app/src-tauri
RUN cargo build --tests && rm -rf src target/debug/deps/free_deep_research*

# Back to app directory
WORKDIR /app

# Default command for testing
CMD ["./docker/testing/run-tests.sh"]
