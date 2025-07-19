use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Marketplace user model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub reputation_score: u32,
    pub total_contributions: u32,
    pub verified: bool,
    pub organization_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

/// AI agent in marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAgentMarketplace {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: AgentCategory,
    pub creator_id: Uuid,
    pub version: String,
    pub agent_config: AgentConfiguration,
    pub pricing_model: PricingModel,
    pub price_per_use: f64,
    pub downloads: u32,
    pub rating: f64,
    pub rating_count: u32,
    pub tags: Vec<String>,
    pub requirements: SystemRequirements,
    pub status: AgentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Agent category
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCategory {
    Research,
    Analysis,
    Automation,
    DataProcessing,
    Visualization,
    NaturalLanguage,
    MachineLearning,
    Integration,
    Security,
    Custom,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfiguration {
    pub agent_type: String,
    pub capabilities: Vec<String>,
    pub input_formats: Vec<String>,
    pub output_formats: Vec<String>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub dependencies: Vec<String>,
    pub resource_requirements: ResourceRequirements,
    pub execution_environment: String,
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub min_memory_mb: u32,
    pub min_cpu_cores: u8,
    pub gpu_required: bool,
    pub disk_space_mb: u32,
    pub network_access: bool,
    pub special_permissions: Vec<String>,
}

/// Pricing model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PricingModel {
    Free,
    Credits,
    Subscription,
    OneTime,
    PayPerUse,
}

/// System requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirements {
    pub supported_platforms: Vec<String>,
    pub minimum_version: String,
    pub required_services: Vec<String>,
    pub optional_services: Vec<String>,
    pub compatibility_notes: Option<String>,
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Draft,
    Published,
    Deprecated,
    Suspended,
    UnderReview,
}

/// Research methodology in marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMethodologyMarketplace {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub creator_id: Uuid,
    pub methodology_config: MethodologyConfiguration,
    pub category: MethodologyCategory,
    pub complexity_level: u8, // 1-5 scale
    pub estimated_time_minutes: Option<u32>,
    pub success_rate: Option<f64>,
    pub usage_count: u32,
    pub rating: f64,
    pub rating_count: u32,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Methodology configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyConfiguration {
    pub steps: Vec<MethodologyStep>,
    pub parameters: HashMap<String, ParameterDefinition>,
    pub validation_rules: Vec<ValidationRule>,
    pub output_format: String,
    pub quality_metrics: Vec<String>,
}

/// Methodology step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyStep {
    pub step_number: u32,
    pub name: String,
    pub description: String,
    pub step_type: StepType,
    pub required_inputs: Vec<String>,
    pub expected_outputs: Vec<String>,
    pub estimated_duration_minutes: Option<u32>,
    pub dependencies: Vec<u32>, // Step numbers this depends on
}

/// Step type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    DataCollection,
    Analysis,
    Validation,
    Synthesis,
    Reporting,
    Custom,
}

/// Parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub name: String,
    pub parameter_type: String,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation_rules: Vec<String>,
    pub description: String,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: String,
    pub condition: String,
    pub error_message: String,
    pub severity: ValidationSeverity,
}

/// Validation severity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Methodology category
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MethodologyCategory {
    Academic,
    Business,
    Technical,
    Market,
    Competitive,
    Scientific,
    Legal,
    Medical,
    Financial,
    Custom,
}

/// Community rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityRating {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_type: RatingTargetType,
    pub target_id: Uuid,
    pub rating: u8, // 1-5 scale
    pub review_text: Option<String>,
    pub helpful_votes: u32,
    pub created_at: DateTime<Utc>,
}

/// Rating target type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RatingTargetType {
    Agent,
    Methodology,
    User,
}

/// Marketplace search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSearchQuery {
    pub query: String,
    pub category: Option<AgentCategory>,
    pub pricing_model: Option<PricingModel>,
    pub min_rating: Option<f64>,
    pub tags: Vec<String>,
    pub sort_by: SortOption,
    pub sort_order: SortOrder,
    pub page: u32,
    pub page_size: u32,
}

/// Sort option
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOption {
    Relevance,
    Rating,
    Downloads,
    CreatedDate,
    UpdatedDate,
    Price,
    Name,
}

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Marketplace search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSearchResult {
    pub items: Vec<MarketplaceItem>,
    pub total_count: u32,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Marketplace item (union of agents and methodologies)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "item_type")]
pub enum MarketplaceItem {
    Agent(AIAgentMarketplace),
    Methodology(ResearchMethodologyMarketplace),
}

/// Agent installation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInstallationRequest {
    pub agent_id: Uuid,
    pub installation_path: Option<String>,
    pub configuration_overrides: HashMap<String, serde_json::Value>,
    pub auto_update: bool,
}

/// Agent installation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInstallationResult {
    pub success: bool,
    pub installation_id: Option<Uuid>,
    pub error_message: Option<String>,
    pub installed_version: Option<String>,
    pub installation_path: Option<String>,
}

/// Marketplace analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceAnalytics {
    pub user_id: Uuid,
    pub total_downloads: u32,
    pub total_uploads: u32,
    pub average_rating_given: f64,
    pub average_rating_received: f64,
    pub most_popular_category: AgentCategory,
    pub contribution_streak_days: u32,
    pub last_contribution: DateTime<Utc>,
}
