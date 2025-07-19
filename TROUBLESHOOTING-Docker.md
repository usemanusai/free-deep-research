# Docker Troubleshooting Guide

This guide helps resolve common issues when running the Free Deep Research System with Docker.

## 🚨 Common Issues and Solutions

### 1. Port Conflicts

**Problem**: Port already in use errors
```
Error: bind: address already in use
```

**Solutions**:
```bash
# Check what's using the port
netstat -tulpn | grep :8080
lsof -i :8080

# Kill the process using the port
sudo kill -9 <PID>

# Or change ports in .env file
BACKEND_PORT=8081
FRONTEND_PORT=3001
```

### 2. Permission Issues

**Problem**: Permission denied errors
```
Permission denied: '/app/logs'
mkdir: cannot create directory: Permission denied
```

**Solutions**:
```bash
# Fix ownership of Docker directories
sudo chown -R $USER:$USER docker/
chmod -R 755 docker/

# For Linux users, add user to docker group
sudo usermod -aG docker $USER
# Then logout and login again

# Fix SELinux context (if applicable)
sudo setsebool -P container_manage_cgroup on
```

### 3. Database Connection Issues

**Problem**: Cannot connect to database
```
Connection refused
FATAL: password authentication failed
```

**Solutions**:
```bash
# Check database container status
docker-compose ps database

# Check database logs
docker-compose logs database

# Reset database
docker-compose down -v
docker-compose up -d database

# Wait for database to be ready
docker-compose exec database pg_isready -U $DB_USER

# For SQLite issues, check file permissions
ls -la bmad-agent/free-deep-research/data/
```

### 4. SSL Certificate Issues

**Problem**: SSL certificate errors
```
SSL certificate problem: self signed certificate
```

**Solutions**:
```bash
# Regenerate certificates
rm -rf docker/nginx/ssl/*
./setup.sh --skip-deps

# For development, accept self-signed certificates
# In browser: Advanced -> Proceed to localhost (unsafe)

# For production, use valid certificates
cp your-cert.pem docker/nginx/ssl/cert.pem
cp your-key.pem docker/nginx/ssl/key.pem
```

### 5. Memory Issues

**Problem**: Out of memory errors
```
Killed
Container exited with code 137
```

**Solutions**:
```bash
# Check memory usage
docker stats

# Increase Docker memory limit (Docker Desktop)
# Settings -> Resources -> Memory -> Increase limit

# Add memory limits to docker-compose.yml
services:
  backend:
    deploy:
      resources:
        limits:
          memory: 2G

# Clean up unused resources
docker system prune -a
```

### 6. Build Issues

**Problem**: Docker build failures
```
failed to solve: process "/bin/sh -c cargo build" did not complete successfully
```

**Solutions**:
```bash
# Clear build cache
docker builder prune -a

# Build with no cache
docker-compose build --no-cache

# Check Dockerfile syntax
docker build --dry-run -f docker/backend/Dockerfile .

# For Rust build issues, check dependencies
docker-compose exec backend cargo check
```

### 7. Network Issues

**Problem**: Services cannot communicate
```
Connection refused to backend:8080
```

**Solutions**:
```bash
# Check network configuration
docker network ls
docker network inspect free-deep-research_fdr-network

# Restart networking
docker-compose down
docker-compose up -d

# Check service discovery
docker-compose exec frontend nslookup backend
docker-compose exec backend nslookup database
```

### 8. Volume Issues

**Problem**: Data not persisting
```
Data lost after container restart
```

**Solutions**:
```bash
# Check volume mounts
docker-compose config

# List volumes
docker volume ls

# Inspect volume
docker volume inspect free-deep-research_postgres_data

# Backup volume data
docker run --rm -v free-deep-research_postgres_data:/data -v $(pwd):/backup alpine tar czf /backup/volume-backup.tar.gz /data
```

## 🔍 Debugging Commands

### Container Inspection
```bash
# Check container status
docker-compose ps

# View container logs
docker-compose logs -f backend
docker-compose logs --tail=100 frontend

# Access container shell
docker-compose exec backend bash
docker-compose exec frontend sh

# Inspect container configuration
docker inspect <container_name>
```

### Resource Monitoring
```bash
# Monitor resource usage
docker stats

# Check disk usage
docker system df

# Monitor specific container
docker stats <container_name>
```

### Network Debugging
```bash
# Test connectivity between containers
docker-compose exec frontend ping backend
docker-compose exec backend curl -f http://frontend:80/health

# Check port bindings
docker port <container_name>

# Test external connectivity
curl -f http://localhost:8080/health
curl -f http://localhost:3000
```

### Database Debugging
```bash
# Connect to PostgreSQL
docker-compose exec database psql -U $DB_USER $DB_NAME

# Check database status
docker-compose exec database pg_isready -U $DB_USER

# View database logs
docker-compose logs database

# For SQLite
docker-compose exec backend sqlite3 /data/research.db ".tables"
```

### Redis Debugging
```bash
# Connect to Redis
docker-compose exec redis redis-cli

# Check Redis status
docker-compose exec redis redis-cli ping

# Monitor Redis commands
docker-compose exec redis redis-cli monitor
```

## 🛠️ Advanced Troubleshooting

### Performance Issues

**Slow startup times**:
```bash
# Check resource limits
docker-compose config

# Monitor startup process
docker-compose up --no-deps backend

# Check for resource constraints
docker stats --no-stream
```

**High CPU usage**:
```bash
# Identify resource-heavy containers
docker stats --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}"

# Check for infinite loops in logs
docker-compose logs backend | grep -i error
```

### Security Issues

**Container security scanning**:
```bash
# Scan images for vulnerabilities
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy image fdr-backend:latest

# Check for security best practices
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  docker/docker-bench-security
```

### Data Recovery

**Recover from corrupted volumes**:
```bash
# Stop services
docker-compose down

# Backup existing volume
docker run --rm -v free-deep-research_postgres_data:/data -v $(pwd):/backup \
  alpine tar czf /backup/corrupted-volume.tar.gz /data

# Remove corrupted volume
docker volume rm free-deep-research_postgres_data

# Restore from backup
docker volume create free-deep-research_postgres_data
docker run --rm -v free-deep-research_postgres_data:/data -v $(pwd):/backup \
  alpine tar xzf /backup/good-backup.tar.gz -C /
```

## 📊 Health Checks

### Service Health Verification
```bash
# Check all service health
for service in backend frontend database redis nginx; do
  echo "Checking $service..."
  docker-compose exec $service curl -f http://localhost/health || echo "$service unhealthy"
done

# Automated health check script
cat > health-check.sh << 'EOF'
#!/bin/bash
services=("backend:8080" "frontend:80" "database:5432" "redis:6379")
for service in "${services[@]}"; do
  name=${service%:*}
  port=${service#*:}
  if docker-compose exec $name nc -z localhost $port; then
    echo "✅ $name is healthy"
  else
    echo "❌ $name is unhealthy"
  fi
done
EOF
chmod +x health-check.sh
./health-check.sh
```

### Log Analysis
```bash
# Search for errors across all services
docker-compose logs | grep -i error

# Monitor logs in real-time
docker-compose logs -f | grep -E "(ERROR|FATAL|CRITICAL)"

# Export logs for analysis
docker-compose logs > system-logs-$(date +%Y%m%d_%H%M%S).log
```

## 🔄 Recovery Procedures

### Complete System Reset
```bash
# Stop all services
docker-compose down -v --remove-orphans

# Remove all containers and images
docker system prune -a --volumes

# Remove project-specific resources
docker volume prune
docker network prune

# Restart from scratch
./setup.sh
```

### Partial Recovery
```bash
# Restart specific service
docker-compose restart backend

# Rebuild and restart service
docker-compose up -d --build backend

# Reset service data
docker-compose stop backend
docker volume rm free-deep-research_backend_data
docker-compose up -d backend
```

## 📞 Getting Help

### Information to Collect
When reporting issues, include:

1. **System Information**:
```bash
# Docker version
docker --version
docker-compose --version

# System info
uname -a
cat /etc/os-release

# Resource usage
free -h
df -h
```

2. **Container Information**:
```bash
# Container status
docker-compose ps

# Service logs
docker-compose logs --tail=50 backend > backend-logs.txt
docker-compose logs --tail=50 frontend > frontend-logs.txt
```

3. **Configuration**:
```bash
# Environment configuration (remove sensitive data)
cat .env | grep -v PASSWORD | grep -v SECRET | grep -v KEY

# Docker Compose configuration
docker-compose config
```

### Support Channels
- GitHub Issues: Include logs and system information
- Documentation: Check README-Docker.md for setup instructions
- Community: Search existing issues for similar problems

## 🔧 Maintenance Tasks

### Regular Maintenance
```bash
# Weekly cleanup
docker system prune -f
docker volume prune -f

# Update images
docker-compose pull
docker-compose up -d

# Backup data
./docker/backup/backup-scripts/run-backup.sh

# Check security updates
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy image --severity HIGH,CRITICAL fdr-backend:latest
```

### Performance Optimization
```bash
# Optimize Docker daemon
# Add to /etc/docker/daemon.json:
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  },
  "storage-driver": "overlay2"
}

# Restart Docker daemon
sudo systemctl restart docker
```
