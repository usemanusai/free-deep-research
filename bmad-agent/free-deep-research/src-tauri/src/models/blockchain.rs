use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Blockchain transaction model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainTransaction {
    pub id: Uuid,
    pub transaction_hash: String,
    pub block_number: Option<u64>,
    pub transaction_type: TransactionType,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub data_payload: TransactionData,
    pub gas_used: Option<u64>,
    pub transaction_fee: Option<f64>,
    pub status: TransactionStatus,
    pub confirmations: u32,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

/// Transaction type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Validation,
    Reward,
    Governance,
    DataStorage,
    PeerReview,
    ResearchPublication,
    Attribution,
}

/// Transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub data_type: String,
    pub payload: HashMap<String, serde_json::Value>,
    pub metadata: TransactionMetadata,
    pub signatures: Vec<DigitalSignature>,
}

/// Transaction metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMetadata {
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub encoding: String,
    pub compression: Option<String>,
    pub checksum: String,
    pub size_bytes: u32,
}

/// Digital signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignature {
    pub signer_address: String,
    pub signature: String,
    pub algorithm: String,
    pub public_key: String,
    pub timestamp: DateTime<Utc>,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Rejected,
    Cancelled,
}

/// Peer review model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReview {
    pub id: Uuid,
    pub research_workflow_id: Uuid,
    pub reviewer_id: Uuid,
    pub review_type: ReviewType,
    pub review_score: u8, // 1-10 scale
    pub review_comments: Option<String>,
    pub review_criteria: ReviewCriteria,
    pub blockchain_transaction_id: Option<Uuid>,
    pub review_status: ReviewStatus,
    pub created_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
}

/// Review type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewType {
    Methodology,
    Results,
    Quality,
    Reproducibility,
    Ethics,
    Statistical,
    Comprehensive,
}

/// Review criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewCriteria {
    pub methodology_score: Option<u8>,
    pub data_quality_score: Option<u8>,
    pub analysis_rigor_score: Option<u8>,
    pub conclusion_validity_score: Option<u8>,
    pub reproducibility_score: Option<u8>,
    pub ethical_compliance_score: Option<u8>,
    pub presentation_quality_score: Option<u8>,
    pub overall_contribution_score: Option<u8>,
    pub detailed_feedback: HashMap<String, String>,
}

/// Review status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Draft,
    Submitted,
    Validated,
    Disputed,
    Accepted,
    Rejected,
}

/// Research validation model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchValidation {
    pub id: Uuid,
    pub research_workflow_id: Uuid,
    pub validation_type: ValidationType,
    pub validation_score: f64,
    pub validation_criteria: ValidationCriteria,
    pub validator_nodes: Vec<ValidatorNode>,
    pub consensus_reached: bool,
    pub blockchain_record_hash: Option<String>,
    pub validation_status: ValidationStatus,
    pub created_at: DateTime<Utc>,
    pub validated_at: Option<DateTime<Utc>>,
}

/// Validation type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationType {
    PeerReview,
    Automated,
    Consensus,
    Statistical,
    Reproducibility,
    Hybrid,
}

/// Validation criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCriteria {
    pub minimum_reviewers: u32,
    pub consensus_threshold: f64,
    pub quality_threshold: f64,
    pub required_expertise_areas: Vec<String>,
    pub validation_rules: Vec<ValidationRule>,
    pub time_limit_hours: Option<u32>,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_id: String,
    pub rule_type: ValidationRuleType,
    pub condition: String,
    pub weight: f64,
    pub mandatory: bool,
}

/// Validation rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationRuleType {
    DataQuality,
    MethodologyCompliance,
    StatisticalSignificance,
    EthicalCompliance,
    Reproducibility,
    CitationAccuracy,
    Custom,
}

/// Validator node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorNode {
    pub node_id: String,
    pub validator_address: String,
    pub reputation_score: f64,
    pub expertise_areas: Vec<String>,
    pub validation_count: u32,
    pub success_rate: f64,
    pub stake_amount: Option<f64>,
    pub last_active: DateTime<Utc>,
}

/// Validation status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Disputed,
    Expired,
}

/// Token reward model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenReward {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reward_type: RewardType,
    pub reward_amount: f64,
    pub research_workflow_id: Option<Uuid>,
    pub blockchain_transaction_id: Option<Uuid>,
    pub reward_criteria: RewardCriteria,
    pub status: RewardStatus,
    pub created_at: DateTime<Utc>,
    pub distributed_at: Option<DateTime<Utc>>,
}

/// Reward type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewardType {
    ResearchContribution,
    PeerReview,
    Validation,
    DataSharing,
    MethodologyContribution,
    QualityAssurance,
    CommunityModeration,
    BugReport,
    FeatureContribution,
}

/// Reward criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCriteria {
    pub base_reward: f64,
    pub quality_multiplier: f64,
    pub impact_multiplier: f64,
    pub timeliness_multiplier: f64,
    pub complexity_multiplier: f64,
    pub criteria_met: Vec<String>,
    pub bonus_conditions: HashMap<String, f64>,
}

/// Reward status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewardStatus {
    Pending,
    Approved,
    Distributed,
    Failed,
    Disputed,
    Cancelled,
}

/// Blockchain block model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_number: u64,
    pub block_hash: String,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Uuid>,
    pub validator: String,
    pub difficulty: u64,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub block_size_bytes: u32,
    pub transaction_count: u32,
}

/// Smart contract model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContract {
    pub id: Uuid,
    pub contract_address: String,
    pub contract_name: String,
    pub contract_type: ContractType,
    pub bytecode: String,
    pub abi: String, // JSON ABI
    pub source_code: Option<String>,
    pub compiler_version: String,
    pub deployment_transaction: String,
    pub creator_address: String,
    pub status: ContractStatus,
    pub created_at: DateTime<Utc>,
    pub last_interaction: Option<DateTime<Utc>>,
}

/// Contract type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractType {
    ResearchValidation,
    PeerReview,
    TokenReward,
    DataStorage,
    Governance,
    Attribution,
    Reputation,
}

/// Contract status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatus {
    Active,
    Paused,
    Deprecated,
    Upgraded,
    Destroyed,
}

/// Consensus mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMechanism {
    pub mechanism_type: ConsensusType,
    pub parameters: ConsensusParameters,
    pub current_validators: Vec<ValidatorNode>,
    pub consensus_threshold: f64,
    pub block_time_seconds: u32,
    pub finality_blocks: u32,
}

/// Consensus type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsensusType {
    ProofOfStake,
    ProofOfAuthority,
    ProofOfWork,
    DelegatedProofOfStake,
    PracticalByzantineFaultTolerance,
    Hybrid,
}

/// Consensus parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusParameters {
    pub minimum_stake: Option<f64>,
    pub validator_count: u32,
    pub slashing_conditions: Vec<String>,
    pub reward_distribution: HashMap<String, f64>,
    pub governance_rules: GovernanceRules,
}

/// Governance rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRules {
    pub proposal_threshold: f64,
    pub voting_period_hours: u32,
    pub quorum_requirement: f64,
    pub execution_delay_hours: u32,
    pub veto_power: Vec<String>,
}

/// Blockchain network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatistics {
    pub total_blocks: u64,
    pub total_transactions: u64,
    pub active_validators: u32,
    pub network_hash_rate: Option<f64>,
    pub average_block_time_seconds: f64,
    pub pending_transactions: u32,
    pub total_value_locked: f64,
    pub network_fees_24h: f64,
    pub last_updated: DateTime<Utc>,
}

/// Audit trail entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrailEntry {
    pub id: Uuid,
    pub transaction_hash: String,
    pub action_type: AuditActionType,
    pub actor_address: String,
    pub target_resource: String,
    pub action_details: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub block_number: u64,
    pub gas_cost: Option<u64>,
}

/// Audit action type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditActionType {
    ResearchSubmission,
    PeerReviewSubmission,
    ValidationComplete,
    RewardDistribution,
    DataAccess,
    PermissionChange,
    ContractDeployment,
    GovernanceVote,
}
