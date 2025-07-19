#!/bin/bash

# Free Deep Research System - Intelligent Port Manager
# Handles dynamic port allocation, conflict prevention, and lifecycle management

set -e

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
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PORT_REGISTRY_FILE="$PROJECT_ROOT/.env.ports"
PORT_LOCK_DIR="/tmp/fdr-port-locks"
PROJECT_NAME="${COMPOSE_PROJECT_NAME:-free-deep-research}"

# Port ranges for different service types
declare -A PORT_RANGES=(
    ["frontend"]="30000-35000"
    ["backend"]="35000-40000"
    ["database"]="40000-45000"
    ["redis"]="45000-50000"
    ["nginx"]="50000-55000"
    ["monitoring"]="55000-60000"
    ["devtools"]="60000-65000"
)

# Service port mappings
declare -A SERVICE_PORTS=(
    ["frontend"]="3000"
    ["backend"]="8080"
    ["database"]="5432"
    ["redis"]="6379"
    ["nginx_http"]="80"
    ["nginx_https"]="443"
    ["prometheus"]="9090"
    ["grafana"]="3001"
    ["adminer"]="8082"
    ["redis_commander"]="8083"
    ["mailhog_web"]="8025"
    ["mailhog_smtp"]="1025"
    ["dev_dashboard"]="8081"
)

# Function to print colored output
print_status() {
    echo -e "${BLUE}[PORT-MGR]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PORT-MGR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[PORT-MGR]${NC} $1"
}

print_error() {
    echo -e "${RED}[PORT-MGR]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}$1${NC}"
}

# Function to create lock directory
create_lock_dir() {
    mkdir -p "$PORT_LOCK_DIR"
}

# Function to check if port is available
is_port_available() {
    local port=$1
    local host=${2:-localhost}
    
    # Check if port is in use by any process
    if command -v ss >/dev/null 2>&1; then
        ! ss -tuln | grep -q ":$port "
    elif command -v netstat >/dev/null 2>&1; then
        ! netstat -tuln 2>/dev/null | grep -q ":$port "
    else
        # Fallback: try to bind to the port
        ! timeout 1 bash -c "echo >/dev/tcp/$host/$port" 2>/dev/null
    fi
}

# Function to check Docker container port usage
get_docker_used_ports() {
    if command -v docker >/dev/null 2>&1; then
        docker ps --format "table {{.Ports}}" | grep -oE '[0-9]+:[0-9]+' | cut -d: -f1 | sort -n | uniq
    fi
}

# Function to get random port from range
get_random_port_from_range() {
    local range=$1
    local min_port=$(echo "$range" | cut -d- -f1)
    local max_port=$(echo "$range" | cut -d- -f2)
    
    echo $((min_port + RANDOM % (max_port - min_port + 1)))
}

# Function to allocate port with conflict prevention
allocate_port() {
    local service_type=$1
    local service_name=$2
    local max_attempts=50
    local attempt=1
    
    local range=${PORT_RANGES[$service_type]}
    if [[ -z "$range" ]]; then
        print_error "Unknown service type: $service_type"
        return 1
    fi
    
    print_status "Allocating port for $service_name (type: $service_type, range: $range)"
    
    while [[ $attempt -le $max_attempts ]]; do
        local port=$(get_random_port_from_range "$range")
        local lock_file="$PORT_LOCK_DIR/$port.lock"
        
        # Check if port is locked by our system
        if [[ -f "$lock_file" ]]; then
            ((attempt++))
            continue
        fi
        
        # Check if port is available system-wide
        if is_port_available "$port"; then
            # Create lock file
            echo "$service_name:$(date):$$" > "$lock_file"
            
            # Double-check port is still available after lock
            if is_port_available "$port"; then
                print_success "Allocated port $port for $service_name"
                echo "$port"
                return 0
            else
                # Remove lock if port became unavailable
                rm -f "$lock_file"
            fi
        fi
        
        ((attempt++))
        
        # Exponential backoff
        if [[ $attempt -gt 10 ]]; then
            sleep $((attempt / 10))
        fi
    done
    
    print_error "Failed to allocate port for $service_name after $max_attempts attempts"
    return 1
}

# Function to release port
release_port() {
    local port=$1
    local lock_file="$PORT_LOCK_DIR/$port.lock"
    
    if [[ -f "$lock_file" ]]; then
        rm -f "$lock_file"
        print_status "Released port $port"
    fi
}

# Function to cleanup orphaned locks
cleanup_orphaned_locks() {
    print_status "Cleaning up orphaned port locks..."
    
    if [[ ! -d "$PORT_LOCK_DIR" ]]; then
        return 0
    fi
    
    local cleaned=0
    for lock_file in "$PORT_LOCK_DIR"/*.lock; do
        [[ -f "$lock_file" ]] || continue
        
        local port=$(basename "$lock_file" .lock)
        local lock_content=$(cat "$lock_file" 2>/dev/null || echo "")
        local pid=$(echo "$lock_content" | cut -d: -f3)
        
        # Check if process is still running
        if [[ -n "$pid" ]] && ! kill -0 "$pid" 2>/dev/null; then
            rm -f "$lock_file"
            ((cleaned++))
            print_status "Cleaned orphaned lock for port $port (PID $pid no longer exists)"
        fi
    done
    
    if [[ $cleaned -gt 0 ]]; then
        print_success "Cleaned up $cleaned orphaned port locks"
    else
        print_status "No orphaned port locks found"
    fi
}

# Function to scan existing containers
scan_existing_containers() {
    print_status "Scanning for existing containers..."
    
    local existing_containers=()
    if command -v docker >/dev/null 2>&1; then
        # Get containers with our project name
        mapfile -t existing_containers < <(docker ps -a --filter "name=$PROJECT_NAME" --format "{{.Names}}")
    fi
    
    if [[ ${#existing_containers[@]} -gt 0 ]]; then
        print_warning "Found existing containers:"
        for container in "${existing_containers[@]}"; do
            local status=$(docker inspect --format='{{.State.Status}}' "$container" 2>/dev/null || echo "unknown")
            echo "  - $container ($status)"
        done
        
        echo "${existing_containers[@]}"
    else
        print_status "No existing containers found"
        echo ""
    fi
}

# Function to generate port registry
generate_port_registry() {
    local environment=${1:-development}
    
    print_header "Generating port registry for $environment environment..."
    
    # Create lock directory
    create_lock_dir
    
    # Cleanup orphaned locks
    cleanup_orphaned_locks
    
    # Check for existing containers
    local existing_containers=$(scan_existing_containers)
    
    # Create port registry file
    cat > "$PORT_REGISTRY_FILE" << EOF
# Free Deep Research System - Port Registry
# Generated on: $(date)
# Environment: $environment
# Project: $PROJECT_NAME

# WARNING: This file is auto-generated. Do not edit manually.
# Use the port manager to modify port assignments.

EOF
    
    # Allocate ports for each service
    declare -A allocated_ports
    
    # Core services
    if port=$(allocate_port "frontend" "frontend"); then
        allocated_ports["FRONTEND_PORT"]=$port
        echo "FRONTEND_PORT=$port" >> "$PORT_REGISTRY_FILE"
    fi
    
    if port=$(allocate_port "backend" "backend"); then
        allocated_ports["BACKEND_PORT"]=$port
        echo "BACKEND_PORT=$port" >> "$PORT_REGISTRY_FILE"
    fi
    
    if port=$(allocate_port "database" "database"); then
        allocated_ports["DB_PORT"]=$port
        echo "DB_PORT=$port" >> "$PORT_REGISTRY_FILE"
    fi
    
    if port=$(allocate_port "redis" "redis"); then
        allocated_ports["REDIS_PORT"]=$port
        echo "REDIS_PORT=$port" >> "$PORT_REGISTRY_FILE"
    fi
    
    if port=$(allocate_port "nginx" "nginx-http"); then
        allocated_ports["HTTP_PORT"]=$port
        echo "HTTP_PORT=$port" >> "$PORT_REGISTRY_FILE"
    fi
    
    if port=$(allocate_port "nginx" "nginx-https"); then
        allocated_ports["HTTPS_PORT"]=$port
        echo "HTTPS_PORT=$port" >> "$PORT_REGISTRY_FILE"
    fi
    
    # Monitoring services
    if port=$(allocate_port "monitoring" "prometheus"); then
        allocated_ports["PROMETHEUS_PORT"]=$port
        echo "PROMETHEUS_PORT=$port" >> "$PORT_REGISTRY_FILE"
    fi
    
    if port=$(allocate_port "monitoring" "grafana"); then
        allocated_ports["GRAFANA_PORT"]=$port
        echo "GRAFANA_PORT=$port" >> "$PORT_REGISTRY_FILE"
    fi
    
    # Development tools (only for dev environment)
    if [[ "$environment" == "development" ]]; then
        if port=$(allocate_port "devtools" "adminer"); then
            allocated_ports["ADMINER_PORT"]=$port
            echo "ADMINER_PORT=$port" >> "$PORT_REGISTRY_FILE"
        fi
        
        if port=$(allocate_port "devtools" "redis-commander"); then
            allocated_ports["REDIS_COMMANDER_PORT"]=$port
            echo "REDIS_COMMANDER_PORT=$port" >> "$PORT_REGISTRY_FILE"
        fi
        
        if port=$(allocate_port "devtools" "mailhog-web"); then
            allocated_ports["MAILHOG_WEB_PORT"]=$port
            echo "MAILHOG_WEB_PORT=$port" >> "$PORT_REGISTRY_FILE"
        fi
        
        if port=$(allocate_port "devtools" "mailhog-smtp"); then
            allocated_ports["MAILHOG_SMTP_PORT"]=$port
            echo "MAILHOG_SMTP_PORT=$port" >> "$PORT_REGISTRY_FILE"
        fi
        
        if port=$(allocate_port "devtools" "dev-dashboard"); then
            allocated_ports["DEV_DASHBOARD_PORT"]=$port
            echo "DEV_DASHBOARD_PORT=$port" >> "$PORT_REGISTRY_FILE"
        fi
    fi
    
    # Add metadata
    cat >> "$PORT_REGISTRY_FILE" << EOF

# Port allocation metadata
PORT_REGISTRY_VERSION=1.0.0
PORT_ALLOCATION_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
PORT_ALLOCATION_ENVIRONMENT=$environment
PORT_ALLOCATION_PROJECT=$PROJECT_NAME

# Service URLs (for reference)
EOF
    
    # Generate service URLs
    if [[ -n "${allocated_ports[FRONTEND_PORT]}" ]]; then
        echo "# Frontend: http://localhost:${allocated_ports[FRONTEND_PORT]}" >> "$PORT_REGISTRY_FILE"
    fi
    
    if [[ -n "${allocated_ports[BACKEND_PORT]}" ]]; then
        echo "# Backend API: http://localhost:${allocated_ports[BACKEND_PORT]}" >> "$PORT_REGISTRY_FILE"
    fi
    
    if [[ -n "${allocated_ports[GRAFANA_PORT]}" ]]; then
        echo "# Grafana: http://localhost:${allocated_ports[GRAFANA_PORT]}" >> "$PORT_REGISTRY_FILE"
    fi
    
    if [[ -n "${allocated_ports[PROMETHEUS_PORT]}" ]]; then
        echo "# Prometheus: http://localhost:${allocated_ports[PROMETHEUS_PORT]}" >> "$PORT_REGISTRY_FILE"
    fi
    
    print_success "Port registry generated: $PORT_REGISTRY_FILE"
    
    # Display allocated ports
    print_header "Allocated Ports Summary:"
    for service in "${!allocated_ports[@]}"; do
        echo "  $service: ${allocated_ports[$service]}"
    done
}

# Function to display current port status
show_port_status() {
    print_header "Current Port Status"
    
    if [[ ! -f "$PORT_REGISTRY_FILE" ]]; then
        print_warning "No port registry found. Run 'generate' command first."
        return 1
    fi
    
    print_status "Port Registry: $PORT_REGISTRY_FILE"
    echo ""
    
    # Parse and display port assignments
    while IFS='=' read -r key value; do
        [[ "$key" =~ ^[A-Z_]+_PORT$ ]] || continue
        [[ -n "$value" ]] || continue
        
        local service_name=$(echo "$key" | sed 's/_PORT$//' | tr '[:upper:]' '[:lower:]')
        local status="Unknown"
        
        if is_port_available "$value"; then
            status="${RED}Available (not in use)${NC}"
        else
            status="${GREEN}In use${NC}"
        fi
        
        printf "  %-20s: %5s (%s)\n" "$service_name" "$value" "$status"
    done < <(grep "^[A-Z_]*_PORT=" "$PORT_REGISTRY_FILE" 2>/dev/null)
    
    echo ""
    
    # Display service URLs
    print_header "Service URLs:"
    while IFS='=' read -r key value; do
        case "$key" in
            "FRONTEND_PORT")
                echo "  🌐 Frontend:        http://localhost:$value"
                ;;
            "BACKEND_PORT")
                echo "  🔧 Backend API:     http://localhost:$value"
                ;;
            "GRAFANA_PORT")
                echo "  📈 Grafana:         http://localhost:$value"
                ;;
            "PROMETHEUS_PORT")
                echo "  📊 Prometheus:      http://localhost:$value"
                ;;
            "ADMINER_PORT")
                echo "  🗄️  Database Admin:  http://localhost:$value"
                ;;
            "REDIS_COMMANDER_PORT")
                echo "  🔴 Redis Commander: http://localhost:$value"
                ;;
            "MAILHOG_WEB_PORT")
                echo "  📧 Mailhog:         http://localhost:$value"
                ;;
            "DEV_DASHBOARD_PORT")
                echo "  🛠️  Dev Dashboard:   http://localhost:$value"
                ;;
        esac
    done < <(grep "^[A-Z_]*_PORT=" "$PORT_REGISTRY_FILE" 2>/dev/null)
}

# Function to cleanup all port allocations
cleanup_ports() {
    print_status "Cleaning up port allocations..."
    
    # Remove port registry
    if [[ -f "$PORT_REGISTRY_FILE" ]]; then
        rm -f "$PORT_REGISTRY_FILE"
        print_success "Removed port registry"
    fi
    
    # Remove all lock files
    if [[ -d "$PORT_LOCK_DIR" ]]; then
        rm -rf "$PORT_LOCK_DIR"
        print_success "Removed port locks"
    fi
    
    print_success "Port cleanup completed"
}

# Function to regenerate ports
regenerate_ports() {
    local environment=${1:-development}
    
    print_status "Regenerating port allocations for $environment environment..."
    
    # Cleanup existing allocations
    cleanup_ports
    
    # Generate new allocations
    generate_port_registry "$environment"
    
    print_success "Port regeneration completed"
}

# Function to show usage
show_usage() {
    cat << EOF
Free Deep Research System - Port Manager

Usage: $0 <command> [options]

Commands:
    generate <env>     Generate port registry for environment (development|production)
    status             Show current port status and service URLs
    cleanup            Clean up all port allocations
    regenerate <env>   Regenerate port allocations for environment
    help               Show this help message

Examples:
    $0 generate development
    $0 status
    $0 cleanup
    $0 regenerate production

EOF
}

# Main execution
main() {
    case "${1:-help}" in
        generate)
            generate_port_registry "${2:-development}"
            ;;
        status)
            show_port_status
            ;;
        cleanup)
            cleanup_ports
            ;;
        regenerate)
            regenerate_ports "${2:-development}"
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            print_error "Unknown command: $1"
            show_usage
            exit 1
            ;;
    esac
}

# Execute main function
main "$@"
