#!/bin/bash

# Free Deep Research System - Docker Setup Script
# Version 3.0.0 "Global Intelligence Network"
# Compatible with: Linux, macOS, WSL

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_NAME="free-deep-research"
VERSION="3.0.0"

# Default values
ENVIRONMENT="development"
SKIP_DEPS=false
SKIP_SSL=false
SKIP_MIGRATION=false
VERBOSE=false

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}$1${NC}"
}

# Function to show usage
show_usage() {
    cat << EOF
Free Deep Research System - Docker Setup Script v${VERSION}

Usage: $0 [OPTIONS]

OPTIONS:
    -e, --environment ENV    Set environment (development|production) [default: development]
    -s, --skip-deps         Skip dependency checks
    --skip-ssl              Skip SSL certificate generation
    --skip-migration        Skip database migration
    -v, --verbose           Enable verbose output
    -h, --help              Show this help message

EXAMPLES:
    $0                      # Setup development environment
    $0 -e production        # Setup production environment
    $0 --skip-deps -v       # Skip dependency checks with verbose output

ENVIRONMENT VARIABLES:
    FDR_ENVIRONMENT         Override environment setting
    FDR_SKIP_DEPS          Skip dependency checks (true/false)
    FDR_VERBOSE            Enable verbose output (true/false)

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -e|--environment)
                ENVIRONMENT="$2"
                shift 2
                ;;
            -s|--skip-deps)
                SKIP_DEPS=true
                shift
                ;;
            --skip-ssl)
                SKIP_SSL=true
                shift
                ;;
            --skip-migration)
                SKIP_MIGRATION=true
                shift
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done

    # Override with environment variables
    ENVIRONMENT="${FDR_ENVIRONMENT:-$ENVIRONMENT}"
    SKIP_DEPS="${FDR_SKIP_DEPS:-$SKIP_DEPS}"
    VERBOSE="${FDR_VERBOSE:-$VERBOSE}"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check system requirements
check_requirements() {
    if [[ "$SKIP_DEPS" == "true" ]]; then
        print_warning "Skipping dependency checks as requested"
        return 0
    fi

    print_status "Checking system requirements..."

    local missing_deps=()

    # Check Docker
    if ! command_exists docker; then
        missing_deps+=("docker")
    else
        # Check Docker version
        local docker_version=$(docker --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        print_status "Found Docker version: $docker_version"
    fi

    # Check Docker Compose
    if ! command_exists docker-compose && ! docker compose version >/dev/null 2>&1; then
        missing_deps+=("docker-compose")
    else
        if command_exists docker-compose; then
            local compose_version=$(docker-compose --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
            print_status "Found Docker Compose version: $compose_version"
        else
            print_status "Found Docker Compose (plugin version)"
        fi
    fi

    # Check curl
    if ! command_exists curl; then
        missing_deps+=("curl")
    fi

    # Check openssl for SSL certificate generation
    if ! command_exists openssl && [[ "$SKIP_SSL" != "true" ]]; then
        missing_deps+=("openssl")
    fi

    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        print_error "Missing required dependencies: ${missing_deps[*]}"
        print_status "Please install the missing dependencies and run this script again."
        
        # Provide installation instructions based on OS
        if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            print_status "On Ubuntu/Debian: sudo apt-get update && sudo apt-get install ${missing_deps[*]}"
            print_status "On CentOS/RHEL: sudo yum install ${missing_deps[*]}"
        elif [[ "$OSTYPE" == "darwin"* ]]; then
            print_status "On macOS with Homebrew: brew install ${missing_deps[*]}"
        fi
        
        exit 1
    fi

    print_success "All system requirements satisfied"
}

# Function to setup port management
setup_port_management() {
    print_status "Setting up intelligent port management..."

    # Make port manager executable
    chmod +x docker/port-manager/port-manager.sh
    chmod +x docker/port-manager/container-lifecycle.sh

    # Generate port registry
    if docker/port-manager/port-manager.sh generate "$ENVIRONMENT"; then
        print_success "Port registry generated successfully"
    else
        print_error "Failed to generate port registry"
        exit 1
    fi

    # Check for existing containers and manage them
    if docker/port-manager/container-lifecycle.sh manage "$ENVIRONMENT"; then
        print_success "Container lifecycle management completed"
    else
        print_warning "Container lifecycle management had issues, but continuing..."
    fi
}

# Function to setup environment files
setup_environment() {
    print_status "Setting up environment configuration for: $ENVIRONMENT"

    local env_file=".env"
    local source_env_file

    case $ENVIRONMENT in
        development)
            source_env_file=".env.dev"
            ;;
        production)
            source_env_file=".env.prod"
            ;;
        *)
            print_error "Invalid environment: $ENVIRONMENT. Must be 'development' or 'production'"
            exit 1
            ;;
    esac

    # Copy environment file
    if [[ -f "$source_env_file" ]]; then
        cp "$source_env_file" "$env_file"
        print_success "Environment file copied from $source_env_file to $env_file"
    else
        print_warning "Source environment file $source_env_file not found, using .env.example"
        if [[ -f ".env.example" ]]; then
            cp ".env.example" "$env_file"
        else
            print_error "No environment template found!"
            exit 1
        fi
    fi

    # Set build date
    local build_date=$(date -u +"%Y-%m-%d")
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/BUILD_DATE=.*/BUILD_DATE=$build_date/" "$env_file"
    else
        sed -i "s/BUILD_DATE=.*/BUILD_DATE=$build_date/" "$env_file"
    fi

    # Setup port management
    setup_port_management

    # Merge port registry with environment file
    if [[ -f ".env.ports" ]]; then
        print_status "Merging port registry with environment file..."
        echo "" >> "$env_file"
        echo "# Dynamic Port Assignments (Auto-generated)" >> "$env_file"
        cat .env.ports >> "$env_file"
        print_success "Port assignments merged into environment file"
    fi

    print_success "Environment configuration completed"
}

# Function to create necessary directories
create_directories() {
    print_status "Creating necessary directories..."

    local directories=(
        "docker/nginx/logs"
        "docker/nginx/ssl"
        "docker/nginx/cache"
        "docker/nginx/html"
        "docker/backend/logs"
        "docker/backend/uploads"
        "docker/backend/cache"
        "docker/backend/ssl"
        "docker/frontend/logs"
        "docker/database/backups"
        "docker/database/init"
        "docker/redis"
        "docker/prometheus"
        "docker/grafana/provisioning/dashboards"
        "docker/grafana/provisioning/datasources"
        "docker/grafana/dashboards"
        "docker/loki"
        "docker/backup"
        "bmad-agent/free-deep-research/data"
    )

    for dir in "${directories[@]}"; do
        mkdir -p "$dir"
        if [[ "$VERBOSE" == "true" ]]; then
            print_status "Created directory: $dir"
        fi
    done

    print_success "Directory structure created"
}

# Function to generate SSL certificates
generate_ssl_certificates() {
    if [[ "$SKIP_SSL" == "true" ]]; then
        print_warning "Skipping SSL certificate generation as requested"
        return 0
    fi

    print_status "Generating SSL certificates..."

    local ssl_dir="docker/nginx/ssl"
    local cert_file="$ssl_dir/cert.pem"
    local key_file="$ssl_dir/key.pem"
    local dhparam_file="$ssl_dir/dhparam.pem"

    # Generate self-signed certificate for development
    if [[ "$ENVIRONMENT" == "development" ]]; then
        openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
            -keyout "$key_file" \
            -out "$cert_file" \
            -subj "/C=US/ST=Development/L=Development/O=Free Deep Research/OU=Development/CN=localhost" \
            >/dev/null 2>&1

        print_success "Self-signed SSL certificate generated for development"
    else
        print_warning "Production environment detected. Please replace the generated certificates with valid ones."
        
        # Generate temporary certificates for production setup
        openssl req -x509 -nodes -days 30 -newkey rsa:2048 \
            -keyout "$key_file" \
            -out "$cert_file" \
            -subj "/C=US/ST=Production/L=Production/O=Free Deep Research/OU=Production/CN=your-domain.com" \
            >/dev/null 2>&1
    fi

    # Generate Diffie-Hellman parameters
    if [[ ! -f "$dhparam_file" ]]; then
        print_status "Generating Diffie-Hellman parameters (this may take a while)..."
        openssl dhparam -out "$dhparam_file" 2048 >/dev/null 2>&1
        print_success "Diffie-Hellman parameters generated"
    fi
}

# Function to setup configuration files
setup_configuration_files() {
    print_status "Setting up configuration files..."

    # Create nginx configuration
    create_nginx_config

    # Create redis configuration
    create_redis_config

    # Create prometheus configuration
    create_prometheus_config

    # Create grafana configuration
    create_grafana_config

    print_success "Configuration files created"
}

# Function to create nginx configuration
create_nginx_config() {
    local nginx_conf="docker/nginx/nginx.conf"
    
    cat > "$nginx_conf" << 'EOF'
user nginx;
worker_processes auto;
error_log /var/log/nginx/error.log warn;
pid /var/run/nginx.pid;

events {
    worker_connections 1024;
    use epoll;
    multi_accept on;
}

http {
    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # Logging
    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';
    access_log /var/log/nginx/access.log main;

    # Performance
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/xml+rss application/json;

    # Security headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_req_zone $binary_remote_addr zone=login:10m rate=1r/s;

    # Upstream servers
    upstream backend {
        server backend:8080;
    }

    upstream frontend {
        server frontend:80;
    }

    # HTTP server (redirect to HTTPS in production)
    server {
        listen 80;
        server_name _;

        # Health check endpoint
        location /nginx-health {
            access_log off;
            return 200 "healthy\n";
            add_header Content-Type text/plain;
        }

        # Redirect to HTTPS in production
        location / {
            return 301 https://$server_name$request_uri;
        }
    }

    # HTTPS server
    server {
        listen 443 ssl http2;
        server_name _;

        # SSL configuration
        ssl_certificate /etc/nginx/ssl/cert.pem;
        ssl_certificate_key /etc/nginx/ssl/key.pem;
        ssl_dhparam /etc/nginx/ssl/dhparam.pem;

        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
        ssl_prefer_server_ciphers off;
        ssl_session_cache shared:SSL:10m;
        ssl_session_timeout 10m;

        # API routes
        location /api/ {
            limit_req zone=api burst=20 nodelay;
            proxy_pass http://backend/;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # WebSocket support
        location /ws {
            proxy_pass http://backend;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Frontend routes
        location / {
            proxy_pass http://frontend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
EOF
}

# Function to create redis configuration
create_redis_config() {
    local redis_conf="docker/redis/redis.conf"
    
    cat > "$redis_conf" << 'EOF'
# Redis configuration for Free Deep Research System

# Network
bind 0.0.0.0
port 6379
timeout 300
tcp-keepalive 60

# General
daemonize no
supervised no
pidfile /var/run/redis_6379.pid
loglevel notice
logfile ""

# Snapshotting
save 900 1
save 300 10
save 60 10000
stop-writes-on-bgsave-error yes
rdbcompression yes
rdbchecksum yes
dbfilename dump.rdb
dir /data

# Replication
replica-serve-stale-data yes
replica-read-only yes

# Security
# requirepass will be set via environment variable

# Memory management
maxmemory-policy allkeys-lru

# Append only file
appendonly yes
appendfilename "appendonly.aof"
appendfsync everysec
no-appendfsync-on-rewrite no
auto-aof-rewrite-percentage 100
auto-aof-rewrite-min-size 64mb
aof-load-truncated yes
EOF
}

# Function to create prometheus configuration
create_prometheus_config() {
    local prometheus_conf="docker/prometheus/prometheus.yml"
    
    cat > "$prometheus_conf" << 'EOF'
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "rules/*.yml"

scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']

  - job_name: 'free-deep-research-backend'
    static_configs:
      - targets: ['backend:8080']
    metrics_path: '/metrics'
    scrape_interval: 30s

  - job_name: 'free-deep-research-frontend'
    static_configs:
      - targets: ['frontend:80']
    metrics_path: '/metrics'
    scrape_interval: 30s

  - job_name: 'nginx'
    static_configs:
      - targets: ['nginx:80']
    metrics_path: '/nginx_status'
    scrape_interval: 30s

  - job_name: 'redis'
    static_configs:
      - targets: ['redis:6379']
    scrape_interval: 30s

  - job_name: 'postgres'
    static_configs:
      - targets: ['database:5432']
    scrape_interval: 30s
EOF
}

# Function to create grafana configuration
create_grafana_config() {
    local grafana_datasource="docker/grafana/provisioning/datasources/prometheus.yml"
    
    mkdir -p "$(dirname "$grafana_datasource")"
    
    cat > "$grafana_datasource" << 'EOF'
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
    editable: true
EOF
}

# Function to build and start services
start_services() {
    print_status "Building and starting services for $ENVIRONMENT environment..."

    local compose_file
    case $ENVIRONMENT in
        development)
            compose_file="docker-compose.dev.yml"
            ;;
        production)
            compose_file="docker-compose.prod.yml"
            ;;
    esac

    # Build images
    print_status "Building Docker images..."
    if [[ "$VERBOSE" == "true" ]]; then
        docker-compose -f "$compose_file" build
    else
        docker-compose -f "$compose_file" build >/dev/null 2>&1
    fi

    # Start services
    print_status "Starting services..."
    if [[ "$VERBOSE" == "true" ]]; then
        docker-compose -f "$compose_file" up -d
    else
        docker-compose -f "$compose_file" up -d >/dev/null 2>&1
    fi

    print_success "Services started successfully"
}

# Function to run database migration
run_migration() {
    if [[ "$SKIP_MIGRATION" == "true" ]]; then
        print_warning "Skipping database migration as requested"
        return 0
    fi

    print_status "Running database migration..."

    # Wait for database to be ready
    print_status "Waiting for database to be ready..."
    local max_attempts=30
    local attempt=1

    while [[ $attempt -le $max_attempts ]]; do
        if docker-compose exec -T backend curl -f http://localhost:8080/health >/dev/null 2>&1; then
            break
        fi
        
        if [[ $attempt -eq $max_attempts ]]; then
            print_error "Database failed to become ready after $max_attempts attempts"
            exit 1
        fi
        
        print_status "Attempt $attempt/$max_attempts - waiting for database..."
        sleep 5
        ((attempt++))
    done

    # Run migration
    docker-compose exec -T backend ./free-deep-research migrate || {
        print_error "Database migration failed"
        exit 1
    }

    print_success "Database migration completed"
}

# Function to show final status
show_final_status() {
    print_header "
╔══════════════════════════════════════════════════════════════════════════════╗
║                    Free Deep Research System v${VERSION}                     ║
║                        'Global Intelligence Network'                        ║
║                            Setup Complete!                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
"

    print_success "Environment: $ENVIRONMENT"
    print_success "All services are running successfully!"

    echo ""
    print_status "Service URLs (Dynamic Port Assignments):"

    # Display dynamically assigned ports
    if docker/port-manager/port-manager.sh status; then
        print_success "Port assignments displayed above"
    else
        print_warning "Could not display port assignments, using defaults:"

        if [[ "$ENVIRONMENT" == "development" ]]; then
            echo "  🌐 Frontend:           http://localhost:3000"
            echo "  🔧 Backend API:        http://localhost:8080"
            echo "  📊 Prometheus:         http://localhost:9090"
            echo "  📈 Grafana:            http://localhost:3001"
            echo "  🗄️  Database Admin:     http://localhost:8082"
            echo "  🔴 Redis Commander:    http://localhost:8083"
            echo "  📧 Mailhog:            http://localhost:8025"
            echo "  🛠️  Dev Dashboard:      http://localhost:8081"
        else
            echo "  🌐 Frontend:           https://localhost"
            echo "  🔧 Backend API:        https://localhost/api"
            echo "  📊 Prometheus:         http://localhost:9090"
            echo "  📈 Grafana:            http://localhost:3001"
        fi
    fi

    echo ""
    print_status "Useful commands:"
    echo "  📋 View logs:          docker-compose logs -f"
    echo "  🔄 Restart services:   docker-compose restart"
    echo "  🛑 Stop services:      docker-compose down"
    echo "  🧹 Clean up:           docker-compose down -v --remove-orphans"
    echo ""
    print_status "Port management commands:"
    echo "  🔍 Check port status:  docker/port-manager/port-manager.sh status"
    echo "  🔄 Regenerate ports:   docker/port-manager/port-manager.sh regenerate $ENVIRONMENT"
    echo "  🧹 Cleanup ports:      docker/port-manager/port-manager.sh cleanup"
    echo "  📊 Container status:   docker/port-manager/container-lifecycle.sh status"

    if [[ "$ENVIRONMENT" == "production" ]]; then
        echo ""
        print_warning "PRODUCTION SETUP REMINDERS:"
        echo "  1. Update all passwords in .env file"
        echo "  2. Replace SSL certificates with valid ones"
        echo "  3. Configure proper domain names"
        echo "  4. Set up proper backup procedures"
        echo "  5. Configure monitoring and alerting"
    fi

    echo ""
    print_status "For more information, see README-Docker.md"
}

# Main execution
main() {
    print_header "
╔══════════════════════════════════════════════════════════════════════════════╗
║                    Free Deep Research System v${VERSION}                     ║
║                        'Global Intelligence Network'                        ║
║                           Docker Setup Script                               ║
╚══════════════════════════════════════════════════════════════════════════════╝
"

    parse_args "$@"
    
    print_status "Starting setup for $ENVIRONMENT environment..."
    
    check_requirements
    setup_environment
    create_directories
    generate_ssl_certificates
    setup_configuration_files
    start_services
    run_migration
    show_final_status
}

# Run main function with all arguments
main "$@"
