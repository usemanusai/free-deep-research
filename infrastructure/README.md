# Infrastructure

This directory contains all infrastructure, deployment, and operational configurations for the Free Deep Research System.

## 📁 Structure

```
infrastructure/
├── docker/                     # Docker configurations
│   ├── backend/               # Backend service configs
│   ├── frontend/              # Frontend service configs
│   ├── database/              # Database configurations
│   ├── nginx/                 # Reverse proxy configs
│   ├── redis/                 # Redis cache configs
│   ├── grafana/               # Monitoring configs
│   └── port-manager/          # Port management
├── scripts/                   # Build and deployment scripts
│   ├── setup.sh              # Initial system setup
│   ├── setup.bat             # Windows setup script
│   ├── verify-setup.sh       # Setup verification
│   ├── deploy-complete-system.sh  # Full deployment
│   └── dependency-manager/   # Dependency management tools
├── docker-compose.yml         # Default configuration
├── docker-compose.dev.yml     # Development environment
└── docker-compose.prod.yml    # Production environment
```

## 🐳 Docker Configurations

### Development Environment
```bash
# Start development environment
docker-compose -f docker-compose.dev.yml up

# With rebuild
docker-compose -f docker-compose.dev.yml up --build
```

### Production Environment
```bash
# Start production environment
docker-compose -f docker-compose.prod.yml up -d

# Scale services
docker-compose -f docker-compose.prod.yml up -d --scale backend=3
```

### Default Environment
```bash
# Quick start with defaults
docker-compose up
```

## 🔧 Scripts

### Setup Scripts
- **`setup.sh`**: Linux/macOS system setup
- **`setup.bat`**: Windows system setup
- **`verify-setup.sh`**: Verify installation and dependencies

### Deployment Scripts
- **`deploy-complete-system.sh`**: Full system deployment
- **`dependency-manager/`**: Dependency health checking and management

### Usage
```bash
# Initial setup
./infrastructure/scripts/setup.sh

# Verify setup
./infrastructure/scripts/verify-setup.sh

# Deploy full system
./infrastructure/scripts/deploy-complete-system.sh
```

## 🏗️ Services

### Core Services
- **Frontend**: React web application
- **Backend**: Tauri/Rust backend services
- **Database**: PostgreSQL with Redis cache
- **Nginx**: Reverse proxy and load balancer

### Monitoring & Operations
- **Grafana**: Metrics and monitoring dashboards
- **Redis**: Caching and session storage
- **Port Manager**: Dynamic port allocation

### Development Services
- **Hot Reload**: Development file watching
- **Debug Tools**: Debugging and profiling tools
- **Test Runners**: Automated testing services

## 🔒 Security

### Production Security
- SSL/TLS termination at Nginx
- Environment variable management
- Secret management with Docker secrets
- Network isolation between services

### Development Security
- Local certificate generation
- Development-only credentials
- Isolated development networks

## 📊 Monitoring

### Health Checks
- Service health monitoring
- Dependency health verification
- Performance metrics collection

### Logging
- Centralized logging with structured formats
- Log aggregation and analysis
- Error tracking and alerting

## 🚀 Deployment Options

### Local Development
```bash
# Quick development setup
docker-compose -f docker-compose.dev.yml up
```

### Staging Environment
```bash
# Staging deployment
docker-compose -f docker-compose.prod.yml up -d
```

### Production Deployment
```bash
# Production with scaling
docker-compose -f docker-compose.prod.yml up -d --scale backend=3 --scale frontend=2
```

## 🔧 Configuration

### Environment Variables
- **Development**: `.env.dev`
- **Production**: `.env.prod`
- **Local**: `.env.local`

### Service Configuration
- **Nginx**: `docker/nginx/nginx.conf`
- **Database**: `docker/database/`
- **Redis**: `docker/redis/redis.conf`

## 📚 Documentation

- [Docker Deployment Guide](../docs/deployment/DOCKER-IMPLEMENTATION-SUMMARY.md)
- [Troubleshooting Guide](../docs/deployment/TROUBLESHOOTING-Docker.md)
- [Port Management](../docs/deployment/INTELLIGENT-PORT-MANAGEMENT-SUMMARY.md)

## 🤝 Contributing

When contributing to infrastructure:
1. Test changes in development environment first
2. Update documentation for configuration changes
3. Ensure backward compatibility
4. Follow security best practices
5. Update monitoring and health checks

For detailed guidelines, see [Contributing Guide](../apps/desktop/CONTRIBUTING.md).
