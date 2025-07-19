#!/bin/bash

# Docker Manager Script for Free Deep Research Tauri Application
# This script provides easy management of Docker services for development and testing

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
COMPOSE_FILE="docker-compose.tauri.yml"
PROJECT_NAME="fdr-tauri"

# Function to print colored output
print_header() {
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE}  Free Deep Research - Docker${NC}"
    echo -e "${PURPLE}================================${NC}"
    echo ""
}

print_status() {
    echo -e "${BLUE}[$(date +'%H:%M:%S')]${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ️  $1${NC}"
}

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi
}

# Function to check if Docker Compose is available
check_docker_compose() {
    if ! command -v docker-compose > /dev/null 2>&1; then
        print_error "Docker Compose is not installed. Please install Docker Compose and try again."
        exit 1
    fi
}

# Function to show usage
show_usage() {
    print_header
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  start [profile]     Start services (profiles: cache, monitoring, testing, development)"
    echo "  stop               Stop all services"
    echo "  restart [profile]  Restart services"
    echo "  status             Show service status"
    echo "  logs [service]     Show logs for all services or specific service"
    echo "  clean              Stop services and remove volumes"
    echo "  test               Run test suite"
    echo "  build              Build custom images"
    echo "  health             Check service health"
    echo "  backup             Backup database and data"
    echo "  restore [file]     Restore from backup"
    echo "  shell [service]    Open shell in service container"
    echo "  update             Update Docker images"
    echo ""
    echo "Profiles:"
    echo "  basic              Database only (default)"
    echo "  cache              Database + Redis"
    echo "  monitoring         Database + Prometheus + Grafana"
    echo "  testing            Database + Redis + Mock Server + Test Runner"
    echo "  development        All services for development"
    echo "  full               All services including monitoring"
    echo ""
    echo "Examples:"
    echo "  $0 start                    # Start basic services (database only)"
    echo "  $0 start cache              # Start with caching"
    echo "  $0 start monitoring         # Start with monitoring"
    echo "  $0 start full               # Start all services"
    echo "  $0 logs backend             # Show backend logs"
    echo "  $0 shell database           # Open shell in database container"
    echo "  $0 test                     # Run test suite"
}

# Function to start services
start_services() {
    local profile=${1:-basic}
    
    print_status "Starting Free Deep Research services with profile: $profile"
    
    case $profile in
        basic)
            docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME up -d database
            ;;
        cache)
            docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME --profile cache up -d
            ;;
        monitoring)
            docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME --profile monitoring up -d
            ;;
        testing)
            docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME --profile testing up -d
            ;;
        development)
            docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME --profile cache --profile development up -d
            ;;
        full)
            docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME --profile cache --profile monitoring --profile testing --profile development up -d
            ;;
        *)
            print_error "Unknown profile: $profile"
            print_info "Available profiles: basic, cache, monitoring, testing, development, full"
            exit 1
            ;;
    esac
    
    print_success "Services started successfully!"
    show_service_info $profile
}

# Function to stop services
stop_services() {
    print_status "Stopping Free Deep Research services..."
    docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME down
    print_success "Services stopped successfully!"
}

# Function to restart services
restart_services() {
    local profile=${1:-basic}
    print_status "Restarting Free Deep Research services..."
    stop_services
    sleep 2
    start_services $profile
}

# Function to show service status
show_status() {
    print_status "Service Status:"
    docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME ps
}

# Function to show logs
show_logs() {
    local service=$1
    if [ -z "$service" ]; then
        print_status "Showing logs for all services..."
        docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME logs -f --tail=100
    else
        print_status "Showing logs for service: $service"
        docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME logs -f --tail=100 $service
    fi
}

# Function to clean up
clean_services() {
    print_warning "This will stop all services and remove volumes. Are you sure? (y/N)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        print_status "Cleaning up services and volumes..."
        docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME down --volumes --remove-orphans
        print_success "Cleanup completed!"
    else
        print_info "Cleanup cancelled."
    fi
}

# Function to run tests
run_tests() {
    print_status "Running test suite..."
    docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME --profile testing up --build test-runner
    print_success "Test suite completed!"
}

# Function to build images
build_images() {
    print_status "Building custom Docker images..."
    docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME build
    print_success "Images built successfully!"
}

# Function to check health
check_health() {
    print_status "Checking service health..."
    
    # Check running containers
    local containers=$(docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME ps -q)
    
    if [ -z "$containers" ]; then
        print_warning "No services are running"
        return
    fi
    
    for container in $containers; do
        local name=$(docker inspect --format='{{.Name}}' $container | sed 's/\///')
        local health=$(docker inspect --format='{{.State.Health.Status}}' $container 2>/dev/null || echo "no-healthcheck")
        local status=$(docker inspect --format='{{.State.Status}}' $container)
        
        if [ "$status" = "running" ]; then
            if [ "$health" = "healthy" ]; then
                print_success "$name: Running (Healthy)"
            elif [ "$health" = "no-healthcheck" ]; then
                print_info "$name: Running (No health check)"
            else
                print_warning "$name: Running ($health)"
            fi
        else
            print_error "$name: $status"
        fi
    done
}

# Function to backup data
backup_data() {
    print_status "Creating backup..."
    local backup_dir="./backups/$(date +%Y%m%d_%H%M%S)"
    mkdir -p "$backup_dir"
    
    # Backup database
    if docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME ps database | grep -q "Up"; then
        docker cp "${PROJECT_NAME}_database_1:/data" "$backup_dir/"
        print_success "Database backed up to: $backup_dir"
    else
        print_warning "Database service is not running"
    fi
}

# Function to open shell
open_shell() {
    local service=${1:-database}
    print_status "Opening shell in service: $service"
    docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME exec $service sh
}

# Function to update images
update_images() {
    print_status "Updating Docker images..."
    docker-compose -f $COMPOSE_FILE -p $PROJECT_NAME pull
    print_success "Images updated successfully!"
}

# Function to show service information
show_service_info() {
    local profile=$1
    echo ""
    print_info "Service Information:"
    echo ""
    
    case $profile in
        basic)
            echo "  📁 Database: SQLite at ./data/research.db"
            ;;
        cache)
            echo "  📁 Database: SQLite at ./data/research.db"
            echo "  🔄 Redis: localhost:6380 (password: tauri_redis_pass)"
            ;;
        monitoring)
            echo "  📁 Database: SQLite at ./data/research.db"
            echo "  📊 Prometheus: http://localhost:9091"
            echo "  📈 Grafana: http://localhost:3002 (admin/tauri_grafana_pass)"
            ;;
        testing)
            echo "  📁 Database: SQLite at ./data/research.db"
            echo "  🔄 Redis: localhost:6380"
            echo "  🎭 Mock Server: http://localhost:1080"
            ;;
        development|full)
            echo "  📁 Database: SQLite at ./data/research.db"
            echo "  🔄 Redis: localhost:6380 (password: tauri_redis_pass)"
            echo "  📊 Prometheus: http://localhost:9091"
            echo "  📈 Grafana: http://localhost:3002 (admin/tauri_grafana_pass)"
            echo "  🎭 Mock Server: http://localhost:1080"
            ;;
    esac
    
    echo ""
    print_info "Use '$0 status' to check service status"
    print_info "Use '$0 logs' to view service logs"
}

# Main script logic
main() {
    # Check prerequisites
    check_docker
    check_docker_compose
    
    # Parse command
    local command=${1:-help}
    
    case $command in
        start)
            start_services $2
            ;;
        stop)
            stop_services
            ;;
        restart)
            restart_services $2
            ;;
        status)
            show_status
            ;;
        logs)
            show_logs $2
            ;;
        clean)
            clean_services
            ;;
        test)
            run_tests
            ;;
        build)
            build_images
            ;;
        health)
            check_health
            ;;
        backup)
            backup_data
            ;;
        shell)
            open_shell $2
            ;;
        update)
            update_images
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            print_error "Unknown command: $command"
            echo ""
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
