use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// NLP model definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLPModel {
    pub id: Uuid,
    pub name: String,
    pub model_type: ModelType,
    pub provider: ModelProvider,
    pub model_path: Option<String>,
    pub api_endpoint: Option<String>,
    pub capabilities: Vec<ModelCapability>,
    pub language_support: Vec<String>,
    pub max_tokens: Option<u32>,
    pub context_window: Option<u32>,
    pub performance_metrics: NLPPerformanceMetrics,
    pub cost_per_token: Option<f64>,
    pub status: ModelStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Model type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    Transformer,
    Embedding,
    Classification,
    NamedEntityRecognition,
    SentimentAnalysis,
    Summarization,
    QuestionAnswering,
    TextGeneration,
    Translation,
}

/// Model provider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelProvider {
    Local,
    OpenRouter,
    HuggingFace,
    OpenAI,
    Anthropic,
    Custom,
}

/// Model capability
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelCapability {
    TextGeneration,
    TextClassification,
    NamedEntityRecognition,
    SentimentAnalysis,
    Summarization,
    QuestionAnswering,
    Translation,
    EmbeddingGeneration,
    TopicModeling,
    IntentClassification,
    EntityExtraction,
    RelationExtraction,
}

/// NLP performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLPPerformanceMetrics {
    pub accuracy: Option<f64>,
    pub precision: Option<f64>,
    pub recall: Option<f64>,
    pub f1_score: Option<f64>,
    pub bleu_score: Option<f64>,
    pub rouge_score: Option<f64>,
    pub perplexity: Option<f64>,
    pub inference_time_ms: Option<f64>,
    pub throughput_tokens_per_second: Option<f64>,
    pub memory_usage_mb: Option<f64>,
}

/// Model status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelStatus {
    Available,
    Loading,
    Unavailable,
    Error,
    Deprecated,
}

/// Literature review model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteratureReview {
    pub id: Uuid,
    pub research_query: String,
    pub nlp_model_id: Uuid,
    pub search_parameters: SearchParameters,
    pub sources_found: u32,
    pub sources_analyzed: u32,
    pub key_findings: Vec<KeyFinding>,
    pub sentiment_analysis: SentimentAnalysis,
    pub topic_clusters: Vec<TopicCluster>,
    pub confidence_score: f64,
    pub processing_time_ms: u32,
    pub status: ReviewStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Search parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParameters {
    pub databases: Vec<String>,
    pub date_range: Option<DateRange>,
    pub language_filters: Vec<String>,
    pub document_types: Vec<String>,
    pub max_results: u32,
    pub relevance_threshold: f64,
    pub include_citations: bool,
    pub exclude_keywords: Vec<String>,
}

/// Date range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

/// Key finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyFinding {
    pub finding_id: Uuid,
    pub title: String,
    pub summary: String,
    pub evidence_strength: EvidenceStrength,
    pub source_count: u32,
    pub confidence_score: f64,
    pub related_concepts: Vec<String>,
    pub supporting_quotes: Vec<SupportingQuote>,
    pub contradictory_evidence: Vec<String>,
}

/// Evidence strength
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStrength {
    Weak,
    Moderate,
    Strong,
    VeryStrong,
    Conclusive,
}

/// Supporting quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportingQuote {
    pub text: String,
    pub source: String,
    pub page_number: Option<u32>,
    pub relevance_score: f64,
}

/// Sentiment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    pub overall_sentiment: Sentiment,
    pub sentiment_distribution: SentimentDistribution,
    pub sentiment_trends: Vec<SentimentTrend>,
    pub emotional_indicators: Vec<EmotionalIndicator>,
}

/// Sentiment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sentiment {
    VeryNegative,
    Negative,
    Neutral,
    Positive,
    VeryPositive,
}

/// Sentiment distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentDistribution {
    pub very_negative: f64,
    pub negative: f64,
    pub neutral: f64,
    pub positive: f64,
    pub very_positive: f64,
}

/// Sentiment trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentTrend {
    pub time_period: String,
    pub sentiment_score: f64,
    pub document_count: u32,
}

/// Emotional indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalIndicator {
    pub emotion: String,
    pub intensity: f64,
    pub frequency: u32,
}

/// Topic cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicCluster {
    pub cluster_id: Uuid,
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub document_count: u32,
    pub coherence_score: f64,
    pub representative_documents: Vec<String>,
    pub related_clusters: Vec<Uuid>,
}

/// Review status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// Semantic query model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticQuery {
    pub id: Uuid,
    pub original_query: String,
    pub processed_query: String,
    pub intent_classification: IntentClassification,
    pub entity_extraction: EntityExtraction,
    pub query_expansion: QueryExpansion,
    pub semantic_embedding: Vec<f32>, // Vector embedding
    pub nlp_model_id: Uuid,
    pub confidence_score: f64,
    pub processing_time_ms: u32,
    pub created_at: DateTime<Utc>,
}

/// Intent classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentClassification {
    pub primary_intent: Intent,
    pub secondary_intents: Vec<Intent>,
    pub confidence_scores: HashMap<String, f64>,
    pub intent_parameters: HashMap<String, serde_json::Value>,
}

/// Intent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    Search,
    Analysis,
    Comparison,
    Summarization,
    Question,
    Definition,
    Explanation,
    Recommendation,
    Prediction,
    Classification,
}

/// Entity extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityExtraction {
    pub entities: Vec<ExtractedEntity>,
    pub relationships: Vec<EntityRelationship>,
    pub confidence_threshold: f64,
}

/// Extracted entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntity {
    pub entity_id: Uuid,
    pub text: String,
    pub entity_type: EntityType,
    pub start_position: u32,
    pub end_position: u32,
    pub confidence_score: f64,
    pub normalized_form: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Entity type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Time,
    Money,
    Percentage,
    Product,
    Event,
    Concept,
    Technology,
    Method,
    Custom,
}

/// Entity relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRelationship {
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub relationship_type: RelationshipType,
    pub confidence_score: f64,
    pub context: String,
}

/// Relationship type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    PartOf,
    LocatedIn,
    WorksFor,
    CreatedBy,
    RelatedTo,
    CausedBy,
    UsedFor,
    InstanceOf,
    SimilarTo,
    OppositeOf,
}

/// Query expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryExpansion {
    pub expanded_terms: Vec<ExpandedTerm>,
    pub synonyms: Vec<String>,
    pub related_concepts: Vec<String>,
    pub contextual_terms: Vec<String>,
    pub expansion_strategy: ExpansionStrategy,
}

/// Expanded term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandedTerm {
    pub term: String,
    pub relevance_score: f64,
    pub term_type: TermType,
    pub source: String,
}

/// Term type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TermType {
    Synonym,
    Hypernym,
    Hyponym,
    Related,
    Contextual,
    Acronym,
    Abbreviation,
}

/// Expansion strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpansionStrategy {
    Semantic,
    Statistical,
    Hybrid,
    DomainSpecific,
    UserDefined,
}

/// NLP processing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLPProcessingRequest {
    pub text: String,
    pub model_id: Uuid,
    pub processing_types: Vec<ProcessingType>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub return_embeddings: bool,
    pub max_processing_time_ms: Option<u32>,
}

/// Processing type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingType {
    Tokenization,
    PartOfSpeechTagging,
    NamedEntityRecognition,
    SentimentAnalysis,
    TopicModeling,
    Summarization,
    KeywordExtraction,
    LanguageDetection,
    TextClassification,
}

/// NLP processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLPProcessingResult {
    pub request_id: Uuid,
    pub results: HashMap<ProcessingType, serde_json::Value>,
    pub embeddings: Option<Vec<f32>>,
    pub processing_time_ms: u32,
    pub model_used: Uuid,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}
