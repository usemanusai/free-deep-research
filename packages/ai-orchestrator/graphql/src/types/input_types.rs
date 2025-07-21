// GraphQL Input Types for Free Deep Research System
// Phase 4.4: API Gateway & GraphQL

use async_graphql::{InputObject, ID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::*;

// Authentication input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProfileInput {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub preferences: Option<UserPreferencesInput>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UserPreferencesInput {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub notifications: Option<NotificationSettingsInput>,
    pub dashboard_layout: Option<JSON>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSettingsInput {
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub workflow_updates: Option<bool>,
    pub system_alerts: Option<bool>,
    pub collaboration_invites: Option<bool>,
}

// API Key input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct CreateApiKeyInput {
    pub name: String,
    pub service: ServiceProvider,
    pub key: String,
    pub rate_limit: Option<i32>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateApiKeyInput {
    pub name: Option<String>,
    pub rate_limit: Option<i32>,
    pub status: Option<ApiKeyStatus>,
}

// Research Workflow input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct CreateWorkflowInput {
    pub name: String,
    pub description: Option<String>,
    pub methodology: ResearchMethodology,
    pub template_id: Option<ID>,
    pub configuration: WorkflowConfigurationInput,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateWorkflowInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub methodology: Option<ResearchMethodology>,
    pub configuration: Option<WorkflowConfigurationInput>,
    pub collaborators: Option<Vec<ID>>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowConfigurationInput {
    pub max_depth: Option<i32>,
    pub max_sources: Option<i32>,
    pub quality_threshold: Option<f64>,
    pub enable_fact_checking: Option<bool>,
    pub output_language: Option<String>,
    pub custom_parameters: Option<JSON>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteWorkflowInput {
    pub parameters: Option<JSON>,
    pub priority: Option<ExecutionPriority>,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionPriority {
    Low,
    Normal,
    High,
    Critical,
}

// Template input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct CreateTemplateInput {
    pub name: String,
    pub description: Option<String>,
    pub methodology: ResearchMethodology,
    pub configuration: WorkflowConfigurationInput,
    pub is_public: Option<bool>,
    pub tags: Option<Vec<String>>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTemplateInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub configuration: Option<WorkflowConfigurationInput>,
    pub is_public: Option<bool>,
    pub tags: Option<Vec<String>>,
}

// System Configuration input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct SystemConfigInput {
    pub features: Option<FeatureFlagsInput>,
    pub limits: Option<SystemLimitsInput>,
    pub security: Option<SecurityConfigInput>,
    pub integrations: Option<IntegrationConfigInput>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureFlagsInput {
    pub federated_research: Option<bool>,
    pub ai_marketplace: Option<bool>,
    pub quantum_computing: Option<bool>,
    pub knowledge_graphs: Option<bool>,
    pub bmad_integration: Option<bool>,
    pub real_time_collaboration: Option<bool>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct SystemLimitsInput {
    pub max_concurrent_workflows: Option<i32>,
    pub max_api_keys_per_user: Option<i32>,
    pub max_file_upload_size: Option<i64>,
    pub rate_limit_per_minute: Option<i32>,
    pub max_query_complexity: Option<i32>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityConfigInput {
    pub enable_2fa: Option<bool>,
    pub session_timeout: Option<i32>,
    pub password_policy: Option<PasswordPolicyInput>,
    pub ip_whitelist: Option<Vec<String>>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct PasswordPolicyInput {
    pub min_length: Option<i32>,
    pub require_uppercase: Option<bool>,
    pub require_lowercase: Option<bool>,
    pub require_numbers: Option<bool>,
    pub require_symbols: Option<bool>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct IntegrationConfigInput {
    pub webhook_endpoints: Option<Vec<WebhookEndpointInput>>,
    pub external_apis: Option<Vec<ExternalApiConfigInput>>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct WebhookEndpointInput {
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalApiConfigInput {
    pub name: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub rate_limit: Option<i32>,
    pub timeout: Option<i32>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UserConfigInput {
    pub preferences: Option<UserPreferencesInput>,
    pub api_settings: Option<ApiSettingsInput>,
    pub notification_settings: Option<NotificationSettingsInput>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct ApiSettingsInput {
    pub default_rate_limit: Option<i32>,
    pub enable_analytics: Option<bool>,
    pub preferred_providers: Option<Vec<ServiceProvider>>,
}

// V3.0.0 Feature input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct FederatedResearchInput {
    pub name: String,
    pub description: String,
    pub network_id: ID,
    pub node_type: FederatedNodeType,
    pub capabilities: Vec<String>,
    pub endpoint: Option<String>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct MarketplacePublishInput {
    pub name: String,
    pub description: String,
    pub category: MarketplaceCategory,
    pub price: Option<f64>,
    pub license: String,
    pub tags: Vec<String>,
    pub metadata: Option<JSON>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketplaceCategory {
    Models,
    Datasets,
    Tools,
    Templates,
    Workflows,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct QuantumDeployInput {
    pub workflow_id: ID,
    pub quantum_backend: QuantumBackend,
    pub optimization_level: Option<i32>,
    pub shots: Option<i32>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantumBackend {
    Simulator,
    IbmQuantum,
    GoogleQuantum,
    AmazonBraket,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct NLPTrainingInput {
    pub model_name: String,
    pub dataset_id: ID,
    pub training_config: NLPTrainingConfig,
    pub validation_split: Option<f64>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct NLPTrainingConfig {
    pub epochs: i32,
    pub batch_size: i32,
    pub learning_rate: f64,
    pub model_type: NLPModelType,
    pub hyperparameters: Option<JSON>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NLPModelType {
    Transformer,
    Bert,
    Gpt,
    T5,
    Custom,
}

// Knowledge Graph input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct CreateKnowledgeGraphInput {
    pub name: String,
    pub description: String,
    pub domain: String,
    pub configuration: GraphConfigurationInput,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct GraphConfigurationInput {
    pub embedding_model: Option<String>,
    pub similarity_threshold: Option<f64>,
    pub max_connections_per_node: Option<i32>,
    pub enable_auto_linking: Option<bool>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct AddNodeInput {
    pub node_type: NodeType,
    pub name: String,
    pub description: Option<String>,
    pub properties: JSON,
    pub source_type: SourceType,
    pub source_id: Option<String>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Entity,
    Concept,
    Document,
    Person,
    Organization,
    Location,
    Event,
    Custom,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    Manual,
    Document,
    Web,
    Database,
    Api,
    Import,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct AddEdgeInput {
    pub from_node_id: ID,
    pub to_node_id: ID,
    pub edge_type: EdgeType,
    pub weight: Option<f64>,
    pub properties: Option<JSON>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    RelatedTo,
    PartOf,
    InstanceOf,
    SimilarTo,
    CausedBy,
    LocatedIn,
    Custom,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateNodeInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub properties: Option<JSON>,
}

// BMAD Integration input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct BMadExecutionInput {
    pub workflow_input: JSON,
    pub parameters: Option<JSON>,
    pub priority: Option<ExecutionPriority>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct BMadAgentInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub configuration: Option<JSON>,
}

// Filter input types
#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserFilter {
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
    pub search: Option<String>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ApiKeyFilter {
    pub service: Option<ServiceProvider>,
    pub status: Option<ApiKeyStatus>,
    pub search: Option<String>,
}

#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorkflowFilter {
    pub status: Option<WorkflowStatus>,
    pub methodology: Option<ResearchMethodology>,
    pub created_by: Option<Uuid>,
    pub search: Option<String>,
}

#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TemplateFilter {
    pub methodology: Option<ResearchMethodology>,
    pub is_public: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub search: Option<String>,
}

#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct FederatedFilter {
    pub network_id: Option<ID>,
    pub node_type: Option<FederatedNodeType>,
    pub status: Option<NodeStatus>,
}

#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct QuantumFilter {
    pub backend: Option<QuantumBackend>,
    pub status: Option<ExecutionStatus>,
}

#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct NLPModelFilter {
    pub model_type: Option<NLPModelType>,
    pub status: Option<ModelStatus>,
    pub search: Option<String>,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    Training,
    Ready,
    Deployed,
    Error,
}

#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct NodeFilter {
    pub node_type: Option<NodeType>,
    pub source_type: Option<SourceType>,
    pub search: Option<String>,
}

#[derive(InputObject, Clone, Debug, Default, Serialize, Deserialize)]
pub struct AuditLogFilter {
    pub user_id: Option<ID>,
    pub action: Option<String>,
    pub resource: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

// Pagination input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct PaginationInput {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub last: Option<i32>,
    pub before: Option<String>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeRange {
    pub fn last_24_hours() -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::hours(24);
        Self { start, end }
    }

    pub fn last_week() -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::weeks(1);
        Self { start, end }
    }

    pub fn last_month() -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::days(30);
        Self { start, end }
    }
}
