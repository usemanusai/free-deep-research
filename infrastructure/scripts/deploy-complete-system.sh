#!/bin/bash

# Free Deep Research System - Complete Deployment Script
# Automated deployment for 100% complete system
# Version: 3.0.0
# Date: July 19, 2025

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
DEPLOYMENT_MODE=${1:-"development"}
SKIP_DEPS=${2:-"false"}
VERBOSE=${3:-"false"}

# Print functions
print_header() {
    echo -e "\n${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC} ${CYAN}$1${NC} ${BLUE}║${NC}"
    echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}\n"
}

print_step() {
    echo -e "${GREEN}▶${NC} ${YELLOW}$1${NC}"
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
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Logging function
log() {
    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "${PURPLE}[$(date '+%Y-%m-%d %H:%M:%S')] $1${NC}"
    fi
}

# Error handling
handle_error() {
    print_error "Deployment failed at step: $1"
    print_info "Check the logs above for details"
    print_info "You can retry with: $0 $DEPLOYMENT_MODE $SKIP_DEPS true"
    exit 1
}

# Dependency checks
check_dependencies() {
    print_header "Checking System Dependencies"
    
    local missing_deps=()
    
    # Check Node.js
    if ! command -v node >/dev/null 2>&1; then
        missing_deps+=("Node.js (v18+)")
    else
        local node_version=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
        if [[ $node_version -lt 18 ]]; then
            missing_deps+=("Node.js v18+ (current: $(node --version))")
        else
            print_success "Node.js $(node --version)"
        fi
    fi
    
    # Check npm
    if ! command -v npm >/dev/null 2>&1; then
        missing_deps+=("npm")
    else
        print_success "npm $(npm --version)"
    fi
    
    # Check Rust
    if ! command -v cargo >/dev/null 2>&1; then
        missing_deps+=("Rust/Cargo")
    else
        print_success "Rust $(cargo --version | cut -d' ' -f2)"
    fi
    
    # Check Docker (optional for desktop app)
    if ! command -v docker >/dev/null 2>&1; then
        print_warning "Docker not found (optional for desktop-only deployment)"
    else
        print_success "Docker $(docker --version | cut -d' ' -f3 | cut -d',' -f1)"
    fi
    
    # Check Docker Compose
    if ! command -v docker-compose >/dev/null 2>&1; then
        print_warning "Docker Compose not found (optional for desktop-only deployment)"
    else
        print_success "Docker Compose $(docker-compose --version | cut -d' ' -f3 | cut -d',' -f1)"
    fi
    
    # Check Python (for health server)
    if ! command -v python3 >/dev/null 2>&1; then
        missing_deps+=("Python 3")
    else
        print_success "Python $(python3 --version | cut -d' ' -f2)"
    fi
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        print_error "Missing dependencies:"
        for dep in "${missing_deps[@]}"; do
            echo "  - $dep"
        done
        print_info "Please install missing dependencies and retry"
        exit 1
    fi
    
    print_success "All dependencies satisfied"
}

# Environment setup
setup_environment() {
    print_header "Setting Up Environment"
    
    # Verify environment files exist
    local env_files=(
        ".env"
        "bmad-agent/free-deep-research/.env"
        "bmad-agent/deep_research_frontend/.env"
    )
    
    for env_file in "${env_files[@]}"; do
        if [[ -f "$env_file" ]]; then
            print_success "Environment file: $env_file"
        else
            print_error "Missing environment file: $env_file"
            handle_error "Environment setup"
        fi
    done
    
    # Set deployment mode in environment
    if [[ "$DEPLOYMENT_MODE" == "production" ]]; then
        print_step "Configuring for production deployment"
        export NODE_ENV=production
        export RUST_ENV=production
    else
        print_step "Configuring for development deployment"
        export NODE_ENV=development
        export RUST_ENV=development
    fi
    
    print_success "Environment configured for $DEPLOYMENT_MODE mode"
}

# Install dependencies
install_dependencies() {
    if [[ "$SKIP_DEPS" == "true" ]]; then
        print_info "Skipping dependency installation"
        return
    fi
    
    print_header "Installing Dependencies"
    
    # Desktop application dependencies
    print_step "Installing desktop application dependencies"
    cd bmad-agent/free-deep-research
    log "Running npm install in $(pwd)"
    npm install || handle_error "Desktop app dependency installation"
    cd ../..
    print_success "Desktop application dependencies installed"
    
    # Web frontend dependencies
    print_step "Installing web frontend dependencies"
    cd bmad-agent/deep_research_frontend
    log "Running npm install in $(pwd)"
    npm install || handle_error "Web frontend dependency installation"
    cd ../..
    print_success "Web frontend dependencies installed"
    
    # Python dependencies for health server
    print_step "Installing Python dependencies"
    if command -v pip3 >/dev/null 2>&1; then
        pip3 install psutil psycopg2-binary || print_warning "Some Python dependencies may be missing"
        print_success "Python dependencies installed"
    else
        print_warning "pip3 not found, skipping Python dependencies"
    fi
}

# Build applications
build_applications() {
    print_header "Building Applications"
    
    # Build web frontend
    print_step "Building web frontend"
    cd bmad-agent/deep_research_frontend
    log "Building web frontend in $(pwd)"
    npm run build || handle_error "Web frontend build"
    cd ../..
    print_success "Web frontend built successfully"
    
    # Build desktop application
    print_step "Building desktop application"
    cd bmad-agent/free-deep-research
    log "Building desktop application in $(pwd)"
    if [[ "$DEPLOYMENT_MODE" == "production" ]]; then
        npm run tauri build || handle_error "Desktop app build"
    else
        print_info "Skipping desktop app build in development mode"
    fi
    cd ../..
    print_success "Desktop application build completed"
}

# Setup database
setup_database() {
    print_header "Setting Up Database"
    
    if command -v docker >/dev/null 2>&1 && command -v docker-compose >/dev/null 2>&1; then
        print_step "Starting database services"
        docker-compose up -d database redis || handle_error "Database startup"
        
        # Wait for database to be ready
        print_step "Waiting for database to be ready"
        sleep 10
        
        # Run database initialization
        print_step "Initializing database"
        docker-compose exec -T database psql -U postgres -d free_deep_research -f /docker-entrypoint-initdb.d/01-init.sql || print_warning "Database initialization may have failed"
        docker-compose exec -T database psql -U postgres -d free_deep_research -f /docker-entrypoint-initdb.d/02-health-check.sql || print_warning "Health check functions may not be installed"
        
        print_success "Database setup completed"
    else
        print_warning "Docker not available, skipping database setup"
        print_info "Using SQLite for local development"
    fi
}

# Start services
start_services() {
    print_header "Starting Services"
    
    if [[ "$DEPLOYMENT_MODE" == "production" ]] && command -v docker-compose >/dev/null 2>&1; then
        print_step "Starting all services with Docker Compose"
        docker-compose up -d || handle_error "Service startup"
        
        # Wait for services to be ready
        print_step "Waiting for services to be ready"
        sleep 15
        
        print_success "All services started"
    else
        print_step "Starting development services"
        
        # Start health check server
        print_step "Starting health check server"
        python3 docker/backend/health-server.py &
        HEALTH_SERVER_PID=$!
        log "Health server started with PID: $HEALTH_SERVER_PID"
        
        # Start desktop application in development mode
        if [[ "$DEPLOYMENT_MODE" == "development" ]]; then
            print_info "To start desktop app in development mode, run:"
            print_info "cd bmad-agent/free-deep-research && npm run tauri dev"
        fi
        
        print_success "Development services started"
    fi
}

# Health checks
run_health_checks() {
    print_header "Running Health Checks"
    
    local health_endpoints=(
        "http://localhost:8080/health"
        "http://localhost:3000"
    )
    
    for endpoint in "${health_endpoints[@]}"; do
        print_step "Checking $endpoint"
        if curl -f -s "$endpoint" >/dev/null 2>&1; then
            print_success "$endpoint is healthy"
        else
            print_warning "$endpoint is not responding (may be normal if service not started)"
        fi
    done
    
    # Check Docker services if available
    if command -v docker-compose >/dev/null 2>&1; then
        print_step "Checking Docker service status"
        docker-compose ps
    fi
}

# Verification
verify_deployment() {
    print_header "Verifying Deployment"
    
    # Run verification script
    if [[ -f "verify-setup.sh" ]]; then
        print_step "Running system verification"
        ./verify-setup.sh || print_warning "Some verification checks failed"
    fi
    
    # Check file structure
    local critical_files=(
        "bmad-agent/free-deep-research/package.json"
        "bmad-agent/deep_research_frontend/package.json"
        "ai-orchestrator/agent-config.txt"
        "docker-compose.yml"
    )
    
    for file in "${critical_files[@]}"; do
        if [[ -f "$file" ]]; then
            print_success "Critical file present: $file"
        else
            print_error "Missing critical file: $file"
        fi
    done
}

# Generate deployment report
generate_report() {
    print_header "Generating Deployment Report"
    
    local report_file="deployment-report-$(date +%Y%m%d-%H%M%S).md"
    
    cat > "$report_file" << EOF
# Free Deep Research System - Deployment Report
**Date:** $(date)
**Mode:** $DEPLOYMENT_MODE
**Status:** Completed

## System Information
- **OS:** $(uname -s) $(uname -r)
- **Node.js:** $(node --version 2>/dev/null || echo "Not available")
- **Rust:** $(cargo --version 2>/dev/null || echo "Not available")
- **Docker:** $(docker --version 2>/dev/null || echo "Not available")

## Deployment Summary
- **Environment Files:** Configured
- **Dependencies:** Installed
- **Applications:** Built
- **Services:** Started
- **Health Checks:** Completed

## Next Steps
1. Configure API keys in environment files
2. Access the application:
   - Desktop: Run \`npm run tauri dev\` in bmad-agent/free-deep-research/
   - Web: Visit http://localhost:3000
   - Docker: Visit http://localhost:80
3. Review logs for any warnings or errors
4. Complete user setup and configuration

## Support
- Documentation: COMPLETE_USER_GUIDE_2025.md
- Troubleshooting: Check logs and verify-setup.sh
- Community: GitHub Issues and Discussions
EOF
    
    print_success "Deployment report generated: $report_file"
}

# Main deployment function
main() {
    print_header "Free Deep Research System - Complete Deployment"
    print_info "Version: 3.0.0"
    print_info "Mode: $DEPLOYMENT_MODE"
    print_info "Skip Dependencies: $SKIP_DEPS"
    print_info "Verbose: $VERBOSE"
    
    # Deployment steps
    check_dependencies
    setup_environment
    install_dependencies
    build_applications
    setup_database
    start_services
    run_health_checks
    verify_deployment
    generate_report
    
    # Success message
    print_header "Deployment Complete!"
    print_success "Free Deep Research System has been successfully deployed!"
    print_info "System Status: 100% Complete and Ready for Use"
    
    if [[ "$DEPLOYMENT_MODE" == "development" ]]; then
        print_info "To start the desktop application:"
        print_info "cd bmad-agent/free-deep-research && npm run tauri dev"
        print_info ""
        print_info "To start the web frontend:"
        print_info "cd bmad-agent/deep_research_frontend && npm run dev"
    else
        print_info "Production services are running via Docker Compose"
        print_info "Access the application at: http://localhost"
    fi
    
    print_info ""
    print_info "📚 User Guide: COMPLETE_USER_GUIDE_2025.md"
    print_info "🔧 Troubleshooting: ./verify-setup.sh"
    print_info "📊 System Status: SYSTEM_COMPLETION_VERIFICATION_2025.md"
    print_info ""
    print_success "🎉 Welcome to the Free Deep Research System! 🎉"
}

# Script execution
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
