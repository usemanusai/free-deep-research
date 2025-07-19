use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Quantum algorithm model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumAlgorithm {
    pub id: Uuid,
    pub name: String,
    pub algorithm_type: AlgorithmType,
    pub classical_equivalent: Option<String>,
    pub quantum_safe: bool,
    pub implementation_status: ImplementationStatus,
    pub performance_metrics: PerformanceMetrics,
    pub security_level: u8, // NIST security levels 1-5
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Algorithm type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlgorithmType {
    Encryption,
    Signature,
    KeyExchange,
    HashFunction,
    RandomNumberGeneration,
    QuantumKeyDistribution,
}

/// Implementation status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImplementationStatus {
    Experimental,
    Stable,
    Deprecated,
    UnderDevelopment,
    Standardized,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub key_generation_time_ms: Option<f64>,
    pub encryption_time_ms: Option<f64>,
    pub decryption_time_ms: Option<f64>,
    pub signature_time_ms: Option<f64>,
    pub verification_time_ms: Option<f64>,
    pub key_size_bytes: Option<u32>,
    pub signature_size_bytes: Option<u32>,
    pub ciphertext_expansion_factor: Option<f64>,
    pub memory_usage_mb: Option<f64>,
    pub cpu_cycles: Option<u64>,
}

/// Compute resource model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResource {
    pub id: Uuid,
    pub resource_type: ResourceType,
    pub provider: String,
    pub endpoint_url: Option<String>,
    pub capabilities: ResourceCapabilities,
    pub availability_status: AvailabilityStatus,
    pub cost_per_operation: Option<f64>,
    pub quantum_specs: Option<QuantumSpecs>,
    pub classical_specs: Option<ClassicalSpecs>,
    pub created_at: DateTime<Utc>,
    pub last_health_check: DateTime<Utc>,
}

/// Resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Classical,
    Quantum,
    Hybrid,
    Simulator,
}

/// Resource capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCapabilities {
    pub supported_algorithms: Vec<String>,
    pub max_parallel_operations: u32,
    pub supported_languages: Vec<String>,
    pub api_versions: Vec<String>,
    pub special_features: Vec<String>,
}

/// Availability status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityStatus {
    Available,
    Busy,
    Maintenance,
    Offline,
    Limited,
}

/// Quantum specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSpecs {
    pub max_qubits: u32,
    pub coherence_time_ms: f64,
    pub gate_error_rate: f64,
    pub readout_error_rate: f64,
    pub connectivity_graph: String, // JSON representation
    pub supported_gates: Vec<String>,
    pub quantum_volume: Option<u32>,
}

/// Classical specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicalSpecs {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_gb: u32,
    pub gpu_count: u32,
    pub network_bandwidth_gbps: f64,
    pub architecture: String,
}

/// Security protocol model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProtocol {
    pub id: Uuid,
    pub protocol_name: String,
    pub protocol_version: String,
    pub quantum_safe: bool,
    pub classical_fallback: bool,
    pub implementation_config: ProtocolConfig,
    pub migration_path: MigrationPath,
    pub status: ProtocolStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    pub encryption_algorithm: String,
    pub key_size: u32,
    pub signature_algorithm: Option<String>,
    pub hash_algorithm: String,
    pub key_exchange_method: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub compatibility_mode: bool,
}

/// Migration path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPath {
    pub from_protocol: String,
    pub migration_steps: Vec<MigrationStep>,
    pub estimated_duration_hours: f64,
    pub rollback_possible: bool,
    pub data_backup_required: bool,
    pub downtime_required: bool,
}

/// Migration step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub step_number: u32,
    pub description: String,
    pub action_type: MigrationActionType,
    pub estimated_duration_minutes: u32,
    pub prerequisites: Vec<String>,
    pub validation_criteria: Vec<String>,
    pub rollback_instructions: Option<String>,
}

/// Migration action type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationActionType {
    BackupData,
    UpdateConfiguration,
    RegenerateKeys,
    TestConnectivity,
    ValidateIntegrity,
    SwitchProtocol,
    VerifyOperation,
}

/// Protocol status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolStatus {
    Active,
    Deprecated,
    Experimental,
    Migrating,
    Failed,
}

/// Quantum readiness assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumReadinessAssessment {
    pub id: Uuid,
    pub system_component: String,
    pub current_algorithms: Vec<String>,
    pub quantum_vulnerability: VulnerabilityLevel,
    pub recommended_upgrades: Vec<UpgradeRecommendation>,
    pub migration_priority: Priority,
    pub estimated_migration_cost: Option<f64>,
    pub compliance_requirements: Vec<String>,
    pub assessment_date: DateTime<Utc>,
    pub next_review_date: DateTime<Utc>,
}

/// Vulnerability level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VulnerabilityLevel {
    Low,
    Medium,
    High,
    Critical,
    QuantumSafe,
}

/// Upgrade recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeRecommendation {
    pub component: String,
    pub current_algorithm: String,
    pub recommended_algorithm: String,
    pub urgency: Priority,
    pub estimated_effort_hours: f64,
    pub dependencies: Vec<String>,
    pub benefits: Vec<String>,
    pub risks: Vec<String>,
}

/// Priority level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
    Immediate,
}

/// Quantum key distribution session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QKDSession {
    pub id: Uuid,
    pub alice_node: String,
    pub bob_node: String,
    pub protocol_type: QKDProtocol,
    pub key_length_bits: u32,
    pub error_rate: f64,
    pub key_generation_rate_bps: f64,
    pub session_status: QKDStatus,
    pub security_parameters: QKDSecurityParams,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// QKD protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QKDProtocol {
    Bb84,
    Sarg04,
    DecoyState,
    ContinuousVariable,
}

/// QKD status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QKDStatus {
    Initializing,
    KeyGeneration,
    ErrorCorrection,
    PrivacyAmplification,
    Completed,
    Failed,
    Aborted,
}

/// QKD security parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QKDSecurityParams {
    pub quantum_bit_error_rate: f64,
    pub privacy_amplification_ratio: f64,
    pub error_correction_efficiency: f64,
    pub security_parameter: f64,
    pub final_key_rate_bps: f64,
}

/// Hybrid cryptographic operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridCryptoOperation {
    pub id: Uuid,
    pub operation_type: CryptoOperationType,
    pub classical_algorithm: String,
    pub quantum_safe_algorithm: String,
    pub input_size_bytes: u32,
    pub output_size_bytes: u32,
    pub processing_time_ms: f64,
    pub security_level: u8,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Crypto operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CryptoOperationType {
    Encrypt,
    Decrypt,
    Sign,
    Verify,
    KeyGeneration,
    KeyExchange,
}
