# Multi-stage Dockerfile for React Frontend
# Stage 1: Build dependencies
FROM node:18-alpine AS dependencies

# Install system dependencies
RUN apk add --no-cache \
    python3 \
    make \
    g++ \
    git

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production && npm cache clean --force

# Stage 2: Development build
FROM node:18-alpine AS development

# Install development dependencies
RUN apk add --no-cache \
    python3 \
    make \
    g++ \
    git \
    curl

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install all dependencies (including dev)
RUN npm ci && npm cache clean --force

# Create non-root user
RUN addgroup -g 1000 -S appgroup && \
    adduser -u 1000 -S appuser -G appgroup

# Change ownership
RUN chown -R appuser:appgroup /app

USER appuser

# Expose ports
EXPOSE 3000 24678

# Health check for development
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:3000 || exit 1

# Development command
CMD ["npm", "start"]

# Stage 3: Build application
FROM dependencies AS builder

# Set build arguments
ARG REACT_APP_API_URL
ARG REACT_APP_WS_URL
ARG REACT_APP_VERSION
ARG REACT_APP_BUILD_DATE
ARG REACT_APP_SENTRY_DSN

# Set environment variables
ENV REACT_APP_API_URL=$REACT_APP_API_URL
ENV REACT_APP_WS_URL=$REACT_APP_WS_URL
ENV REACT_APP_VERSION=$REACT_APP_VERSION
ENV REACT_APP_BUILD_DATE=$REACT_APP_BUILD_DATE
ENV REACT_APP_SENTRY_DSN=$REACT_APP_SENTRY_DSN
ENV NODE_ENV=production
ENV GENERATE_SOURCEMAP=false

WORKDIR /app

# Copy source code
COPY . .

# Install all dependencies for build
RUN npm ci

# Build the application
RUN npm run build

# Stage 4: Production runtime with Nginx
FROM nginx:1.25-alpine AS production

# Install additional tools
RUN apk add --no-cache \
    curl \
    jq

# Remove default nginx config
RUN rm /etc/nginx/conf.d/default.conf

# Copy built application
COPY --from=builder /app/build /usr/share/nginx/html

# Copy nginx configuration
COPY docker/frontend/nginx.conf /etc/nginx/nginx.conf

# Create nginx user and set permissions
RUN addgroup -g 1001 -S nginx-app && \
    adduser -u 1001 -S nginx-app -G nginx-app && \
    chown -R nginx-app:nginx-app /usr/share/nginx/html && \
    chown -R nginx-app:nginx-app /var/cache/nginx && \
    chown -R nginx-app:nginx-app /var/log/nginx && \
    chown -R nginx-app:nginx-app /etc/nginx/conf.d

# Create directories for nginx
RUN mkdir -p /var/cache/nginx/client_temp && \
    mkdir -p /var/cache/nginx/proxy_temp && \
    mkdir -p /var/cache/nginx/fastcgi_temp && \
    mkdir -p /var/cache/nginx/uwsgi_temp && \
    mkdir -p /var/cache/nginx/scgi_temp && \
    chown -R nginx-app:nginx-app /var/cache/nginx

# Create health check endpoint
RUN echo '{"status":"healthy","service":"frontend","version":"3.0.0"}' > /usr/share/nginx/html/health

# Switch to non-root user
USER nginx-app

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:80/health || exit 1

# Expose port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]

# Stage 5: Security scanning
FROM production AS security-scan

USER root

# Install security scanning tools
RUN apk add --no-cache \
    wget \
    tar

# Download and install security scanner
RUN wget -O /tmp/grype.tar.gz https://github.com/anchore/grype/releases/download/v0.65.0/grype_0.65.0_linux_amd64.tar.gz && \
    tar -xzf /tmp/grype.tar.gz -C /tmp && \
    mv /tmp/grype /usr/local/bin/ && \
    rm /tmp/grype.tar.gz

# Run security scan
RUN grype /usr/share/nginx/html --output json > /tmp/frontend-security-report.json || true

USER nginx-app
