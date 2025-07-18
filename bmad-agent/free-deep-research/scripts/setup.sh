#!/bin/bash

# Free Deep Research System - Development Setup Script
# This script sets up the development environment for the Free Deep Research System

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check system requirements
check_requirements() {
    log_info "Checking system requirements..."
    
    # Check Node.js
    if command_exists node; then
        NODE_VERSION=$(node --version | cut -d'v' -f2)
        REQUIRED_NODE="18.0.0"
        if [ "$(printf '%s\n' "$REQUIRED_NODE" "$NODE_VERSION" | sort -V | head -n1)" = "$REQUIRED_NODE" ]; then
            log_success "Node.js $NODE_VERSION is installed"
        else
            log_error "Node.js $REQUIRED_NODE or higher is required. Found: $NODE_VERSION"
            exit 1
        fi
    else
        log_error "Node.js is not installed. Please install Node.js 18.0.0 or higher"
        exit 1
    fi
    
    # Check npm
    if command_exists npm; then
        NPM_VERSION=$(npm --version)
        log_success "npm $NPM_VERSION is installed"
    else
        log_error "npm is not installed"
        exit 1
    fi
    
    # Check Rust
    if command_exists rustc; then
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
        log_success "Rust $RUST_VERSION is installed"
    else
        log_error "Rust is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    # Check Cargo
    if command_exists cargo; then
        CARGO_VERSION=$(cargo --version | cut -d' ' -f2)
        log_success "Cargo $CARGO_VERSION is installed"
    else
        log_error "Cargo is not installed"
        exit 1
    fi
    
    # Check Git
    if command_exists git; then
        GIT_VERSION=$(git --version | cut -d' ' -f3)
        log_success "Git $GIT_VERSION is installed"
    else
        log_error "Git is not installed"
        exit 1
    fi
}

# Install system dependencies
install_system_deps() {
    log_info "Installing system dependencies..."
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        if command_exists apt-get; then
            sudo apt-get update
            sudo apt-get install -y webkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
        elif command_exists yum; then
            sudo yum groupinstall -y "Development Tools"
            sudo yum install -y webkit2gtk3-devel openssl-devel curl wget gtk3-devel libappindicator-gtk3-devel librsvg2-devel
        else
            log_warning "Unsupported Linux distribution. Please install dependencies manually."
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        if command_exists brew; then
            brew install curl wget
        else
            log_warning "Homebrew not found. Please install dependencies manually."
        fi
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        # Windows
        log_info "Windows detected. Please ensure you have Visual Studio Build Tools installed."
    fi
    
    log_success "System dependencies installed"
}

# Install Rust tools
install_rust_tools() {
    log_info "Installing Rust tools..."
    
    # Install Tauri CLI
    if ! command_exists tauri; then
        cargo install tauri-cli
        log_success "Tauri CLI installed"
    else
        log_success "Tauri CLI already installed"
    fi
    
    # Install cargo-watch for development
    if ! cargo install --list | grep -q "cargo-watch"; then
        cargo install cargo-watch
        log_success "cargo-watch installed"
    else
        log_success "cargo-watch already installed"
    fi
    
    # Install cargo-audit for security auditing
    if ! cargo install --list | grep -q "cargo-audit"; then
        cargo install cargo-audit
        log_success "cargo-audit installed"
    else
        log_success "cargo-audit already installed"
    fi
    
    # Install cargo-tarpaulin for coverage
    if ! cargo install --list | grep -q "cargo-tarpaulin"; then
        cargo install cargo-tarpaulin
        log_success "cargo-tarpaulin installed"
    else
        log_success "cargo-tarpaulin already installed"
    fi
}

# Install Node.js dependencies
install_node_deps() {
    log_info "Installing Node.js dependencies..."
    
    if [ -f "package-lock.json" ]; then
        npm ci
    else
        npm install
    fi
    
    log_success "Node.js dependencies installed"
}

# Setup environment configuration
setup_environment() {
    log_info "Setting up environment configuration..."
    
    if [ ! -f ".env" ]; then
        if [ -f ".env.template" ]; then
            cp .env.template .env
            log_success "Created .env file from template"
            log_warning "Please edit .env file and add your API keys"
        else
            log_error ".env.template not found"
            exit 1
        fi
    else
        log_success ".env file already exists"
    fi
}

# Create necessary directories
create_directories() {
    log_info "Creating necessary directories..."
    
    mkdir -p data
    mkdir -p logs
    mkdir -p backups
    mkdir -p test-results
    mkdir -p coverage
    
    log_success "Directories created"
}

# Setup Git hooks
setup_git_hooks() {
    log_info "Setting up Git hooks..."
    
    if [ -d ".git" ]; then
        # Install husky if not already installed
        if ! npm list husky >/dev/null 2>&1; then
            npm install --save-dev husky
        fi
        
        # Initialize husky
        npx husky install
        
        # Add pre-commit hook
        npx husky add .husky/pre-commit "npm run lint && npm run type-check && cargo fmt --check && cargo clippy -- -D warnings"
        
        # Add pre-push hook
        npx husky add .husky/pre-push "npm test && cargo test"
        
        log_success "Git hooks configured"
    else
        log_warning "Not a Git repository. Skipping Git hooks setup."
    fi
}

# Run initial build
initial_build() {
    log_info "Running initial build..."
    
    # Build frontend
    npm run build:frontend
    log_success "Frontend built successfully"
    
    # Check Rust compilation
    cd src-tauri
    cargo check
    cd ..
    log_success "Rust code compiled successfully"
}

# Run tests
run_tests() {
    log_info "Running tests..."
    
    # Frontend tests
    npm test -- --run
    log_success "Frontend tests passed"
    
    # Backend tests
    cd src-tauri
    cargo test
    cd ..
    log_success "Backend tests passed"
}

# Main setup function
main() {
    echo "🚀 Free Deep Research System - Development Setup"
    echo "================================================"
    echo ""
    
    check_requirements
    install_system_deps
    install_rust_tools
    install_node_deps
    setup_environment
    create_directories
    setup_git_hooks
    initial_build
    run_tests
    
    echo ""
    echo "🎉 Setup completed successfully!"
    echo ""
    echo "Next steps:"
    echo "1. Edit .env file and add your API keys"
    echo "2. Run 'npm run dev' to start development server"
    echo "3. Open http://localhost:1420 in your browser"
    echo ""
    echo "For more information, see README.md and SETUP_GUIDE.md"
}

# Run main function
main "$@"
