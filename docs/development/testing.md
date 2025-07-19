# 🧪 Testing Guide

## Overview

This guide covers the comprehensive testing strategy for the Free Deep Research System, including unit tests, integration tests, end-to-end tests, and performance testing.

## 🎯 Testing Strategy

### Testing Pyramid

```
    /\
   /  \     E2E Tests (Few)
  /____\    
 /      \   Integration Tests (Some)
/__________\ Unit Tests (Many)
```

- **Unit Tests (70%)**: Fast, isolated, focused on individual functions
- **Integration Tests (20%)**: Test component interactions
- **E2E Tests (10%)**: Test complete user workflows

### Test Categories

1. **Functional Testing**: Verify features work as expected
2. **Security Testing**: Validate security controls
3. **Performance Testing**: Ensure system meets performance requirements
4. **Compatibility Testing**: Cross-platform and browser compatibility
5. **Accessibility Testing**: WCAG compliance verification

## 🔧 Testing Setup

### Frontend Testing Stack

```json
{
  "devDependencies": {
    "@testing-library/react": "^13.4.0",
    "@testing-library/jest-dom": "^5.16.5",
    "@testing-library/user-event": "^14.4.3",
    "vitest": "^0.34.0",
    "jsdom": "^22.1.0",
    "msw": "^1.3.0",
    "playwright": "^1.37.0"
  }
}
```

### Backend Testing Stack

```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.11"
wiremock = "0.5"
criterion = "0.5"
proptest = "1.2"
```

### Test Configuration

```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      threshold: {
        global: {
          branches: 80,
          functions: 80,
          lines: 80,
          statements: 80
        }
      }
    }
  }
});
```

## 🧪 Unit Testing

### Frontend Unit Tests

```typescript
// ResearchWorkflow.test.tsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import { ResearchWorkflow } from '../ResearchWorkflow';

describe('ResearchWorkflow', () => {
  const mockInvoke = vi.fn();
  
  beforeEach(() => {
    vi.clearAllMocks();
    (window as any).__TAURI__ = {
      invoke: mockInvoke
    };
  });

  it('should create research workflow with valid input', async () => {
    mockInvoke.mockResolvedValue({ workflowId: 'test-123' });
    
    render(<ResearchWorkflow />);
    
    const queryInput = screen.getByLabelText('Research Query');
    const submitButton = screen.getByRole('button', { name: 'Start Research' });
    
    fireEvent.change(queryInput, { target: { value: 'AI in healthcare' } });
    fireEvent.click(submitButton);
    
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('create_research_workflow', {
        query: 'AI in healthcare',
        methodology: 'hybrid',
        parameters: expect.any(Object)
      });
    });
  });

  it('should display error for invalid input', async () => {
    render(<ResearchWorkflow />);
    
    const submitButton = screen.getByRole('button', { name: 'Start Research' });
    fireEvent.click(submitButton);
    
    await waitFor(() => {
      expect(screen.getByText('Query is required')).toBeInTheDocument();
    });
  });
});
```

### Backend Unit Tests

```rust
// tests/research_workflow_tests.rs
use crate::services::research_engine::ResearchEngine;
use mockall::predicate::*;

#[tokio::test]
async fn test_create_research_workflow() {
    let mut mock_engine = MockResearchEngine::new();
    
    mock_engine
        .expect_create_workflow()
        .with(eq("AI in healthcare"))
        .times(1)
        .returning(|_| Ok(WorkflowId::new("test-123")));
    
    let result = mock_engine
        .create_workflow("AI in healthcare")
        .await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_string(), "test-123");
}

#[tokio::test]
async fn test_invalid_query_returns_error() {
    let mut mock_engine = MockResearchEngine::new();
    
    let result = mock_engine
        .create_workflow("")
        .await;
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
}
```

## 🔗 Integration Testing

### API Integration Tests

```typescript
// api.integration.test.ts
import { setupServer } from 'msw/node';
import { rest } from 'msw';
import { ApiClient } from '../services/ApiClient';

const server = setupServer(
  rest.post('/api/research/workflows', (req, res, ctx) => {
    return res(
      ctx.json({
        workflowId: 'integration-test-123',
        status: 'created'
      })
    );
  })
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());

describe('API Integration Tests', () => {
  it('should create workflow via API', async () => {
    const client = new ApiClient('http://localhost:3000');
    
    const result = await client.createWorkflow({
      query: 'AI in healthcare',
      methodology: 'hybrid'
    });
    
    expect(result.workflowId).toBe('integration-test-123');
    expect(result.status).toBe('created');
  });
});
```

### Database Integration Tests

```rust
// tests/database_integration.rs
use sqlx::PgPool;
use testcontainers::clients::Cli;
use testcontainers::images::postgres::Postgres;

#[tokio::test]
async fn test_workflow_persistence() {
    let docker = Cli::default();
    let postgres = docker.run(Postgres::default());
    let port = postgres.get_host_port_ipv4(5432);
    
    let database_url = format!("postgres://postgres:postgres@localhost:{}/test", port);
    let pool = PgPool::connect(&database_url).await.unwrap();
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    // Test workflow creation
    let workflow = create_test_workflow(&pool).await.unwrap();
    
    // Verify workflow was saved
    let saved_workflow = get_workflow_by_id(&pool, workflow.id).await.unwrap();
    assert_eq!(saved_workflow.query, "Test query");
}
```

## 🎭 End-to-End Testing

### Playwright E2E Tests

```typescript
// e2e/research-workflow.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Research Workflow E2E', () => {
  test('complete research workflow', async ({ page }) => {
    await page.goto('/');
    
    // Navigate to research page
    await page.click('[data-testid="research-tab"]');
    
    // Fill in research form
    await page.fill('[data-testid="query-input"]', 'AI in healthcare');
    await page.selectOption('[data-testid="methodology-select"]', 'hybrid');
    await page.fill('[data-testid="max-sources"]', '50');
    
    // Start research
    await page.click('[data-testid="start-research-btn"]');
    
    // Wait for workflow creation
    await expect(page.locator('[data-testid="workflow-status"]')).toContainText('Running');
    
    // Wait for completion (with timeout)
    await expect(page.locator('[data-testid="workflow-status"]')).toContainText('Completed', {
      timeout: 300000 // 5 minutes
    });
    
    // Verify results
    await expect(page.locator('[data-testid="results-summary"]')).toBeVisible();
    await expect(page.locator('[data-testid="source-count"]')).toContainText(/\d+ sources/);
  });

  test('error handling for invalid input', async ({ page }) => {
    await page.goto('/');
    await page.click('[data-testid="research-tab"]');
    
    // Try to submit empty form
    await page.click('[data-testid="start-research-btn"]');
    
    // Verify error message
    await expect(page.locator('[data-testid="error-message"]')).toContainText('Query is required');
  });
});
```

### Mobile E2E Tests

```typescript
// e2e/mobile.spec.ts
import { test, expect, devices } from '@playwright/test';

test.use({ ...devices['iPhone 13'] });

test('mobile research workflow', async ({ page }) => {
  await page.goto('/');
  
  // Test mobile-specific interactions
  await page.tap('[data-testid="mobile-menu"]');
  await page.tap('[data-testid="research-option"]');
  
  // Test touch interactions
  await page.fill('[data-testid="query-input"]', 'Mobile AI research');
  await page.tap('[data-testid="start-research-btn"]');
  
  // Verify mobile layout
  await expect(page.locator('[data-testid="mobile-progress"]')).toBeVisible();
});
```

## ⚡ Performance Testing

### Load Testing

```typescript
// performance/load-test.ts
import { check } from 'k6';
import http from 'k6/http';

export let options = {
  stages: [
    { duration: '2m', target: 10 }, // Ramp up
    { duration: '5m', target: 10 }, // Stay at 10 users
    { duration: '2m', target: 20 }, // Ramp up to 20 users
    { duration: '5m', target: 20 }, // Stay at 20 users
    { duration: '2m', target: 0 },  // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<2000'], // 95% of requests under 2s
    http_req_failed: ['rate<0.1'],     // Error rate under 10%
  },
};

export default function() {
  const response = http.post('http://localhost:3000/api/research/workflows', {
    query: 'Performance test query',
    methodology: 'hybrid',
    maxSources: 25
  });
  
  check(response, {
    'status is 201': (r) => r.status === 201,
    'response time < 2s': (r) => r.timings.duration < 2000,
  });
}
```

### Benchmark Tests

```rust
// benches/research_engine_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use free_deep_research::services::research_engine::ResearchEngine;

fn benchmark_workflow_creation(c: &mut Criterion) {
    let engine = ResearchEngine::new();
    
    c.bench_function("create_workflow", |b| {
        b.iter(|| {
            engine.create_workflow(black_box("Benchmark query"))
        })
    });
}

fn benchmark_source_analysis(c: &mut Criterion) {
    let engine = ResearchEngine::new();
    let sources = generate_test_sources(100);
    
    c.bench_function("analyze_sources", |b| {
        b.iter(|| {
            engine.analyze_sources(black_box(&sources))
        })
    });
}

criterion_group!(benches, benchmark_workflow_creation, benchmark_source_analysis);
criterion_main!(benches);
```

## 🔒 Security Testing

### Security Test Suite

```typescript
// security/security.test.ts
describe('Security Tests', () => {
  it('should prevent SQL injection', async () => {
    const maliciousQuery = "'; DROP TABLE users; --";
    
    const response = await request(app)
      .post('/api/research/workflows')
      .send({ query: maliciousQuery })
      .expect(400);
    
    expect(response.body.error).toContain('Invalid input');
  });

  it('should prevent XSS attacks', async () => {
    const xssPayload = '<script>alert("xss")</script>';
    
    const response = await request(app)
      .post('/api/research/workflows')
      .send({ query: xssPayload })
      .expect(400);
    
    expect(response.body.error).toContain('Invalid characters');
  });

  it('should enforce rate limiting', async () => {
    const requests = Array(101).fill(null).map(() =>
      request(app).post('/api/research/workflows').send({ query: 'test' })
    );
    
    const responses = await Promise.all(requests);
    const rateLimitedResponses = responses.filter(r => r.status === 429);
    
    expect(rateLimitedResponses.length).toBeGreaterThan(0);
  });
});
```

## 📊 Test Reporting

### Coverage Reports

```bash
# Generate coverage reports
npm run test:coverage
cargo tarpaulin --out html

# View coverage
open coverage/index.html
open tarpaulin-report.html
```

### Test Results Dashboard

```typescript
// test-results.ts
interface TestResults {
  total: number;
  passed: number;
  failed: number;
  skipped: number;
  coverage: {
    lines: number;
    functions: number;
    branches: number;
    statements: number;
  };
  performance: {
    averageResponseTime: number;
    p95ResponseTime: number;
    errorRate: number;
  };
}

const generateTestReport = (results: TestResults) => {
  return {
    summary: `${results.passed}/${results.total} tests passed`,
    coverage: `${results.coverage.lines}% line coverage`,
    performance: `${results.performance.averageResponseTime}ms avg response time`,
    status: results.failed === 0 ? 'PASS' : 'FAIL'
  };
};
```

## 🚀 Continuous Testing

### CI/CD Pipeline

```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run unit tests
        run: npm run test:unit
      
      - name: Run integration tests
        run: npm run test:integration
      
      - name: Run E2E tests
        run: npm run test:e2e
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

### Test Automation

```bash
# Pre-commit hooks
#!/bin/sh
# .git/hooks/pre-commit

# Run tests before commit
npm run test:unit
npm run lint
npm run type-check

# Run security checks
npm audit
cargo audit
```

## 📚 Testing Best Practices

### Test Organization

1. **Arrange-Act-Assert**: Structure tests clearly
2. **One Assertion Per Test**: Focus on single behavior
3. **Descriptive Names**: Test names should explain what they test
4. **Test Data**: Use factories and fixtures for consistent test data

### Test Maintenance

1. **Regular Updates**: Keep tests updated with code changes
2. **Flaky Test Management**: Identify and fix unreliable tests
3. **Performance Monitoring**: Track test execution times
4. **Coverage Goals**: Maintain minimum coverage thresholds

---

**Next**: Check out [Performance Testing](./performance.md) for detailed performance testing strategies.
