// Command Side Implementation for CQRS
// Phase 4.2: CQRS Pattern Implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

use super::error::{CQRSError, CQRSResult};
use crate::event_store::events::ResearchMethodology;

/// Base trait for all commands
pub trait Command: Send + Sync + std::fmt::Debug {
    /// Validate the command
    fn validate(&self) -> CQRSResult<()> {
        Ok(())
    }
    
    /// Get command name for logging/metrics
    fn command_name(&self) -> &'static str;
    
    /// Get command ID for tracking
    fn command_id(&self) -> Uuid;
    
    /// Get correlation ID for tracing
    fn correlation_id(&self) -> Option<Uuid> {
        None
    }
}

/// Command handler trait
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    /// Handle the command
    async fn handle(&self, command: C) -> CQRSResult<CommandResult>;
    
    /// Get handler name for logging
    fn handler_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command_id: Uuid,
    pub aggregate_id: Option<Uuid>,
    pub version: Option<u64>,
    pub success: bool,
    pub message: Option<String>,
    pub executed_at: DateTime<Utc>,
    pub execution_time_ms: u64,
}

impl CommandResult {
    pub fn success(command_id: Uuid, aggregate_id: Uuid, version: u64) -> Self {
        Self {
            command_id,
            aggregate_id: Some(aggregate_id),
            version: Some(version),
            success: true,
            message: None,
            executed_at: Utc::now(),
            execution_time_ms: 0,
        }
    }
    
    pub fn failure(command_id: Uuid, message: String) -> Self {
        Self {
            command_id,
            aggregate_id: None,
            version: None,
            success: false,
            message: Some(message),
            executed_at: Utc::now(),
            execution_time_ms: 0,
        }
    }
    
    pub fn with_execution_time(mut self, execution_time_ms: u64) -> Self {
        self.execution_time_ms = execution_time_ms;
        self
    }
}

/// Research workflow commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResearchWorkflowCommand {
    pub command_id: Uuid,
    pub workflow_id: Uuid,
    pub name: String,
    pub query: String,
    pub methodology: ResearchMethodology,
    pub correlation_id: Option<Uuid>,
}

impl Command for CreateResearchWorkflowCommand {
    fn validate(&self) -> CQRSResult<()> {
        if self.name.trim().is_empty() {
            return Err(CQRSError::ValidationError("Workflow name cannot be empty".to_string()));
        }
        if self.query.trim().is_empty() {
            return Err(CQRSError::ValidationError("Research query cannot be empty".to_string()));
        }
        if self.methodology.name.trim().is_empty() {
            return Err(CQRSError::ValidationError("Methodology name cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn command_name(&self) -> &'static str {
        "CreateResearchWorkflow"
    }
    
    fn command_id(&self) -> Uuid {
        self.command_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartWorkflowExecutionCommand {
    pub command_id: Uuid,
    pub workflow_id: Uuid,
    pub correlation_id: Option<Uuid>,
}

impl Command for StartWorkflowExecutionCommand {
    fn command_name(&self) -> &'static str {
        "StartWorkflowExecution"
    }
    
    fn command_id(&self) -> Uuid {
        self.command_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskCommand {
    pub command_id: Uuid,
    pub workflow_id: Uuid,
    pub task_id: Uuid,
    pub task_type: String,
    pub agent_type: Option<String>,
    pub correlation_id: Option<Uuid>,
}

impl Command for CreateTaskCommand {
    fn validate(&self) -> CQRSResult<()> {
        if self.task_type.trim().is_empty() {
            return Err(CQRSError::ValidationError("Task type cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn command_name(&self) -> &'static str {
        "CreateTask"
    }
    
    fn command_id(&self) -> Uuid {
        self.command_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteTaskCommand {
    pub command_id: Uuid,
    pub workflow_id: Uuid,
    pub task_id: Uuid,
    pub results: serde_json::Value,
    pub correlation_id: Option<Uuid>,
}

impl Command for CompleteTaskCommand {
    fn command_name(&self) -> &'static str {
        "CompleteTask"
    }
    
    fn command_id(&self) -> Uuid {
        self.command_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteWorkflowCommand {
    pub command_id: Uuid,
    pub workflow_id: Uuid,
    pub results: crate::event_store::events::ResearchResults,
    pub correlation_id: Option<Uuid>,
}

impl Command for CompleteWorkflowCommand {
    fn command_name(&self) -> &'static str {
        "CompleteWorkflow"
    }
    
    fn command_id(&self) -> Uuid {
        self.command_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailWorkflowCommand {
    pub command_id: Uuid,
    pub workflow_id: Uuid,
    pub error_message: String,
    pub correlation_id: Option<Uuid>,
}

impl Command for FailWorkflowCommand {
    fn validate(&self) -> CQRSResult<()> {
        if self.error_message.trim().is_empty() {
            return Err(CQRSError::ValidationError("Error message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn command_name(&self) -> &'static str {
        "FailWorkflow"
    }
    
    fn command_id(&self) -> Uuid {
        self.command_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

/// Command bus for routing commands to handlers
pub struct CommandBus {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    metrics: CommandBusMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct CommandBusMetrics {
    pub commands_executed: u64,
    pub commands_failed: u64,
    pub average_duration_ms: f64,
    pub total_duration_ms: u64,
}

impl CommandBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            metrics: CommandBusMetrics::default(),
        }
    }
    
    /// Register a command handler
    pub async fn register_handler<C, H>(&mut self, handler: H)
    where
        C: Command + 'static,
        H: CommandHandler<C> + 'static,
    {
        let type_id = TypeId::of::<C>();
        self.handlers.insert(type_id, Box::new(Arc::new(handler)));
    }
    
    /// Execute a command
    pub async fn execute<C>(&self, command: C) -> CQRSResult<CommandResult>
    where
        C: Command + 'static,
    {
        let start_time = Instant::now();
        let command_id = command.command_id();
        let command_name = command.command_name();
        
        // Find handler
        let type_id = TypeId::of::<C>();
        let handler = self.handlers.get(&type_id)
            .ok_or_else(|| CQRSError::HandlerNotFound(command_name.to_string()))?;
        
        // Cast handler to correct type
        let handler = handler
            .downcast_ref::<Arc<dyn CommandHandler<C>>>()
            .ok_or_else(|| CQRSError::HandlerCastError(command_name.to_string()))?;
        
        // Execute command
        let result = match handler.handle(command).await {
            Ok(mut result) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                result = result.with_execution_time(execution_time);
                
                // Update metrics
                self.update_metrics(true, execution_time).await;
                
                Ok(result)
            }
            Err(error) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                
                // Update metrics
                self.update_metrics(false, execution_time).await;
                
                // Return failure result
                Ok(CommandResult::failure(command_id, error.to_string())
                    .with_execution_time(execution_time))
            }
        };
        
        result
    }
    
    /// Update command bus metrics
    async fn update_metrics(&self, success: bool, execution_time_ms: u64) {
        // In a real implementation, you'd use atomic operations or a proper metrics system
        // For now, this is a placeholder
    }
    
    /// Get command bus metrics
    pub fn get_metrics(&self) -> &CommandBusMetrics {
        &self.metrics
    }
    
    /// Check if command bus is healthy
    pub async fn is_healthy(&self) -> bool {
        // Check if handlers are registered and responsive
        !self.handlers.is_empty()
    }
    
    /// Get registered command types
    pub fn get_registered_commands(&self) -> Vec<String> {
        self.handlers.keys()
            .map(|type_id| format!("{:?}", type_id))
            .collect()
    }
}

impl Default for CommandBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Command factory for creating commands with proper IDs
pub struct CommandFactory {
    correlation_id: Option<Uuid>,
}

impl CommandFactory {
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
    
    pub fn create_research_workflow(
        &self,
        workflow_id: Uuid,
        name: String,
        query: String,
        methodology: ResearchMethodology,
    ) -> CreateResearchWorkflowCommand {
        CreateResearchWorkflowCommand {
            command_id: Uuid::new_v4(),
            workflow_id,
            name,
            query,
            methodology,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn start_workflow_execution(&self, workflow_id: Uuid) -> StartWorkflowExecutionCommand {
        StartWorkflowExecutionCommand {
            command_id: Uuid::new_v4(),
            workflow_id,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn create_task(
        &self,
        workflow_id: Uuid,
        task_id: Uuid,
        task_type: String,
        agent_type: Option<String>,
    ) -> CreateTaskCommand {
        CreateTaskCommand {
            command_id: Uuid::new_v4(),
            workflow_id,
            task_id,
            task_type,
            agent_type,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn complete_task(
        &self,
        workflow_id: Uuid,
        task_id: Uuid,
        results: serde_json::Value,
    ) -> CompleteTaskCommand {
        CompleteTaskCommand {
            command_id: Uuid::new_v4(),
            workflow_id,
            task_id,
            results,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn complete_workflow(
        &self,
        workflow_id: Uuid,
        results: crate::event_store::events::ResearchResults,
    ) -> CompleteWorkflowCommand {
        CompleteWorkflowCommand {
            command_id: Uuid::new_v4(),
            workflow_id,
            results,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn fail_workflow(
        &self,
        workflow_id: Uuid,
        error_message: String,
    ) -> FailWorkflowCommand {
        FailWorkflowCommand {
            command_id: Uuid::new_v4(),
            workflow_id,
            error_message,
            correlation_id: self.correlation_id,
        }
    }
}

impl Default for CommandFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_store::events::ResearchMethodology;

    #[test]
    fn test_command_creation() {
        let factory = CommandFactory::new();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec!["Step 1".to_string()],
            ai_agents: vec!["agent1".to_string()],
            estimated_duration_minutes: 30,
        };
        
        let command = factory.create_research_workflow(
            Uuid::new_v4(),
            "Test Workflow".to_string(),
            "Test Query".to_string(),
            methodology,
        );
        
        assert_eq!(command.command_name(), "CreateResearchWorkflow");
        assert!(command.validate().is_ok());
    }

    #[test]
    fn test_command_validation() {
        let factory = CommandFactory::new();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec![],
            ai_agents: vec![],
            estimated_duration_minutes: 10,
        };
        
        // Valid command
        let valid_command = factory.create_research_workflow(
            Uuid::new_v4(),
            "Valid Name".to_string(),
            "Valid Query".to_string(),
            methodology.clone(),
        );
        assert!(valid_command.validate().is_ok());
        
        // Invalid command - empty name
        let invalid_command = factory.create_research_workflow(
            Uuid::new_v4(),
            "".to_string(),
            "Valid Query".to_string(),
            methodology,
        );
        assert!(invalid_command.validate().is_err());
    }

    #[tokio::test]
    async fn test_command_bus_creation() {
        let command_bus = CommandBus::new();
        assert!(command_bus.is_healthy().await);
        assert_eq!(command_bus.get_registered_commands().len(), 0);
    }

    #[test]
    fn test_command_result() {
        let command_id = Uuid::new_v4();
        let aggregate_id = Uuid::new_v4();
        
        let success_result = CommandResult::success(command_id, aggregate_id, 1);
        assert!(success_result.success);
        assert_eq!(success_result.aggregate_id, Some(aggregate_id));
        assert_eq!(success_result.version, Some(1));
        
        let failure_result = CommandResult::failure(command_id, "Test error".to_string());
        assert!(!failure_result.success);
        assert_eq!(failure_result.message, Some("Test error".to_string()));
    }
}
