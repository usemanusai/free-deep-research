# Free Deep Research System - Testing Guide

## Overview

This guide covers comprehensive testing strategies for all versions of the Free Deep Research System, including unit tests, integration tests, performance tests, and security tests.

## Testing Framework

### Rust Backend Testing
```toml
# Cargo.toml [dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"
wiremock = "0.5"
criterion = "0.5"
proptest = "1.4"
```

### Frontend Testing
```json
{
  "devDependencies": {
    "@testing-library/react": "^14.0.0",
    "@testing-library/jest-dom": "^6.1.0",
    "@testing-library/user-event": "^14.5.0",
    "jest": "^29.7.0",
    "cypress": "^13.6.0",
    "playwright": "^1.40.0"
  }
}
```

---

## Unit Testing

### Rust Backend Unit Tests

#### Service Layer Tests
```rust
// src/services/research_engine/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use tokio_test;

    #[tokio::test]
    async fn test_research_workflow_creation() {
        let mut mock_service = MockResearchEngineService::new();
        
        mock_service
            .expect_create_workflow()
            .with(eq("test_query"), eq("systematic_review"))
            .times(1)
            .returning(|_, _| {
                Ok(ResearchWorkflow {
                    id: Uuid::new_v4(),
                    name: "Test Workflow".to_string(),
                    status: WorkflowStatus::Created,
                    // ... other fields
                })
            });

        let result = mock_service
            .create_workflow("test_query".to_string(), "systematic_review".to_string())
            .await;

        assert!(result.is_ok());
        let workflow = result.unwrap();
        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.status, WorkflowStatus::Created);
    }

    #[tokio::test]
    async fn test_api_manager_rate_limiting() {
        let api_manager = ApiManagerService::new().await.unwrap();
        
        // Test rate limiting
        for i in 0..100 {
            let result = api_manager.make_request("test_provider", "test_endpoint").await;
            if i < 50 {
                assert!(result.is_ok());
            } else {
                // Should hit rate limit
                assert!(matches!(result, Err(ResearchError::RateLimitExceeded(_))));
            }
        }
    }

    #[tokio::test]
    async fn test_encryption_service() {
        let encryption_service = EncryptionService::new().await.unwrap();
        let test_data = "sensitive research data";
        
        // Test encryption
        let encrypted = encryption_service.encrypt(test_data.as_bytes()).await.unwrap();
        assert_ne!(encrypted, test_data.as_bytes());
        
        // Test decryption
        let decrypted = encryption_service.decrypt(&encrypted).await.unwrap();
        assert_eq!(String::from_utf8(decrypted).unwrap(), test_data);
    }
}
```

#### Model Tests
```rust
// src/models/research_workflow/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_workflow_serialization(
            name in "[a-zA-Z0-9 ]{1,100}",
            query in "[a-zA-Z0-9 ]{1,500}",
        ) {
            let workflow = ResearchWorkflow {
                id: Uuid::new_v4(),
                name,
                query,
                status: WorkflowStatus::Created,
                created_at: Utc::now(),
                // ... other fields
            };

            // Test serialization/deserialization
            let serialized = serde_json::to_string(&workflow).unwrap();
            let deserialized: ResearchWorkflow = serde_json::from_str(&serialized).unwrap();
            
            assert_eq!(workflow.id, deserialized.id);
            assert_eq!(workflow.name, deserialized.name);
            assert_eq!(workflow.query, deserialized.query);
        }
    }

    #[test]
    fn test_workflow_status_transitions() {
        let mut workflow = ResearchWorkflow::new("test".to_string(), "test query".to_string());
        
        // Test valid transitions
        assert!(workflow.transition_to(WorkflowStatus::Running).is_ok());
        assert!(workflow.transition_to(WorkflowStatus::Completed).is_ok());
        
        // Test invalid transitions
        assert!(workflow.transition_to(WorkflowStatus::Created).is_err());
    }
}
```

### Frontend Unit Tests

#### React Component Tests
```typescript
// src/components/WorkflowManager.test.tsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { WorkflowManager } from './WorkflowManager';
import { mockInvoke } from '../__mocks__/tauri';

jest.mock('@tauri-apps/api/tauri');

describe('WorkflowManager', () => {
  beforeEach(() => {
    mockInvoke.mockClear();
  });

  test('creates new workflow', async () => {
    mockInvoke.mockResolvedValueOnce({
      id: 'test-id',
      name: 'Test Workflow',
      status: 'Created'
    });

    render(<WorkflowManager />);
    
    const nameInput = screen.getByLabelText('Workflow Name');
    const queryInput = screen.getByLabelText('Research Query');
    const createButton = screen.getByText('Create Workflow');

    fireEvent.change(nameInput, { target: { value: 'Test Workflow' } });
    fireEvent.change(queryInput, { target: { value: 'AI in healthcare' } });
    fireEvent.click(createButton);

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('create_research_workflow', {
        name: 'Test Workflow',
        query: 'AI in healthcare'
      });
    });

    expect(screen.getByText('Workflow created successfully')).toBeInTheDocument();
  });

  test('handles workflow creation error', async () => {
    mockInvoke.mockRejectedValueOnce(new Error('API Error'));

    render(<WorkflowManager />);
    
    const createButton = screen.getByText('Create Workflow');
    fireEvent.click(createButton);

    await waitFor(() => {
      expect(screen.getByText('Error creating workflow')).toBeInTheDocument();
    });
  });
});
```

#### Hook Tests
```typescript
// src/hooks/useWorkflows.test.ts
import { renderHook, act } from '@testing-library/react';
import { useWorkflows } from './useWorkflows';
import { mockInvoke } from '../__mocks__/tauri';

describe('useWorkflows', () => {
  test('loads workflows on mount', async () => {
    const mockWorkflows = [
      { id: '1', name: 'Workflow 1', status: 'Completed' },
      { id: '2', name: 'Workflow 2', status: 'Running' }
    ];

    mockInvoke.mockResolvedValueOnce(mockWorkflows);

    const { result } = renderHook(() => useWorkflows());

    expect(result.current.loading).toBe(true);

    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 0));
    });

    expect(result.current.loading).toBe(false);
    expect(result.current.workflows).toEqual(mockWorkflows);
  });
});
```

---

## Integration Testing

### API Integration Tests
```rust
// tests/integration/api_tests.rs
use free_deep_research::services::ServiceManager;
use tokio_test;
use wiremock::{MockServer, Mock, ResponseTemplate};

#[tokio::test]
async fn test_openrouter_integration() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "choices": [{
                    "message": {
                        "content": "Test response"
                    }
                }]
            })))
        .mount(&mock_server)
        .await;

    let service_manager = ServiceManager::new().await.unwrap();
    
    // Configure to use mock server
    let mut api_manager = service_manager.api_manager.write().await;
    api_manager.set_base_url("openrouter", &mock_server.uri());

    let result = api_manager.make_request(
        "openrouter",
        "/api/v1/chat/completions",
        Some(json!({
            "model": "anthropic/claude-3.5-sonnet",
            "messages": [{"role": "user", "content": "test"}]
        }))
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_workflow_end_to_end() {
    let service_manager = ServiceManager::new().await.unwrap();
    
    // Create workflow
    let workflow = service_manager
        .research_engine
        .write()
        .await
        .create_workflow("test query".to_string(), "systematic_review".to_string())
        .await
        .unwrap();

    // Execute workflow
    let execution_id = service_manager
        .research_engine
        .write()
        .await
        .execute_workflow(workflow.id)
        .await
        .unwrap();

    // Wait for completion
    let mut attempts = 0;
    loop {
        let status = service_manager
            .research_engine
            .read()
            .await
            .get_workflow_status(workflow.id)
            .await
            .unwrap();

        if matches!(status, WorkflowStatus::Completed | WorkflowStatus::Failed) {
            break;
        }

        attempts += 1;
        if attempts > 30 {
            panic!("Workflow did not complete in time");
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // Verify results
    let results = service_manager
        .research_engine
        .read()
        .await
        .get_workflow_results(workflow.id)
        .await
        .unwrap();

    assert!(!results.is_empty());
}
```

### Database Integration Tests
```rust
// tests/integration/database_tests.rs
use free_deep_research::services::data_persistence::DataPersistenceService;
use tempfile::TempDir;

#[tokio::test]
async fn test_database_operations() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let data_service = DataPersistenceService::new(db_path.to_str().unwrap()).await.unwrap();
    
    // Test workflow storage
    let workflow = ResearchWorkflow::new("test".to_string(), "test query".to_string());
    data_service.store_workflow(&workflow).await.unwrap();
    
    let retrieved = data_service.get_workflow(workflow.id).await.unwrap();
    assert_eq!(retrieved.id, workflow.id);
    assert_eq!(retrieved.name, workflow.name);
    
    // Test workflow update
    let mut updated_workflow = retrieved;
    updated_workflow.status = WorkflowStatus::Completed;
    data_service.update_workflow(&updated_workflow).await.unwrap();
    
    let final_workflow = data_service.get_workflow(workflow.id).await.unwrap();
    assert_eq!(final_workflow.status, WorkflowStatus::Completed);
}
```

---

## End-to-End Testing

### Cypress Tests
```typescript
// cypress/e2e/workflow-management.cy.ts
describe('Workflow Management', () => {
  beforeEach(() => {
    cy.visit('/');
    cy.login('test@example.com', 'password');
  });

  it('creates and executes a workflow', () => {
    // Navigate to workflow creation
    cy.get('[data-testid="create-workflow-btn"]').click();
    
    // Fill workflow form
    cy.get('[data-testid="workflow-name"]').type('E2E Test Workflow');
    cy.get('[data-testid="research-query"]').type('Machine learning applications');
    cy.get('[data-testid="methodology"]').select('systematic_review');
    
    // Create workflow
    cy.get('[data-testid="create-btn"]').click();
    
    // Verify creation
    cy.get('[data-testid="success-message"]').should('contain', 'Workflow created');
    
    // Execute workflow
    cy.get('[data-testid="execute-btn"]').click();
    
    // Wait for completion
    cy.get('[data-testid="workflow-status"]', { timeout: 60000 })
      .should('contain', 'Completed');
    
    // Verify results
    cy.get('[data-testid="results-section"]').should('be.visible');
    cy.get('[data-testid="download-btn"]').should('be.enabled');
  });

  it('handles workflow errors gracefully', () => {
    // Create workflow with invalid parameters
    cy.get('[data-testid="create-workflow-btn"]').click();
    cy.get('[data-testid="research-query"]').type(''); // Empty query
    cy.get('[data-testid="create-btn"]').click();
    
    // Verify error handling
    cy.get('[data-testid="error-message"]').should('contain', 'Query is required');
  });
});
```

### Playwright Tests
```typescript
// tests/e2e/collaboration.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Real-time Collaboration', () => {
  test('multiple users can collaborate on workflow', async ({ browser }) => {
    // Create two browser contexts for different users
    const context1 = await browser.newContext();
    const context2 = await browser.newContext();
    
    const page1 = await context1.newPage();
    const page2 = await context2.newPage();
    
    // User 1 creates and shares workflow
    await page1.goto('/');
    await page1.fill('[data-testid="email"]', 'user1@example.com');
    await page1.fill('[data-testid="password"]', 'password');
    await page1.click('[data-testid="login-btn"]');
    
    await page1.click('[data-testid="create-workflow-btn"]');
    await page1.fill('[data-testid="workflow-name"]', 'Collaborative Workflow');
    await page1.click('[data-testid="create-btn"]');
    
    const workflowId = await page1.getAttribute('[data-testid="workflow-id"]', 'value');
    
    await page1.click('[data-testid="share-btn"]');
    await page1.fill('[data-testid="share-email"]', 'user2@example.com');
    await page1.click('[data-testid="send-invite-btn"]');
    
    // User 2 joins collaboration
    await page2.goto('/');
    await page2.fill('[data-testid="email"]', 'user2@example.com');
    await page2.fill('[data-testid="password"]', 'password');
    await page2.click('[data-testid="login-btn"]');
    
    await page2.goto(`/workflow/${workflowId}`);
    
    // Test real-time updates
    await page1.fill('[data-testid="research-query"]', 'AI in healthcare');
    
    // Verify user 2 sees the update
    await expect(page2.locator('[data-testid="research-query"]')).toHaveValue('AI in healthcare');
    
    // Test presence indicators
    await expect(page1.locator('[data-testid="user-presence"]')).toContainText('user2@example.com');
    await expect(page2.locator('[data-testid="user-presence"]')).toContainText('user1@example.com');
  });
});
```

---

## Performance Testing

### Load Testing with Criterion
```rust
// benches/workflow_performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use free_deep_research::services::research_engine::ResearchEngineService;
use tokio::runtime::Runtime;

fn benchmark_workflow_creation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let service = rt.block_on(async {
        ResearchEngineService::new().await.unwrap()
    });

    c.bench_function("workflow_creation", |b| {
        b.to_async(&rt).iter(|| async {
            let result = service.create_workflow(
                black_box("benchmark query".to_string()),
                black_box("systematic_review".to_string())
            ).await;
            black_box(result)
        })
    });
}

fn benchmark_concurrent_workflows(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let service = rt.block_on(async {
        ResearchEngineService::new().await.unwrap()
    });

    c.bench_function("concurrent_workflows", |b| {
        b.to_async(&rt).iter(|| async {
            let futures: Vec<_> = (0..10).map(|i| {
                service.create_workflow(
                    format!("query {}", i),
                    "systematic_review".to_string()
                )
            }).collect();
            
            let results = futures::future::join_all(futures).await;
            black_box(results)
        })
    });
}

criterion_group!(benches, benchmark_workflow_creation, benchmark_concurrent_workflows);
criterion_main!(benches);
```

### Stress Testing
```bash
#!/bin/bash
# scripts/stress-test.sh

echo "Starting stress test..."

# Test concurrent users
for i in {1..100}; do
    curl -X POST http://localhost:8080/api/workflows \
         -H "Content-Type: application/json" \
         -d '{"name":"Stress Test '$i'","query":"test query"}' &
done

wait

echo "Stress test completed"

# Check system resources
echo "System resources after test:"
ps aux | grep free-deep-research
free -h
df -h
```

### Memory Leak Testing
```rust
// tests/memory_leak_tests.rs
#[tokio::test]
async fn test_memory_usage_over_time() {
    let service = ResearchEngineService::new().await.unwrap();
    let initial_memory = get_memory_usage();
    
    // Create and destroy many workflows
    for i in 0..1000 {
        let workflow = service.create_workflow(
            format!("test query {}", i),
            "systematic_review".to_string()
        ).await.unwrap();
        
        // Simulate workflow execution and cleanup
        service.execute_workflow(workflow.id).await.unwrap();
        service.delete_workflow(workflow.id).await.unwrap();
        
        if i % 100 == 0 {
            // Force garbage collection
            tokio::task::yield_now().await;
            
            let current_memory = get_memory_usage();
            let memory_growth = current_memory - initial_memory;
            
            // Memory growth should be reasonable
            assert!(memory_growth < 100 * 1024 * 1024, // 100MB
                "Memory growth too large: {} bytes", memory_growth);
        }
    }
}

fn get_memory_usage() -> usize {
    // Platform-specific memory usage implementation
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        let status = fs::read_to_string("/proc/self/status").unwrap();
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                return parts[1].parse::<usize>().unwrap() * 1024; // Convert KB to bytes
            }
        }
    }
    0
}
```

---

## Security Testing

### Authentication Tests
```rust
// tests/security/auth_tests.rs
#[tokio::test]
async fn test_jwt_token_validation() {
    let auth_service = AuthenticationService::new().await.unwrap();
    
    // Test valid token
    let token = auth_service.create_token("user123").await.unwrap();
    let claims = auth_service.validate_token(&token).await.unwrap();
    assert_eq!(claims.user_id, "user123");
    
    // Test expired token
    let expired_token = create_expired_token();
    let result = auth_service.validate_token(&expired_token).await;
    assert!(matches!(result, Err(AuthError::TokenExpired)));
    
    // Test tampered token
    let tampered_token = token + "tampered";
    let result = auth_service.validate_token(&tampered_token).await;
    assert!(matches!(result, Err(AuthError::InvalidToken)));
}

#[tokio::test]
async fn test_rate_limiting() {
    let auth_service = AuthenticationService::new().await.unwrap();
    
    // Test rate limiting
    for i in 0..10 {
        let result = auth_service.attempt_login("user", "wrong_password").await;
        if i < 5 {
            assert!(matches!(result, Err(AuthError::InvalidCredentials)));
        } else {
            assert!(matches!(result, Err(AuthError::RateLimited)));
        }
    }
}
```

### Encryption Tests
```rust
// tests/security/encryption_tests.rs
#[tokio::test]
async fn test_data_encryption() {
    let encryption_service = EncryptionService::new().await.unwrap();
    
    let sensitive_data = "confidential research data";
    let encrypted = encryption_service.encrypt(sensitive_data.as_bytes()).await.unwrap();
    
    // Verify data is actually encrypted
    assert_ne!(encrypted, sensitive_data.as_bytes());
    assert!(!String::from_utf8_lossy(&encrypted).contains("confidential"));
    
    // Verify decryption works
    let decrypted = encryption_service.decrypt(&encrypted).await.unwrap();
    assert_eq!(String::from_utf8(decrypted).unwrap(), sensitive_data);
}

#[tokio::test]
async fn test_key_rotation() {
    let encryption_service = EncryptionService::new().await.unwrap();
    
    let data = "test data";
    let encrypted_v1 = encryption_service.encrypt(data.as_bytes()).await.unwrap();
    
    // Rotate key
    encryption_service.rotate_key().await.unwrap();
    
    // Old data should still be decryptable
    let decrypted_v1 = encryption_service.decrypt(&encrypted_v1).await.unwrap();
    assert_eq!(String::from_utf8(decrypted_v1).unwrap(), data);
    
    // New data should use new key
    let encrypted_v2 = encryption_service.encrypt(data.as_bytes()).await.unwrap();
    assert_ne!(encrypted_v1, encrypted_v2);
}
```

### Penetration Testing
```bash
#!/bin/bash
# scripts/security-scan.sh

echo "Running security scans..."

# SQL injection testing
sqlmap -u "http://localhost:8080/api/workflows?id=1" --batch --level=3

# XSS testing
python3 -m xsser -u "http://localhost:8080" --auto

# Port scanning
nmap -sS -O localhost

# SSL/TLS testing
testssl.sh localhost:8080

# Dependency vulnerability scanning
cargo audit

echo "Security scan completed"
```

---

## Test Automation

### CI/CD Pipeline
```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run unit tests
        run: cargo test --lib
      - name: Run frontend tests
        run: npm test

  integration-tests:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v4
      - name: Run integration tests
        run: cargo test --test integration

  e2e-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install dependencies
        run: npm ci
      - name: Run Cypress tests
        run: npm run cypress:run
      - name: Run Playwright tests
        run: npm run playwright:test

  performance-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run benchmarks
        run: cargo bench
      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: target/criterion/

  security-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Security audit
        run: cargo audit
      - name: SAST scan
        run: |
          cargo install cargo-geiger
          cargo geiger
```

### Test Reporting
```rust
// tests/test_reporter.rs
use std::fs::File;
use std::io::Write;
use serde_json::json;

pub struct TestReporter {
    results: Vec<TestResult>,
}

impl TestReporter {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.results.push(result);
    }

    pub fn generate_report(&self) -> String {
        let total_tests = self.results.len();
        let passed_tests = self.results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;

        json!({
            "summary": {
                "total": total_tests,
                "passed": passed_tests,
                "failed": failed_tests,
                "success_rate": (passed_tests as f64 / total_tests as f64) * 100.0
            },
            "results": self.results
        }).to_string()
    }

    pub fn save_report(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(self.generate_report().as_bytes())?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}
```

---

## Test Data Management

### Test Fixtures
```rust
// tests/fixtures/mod.rs
use free_deep_research::models::*;
use uuid::Uuid;
use chrono::Utc;

pub fn create_test_workflow() -> ResearchWorkflow {
    ResearchWorkflow {
        id: Uuid::new_v4(),
        name: "Test Workflow".to_string(),
        query: "test query".to_string(),
        methodology: "systematic_review".to_string(),
        status: WorkflowStatus::Created,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        user_id: Uuid::new_v4(),
        parameters: HashMap::new(),
        results: None,
    }
}

pub fn create_test_user() -> User {
    User {
        id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        username: "testuser".to_string(),
        created_at: Utc::now(),
        last_login: None,
        active: true,
    }
}
```

### Database Seeding
```sql
-- tests/fixtures/test_data.sql
INSERT INTO users (id, email, username, created_at, active) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'test1@example.com', 'testuser1', NOW(), true),
('550e8400-e29b-41d4-a716-446655440001', 'test2@example.com', 'testuser2', NOW(), true);

INSERT INTO workflows (id, name, query, methodology, status, user_id, created_at) VALUES
('660e8400-e29b-41d4-a716-446655440000', 'Test Workflow 1', 'AI research', 'systematic_review', 'Created', '550e8400-e29b-41d4-a716-446655440000', NOW()),
('660e8400-e29b-41d4-a716-446655440001', 'Test Workflow 2', 'ML applications', 'meta_analysis', 'Running', '550e8400-e29b-41d4-a716-446655440001', NOW());
```

---

## Running Tests

### Local Development
```bash
# Run all tests
make test

# Run specific test suites
cargo test --lib                    # Unit tests
cargo test --test integration       # Integration tests
npm test                            # Frontend tests
npm run cypress:run                 # E2E tests
cargo bench                         # Performance tests

# Run with coverage
cargo tarpaulin --out Html --output-dir coverage/

# Run security tests
cargo audit
npm audit
```

### Production Testing
```bash
# Smoke tests
./scripts/smoke-test.sh production-url

# Load testing
./scripts/load-test.sh --users 1000 --duration 10m

# Security scanning
./scripts/security-scan.sh production-url
```
