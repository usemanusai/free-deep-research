use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::Utc;

use crate::error::{AppResult, ResearchError};
use crate::services::{Service, DataPersistenceService, SecurityService};
use crate::models::quantum_ready::*;

pub mod algorithm_manager;
pub mod compute_resource_manager;
pub mod security_protocol_manager;
pub mod quantum_crypto;
pub mod migration_planner;
pub mod readiness_assessor;
pub mod hybrid_operations;

use algorithm_manager::AlgorithmManager;
use compute_resource_manager::ComputeResourceManager;
use security_protocol_manager::SecurityProtocolManager;
use quantum_crypto::QuantumCryptoManager;
use migration_planner::MigrationPlanner;
use readiness_assessor::ReadinessAssessor;
use hybrid_operations::HybridOperationsManager;

/// Quantum-Ready Architecture Service for post-quantum cryptography and quantum computing integration
pub struct QuantumReadyService {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    security: Arc<RwLock<SecurityService>>,
    algorithm_manager: Arc<RwLock<AlgorithmManager>>,
    compute_resource_manager: Arc<RwLock<ComputeResourceManager>>,
    security_protocol_manager: Arc<RwLock<SecurityProtocolManager>>,
    quantum_crypto: Arc<RwLock<QuantumCryptoManager>>,
    migration_planner: Arc<RwLock<MigrationPlanner>>,
    readiness_assessor: Arc<RwLock<ReadinessAssessor>>,
    hybrid_operations: Arc<RwLock<HybridOperationsManager>>,
}

impl QuantumReadyService {
    /// Create a new quantum-ready service
    pub async fn new(
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        security: Arc<RwLock<SecurityService>>,
    ) -> AppResult<Self> {
        info!("Initializing Quantum-Ready Architecture Service");

        let algorithm_manager = Arc::new(RwLock::new(
            AlgorithmManager::new(data_persistence.clone()).await?
        ));

        let compute_resource_manager = Arc::new(RwLock::new(
            ComputeResourceManager::new(data_persistence.clone()).await?
        ));

        let security_protocol_manager = Arc::new(RwLock::new(
            SecurityProtocolManager::new(
                data_persistence.clone(),
                algorithm_manager.clone(),
            ).await?
        ));

        let quantum_crypto = Arc::new(RwLock::new(
            QuantumCryptoManager::new(
                security.clone(),
                algorithm_manager.clone(),
            ).await?
        ));

        let migration_planner = Arc::new(RwLock::new(
            MigrationPlanner::new(
                data_persistence.clone(),
                security_protocol_manager.clone(),
            ).await?
        ));

        let readiness_assessor = Arc::new(RwLock::new(
            ReadinessAssessor::new(
                data_persistence.clone(),
                algorithm_manager.clone(),
                security_protocol_manager.clone(),
            ).await?
        ));

        let hybrid_operations = Arc::new(RwLock::new(
            HybridOperationsManager::new(
                quantum_crypto.clone(),
                compute_resource_manager.clone(),
            ).await?
        ));

        Ok(Self {
            data_persistence,
            security,
            algorithm_manager,
            compute_resource_manager,
            security_protocol_manager,
            quantum_crypto,
            migration_planner,
            readiness_assessor,
            hybrid_operations,
        })
    }

    /// Register a quantum algorithm
    pub async fn register_algorithm(
        &self,
        algorithm: QuantumAlgorithm,
    ) -> AppResult<QuantumAlgorithm> {
        info!("Registering quantum algorithm: {}", algorithm.name);

        let algorithm_manager = self.algorithm_manager.write().await;
        let registered_algorithm = algorithm_manager.register_algorithm(algorithm).await?;
        
        info!("Successfully registered algorithm: {}", registered_algorithm.id);
        Ok(registered_algorithm)
    }

    /// Register a compute resource
    pub async fn register_compute_resource(
        &self,
        resource: ComputeResource,
    ) -> AppResult<ComputeResource> {
        info!("Registering compute resource: {} ({})", resource.provider, resource.resource_type);

        let compute_resource_manager = self.compute_resource_manager.write().await;
        let registered_resource = compute_resource_manager.register_resource(resource).await?;
        
        info!("Successfully registered compute resource: {}", registered_resource.id);
        Ok(registered_resource)
    }

    /// Create a security protocol
    pub async fn create_security_protocol(
        &self,
        protocol: SecurityProtocol,
    ) -> AppResult<SecurityProtocol> {
        info!("Creating security protocol: {} v{}", protocol.protocol_name, protocol.protocol_version);

        let security_protocol_manager = self.security_protocol_manager.write().await;
        let created_protocol = security_protocol_manager.create_protocol(protocol).await?;
        
        info!("Successfully created security protocol: {}", created_protocol.id);
        Ok(created_protocol)
    }

    /// Perform quantum readiness assessment
    pub async fn assess_quantum_readiness(
        &self,
        system_component: String,
    ) -> AppResult<QuantumReadinessAssessment> {
        info!("Performing quantum readiness assessment for: {}", system_component);

        let readiness_assessor = self.readiness_assessor.read().await;
        let assessment = readiness_assessor.assess_component(&system_component).await?;
        
        info!("Quantum readiness assessment completed with vulnerability level: {:?}", 
               assessment.quantum_vulnerability);
        Ok(assessment)
    }

    /// Plan migration to quantum-safe algorithms
    pub async fn plan_quantum_migration(
        &self,
        current_protocols: Vec<String>,
    ) -> AppResult<Vec<MigrationPath>> {
        info!("Planning quantum migration for {} protocols", current_protocols.len());

        let migration_planner = self.migration_planner.read().await;
        let migration_paths = migration_planner.plan_migration(current_protocols).await?;
        
        info!("Generated {} migration paths", migration_paths.len());
        Ok(migration_paths)
    }

    /// Execute hybrid cryptographic operation
    pub async fn execute_hybrid_crypto_operation(
        &self,
        operation_type: CryptoOperationType,
        data: Vec<u8>,
        classical_algorithm: String,
        quantum_safe_algorithm: String,
    ) -> AppResult<HybridCryptoOperation> {
        debug!("Executing hybrid crypto operation: {:?}", operation_type);

        let hybrid_operations = self.hybrid_operations.read().await;
        let operation_result = hybrid_operations
            .execute_operation(operation_type, data, classical_algorithm, quantum_safe_algorithm)
            .await?;
        
        debug!("Hybrid crypto operation completed in {}ms", operation_result.processing_time_ms);
        Ok(operation_result)
    }

    /// Start QKD session
    pub async fn start_qkd_session(
        &self,
        alice_node: String,
        bob_node: String,
        protocol_type: QKDProtocol,
        key_length_bits: u32,
    ) -> AppResult<QKDSession> {
        info!("Starting QKD session between {} and {} using {:?}", 
               alice_node, bob_node, protocol_type);

        let quantum_crypto = self.quantum_crypto.write().await;
        let qkd_session = quantum_crypto
            .start_qkd_session(alice_node, bob_node, protocol_type, key_length_bits)
            .await?;
        
        info!("QKD session started: {}", qkd_session.id);
        Ok(qkd_session)
    }

    /// Get available quantum algorithms
    pub async fn get_available_algorithms(
        &self,
        algorithm_type: Option<AlgorithmType>,
    ) -> AppResult<Vec<QuantumAlgorithm>> {
        debug!("Getting available quantum algorithms");

        let algorithm_manager = self.algorithm_manager.read().await;
        let algorithms = algorithm_manager.get_algorithms(algorithm_type).await?;
        
        Ok(algorithms)
    }

    /// Get available compute resources
    pub async fn get_available_compute_resources(
        &self,
        resource_type: Option<ResourceType>,
    ) -> AppResult<Vec<ComputeResource>> {
        debug!("Getting available compute resources");

        let compute_resource_manager = self.compute_resource_manager.read().await;
        let resources = compute_resource_manager.get_resources(resource_type).await?;
        
        Ok(resources)
    }

    /// Update algorithm performance metrics
    pub async fn update_algorithm_performance(
        &self,
        algorithm_id: Uuid,
        metrics: PerformanceMetrics,
    ) -> AppResult<()> {
        debug!("Updating performance metrics for algorithm: {}", algorithm_id);

        let algorithm_manager = self.algorithm_manager.write().await;
        algorithm_manager.update_performance_metrics(algorithm_id, metrics).await?;
        
        Ok(())
    }

    /// Check compute resource health
    pub async fn check_resource_health(
        &self,
        resource_id: Uuid,
    ) -> AppResult<AvailabilityStatus> {
        debug!("Checking health for compute resource: {}", resource_id);

        let compute_resource_manager = self.compute_resource_manager.read().await;
        let health_status = compute_resource_manager.check_resource_health(resource_id).await?;
        
        Ok(health_status)
    }

    /// Migrate security protocol
    pub async fn migrate_security_protocol(
        &self,
        protocol_id: Uuid,
        migration_path: MigrationPath,
    ) -> AppResult<SecurityProtocol> {
        info!("Migrating security protocol: {}", protocol_id);

        let migration_planner = self.migration_planner.write().await;
        let migrated_protocol = migration_planner
            .execute_migration(protocol_id, migration_path)
            .await?;
        
        info!("Security protocol migration completed: {}", migrated_protocol.id);
        Ok(migrated_protocol)
    }

    /// Get quantum readiness summary
    pub async fn get_readiness_summary(&self) -> AppResult<QuantumReadinessSummary> {
        debug!("Getting quantum readiness summary");

        let readiness_assessor = self.readiness_assessor.read().await;
        let summary = readiness_assessor.get_system_summary().await?;
        
        Ok(summary)
    }

    /// Benchmark algorithm performance
    pub async fn benchmark_algorithm(
        &self,
        algorithm_id: Uuid,
        test_data_size: u32,
        iterations: u32,
    ) -> AppResult<PerformanceMetrics> {
        info!("Benchmarking algorithm: {} with {} iterations", algorithm_id, iterations);

        let algorithm_manager = self.algorithm_manager.read().await;
        let benchmark_results = algorithm_manager
            .benchmark_algorithm(algorithm_id, test_data_size, iterations)
            .await?;
        
        info!("Algorithm benchmark completed");
        Ok(benchmark_results)
    }

    /// Start background tasks
    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting quantum-ready architecture background tasks...");

        // Start resource health monitoring
        let compute_resource_manager = self.compute_resource_manager.read().await;
        compute_resource_manager.start_health_monitoring().await?;

        // Start algorithm performance monitoring
        let algorithm_manager = self.algorithm_manager.read().await;
        algorithm_manager.start_performance_monitoring().await?;

        // Start security protocol monitoring
        let security_protocol_manager = self.security_protocol_manager.read().await;
        security_protocol_manager.start_protocol_monitoring().await?;

        info!("Quantum-ready architecture background tasks started successfully");
        Ok(())
    }
}

/// Quantum readiness summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuantumReadinessSummary {
    pub total_components_assessed: u32,
    pub quantum_safe_components: u32,
    pub vulnerable_components: u32,
    pub migration_in_progress: u32,
    pub overall_readiness_score: f64,
    pub critical_vulnerabilities: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub estimated_migration_time_hours: f64,
    pub last_assessment: chrono::DateTime<Utc>,
}

#[async_trait::async_trait]
impl Service for QuantumReadyService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing quantum-ready service health check");
        
        // Check all sub-managers
        let algorithm_manager = self.algorithm_manager.read().await;
        algorithm_manager.health_check().await?;

        let compute_resource_manager = self.compute_resource_manager.read().await;
        compute_resource_manager.health_check().await?;

        let security_protocol_manager = self.security_protocol_manager.read().await;
        security_protocol_manager.health_check().await?;

        let quantum_crypto = self.quantum_crypto.read().await;
        quantum_crypto.health_check().await?;

        Ok(())
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down quantum-ready service...");
        
        // Shutdown all sub-managers
        let algorithm_manager = self.algorithm_manager.read().await;
        algorithm_manager.shutdown().await?;

        let compute_resource_manager = self.compute_resource_manager.read().await;
        compute_resource_manager.shutdown().await?;

        let security_protocol_manager = self.security_protocol_manager.read().await;
        security_protocol_manager.shutdown().await?;

        let quantum_crypto = self.quantum_crypto.read().await;
        quantum_crypto.shutdown().await?;

        let migration_planner = self.migration_planner.read().await;
        migration_planner.shutdown().await?;

        let readiness_assessor = self.readiness_assessor.read().await;
        readiness_assessor.shutdown().await?;

        let hybrid_operations = self.hybrid_operations.read().await;
        hybrid_operations.shutdown().await?;

        info!("Quantum-ready service shutdown complete");
        Ok(())
    }
}
