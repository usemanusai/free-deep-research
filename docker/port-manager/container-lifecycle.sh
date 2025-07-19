#!/bin/bash

# Free Deep Research System - Container Lifecycle Manager
# Handles existing container detection, cleanup, and management

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
PROJECT_NAME="${COMPOSE_PROJECT_NAME:-free-deep-research}"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[LIFECYCLE]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[LIFECYCLE]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[LIFECYCLE]${NC} $1"
}

print_error() {
    echo -e "${RED}[LIFECYCLE]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}$1${NC}"
}

# Function to check if Docker is available
check_docker() {
    if ! command -v docker >/dev/null 2>&1; then
        print_error "Docker is not installed or not in PATH"
        return 1
    fi
    
    if ! docker info >/dev/null 2>&1; then
        print_error "Docker daemon is not running"
        return 1
    fi
    
    return 0
}

# Function to get container version from labels
get_container_version() {
    local container_name=$1
    docker inspect --format='{{index .Config.Labels "version"}}' "$container_name" 2>/dev/null || echo "unknown"
}

# Function to get container creation time
get_container_created() {
    local container_name=$1
    docker inspect --format='{{.Created}}' "$container_name" 2>/dev/null || echo "unknown"
}

# Function to get container status
get_container_status() {
    local container_name=$1
    docker inspect --format='{{.State.Status}}' "$container_name" 2>/dev/null || echo "unknown"
}

# Function to get container ports
get_container_ports() {
    local container_name=$1
    docker port "$container_name" 2>/dev/null | sed 's/0.0.0.0:/localhost:/' || echo "No ports exposed"
}

# Function to scan for existing containers
scan_existing_containers() {
    print_header "Scanning for existing Free Deep Research containers..."
    
    if ! check_docker; then
        return 1
    fi
    
    # Get all containers (running and stopped) with our project name
    local containers
    mapfile -t containers < <(docker ps -a --filter "name=$PROJECT_NAME" --format "{{.Names}}" 2>/dev/null)
    
    if [[ ${#containers[@]} -eq 0 ]]; then
        print_status "No existing containers found"
        return 0
    fi
    
    print_warning "Found ${#containers[@]} existing container(s):"
    echo ""
    
    # Display container information
    printf "%-30s %-12s %-10s %-20s %s\n" "CONTAINER NAME" "STATUS" "VERSION" "CREATED" "PORTS"
    printf "%-30s %-12s %-10s %-20s %s\n" "$(printf '%*s' 30 '' | tr ' ' '-')" "$(printf '%*s' 12 '' | tr ' ' '-')" "$(printf '%*s' 10 '' | tr ' ' '-')" "$(printf '%*s' 20 '' | tr ' ' '-')" "$(printf '%*s' 20 '' | tr ' ' '-')"
    
    for container in "${containers[@]}"; do
        local status=$(get_container_status "$container")
        local version=$(get_container_version "$container")
        local created=$(get_container_created "$container" | cut -d'T' -f1)
        local ports=$(get_container_ports "$container" | head -1)
        
        printf "%-30s %-12s %-10s %-20s %s\n" "$container" "$status" "$version" "$created" "$ports"
    done
    
    echo ""
    return 0
}

# Function to prompt user for action
prompt_user_action() {
    local containers=("$@")
    
    print_header "Container Management Options:"
    echo "1. Reuse existing containers (recommended if same version)"
    echo "2. Stop and replace existing containers"
    echo "3. Run alongside with different ports (development only)"
    echo "4. Clean up all containers and start fresh"
    echo "5. Cancel and exit"
    echo ""
    
    while true; do
        read -p "Please select an option (1-5): " choice
        case $choice in
            1)
                print_status "Selected: Reuse existing containers"
                return 1  # Reuse
                ;;
            2)
                print_status "Selected: Stop and replace existing containers"
                return 2  # Replace
                ;;
            3)
                print_status "Selected: Run alongside with different ports"
                return 3  # Alongside
                ;;
            4)
                print_status "Selected: Clean up all containers and start fresh"
                return 4  # Clean
                ;;
            5)
                print_status "Selected: Cancel and exit"
                return 5  # Cancel
                ;;
            *)
                print_warning "Invalid option. Please select 1-5."
                ;;
        esac
    done
}

# Function to stop and remove containers
stop_and_remove_containers() {
    local containers=("$@")
    
    print_status "Stopping and removing existing containers..."
    
    for container in "${containers[@]}"; do
        local status=$(get_container_status "$container")
        
        if [[ "$status" == "running" ]]; then
            print_status "Stopping container: $container"
            docker stop "$container" >/dev/null 2>&1 || print_warning "Failed to stop $container"
        fi
        
        print_status "Removing container: $container"
        docker rm "$container" >/dev/null 2>&1 || print_warning "Failed to remove $container"
    done
    
    print_success "Container cleanup completed"
}

# Function to clean up all project containers
cleanup_all_containers() {
    print_status "Cleaning up all Free Deep Research containers..."
    
    # Stop all running containers
    local running_containers
    mapfile -t running_containers < <(docker ps --filter "name=$PROJECT_NAME" --format "{{.Names}}" 2>/dev/null)
    
    if [[ ${#running_containers[@]} -gt 0 ]]; then
        print_status "Stopping running containers..."
        for container in "${running_containers[@]}"; do
            docker stop "$container" >/dev/null 2>&1 || print_warning "Failed to stop $container"
        done
    fi
    
    # Remove all containers
    local all_containers
    mapfile -t all_containers < <(docker ps -a --filter "name=$PROJECT_NAME" --format "{{.Names}}" 2>/dev/null)
    
    if [[ ${#all_containers[@]} -gt 0 ]]; then
        print_status "Removing containers..."
        for container in "${all_containers[@]}"; do
            docker rm "$container" >/dev/null 2>&1 || print_warning "Failed to remove $container"
        done
    fi
    
    # Clean up volumes
    local volumes
    mapfile -t volumes < <(docker volume ls --filter "name=$PROJECT_NAME" --format "{{.Name}}" 2>/dev/null)
    
    if [[ ${#volumes[@]} -gt 0 ]]; then
        print_status "Removing volumes..."
        for volume in "${volumes[@]}"; do
            docker volume rm "$volume" >/dev/null 2>&1 || print_warning "Failed to remove volume $volume"
        done
    fi
    
    # Clean up networks
    local networks
    mapfile -t networks < <(docker network ls --filter "name=$PROJECT_NAME" --format "{{.Name}}" 2>/dev/null)
    
    for network in "${networks[@]}"; do
        if [[ "$network" != "bridge" && "$network" != "host" && "$network" != "none" ]]; then
            print_status "Removing network: $network"
            docker network rm "$network" >/dev/null 2>&1 || print_warning "Failed to remove network $network"
        fi
    done
    
    print_success "Complete cleanup finished"
}

# Function to generate unique container names
generate_unique_names() {
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local uuid=$(uuidgen 2>/dev/null | cut -d'-' -f1 || echo "$RANDOM")
    
    echo "${PROJECT_NAME}_${timestamp}_${uuid}"
}

# Function to check container health
check_container_health() {
    local container_name=$1
    
    # Check if container exists
    if ! docker inspect "$container_name" >/dev/null 2>&1; then
        echo "not_found"
        return 1
    fi
    
    # Check container status
    local status=$(get_container_status "$container_name")
    
    case "$status" in
        "running")
            # Check if container is healthy (if health check is defined)
            local health=$(docker inspect --format='{{.State.Health.Status}}' "$container_name" 2>/dev/null || echo "none")
            if [[ "$health" == "healthy" || "$health" == "none" ]]; then
                echo "healthy"
                return 0
            else
                echo "unhealthy"
                return 1
            fi
            ;;
        "exited")
            echo "stopped"
            return 1
            ;;
        *)
            echo "$status"
            return 1
            ;;
    esac
}

# Function to wait for container health
wait_for_container_health() {
    local container_name=$1
    local max_wait=${2:-60}
    local wait_time=0
    
    print_status "Waiting for container $container_name to become healthy..."
    
    while [[ $wait_time -lt $max_wait ]]; do
        local health=$(check_container_health "$container_name")
        
        case "$health" in
            "healthy")
                print_success "Container $container_name is healthy"
                return 0
                ;;
            "not_found")
                print_error "Container $container_name not found"
                return 1
                ;;
            "stopped")
                print_error "Container $container_name has stopped"
                return 1
                ;;
            *)
                print_status "Container $container_name status: $health (waiting...)"
                sleep 5
                wait_time=$((wait_time + 5))
                ;;
        esac
    done
    
    print_warning "Container $container_name did not become healthy within $max_wait seconds"
    return 1
}

# Function to manage existing containers
manage_existing_containers() {
    local environment=${1:-development}
    
    # Scan for existing containers
    local containers
    mapfile -t containers < <(docker ps -a --filter "name=$PROJECT_NAME" --format "{{.Names}}" 2>/dev/null)
    
    if [[ ${#containers[@]} -eq 0 ]]; then
        print_status "No existing containers found. Proceeding with fresh deployment."
        return 0
    fi
    
    # Display existing containers
    scan_existing_containers
    
    # Prompt user for action
    prompt_user_action "${containers[@]}"
    local action=$?
    
    case $action in
        1)  # Reuse existing containers
            print_status "Reusing existing containers..."
            
            # Check health of existing containers
            local unhealthy_containers=()
            for container in "${containers[@]}"; do
                local health=$(check_container_health "$container")
                if [[ "$health" != "healthy" ]]; then
                    unhealthy_containers+=("$container")
                fi
            done
            
            if [[ ${#unhealthy_containers[@]} -gt 0 ]]; then
                print_warning "Some containers are not healthy: ${unhealthy_containers[*]}"
                print_status "Starting unhealthy containers..."
                
                for container in "${unhealthy_containers[@]}"; do
                    docker start "$container" >/dev/null 2>&1 || print_warning "Failed to start $container"
                done
            fi
            
            return 0
            ;;
        2)  # Stop and replace existing containers
            stop_and_remove_containers "${containers[@]}"
            return 0
            ;;
        3)  # Run alongside with different ports
            if [[ "$environment" != "development" ]]; then
                print_error "Running alongside is only supported in development environment"
                return 1
            fi
            
            print_status "Will run alongside existing containers with different ports"
            
            # Generate unique project name
            local unique_name=$(generate_unique_names)
            export COMPOSE_PROJECT_NAME="$unique_name"
            print_status "Using unique project name: $unique_name"
            
            return 0
            ;;
        4)  # Clean up all containers and start fresh
            cleanup_all_containers
            return 0
            ;;
        5)  # Cancel and exit
            print_status "Operation cancelled by user"
            return 1
            ;;
        *)
            print_error "Invalid action returned"
            return 1
            ;;
    esac
}

# Function to show container status
show_container_status() {
    print_header "Container Status Report"
    
    if ! check_docker; then
        return 1
    fi
    
    # Get all project containers
    local containers
    mapfile -t containers < <(docker ps -a --filter "name=$PROJECT_NAME" --format "{{.Names}}" 2>/dev/null)
    
    if [[ ${#containers[@]} -eq 0 ]]; then
        print_status "No containers found for project: $PROJECT_NAME"
        return 0
    fi
    
    echo ""
    printf "%-30s %-12s %-10s %-15s %s\n" "CONTAINER NAME" "STATUS" "HEALTH" "UPTIME" "PORTS"
    printf "%-30s %-12s %-10s %-15s %s\n" "$(printf '%*s' 30 '' | tr ' ' '-')" "$(printf '%*s' 12 '' | tr ' ' '-')" "$(printf '%*s' 10 '' | tr ' ' '-')" "$(printf '%*s' 15 '' | tr ' ' '-')" "$(printf '%*s' 20 '' | tr ' ' '-')"
    
    for container in "${containers[@]}"; do
        local status=$(get_container_status "$container")
        local health=$(check_container_health "$container")
        local uptime="N/A"
        local ports=$(get_container_ports "$container" | head -1)
        
        if [[ "$status" == "running" ]]; then
            uptime=$(docker inspect --format='{{.State.StartedAt}}' "$container" 2>/dev/null | cut -d'T' -f1 || echo "unknown")
        fi
        
        printf "%-30s %-12s %-10s %-15s %s\n" "$container" "$status" "$health" "$uptime" "$ports"
    done
    
    echo ""
}

# Function to show usage
show_usage() {
    cat << EOF
Free Deep Research System - Container Lifecycle Manager

Usage: $0 <command> [options]

Commands:
    scan               Scan for existing containers
    manage <env>       Manage existing containers for environment
    cleanup            Clean up all project containers and resources
    status             Show current container status
    health <name>      Check health of specific container
    help               Show this help message

Examples:
    $0 scan
    $0 manage development
    $0 cleanup
    $0 status
    $0 health fdr-backend

EOF
}

# Main execution
main() {
    case "${1:-help}" in
        scan)
            scan_existing_containers
            ;;
        manage)
            manage_existing_containers "${2:-development}"
            ;;
        cleanup)
            cleanup_all_containers
            ;;
        status)
            show_container_status
            ;;
        health)
            if [[ -n "$2" ]]; then
                health=$(check_container_health "$2")
                echo "Container $2 health: $health"
            else
                print_error "Container name required for health check"
                exit 1
            fi
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
