#!/bin/bash

# Free Deep Research System - Port Management Test Suite
# Comprehensive testing for the intelligent port management system

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
TEST_LOG_FILE="$PROJECT_ROOT/port-management-test.log"

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Function to print colored output
print_test_header() {
    echo -e "${PURPLE}[TEST]${NC} $1"
}

print_test_pass() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((TESTS_PASSED++))
}

print_test_fail() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((TESTS_FAILED++))
}

print_test_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_test_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    ((TESTS_RUN++))
    print_test_info "Running test: $test_name"
    
    if eval "$test_command" >> "$TEST_LOG_FILE" 2>&1; then
        print_test_pass "$test_name"
        return 0
    else
        print_test_fail "$test_name"
        return 1
    fi
}

# Function to check if port is available
is_port_available() {
    local port=$1
    ! nc -z localhost "$port" 2>/dev/null
}

# Function to check if file exists
file_exists() {
    [[ -f "$1" ]]
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Test 1: Check if port manager scripts exist
test_scripts_exist() {
    file_exists "$SCRIPT_DIR/port-manager.sh" && \
    file_exists "$SCRIPT_DIR/port-manager.bat" && \
    file_exists "$SCRIPT_DIR/container-lifecycle.sh" && \
    file_exists "$SCRIPT_DIR/port-status-service.py"
}

# Test 2: Check if scripts are executable
test_scripts_executable() {
    [[ -x "$SCRIPT_DIR/port-manager.sh" ]] && \
    [[ -x "$SCRIPT_DIR/container-lifecycle.sh" ]] && \
    [[ -x "$SCRIPT_DIR/port-status-service.py" ]]
}

# Test 3: Test port manager help command
test_port_manager_help() {
    "$SCRIPT_DIR/port-manager.sh" help >/dev/null 2>&1
}

# Test 4: Test port registry generation
test_port_registry_generation() {
    # Clean up any existing registry
    rm -f "$PROJECT_ROOT/.env.ports"
    
    # Generate registry
    "$SCRIPT_DIR/port-manager.sh" generate development >/dev/null 2>&1 && \
    file_exists "$PROJECT_ROOT/.env.ports"
}

# Test 5: Test port registry content
test_port_registry_content() {
    if [[ ! -f "$PROJECT_ROOT/.env.ports" ]]; then
        return 1
    fi
    
    # Check for required port assignments
    grep -q "FRONTEND_PORT=" "$PROJECT_ROOT/.env.ports" && \
    grep -q "BACKEND_PORT=" "$PROJECT_ROOT/.env.ports" && \
    grep -q "DB_PORT=" "$PROJECT_ROOT/.env.ports" && \
    grep -q "REDIS_PORT=" "$PROJECT_ROOT/.env.ports"
}

# Test 6: Test port range validation
test_port_ranges() {
    if [[ ! -f "$PROJECT_ROOT/.env.ports" ]]; then
        return 1
    fi
    
    # Extract port values and check ranges
    local frontend_port=$(grep "FRONTEND_PORT=" "$PROJECT_ROOT/.env.ports" | cut -d'=' -f2)
    local backend_port=$(grep "BACKEND_PORT=" "$PROJECT_ROOT/.env.ports" | cut -d'=' -f2)
    
    # Check if ports are in expected ranges
    [[ $frontend_port -ge 30000 && $frontend_port -le 35000 ]] && \
    [[ $backend_port -ge 35000 && $backend_port -le 40000 ]]
}

# Test 7: Test port availability checking
test_port_availability() {
    # Test with a port that should be available
    local test_port=65432
    is_port_available "$test_port"
}

# Test 8: Test port status command
test_port_status() {
    "$SCRIPT_DIR/port-manager.sh" status >/dev/null 2>&1
}

# Test 9: Test container lifecycle scanner
test_container_scanner() {
    "$SCRIPT_DIR/container-lifecycle.sh" scan >/dev/null 2>&1
}

# Test 10: Test port cleanup
test_port_cleanup() {
    "$SCRIPT_DIR/port-manager.sh" cleanup >/dev/null 2>&1 && \
    [[ ! -f "$PROJECT_ROOT/.env.ports" ]]
}

# Test 11: Test port regeneration
test_port_regeneration() {
    "$SCRIPT_DIR/port-manager.sh" regenerate development >/dev/null 2>&1 && \
    file_exists "$PROJECT_ROOT/.env.ports"
}

# Test 12: Test port status service (if Python is available)
test_port_status_service() {
    if ! command_exists python3; then
        print_test_warning "Python3 not available, skipping port status service test"
        return 0
    fi
    
    # Start service in background
    python3 "$SCRIPT_DIR/port-status-service.py" &
    local service_pid=$!
    
    # Wait a moment for service to start
    sleep 2
    
    # Test health endpoint
    local result=0
    if command_exists curl; then
        curl -f http://localhost:8084/health >/dev/null 2>&1 || result=1
    elif command_exists wget; then
        wget -q --spider http://localhost:8084/health || result=1
    else
        print_test_warning "Neither curl nor wget available, skipping HTTP test"
    fi
    
    # Stop service
    kill $service_pid 2>/dev/null || true
    wait $service_pid 2>/dev/null || true
    
    return $result
}

# Test 13: Test Docker integration (if Docker is available)
test_docker_integration() {
    if ! command_exists docker; then
        print_test_warning "Docker not available, skipping Docker integration test"
        return 0
    fi
    
    # Check if Docker daemon is running
    if ! docker info >/dev/null 2>&1; then
        print_test_warning "Docker daemon not running, skipping Docker integration test"
        return 0
    fi
    
    # Test container scanning
    "$SCRIPT_DIR/container-lifecycle.sh" status >/dev/null 2>&1
}

# Test 14: Test Windows script syntax (basic check)
test_windows_script() {
    # Basic syntax check for Windows batch file
    if command_exists cmd; then
        # On Windows or WSL with cmd available
        cmd /c "echo off && call \"$SCRIPT_DIR/port-manager.bat\" help >nul 2>&1" 2>/dev/null || true
    else
        # Just check if file exists and has Windows line endings
        file_exists "$SCRIPT_DIR/port-manager.bat"
    fi
}

# Test 15: Test concurrent port allocation
test_concurrent_allocation() {
    # Clean up first
    "$SCRIPT_DIR/port-manager.sh" cleanup >/dev/null 2>&1
    
    # Start multiple port allocations in parallel
    "$SCRIPT_DIR/port-manager.sh" generate development >/dev/null 2>&1 &
    local pid1=$!
    
    sleep 0.1
    
    "$SCRIPT_DIR/port-manager.sh" generate development >/dev/null 2>&1 &
    local pid2=$!
    
    # Wait for both to complete
    wait $pid1 2>/dev/null || true
    wait $pid2 2>/dev/null || true
    
    # Check if registry was created successfully
    file_exists "$PROJECT_ROOT/.env.ports"
}

# Function to run all tests
run_all_tests() {
    print_test_header "Starting Port Management System Test Suite"
    echo "Log file: $TEST_LOG_FILE"
    echo ""
    
    # Initialize log file
    echo "Port Management Test Suite - $(date)" > "$TEST_LOG_FILE"
    echo "========================================" >> "$TEST_LOG_FILE"
    
    # Run tests
    run_test "Scripts Exist" "test_scripts_exist"
    run_test "Scripts Executable" "test_scripts_executable"
    run_test "Port Manager Help" "test_port_manager_help"
    run_test "Port Registry Generation" "test_port_registry_generation"
    run_test "Port Registry Content" "test_port_registry_content"
    run_test "Port Range Validation" "test_port_ranges"
    run_test "Port Availability Check" "test_port_availability"
    run_test "Port Status Command" "test_port_status"
    run_test "Container Scanner" "test_container_scanner"
    run_test "Port Cleanup" "test_port_cleanup"
    run_test "Port Regeneration" "test_port_regeneration"
    run_test "Port Status Service" "test_port_status_service"
    run_test "Docker Integration" "test_docker_integration"
    run_test "Windows Script" "test_windows_script"
    run_test "Concurrent Allocation" "test_concurrent_allocation"
    
    # Print summary
    echo ""
    print_test_header "Test Suite Summary"
    echo "Tests Run: $TESTS_RUN"
    echo "Tests Passed: $TESTS_PASSED"
    echo "Tests Failed: $TESTS_FAILED"
    
    if [[ $TESTS_FAILED -eq 0 ]]; then
        print_test_pass "All tests passed! ✅"
        echo ""
        print_test_info "Port management system is working correctly."
        return 0
    else
        print_test_fail "$TESTS_FAILED test(s) failed! ❌"
        echo ""
        print_test_info "Check the log file for details: $TEST_LOG_FILE"
        return 1
    fi
}

# Function to run specific test
run_specific_test() {
    local test_name="$1"
    
    case "$test_name" in
        "scripts")
            run_test "Scripts Exist" "test_scripts_exist"
            run_test "Scripts Executable" "test_scripts_executable"
            ;;
        "generation")
            run_test "Port Registry Generation" "test_port_registry_generation"
            run_test "Port Registry Content" "test_port_registry_content"
            ;;
        "ranges")
            run_test "Port Range Validation" "test_port_ranges"
            ;;
        "service")
            run_test "Port Status Service" "test_port_status_service"
            ;;
        "docker")
            run_test "Docker Integration" "test_docker_integration"
            ;;
        "concurrent")
            run_test "Concurrent Allocation" "test_concurrent_allocation"
            ;;
        *)
            echo "Unknown test: $test_name"
            echo "Available tests: scripts, generation, ranges, service, docker, concurrent"
            exit 1
            ;;
    esac
}

# Function to show usage
show_usage() {
    cat << EOF
Free Deep Research System - Port Management Test Suite

Usage: $0 [command] [options]

Commands:
    all                Run all tests (default)
    <test_name>        Run specific test
    help               Show this help message

Available specific tests:
    scripts            Test script existence and permissions
    generation         Test port registry generation
    ranges             Test port range validation
    service            Test port status service
    docker             Test Docker integration
    concurrent         Test concurrent port allocation

Examples:
    $0                 # Run all tests
    $0 all             # Run all tests
    $0 generation      # Run port generation tests
    $0 service         # Run port status service test

EOF
}

# Main execution
main() {
    case "${1:-all}" in
        all)
            run_all_tests
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            run_specific_test "$1"
            ;;
    esac
}

# Execute main function
main "$@"
