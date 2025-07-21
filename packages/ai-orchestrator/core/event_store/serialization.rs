// Event Serialization for Event Store
// Phase 4.1: Event Sourcing Foundation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use super::error::{EventStoreError, EventStoreResult};
use super::events::{DomainEvent, EventMetadata, ResearchWorkflowEvent, AIAgentEvent};

/// Event serializer trait for pluggable serialization strategies
#[async_trait]
pub trait EventSerializer: Send + Sync {
    /// Serialize a domain event to JSON
    fn serialize(&self, event: &dyn DomainEvent) -> EventStoreResult<serde_json::Value>;
    
    /// Deserialize JSON to a domain event
    fn deserialize(
        &self,
        data: &serde_json::Value,
        event_type: &str,
        metadata: EventMetadata,
    ) -> EventStoreResult<Box<dyn DomainEvent>>;
    
    /// Get supported event types
    fn supported_event_types(&self) -> Vec<&'static str>;
    
    /// Validate event schema
    fn validate_schema(&self, event_type: &str, data: &serde_json::Value) -> EventStoreResult<()>;
}

/// JSON-based event serializer implementation
pub struct JsonEventSerializer {
    event_registry: HashMap<String, Box<dyn EventTypeHandler>>,
    schema_validator: Arc<SchemaValidator>,
}

impl JsonEventSerializer {
    pub fn new() -> Self {
        let mut serializer = Self {
            event_registry: HashMap::new(),
            schema_validator: Arc::new(SchemaValidator::new()),
        };
        
        // Register built-in event types
        serializer.register_event_type(Box::new(ResearchWorkflowEventHandler));
        serializer.register_event_type(Box::new(AIAgentEventHandler));
        
        serializer
    }
    
    /// Register a new event type handler
    pub fn register_event_type(&mut self, handler: Box<dyn EventTypeHandler>) {
        for event_type in handler.supported_types() {
            self.event_registry.insert(event_type.to_string(), handler.clone_box());
        }
    }
    
    /// Get event type handler
    fn get_handler(&self, event_type: &str) -> EventStoreResult<&dyn EventTypeHandler> {
        self.event_registry
            .get(event_type)
            .map(|h| h.as_ref())
            .ok_or_else(|| EventStoreError::invalid_event_type(event_type))
    }
}

#[async_trait]
impl EventSerializer for JsonEventSerializer {
    fn serialize(&self, event: &dyn DomainEvent) -> EventStoreResult<serde_json::Value> {
        // Validate event before serialization
        event.validate().map_err(|e| EventStoreError::schema_validation(e))?;
        
        // Serialize using the event's own method
        let data = event.serialize()?;
        
        // Validate schema
        self.validate_schema(event.event_type(), &data)?;
        
        Ok(data)
    }
    
    fn deserialize(
        &self,
        data: &serde_json::Value,
        event_type: &str,
        metadata: EventMetadata,
    ) -> EventStoreResult<Box<dyn DomainEvent>> {
        // Validate schema first
        self.validate_schema(event_type, data)?;
        
        // Get appropriate handler
        let handler = self.get_handler(event_type)?;
        
        // Deserialize using handler
        handler.deserialize(data, metadata)
    }
    
    fn supported_event_types(&self) -> Vec<&'static str> {
        self.event_registry.keys().map(|s| s.as_str()).collect()
    }
    
    fn validate_schema(&self, event_type: &str, data: &serde_json::Value) -> EventStoreResult<()> {
        self.schema_validator.validate(event_type, data)
    }
}

/// Event type handler trait for specific event types
pub trait EventTypeHandler: Send + Sync {
    /// Get supported event types
    fn supported_types(&self) -> Vec<&'static str>;
    
    /// Deserialize event data
    fn deserialize(
        &self,
        data: &serde_json::Value,
        metadata: EventMetadata,
    ) -> EventStoreResult<Box<dyn DomainEvent>>;
    
    /// Clone the handler (for registry storage)
    fn clone_box(&self) -> Box<dyn EventTypeHandler>;
}

/// Research workflow event handler
#[derive(Clone)]
pub struct ResearchWorkflowEventHandler;

impl EventTypeHandler for ResearchWorkflowEventHandler {
    fn supported_types(&self) -> Vec<&'static str> {
        vec![
            "research.workflow.created",
            "research.workflow.started",
            "research.task.created",
            "research.task.completed",
            "research.workflow.completed",
            "research.workflow.failed",
            "research.workflow.updated",
        ]
    }
    
    fn deserialize(
        &self,
        data: &serde_json::Value,
        _metadata: EventMetadata,
    ) -> EventStoreResult<Box<dyn DomainEvent>> {
        let event: ResearchWorkflowEvent = serde_json::from_value(data.clone())?;
        Ok(Box::new(event))
    }
    
    fn clone_box(&self) -> Box<dyn EventTypeHandler> {
        Box::new(self.clone())
    }
}

/// AI agent event handler
#[derive(Clone)]
pub struct AIAgentEventHandler;

impl EventTypeHandler for AIAgentEventHandler {
    fn supported_types(&self) -> Vec<&'static str> {
        vec![
            "ai.agent.created",
            "ai.agent.task_assigned",
            "ai.agent.response_generated",
            "ai.agent.error",
        ]
    }
    
    fn deserialize(
        &self,
        data: &serde_json::Value,
        _metadata: EventMetadata,
    ) -> EventStoreResult<Box<dyn DomainEvent>> {
        let event: AIAgentEvent = serde_json::from_value(data.clone())?;
        Ok(Box::new(event))
    }
    
    fn clone_box(&self) -> Box<dyn EventTypeHandler> {
        Box::new(self.clone())
    }
}

/// Schema validator for event data
pub struct SchemaValidator {
    schemas: HashMap<String, serde_json::Value>,
}

impl SchemaValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            schemas: HashMap::new(),
        };
        
        // Load built-in schemas
        validator.load_builtin_schemas();
        
        validator
    }
    
    /// Load built-in event schemas
    fn load_builtin_schemas(&mut self) {
        // Research workflow schemas
        self.schemas.insert(
            "research.workflow.created".to_string(),
            serde_json::json!({
                "type": "object",
                "required": ["workflow_id", "name", "query", "methodology", "created_at"],
                "properties": {
                    "workflow_id": {"type": "string", "format": "uuid"},
                    "name": {"type": "string", "minLength": 1, "maxLength": 255},
                    "query": {"type": "string", "minLength": 1, "maxLength": 2000},
                    "methodology": {"type": "object"},
                    "created_at": {"type": "string", "format": "date-time"},
                    "correlation_id": {"type": ["string", "null"], "format": "uuid"}
                }
            })
        );
        
        self.schemas.insert(
            "research.workflow.started".to_string(),
            serde_json::json!({
                "type": "object",
                "required": ["workflow_id", "started_at"],
                "properties": {
                    "workflow_id": {"type": "string", "format": "uuid"},
                    "started_at": {"type": "string", "format": "date-time"},
                    "correlation_id": {"type": ["string", "null"], "format": "uuid"}
                }
            })
        );
        
        self.schemas.insert(
            "research.task.created".to_string(),
            serde_json::json!({
                "type": "object",
                "required": ["workflow_id", "task_id", "task_type", "created_at"],
                "properties": {
                    "workflow_id": {"type": "string", "format": "uuid"},
                    "task_id": {"type": "string", "format": "uuid"},
                    "task_type": {"type": "string", "minLength": 1},
                    "agent_type": {"type": ["string", "null"]},
                    "created_at": {"type": "string", "format": "date-time"},
                    "correlation_id": {"type": ["string", "null"], "format": "uuid"}
                }
            })
        );
        
        // AI agent schemas
        self.schemas.insert(
            "ai.agent.created".to_string(),
            serde_json::json!({
                "type": "object",
                "required": ["agent_id", "agent_type", "configuration", "created_at"],
                "properties": {
                    "agent_id": {"type": "string", "format": "uuid"},
                    "agent_type": {"type": "string", "minLength": 1},
                    "configuration": {"type": "object"},
                    "created_at": {"type": "string", "format": "date-time"},
                    "correlation_id": {"type": ["string", "null"], "format": "uuid"}
                }
            })
        );
    }
    
    /// Validate event data against schema
    pub fn validate(&self, event_type: &str, data: &serde_json::Value) -> EventStoreResult<()> {
        // For now, implement basic validation
        // In production, you might want to use a proper JSON Schema validator like `jsonschema`
        
        if let Some(_schema) = self.schemas.get(event_type) {
            // Basic validation - check required fields exist
            if !data.is_object() {
                return Err(EventStoreError::schema_validation(
                    "Event data must be an object"
                ));
            }
            
            // Additional validation logic would go here
            // For now, we'll just ensure it's valid JSON
            Ok(())
        } else {
            // Unknown event type - allow it but log warning
            eprintln!("Warning: No schema found for event type: {}", event_type);
            Ok(())
        }
    }
    
    /// Add or update a schema
    pub fn add_schema(&mut self, event_type: String, schema: serde_json::Value) {
        self.schemas.insert(event_type, schema);
    }
}

/// Event versioning support for schema evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventVersion {
    pub event_type: String,
    pub version: u32,
    pub schema: serde_json::Value,
    pub migration_rules: Vec<MigrationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRule {
    pub from_version: u32,
    pub to_version: u32,
    pub transformation: String, // JSON path transformation rules
}

/// Event migration service for handling schema evolution
pub struct EventMigrationService {
    versions: HashMap<String, Vec<EventVersion>>,
}

impl EventMigrationService {
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }
    
    /// Register event version
    pub fn register_version(&mut self, version: EventVersion) {
        self.versions
            .entry(version.event_type.clone())
            .or_insert_with(Vec::new)
            .push(version);
    }
    
    /// Migrate event data to latest version
    pub fn migrate_to_latest(
        &self,
        event_type: &str,
        current_version: u32,
        data: serde_json::Value,
    ) -> EventStoreResult<serde_json::Value> {
        if let Some(versions) = self.versions.get(event_type) {
            let latest_version = versions.iter().map(|v| v.version).max().unwrap_or(1);
            
            if current_version < latest_version {
                // Apply migration rules
                self.apply_migrations(event_type, current_version, latest_version, data)
            } else {
                Ok(data)
            }
        } else {
            // No versions registered, return as-is
            Ok(data)
        }
    }
    
    fn apply_migrations(
        &self,
        event_type: &str,
        from_version: u32,
        to_version: u32,
        mut data: serde_json::Value,
    ) -> EventStoreResult<serde_json::Value> {
        // This is a simplified migration implementation
        // In production, you'd want more sophisticated transformation logic
        
        if let Some(versions) = self.versions.get(event_type) {
            for version in versions {
                for rule in &version.migration_rules {
                    if rule.from_version == from_version && rule.to_version <= to_version {
                        // Apply transformation (simplified)
                        data = self.apply_transformation(&data, &rule.transformation)?;
                    }
                }
            }
        }
        
        Ok(data)
    }
    
    fn apply_transformation(
        &self,
        data: &serde_json::Value,
        _transformation: &str,
    ) -> EventStoreResult<serde_json::Value> {
        // Simplified transformation - in production you'd implement JSON path transformations
        Ok(data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_store::events::*;
    use chrono::Utc;

    #[test]
    fn test_json_serializer_creation() {
        let serializer = JsonEventSerializer::new();
        let supported_types = serializer.supported_event_types();
        
        assert!(supported_types.contains(&"research.workflow.created"));
        assert!(supported_types.contains(&"ai.agent.created"));
    }

    #[test]
    fn test_event_serialization() {
        let serializer = JsonEventSerializer::new();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec!["Step 1".to_string()],
            ai_agents: vec!["agent1".to_string()],
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
        
        let serialized = serializer.serialize(&event).unwrap();
        assert!(serialized.is_object());
    }

    #[test]
    fn test_schema_validation() {
        let validator = SchemaValidator::new();
        
        let valid_data = serde_json::json!({
            "workflow_id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Test Workflow",
            "query": "Test Query",
            "methodology": {},
            "created_at": "2025-01-01T00:00:00Z"
        });
        
        assert!(validator.validate("research.workflow.created", &valid_data).is_ok());
        
        let invalid_data = serde_json::json!("not an object");
        assert!(validator.validate("research.workflow.created", &invalid_data).is_err());
    }
}
