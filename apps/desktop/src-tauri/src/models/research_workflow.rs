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
    Created,
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Workflow step status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
    Retrying,
}

/// Research workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_number: u32,
    pub name: String,
    pub description: String,
    pub service_provider: Option<String>,
    pub endpoint: Option<String>,
    pub input_data: HashMap<String, serde_json::Value>,
    pub output_data: Option<HashMap<String, serde_json::Value>>,
    pub status: StepStatus,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub execution_time_ms: Option<u32>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub depends_on: Vec<Uuid>, // Step dependencies
    pub metadata: HashMap<String, String>,
}

impl WorkflowStep {
    /// Create a new workflow step
    pub fn new(
        workflow_id: Uuid,
        step_number: u32,
        name: String,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_id,
            step_number,
            name,
            description,
            service_provider: None,
            endpoint: None,
            input_data: HashMap::new(),
            output_data: None,
            status: StepStatus::Pending,
            error_message: None,
            started_at: None,
            completed_at: None,
            execution_time_ms: None,
            retry_count: 0,
            max_retries: 3,
            depends_on: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Mark step as started
    pub fn start(&mut self) {
        self.status = StepStatus::Running;
        self.started_at = Some(Utc::now());
    }

    /// Mark step as completed with output
    pub fn complete(&mut self, output: HashMap<String, serde_json::Value>) {
        self.status = StepStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.output_data = Some(output);

        if let Some(started) = self.started_at {
            self.execution_time_ms = Some((Utc::now() - started).num_milliseconds() as u32);
        }
    }

    /// Mark step as failed with error
    pub fn fail(&mut self, error: String) {
        self.status = StepStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error_message = Some(error);

        if let Some(started) = self.started_at {
            self.execution_time_ms = Some((Utc::now() - started).num_milliseconds() as u32);
        }
    }

    /// Check if step can be executed (dependencies met)
    pub fn can_execute(&self, completed_steps: &[Uuid]) -> bool {
        self.status == StepStatus::Pending &&
        self.depends_on.iter().all(|dep| completed_steps.contains(dep))
    }

    /// Check if step should be retried
    pub fn should_retry(&self) -> bool {
        self.status == StepStatus::Failed && self.retry_count < self.max_retries
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
        self.status = StepStatus::Retrying;
    }
}

/// Research workflow parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowParameters {
    pub max_iterations: u32,
    pub methodology: ResearchMethodology,
    pub output_format: OutputFormat,
    pub include_sources: bool,
    pub max_sources: Option<u32>,
    pub max_concurrent_steps: u32,
    pub timeout_minutes: u32,
    pub auto_retry: bool,
    pub save_intermediate_results: bool,
    pub enable_caching: bool,
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
            max_concurrent_steps: 3,
            timeout_minutes: 30,
            auto_retry: true,
            save_intermediate_results: true,
            enable_caching: true,
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
    pub steps: Vec<WorkflowStep>,
    pub results: Option<ResearchResults>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl ResearchWorkflow {
    /// Create a new research workflow
    pub fn new(name: String, query: String, parameters: WorkflowParameters, created_by: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            name,
            template_id: None,
            query,
            parameters,
            status: WorkflowStatus::Created,
            progress: 0.0,
            steps: Vec::new(),
            results: None,
            error_message: None,
            created_at: now,
            started_at: None,
            completed_at: None,
            updated_at: now,
            created_by,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a step to the workflow
    pub fn add_step(&mut self, mut step: WorkflowStep) {
        step.workflow_id = self.id;
        step.step_number = self.steps.len() as u32 + 1;
        self.steps.push(step);
        self.updated_at = Utc::now();
    }
    
    /// Start the workflow
    pub fn start(&mut self) {
        self.status = WorkflowStatus::Running;
        self.started_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Pause the workflow
    pub fn pause(&mut self) {
        if self.status == WorkflowStatus::Running {
            self.status = WorkflowStatus::Paused;
            self.updated_at = Utc::now();
        }
    }

    /// Resume the workflow
    pub fn resume(&mut self) {
        if self.status == WorkflowStatus::Paused {
            self.status = WorkflowStatus::Running;
            self.updated_at = Utc::now();
        }
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

    /// Get next executable steps
    pub fn get_next_steps(&self) -> Vec<&WorkflowStep> {
        let completed_step_ids: Vec<Uuid> = self.steps.iter()
            .filter(|step| step.status == StepStatus::Completed)
            .map(|step| step.id)
            .collect();

        self.steps.iter()
            .filter(|step| step.can_execute(&completed_step_ids))
            .collect()
    }

    /// Get failed steps that can be retried
    pub fn get_retryable_steps(&self) -> Vec<&WorkflowStep> {
        self.steps.iter()
            .filter(|step| step.should_retry())
            .collect()
    }

    /// Get running steps
    pub fn get_running_steps(&self) -> Vec<&WorkflowStep> {
        self.steps.iter()
            .filter(|step| step.status == StepStatus::Running)
            .collect()
    }

    /// Calculate progress based on completed steps
    pub fn calculate_progress(&mut self) {
        if self.steps.is_empty() {
            self.progress = 0.0;
            return;
        }

        let completed_steps = self.steps.iter()
            .filter(|step| step.status == StepStatus::Completed)
            .count();

        self.progress = (completed_steps as f64 / self.steps.len() as f64) * 100.0;
        self.updated_at = Utc::now();
    }

    /// Check if all steps are completed
    pub fn all_steps_completed(&self) -> bool {
        !self.steps.is_empty() &&
        self.steps.iter().all(|step| step.status == StepStatus::Completed || step.status == StepStatus::Skipped)
    }

    /// Check if any step has failed
    pub fn has_failed_steps(&self) -> bool {
        self.steps.iter().any(|step| step.status == StepStatus::Failed)
    }

    /// Get step by ID
    pub fn get_step_mut(&mut self, step_id: Uuid) -> Option<&mut WorkflowStep> {
        self.steps.iter_mut().find(|step| step.id == step_id)
    }

    /// Get step by ID (immutable)
    pub fn get_step(&self, step_id: Uuid) -> Option<&WorkflowStep> {
        self.steps.iter().find(|step| step.id == step_id)
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
