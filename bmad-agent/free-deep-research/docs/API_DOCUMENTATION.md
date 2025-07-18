# Free Deep Research System - API Documentation

## Overview

The Free Deep Research System provides comprehensive APIs for all major functionality across versions 1.1.0, 1.2.0, and 2.0.0. All APIs are implemented as Tauri commands for the desktop application and REST endpoints for distributed deployments.

## Authentication

### API Key Authentication
```rust
// All API calls require authentication
Authorization: Bearer <api_key>
Content-Type: application/json
```

### Session-based Authentication (Enterprise)
```rust
// Enterprise features use session-based auth
Cookie: session_id=<session_id>
X-CSRF-Token: <csrf_token>
```

---

## Version 1.1.0 APIs

### Enhanced AI Models

#### Get Latest AI Models
```rust
#[tauri::command]
pub async fn get_latest_ai_models() -> AppResult<Vec<String>>
```

**Response:**
```json
[
  "anthropic/claude-3.5-sonnet",
  "openai/gpt-4-turbo",
  "openai/gpt-4o",
  "google/gemini-1.5-pro",
  "meta-llama/llama-3.1-405b"
]
```

#### Get Model Recommendations
```rust
#[tauri::command]
pub async fn get_model_recommendations(
    use_case: String,
    budget_limit: Option<f64>
) -> AppResult<Vec<ModelRecommendation>>
```

**Request:**
```json
{
  "use_case": "research_analysis",
  "budget_limit": 0.01
}
```

**Response:**
```json
[
  {
    "model_id": "anthropic/claude-3.5-sonnet",
    "confidence_score": 0.95,
    "reasoning": "Excellent for research analysis with high accuracy",
    "estimated_cost": 0.003,
    "expected_performance": {
      "response_time_ms": 2000,
      "quality_score": 0.92,
      "success_rate": 0.98
    }
  }
]
```

### Advanced Analytics

#### Get Analytics Dashboard
```rust
#[tauri::command]
pub async fn get_analytics_dashboard() -> AppResult<serde_json::Value>
```

**Response:**
```json
{
  "system_metrics": {
    "cpu_usage": 45.2,
    "memory_usage": 67.8,
    "active_sessions": 12,
    "requests_per_minute": 150
  },
  "usage_analytics": {
    "total_workflows": 1250,
    "completed_today": 45,
    "success_rate": 0.94,
    "average_duration_minutes": 8.5
  },
  "performance_metrics": {
    "bottlenecks": ["api_rate_limits", "model_selection"],
    "optimization_score": 0.87,
    "recommendations": ["Increase cache size", "Optimize model selection"]
  }
}
```

### Enhanced Export

#### Export Workflows Enhanced
```rust
#[tauri::command]
pub async fn export_workflows_enhanced(
    workflow_ids: Vec<String>,
    format: String,
    destination_path: String,
    filename: String,
    options: serde_json::Value
) -> AppResult<EnhancedExportResult>
```

**Request:**
```json
{
  "workflow_ids": ["uuid1", "uuid2"],
  "format": "pdf",
  "destination_path": "/exports",
  "filename": "research_report.pdf",
  "options": {
    "include_charts": true,
    "include_metadata": true,
    "watermark": "Confidential",
    "password_protect": "secure123"
  }
}
```

### Collaboration

#### Start Collaboration Session
```rust
#[tauri::command]
pub async fn start_collaboration_session(
    user_id: String,
    workflow_id: String,
    session_type: String
) -> AppResult<CollaborationSession>
```

### Mobile API

#### Get Mobile Dashboard
```rust
#[tauri::command]
pub async fn get_mobile_dashboard(
    user_id: String
) -> AppResult<MobileDashboard>
```

---

## Version 1.2.0 APIs

### Plugin System

#### Install Plugin
```rust
#[tauri::command]
pub async fn install_plugin(
    plugin_id: String,
    user_id: String
) -> AppResult<String>
```

**Request:**
```json
{
  "plugin_id": "data-visualizer-pro",
  "user_id": "user123"
}
```

#### Execute Plugin
```rust
#[tauri::command]
pub async fn execute_plugin(
    plugin_id: String,
    context: serde_json::Value
) -> AppResult<serde_json::Value>
```

**Request:**
```json
{
  "plugin_id": "data-processor",
  "context": {
    "input_data": {...},
    "parameters": {...},
    "user_preferences": {...}
  }
}
```

### Workflow Engine

#### Create Workflow
```rust
#[tauri::command]
pub async fn create_workflow(
    user_id: String,
    name: String,
    description: Option<String>
) -> AppResult<serde_json::Value>
```

#### Execute Workflow
```rust
#[tauri::command]
pub async fn execute_workflow(
    workflow_id: String,
    user_id: String,
    parameters: serde_json::Value
) -> AppResult<String>
```

**Request:**
```json
{
  "workflow_id": "workflow123",
  "user_id": "user123",
  "parameters": {
    "research_query": "AI in healthcare",
    "methodology": "systematic_review",
    "output_format": "comprehensive"
  }
}
```

### Machine Learning Engine

#### Train ML Model
```rust
#[tauri::command]
pub async fn train_ml_model(
    model_name: String,
    model_type: String,
    user_id: String
) -> AppResult<String>
```

#### Make ML Prediction
```rust
#[tauri::command]
pub async fn make_ml_prediction(
    model_id: String,
    features: serde_json::Value
) -> AppResult<serde_json::Value>
```

**Request:**
```json
{
  "model_id": "research-optimizer-v1",
  "features": {
    "query_complexity": 0.8,
    "domain_specificity": 0.6,
    "time_constraint": 0.3,
    "quality_requirement": 0.9
  }
}
```

**Response:**
```json
{
  "prediction": {
    "recommended_methodology": "hybrid_approach",
    "estimated_duration": 12.5,
    "confidence_score": 0.87,
    "resource_requirements": {
      "api_calls": 45,
      "processing_time": 8.2
    }
  }
}
```

### Cloud Sync

#### Start Cloud Sync
```rust
#[tauri::command]
pub async fn start_cloud_sync(
    user_id: String,
    device_id: String,
    provider: String,
    sync_type: String
) -> AppResult<String>
```

### Enterprise Features

#### Create Enterprise User
```rust
#[tauri::command]
pub async fn create_enterprise_user(
    user_request: serde_json::Value,
    created_by: String
) -> AppResult<serde_json::Value>
```

#### Check Enterprise Access
```rust
#[tauri::command]
pub async fn check_enterprise_access(
    user_id: String,
    resource_type: String,
    resource_id: String,
    action: String
) -> AppResult<serde_json::Value>
```

---

## Version 2.0.0 APIs

### Distributed System

#### Join Cluster
```rust
#[tauri::command]
pub async fn join_cluster(
    node_config: serde_json::Value
) -> AppResult<()>
```

**Request:**
```json
{
  "node_name": "research-node-01",
  "ip_address": "10.0.1.100",
  "port": 8080,
  "node_type": "Worker",
  "capabilities": {
    "cpu_cores": 16,
    "memory_gb": 64,
    "storage_gb": 1000,
    "gpu_count": 2
  }
}
```

#### Deploy Service to Cluster
```rust
#[tauri::command]
pub async fn deploy_service_to_cluster(
    deployment_request: serde_json::Value
) -> AppResult<String>
```

**Request:**
```json
{
  "service_name": "research-processor",
  "service_version": "2.0.0",
  "replicas": 3,
  "container_image": "research-processor:2.0.0",
  "resource_requirements": {
    "cpu_request": "500m",
    "cpu_limit": "2000m",
    "memory_request": "1Gi",
    "memory_limit": "4Gi"
  },
  "ports": [
    {
      "name": "http",
      "port": 8080,
      "target_port": 8080,
      "protocol": "HTTP"
    }
  ]
}
```

### AI Orchestration

#### Register AI Agent
```rust
#[tauri::command]
pub async fn register_ai_agent(
    agent: serde_json::Value
) -> AppResult<()>
```

**Request:**
```json
{
  "agent_name": "research-analyzer",
  "agent_type": "AnalysisAgent",
  "capabilities": {
    "supported_tasks": ["Research", "Analysis"],
    "max_concurrent_tasks": 5,
    "processing_power": 0.8,
    "specialized_skills": ["nlp", "data_analysis", "visualization"]
  },
  "endpoint": "http://agent-service:8080",
  "version": "2.0.0"
}
```

#### Submit AI Task
```rust
#[tauri::command]
pub async fn submit_ai_task(
    task: serde_json::Value
) -> AppResult<String>
```

**Request:**
```json
{
  "task_name": "analyze_research_data",
  "task_type": "Analysis",
  "priority": "High",
  "input_data": {
    "research_results": [...],
    "analysis_type": "sentiment_analysis",
    "output_format": "structured"
  },
  "constraints": {
    "max_execution_time_seconds": 300,
    "required_capabilities": ["nlp", "sentiment_analysis"]
  }
}
```

### Real-time Collaboration

#### Start Real-time Collaboration
```rust
#[tauri::command]
pub async fn start_realtime_collaboration(
    collaboration_request: serde_json::Value
) -> AppResult<String>
```

**Request:**
```json
{
  "user_id": "user123",
  "document_id": "doc456",
  "session_type": "Edit",
  "permissions": {
    "can_read": true,
    "can_write": true,
    "can_comment": true,
    "can_share": false
  },
  "collaboration_mode": "RealTime"
}
```

#### Send Collaboration Message
```rust
#[tauri::command]
pub async fn send_collaboration_message(
    message: serde_json::Value
) -> AppResult<()>
```

**Request:**
```json
{
  "session_id": "session789",
  "user_id": "user123",
  "content": "What do you think about this analysis?",
  "message_type": "Text",
  "mentions": ["user456"],
  "reply_to": null
}
```

---

## WebSocket APIs (Real-time Features)

### Connection
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');
```

### Message Types
```json
{
  "type": "document_operation",
  "data": {
    "operation_id": "op123",
    "document_id": "doc456",
    "operation_type": "Insert",
    "position": 100,
    "content": "New text content",
    "user_id": "user123"
  }
}
```

```json
{
  "type": "cursor_position",
  "data": {
    "user_id": "user123",
    "document_id": "doc456",
    "position": {
      "line": 10,
      "column": 25,
      "offset": 250
    }
  }
}
```

```json
{
  "type": "user_presence",
  "data": {
    "user_id": "user123",
    "session_id": "session789",
    "presence_state": "Online",
    "activity_indicator": "Typing"
  }
}
```

---

## Error Handling

### Standard Error Response
```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "The provided parameters are invalid",
    "details": {
      "field": "user_id",
      "reason": "Invalid UUID format"
    },
    "timestamp": "2025-01-15T10:30:00Z",
    "request_id": "req_123456"
  }
}
```

### Error Codes
- `INVALID_REQUEST`: Malformed request parameters
- `AUTHENTICATION_FAILED`: Invalid credentials or expired session
- `PERMISSION_DENIED`: Insufficient permissions for operation
- `RESOURCE_NOT_FOUND`: Requested resource does not exist
- `RESOURCE_LIMIT_EXCEEDED`: Rate limit or quota exceeded
- `INTERNAL_ERROR`: Unexpected server error
- `SERVICE_UNAVAILABLE`: Service temporarily unavailable

---

## Rate Limiting

### Headers
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1642694400
X-RateLimit-Retry-After: 60
```

### Limits by Tier
- **Free Tier**: 100 requests/hour
- **Pro Tier**: 1,000 requests/hour
- **Enterprise Tier**: 10,000 requests/hour
- **Unlimited Tier**: No limits

---

## SDK Examples

### JavaScript/TypeScript
```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Get latest AI models
const models = await invoke('get_latest_ai_models');

// Execute workflow
const executionId = await invoke('execute_workflow', {
  workflowId: 'workflow123',
  userId: 'user123',
  parameters: {
    query: 'AI in healthcare',
    methodology: 'systematic_review'
  }
});
```

### Python (REST API)
```python
import requests

# Authentication
headers = {
    'Authorization': 'Bearer your_api_key',
    'Content-Type': 'application/json'
}

# Start collaboration
response = requests.post(
    'https://api.freeresearch.ai/v2/collaboration/start',
    headers=headers,
    json={
        'user_id': 'user123',
        'document_id': 'doc456',
        'session_type': 'Edit'
    }
)
```

### Rust
```rust
use serde_json::json;

// Using the service directly
let service_manager = ServiceManager::new().await?;
let result = service_manager
    .workflow_engine
    .read()
    .await
    .execute_workflow(request)
    .await?;
```

---

## Webhooks

### Configuration
```json
{
  "webhook_url": "https://your-app.com/webhooks/research",
  "events": [
    "workflow.completed",
    "collaboration.session_started",
    "export.finished"
  ],
  "secret": "your_webhook_secret"
}
```

### Event Payload
```json
{
  "event": "workflow.completed",
  "timestamp": "2025-01-15T10:30:00Z",
  "data": {
    "workflow_id": "workflow123",
    "user_id": "user123",
    "status": "completed",
    "duration_ms": 45000,
    "result_summary": "Research completed successfully"
  },
  "signature": "sha256=..."
}
```
