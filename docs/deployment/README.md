# 🚀 Deployment Guide

## Overview

This guide provides comprehensive instructions for deploying the Free Deep Research System across different environments, from development to production.

## 📋 Deployment Options

### 1. Docker Deployment (Recommended)
- **Best for**: Production environments, consistent deployments
- **Complexity**: Medium
- **Scalability**: High
- **Maintenance**: Low

### 2. Native Installation
- **Best for**: Development, custom environments
- **Complexity**: High
- **Scalability**: Medium
- **Maintenance**: High

### 3. Cloud Deployment
- **Best for**: Enterprise, high availability
- **Complexity**: Medium
- **Scalability**: Very High
- **Maintenance**: Low

## 🐳 Docker Deployment

### Quick Start

```bash
# Clone the repository
git clone https://github.com/huggingfacer04/free-deep-research.git
cd free-deep-research

# Copy environment configuration
cp .env.template .env
nano .env  # Configure your settings

# Start the system
docker-compose up -d

# Verify deployment
docker-compose ps
```

### Environment Configuration

```bash
# .env file configuration
# Database Configuration
POSTGRES_DB=free_deep_research
POSTGRES_USER=research_user
POSTGRES_PASSWORD=secure_password_here
DATABASE_URL=postgresql://research_user:secure_password_here@postgres:5432/free_deep_research

# Redis Configuration
REDIS_URL=redis://redis:6379

# API Keys (Required)
OPENROUTER_API_KEY=your_openrouter_key_here
SERPAPI_KEY=your_serpapi_key_here
TAVILY_API_KEY=your_tavily_key_here
FIRECRAWL_API_KEY=your_firecrawl_key_here
JINA_API_KEY=your_jina_key_here

# Application Configuration
NODE_ENV=production
RUST_LOG=info
APP_PORT=3000
API_PORT=8080

# Security Configuration
JWT_SECRET=your_jwt_secret_here
ENCRYPTION_KEY=your_32_character_encryption_key
SESSION_SECRET=your_session_secret_here

# Monitoring Configuration
ENABLE_METRICS=true
METRICS_PORT=9090
LOG_LEVEL=info
```

### Docker Compose Configuration

```yaml
# docker-compose.yml
version: '3.8'

services:
  # Main Application
  app:
    build:
      context: .
      dockerfile: docker/app/Dockerfile
    ports:
      - "${APP_PORT:-3000}:3000"
    environment:
      - NODE_ENV=${NODE_ENV:-production}
      - DATABASE_URL=${DATABASE_URL}
      - REDIS_URL=${REDIS_URL}
    depends_on:
      - postgres
      - redis
    volumes:
      - app_data:/app/data
    restart: unless-stopped

  # Database
  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=${POSTGRES_DB}
      - POSTGRES_USER=${POSTGRES_USER}
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./docker/database/init:/docker-entrypoint-initdb.d
    ports:
      - "5432:5432"
    restart: unless-stopped

  # Cache
  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    ports:
      - "6379:6379"
    restart: unless-stopped

  # Monitoring
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./docker/monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    restart: unless-stopped

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD:-admin}
    volumes:
      - grafana_data:/var/lib/grafana
      - ./docker/monitoring/grafana:/etc/grafana/provisioning
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
  app_data:
  prometheus_data:
  grafana_data:

networks:
  default:
    name: free_deep_research_network
```

### Production Docker Configuration

```dockerfile
# docker/app/Dockerfile.prod
FROM node:18-alpine AS frontend-builder

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

FROM rust:1.75-alpine AS backend-builder

WORKDIR /app
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./
RUN cargo fetch

COPY src-tauri/ .
RUN cargo build --release

FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache \
    nodejs \
    npm \
    ca-certificates \
    tzdata

# Create app user
RUN addgroup -g 1001 -S appgroup && \
    adduser -S appuser -u 1001 -G appgroup

WORKDIR /app

# Copy built applications
COPY --from=frontend-builder /app/dist ./frontend/
COPY --from=backend-builder /app/target/release/free-deep-research ./backend/

# Set ownership
RUN chown -R appuser:appgroup /app

USER appuser

EXPOSE 3000 8080

CMD ["./backend/free-deep-research"]
```

## ☁️ Cloud Deployment

### AWS Deployment

```yaml
# aws/cloudformation.yml
AWSTemplateFormatVersion: '2010-09-09'
Description: 'Free Deep Research System - AWS Deployment'

Parameters:
  InstanceType:
    Type: String
    Default: t3.medium
    AllowedValues: [t3.small, t3.medium, t3.large]
  
  KeyPairName:
    Type: AWS::EC2::KeyPair::KeyName
    Description: EC2 Key Pair for SSH access

Resources:
  # VPC Configuration
  VPC:
    Type: AWS::EC2::VPC
    Properties:
      CidrBlock: 10.0.0.0/16
      EnableDnsHostnames: true
      EnableDnsSupport: true

  # Application Load Balancer
  ApplicationLoadBalancer:
    Type: AWS::ElasticLoadBalancingV2::LoadBalancer
    Properties:
      Type: application
      Scheme: internet-facing
      Subnets:
        - !Ref PublicSubnet1
        - !Ref PublicSubnet2

  # ECS Cluster
  ECSCluster:
    Type: AWS::ECS::Cluster
    Properties:
      ClusterName: free-deep-research-cluster

  # RDS Database
  Database:
    Type: AWS::RDS::DBInstance
    Properties:
      DBInstanceClass: db.t3.micro
      Engine: postgres
      EngineVersion: '15.4'
      AllocatedStorage: 20
      DBName: freedeepresearch
      MasterUsername: !Ref DBUsername
      MasterUserPassword: !Ref DBPassword
      VPCSecurityGroups:
        - !Ref DatabaseSecurityGroup

  # ElastiCache Redis
  RedisCluster:
    Type: AWS::ElastiCache::CacheCluster
    Properties:
      CacheNodeType: cache.t3.micro
      Engine: redis
      NumCacheNodes: 1
      VpcSecurityGroupIds:
        - !Ref RedisSecurityGroup
```

### Kubernetes Deployment

```yaml
# k8s/deployment.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: free-deep-research
  labels:
    app: free-deep-research
spec:
  replicas: 3
  selector:
    matchLabels:
      app: free-deep-research
  template:
    metadata:
      labels:
        app: free-deep-research
    spec:
      containers:
      - name: app
        image: free-deep-research:latest
        ports:
        - containerPort: 3000
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: app-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: app-secrets
              key: redis-url
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5

---
apiVersion: v1
kind: Service
metadata:
  name: free-deep-research-service
spec:
  selector:
    app: free-deep-research
  ports:
  - name: web
    port: 80
    targetPort: 3000
  - name: api
    port: 8080
    targetPort: 8080
  type: LoadBalancer
```

## 🔧 Configuration Management

### Environment-Specific Configurations

#### Development
```bash
# .env.development
NODE_ENV=development
RUST_LOG=debug
LOG_LEVEL=debug
ENABLE_HOT_RELOAD=true
DISABLE_RATE_LIMITING=true
```

#### Staging
```bash
# .env.staging
NODE_ENV=staging
RUST_LOG=info
LOG_LEVEL=info
ENABLE_METRICS=true
RATE_LIMIT_REQUESTS=1000
```

#### Production
```bash
# .env.production
NODE_ENV=production
RUST_LOG=warn
LOG_LEVEL=warn
ENABLE_METRICS=true
ENABLE_SECURITY_HEADERS=true
RATE_LIMIT_REQUESTS=100
```

### Secrets Management

```bash
# Using Docker Secrets
echo "your_secret_key" | docker secret create jwt_secret -
echo "your_db_password" | docker secret create db_password -

# Using Kubernetes Secrets
kubectl create secret generic app-secrets \
  --from-literal=database-url="postgresql://user:pass@host:5432/db" \
  --from-literal=redis-url="redis://host:6379" \
  --from-literal=jwt-secret="your_jwt_secret"
```

## 📊 Monitoring & Observability

### Health Checks

```typescript
// Health check endpoints
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    uptime: process.uptime(),
    version: process.env.npm_package_version
  });
});

app.get('/ready', async (req, res) => {
  try {
    // Check database connection
    await db.query('SELECT 1');
    
    // Check Redis connection
    await redis.ping();
    
    res.json({ status: 'ready' });
  } catch (error) {
    res.status(503).json({ status: 'not ready', error: error.message });
  }
});
```

### Metrics Collection

```yaml
# docker/monitoring/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'free-deep-research'
    static_configs:
      - targets: ['app:9090']
    metrics_path: /metrics
    scrape_interval: 5s

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']

  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']
```

### Logging Configuration

```yaml
# docker/logging/fluentd.conf
<source>
  @type forward
  port 24224
  bind 0.0.0.0
</source>

<match app.**>
  @type elasticsearch
  host elasticsearch
  port 9200
  index_name free-deep-research
  type_name _doc
</match>
```

## 🔒 Security Configuration

### SSL/TLS Configuration

```nginx
# nginx/ssl.conf
server {
    listen 443 ssl http2;
    server_name your-domain.com;

    ssl_certificate /etc/ssl/certs/your-domain.crt;
    ssl_certificate_key /etc/ssl/private/your-domain.key;
    
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-AES128-GCM-SHA256;
    ssl_prefer_server_ciphers off;
    
    add_header Strict-Transport-Security "max-age=63072000" always;
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    
    location / {
        proxy_pass http://app:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Firewall Configuration

```bash
# UFW firewall rules
ufw default deny incoming
ufw default allow outgoing
ufw allow ssh
ufw allow 80/tcp
ufw allow 443/tcp
ufw enable
```

## 🚨 Troubleshooting

### Common Issues

1. **Container Won't Start**
   ```bash
   # Check logs
   docker-compose logs app
   
   # Check resource usage
   docker stats
   
   # Verify environment variables
   docker-compose config
   ```

2. **Database Connection Issues**
   ```bash
   # Test database connectivity
   docker-compose exec postgres psql -U $POSTGRES_USER -d $POSTGRES_DB
   
   # Check database logs
   docker-compose logs postgres
   ```

3. **Performance Issues**
   ```bash
   # Monitor resource usage
   docker stats
   
   # Check application metrics
   curl http://localhost:9090/metrics
   ```

### Deployment Checklist

- [ ] Environment variables configured
- [ ] SSL certificates installed
- [ ] Database migrations run
- [ ] Health checks passing
- [ ] Monitoring configured
- [ ] Backups configured
- [ ] Security headers enabled
- [ ] Rate limiting configured
- [ ] Logging configured
- [ ] Alerts configured

---

**Next**: Check out [Docker Implementation Summary](./DOCKER-IMPLEMENTATION-SUMMARY.md) for detailed Docker setup instructions.
