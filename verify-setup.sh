#!/bin/bash

# Free Deep Research System - Setup Verification Script
# Verifies critical gap fixes and system readiness

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print functions
print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
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

# Verification functions
verify_environment_files() {
    print_header "Verifying Environment Configuration"
    
    local files=(
        ".env"
        "bmad-agent/free-deep-research/.env"
        "bmad-agent/deep_research_frontend/.env"
        ".env.example"
    )
    
    for file in "${files[@]}"; do
        if [[ -f "$file" ]]; then
            print_success "Found: $file"
        else
            print_error "Missing: $file"
            return 1
        fi
    done
    
    print_success "All environment files present"
}

verify_ai_orchestrator() {
    print_header "Verifying AI Orchestrator Configuration"
    
    if [[ -f "ai-orchestrator/agent-config.txt" ]]; then
        print_success "AI Orchestrator configuration found"
        
        # Check for required sections
        local sections=(
            "CONFIGURATION"
            "AGENTS"
            "WORKFLOWS"
            "RESEARCH_INTEGRATION"
        )
        
        for section in "${sections[@]}"; do
            if grep -q "START: $section" "ai-orchestrator/agent-config.txt"; then
                print_success "Section found: $section"
            else
                print_warning "Section missing: $section"
            fi
        done
    else
        print_error "AI Orchestrator configuration missing"
        return 1
    fi
}

verify_dependencies() {
    print_header "Verifying Dependencies"
    
    # Check Node.js
    if command -v node >/dev/null 2>&1; then
        local node_version=$(node --version)
        print_success "Node.js: $node_version"
    else
        print_error "Node.js not found"
    fi
    
    # Check Rust
    if command -v cargo >/dev/null 2>&1; then
        local rust_version=$(cargo --version)
        print_success "Rust: $rust_version"
    else
        print_error "Rust not found"
    fi
    
    # Check Docker
    if command -v docker >/dev/null 2>&1; then
        local docker_version=$(docker --version)
        print_success "Docker: $docker_version"
    else
        print_warning "Docker not found (optional for desktop app)"
    fi
}

verify_package_files() {
    print_header "Verifying Package Configurations"
    
    local package_files=(
        "bmad-agent/free-deep-research/package.json"
        "bmad-agent/deep_research_frontend/package.json"
        "bmad-agent/free-deep-research/src-tauri/Cargo.toml"
    )
    
    for file in "${package_files[@]}"; do
        if [[ -f "$file" ]]; then
            print_success "Package file: $file"
        else
            print_error "Missing package file: $file"
        fi
    done
}

verify_docker_config() {
    print_header "Verifying Docker Configuration"
    
    local docker_files=(
        "docker-compose.yml"
        "docker-compose.dev.yml"
        "docker-compose.prod.yml"
    )
    
    for file in "${docker_files[@]}"; do
        if [[ -f "$file" ]]; then
            print_success "Docker config: $file"
        else
            print_warning "Missing Docker config: $file"
        fi
    done
    
    # Check for Docker directories
    if [[ -d "docker" ]]; then
        print_success "Docker directory structure present"
    else
        print_error "Docker directory missing"
    fi
}

verify_bmad_structure() {
    print_header "Verifying BMAD Agent Structure"
    
    local bmad_dirs=(
        "bmad-agent/personas"
        "bmad-agent/tasks"
        "bmad-agent/templates"
        "bmad-agent/checklists"
        "bmad-agent/data"
    )
    
    for dir in "${bmad_dirs[@]}"; do
        if [[ -d "$dir" ]]; then
            print_success "BMAD directory: $dir"
        else
            print_error "Missing BMAD directory: $dir"
        fi
    done
}

check_critical_gaps() {
    print_header "Checking for Critical Gaps"
    
    local critical_issues=0
    
    # Check environment files
    if [[ ! -f ".env" ]]; then
        print_error "Critical: Main .env file missing"
        ((critical_issues++))
    fi
    
    # Check AI orchestrator config
    if [[ ! -f "ai-orchestrator/agent-config.txt" ]]; then
        print_error "Critical: AI Orchestrator config missing"
        ((critical_issues++))
    fi
    
    # Check package files
    if [[ ! -f "bmad-agent/free-deep-research/package.json" ]]; then
        print_error "Critical: Desktop app package.json missing"
        ((critical_issues++))
    fi
    
    if [[ $critical_issues -eq 0 ]]; then
        print_success "No critical gaps detected"
        return 0
    else
        print_error "Found $critical_issues critical issues"
        return 1
    fi
}

test_basic_setup() {
    print_header "Testing Basic Setup"
    
    # Test desktop app dependencies
    if [[ -d "bmad-agent/free-deep-research" ]]; then
        print_info "Testing desktop app dependencies..."
        cd bmad-agent/free-deep-research
        
        if npm list >/dev/null 2>&1; then
            print_success "Desktop app dependencies OK"
        else
            print_warning "Desktop app dependencies need installation"
        fi
        
        cd ../..
    fi
    
    # Test frontend dependencies
    if [[ -d "bmad-agent/deep_research_frontend" ]]; then
        print_info "Testing frontend dependencies..."
        cd bmad-agent/deep_research_frontend
        
        if npm list >/dev/null 2>&1; then
            print_success "Frontend dependencies OK"
        else
            print_warning "Frontend dependencies need installation"
        fi
        
        cd ../..
    fi
}

generate_next_steps() {
    print_header "Next Steps Recommendations"
    
    print_info "1. Install dependencies:"
    echo "   cd bmad-agent/free-deep-research && npm install"
    echo "   cd bmad-agent/deep_research_frontend && npm install"
    
    print_info "2. Update API keys in .env files"
    
    print_info "3. Test desktop app:"
    echo "   cd bmad-agent/free-deep-research && npm run tauri dev"
    
    print_info "4. Test Docker deployment:"
    echo "   ./setup.sh"
    
    print_info "5. Follow Phase 1 implementation roadmap"
}

# Main execution
main() {
    print_header "Free Deep Research System - Setup Verification"
    print_info "Checking system readiness and critical gap fixes..."
    
    local exit_code=0
    
    verify_environment_files || exit_code=1
    verify_ai_orchestrator || exit_code=1
    verify_dependencies || exit_code=1
    verify_package_files || exit_code=1
    verify_docker_config || exit_code=1
    verify_bmad_structure || exit_code=1
    
    check_critical_gaps || exit_code=1
    
    test_basic_setup
    
    if [[ $exit_code -eq 0 ]]; then
        print_header "Verification Complete"
        print_success "System ready for Phase 1 implementation!"
        print_info "All critical gaps have been addressed."
    else
        print_header "Verification Issues Found"
        print_error "Some issues detected. Please review and fix before proceeding."
    fi
    
    generate_next_steps
    
    exit $exit_code
}

# Run main function
main "$@"
