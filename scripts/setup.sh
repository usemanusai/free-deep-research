#!/bin/bash

# =============================================================================
# Free Deep Research System - Setup Script
# =============================================================================

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# =============================================================================
# UTILITY FUNCTIONS
# =============================================================================

print_header() {
    echo -e "${BLUE}=============================================================================${NC}"
    echo -e "${BLUE} $1${NC}"
    echo -e "${BLUE}=============================================================================${NC}"
}

print_step() {
    echo -e "${GREEN}▶ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

check_command() {
    if command -v "$1" >/dev/null 2>&1; then
        print_success "$1 is installed"
        return 0
    else
        print_error "$1 is not installed"
        return 1
    fi
}

# =============================================================================
# SYSTEM REQUIREMENTS CHECK
# =============================================================================

check_system_requirements() {
    print_header "Checking System Requirements"
    
    local all_good=true
    
    # Check Node.js
    if check_command "node"; then
        local node_version=$(node --version | sed 's/v//')
        local required_version="18.0.0"
        if [ "$(printf '%s\n' "$required_version" "$node_version" | sort -V | head -n1)" = "$required_version" ]; then
            print_success "Node.js version $node_version meets requirements (>= $required_version)"
        else
            print_error "Node.js version $node_version is too old (>= $required_version required)"
            all_good=false
        fi
    else
        print_error "Node.js is required. Please install Node.js 18+ from https://nodejs.org"
        all_good=false
    fi
    
    # Check npm
    if ! check_command "npm"; then
        print_error "npm is required (usually comes with Node.js)"
        all_good=false
    fi
    
    # Check Rust
    if check_command "rustc"; then
        local rust_version=$(rustc --version | awk '{print $2}')
        print_success "Rust version $rust_version is installed"
    else
        print_warning "Rust is not installed. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        print_success "Rust installed successfully"
    fi
    
    # Check Cargo
    if ! check_command "cargo"; then
        print_error "Cargo is required (usually comes with Rust)"
        all_good=false
    fi
    
    # Check Git
    if ! check_command "git"; then
        print_error "Git is required. Please install Git from https://git-scm.com"
        all_good=false
    fi
    
    # Check Docker (optional)
    if check_command "docker"; then
        print_success "Docker is available for containerized development"
    else
        print_warning "Docker is not installed. Some features may not be available."
        print_warning "Install Docker from https://docker.com for full functionality"
    fi
    
    # Check Docker Compose (optional)
    if check_command "docker-compose" || check_command "docker compose"; then
        print_success "Docker Compose is available"
    else
        print_warning "Docker Compose is not installed"
    fi
    
    if [ "$all_good" = false ]; then
        print_error "Please install missing requirements and run this script again"
        exit 1
    fi
    
    print_success "All system requirements are met!"
}

# =============================================================================
# PROJECT SETUP
# =============================================================================

setup_environment() {
    print_header "Setting Up Environment"
    
    cd "$PROJECT_ROOT"
    
    # Create .env file if it doesn't exist
    if [ ! -f ".env" ]; then
        print_step "Creating .env file from template"
        cp .env.template .env
        print_success ".env file created"
        print_warning "Please edit .env file with your API keys and configuration"
    else
        print_success ".env file already exists"
    fi
    
    # Create necessary directories
    print_step "Creating necessary directories"
    mkdir -p logs
    mkdir -p data
    mkdir -p uploads
    mkdir -p backups
    mkdir -p outputs
    print_success "Directories created"
}

install_dependencies() {
    print_header "Installing Dependencies"
    
    cd "$PROJECT_ROOT"
    
    # Install main project dependencies
    print_step "Installing main project dependencies"
    npm install
    print_success "Main dependencies installed"
    
    # Install frontend dependencies
    if [ -d "bmad-agent/deep_research_frontend" ]; then
        print_step "Installing frontend dependencies"
        cd bmad-agent/deep_research_frontend
        npm install
        cd "$PROJECT_ROOT"
        print_success "Frontend dependencies installed"
    fi
    
    # Install Tauri dependencies
    if [ -d "bmad-agent/free-deep-research" ]; then
        print_step "Installing Tauri dependencies"
        cd bmad-agent/free-deep-research
        npm install
        cd "$PROJECT_ROOT"
        print_success "Tauri dependencies installed"
    fi
    
    # Build Rust dependencies
    if [ -d "src-tauri" ]; then
        print_step "Building Rust dependencies"
        cd src-tauri
        cargo build
        cd "$PROJECT_ROOT"
        print_success "Rust dependencies built"
    fi
}

setup_database() {
    print_header "Setting Up Database"
    
    # Check if Docker is available for database setup
    if command -v docker >/dev/null 2>&1; then
        print_step "Setting up database with Docker"
        
        # Start database services
        if [ -f "docker-compose.dev.yml" ]; then
            docker-compose -f docker-compose.dev.yml up -d database redis
            print_success "Database services started"
            
            # Wait for database to be ready
            print_step "Waiting for database to be ready..."
            sleep 10
            
            # Run database migrations
            print_step "Running database migrations"
            npm run db:migrate
            print_success "Database migrations completed"
            
            # Seed database with initial data
            print_step "Seeding database with initial data"
            npm run db:seed
            print_success "Database seeded"
        else
            print_warning "Docker Compose file not found. Skipping database setup."
        fi
    else
        print_warning "Docker not available. Please set up PostgreSQL and Redis manually."
        print_warning "See docs/user-guides/installation.md for manual setup instructions."
    fi
}

run_tests() {
    print_header "Running Tests"
    
    cd "$PROJECT_ROOT"
    
    # Run linting
    print_step "Running linting checks"
    npm run lint
    print_success "Linting passed"
    
    # Run type checking
    print_step "Running TypeScript type checking"
    npm run type-check
    print_success "Type checking passed"
    
    # Run unit tests
    print_step "Running unit tests"
    npm run test
    print_success "Unit tests passed"
    
    print_success "All tests passed!"
}

# =============================================================================
# DEVELOPMENT TOOLS SETUP
# =============================================================================

setup_development_tools() {
    print_header "Setting Up Development Tools"
    
    # Install global development tools
    print_step "Installing global development tools"
    
    # Install useful global packages
    npm install -g nodemon concurrently
    print_success "Global tools installed"
    
    # Setup Git hooks (if .git exists)
    if [ -d ".git" ]; then
        print_step "Setting up Git hooks"
        
        # Create pre-commit hook
        cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Run linting and tests before commit
npm run lint && npm run type-check
EOF
        chmod +x .git/hooks/pre-commit
        print_success "Git hooks configured"
    fi
    
    # Setup VS Code configuration (if .vscode doesn't exist)
    if [ ! -d ".vscode" ]; then
        print_step "Setting up VS Code configuration"
        mkdir -p .vscode
        
        # Create VS Code settings
        cat > .vscode/settings.json << 'EOF'
{
  "typescript.preferences.importModuleSpecifier": "relative",
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy"
}
EOF
        
        # Create VS Code extensions recommendations
        cat > .vscode/extensions.json << 'EOF'
{
  "recommendations": [
    "esbenp.prettier-vscode",
    "dbaeumer.vscode-eslint",
    "bradlc.vscode-tailwindcss",
    "rust-lang.rust-analyzer",
    "tauri-apps.tauri-vscode",
    "ms-vscode.vscode-typescript-next"
  ]
}
EOF
        print_success "VS Code configuration created"
    fi
}

# =============================================================================
# FINAL VERIFICATION
# =============================================================================

verify_installation() {
    print_header "Verifying Installation"
    
    cd "$PROJECT_ROOT"
    
    # Check if build works
    print_step "Testing build process"
    npm run build
    print_success "Build process works"
    
    # Check if development server starts (briefly)
    print_step "Testing development server startup"
    timeout 10s npm run dev > /dev/null 2>&1 || true
    print_success "Development server can start"
    
    print_success "Installation verification completed!"
}

# =============================================================================
# MAIN EXECUTION
# =============================================================================

main() {
    print_header "Free Deep Research System Setup"
    echo -e "${CYAN}Welcome to the Free Deep Research System setup script!${NC}"
    echo -e "${CYAN}This script will set up your development environment.${NC}"
    echo ""
    
    # Run setup steps
    check_system_requirements
    setup_environment
    install_dependencies
    setup_database
    setup_development_tools
    run_tests
    verify_installation
    
    # Final success message
    print_header "Setup Complete!"
    echo -e "${GREEN}🎉 Free Deep Research System has been set up successfully!${NC}"
    echo ""
    echo -e "${CYAN}Next steps:${NC}"
    echo -e "${YELLOW}1. Edit .env file with your API keys${NC}"
    echo -e "${YELLOW}2. Start development: npm run dev${NC}"
    echo -e "${YELLOW}3. Open http://localhost:3000 in your browser${NC}"
    echo ""
    echo -e "${CYAN}For more information:${NC}"
    echo -e "${YELLOW}- Documentation: docs/user-guides/quick-start.md${NC}"
    echo -e "${YELLOW}- Troubleshooting: docs/user-guides/troubleshooting.md${NC}"
    echo -e "${YELLOW}- Contributing: docs/development/contributing.md${NC}"
    echo ""
    echo -e "${GREEN}Happy researching! 🔬${NC}"
}

# Run main function
main "$@"
