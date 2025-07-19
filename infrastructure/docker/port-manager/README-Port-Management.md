# Intelligent Docker Port Management System

## 🎯 Overview

The Free Deep Research System includes an intelligent port management system that automatically handles port allocation, conflict prevention, and container lifecycle management. This system eliminates port conflicts and provides a seamless development and deployment experience.

## 🚀 Key Features

### ✅ Dynamic Port Allocation
- **Random Port Assignment**: Automatically assigns ports from safe ranges (30000-65535)
- **Service-Specific Ranges**: Different port ranges for different service types
- **Conflict Prevention**: Scans existing Docker containers and system ports
- **Persistent Assignments**: Port assignments persist across container restarts

### ✅ Container Lifecycle Management
- **Existing Container Detection**: Automatically detects existing containers
- **User Choice Options**: Reuse, replace, run alongside, or clean up
- **Graceful Shutdown**: Proper port cleanup on container stop
- **Health Monitoring**: Continuous health checks and port monitoring

### ✅ Port Conflict Prevention
- **System-wide Scanning**: Checks both Docker and system-level port usage
- **Retry Logic**: Exponential backoff for port allocation failures
- **Lock Mechanism**: Prevents race conditions during allocation
- **Orphan Cleanup**: Removes stale port reservations

## 📁 Components

### Core Scripts
- `port-manager.sh` - Main port management script (Unix/Linux/macOS)
- `port-manager.bat` - Windows port management script
- `container-lifecycle.sh` - Container lifecycle management
- `port-status-service.py` - HTTP service for port status and discovery

### Generated Files
- `.env.ports` - Dynamic port registry (auto-generated)
- `/tmp/fdr-port-locks/` - Port lock files (Unix/Linux/macOS)
- `%TEMP%\fdr-port-locks\` - Port lock files (Windows)

## 🔧 Port Ranges

The system uses predefined port ranges for different service types:

| Service Type | Port Range | Services |
|--------------|------------|----------|
| **Frontend** | 30000-35000 | React frontend, static assets |
| **Backend** | 35000-40000 | Rust API server, debug ports |
| **Database** | 40000-45000 | PostgreSQL, SQLite access |
| **Redis** | 45000-50000 | Redis cache, Redis Commander |
| **Nginx** | 50000-55000 | Reverse proxy, load balancer |
| **Monitoring** | 55000-60000 | Prometheus, Grafana |
| **DevTools** | 60000-65000 | Adminer, Mailhog, dev dashboard |

## 🛠️ Usage

### Basic Commands

```bash
# Generate port registry for development
./docker/port-manager/port-manager.sh generate development

# Generate port registry for production
./docker/port-manager/port-manager.sh generate production

# Show current port status
./docker/port-manager/port-manager.sh status

# Clean up all port allocations
./docker/port-manager/port-manager.sh cleanup

# Regenerate port allocations
./docker/port-manager/port-manager.sh regenerate development
```

### Container Lifecycle Management

```bash
# Scan for existing containers
./docker/port-manager/container-lifecycle.sh scan

# Manage existing containers
./docker/port-manager/container-lifecycle.sh manage development

# Show container status
./docker/port-manager/container-lifecycle.sh status

# Clean up all containers
./docker/port-manager/container-lifecycle.sh cleanup

# Check specific container health
./docker/port-manager/container-lifecycle.sh health container-name
```

### Port Status Service

```bash
# Start port status service
python3 docker/port-manager/port-status-service.py

# Access dashboard
open http://localhost:8084/

# API endpoints
curl http://localhost:8084/ports      # Port status
curl http://localhost:8084/services   # Service discovery
curl http://localhost:8084/containers # Container status
curl http://localhost:8084/health     # Health check
```

## 🔄 Integration with Setup Scripts

The port management system is automatically integrated into the main setup scripts:

### Automatic Integration
- **setup.sh** / **setup.bat** automatically call port management
- Port registry is generated before container startup
- Existing containers are detected and managed
- Port assignments are merged into environment files

### Manual Override
```bash
# Skip port management (use default ports)
./setup.sh --skip-port-management

# Force port regeneration
./setup.sh --regenerate-ports

# Use specific port range
export PORT_RANGE_OFFSET=1000
./setup.sh
```

## 📊 Port Registry Format

The `.env.ports` file contains dynamic port assignments:

```bash
# Free Deep Research System - Port Registry
# Generated on: 2025-07-19
# Environment: development

FRONTEND_PORT=31245
BACKEND_PORT=36789
DB_PORT=41234
REDIS_PORT=46789
HTTP_PORT=51234
HTTPS_PORT=51235
PROMETHEUS_PORT=56789
GRAFANA_PORT=57890
ADMINER_PORT=61234
REDIS_COMMANDER_PORT=62345
MAILHOG_WEB_PORT=63456
MAILHOG_SMTP_PORT=63457
DEV_DASHBOARD_PORT=64567

# Port allocation metadata
PORT_REGISTRY_VERSION=1.0.0
PORT_ALLOCATION_DATE=2025-07-19T10:30:00Z
PORT_ALLOCATION_ENVIRONMENT=development
PORT_ALLOCATION_PROJECT=free-deep-research

# Service URLs (for reference)
# Frontend: http://localhost:31245
# Backend API: http://localhost:36789
# Grafana: http://localhost:57890
# Prometheus: http://localhost:56789
```

## 🔍 Container Management Options

When existing containers are detected, the system provides these options:

### 1. Reuse Existing Containers ✅
- **Best for**: Same version containers that are healthy
- **Action**: Starts stopped containers, keeps running ones
- **Ports**: Uses existing port assignments
- **Data**: Preserves all data and configurations

### 2. Stop and Replace 🔄
- **Best for**: Updating to new version or fixing issues
- **Action**: Stops and removes existing containers
- **Ports**: Generates new port assignments
- **Data**: Preserves volumes, recreates containers

### 3. Run Alongside 🚀
- **Best for**: Development testing with multiple versions
- **Action**: Creates new containers with different names/ports
- **Ports**: Generates completely new port assignments
- **Data**: Separate data volumes for each instance

### 4. Clean Up and Start Fresh 🧹
- **Best for**: Complete reset or troubleshooting
- **Action**: Removes containers, volumes, and networks
- **Ports**: Generates fresh port assignments
- **Data**: ⚠️ **DESTROYS ALL DATA** - use with caution

### 5. Cancel and Exit ❌
- **Action**: Exits without making changes
- **Use**: When you need to backup data or investigate first

## 🔒 Security Considerations

### Port Range Security
- **Safe Ranges**: Uses high-numbered ports (30000+) to avoid system conflicts
- **Non-Privileged**: All ports are above 1024 (no root required)
- **Firewall Friendly**: Predictable ranges for firewall configuration

### Lock File Security
- **Temporary Storage**: Lock files stored in system temp directory
- **Process Tracking**: Includes PID for orphan detection
- **Automatic Cleanup**: Removes stale locks on startup

### Container Security
- **Name Isolation**: Uses project-specific container names
- **Network Isolation**: Containers run in isolated Docker networks
- **Resource Limits**: Configurable CPU and memory limits

## 🐛 Troubleshooting

### Common Issues

#### Port Already in Use
```bash
# Check what's using the port
netstat -tulpn | grep :8080
lsof -i :8080

# Regenerate ports to avoid conflicts
./docker/port-manager/port-manager.sh regenerate development
```

#### Port Registry Not Found
```bash
# Generate new port registry
./docker/port-manager/port-manager.sh generate development

# Check if file was created
ls -la .env.ports
```

#### Container Detection Issues
```bash
# Check Docker daemon
docker info

# Scan containers manually
./docker/port-manager/container-lifecycle.sh scan

# Check container status
docker ps -a --filter "name=free-deep-research"
```

#### Lock File Issues
```bash
# Clean up orphaned locks
./docker/port-manager/port-manager.sh cleanup

# Check lock directory
ls -la /tmp/fdr-port-locks/  # Unix/Linux/macOS
dir %TEMP%\fdr-port-locks\   # Windows
```

### Debug Mode

Enable verbose logging for troubleshooting:

```bash
# Enable debug mode
export FDR_DEBUG=true
export FDR_VERBOSE=true

# Run with debug output
./docker/port-manager/port-manager.sh generate development
```

### Log Files

Check log files for detailed information:

```bash
# Setup script logs
tail -f setup.log

# Docker Compose logs
docker-compose logs -f

# Port status service logs
tail -f /tmp/port-status-service.log
```

## 🔧 Advanced Configuration

### Custom Port Ranges

Override default port ranges:

```bash
# Set custom ranges in environment
export FDR_FRONTEND_RANGE="40000-45000"
export FDR_BACKEND_RANGE="45000-50000"

# Run setup with custom ranges
./setup.sh
```

### Service-Specific Ports

Force specific ports for services:

```bash
# Set specific ports in .env file
FRONTEND_PORT=3000
BACKEND_PORT=8080
GRAFANA_PORT=3001

# Skip port management for these services
export FDR_SKIP_PORT_SERVICES="frontend,backend,grafana"
./setup.sh
```

### Container Naming

Customize container naming:

```bash
# Set custom project name
export COMPOSE_PROJECT_NAME="my-fdr-instance"

# Use timestamp-based names
export FDR_USE_TIMESTAMP_NAMES=true

./setup.sh
```

## 📈 Monitoring and Metrics

### Port Status Dashboard

Access the web dashboard at `http://localhost:8084/`:

- **Real-time Status**: Live port and service status
- **Service Discovery**: Clickable links to all services
- **Container Health**: Container status and health checks
- **Auto-refresh**: Updates every 30 seconds

### API Endpoints

```bash
# Get port status (JSON)
curl http://localhost:8084/ports

# Get service discovery (JSON)
curl http://localhost:8084/services

# Get container status (JSON)
curl http://localhost:8084/containers

# Health check
curl http://localhost:8084/health
```

### Integration with Monitoring

The port management system integrates with Prometheus and Grafana:

- **Metrics Export**: Port usage and allocation metrics
- **Alerts**: Notifications for port conflicts or failures
- **Dashboards**: Visual monitoring of port assignments

## 🤝 Contributing

### Adding New Services

To add a new service to the port management system:

1. **Define Port Range**: Add to `PORT_RANGES` in `port-manager.sh`
2. **Add Service Mapping**: Update `SERVICE_PORTS` array
3. **Update Docker Compose**: Use environment variables for ports
4. **Test Integration**: Verify with setup scripts

### Extending Functionality

- **Custom Allocators**: Add new port allocation strategies
- **Health Checks**: Implement service-specific health checks
- **Notifications**: Add alert integrations (Slack, Discord, etc.)
- **Metrics**: Export custom metrics for monitoring

## 📄 License

This port management system is part of the Free Deep Research System and is licensed under the same terms as the main project.
