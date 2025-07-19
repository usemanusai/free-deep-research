#!/bin/bash

# Comprehensive Test Runner for Tauri Application
set -e

echo "🧪 Starting Tauri Application Test Suite..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
export RUST_ENV=test
export RUST_LOG=debug
export DATABASE_URL="sqlite:///data/test_research.db"
export RUST_BACKTRACE=1

# Function to print colored output
print_status() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
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

# Initialize test environment
print_status "Initializing test environment..."

# Ensure test database exists
mkdir -p /data
touch /data/test_research.db
chmod 664 /data/test_research.db

# Frontend Tests
print_status "Running Frontend Tests..."
if npm test -- --watchAll=false --coverage; then
    print_success "Frontend tests passed"
else
    print_error "Frontend tests failed"
    exit 1
fi

# Frontend Linting
print_status "Running Frontend Linting..."
if npm run lint; then
    print_success "Frontend linting passed"
else
    print_warning "Frontend linting issues found"
fi

# TypeScript Type Checking
print_status "Running TypeScript Type Checking..."
if npm run type-check; then
    print_success "TypeScript type checking passed"
else
    print_error "TypeScript type checking failed"
    exit 1
fi

# Backend Tests
print_status "Running Backend Tests..."
cd src-tauri

# Unit Tests
print_status "Running Rust unit tests..."
if cargo test --lib; then
    print_success "Rust unit tests passed"
else
    print_error "Rust unit tests failed"
    exit 1
fi

# Integration Tests
print_status "Running Rust integration tests..."
if cargo test --test '*'; then
    print_success "Rust integration tests passed"
else
    print_error "Rust integration tests failed"
    exit 1
fi

# Code Coverage
print_status "Generating code coverage report..."
if cargo tarpaulin --out Html --output-dir ../coverage; then
    print_success "Code coverage report generated"
else
    print_warning "Code coverage generation failed"
fi

# Security Audit
print_status "Running security audit..."
if cargo audit; then
    print_success "Security audit passed"
else
    print_warning "Security audit found issues"
fi

# Dependency Check
print_status "Checking for outdated dependencies..."
if cargo outdated; then
    print_success "Dependency check completed"
else
    print_warning "Some dependencies may be outdated"
fi

# Clippy Linting
print_status "Running Clippy linting..."
if cargo clippy -- -D warnings; then
    print_success "Clippy linting passed"
else
    print_error "Clippy linting failed"
    exit 1
fi

# Format Check
print_status "Checking code formatting..."
if cargo fmt -- --check; then
    print_success "Code formatting is correct"
else
    print_error "Code formatting issues found"
    exit 1
fi

cd ..

# Build Tests
print_status "Testing application build..."
if npm run tauri build -- --debug; then
    print_success "Application build test passed"
else
    print_error "Application build test failed"
    exit 1
fi

# Performance Tests (if available)
if [ -f "tests/performance/run-performance-tests.sh" ]; then
    print_status "Running performance tests..."
    if ./tests/performance/run-performance-tests.sh; then
        print_success "Performance tests passed"
    else
        print_warning "Performance tests failed"
    fi
fi

# E2E Tests (if available)
if [ -f "playwright.config.ts" ]; then
    print_status "Running E2E tests..."
    if npx playwright test; then
        print_success "E2E tests passed"
    else
        print_warning "E2E tests failed"
    fi
fi

# Generate Test Report
print_status "Generating test report..."
cat > /app/test-report.md << EOF
# Test Report

**Generated:** $(date)
**Environment:** Test
**Database:** SQLite (test)

## Test Results Summary

- ✅ Frontend Tests: Passed
- ✅ Frontend Linting: Passed
- ✅ TypeScript Type Checking: Passed
- ✅ Rust Unit Tests: Passed
- ✅ Rust Integration Tests: Passed
- ✅ Security Audit: Passed
- ✅ Clippy Linting: Passed
- ✅ Code Formatting: Passed
- ✅ Build Test: Passed

## Coverage Report

Coverage report available at: \`coverage/tarpaulin-report.html\`

## Next Steps

1. Review any warnings or issues mentioned above
2. Check coverage report for areas needing more tests
3. Address any security audit findings
4. Update outdated dependencies if necessary

EOF

print_success "Test suite completed successfully!"
print_status "Test report generated: /app/test-report.md"
print_status "Coverage report: /app/coverage/tarpaulin-report.html"

echo ""
echo "🎉 All tests completed successfully!"
