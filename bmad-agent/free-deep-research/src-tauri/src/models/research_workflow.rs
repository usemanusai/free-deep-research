use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Research methodology types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResearchMethodology {
    DonLim,
    NickScamara,
    Hybrid,
}

impl ResearchMethodology {
    /// Get the display name for the methodology
    pub fn display_name(&self) -> &'static str {
        match self {
            ResearchMethodology::DonLim => "Don Lim (OpenRouter + SerpApi + Jina AI)",
            ResearchMethodology::NickScamara => "Nick Scamara (Firecrawl + AI SDK)",
            ResearchMethodology::Hybrid => "Hybrid (Combined Methodologies)",
        }
    }
    
    /// Get the description for the methodology
    pub fn description(&self) -> &'static str {
        match self {
            ResearchMethodology::DonLim => "Cost-optimized approach using OpenRouter.ai, SerpApi, and Jina AI for comprehensive research",
            ResearchMethodology::NickScamara => "Professional interface approach using Firecrawl and AI SDK for advanced web scraping",
            ResearchMethodology::Hybrid => "Intelligent combination of both methodologies for maximum research coverage",
        }
    }
}

/// Output format options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Markdown,
    Pdf,
    Html,
    Json,
}

/// Research workflow status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Research workflow parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowParameters {
    pub max_iterations: u32,
    pub methodology: ResearchMethodology,
    pub output_format: OutputFormat,
    pub include_sources: bool,
    pub max_sources: Option<u32>,
    pub custom_parameters: HashMap<String, serde_json::Value>,
}

impl Default for WorkflowParameters {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            methodology: ResearchMethodology::Hybrid,
            output_format: OutputFormat::Markdown,
            include_sources: true,
            max_sources: Some(50),
            custom_parameters: HashMap::new(),
        }
    }
}

/// Research results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResults {
    pub content: String,
    pub sources: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub word_count: u32,
    pub source_count: u32,
    pub methodology_used: ResearchMethodology,
    pub execution_time_ms: u64,
}

/// Research workflow model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchWorkflow {
    pub id: Uuid,
    pub name: String,
    pub template_id: Option<Uuid>,
    pub query: String,
    pub parameters: WorkflowParameters,
    pub status: WorkflowStatus,
    pub progress: f64, // 0.0 to 100.0
    pub results: Option<ResearchResults>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

impl ResearchWorkflow {
    /// Create a new research workflow
    pub fn new(name: String, query: String, parameters: WorkflowParameters) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name,
            template_id: None,
            query,
            parameters,
            status: WorkflowStatus::Pending,
            progress: 0.0,
            results: None,
            error_message: None,
            created_at: now,
            started_at: None,
            completed_at: None,
            updated_at: now,
        }
    }
    
    /// Start the workflow
    pub fn start(&mut self) {
        self.status = WorkflowStatus::Running;
        self.started_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    /// Update progress
    pub fn update_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 100.0);
        self.updated_at = Utc::now();
    }
    
    /// Complete the workflow with results
    pub fn complete(&mut self, results: ResearchResults) {
        self.status = WorkflowStatus::Completed;
        self.progress = 100.0;
        self.results = Some(results);
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    /// Fail the workflow with an error
    pub fn fail(&mut self, error_message: String) {
        self.status = WorkflowStatus::Failed;
        self.error_message = Some(error_message);
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    /// Cancel the workflow
    pub fn cancel(&mut self) {
        self.status = WorkflowStatus::Cancelled;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    /// Check if the workflow is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, WorkflowStatus::Running | WorkflowStatus::Pending)
    }
    
    /// Check if the workflow is completed
    pub fn is_completed(&self) -> bool {
        matches!(
            self.status,
            WorkflowStatus::Completed | WorkflowStatus::Failed | WorkflowStatus::Cancelled
        )
    }
    
    /// Get execution duration in milliseconds
    pub fn execution_duration_ms(&self) -> Option<u64> {
        if let (Some(started), Some(completed)) = (self.started_at, self.completed_at) {
            Some((completed - started).num_milliseconds() as u64)
        } else {
            None
        }
    }
}

/// Research workflow creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkflowRequest {
    pub name: String,
    pub query: String,
    pub template_id: Option<Uuid>,
    pub parameters: Option<WorkflowParameters>,
}

/// Research workflow update request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkflowRequest {
    pub name: Option<String>,
    pub query: Option<String>,
    pub parameters: Option<WorkflowParameters>,
}

/// Research template model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub parameters: WorkflowParameters,
    pub is_builtin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResearchTemplate {
    /// Create a new research template
    pub fn new(name: String, description: String, parameters: WorkflowParameters) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            parameters,
            is_builtin: false,
            created_at: now,
            updated_at: now,
        }
    }
}
