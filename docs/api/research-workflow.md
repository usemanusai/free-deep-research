# 🔬 Research Workflow API

## Overview

The Research Workflow API provides comprehensive functionality for creating, executing, and managing research workflows. This includes support for multiple research methodologies, real-time monitoring, and advanced result processing.

## 🚀 Core Workflow Operations

### Create Research Workflow

Create a new research workflow with specified parameters.

**Tauri Command:**
```typescript
const workflowId = await invoke<string>('create_research_workflow', {
  name: 'AI in Healthcare Research',
  query: 'Latest developments in AI for medical diagnosis',
  methodology: 'hybrid', // 'don_lim', 'nick_scamara', 'hybrid'
  depth: 'comprehensive', // 'basic', 'standard', 'comprehensive'
  maxSources: 50,
  timeframe: '2024-2025',
  languages: ['en', 'es'],
  domains: ['academic', 'industry', 'news'],
  budget: {
    maxCostUsd: 25.00,
    costPerSource: 0.50
  }
})
```

**REST Endpoint:**
```http
POST /api/research/workflows
Authorization: Bearer YOUR_API_KEY
Content-Type: application/json

{
  "name": "AI in Healthcare Research",
  "query": "Latest developments in AI for medical diagnosis",
  "methodology": "hybrid",
  "depth": "comprehensive",
  "maxSources": 50,
  "timeframe": "2024-2025",
  "languages": ["en", "es"],
  "domains": ["academic", "industry", "news"],
  "budget": {
    "maxCostUsd": 25.00,
    "costPerSource": 0.50
  }
}
```

**Response:**
```json
{
  "workflowId": "550e8400-e29b-41d4-a716-446655440000",
  "status": "created",
  "estimatedDuration": "15-20 minutes",
  "estimatedCost": "$18.50"
}
```

### Execute Research Workflow

Start execution of a created research workflow.

**Tauri Command:**
```typescript
await invoke('execute_research', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000'
})
```

### Get Research Status

Monitor the progress of a running research workflow.

**Tauri Command:**
```typescript
const status = await invoke<ResearchStatus>('get_research_status', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000'
})
```

**Response:**
```json
{
  "workflowId": "550e8400-e29b-41d4-a716-446655440000",
  "status": "running",
  "progress": {
    "percentage": 65,
    "currentPhase": "source_analysis",
    "phasesCompleted": ["query_expansion", "source_discovery"],
    "phasesRemaining": ["synthesis", "validation"]
  },
  "metrics": {
    "sourcesFound": 42,
    "sourcesAnalyzed": 28,
    "costIncurred": 12.30,
    "timeElapsed": "8m 32s",
    "estimatedTimeRemaining": "6m 45s"
  },
  "currentActivity": "Analyzing academic papers from PubMed"
}
```

### Get Research Results

Retrieve the results of a completed research workflow.

**Tauri Command:**
```typescript
const results = await invoke<ResearchResults>('get_research_results', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000'
})
```

**Response:**
```json
{
  "workflowId": "550e8400-e29b-41d4-a716-446655440000",
  "status": "completed",
  "results": {
    "summary": "Comprehensive analysis of AI in medical diagnosis...",
    "keyFindings": [
      "Deep learning models show 95% accuracy in radiology",
      "Natural language processing improves clinical documentation",
      "Ethical considerations remain a primary concern"
    ],
    "sources": [
      {
        "id": "source_001",
        "title": "Deep Learning in Medical Imaging",
        "url": "https://example.com/paper1",
        "type": "academic_paper",
        "relevanceScore": 0.95,
        "credibilityScore": 0.92,
        "publicationDate": "2024-11-15"
      }
    ],
    "metadata": {
      "totalSources": 45,
      "sourcesAnalyzed": 45,
      "averageRelevance": 0.87,
      "averageCredibility": 0.89,
      "costBreakdown": {
        "totalCost": 18.45,
        "searchCosts": 8.20,
        "analysisCosts": 10.25
      }
    }
  }
}
```

### Cancel Research Workflow

Stop a running research workflow.

**Tauri Command:**
```typescript
await invoke('cancel_research', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000'
})
```

## 📊 Workflow Management

### List All Workflows

Get all research workflows for the current user.

**Tauri Command:**
```typescript
const workflows = await invoke<ResearchWorkflow[]>('list_research_workflows', {
  status: 'all', // 'all', 'running', 'completed', 'failed', 'cancelled'
  limit: 50,
  offset: 0,
  sortBy: 'created_at',
  sortOrder: 'desc'
})
```

### Get Workflow Details

Retrieve detailed information about a specific workflow.

**Tauri Command:**
```typescript
const workflow = await invoke<ResearchWorkflow>('get_workflow_details', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000'
})
```

### Delete Workflow

Remove a workflow and its results.

**Tauri Command:**
```typescript
await invoke('delete_workflow', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000'
})
```

## 🔄 Batch Operations

### Create Batch Workflow

Execute multiple research queries in a single batch.

**Tauri Command:**
```typescript
const batchId = await invoke<string>('create_batch_workflow', {
  name: 'Healthcare AI Research Batch',
  queries: [
    'AI in radiology diagnosis',
    'Machine learning in drug discovery',
    'Natural language processing in clinical notes'
  ],
  methodology: 'hybrid',
  sharedSettings: {
    depth: 'standard',
    maxSources: 30,
    budget: { maxCostUsd: 15.00 }
  }
})
```

### Get Batch Status

Monitor the progress of a batch workflow.

**Tauri Command:**
```typescript
const batchStatus = await invoke<BatchWorkflowStatus>('get_batch_status', {
  batchId: '550e8400-e29b-41d4-a716-446655440000'
})
```

## 🎯 Research Methodologies

### Available Methodologies

| Methodology | Description | Best For | Avg Duration | Avg Cost |
|-------------|-------------|----------|--------------|----------|
| **don_lim** | Comprehensive academic focus | Academic research | 20-30 min | $15-25 |
| **nick_scamara** | Industry and practical focus | Business research | 15-25 min | $12-20 |
| **hybrid** | Balanced academic + industry | General research | 18-28 min | $14-22 |

### Get Methodology Details

**Tauri Command:**
```typescript
const methodology = await invoke<ResearchMethodology>('get_methodology_details', {
  methodology: 'hybrid'
})
```

## 🔧 Advanced Features

### Custom Research Templates

Create reusable research templates.

**Tauri Command:**
```typescript
const templateId = await invoke<string>('create_research_template', {
  name: 'Healthcare AI Template',
  methodology: 'hybrid',
  defaultSettings: {
    depth: 'comprehensive',
    maxSources: 50,
    domains: ['academic', 'industry'],
    languages: ['en']
  },
  queryTemplates: [
    'Latest developments in {topic} for {application}',
    'Clinical trials for {topic} in {year}'
  ]
})
```

### Real-time Collaboration

Share research workflows with team members.

**Tauri Command:**
```typescript
await invoke('share_workflow', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000',
  shareWith: ['user@example.com'],
  permissions: ['read', 'comment']
})
```

### Export Results

Export research results in various formats.

**Tauri Command:**
```typescript
const exportData = await invoke<ExportResult>('export_research_results', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000',
  format: 'pdf', // 'pdf', 'docx', 'json', 'csv', 'markdown'
  includeMetadata: true,
  includeSources: true
})
```

## 📈 Performance Optimization

### Resource Management

Monitor and manage computational resources.

**Tauri Command:**
```typescript
const resourceStatus = await invoke<ResourceStatus>('get_resource_status')
```

### Caching

Enable intelligent caching for repeated queries.

**Tauri Command:**
```typescript
await invoke('configure_caching', {
  enabled: true,
  ttlHours: 24,
  maxCacheSize: '1GB'
})
```

## 🚨 Error Handling

Common error scenarios and handling:

```typescript
try {
  const workflowId = await invoke('create_research_workflow', params)
} catch (error) {
  if (error.includes('INSUFFICIENT_BUDGET')) {
    // Handle budget constraints
  } else if (error.includes('INVALID_METHODOLOGY')) {
    // Handle invalid methodology
  } else if (error.includes('QUOTA_EXCEEDED')) {
    // Handle quota limits
  }
}
```

## 📚 Related Documentation

- [Authentication API](./authentication.md)
- [Analytics API](./analytics.md)
- [Template Management API](./template-management.md)

---

**Next**: Explore [Analytics API](./analytics.md) for research insights and metrics.
