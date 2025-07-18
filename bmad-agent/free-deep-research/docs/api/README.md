# Free Deep Research System - API Documentation

## Overview

The Free Deep Research System provides a comprehensive API for managing research workflows, API keys, templates, and system monitoring. This documentation covers all available endpoints, data structures, and usage examples.

## Table of Contents

1. [Authentication & Security](#authentication--security)
2. [API Key Management](#api-key-management)
3. [Research Workflows](#research-workflows)
4. [Template Management](#template-management)
5. [System Monitoring](#system-monitoring)
6. [Error Handling](#error-handling)
7. [Rate Limiting](#rate-limiting)
8. [Examples](#examples)

## Authentication & Security

### Security Headers

All API requests should include appropriate security headers:

```typescript
const headers = {
  'Content-Type': 'application/json',
  'X-CSRF-Token': csrfToken,
  'Authorization': `Bearer ${sessionToken}`
}
```

### Rate Limiting

- **Default Limit**: 100 requests per minute per session
- **Burst Limit**: 10 requests per second
- **Headers**: Rate limit information is returned in response headers

```typescript
// Rate limit headers in response
'X-RateLimit-Limit': '100'
'X-RateLimit-Remaining': '95'
'X-RateLimit-Reset': '1642694400'
```

## API Key Management

### Data Structures

```typescript
interface ApiKey {
  id: string
  name: string
  service: 'openrouter' | 'serpapi' | 'jina' | 'firecrawl'
  encrypted_key: string
  created_at: string
  last_used?: string
  is_active: boolean
  usage_count: number
  rate_limit: number
  rate_limit_window: number
  current_usage: number
  last_reset: string
  metadata: Record<string, any>
  tags: string[]
  description?: string
}

interface CreateApiKeyRequest {
  name: string
  service: ServiceProvider
  key: string
  description?: string
  rate_limit?: number
  tags?: string[]
}
```

### Endpoints

#### Get All API Keys
```typescript
await invoke<ApiKey[]>('get_api_keys')
```

**Response**: Array of API key objects

#### Create API Key
```typescript
await invoke<ApiKey>('add_api_key', { 
  key: CreateApiKeyRequest 
})
```

**Parameters**:
- `key`: CreateApiKeyRequest object

**Response**: Created API key object

#### Update API Key
```typescript
await invoke<ApiKey>('update_api_key', { 
  id: string, 
  updates: Partial<ApiKey> 
})
```

**Parameters**:
- `id`: API key ID
- `updates`: Partial API key object with fields to update

#### Delete API Key
```typescript
await invoke('delete_api_key', { id: string })
```

**Parameters**:
- `id`: API key ID to delete

#### Test API Key
```typescript
await invoke<ApiKeyTestResult>('test_api_key', { id: string })
```

**Response**:
```typescript
interface ApiKeyTestResult {
  success: boolean
  response_time_ms: number
  status_code: number
  error_message?: string
  tested_at: string
}
```

## Research Workflows

### Data Structures

```typescript
interface ResearchWorkflow {
  id: string
  name: string
  description: string
  status: 'idle' | 'running' | 'paused' | 'completed' | 'failed' | 'cancelled'
  progress: number
  created_at: string
  updated_at: string
  started_at?: string
  completed_at?: string
  error_message?: string
  methodology: 'don_lim' | 'nick_scamara' | 'hybrid' | 'custom'
  query: string
  parameters: WorkflowParameters
  results?: WorkflowResults
  metadata: Record<string, any>
}

interface WorkflowParameters {
  max_results?: number
  search_depth?: 'basic' | 'comprehensive' | 'deep'
  include_academic?: boolean
  include_news?: boolean
  date_range?: string
  custom_filters?: Record<string, any>
}
```

### Endpoints

#### Create Research Workflow
```typescript
await invoke<ResearchWorkflow>('create_research_workflow', { 
  workflow: CreateWorkflowRequest 
})
```

#### Get All Workflows
```typescript
await invoke<ResearchWorkflow[]>('get_all_research_workflows')
```

#### Get Workflow by ID
```typescript
await invoke<ResearchWorkflow>('get_research_workflow', { 
  workflowId: string 
})
```

#### Start Workflow
```typescript
await invoke<ResearchWorkflow>('start_research_workflow', { 
  workflowId: string 
})
```

#### Pause Workflow
```typescript
await invoke<ResearchWorkflow>('pause_research_workflow', { 
  workflowId: string 
})
```

#### Cancel Workflow
```typescript
await invoke<ResearchWorkflow>('cancel_research_workflow', { 
  workflowId: string 
})
```

#### Get Workflow Progress
```typescript
await invoke<WorkflowProgress>('get_workflow_progress', { 
  workflowId: string 
})
```

**Response**:
```typescript
interface WorkflowProgress {
  workflow_id: string
  current_step: number
  total_steps: number
  progress_percentage: number
  current_step_name: string
  estimated_completion?: string
  steps_completed: string[]
  current_step_details?: Record<string, any>
}
```

## Template Management

### Data Structures

```typescript
interface ResearchTemplate {
  id: string
  name: string
  description: string
  category: string
  version: string
  author: string
  is_public: boolean
  is_featured: boolean
  rating: number
  usage_count: number
  parameters: TemplateParameter[]
  workflow_steps: TemplateStep[]
  created_at: string
  updated_at: string
  tags: string[]
}
```

### Endpoints

#### Get All Templates
```typescript
await invoke<ResearchTemplate[]>('get_all_research_templates')
```

#### Get Template by ID
```typescript
await invoke<ResearchTemplate>('get_research_template', { 
  templateId: string 
})
```

#### Create Template
```typescript
await invoke<ResearchTemplate>('create_research_template', { 
  template: CreateTemplateRequest 
})
```

#### Execute Template
```typescript
await invoke('execute_research_template', { 
  context: TemplateExecutionContext 
})
```

## System Monitoring

### Health Check
```typescript
await invoke<string>('health_check')
```

### Service Health
```typescript
await invoke<ServiceHealthStatus>('get_service_health')
```

**Response**:
```typescript
interface ServiceHealthStatus {
  security: 'Healthy' | 'Unhealthy' | 'Unknown'
  data_persistence: 'Healthy' | 'Unhealthy' | 'Unknown'
  monitoring: 'Healthy' | 'Unhealthy' | 'Unknown'
  api_manager: 'Healthy' | 'Unhealthy' | 'Unknown'
  research_engine: 'Healthy' | 'Unhealthy' | 'Unknown'
}
```

### System Metrics
```typescript
await invoke<SystemMetrics>('get_system_metrics')
```

### Queue Statistics
```typescript
await invoke<QueueStatistics>('get_queue_statistics')
```

## Error Handling

### Error Response Format

All API errors follow a consistent format:

```typescript
interface ApiError {
  error: string
  message: string
  code?: string
  details?: Record<string, any>
  timestamp: string
}
```

### Common Error Codes

- `INVALID_INPUT`: Input validation failed
- `NOT_FOUND`: Resource not found
- `UNAUTHORIZED`: Authentication required
- `FORBIDDEN`: Insufficient permissions
- `RATE_LIMITED`: Rate limit exceeded
- `INTERNAL_ERROR`: Server error

### Error Handling Example

```typescript
try {
  const result = await invoke('get_api_keys')
  return result
} catch (error) {
  if (error.message.includes('RATE_LIMITED')) {
    // Handle rate limiting
    await new Promise(resolve => setTimeout(resolve, 1000))
    return retry()
  } else if (error.message.includes('UNAUTHORIZED')) {
    // Handle authentication
    redirectToLogin()
  } else {
    // Handle other errors
    showErrorMessage(error.message)
  }
}
```

## Examples

### Complete Workflow Example

```typescript
import { invoke } from '@tauri-apps/api/core'

// 1. Create and test API key
const apiKey = await invoke('add_api_key', {
  key: {
    name: 'OpenRouter Research Key',
    service: 'openrouter',
    key: 'your-api-key-here',
    description: 'For AI research workflows'
  }
})

const testResult = await invoke('test_api_key', { id: apiKey.id })
if (!testResult.success) {
  throw new Error('API key test failed')
}

// 2. Create research workflow
const workflow = await invoke('create_research_workflow', {
  workflow: {
    name: 'AI Healthcare Research',
    description: 'Research on AI applications in healthcare',
    methodology: 'hybrid',
    query: 'artificial intelligence healthcare applications 2024',
    parameters: {
      max_results: 100,
      search_depth: 'comprehensive',
      include_academic: true,
      include_news: true,
      date_range: '1y'
    }
  }
})

// 3. Start workflow
await invoke('start_research_workflow', { workflowId: workflow.id })

// 4. Monitor progress
const checkProgress = async () => {
  const progress = await invoke('get_workflow_progress', { 
    workflowId: workflow.id 
  })
  
  console.log(`Progress: ${progress.progress_percentage}%`)
  console.log(`Current step: ${progress.current_step_name}`)
  
  if (progress.progress_percentage < 100) {
    setTimeout(checkProgress, 5000) // Check every 5 seconds
  } else {
    const results = await invoke('get_workflow_results', { 
      workflowId: workflow.id 
    })
    console.log('Workflow completed:', results)
  }
}

checkProgress()
```

### Security Best Practices

```typescript
import { useSecurity } from '@/hooks/useSecurity'

function SecureComponent() {
  const { secureInvoke, validateInput, sanitize } = useSecurity()
  
  const handleApiKeyCreation = async (formData: any) => {
    // 1. Validate input
    const validation = validateInput.apiKey(formData.key)
    if (!validation.isValid) {
      throw new Error(validation.errors.join(', '))
    }
    
    // 2. Sanitize input
    const sanitizedData = {
      name: sanitize.string(formData.name, 100),
      service: formData.service,
      key: validation.sanitized,
      description: sanitize.string(formData.description, 500)
    }
    
    // 3. Make secure API call
    const result = await secureInvoke('add_api_key', 
      { key: sanitizedData },
      {
        rateLimitKey: 'api-key-creation',
        auditEvent: {
          event_type: 'api_key_created',
          action: 'create_api_key',
          risk_level: 'medium'
        }
      }
    )
    
    return result
  }
}
```

## Support

For additional support or questions:

- **Documentation**: Check the `/docs` directory for detailed guides
- **Issues**: Report bugs and feature requests on GitHub
- **Security**: Report security issues privately to the maintainers

## Changelog

### v1.0.0 (2025-07-18)
- Initial API documentation
- Complete endpoint coverage
- Security guidelines
- Usage examples
