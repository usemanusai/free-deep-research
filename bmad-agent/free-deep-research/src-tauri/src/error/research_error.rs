use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Research-related errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ResearchError {
    #[error("Workflow not found: {workflow_id}")]
    WorkflowNotFound { workflow_id: String },
    
    #[error("Invalid workflow configuration: {message}")]
    InvalidWorkflowConfig { message: String },
    
    #[error("Template not found: {template_id}")]
    TemplateNotFound { template_id: String },
    
    #[error("Invalid template: {message}")]
    InvalidTemplate { message: String },
    
    #[error("Workflow execution failed: {workflow_id}: {message}")]
    ExecutionFailed {
        workflow_id: String,
        message: String,
    },
    
    #[error("Workflow timeout: {workflow_id}")]
    WorkflowTimeout { workflow_id: String },
    
    #[error("Workflow cancelled: {workflow_id}")]
    WorkflowCancelled { workflow_id: String },
    
    #[error("Queue full: cannot accept more research requests")]
    QueueFull,
    
    #[error("Invalid research query: {message}")]
    InvalidQuery { message: String },
    
    #[error("Result processing failed: {message}")]
    ResultProcessingFailed { message: String },
    
    #[error("Methodology not supported: {methodology}")]
    UnsupportedMethodology { methodology: String },
    
    #[error("Resource limit exceeded: {resource}: {current}/{limit}")]
    ResourceLimitExceeded {
        resource: String,
        current: u32,
        limit: u32,
    },
    
    #[error("Dependency failed: {dependency}: {message}")]
    DependencyFailed {
        dependency: String,
        message: String,
    },
}

impl ResearchError {
    /// Create a new workflow not found error
    pub fn workflow_not_found(workflow_id: impl Into<String>) -> Self {
        Self::WorkflowNotFound {
            workflow_id: workflow_id.into(),
        }
    }
    
    /// Create a new execution failed error
    pub fn execution_failed(workflow_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExecutionFailed {
            workflow_id: workflow_id.into(),
            message: message.into(),
        }
    }
    
    /// Create a new invalid query error
    pub fn invalid_query(message: impl Into<String>) -> Self {
        Self::InvalidQuery {
            message: message.into(),
        }
    }
    
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ResearchError::WorkflowTimeout { .. }
                | ResearchError::QueueFull
                | ResearchError::ResourceLimitExceeded { .. }
                | ResearchError::DependencyFailed { .. }
        )
    }
    
    /// Check if this error indicates a configuration issue
    pub fn is_config_error(&self) -> bool {
        matches!(
            self,
            ResearchError::InvalidWorkflowConfig { .. }
                | ResearchError::InvalidTemplate { .. }
                | ResearchError::UnsupportedMethodology { .. }
        )
    }
}
