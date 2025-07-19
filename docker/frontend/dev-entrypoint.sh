#!/bin/bash

# Free Deep Research System - Frontend Development Entrypoint
# This script sets up the development environment for the React frontend

set -e

echo "🎨 Starting Free Deep Research Frontend Development Environment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[DEV-FRONTEND]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[DEV-FRONTEND]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[DEV-FRONTEND]${NC} $1"
}

print_error() {
    echo -e "${RED}[DEV-FRONTEND]${NC} $1"
}

# Function to wait for backend
wait_for_backend() {
    print_status "Waiting for backend to be ready..."
    
    local backend_url="${REACT_APP_API_URL:-http://localhost:8080}"
    local max_attempts=60
    local attempt=1
    
    while [[ $attempt -le $max_attempts ]]; do
        if curl -f "${backend_url}/health" >/dev/null 2>&1; then
            print_success "Backend is ready at ${backend_url}"
            break
        fi
        
        if [[ $attempt -eq $max_attempts ]]; then
            print_warning "Backend not ready after $max_attempts attempts, continuing anyway..."
            break
        fi
        
        print_status "Waiting for backend... (attempt $attempt/$max_attempts)"
        sleep 2
        ((attempt++))
    done
}

# Function to setup development environment
setup_dev_environment() {
    print_status "Setting up development environment..."
    
    # Set development environment variables
    export NODE_ENV=development
    export CHOKIDAR_USEPOLLING=true
    export WATCHPACK_POLLING=true
    export FAST_REFRESH=true
    export GENERATE_SOURCEMAP=true
    
    # Create necessary directories
    mkdir -p /app/public /app/src /app/build
    
    print_success "Development environment setup complete"
}

# Function to install dependencies
install_dependencies() {
    print_status "Installing/updating Node.js dependencies..."
    
    # Check if package.json exists
    if [[ ! -f package.json ]]; then
        print_error "package.json not found!"
        exit 1
    fi
    
    # Check if node_modules needs update
    if [[ ! -d node_modules ]] || [[ package.json -nt node_modules ]]; then
        print_status "Installing dependencies..."
        npm ci --prefer-offline --no-audit
    else
        print_status "Dependencies are up to date"
    fi
    
    # Install global development tools if needed
    if ! command -v eslint >/dev/null 2>&1; then
        print_status "Installing global development tools..."
        npm install -g eslint prettier typescript
    fi
    
    print_success "Dependencies ready"
}

# Function to run linting and type checking
run_checks() {
    print_status "Running development checks..."
    
    # Run ESLint if available
    if [[ -f .eslintrc.js ]] || [[ -f .eslintrc.json ]] || [[ -f package.json ]]; then
        if npm run lint --if-present >/dev/null 2>&1; then
            print_success "ESLint checks passed"
        else
            print_warning "ESLint checks failed, but continuing..."
        fi
    fi
    
    # Run TypeScript type checking if available
    if [[ -f tsconfig.json ]]; then
        if npm run type-check --if-present >/dev/null 2>&1; then
            print_success "TypeScript checks passed"
        else
            print_warning "TypeScript checks failed, but continuing..."
        fi
    fi
}

# Function to start the development server
start_dev_server() {
    print_status "Starting development server..."
    
    # Display environment information
    print_status "Environment variables:"
    print_status "  NODE_ENV: ${NODE_ENV}"
    print_status "  REACT_APP_API_URL: ${REACT_APP_API_URL}"
    print_status "  REACT_APP_WS_URL: ${REACT_APP_WS_URL}"
    print_status "  REACT_APP_VERSION: ${REACT_APP_VERSION}"
    
    # Start the development server
    print_success "Starting React development server with hot reload..."
    
    # Use npm start or react-scripts start
    if npm run start:dev --if-present 2>/dev/null; then
        exec npm run start:dev
    elif npm run dev --if-present 2>/dev/null; then
        exec npm run dev
    else
        exec npm start
    fi
}

# Function to handle cleanup on exit
cleanup() {
    print_status "Cleaning up development environment..."
    # Kill any background processes
    jobs -p | xargs -r kill
    exit 0
}

# Set up signal handlers
trap cleanup SIGTERM SIGINT

# Function to create development configuration files
create_dev_configs() {
    print_status "Creating development configuration files..."
    
    # Create .env.local for development overrides
    if [[ ! -f .env.local ]]; then
        cat > .env.local << EOF
# Development environment overrides
GENERATE_SOURCEMAP=true
FAST_REFRESH=true
CHOKIDAR_USEPOLLING=true
WATCHPACK_POLLING=true

# API Configuration
REACT_APP_API_URL=${REACT_APP_API_URL:-http://localhost:8080}
REACT_APP_WS_URL=${REACT_APP_WS_URL:-ws://localhost:8080}
REACT_APP_VERSION=${REACT_APP_VERSION:-3.0.0-dev}

# Development flags
REACT_APP_DEBUG=true
REACT_APP_MOCK_API=false
EOF
        print_success "Created .env.local for development"
    fi
    
    # Create or update .eslintrc.js if it doesn't exist
    if [[ ! -f .eslintrc.js ]] && [[ ! -f .eslintrc.json ]]; then
        cat > .eslintrc.js << 'EOF'
module.exports = {
  extends: [
    'react-app',
    'react-app/jest'
  ],
  rules: {
    // Add custom rules for development
    'no-console': 'warn',
    'no-debugger': 'warn'
  },
  env: {
    browser: true,
    es6: true,
    node: true
  }
};
EOF
        print_success "Created .eslintrc.js for development"
    fi
}

# Function to setup hot reload
setup_hot_reload() {
    print_status "Configuring hot reload..."
    
    # Ensure polling is enabled for file watching in containers
    export CHOKIDAR_USEPOLLING=true
    export WATCHPACK_POLLING=true
    
    # Set up fast refresh
    export FAST_REFRESH=true
    
    print_success "Hot reload configured"
}

# Main execution
main() {
    print_status "Free Deep Research Frontend Development Environment v3.0.0"
    print_status "Node.js version: $(node --version)"
    print_status "npm version: $(npm --version)"
    
    # Setup development environment
    setup_dev_environment
    
    # Create development configuration files
    create_dev_configs
    
    # Setup hot reload
    setup_hot_reload
    
    # Install/update dependencies
    install_dependencies
    
    # Wait for backend (optional)
    if [[ "${WAIT_FOR_BACKEND:-true}" == "true" ]]; then
        wait_for_backend
    fi
    
    # Run development checks
    run_checks
    
    # Start development server
    start_dev_server
}

# Execute main function
main "$@"
