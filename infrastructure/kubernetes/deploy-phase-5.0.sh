#!/bin/bash

# =============================================================================
# Phase 5.0 Enhancement Deployment Script
# Free Deep Research System - Advanced AI/ML Capabilities
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
DEPLOYMENT_DATE=$(date +"%Y-%m-%d")

echo -e "${BLUE}==============================================================================${NC}"
echo -e "${BLUE}🚀 Free Deep Research System - Phase 5.0 Enhancement Deployment${NC}"
echo -e "${BLUE}   Advanced AI/ML Capabilities with RAG, Vector DB, Local LLM, and MCP${NC}"
echo -e "${BLUE}==============================================================================${NC}"
echo ""

# Function to print status
print_status() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

print_error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}"
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if kubectl is available
    if ! command -v kubectl &> /dev/null; then
        print_error "kubectl is not installed or not in PATH"
        exit 1
    fi
    
    # Check if helm is available
    if ! command -v helm &> /dev/null; then
        print_error "helm is not installed or not in PATH"
        exit 1
    fi
    
    # Check if namespace exists
    if ! kubectl get namespace $NAMESPACE &> /dev/null; then
        print_error "Namespace $NAMESPACE does not exist. Please run Phase 4 deployment first."
        exit 1
    fi
    
    # Check if previous phases are deployed
    if ! kubectl get deployment backend -n $NAMESPACE &> /dev/null; then
        print_error "Backend deployment not found. Please ensure Phase 4 is deployed."
        exit 1
    fi
    
    print_status "Prerequisites check passed ✅"
}

# Function to deploy vector database (Qdrant)
deploy_vector_database() {
    print_status "Deploying Qdrant Vector Database..."
    
    # Add Qdrant Helm repository
    helm repo add qdrant https://qdrant.github.io/qdrant-helm
    helm repo update
    
    # Deploy Qdrant
    kubectl apply -f phase-5.0/vector-db/
    
    # Wait for Qdrant to be ready
    kubectl wait --for=condition=available --timeout=300s deployment/qdrant -n $NAMESPACE
    
    print_status "Qdrant Vector Database deployed successfully ✅"
}

# Function to deploy RAG services
deploy_rag_services() {
    print_status "Deploying RAG (Retrieval-Augmented Generation) Services..."
    
    kubectl apply -f phase-5.0/rag/
    
    # Wait for RAG services to be ready
    kubectl wait --for=condition=available --timeout=300s deployment/rag-service -n $NAMESPACE
    kubectl wait --for=condition=available --timeout=300s deployment/embedding-service -n $NAMESPACE
    kubectl wait --for=condition=available --timeout=300s deployment/document-processor -n $NAMESPACE
    
    print_status "RAG Services deployed successfully ✅"
}

# Function to deploy local LLM services
deploy_local_llm() {
    print_status "Deploying Local LLM Services (Ollama)..."
    
    # Check for GPU nodes
    GPU_NODES=$(kubectl get nodes -l accelerator=nvidia-tesla-k80 --no-headers 2>/dev/null | wc -l || echo "0")
    if [ "$GPU_NODES" -eq 0 ]; then
        print_warning "No GPU nodes detected. Local LLM will run on CPU (slower performance)"
    else
        print_status "Found $GPU_NODES GPU nodes for accelerated inference"
    fi
    
    kubectl apply -f phase-5.0/local-llm/
    
    # Wait for Ollama to be ready
    kubectl wait --for=condition=available --timeout=600s deployment/ollama -n $NAMESPACE
    
    print_status "Local LLM Services deployed successfully ✅"
}

# Function to deploy AI provider gateway
deploy_ai_providers() {
    print_status "Deploying AI Provider Gateway..."
    
    kubectl apply -f phase-5.0/ai-providers/
    
    # Wait for provider gateway to be ready
    kubectl wait --for=condition=available --timeout=300s deployment/provider-gateway -n $NAMESPACE
    
    print_status "AI Provider Gateway deployed successfully ✅"
}

# Function to deploy optimization services
deploy_optimization() {
    print_status "Deploying Hybrid Optimization System..."
    
    kubectl apply -f phase-5.0/optimization/
    
    # Wait for optimization services to be ready
    kubectl wait --for=condition=available --timeout=300s deployment/model-router -n $NAMESPACE
    kubectl wait --for=condition=available --timeout=300s deployment/cost-optimizer -n $NAMESPACE
    
    print_status "Optimization System deployed successfully ✅"
}

# Function to deploy MCP server
deploy_mcp_server() {
    print_status "Deploying MCP (Model Context Protocol) Server..."
    
    kubectl apply -f phase-5.0/mcp/
    
    # Wait for MCP server to be ready
    kubectl wait --for=condition=available --timeout=300s deployment/mcp-server -n $NAMESPACE
    
    print_status "MCP Server deployed successfully ✅"
}

# Function to update GraphQL gateway
update_graphql_gateway() {
    print_status "Updating GraphQL Gateway with Phase 5.0 schema..."
    
    # Apply updated GraphQL gateway configuration
    kubectl apply -f phase-5.0/graphql/
    
    # Restart GraphQL gateway to load new schema
    kubectl rollout restart deployment/graphql-gateway -n $NAMESPACE
    kubectl rollout status deployment/graphql-gateway -n $NAMESPACE
    
    print_status "GraphQL Gateway updated successfully ✅"
}

# Function to run validation tests
run_validation() {
    print_status "Running Phase 5.0 validation tests..."
    
    # Run validation script
    ./validate-phase-5.0.sh
    
    print_status "Validation completed ✅"
}

# Function to display deployment summary
display_summary() {
    echo ""
    echo -e "${GREEN}==============================================================================${NC}"
    echo -e "${GREEN}🎉 Phase 5.0 Enhancement Deployment Complete!${NC}"
    echo -e "${GREEN}==============================================================================${NC}"
    echo ""
    echo -e "${BLUE}📊 Deployed Services:${NC}"
    echo "   • Qdrant Vector Database (v1.11.0)"
    echo "   • RAG Engine with Semantic Search"
    echo "   • Document Processing Pipeline"
    echo "   • Embedding Generation Service"
    echo "   • Local LLM Runtime (Ollama)"
    echo "   • AI Provider Gateway (HuggingFace, Groq, Together AI, Replicate)"
    echo "   • Hybrid Optimization System"
    echo "   • Model Context Protocol (MCP) Server"
    echo "   • Enhanced GraphQL API"
    echo ""
    echo -e "${BLUE}🔗 Access Points:${NC}"
    echo "   • RAG API: https://api.freedeepresearch.org/rag"
    echo "   • Vector Search: https://api.freedeepresearch.org/vector"
    echo "   • Local LLM: https://api.freedeepresearch.org/llm"
    echo "   • MCP Server: https://api.freedeepresearch.org/mcp"
    echo "   • GraphQL Playground: https://api.freedeepresearch.org/graphql"
    echo ""
    echo -e "${BLUE}📈 Performance Targets:${NC}"
    echo "   • RAG Query Response: <500ms"
    echo "   • Vector Search: <100ms"
    echo "   • Local LLM Inference: <2s"
    echo "   • API Cost Reduction: 40-60%"
    echo ""
    echo -e "${YELLOW}⚠️  Next Steps:${NC}"
    echo "   1. Configure API keys for external providers"
    echo "   2. Load initial document corpus for RAG"
    echo "   3. Download and configure local LLM models"
    echo "   4. Set up monitoring dashboards"
    echo "   5. Run performance benchmarks"
    echo ""
    echo -e "${GREEN}Phase 5.0 Enhancement is now ready for production use! 🚀${NC}"
}

# Main deployment flow
main() {
    print_status "Starting Phase 5.0 Enhancement deployment..."
    
    check_prerequisites
    deploy_vector_database
    deploy_rag_services
    deploy_local_llm
    deploy_ai_providers
    deploy_optimization
    deploy_mcp_server
    update_graphql_gateway
    run_validation
    display_summary
    
    print_status "Phase 5.0 deployment completed successfully! 🎉"
}

# Handle script interruption
trap 'print_error "Deployment interrupted. Please check the status of deployed resources."; exit 1' INT TERM

# Run main function
main "$@"
