use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Federated organization model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedOrganization {
    pub id: Uuid,
    pub name: String,
    pub domain: String,
    pub public_key: String,
    pub trust_level: u8, // 0-100 trust score
    pub api_endpoint: String,
    pub contact_email: Option<String>,
    pub description: Option<String>,
    pub status: OrganizationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Organization status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationStatus {
    Pending,
    Active,
    Suspended,
    Revoked,
}

/// Research partnership model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPartnership {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub partner_organization_id: Uuid,
    pub partnership_type: PartnershipType,
    pub data_sharing_level: DataSharingLevel,
    pub permissions: Vec<String>,
    pub status: PartnershipStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Partnership type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnershipType {
    Bilateral,
    Multilateral,
    Observer,
}

/// Data sharing level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataSharingLevel {
    Public,
    Restricted,
    Private,
}

/// Partnership status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnershipStatus {
    Pending,
    Active,
    Suspended,
    Expired,
}

/// Shared research session model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResearchSession {
    pub id: Uuid,
    pub local_workflow_id: Uuid,
    pub sharing_organization_id: Uuid,
    pub shared_data: String, // Encrypted JSON data
    pub access_permissions: Vec<String>,
    pub sharing_level: SharingLevel,
    pub expiration_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Sharing level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharingLevel {
    MetadataOnly,
    Partial,
    Full,
}

/// Request to create a federated organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFederatedOrganizationRequest {
    pub name: String,
    pub domain: String,
    pub public_key: String,
    pub api_endpoint: String,
    pub contact_email: Option<String>,
    pub description: Option<String>,
}

/// Request to create a research partnership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResearchPartnershipRequest {
    pub partner_organization_id: Uuid,
    pub partnership_type: PartnershipType,
    pub data_sharing_level: DataSharingLevel,
    pub permissions: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request to share research session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareResearchSessionRequest {
    pub workflow_id: Uuid,
    pub sharing_organization_id: Uuid,
    pub sharing_level: SharingLevel,
    pub access_permissions: Vec<String>,
    pub expiration_date: Option<DateTime<Utc>>,
}

/// Federated research query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedResearchQuery {
    pub id: Uuid,
    pub query: String,
    pub requesting_organization_id: Uuid,
    pub target_organizations: Vec<Uuid>,
    pub query_parameters: HashMap<String, serde_json::Value>,
    pub privacy_level: DataSharingLevel,
    pub max_response_time_seconds: u32,
    pub created_at: DateTime<Utc>,
}

/// Federated research response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedResearchResponse {
    pub id: Uuid,
    pub query_id: Uuid,
    pub responding_organization_id: Uuid,
    pub response_data: HashMap<String, serde_json::Value>,
    pub confidence_score: f64,
    pub processing_time_ms: u32,
    pub status: ResponseStatus,
    pub created_at: DateTime<Utc>,
}

/// Response status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Pending,
    Completed,
    Failed,
    Timeout,
    AccessDenied,
}

/// Federated authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedAuthToken {
    pub token: String,
    pub organization_id: Uuid,
    pub permissions: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
}

/// Cross-organization research collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossOrgCollaboration {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub lead_organization_id: Uuid,
    pub participating_organizations: Vec<Uuid>,
    pub research_objectives: Vec<String>,
    pub data_sharing_agreements: HashMap<Uuid, DataSharingLevel>,
    pub status: CollaborationStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Collaboration status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationStatus {
    Planning,
    Active,
    Paused,
    Completed,
    Cancelled,
}

/// Privacy control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyControls {
    pub organization_id: Uuid,
    pub default_sharing_level: DataSharingLevel,
    pub allowed_data_types: Vec<String>,
    pub restricted_data_types: Vec<String>,
    pub auto_approve_trusted_orgs: bool,
    pub require_manual_approval: bool,
    pub data_retention_days: u32,
    pub anonymization_required: bool,
    pub encryption_required: bool,
}

/// Federated research metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedResearchMetrics {
    pub organization_id: Uuid,
    pub total_queries_sent: u32,
    pub total_queries_received: u32,
    pub successful_collaborations: u32,
    pub average_response_time_ms: f64,
    pub trust_score_changes: Vec<TrustScoreChange>,
    pub data_shared_mb: f64,
    pub data_received_mb: f64,
    pub last_activity: DateTime<Utc>,
}

/// Trust score change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScoreChange {
    pub timestamp: DateTime<Utc>,
    pub old_score: u8,
    pub new_score: u8,
    pub reason: String,
    pub changed_by: Option<Uuid>,
}
