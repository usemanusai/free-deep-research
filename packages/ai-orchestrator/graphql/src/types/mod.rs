// GraphQL Types for Free Deep Research System
// Phase 4.4: API Gateway & GraphQL

use async_graphql::{SimpleObject, InputObject, Enum, Union, ID, scalar};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// Scalar types
scalar!(Uuid);
scalar!(DateTime<Utc>, "DateTime");

pub type JSON = serde_json::Value;

// User types
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub role: UserRole,
    pub permissions: Vec<Permission>,
    pub preferences: UserPreferences,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Researcher,
    Collaborator,
    Viewer,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub actions: Vec<String>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub timezone: String,
    pub notifications: NotificationSettings,
    pub dashboard_layout: JSON,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub workflow_updates: bool,
    pub system_alerts: bool,
    pub collaboration_invites: bool,
}

// API Key types
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub service: ServiceProvider,
    pub status: ApiKeyStatus,
    pub rate_limit: i32,
    pub current_usage: i32,
    pub usage_percentage: f64,
    pub last_used: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceProvider {
    OpenRouter,
    SerpApi,
    Jina,
    Firecrawl,
    Tavily,
    Exa,
    Custom,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiKeyStatus {
    Active,
    Inactive,
    RateLimited,
    Error,
    Expired,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct UsageStats {
    pub total_requests: i32,
    pub successful_requests: i32,
    pub failed_requests: i32,
    pub average_response_time: f64,
    pub peak_usage_hour: Option<i32>,
    pub daily_usage: Vec<DailyUsage>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct DailyUsage {
    pub date: String,
    pub requests: i32,
    pub errors: i32,
    pub average_response_time: f64,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyTestResult {
    pub success: bool,
    pub response_time: i32,
    pub error: Option<String>,
    pub metadata: Option<JSON>,
}

// Research Workflow types
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct ResearchWorkflow {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub methodology: ResearchMethodology,
    pub status: WorkflowStatus,
    pub progress: f64,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub collaborators: Vec<Uuid>,
    pub configuration: WorkflowConfiguration,
    pub output_formats: Vec<OutputFormat>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Draft,
    Ready,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchMethodology {
    DonLim,
    NickScamara,
    Hybrid,
    Custom,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowConfiguration {
    pub max_depth: i32,
    pub max_sources: i32,
    pub quality_threshold: f64,
    pub enable_fact_checking: bool,
    pub output_language: String,
    pub custom_parameters: JSON,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    Pdf,
    Html,
    Json,
    Docx,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub executor_id: Uuid,
    pub status: ExecutionStatus,
    pub progress: f64,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub results: Option<JSON>,
    pub artifacts: Vec<WorkflowArtifact>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowArtifact {
    pub id: Uuid,
    pub name: String,
    pub artifact_type: ArtifactType,
    pub size: i64,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactType {
    Document,
    Dataset,
    Image,
    Video,
    Code,
    Other,
}

// System Configuration types
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct SystemConfiguration {
    pub id: Uuid,
    pub version: String,
    pub environment: Environment,
    pub features: FeatureFlags,
    pub limits: SystemLimits,
    pub security: SecurityConfig,
    pub integrations: IntegrationConfig,
    pub updated_at: DateTime<Utc>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub federated_research: bool,
    pub ai_marketplace: bool,
    pub quantum_computing: bool,
    pub knowledge_graphs: bool,
    pub bmad_integration: bool,
    pub real_time_collaboration: bool,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct SystemLimits {
    pub max_concurrent_workflows: i32,
    pub max_api_keys_per_user: i32,
    pub max_file_upload_size: i64,
    pub rate_limit_per_minute: i32,
    pub max_query_complexity: i32,
}

// Monitoring types
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_io: NetworkIO,
    pub active_connections: i32,
    pub request_rate: f64,
    pub error_rate: f64,
    pub response_time_p95: f64,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkIO {
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub packets_in: i64,
    pub packets_out: i64,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub service: String,
    pub timestamp: DateTime<Utc>,
    pub request_count: i32,
    pub error_count: i32,
    pub average_response_time: f64,
    pub p95_response_time: f64,
    pub p99_response_time: f64,
    pub throughput: f64,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct LiveMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub active_workflows: i32,
    pub api_requests_per_second: f64,
    pub error_rate: f64,
    pub database_connections: i32,
    pub cache_hit_rate: f64,
}

// V3.0.0 Feature types
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct FederatedResearchNode {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub network_id: Uuid,
    pub node_type: FederatedNodeType,
    pub capabilities: Vec<String>,
    pub status: NodeStatus,
    pub endpoint: String,
    pub last_seen: DateTime<Utc>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FederatedNodeType {
    Coordinator,
    Participant,
    Observer,
    ResourceProvider,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Syncing,
    Error,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub domain: String,
    pub status: GraphStatus,
    pub statistics: GraphStatistics,
    pub configuration: GraphConfiguration,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl KnowledgeGraph {
    pub fn has_read_access(&self, user_id: Uuid) -> bool {
        // TODO: Implement proper access control
        true
    }
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphStatus {
    Building,
    Ready,
    Updating,
    Error,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct GraphStatistics {
    pub node_count: i32,
    pub edge_count: i32,
    pub avg_degree: f64,
    pub clustering_coefficient: f64,
    pub diameter: i32,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct GraphConfiguration {
    pub embedding_model: String,
    pub similarity_threshold: f64,
    pub max_connections_per_node: i32,
    pub enable_auto_linking: bool,
}

// BMAD Integration types
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct BMadAgent {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub status: AgentStatus,
    pub last_active: DateTime<Utc>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    Active,
    Inactive,
    Busy,
    Error,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct BMadExecution {
    pub id: Uuid,
    pub agent_id: String,
    pub executor_id: Uuid,
    pub status: ExecutionStatus,
    pub progress: f64,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub results: Option<JSON>,
}

// Connection types for pagination
#[derive(SimpleObject, Clone, Debug)]
pub struct UserConnection {
    pub edges: Vec<UserEdge>,
    pub page_info: PageInfo,
    pub total_count: i32,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct UserEdge {
    pub node: User,
    pub cursor: String,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct ApiKeyConnection {
    pub edges: Vec<ApiKeyEdge>,
    pub page_info: PageInfo,
    pub total_count: i32,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct ApiKeyEdge {
    pub node: ApiKey,
    pub cursor: String,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct WorkflowConnection {
    pub edges: Vec<WorkflowEdge>,
    pub page_info: PageInfo,
    pub total_count: i32,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct WorkflowEdge {
    pub node: ResearchWorkflow,
    pub cursor: String,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

// Re-export commonly used types
pub use input_types::*;
pub use subscription_types::*;
pub use command_types::*;

pub mod input_types;
pub mod subscription_types;
pub mod command_types;
