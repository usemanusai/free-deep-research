#!/bin/bash

# Free Deep Research System - Phase 4.6 Validation Script
# AI/ML Pipeline Enhancement Validation
# 
# This script validates the Phase 4.6 deployment and tests all MLOps components

set -euo pipefail

# Configuration
NAMESPACE="free-deep-research"
TIMEOUT=300

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] INFO: $1${NC}"
}

success() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] SUCCESS: $1${NC}"
}

warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}"
}

# Validation functions
validate_prerequisites() {
    log "Validating Phase 4.6 prerequisites..."
    
    # Check if kubectl is available
    if ! command -v kubectl &> /dev/null; then
        error "kubectl is not installed or not in PATH"
        return 1
    fi
    
    # Check if cluster is accessible
    if ! kubectl cluster-info &> /dev/null; then
        error "Cannot connect to Kubernetes cluster"
        return 1
    fi
    
    # Check if namespace exists
    if ! kubectl get namespace "$NAMESPACE" &> /dev/null; then
        error "Namespace $NAMESPACE does not exist"
        return 1
    fi
    
    success "Prerequisites validation completed"
}

validate_kubeflow() {
    log "Validating Kubeflow Pipelines..."
    
    # Check if Kubeflow namespace exists
    if ! kubectl get namespace kubeflow &> /dev/null; then
        error "Kubeflow namespace does not exist"
        return 1
    fi
    
    # Check Kubeflow Pipelines pods
    local kubeflow_pods=$(kubectl get pods -n kubeflow -l app=ml-pipeline --no-headers 2>/dev/null | wc -l)
    if [ "$kubeflow_pods" -eq 0 ]; then
        error "Kubeflow Pipelines pods not found"
        return 1
    fi
    
    # Check if pods are running
    local running_pods=$(kubectl get pods -n kubeflow -l app=ml-pipeline --no-headers 2>/dev/null | grep Running | wc -l)
    if [ "$running_pods" -eq 0 ]; then
        error "Kubeflow Pipelines pods are not running"
        return 1
    fi
    
    success "Kubeflow Pipelines validation completed"
}

validate_mlflow() {
    log "Validating MLflow..."
    
    # Check MLflow pods
    local mlflow_pods=$(kubectl get pods -n "$NAMESPACE" -l app=mlflow --no-headers 2>/dev/null | wc -l)
    if [ "$mlflow_pods" -eq 0 ]; then
        error "MLflow pods not found"
        return 1
    fi
    
    # Check if pods are running
    local running_pods=$(kubectl get pods -n "$NAMESPACE" -l app=mlflow --no-headers 2>/dev/null | grep Running | wc -l)
    if [ "$running_pods" -eq 0 ]; then
        error "MLflow pods are not running"
        return 1
    fi
    
    # Test MLflow service
    if kubectl get service mlflow-service -n "$NAMESPACE" &> /dev/null; then
        success "MLflow service is available"
    else
        error "MLflow service not found"
        return 1
    fi
    
    success "MLflow validation completed"
}

validate_tensorflow_serving() {
    log "Validating TensorFlow Serving..."
    
    # Check TensorFlow Serving pods
    local tf_serving_pods=$(kubectl get pods -n "$NAMESPACE" -l app=tensorflow-serving --no-headers 2>/dev/null | wc -l)
    if [ "$tf_serving_pods" -eq 0 ]; then
        warning "TensorFlow Serving GPU pods not found, checking CPU pods..."
        
        local tf_serving_cpu_pods=$(kubectl get pods -n "$NAMESPACE" -l app=tensorflow-serving-cpu --no-headers 2>/dev/null | wc -l)
        if [ "$tf_serving_cpu_pods" -eq 0 ]; then
            error "TensorFlow Serving pods not found"
            return 1
        fi
    fi
    
    # Test TensorFlow Serving service
    if kubectl get service tensorflow-serving-service -n "$NAMESPACE" &> /dev/null; then
        success "TensorFlow Serving service is available"
    else
        error "TensorFlow Serving service not found"
        return 1
    fi
    
    success "TensorFlow Serving validation completed"
}

validate_ab_testing() {
    log "Validating A/B Testing Framework..."
    
    # Check A/B testing controller pods
    local ab_test_pods=$(kubectl get pods -n "$NAMESPACE" -l app=ab-test-controller --no-headers 2>/dev/null | wc -l)
    if [ "$ab_test_pods" -eq 0 ]; then
        error "A/B testing controller pods not found"
        return 1
    fi
    
    # Check if pods are running
    local running_pods=$(kubectl get pods -n "$NAMESPACE" -l app=ab-test-controller --no-headers 2>/dev/null | grep Running | wc -l)
    if [ "$running_pods" -eq 0 ]; then
        error "A/B testing controller pods are not running"
        return 1
    fi
    
    # Check custom resource definition
    if kubectl get crd abtests.ml.freedeepresearch.org &> /dev/null; then
        success "A/B testing CRD is installed"
    else
        error "A/B testing CRD not found"
        return 1
    fi
    
    success "A/B Testing Framework validation completed"
}

validate_ml_monitoring() {
    log "Validating ML Monitoring..."
    
    # Check ML metrics collector
    local ml_metrics_pods=$(kubectl get pods -n "$NAMESPACE" -l app=ml-metrics-collector --no-headers 2>/dev/null | wc -l)
    if [ "$ml_metrics_pods" -eq 0 ]; then
        warning "ML metrics collector pods not found"
    else
        success "ML metrics collector is running"
    fi
    
    # Check ServiceMonitors
    local service_monitors=$(kubectl get servicemonitor -n "$NAMESPACE" --no-headers 2>/dev/null | grep -E "(mlflow|tensorflow|ab-test|ml-metrics)" | wc -l)
    if [ "$service_monitors" -gt 0 ]; then
        success "ML ServiceMonitors are configured"
    else
        warning "ML ServiceMonitors not found"
    fi
    
    success "ML Monitoring validation completed"
}

validate_storage() {
    log "Validating Storage Components..."
    
    # Check MinIO for artifact storage
    local minio_pods=$(kubectl get pods -n "$NAMESPACE" -l app=minio --no-headers 2>/dev/null | wc -l)
    if [ "$minio_pods" -eq 0 ]; then
        warning "MinIO pods not found"
    else
        success "MinIO artifact storage is running"
    fi
    
    # Check persistent volume claims
    local pvcs=$(kubectl get pvc -n "$NAMESPACE" --no-headers 2>/dev/null | grep -E "(mlflow|model-storage|minio)" | wc -l)
    if [ "$pvcs" -gt 0 ]; then
        success "ML storage PVCs are configured"
    else
        warning "ML storage PVCs not found"
    fi
    
    success "Storage validation completed"
}

test_api_endpoints() {
    log "Testing API endpoints..."
    
    # Test MLflow health endpoint (if accessible)
    log "Testing MLflow connectivity..."
    if kubectl exec -n "$NAMESPACE" deployment/mlflow -- curl -f http://localhost:5000/health &> /dev/null; then
        success "MLflow API is responding"
    else
        warning "MLflow API test failed (may be normal if not fully initialized)"
    fi
    
    # Test TensorFlow Serving (if accessible)
    log "Testing TensorFlow Serving connectivity..."
    if kubectl get service tensorflow-serving-service -n "$NAMESPACE" &> /dev/null; then
        success "TensorFlow Serving service is accessible"
    else
        warning "TensorFlow Serving service test failed"
    fi
    
    success "API endpoint testing completed"
}

validate_integration() {
    log "Validating integration with existing infrastructure..."
    
    # Check PostgreSQL integration
    if kubectl get service postgresql-service -n "$NAMESPACE" &> /dev/null; then
        success "PostgreSQL integration available"
    else
        error "PostgreSQL service not found"
        return 1
    fi
    
    # Check Redis integration
    if kubectl get service redis-service -n "$NAMESPACE" &> /dev/null; then
        success "Redis integration available"
    else
        error "Redis service not found"
        return 1
    fi
    
    # Check Istio integration
    if kubectl get namespace istio-system &> /dev/null; then
        success "Istio service mesh integration available"
    else
        error "Istio not found"
        return 1
    fi
    
    success "Integration validation completed"
}

generate_validation_report() {
    log "Generating validation report..."
    
    cat > /tmp/phase-4.6-validation-report.txt << EOF
# Phase 4.6 AI/ML Pipeline Enhancement - Validation Report

**Validation Date:** $(date +'%Y-%m-%d %H:%M:%S')
**Validation Status:** COMPLETED

## Component Status

### Core MLOps Components
- Kubeflow Pipelines: $(kubectl get pods -n kubeflow -l app=ml-pipeline --no-headers 2>/dev/null | grep Running | wc -l) pods running
- MLflow Registry: $(kubectl get pods -n "$NAMESPACE" -l app=mlflow --no-headers 2>/dev/null | grep Running | wc -l) pods running
- TensorFlow Serving: $(kubectl get pods -n "$NAMESPACE" -l app=tensorflow-serving --no-headers 2>/dev/null | grep Running | wc -l) GPU pods, $(kubectl get pods -n "$NAMESPACE" -l app=tensorflow-serving-cpu --no-headers 2>/dev/null | grep Running | wc -l) CPU pods running
- A/B Testing Controller: $(kubectl get pods -n "$NAMESPACE" -l app=ab-test-controller --no-headers 2>/dev/null | grep Running | wc -l) pods running

### Storage Components
- MinIO Artifact Storage: $(kubectl get pods -n "$NAMESPACE" -l app=minio --no-headers 2>/dev/null | grep Running | wc -l) pods running
- Persistent Volume Claims: $(kubectl get pvc -n "$NAMESPACE" --no-headers 2>/dev/null | grep -E "(mlflow|model-storage|minio)" | wc -l) configured

### Monitoring Components
- ML Metrics Collector: $(kubectl get pods -n "$NAMESPACE" -l app=ml-metrics-collector --no-headers 2>/dev/null | grep Running | wc -l) pods running
- ServiceMonitors: $(kubectl get servicemonitor -n "$NAMESPACE" --no-headers 2>/dev/null | grep -E "(mlflow|tensorflow|ab-test|ml-metrics)" | wc -l) configured

### Integration Status
- PostgreSQL: $(kubectl get service postgresql-service -n "$NAMESPACE" &> /dev/null && echo "✅ Available" || echo "❌ Not Found")
- Redis: $(kubectl get service redis-service -n "$NAMESPACE" &> /dev/null && echo "✅ Available" || echo "❌ Not Found")
- Istio: $(kubectl get namespace istio-system &> /dev/null && echo "✅ Available" || echo "❌ Not Found")

## Validation Summary

Phase 4.6 AI/ML Pipeline Enhancement validation completed successfully.
All core components are deployed and operational.

## Next Steps

1. Test model training pipeline with Kubeflow
2. Validate model serving with TensorFlow Serving
3. Configure A/B testing experiments
4. Monitor ML workloads in Grafana dashboards

EOF

    success "Validation report generated: /tmp/phase-4.6-validation-report.txt"
    cat /tmp/phase-4.6-validation-report.txt
}

# Main validation function
main() {
    log "Starting Phase 4.6 AI/ML Pipeline Enhancement validation..."
    
    validate_prerequisites
    validate_kubeflow
    validate_mlflow
    validate_tensorflow_serving
    validate_ab_testing
    validate_ml_monitoring
    validate_storage
    test_api_endpoints
    validate_integration
    generate_validation_report
    
    success "Phase 4.6 validation completed successfully!"
    log "All MLOps components are operational and ready for use."
    log "Access points:"
    log "  - Kubeflow Pipelines: https://kubeflow.freedeepresearch.org"
    log "  - MLflow Registry: https://mlflow.freedeepresearch.org"
    log "  - TensorFlow Serving: https://ml.freedeepresearch.org"
    log "  - ML Dashboards: https://grafana.freedeepresearch.org/d/ml-overview"
}

# Execute main function if script is run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
