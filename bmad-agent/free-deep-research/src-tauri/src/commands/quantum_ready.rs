use tauri::State;
use uuid::Uuid;
use tracing::{info, debug, error};

use crate::services::ServiceManager;
use crate::models::quantum_ready::*;

#[tauri::command]
pub async fn register_quantum_algorithm(
    service_manager: State<'_, ServiceManager>,
    algorithm: QuantumAlgorithm,
) -> Result<QuantumAlgorithm, String> {
    info!("API: Registering quantum algorithm: {}", algorithm.name);
    match service_manager.quantum_ready_service.register_algorithm(algorithm).await {
        Ok(registered_algorithm) => Ok(registered_algorithm),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn register_compute_resource(
    service_manager: State<'_, ServiceManager>,
    resource: ComputeResource,
) -> Result<ComputeResource, String> {
    info!("API: Registering compute resource: {}", resource.provider);
    match service_manager.quantum_ready_service.register_compute_resource(resource).await {
        Ok(registered_resource) => Ok(registered_resource),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn assess_quantum_readiness(
    service_manager: State<'_, ServiceManager>,
    system_component: String,
) -> Result<QuantumReadinessAssessment, String> {
    info!("API: Assessing quantum readiness for: {}", system_component);
    match service_manager.quantum_ready_service.assess_quantum_readiness(system_component).await {
        Ok(assessment) => Ok(assessment),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn plan_quantum_migration(
    service_manager: State<'_, ServiceManager>,
    current_protocols: Vec<String>,
) -> Result<Vec<MigrationPath>, String> {
    info!("API: Planning quantum migration for {} protocols", current_protocols.len());
    match service_manager.quantum_ready_service.plan_quantum_migration(current_protocols).await {
        Ok(migration_paths) => Ok(migration_paths),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn execute_hybrid_crypto_operation(
    service_manager: State<'_, ServiceManager>,
    operation_type: CryptoOperationType,
    data: Vec<u8>,
    classical_algorithm: String,
    quantum_safe_algorithm: String,
) -> Result<HybridCryptoOperation, String> {
    debug!("API: Executing hybrid crypto operation: {:?}", operation_type);
    match service_manager.quantum_ready_service
        .execute_hybrid_crypto_operation(operation_type, data, classical_algorithm, quantum_safe_algorithm).await {
        Ok(operation_result) => Ok(operation_result),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_available_quantum_algorithms(
    service_manager: State<'_, ServiceManager>,
    algorithm_type: Option<AlgorithmType>,
) -> Result<Vec<QuantumAlgorithm>, String> {
    debug!("API: Getting available quantum algorithms");
    match service_manager.quantum_ready_service.get_available_algorithms(algorithm_type).await {
        Ok(algorithms) => Ok(algorithms),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_quantum_readiness_summary(
    service_manager: State<'_, ServiceManager>,
) -> Result<crate::services::quantum_ready::QuantumReadinessSummary, String> {
    debug!("API: Getting quantum readiness summary");
    match service_manager.quantum_ready_service.get_readiness_summary().await {
        Ok(summary) => Ok(summary),
        Err(e) => Err(e.to_string())
    }
}
