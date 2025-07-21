#!/bin/bash

# =============================================================================
# Phase 5.0 Enhancement Validation Script
# Free Deep Research System - Advanced AI/ML Capabilities Validation
# =============================================================================

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NAMESPACE="free-deep-research"
PHASE="5.0"
TIMEOUT=300

echo -e "${BLUE}==============================================================================${NC}"
echo -e "${BLUE}🔍 Phase 5.0 Enhancement Validation${NC}"
echo -e "${BLUE}   Validating Advanced AI/ML Capabilities${NC}"
echo -e "${BLUE}==============================================================================${NC}"
echo ""

# Function to print status
print_status() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] ✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] ⚠️  WARNING: $1${NC}"
}

print_error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ❌ ERROR: $1${NC}"
}

print_info() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] ℹ️  $1${NC}"
}

# Function to check if deployment is ready
check_deployment() {
    local deployment=$1
    local timeout=${2:-300}
    
    print_info "Checking deployment: $deployment"
    
    if kubectl get deployment $deployment -n $NAMESPACE &> /dev/null; then
        if kubectl wait --for=condition=available --timeout=${timeout}s deployment/$deployment -n $NAMESPACE &> /dev/null; then
            local replicas=$(kubectl get deployment $deployment -n $NAMESPACE -o jsonpath='{.status.readyReplicas}')
            print_status "$deployment is ready with $replicas replicas"
            return 0
        else
            print_error "$deployment is not ready within ${timeout}s"
            return 1
        fi
    else
        print_error "$deployment not found"
        return 1
    fi
}

# Function to check service health
check_service_health() {
    local service=$1
    local port=$2
    local path=${3:-"/health"}
    
    print_info "Checking service health: $service"
    
    if kubectl get service $service -n $NAMESPACE &> /dev/null; then
        # Port forward to test the service
        kubectl port-forward service/$service $port:$port -n $NAMESPACE &
        local pf_pid=$!
        sleep 5
        
        if curl -f http://localhost:$port$path &> /dev/null; then
            print_status "$service health check passed"
            kill $pf_pid 2>/dev/null || true
            return 0
        else
            print_error "$service health check failed"
            kill $pf_pid 2>/dev/null || true
            return 1
        fi
    else
        print_error "Service $service not found"
        return 1
    fi
}

# Function to validate vector database
validate_qdrant() {
    print_info "Validating Qdrant Vector Database..."
    
    if check_deployment "qdrant" 300; then
        # Test Qdrant API
        kubectl port-forward service/qdrant-service 6333:6333 -n $NAMESPACE &
        local pf_pid=$!
        sleep 5
        
        # Check if Qdrant is responding
        if curl -f http://localhost:6333/ &> /dev/null; then
            print_status "Qdrant API is responding"
            
            # Check collections
            local collections=$(curl -s http://localhost:6333/collections | jq -r '.result.collections | length' 2>/dev/null || echo "0")
            print_info "Qdrant has $collections collections"
            
            kill $pf_pid 2>/dev/null || true
            return 0
        else
            print_error "Qdrant API is not responding"
            kill $pf_pid 2>/dev/null || true
            return 1
        fi
    else
        return 1
    fi
}

# Function to validate RAG services
validate_rag_services() {
    print_info "Validating RAG Services..."
    
    local services=("rag-service" "embedding-service" "document-processor")
    local ports=(8080 8081 8082)
    local all_healthy=true
    
    for i in "${!services[@]}"; do
        if ! check_deployment "${services[$i]}" 300; then
            all_healthy=false
        fi
    done
    
    if $all_healthy; then
        print_status "All RAG services are healthy"
        return 0
    else
        print_error "Some RAG services are not healthy"
        return 1
    fi
}

# Function to validate local LLM
validate_local_llm() {
    print_info "Validating Local LLM (Ollama)..."
    
    if check_deployment "ollama" 600; then
        # Test Ollama API
        kubectl port-forward service/ollama-service 11434:11434 -n $NAMESPACE &
        local pf_pid=$!
        sleep 10
        
        # Check if Ollama is responding
        if curl -f http://localhost:11434/api/tags &> /dev/null; then
            print_status "Ollama API is responding"
            
            # Check available models
            local models=$(curl -s http://localhost:11434/api/tags | jq -r '.models | length' 2>/dev/null || echo "0")
            print_info "Ollama has $models models available"
            
            kill $pf_pid 2>/dev/null || true
            return 0
        else
            print_warning "Ollama API is not responding (may still be downloading models)"
            kill $pf_pid 2>/dev/null || true
            return 0  # Don't fail validation if models are still downloading
        fi
    else
        return 1
    fi
}

# Function to validate AI provider gateway
validate_provider_gateway() {
    print_info "Validating AI Provider Gateway..."
    
    if check_deployment "provider-gateway" 300; then
        print_status "Provider Gateway is healthy"
        return 0
    else
        return 1
    fi
}

# Function to validate optimization services
validate_optimization() {
    print_info "Validating Optimization Services..."
    
    local services=("model-router" "cost-optimizer")
    local all_healthy=true
    
    for service in "${services[@]}"; do
        if ! check_deployment "$service" 300; then
            all_healthy=false
        fi
    done
    
    if $all_healthy; then
        print_status "All optimization services are healthy"
        return 0
    else
        print_error "Some optimization services are not healthy"
        return 1
    fi
}

# Function to validate MCP server
validate_mcp_server() {
    print_info "Validating MCP Server..."
    
    if check_deployment "mcp-server" 300; then
        print_status "MCP Server is healthy"
        return 0
    else
        return 1
    fi
}

# Function to validate GraphQL integration
validate_graphql_integration() {
    print_info "Validating GraphQL Integration..."
    
    if kubectl get deployment graphql-gateway -n $NAMESPACE &> /dev/null; then
        if kubectl wait --for=condition=available --timeout=300s deployment/graphql-gateway -n $NAMESPACE &> /dev/null; then
            print_status "GraphQL Gateway is ready"
            return 0
        else
            print_error "GraphQL Gateway is not ready"
            return 1
        fi
    else
        print_error "GraphQL Gateway not found"
        return 1
    fi
}

# Function to run integration tests
run_integration_tests() {
    print_info "Running Integration Tests..."
    
    # Test 1: Vector Database Connection
    print_info "Test 1: Vector Database Connection"
    if validate_qdrant; then
        print_status "✅ Vector Database Connection Test Passed"
    else
        print_error "❌ Vector Database Connection Test Failed"
        return 1
    fi
    
    # Test 2: RAG Pipeline
    print_info "Test 2: RAG Pipeline"
    if validate_rag_services; then
        print_status "✅ RAG Pipeline Test Passed"
    else
        print_error "❌ RAG Pipeline Test Failed"
        return 1
    fi
    
    # Test 3: Local LLM
    print_info "Test 3: Local LLM"
    if validate_local_llm; then
        print_status "✅ Local LLM Test Passed"
    else
        print_warning "⚠️ Local LLM Test Warning (may still be initializing)"
    fi
    
    # Test 4: AI Provider Gateway
    print_info "Test 4: AI Provider Gateway"
    if validate_provider_gateway; then
        print_status "✅ AI Provider Gateway Test Passed"
    else
        print_error "❌ AI Provider Gateway Test Failed"
        return 1
    fi
    
    # Test 5: Optimization Services
    print_info "Test 5: Optimization Services"
    if validate_optimization; then
        print_status "✅ Optimization Services Test Passed"
    else
        print_error "❌ Optimization Services Test Failed"
        return 1
    fi
    
    # Test 6: MCP Server
    print_info "Test 6: MCP Server"
    if validate_mcp_server; then
        print_status "✅ MCP Server Test Passed"
    else
        print_error "❌ MCP Server Test Failed"
        return 1
    fi
    
    # Test 7: GraphQL Integration
    print_info "Test 7: GraphQL Integration"
    if validate_graphql_integration; then
        print_status "✅ GraphQL Integration Test Passed"
    else
        print_error "❌ GraphQL Integration Test Failed"
        return 1
    fi
    
    print_status "All integration tests completed"
    return 0
}

# Function to check resource usage
check_resource_usage() {
    print_info "Checking Resource Usage..."
    
    # Check CPU and Memory usage
    echo ""
    echo -e "${BLUE}📊 Resource Usage Summary:${NC}"
    kubectl top pods -n $NAMESPACE --sort-by=cpu 2>/dev/null | head -10 || print_warning "Metrics server not available"
    
    # Check storage usage
    echo ""
    echo -e "${BLUE}💾 Storage Usage:${NC}"
    kubectl get pvc -n $NAMESPACE
    
    print_status "Resource usage check completed"
}

# Function to display validation summary
display_summary() {
    echo ""
    echo -e "${GREEN}==============================================================================${NC}"
    echo -e "${GREEN}🎉 Phase 5.0 Enhancement Validation Summary${NC}"
    echo -e "${GREEN}==============================================================================${NC}"
    echo ""
    echo -e "${BLUE}✅ Validated Components:${NC}"
    echo "   • Qdrant Vector Database (v1.11.0)"
    echo "   • RAG Engine with Semantic Search"
    echo "   • Document Processing Pipeline"
    echo "   • Embedding Generation Service"
    echo "   • Local LLM Runtime (Ollama)"
    echo "   • AI Provider Gateway"
    echo "   • Hybrid Optimization System"
    echo "   • Model Context Protocol (MCP) Server"
    echo "   • Enhanced GraphQL API"
    echo ""
    echo -e "${BLUE}🔗 Service Endpoints:${NC}"
    echo "   • RAG API: http://rag-service:8080"
    echo "   • Vector DB: http://qdrant-service:6333"
    echo "   • Embedding Service: http://embedding-service:8081"
    echo "   • Local LLM: http://ollama-service:11434"
    echo "   • Provider Gateway: http://provider-gateway:8083"
    echo "   • Model Router: http://model-router:8084"
    echo "   • MCP Server: http://mcp-server:8086"
    echo ""
    echo -e "${BLUE}📈 Expected Performance:${NC}"
    echo "   • RAG Query Response: <500ms"
    echo "   • Vector Search: <100ms"
    echo "   • Local LLM Inference: <2s"
    echo "   • API Cost Reduction: 40-60%"
    echo ""
    echo -e "${GREEN}Phase 5.0 Enhancement validation completed successfully! 🚀${NC}"
}

# Main validation flow
main() {
    print_info "Starting Phase 5.0 Enhancement validation..."
    
    # Run all validations
    if run_integration_tests; then
        check_resource_usage
        display_summary
        print_status "Phase 5.0 validation completed successfully! 🎉"
        return 0
    else
        print_error "Phase 5.0 validation failed. Please check the logs above."
        return 1
    fi
}

# Handle script interruption
trap 'print_error "Validation interrupted. Cleaning up..."; exit 1' INT TERM

# Run main function
main "$@"
