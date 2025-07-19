# Multi-stage Dockerfile for Rust Backend
# Stage 1: Build dependencies and cache them
FROM rust:1.75-slim-bookworm AS dependencies

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    libsqlite3-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
COPY src-tauri/Cargo.toml ./src-tauri/

# Create dummy main.rs to build dependencies
RUN mkdir -p src-tauri/src && echo "fn main() {}" > src-tauri/src/main.rs

# Build dependencies (this layer will be cached)
RUN cd src-tauri && cargo build --release && rm -rf src target/release/deps/free_deep_research*

# Stage 2: Development build
FROM dependencies AS development

# Install development tools
RUN apt-get update && apt-get install -y \
    gdb \
    valgrind \
    strace \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

# Set development environment
ENV RUST_ENV=development
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1

# Create non-root user for development
RUN useradd -m -u 1000 -s /bin/bash developer && \
    chown -R developer:developer /app

USER developer

# Expose ports
EXPOSE 8080 9229

# Development command with hot reload
CMD ["cargo", "watch", "-x", "run", "--workdir", "src-tauri"]

# Stage 3: Build application
FROM dependencies AS builder

# Copy source code
COPY . .

# Build the application
WORKDIR /app/src-tauri
RUN cargo build --release

# Stage 4: Runtime image
FROM debian:bookworm-slim AS production

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    libsqlite3-0 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1001 -s /bin/bash appuser

# Create app directory
WORKDIR /app

# Copy health server and configuration
COPY docker/backend/health-server.py /app/
COPY docker/backend/config/ /app/config/

# Install Python dependencies for health server
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install psutil

# Create necessary directories
RUN mkdir -p /app/logs /app/uploads /app/cache /app/ssl && \
    chown -R appuser:appuser /app

# Make health server executable
RUN chmod +x /app/health-server.py

# Switch to non-root user
USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose port
EXPOSE 8080

# Set environment variables
ENV HEALTH_CHECK_PORT=8080
ENV PYTHONUNBUFFERED=1

# Start the health server
CMD ["python3", "/app/health-server.py"]

# Stage 5: Security scanning
FROM production AS security-scan

# Install security scanning tools
USER root
RUN apt-get update && apt-get install -y \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Download and run security scanner
RUN wget -O /tmp/grype.tar.gz https://github.com/anchore/grype/releases/download/v0.65.0/grype_0.65.0_linux_amd64.tar.gz && \
    tar -xzf /tmp/grype.tar.gz -C /tmp && \
    mv /tmp/grype /usr/local/bin/ && \
    rm /tmp/grype.tar.gz

# Run security scan
RUN grype /app/free-deep-research --output json > /tmp/security-report.json || true

# Switch back to app user
USER appuser
