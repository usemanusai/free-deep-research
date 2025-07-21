// Domain Events for Event Sourcing
// Phase 4.1: Event Sourcing Foundation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

/// Event metadata containing system-level information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: Uuid,
    pub stream_id: Uuid,
    pub event_type: String,
    pub event_version: u32,
    pub sequence_number: u64,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

/// Base trait for all domain events
#[async_trait]
pub trait DomainEvent: Debug + Send + Sync {
    /// Get the event type identifier
    fn event_type(&self) -> &'static str;
    
    /// Get the event version for schema evolution
    fn event_version(&self) -> u32 {
        1
    }
    
    /// Get correlation ID for tracking related events
    fn correlation_id(&self) -> Option<Uuid> {
        None
    }
    
    /// Get causation ID for tracking event causality
    fn causation_id(&self) -> Option<Uuid> {
        None
    }
    
    /// Serialize event data to JSON
    fn serialize(&self) -> Result<serde_json::Value, serde_json::Error>;
    
    /// Validate event data
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Research workflow domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchWorkflowEvent {
    WorkflowCreated {
        workflow_id: Uuid,
        name: String,
        query: String,
        methodology: ResearchMethodology,
        created_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    ExecutionStarted {
        workflow_id: Uuid,
        started_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    TaskCreated {
        workflow_id: Uuid,
        task_id: Uuid,
        task_type: String,
        agent_type: Option<String>,
        created_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    TaskCompleted {
        workflow_id: Uuid,
        task_id: Uuid,
        results: serde_json::Value,
        completed_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    ExecutionCompleted {
        workflow_id: Uuid,
        results: ResearchResults,
        completed_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    ExecutionFailed {
        workflow_id: Uuid,
        error: String,
        failed_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    WorkflowUpdated {
        workflow_id: Uuid,
        updates: serde_json::Value,
        updated_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
}

#[async_trait]
impl DomainEvent for ResearchWorkflowEvent {
    fn event_type(&self) -> &'static str {
        match self {
            ResearchWorkflowEvent::WorkflowCreated { .. } => "research.workflow.created",
            ResearchWorkflowEvent::ExecutionStarted { .. } => "research.workflow.started",
            ResearchWorkflowEvent::TaskCreated { .. } => "research.task.created",
            ResearchWorkflowEvent::TaskCompleted { .. } => "research.task.completed",
            ResearchWorkflowEvent::ExecutionCompleted { .. } => "research.workflow.completed",
            ResearchWorkflowEvent::ExecutionFailed { .. } => "research.workflow.failed",
            ResearchWorkflowEvent::WorkflowUpdated { .. } => "research.workflow.updated",
        }
    }

    fn correlation_id(&self) -> Option<Uuid> {
        match self {
            ResearchWorkflowEvent::WorkflowCreated { correlation_id, .. } => *correlation_id,
            ResearchWorkflowEvent::ExecutionStarted { correlation_id, .. } => *correlation_id,
            ResearchWorkflowEvent::TaskCreated { correlation_id, .. } => *correlation_id,
            ResearchWorkflowEvent::TaskCompleted { correlation_id, .. } => *correlation_id,
            ResearchWorkflowEvent::ExecutionCompleted { correlation_id, .. } => *correlation_id,
            ResearchWorkflowEvent::ExecutionFailed { correlation_id, .. } => *correlation_id,
            ResearchWorkflowEvent::WorkflowUpdated { correlation_id, .. } => *correlation_id,
        }
    }

    fn serialize(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    fn validate(&self) -> Result<(), String> {
        match self {
            ResearchWorkflowEvent::WorkflowCreated { name, query, .. } => {
                if name.trim().is_empty() {
                    return Err("Workflow name cannot be empty".to_string());
                }
                if query.trim().is_empty() {
                    return Err("Research query cannot be empty".to_string());
                }
            }
            ResearchWorkflowEvent::TaskCreated { task_type, .. } => {
                if task_type.trim().is_empty() {
                    return Err("Task type cannot be empty".to_string());
                }
            }
            _ => {}
        }
        Ok(())
    }
}

/// AI Agent domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIAgentEvent {
    AgentCreated {
        agent_id: Uuid,
        agent_type: String,
        configuration: serde_json::Value,
        created_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    TaskAssigned {
        agent_id: Uuid,
        task_id: Uuid,
        task_data: serde_json::Value,
        assigned_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    ResponseGenerated {
        agent_id: Uuid,
        task_id: Uuid,
        response: serde_json::Value,
        generated_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
    AgentError {
        agent_id: Uuid,
        task_id: Option<Uuid>,
        error: String,
        occurred_at: DateTime<Utc>,
        correlation_id: Option<Uuid>,
    },
}

#[async_trait]
impl DomainEvent for AIAgentEvent {
    fn event_type(&self) -> &'static str {
        match self {
            AIAgentEvent::AgentCreated { .. } => "ai.agent.created",
            AIAgentEvent::TaskAssigned { .. } => "ai.agent.task_assigned",
            AIAgentEvent::ResponseGenerated { .. } => "ai.agent.response_generated",
            AIAgentEvent::AgentError { .. } => "ai.agent.error",
        }
    }

    fn correlation_id(&self) -> Option<Uuid> {
        match self {
            AIAgentEvent::AgentCreated { correlation_id, .. } => *correlation_id,
            AIAgentEvent::TaskAssigned { correlation_id, .. } => *correlation_id,
            AIAgentEvent::ResponseGenerated { correlation_id, .. } => *correlation_id,
            AIAgentEvent::AgentError { correlation_id, .. } => *correlation_id,
        }
    }

    fn serialize(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    fn validate(&self) -> Result<(), String> {
        match self {
            AIAgentEvent::AgentCreated { agent_type, .. } => {
                if agent_type.trim().is_empty() {
                    return Err("Agent type cannot be empty".to_string());
                }
            }
            AIAgentEvent::AgentError { error, .. } => {
                if error.trim().is_empty() {
                    return Err("Error message cannot be empty".to_string());
                }
            }
            _ => {}
        }
        Ok(())
    }
}

/// Supporting data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMethodology {
    pub name: String,
    pub steps: Vec<String>,
    pub ai_agents: Vec<String>,
    pub estimated_duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResults {
    pub summary: String,
    pub findings: Vec<ResearchFinding>,
    pub sources: Vec<ResearchSource>,
    pub confidence_score: f64,
    pub completion_time_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchFinding {
    pub title: String,
    pub content: String,
    pub confidence: f64,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    pub url: String,
    pub title: String,
    pub relevance_score: f64,
    pub accessed_at: DateTime<Utc>,
}

/// Event factory for creating events with proper metadata
pub struct EventFactory {
    correlation_id: Option<Uuid>,
}

impl EventFactory {
    pub fn new() -> Self {
        Self {
            correlation_id: None,
        }
    }

    pub fn with_correlation_id(correlation_id: Uuid) -> Self {
        Self {
            correlation_id: Some(correlation_id),
        }
    }

    pub fn create_workflow_created(
        &self,
        workflow_id: Uuid,
        name: String,
        query: String,
        methodology: ResearchMethodology,
    ) -> ResearchWorkflowEvent {
        ResearchWorkflowEvent::WorkflowCreated {
            workflow_id,
            name,
            query,
            methodology,
            created_at: Utc::now(),
            correlation_id: self.correlation_id,
        }
    }

    pub fn create_execution_started(&self, workflow_id: Uuid) -> ResearchWorkflowEvent {
        ResearchWorkflowEvent::ExecutionStarted {
            workflow_id,
            started_at: Utc::now(),
            correlation_id: self.correlation_id,
        }
    }

    pub fn create_task_created(
        &self,
        workflow_id: Uuid,
        task_id: Uuid,
        task_type: String,
        agent_type: Option<String>,
    ) -> ResearchWorkflowEvent {
        ResearchWorkflowEvent::TaskCreated {
            workflow_id,
            task_id,
            task_type,
            agent_type,
            created_at: Utc::now(),
            correlation_id: self.correlation_id,
        }
    }

    pub fn create_agent_created(
        &self,
        agent_id: Uuid,
        agent_type: String,
        configuration: serde_json::Value,
    ) -> AIAgentEvent {
        AIAgentEvent::AgentCreated {
            agent_id,
            agent_type,
            configuration,
            created_at: Utc::now(),
            correlation_id: self.correlation_id,
        }
    }
}

impl Default for EventFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_created_event() {
        let methodology = ResearchMethodology {
            name: "Deep Research".to_string(),
            steps: vec!["Search".to_string(), "Analyze".to_string()],
            ai_agents: vec!["researcher".to_string()],
            estimated_duration_minutes: 30,
        };

        let event = ResearchWorkflowEvent::WorkflowCreated {
            workflow_id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            query: "Test Query".to_string(),
            methodology,
            created_at: Utc::now(),
            correlation_id: Some(Uuid::new_v4()),
        };

        assert_eq!(event.event_type(), "research.workflow.created");
        assert!(event.validate().is_ok());
        assert!(event.serialize().is_ok());
    }

    #[test]
    fn test_event_validation() {
        let event = ResearchWorkflowEvent::WorkflowCreated {
            workflow_id: Uuid::new_v4(),
            name: "".to_string(), // Empty name should fail validation
            query: "Test Query".to_string(),
            methodology: ResearchMethodology {
                name: "Test".to_string(),
                steps: vec![],
                ai_agents: vec![],
                estimated_duration_minutes: 0,
            },
            created_at: Utc::now(),
            correlation_id: None,
        };

        assert!(event.validate().is_err());
    }

    #[test]
    fn test_event_factory() {
        let correlation_id = Uuid::new_v4();
        let factory = EventFactory::with_correlation_id(correlation_id);

        let event = factory.create_execution_started(Uuid::new_v4());
        assert_eq!(event.correlation_id(), Some(correlation_id));
    }
}
