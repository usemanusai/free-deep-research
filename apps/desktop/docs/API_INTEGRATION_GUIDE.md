# API Integration Guide

## Overview

This guide explains how the Free Deep Research System integrates with external APIs and how the frontend communicates with the Tauri backend.

## Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   React Frontend │    │  Tauri Backend  │    │  External APIs  │
│                 │    │                 │    │                 │
│  - UI Components│◄──►│  - API Manager  │◄──►│  - OpenRouter   │
│  - State Mgmt   │    │  - Research Eng │    │  - SerpApi      │
│  - React Query  │    │  - Data Store   │    │  - Jina AI      │
│  - Config Bridge│    │  - Security     │    │  - Firecrawl    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Frontend-Backend Communication

### Tauri Commands

The frontend communicates with the Rust backend through Tauri commands:

```typescript
import { invoke } from '@tauri-apps/api/core'

// Example: Get all API keys
const apiKeys = await invoke<ApiKey[]>('get_api_keys')

// Example: Create research workflow
const workflowId = await invoke<string>('create_research_workflow', {
  name: 'My Research',
  query: 'AI in healthcare',
  methodology: 'hybrid'
})
```

### Available Commands

#### API Management Commands
```typescript
// Key management
get_api_keys() -> ApiKey[]
add_api_key(key: ApiKeyData) -> ApiKey
update_api_key(id: string, updates: Partial<ApiKey>) -> ApiKey
delete_api_key(id: string) -> void
test_api_key(id: string) -> TestResult

// Rate limiting
can_make_request(service: string) -> boolean
record_api_request(service: string, keyId: string) -> void
get_key_usage_status(keyId: string) -> UsageStatus

// Key rotation
select_best_key_for_service(service: string) -> ApiKey
record_key_performance(keyId: string, metrics: PerformanceMetrics) -> void
```

#### Research Workflow Commands
```typescript
// Workflow management
create_research_workflow(data: CreateWorkflowData) -> ResearchWorkflow
start_research_workflow(workflowId: string) -> void
pause_research_workflow(workflowId: string) -> void
cancel_research_workflow(workflowId: string) -> void
get_research_workflow(workflowId: string) -> ResearchWorkflow
get_all_research_workflows() -> ResearchWorkflow[]

// Progress monitoring
get_workflow_progress(workflowId: string) -> ProgressData
get_workflow_results(workflowId: string) -> ResearchResults
```

#### Configuration Commands
```typescript
get_configuration() -> Configuration
update_configuration(config: Configuration) -> void
reset_configuration() -> void
```

#### Monitoring Commands
```typescript
get_system_metrics() -> SystemMetrics
get_service_health() -> ServiceHealthStatus
get_audit_logs(limit?: number) -> AuditEvent[]
```

## External API Integration

### OpenRouter.ai Integration

```rust
// Rust backend implementation
pub async fn make_openrouter_request(
    prompt: &str,
    model: &str,
    api_key: &str
) -> Result<OpenRouterResponse, ApiError> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()
        .await?;
    
    Ok(response.json().await?)
}
```

### SerpApi Integration

```rust
pub async fn search_serpapi(
    query: &str,
    api_key: &str
) -> Result<SerpApiResponse, ApiError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://serpapi.com/search")
        .query(&[
            ("q", query),
            ("api_key", api_key),
            ("engine", "google")
        ])
        .send()
        .await?;
    
    Ok(response.json().await?)
}
```

### Jina AI Integration

```rust
pub async fn process_with_jina(
    content: &str,
    api_key: &str
) -> Result<JinaResponse, ApiError> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.jina.ai/v1/embeddings")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "input": content,
            "model": "jina-embeddings-v2-base-en"
        }))
        .send()
        .await?;
    
    Ok(response.json().await?)
}
```

### Firecrawl Integration

```rust
pub async fn crawl_with_firecrawl(
    url: &str,
    api_key: &str
) -> Result<FirecrawlResponse, ApiError> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.firecrawl.dev/v0/scrape")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "url": url,
            "formats": ["markdown", "html"]
        }))
        .send()
        .await?;
    
    Ok(response.json().await?)
}
```

## Rate Limiting Strategy

### Implementation

```rust
pub struct RateLimiter {
    limits: HashMap<String, RateLimit>,
    usage: HashMap<String, Usage>,
}

impl RateLimiter {
    pub fn can_make_request(&self, service: &str, key_id: &str) -> bool {
        let key = format!("{}:{}", service, key_id);
        
        if let Some(usage) = self.usage.get(&key) {
            if let Some(limit) = self.limits.get(service) {
                return usage.current_count < limit.max_requests;
            }
        }
        
        true
    }
    
    pub fn record_request(&mut self, service: &str, key_id: &str) {
        let key = format!("{}:{}", service, key_id);
        let usage = self.usage.entry(key).or_insert(Usage::new());
        usage.current_count += 1;
        usage.last_request = Utc::now();
    }
}
```

### Frontend Usage

```typescript
// Check if request can be made
const canMakeRequest = await invoke<boolean>('can_make_request', {
  service: 'openrouter'
})

if (canMakeRequest) {
  // Make the request
  const result = await invoke('make_service_request', {
    service: 'openrouter',
    endpoint: 'chat/completions',
    data: requestData
  })
  
  // Record the request
  await invoke('record_api_request', {
    service: 'openrouter',
    keyId: selectedKeyId
  })
}
```

## Error Handling

### Backend Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Rate limit exceeded for service: {service}")]
    RateLimitExceeded { service: String },
    
    #[error("API key invalid or expired: {key_id}")]
    InvalidApiKey { key_id: String },
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
}
```

### Frontend Error Handling

```typescript
try {
  const result = await invoke('start_research_workflow', { workflowId })
} catch (error) {
  if (error.includes('Rate limit exceeded')) {
    // Handle rate limiting
    showNotification('Rate limit reached. Please wait.', 'warning')
  } else if (error.includes('Invalid API key')) {
    // Handle invalid API key
    showNotification('API key is invalid. Please check configuration.', 'error')
  } else {
    // Handle general errors
    showNotification('An unexpected error occurred.', 'error')
  }
}
```

## State Management

### React Query Integration

```typescript
// Custom hook for API keys
export function useApiKeys() {
  return useQuery({
    queryKey: ['api-keys'],
    queryFn: () => invoke<ApiKey[]>('get_api_keys'),
    refetchInterval: 30000, // Refetch every 30 seconds
  })
}

// Custom hook for research workflows
export function useResearchWorkflows() {
  return useQuery({
    queryKey: ['research-workflows'],
    queryFn: () => invoke<ResearchWorkflow[]>('get_all_research_workflows'),
    refetchInterval: 15000,
  })
}
```

### Configuration Bridge

```typescript
import { configBridge } from '@/services/configBridge'

// Get configuration value
const apiTimeout = await configBridge.getConfig('api.timeout', 30000)

// Set configuration value
await configBridge.setConfig('api.timeout', 45000)

// Subscribe to changes
const unsubscribe = configBridge.subscribe('api.timeout', (newValue) => {
  console.log('API timeout changed to:', newValue)
})
```

## Security Considerations

### API Key Encryption

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

pub fn encrypt_api_key(key: &str, encryption_key: &[u8]) -> Result<String, CryptoError> {
    let cipher = Aes256Gcm::new(Key::from_slice(encryption_key));
    let nonce = Nonce::from_slice(b"unique nonce"); // Use proper nonce generation
    
    let ciphertext = cipher.encrypt(nonce, key.as_bytes())?;
    Ok(base64::encode(ciphertext))
}
```

### Secure Storage

```rust
pub struct SecureStorage {
    encryption_key: [u8; 32],
    db_path: PathBuf,
}

impl SecureStorage {
    pub fn store_api_key(&self, key_data: &ApiKey) -> Result<(), StorageError> {
        let encrypted_key = encrypt_api_key(&key_data.key, &self.encryption_key)?;
        // Store in encrypted database
        Ok(())
    }
}
```

## Testing

### Mock API Responses

```typescript
// For development/testing
if (process.env.MOCK_APIS === 'true') {
  // Use mock responses instead of real API calls
  const mockResponse = {
    id: 'mock-workflow-123',
    status: 'completed',
    results: mockResearchResults
  }
  return mockResponse
}
```

### Integration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_api_integration() {
        let api_manager = ApiManager::new();
        let result = api_manager.test_connection("openrouter", "test-key").await;
        assert!(result.is_ok());
    }
}
```

## Performance Optimization

### Request Batching

```rust
pub async fn batch_requests(
    requests: Vec<ApiRequest>
) -> Vec<Result<ApiResponse, ApiError>> {
    let futures: Vec<_> = requests
        .into_iter()
        .map(|req| async move { make_request(req).await })
        .collect();
    
    futures::future::join_all(futures).await
}
```

### Caching Strategy

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct ResponseCache {
    cache: HashMap<String, CachedResponse>,
    ttl: Duration,
}

impl ResponseCache {
    pub fn get(&self, key: &str) -> Option<&ApiResponse> {
        if let Some(cached) = self.cache.get(key) {
            if cached.timestamp.elapsed() < self.ttl {
                return Some(&cached.response);
            }
        }
        None
    }
}
```
