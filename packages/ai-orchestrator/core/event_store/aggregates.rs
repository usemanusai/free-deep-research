// Aggregate Root Pattern for Event Sourcing
// Phase 4.1: Event Sourcing Foundation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

use super::error::{EventStoreError, EventStoreResult};
use super::events::{DomainEvent, ResearchWorkflowEvent, AIAgentEvent, ResearchMethodology, ResearchResults};

/// Aggregate ID type
pub type AggregateId = Uuid;

/// Base trait for all aggregate roots
#[async_trait]
pub trait AggregateRoot: Debug + Send + Sync {
    type Event: DomainEvent + Clone;
    type State: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>;
    
    /// Get the aggregate ID
    fn get_id(&self) -> AggregateId;
    
    /// Get the current version
    fn get_version(&self) -> u64;
    
    /// Get uncommitted events
    fn get_uncommitted_events(&self) -> &[Self::Event];
    
    /// Mark events as committed
    fn mark_events_as_committed(&mut self);
    
    /// Apply an event to the aggregate
    fn apply_event(&mut self, event: &Self::Event);
    
    /// Get the current state for snapshotting
    fn get_state(&self) -> &Self::State;
    
    /// Restore from state (for snapshot loading)
    fn restore_from_state(id: AggregateId, state: Self::State, version: u64) -> Self;
    
    /// Validate the aggregate's current state
    fn validate(&self) -> EventStoreResult<()> {
        Ok(())
    }
}

/// Research workflow aggregate state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResearchWorkflowState {
    pub id: AggregateId,
    pub name: String,
    pub query: String,
    pub methodology: Option<ResearchMethodology>,
    pub status: WorkflowStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub results: Option<ResearchResults>,
    pub tasks: Vec<TaskInfo>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Created,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskInfo {
    pub task_id: Uuid,
    pub task_type: String,
    pub agent_type: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub results: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Created,
    Running,
    Completed,
    Failed,
}

impl Default for ResearchWorkflowState {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            query: String::new(),
            methodology: None,
            status: WorkflowStatus::Created,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            results: None,
            tasks: Vec::new(),
            error_message: None,
        }
    }
}

/// Research workflow aggregate
#[derive(Debug, Clone)]
pub struct ResearchWorkflowAggregate {
    id: AggregateId,
    version: u64,
    state: ResearchWorkflowState,
    uncommitted_events: Vec<ResearchWorkflowEvent>,
}

impl ResearchWorkflowAggregate {
    /// Create a new research workflow
    pub fn create_workflow(
        id: AggregateId,
        name: String,
        query: String,
        methodology: ResearchMethodology,
    ) -> EventStoreResult<Self> {
        // Validate inputs
        if name.trim().is_empty() {
            return Err(EventStoreError::invalid_operation("Workflow name cannot be empty"));
        }
        if query.trim().is_empty() {
            return Err(EventStoreError::invalid_operation("Research query cannot be empty"));
        }
        
        let mut aggregate = Self {
            id,
            version: 0,
            state: ResearchWorkflowState::default(),
            uncommitted_events: Vec::new(),
        };
        
        let event = ResearchWorkflowEvent::WorkflowCreated {
            workflow_id: id,
            name,
            query,
            methodology,
            created_at: Utc::now(),
            correlation_id: None,
        };
        
        aggregate.apply_event(&event);
        aggregate.uncommitted_events.push(event);
        
        Ok(aggregate)
    }
    
    /// Start workflow execution
    pub fn start_execution(&mut self) -> EventStoreResult<()> {
        if self.state.status != WorkflowStatus::Created {
            return Err(EventStoreError::invalid_operation(
                format!("Cannot start workflow in status: {:?}", self.state.status)
            ));
        }
        
        let event = ResearchWorkflowEvent::ExecutionStarted {
            workflow_id: self.id,
            started_at: Utc::now(),
            correlation_id: None,
        };
        
        self.apply_event(&event);
        self.uncommitted_events.push(event);
        
        Ok(())
    }
    
    /// Create a new task
    pub fn create_task(
        &mut self,
        task_id: Uuid,
        task_type: String,
        agent_type: Option<String>,
    ) -> EventStoreResult<()> {
        if self.state.status != WorkflowStatus::Running {
            return Err(EventStoreError::invalid_operation(
                "Cannot create task when workflow is not running"
            ));
        }
        
        if task_type.trim().is_empty() {
            return Err(EventStoreError::invalid_operation("Task type cannot be empty"));
        }
        
        let event = ResearchWorkflowEvent::TaskCreated {
            workflow_id: self.id,
            task_id,
            task_type,
            agent_type,
            created_at: Utc::now(),
            correlation_id: None,
        };
        
        self.apply_event(&event);
        self.uncommitted_events.push(event);
        
        Ok(())
    }
    
    /// Complete a task
    pub fn complete_task(
        &mut self,
        task_id: Uuid,
        results: serde_json::Value,
    ) -> EventStoreResult<()> {
        // Check if task exists and is not already completed
        let task_exists = self.state.tasks.iter().any(|t| {
            t.task_id == task_id && t.status != TaskStatus::Completed
        });
        
        if !task_exists {
            return Err(EventStoreError::invalid_operation(
                "Task not found or already completed"
            ));
        }
        
        let event = ResearchWorkflowEvent::TaskCompleted {
            workflow_id: self.id,
            task_id,
            results,
            completed_at: Utc::now(),
            correlation_id: None,
        };
        
        self.apply_event(&event);
        self.uncommitted_events.push(event);
        
        Ok(())
    }
    
    /// Complete workflow execution
    pub fn complete_execution(&mut self, results: ResearchResults) -> EventStoreResult<()> {
        if self.state.status != WorkflowStatus::Running {
            return Err(EventStoreError::invalid_operation(
                "Cannot complete workflow that is not running"
            ));
        }
        
        let event = ResearchWorkflowEvent::ExecutionCompleted {
            workflow_id: self.id,
            results,
            completed_at: Utc::now(),
            correlation_id: None,
        };
        
        self.apply_event(&event);
        self.uncommitted_events.push(event);
        
        Ok(())
    }
    
    /// Fail workflow execution
    pub fn fail_execution(&mut self, error: String) -> EventStoreResult<()> {
        if matches!(self.state.status, WorkflowStatus::Completed | WorkflowStatus::Failed) {
            return Err(EventStoreError::invalid_operation(
                "Cannot fail workflow that is already completed or failed"
            ));
        }
        
        let event = ResearchWorkflowEvent::ExecutionFailed {
            workflow_id: self.id,
            error,
            failed_at: Utc::now(),
            correlation_id: None,
        };
        
        self.apply_event(&event);
        self.uncommitted_events.push(event);
        
        Ok(())
    }
    
    /// Update workflow metadata
    pub fn update_workflow(&mut self, updates: serde_json::Value) -> EventStoreResult<()> {
        let event = ResearchWorkflowEvent::WorkflowUpdated {
            workflow_id: self.id,
            updates,
            updated_at: Utc::now(),
            correlation_id: None,
        };
        
        self.apply_event(&event);
        self.uncommitted_events.push(event);
        
        Ok(())
    }
    
    /// Check if all tasks are completed
    pub fn are_all_tasks_completed(&self) -> bool {
        !self.state.tasks.is_empty() && 
        self.state.tasks.iter().all(|t| t.status == TaskStatus::Completed)
    }
    
    /// Get task by ID
    pub fn get_task(&self, task_id: Uuid) -> Option<&TaskInfo> {
        self.state.tasks.iter().find(|t| t.task_id == task_id)
    }
    
    /// Get tasks by status
    pub fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<&TaskInfo> {
        self.state.tasks.iter().filter(|t| t.status == status).collect()
    }
}

#[async_trait]
impl AggregateRoot for ResearchWorkflowAggregate {
    type Event = ResearchWorkflowEvent;
    type State = ResearchWorkflowState;
    
    fn get_id(&self) -> AggregateId {
        self.id
    }
    
    fn get_version(&self) -> u64 {
        self.version
    }
    
    fn get_uncommitted_events(&self) -> &[Self::Event] {
        &self.uncommitted_events
    }
    
    fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }
    
    fn apply_event(&mut self, event: &Self::Event) {
        match event {
            ResearchWorkflowEvent::WorkflowCreated { 
                workflow_id, name, query, methodology, created_at, .. 
            } => {
                self.state.id = *workflow_id;
                self.state.name = name.clone();
                self.state.query = query.clone();
                self.state.methodology = Some(methodology.clone());
                self.state.status = WorkflowStatus::Created;
                self.state.created_at = *created_at;
            }
            ResearchWorkflowEvent::ExecutionStarted { started_at, .. } => {
                self.state.status = WorkflowStatus::Running;
                self.state.started_at = Some(*started_at);
            }
            ResearchWorkflowEvent::TaskCreated { 
                task_id, task_type, agent_type, created_at, .. 
            } => {
                let task = TaskInfo {
                    task_id: *task_id,
                    task_type: task_type.clone(),
                    agent_type: agent_type.clone(),
                    status: TaskStatus::Created,
                    created_at: *created_at,
                    completed_at: None,
                    results: None,
                };
                self.state.tasks.push(task);
            }
            ResearchWorkflowEvent::TaskCompleted { 
                task_id, results, completed_at, .. 
            } => {
                if let Some(task) = self.state.tasks.iter_mut().find(|t| t.task_id == *task_id) {
                    task.status = TaskStatus::Completed;
                    task.completed_at = Some(*completed_at);
                    task.results = Some(results.clone());
                }
            }
            ResearchWorkflowEvent::ExecutionCompleted { 
                results, completed_at, .. 
            } => {
                self.state.status = WorkflowStatus::Completed;
                self.state.results = Some(results.clone());
                self.state.completed_at = Some(*completed_at);
            }
            ResearchWorkflowEvent::ExecutionFailed { error, failed_at, .. } => {
                self.state.status = WorkflowStatus::Failed;
                self.state.error_message = Some(error.clone());
                self.state.completed_at = Some(*failed_at);
            }
            ResearchWorkflowEvent::WorkflowUpdated { updates, .. } => {
                // Apply updates to workflow metadata
                if let Some(name) = updates.get("name").and_then(|v| v.as_str()) {
                    self.state.name = name.to_string();
                }
                // Add more update fields as needed
            }
        }
        
        self.version += 1;
    }
    
    fn get_state(&self) -> &Self::State {
        &self.state
    }
    
    fn restore_from_state(id: AggregateId, state: Self::State, version: u64) -> Self {
        Self {
            id,
            version,
            state,
            uncommitted_events: Vec::new(),
        }
    }
    
    fn validate(&self) -> EventStoreResult<()> {
        if self.state.name.trim().is_empty() {
            return Err(EventStoreError::invalid_operation("Workflow name cannot be empty"));
        }
        
        if self.state.query.trim().is_empty() {
            return Err(EventStoreError::invalid_operation("Research query cannot be empty"));
        }
        
        // Validate state transitions
        match self.state.status {
            WorkflowStatus::Running => {
                if self.state.started_at.is_none() {
                    return Err(EventStoreError::invalid_operation(
                        "Running workflow must have started_at timestamp"
                    ));
                }
            }
            WorkflowStatus::Completed => {
                if self.state.completed_at.is_none() {
                    return Err(EventStoreError::invalid_operation(
                        "Completed workflow must have completed_at timestamp"
                    ));
                }
                if self.state.results.is_none() {
                    return Err(EventStoreError::invalid_operation(
                        "Completed workflow must have results"
                    ));
                }
            }
            WorkflowStatus::Failed => {
                if self.state.error_message.is_none() {
                    return Err(EventStoreError::invalid_operation(
                        "Failed workflow must have error message"
                    ));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}

/// Aggregate repository trait for loading and saving aggregates
#[async_trait]
pub trait AggregateRepository<T: AggregateRoot>: Send + Sync {
    /// Load aggregate by ID
    async fn load(&self, id: AggregateId) -> EventStoreResult<Option<T>>;
    
    /// Save aggregate (append events)
    async fn save(&self, aggregate: &mut T) -> EventStoreResult<()>;
    
    /// Check if aggregate exists
    async fn exists(&self, id: AggregateId) -> EventStoreResult<bool>;
    
    /// Get aggregate version
    async fn get_version(&self, id: AggregateId) -> EventStoreResult<Option<u64>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_store::events::ResearchMethodology;

    #[test]
    fn test_create_research_workflow() {
        let id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Deep Research".to_string(),
            steps: vec!["Search".to_string(), "Analyze".to_string()],
            ai_agents: vec!["researcher".to_string()],
            estimated_duration_minutes: 30,
        };
        
        let aggregate = ResearchWorkflowAggregate::create_workflow(
            id,
            "Test Workflow".to_string(),
            "Test Query".to_string(),
            methodology,
        ).unwrap();
        
        assert_eq!(aggregate.get_id(), id);
        assert_eq!(aggregate.get_version(), 1);
        assert_eq!(aggregate.state.status, WorkflowStatus::Created);
        assert_eq!(aggregate.get_uncommitted_events().len(), 1);
    }

    #[test]
    fn test_workflow_state_transitions() {
        let id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec![],
            ai_agents: vec![],
            estimated_duration_minutes: 10,
        };
        
        let mut aggregate = ResearchWorkflowAggregate::create_workflow(
            id,
            "Test".to_string(),
            "Query".to_string(),
            methodology,
        ).unwrap();
        
        // Start execution
        aggregate.start_execution().unwrap();
        assert_eq!(aggregate.state.status, WorkflowStatus::Running);
        
        // Create task
        let task_id = Uuid::new_v4();
        aggregate.create_task(task_id, "search".to_string(), Some("researcher".to_string())).unwrap();
        assert_eq!(aggregate.state.tasks.len(), 1);
        
        // Complete task
        let results = serde_json::json!({"findings": "test results"});
        aggregate.complete_task(task_id, results).unwrap();
        
        let task = aggregate.get_task(task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[test]
    fn test_invalid_operations() {
        let id = Uuid::new_v4();
        
        // Empty name should fail
        let result = ResearchWorkflowAggregate::create_workflow(
            id,
            "".to_string(),
            "Query".to_string(),
            ResearchMethodology {
                name: "Test".to_string(),
                steps: vec![],
                ai_agents: vec![],
                estimated_duration_minutes: 10,
            },
        );
        assert!(result.is_err());
        
        // Empty query should fail
        let result = ResearchWorkflowAggregate::create_workflow(
            id,
            "Name".to_string(),
            "".to_string(),
            ResearchMethodology {
                name: "Test".to_string(),
                steps: vec![],
                ai_agents: vec![],
                estimated_duration_minutes: 10,
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregate_validation() {
        let id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Test".to_string(),
            steps: vec![],
            ai_agents: vec![],
            estimated_duration_minutes: 10,
        };
        
        let aggregate = ResearchWorkflowAggregate::create_workflow(
            id,
            "Test Workflow".to_string(),
            "Test Query".to_string(),
            methodology,
        ).unwrap();
        
        // Should be valid
        assert!(aggregate.validate().is_ok());
        
        // Test invalid state
        let mut invalid_state = aggregate.state.clone();
        invalid_state.name = "".to_string();
        
        let invalid_aggregate = ResearchWorkflowAggregate::restore_from_state(
            id, invalid_state, 1
        );
        
        assert!(invalid_aggregate.validate().is_err());
    }
}
