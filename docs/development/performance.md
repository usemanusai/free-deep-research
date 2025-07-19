# ⚡ Performance Guide

## Overview

This guide covers performance optimization strategies, monitoring techniques, and best practices for the Free Deep Research System. Learn how to identify bottlenecks, optimize performance, and ensure scalable operations.

## 🎯 Performance Objectives

### Key Performance Indicators (KPIs)

| Metric | Target | Measurement |
|--------|--------|-------------|
| **API Response Time** | < 200ms (95th percentile) | Average response time for API calls |
| **Research Completion** | < 15 minutes (standard) | Time to complete typical research |
| **Page Load Time** | < 2 seconds | Initial page load performance |
| **Memory Usage** | < 1GB (desktop app) | Peak memory consumption |
| **CPU Usage** | < 50% (average) | Average CPU utilization |
| **Database Query Time** | < 100ms (95th percentile) | Database operation performance |

### Performance Targets by Environment

#### Development Environment
- **Response Time**: < 500ms (acceptable for development)
- **Build Time**: < 2 minutes (full rebuild)
- **Hot Reload**: < 1 second (development changes)

#### Production Environment
- **Response Time**: < 200ms (95th percentile)
- **Throughput**: 1000+ requests/second
- **Availability**: 99.9% uptime
- **Error Rate**: < 0.1%

## 🔍 Performance Monitoring

### Application Performance Monitoring (APM)

#### Frontend Monitoring
```typescript
// Performance monitoring setup
import { getCLS, getFID, getFCP, getLCP, getTTFB } from 'web-vitals';

// Core Web Vitals monitoring
getCLS(console.log);  // Cumulative Layout Shift
getFID(console.log);  // First Input Delay
getFCP(console.log);  // First Contentful Paint
getLCP(console.log);  // Largest Contentful Paint
getTTFB(console.log); // Time to First Byte

// Custom performance metrics
const performanceObserver = new PerformanceObserver((list) => {
  list.getEntries().forEach((entry) => {
    if (entry.entryType === 'navigation') {
      console.log('Page Load Time:', entry.loadEventEnd - entry.fetchStart);
    }
  });
});
performanceObserver.observe({ entryTypes: ['navigation'] });
```

#### Backend Monitoring
```rust
// Rust performance monitoring
use std::time::Instant;
use tracing::{info, instrument};

#[instrument]
pub async fn research_workflow_handler(query: ResearchQuery) -> Result<WorkflowResult> {
    let start = Instant::now();
    
    let result = execute_research_workflow(query).await?;
    
    let duration = start.elapsed();
    info!("Research workflow completed in {:?}", duration);
    
    // Record metrics
    metrics::histogram!("research_workflow_duration", duration.as_millis() as f64);
    metrics::counter!("research_workflow_completed", 1);
    
    Ok(result)
}
```

### Real-Time Metrics Collection

#### Prometheus Metrics
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'free-deep-research'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: /metrics
    scrape_interval: 5s

  - job_name: 'node-exporter'
    static_configs:
      - targets: ['localhost:9100']
```

#### Custom Metrics
```typescript
// Custom metrics collection
interface PerformanceMetrics {
  apiResponseTime: number[];
  researchCompletionTime: number[];
  memoryUsage: number[];
  cpuUsage: number[];
  errorRate: number;
  throughput: number;
}

class MetricsCollector {
  private metrics: PerformanceMetrics = {
    apiResponseTime: [],
    researchCompletionTime: [],
    memoryUsage: [],
    cpuUsage: [],
    errorRate: 0,
    throughput: 0
  };

  recordApiResponse(duration: number) {
    this.metrics.apiResponseTime.push(duration);
    this.calculatePercentiles();
  }

  recordResearchCompletion(duration: number) {
    this.metrics.researchCompletionTime.push(duration);
  }

  private calculatePercentiles() {
    const sorted = this.metrics.apiResponseTime.sort((a, b) => a - b);
    const p95Index = Math.floor(sorted.length * 0.95);
    const p95 = sorted[p95Index];
    
    // Send to monitoring system
    this.sendMetric('api_response_time_p95', p95);
  }
}
```

## 🚀 Frontend Performance Optimization

### React Performance

#### Component Optimization
```typescript
// Memoization for expensive components
import React, { memo, useMemo, useCallback } from 'react';

const ResearchResultsComponent = memo(({ results, onSelect }) => {
  // Memoize expensive calculations
  const processedResults = useMemo(() => {
    return results.map(result => ({
      ...result,
      relevanceScore: calculateRelevance(result)
    }));
  }, [results]);

  // Memoize event handlers
  const handleResultSelect = useCallback((resultId) => {
    onSelect(resultId);
  }, [onSelect]);

  return (
    <div>
      {processedResults.map(result => (
        <ResultItem 
          key={result.id}
          result={result}
          onSelect={handleResultSelect}
        />
      ))}
    </div>
  );
});
```

#### Virtual Scrolling
```typescript
// Virtual scrolling for large lists
import { FixedSizeList as List } from 'react-window';

const VirtualizedResultsList = ({ results }) => {
  const Row = ({ index, style }) => (
    <div style={style}>
      <ResultItem result={results[index]} />
    </div>
  );

  return (
    <List
      height={600}
      itemCount={results.length}
      itemSize={100}
      width="100%"
    >
      {Row}
    </List>
  );
};
```

#### Code Splitting
```typescript
// Lazy loading for route components
import { lazy, Suspense } from 'react';

const ResearchPage = lazy(() => import('./pages/ResearchPage'));
const AnalyticsPage = lazy(() => import('./pages/AnalyticsPage'));
const SettingsPage = lazy(() => import('./pages/SettingsPage'));

function App() {
  return (
    <Router>
      <Suspense fallback={<LoadingSpinner />}>
        <Routes>
          <Route path="/research" element={<ResearchPage />} />
          <Route path="/analytics" element={<AnalyticsPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </Suspense>
    </Router>
  );
}
```

### Bundle Optimization

#### Webpack Configuration
```javascript
// webpack.config.js
module.exports = {
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
        common: {
          name: 'common',
          minChunks: 2,
          chunks: 'all',
          enforce: true,
        },
      },
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
};
```

#### Tree Shaking
```typescript
// Import only what you need
import { debounce } from 'lodash/debounce';  // ✅ Good
import _ from 'lodash';                      // ❌ Bad

// Use ES modules for better tree shaking
export { ResearchEngine } from './research-engine';
export { DataProcessor } from './data-processor';
```

## ⚙️ Backend Performance Optimization

### Database Optimization

#### Query Optimization
```sql
-- Index optimization
CREATE INDEX CONCURRENTLY idx_research_workflows_user_created 
ON research_workflows(user_id, created_at DESC);

CREATE INDEX CONCURRENTLY idx_research_sources_workflow_quality 
ON research_sources(workflow_id, quality_score DESC);

-- Query optimization
EXPLAIN ANALYZE 
SELECT w.id, w.title, COUNT(s.id) as source_count
FROM research_workflows w
LEFT JOIN research_sources s ON w.id = s.workflow_id
WHERE w.user_id = $1 
  AND w.created_at > NOW() - INTERVAL '30 days'
GROUP BY w.id, w.title
ORDER BY w.created_at DESC
LIMIT 20;
```

#### Connection Pooling
```rust
// Database connection pooling
use sqlx::postgres::PgPoolOptions;

pub async fn create_db_pool() -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await
}
```

### Caching Strategies

#### Redis Caching
```rust
// Redis caching implementation
use redis::{Client, Commands};

pub struct CacheService {
    client: Client,
}

impl CacheService {
    pub async fn get_research_result(&self, query_hash: &str) -> Option<ResearchResult> {
        let mut conn = self.client.get_connection().ok()?;
        let cached: Option<String> = conn.get(format!("research:{}", query_hash)).ok()?;
        
        cached.and_then(|data| serde_json::from_str(&data).ok())
    }

    pub async fn cache_research_result(&self, query_hash: &str, result: &ResearchResult) {
        if let Ok(mut conn) = self.client.get_connection() {
            let serialized = serde_json::to_string(result).unwrap();
            let _: () = conn.setex(
                format!("research:{}", query_hash),
                3600, // 1 hour TTL
                serialized
            ).unwrap_or(());
        }
    }
}
```

#### Application-Level Caching
```typescript
// LRU cache for frequently accessed data
import LRU from 'lru-cache';

const cache = new LRU<string, any>({
  max: 1000,
  ttl: 1000 * 60 * 15, // 15 minutes
});

class DataService {
  async getResearchTemplate(id: string) {
    const cacheKey = `template:${id}`;
    
    // Check cache first
    let template = cache.get(cacheKey);
    if (template) {
      return template;
    }

    // Fetch from database
    template = await this.database.getTemplate(id);
    
    // Cache the result
    cache.set(cacheKey, template);
    
    return template;
  }
}
```

### Async Processing

#### Background Jobs
```rust
// Background job processing
use tokio::task;
use tokio::sync::mpsc;

pub struct JobProcessor {
    sender: mpsc::Sender<ResearchJob>,
}

impl JobProcessor {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::channel::<ResearchJob>(100);
        
        // Spawn background worker
        task::spawn(async move {
            while let Some(job) = receiver.recv().await {
                if let Err(e) = process_research_job(job).await {
                    error!("Job processing failed: {}", e);
                }
            }
        });
        
        Self { sender }
    }

    pub async fn enqueue_job(&self, job: ResearchJob) -> Result<()> {
        self.sender.send(job).await
            .map_err(|_| Error::JobQueueFull)
    }
}
```

## 📊 Performance Testing

### Load Testing

#### K6 Load Testing
```javascript
// load-test.js
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '2m', target: 10 },   // Ramp up
    { duration: '5m', target: 10 },   // Stay at 10 users
    { duration: '2m', target: 20 },   // Ramp up to 20 users
    { duration: '5m', target: 20 },   // Stay at 20 users
    { duration: '2m', target: 0 },    // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<2000'], // 95% of requests under 2s
    http_req_failed: ['rate<0.1'],     // Error rate under 10%
  },
};

export default function() {
  let response = http.post('http://localhost:3000/api/research/workflows', {
    query: 'AI in healthcare',
    methodology: 'hybrid',
    maxSources: 25
  });
  
  check(response, {
    'status is 201': (r) => r.status === 201,
    'response time < 2s': (r) => r.timings.duration < 2000,
  });
  
  sleep(1);
}
```

#### Artillery Load Testing
```yaml
# artillery-config.yml
config:
  target: 'http://localhost:3000'
  phases:
    - duration: 60
      arrivalRate: 10
    - duration: 120
      arrivalRate: 20
    - duration: 60
      arrivalRate: 10
  defaults:
    headers:
      Content-Type: 'application/json'

scenarios:
  - name: "Research workflow creation"
    weight: 70
    flow:
      - post:
          url: "/api/research/workflows"
          json:
            query: "{{ $randomString() }}"
            methodology: "hybrid"
            maxSources: 25

  - name: "Get workflow status"
    weight: 30
    flow:
      - get:
          url: "/api/research/workflows/{{ workflowId }}/status"
```

### Benchmark Testing

#### Rust Benchmarks
```rust
// benches/research_engine.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_query_processing(c: &mut Criterion) {
    let engine = ResearchEngine::new();
    
    c.bench_function("process_query", |b| {
        b.iter(|| {
            engine.process_query(black_box("AI in healthcare"))
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

criterion_group!(benches, benchmark_query_processing, benchmark_source_analysis);
criterion_main!(benches);
```

## 🔧 Performance Tuning

### System-Level Optimization

#### Operating System Tuning
```bash
# Linux system optimization
# Increase file descriptor limits
echo "* soft nofile 65536" >> /etc/security/limits.conf
echo "* hard nofile 65536" >> /etc/security/limits.conf

# Optimize network settings
echo "net.core.somaxconn = 65535" >> /etc/sysctl.conf
echo "net.ipv4.tcp_max_syn_backlog = 65535" >> /etc/sysctl.conf
echo "net.core.netdev_max_backlog = 5000" >> /etc/sysctl.conf

# Apply changes
sysctl -p
```

#### Docker Optimization
```dockerfile
# Multi-stage build for smaller images
FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

FROM node:18-alpine AS runtime
RUN apk add --no-cache dumb-init
WORKDIR /app
COPY --from=builder /app/node_modules ./node_modules
COPY . .

# Optimize for production
ENV NODE_ENV=production
ENV NODE_OPTIONS="--max-old-space-size=1024"

USER node
ENTRYPOINT ["dumb-init", "--"]
CMD ["node", "server.js"]
```

### Application Tuning

#### Memory Management
```rust
// Memory pool for frequent allocations
use object_pool::Pool;

lazy_static! {
    static ref STRING_POOL: Pool<String> = Pool::new(100, || String::new());
}

pub fn process_text(input: &str) -> String {
    let mut buffer = STRING_POOL.try_pull().unwrap_or_else(String::new);
    buffer.clear();
    
    // Process text into buffer
    process_into_buffer(input, &mut buffer);
    
    let result = buffer.clone();
    
    // Return buffer to pool
    if buffer.capacity() < 1024 {
        STRING_POOL.attach(buffer);
    }
    
    result
}
```

#### Garbage Collection Tuning
```bash
# Node.js GC optimization
export NODE_OPTIONS="--max-old-space-size=4096 --gc-interval=100"

# Java GC tuning (if using JVM-based services)
export JAVA_OPTS="-Xmx2g -Xms1g -XX:+UseG1GC -XX:MaxGCPauseMillis=200"
```

## 📈 Performance Monitoring Dashboard

### Grafana Dashboard Configuration
```json
{
  "dashboard": {
    "title": "Free Deep Research Performance",
    "panels": [
      {
        "title": "API Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      },
      {
        "title": "Research Completion Time",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(research_workflow_duration_sum[5m]) / rate(research_workflow_duration_count[5m])",
            "legendFormat": "Average completion time"
          }
        ]
      }
    ]
  }
}
```

---

**Performance is a feature!** Regular monitoring and optimization ensure the best user experience. For performance issues, check our [Troubleshooting Guide](../user-guides/troubleshooting.md#performance-issues).
