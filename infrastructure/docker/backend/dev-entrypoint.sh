#!/bin/bash

# Free Deep Research System - Backend Development Entrypoint
# This script sets up the development environment for the Rust backend

set -e

echo "🚀 Starting Free Deep Research Backend Development Environment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[DEV-BACKEND]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[DEV-BACKEND]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[DEV-BACKEND]${NC} $1"
}

print_error() {
    echo -e "${RED}[DEV-BACKEND]${NC} $1"
}

# Function to wait for dependencies
wait_for_dependencies() {
    print_status "Waiting for dependencies to be ready..."
    
    # Wait for database (SQLite file or PostgreSQL)
    if [[ "$DATABASE_URL" == sqlite* ]]; then
        # For SQLite, just ensure the directory exists
        mkdir -p "$(dirname "${DATABASE_URL#sqlite://}")"
        print_success "SQLite database directory ready"
    else
        # For PostgreSQL, wait for connection
        local max_attempts=30
        local attempt=1
        
        while [[ $attempt -le $max_attempts ]]; do
            if pg_isready -h database -p 5432 -U "${DB_USER:-fdr_user}" >/dev/null 2>&1; then
                print_success "PostgreSQL database is ready"
                break
            fi
            
            if [[ $attempt -eq $max_attempts ]]; then
                print_error "PostgreSQL database failed to become ready after $max_attempts attempts"
                exit 1
            fi
            
            print_status "Waiting for PostgreSQL... (attempt $attempt/$max_attempts)"
            sleep 2
            ((attempt++))
        done
    fi
    
    # Wait for Redis
    local max_attempts=30
    local attempt=1
    
    while [[ $attempt -le $max_attempts ]]; do
        if redis-cli -h redis -p 6379 ping >/dev/null 2>&1; then
            print_success "Redis is ready"
            break
        fi
        
        if [[ $attempt -eq $max_attempts ]]; then
            print_error "Redis failed to become ready after $max_attempts attempts"
            exit 1
        fi
        
        print_status "Waiting for Redis... (attempt $attempt/$max_attempts)"
        sleep 2
        ((attempt++))
    done
}

# Function to setup development environment
setup_dev_environment() {
    print_status "Setting up development environment..."
    
    # Create necessary directories
    mkdir -p /app/logs /app/uploads /app/cache /app/data
    
    # Set proper permissions
    chown -R developer:developer /app
    
    # Install additional development tools if needed
    if ! command -v cargo-watch >/dev/null 2>&1; then
        print_status "Installing cargo-watch for hot reload..."
        cargo install cargo-watch
    fi
    
    if ! command -v cargo-edit >/dev/null 2>&1; then
        print_status "Installing cargo-edit for dependency management..."
        cargo install cargo-edit
    fi
    
    print_success "Development environment setup complete"
}

# Function to run database migrations
run_migrations() {
    print_status "Running database migrations..."
    
    cd /app/src-tauri
    
    # Check if migration is needed
    if cargo run --bin migrate -- --check 2>/dev/null; then
        print_success "Database schema is up to date"
    else
        print_status "Running database migrations..."
        if cargo run --bin migrate; then
            print_success "Database migrations completed successfully"
        else
            print_warning "Database migrations failed, but continuing..."
        fi
    fi
}

# Function to install dependencies
install_dependencies() {
    print_status "Installing/updating Rust dependencies..."
    
    cd /app/src-tauri
    
    # Update dependencies if Cargo.lock doesn't exist or is outdated
    if [[ ! -f Cargo.lock ]] || [[ Cargo.toml -nt Cargo.lock ]]; then
        print_status "Updating dependencies..."
        cargo update
    fi
    
    # Build dependencies
    print_status "Building dependencies..."
    cargo build
    
    print_success "Dependencies ready"
}

# Function to start the development server
start_dev_server() {
    print_status "Starting development server with hot reload..."
    
    cd /app/src-tauri
    
    # Set development-specific environment variables
    export RUST_LOG="${RUST_LOG:-debug}"
    export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"
    
    # Start the server with hot reload
    if command -v cargo-watch >/dev/null 2>&1; then
        print_success "Starting with cargo-watch for hot reload..."
        exec cargo watch -x "run --bin free-deep-research"
    else
        print_warning "cargo-watch not available, starting without hot reload..."
        exec cargo run --bin free-deep-research
    fi
}

# Function to handle cleanup on exit
cleanup() {
    print_status "Cleaning up development environment..."
    # Add any cleanup tasks here
    exit 0
}

# Set up signal handlers
trap cleanup SIGTERM SIGINT

# Main execution
main() {
    print_status "Free Deep Research Backend Development Environment v3.0.0"
    print_status "Environment: ${RUST_ENV:-development}"
    print_status "Database: ${DATABASE_URL}"
    print_status "Redis: ${REDIS_URL}"
    
    # Wait for dependencies
    wait_for_dependencies
    
    # Setup development environment
    setup_dev_environment
    
    # Install/update dependencies
    install_dependencies
    
    # Run migrations
    run_migrations
    
    # Start development server
    start_dev_server
}

# Execute main function
main "$@"
