# 📋 Template Management API

## Overview

The Template Management API provides comprehensive functionality for creating, managing, and executing research templates. Templates allow users to standardize research workflows and reuse proven methodologies across multiple research sessions.

## 🎯 Core Template Operations

### Create Research Template

Create a new research template with predefined parameters and configurations.

**Tauri Command:**
```typescript
const template = await invoke<ResearchTemplate>('create_research_template', {
  name: 'Healthcare AI Research Template',
  description: 'Standardized template for AI in healthcare research',
  methodology: 'hybrid',
  defaultParameters: {
    depth: 'comprehensive',
    maxSources: 50,
    timeframe: 'last_2_years',
    domains: ['academic', 'industry', 'clinical'],
    languages: ['en']
  },
  queryTemplates: [
    'Latest developments in {topic} for {application}',
    'Clinical trials for {topic} in {year}',
    'Regulatory considerations for {topic}'
  ],
  outputFormat: {
    includeExecutiveSummary: true,
    includeSources: true,
    includeMethodology: true,
    format: 'comprehensive_report'
  },
  tags: ['healthcare', 'ai', 'clinical'],
  isPublic: false
})
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Healthcare AI Research Template",
  "description": "Standardized template for AI in healthcare research",
  "methodology": "hybrid",
  "defaultParameters": {
    "depth": "comprehensive",
    "maxSources": 50,
    "timeframe": "last_2_years",
    "domains": ["academic", "industry", "clinical"],
    "languages": ["en"]
  },
  "queryTemplates": [
    "Latest developments in {topic} for {application}",
    "Clinical trials for {topic} in {year}",
    "Regulatory considerations for {topic}"
  ],
  "outputFormat": {
    "includeExecutiveSummary": true,
    "includeSources": true,
    "includeMethodology": true,
    "format": "comprehensive_report"
  },
  "tags": ["healthcare", "ai", "clinical"],
  "isPublic": false,
  "createdAt": "2025-01-20T15:30:00Z",
  "updatedAt": "2025-01-20T15:30:00Z",
  "createdBy": "user_123",
  "usageCount": 0,
  "averageRating": 0.0
}
```

### Get All Templates

Retrieve all available research templates with filtering options.

**Tauri Command:**
```typescript
const templates = await invoke<ResearchTemplate[]>('get_all_templates', {
  filters: {
    tags: ['healthcare', 'ai'],
    methodology: 'hybrid',
    isPublic: true,
    createdBy: 'user_123' // Optional: filter by creator
  },
  sortBy: 'usage_count', // 'name', 'created_at', 'usage_count', 'rating'
  sortOrder: 'desc',
  limit: 50,
  offset: 0
})
```

### Get Template by ID

Retrieve a specific template by its unique identifier.

**Tauri Command:**
```typescript
const template = await invoke<ResearchTemplate>('get_template_by_id', {
  templateId: '550e8400-e29b-41d4-a716-446655440000'
})
```

### Update Research Template

Update an existing research template.

**Tauri Command:**
```typescript
const updatedTemplate = await invoke<ResearchTemplate>('update_research_template', {
  id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Updated Healthcare AI Research Template',
  description: 'Enhanced template with new parameters',
  defaultParameters: {
    depth: 'comprehensive',
    maxSources: 75,
    timeframe: 'last_3_years'
  },
  tags: ['healthcare', 'ai', 'clinical', 'updated']
})
```

### Delete Research Template

Remove a research template from the system.

**Tauri Command:**
```typescript
await invoke('delete_research_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440000'
})
```

## 🚀 Template Execution

### Execute Template

Execute a research workflow using a predefined template.

**Tauri Command:**
```typescript
const workflowId = await invoke<string>('execute_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440000',
  context: {
    variables: {
      topic: 'machine learning',
      application: 'medical diagnosis',
      year: '2024'
    },
    overrides: {
      maxSources: 100,
      budget: { maxCostUsd: 30.00 }
    }
  },
  name: 'ML in Medical Diagnosis Research'
})
```

**Response:**
```json
{
  "workflowId": "workflow_789",
  "templateId": "550e8400-e29b-41d4-a716-446655440000",
  "status": "created",
  "estimatedDuration": "20-25 minutes",
  "estimatedCost": "$22.50",
  "resolvedQuery": "Latest developments in machine learning for medical diagnosis",
  "appliedParameters": {
    "depth": "comprehensive",
    "maxSources": 100,
    "methodology": "hybrid"
  }
}
```

### Preview Template Execution

Preview what a workflow would look like before executing the template.

**Tauri Command:**
```typescript
const preview = await invoke<WorkflowPreview>('preview_template_execution', {
  templateId: '550e8400-e29b-41d4-a716-446655440000',
  variables: {
    topic: 'artificial intelligence',
    application: 'drug discovery',
    year: '2024'
  }
})
```

**Response:**
```json
{
  "resolvedQueries": [
    "Latest developments in artificial intelligence for drug discovery",
    "Clinical trials for artificial intelligence in 2024",
    "Regulatory considerations for artificial intelligence"
  ],
  "estimatedMetrics": {
    "duration": "18-23 minutes",
    "cost": "$19.50",
    "sourcesExpected": 45,
    "confidenceScore": 0.92
  },
  "appliedParameters": {
    "methodology": "hybrid",
    "depth": "comprehensive",
    "maxSources": 50,
    "domains": ["academic", "industry", "clinical"]
  }
}
```

## 📊 Template Analytics

### Get Template Statistics

Retrieve usage statistics and performance metrics for templates.

**Tauri Command:**
```typescript
const stats = await invoke<TemplateStatistics>('get_template_statistics', {
  templateId: '550e8400-e29b-41d4-a716-446655440000',
  timeframe: 'last_30_days'
})
```

**Response:**
```json
{
  "templateId": "550e8400-e29b-41d4-a716-446655440000",
  "usageCount": 45,
  "successRate": 0.96,
  "averageRating": 4.7,
  "averageDuration": "19m 32s",
  "averageCost": "$18.50",
  "totalCostSaved": "$245.00",
  "userFeedback": {
    "positive": 42,
    "neutral": 2,
    "negative": 1
  },
  "popularVariables": {
    "topic": ["machine learning", "artificial intelligence", "deep learning"],
    "application": ["medical diagnosis", "drug discovery", "clinical trials"]
  }
}
```

### Get Template Usage History

Retrieve detailed usage history for a template.

**Tauri Command:**
```typescript
const history = await invoke<TemplateUsageHistory[]>('get_template_usage_history', {
  templateId: '550e8400-e29b-41d4-a716-446655440000',
  limit: 50,
  includeResults: false
})
```

## 🔄 Template Sharing & Collaboration

### Share Template

Share a template with other users or make it public.

**Tauri Command:**
```typescript
const shareResult = await invoke<TemplateShareResult>('share_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440000',
  shareWith: ['user@example.com', 'team@company.com'],
  permissions: ['read', 'execute'], // 'read', 'execute', 'modify'
  isPublic: false,
  shareMessage: 'Great template for healthcare AI research'
})
```

### Clone Template

Create a copy of an existing template for customization.

**Tauri Command:**
```typescript
const clonedTemplate = await invoke<ResearchTemplate>('clone_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440000',
  newName: 'My Custom Healthcare AI Template',
  modifications: {
    methodology: 'don_lim',
    defaultParameters: {
      maxSources: 75
    }
  }
})
```

## 📚 Template Categories & Discovery

### Get Template Categories

Retrieve available template categories and tags.

**Tauri Command:**
```typescript
const categories = await invoke<TemplateCategory[]>('get_template_categories')
```

**Response:**
```json
[
  {
    "name": "Healthcare & Medical",
    "description": "Templates for medical and healthcare research",
    "templateCount": 23,
    "tags": ["healthcare", "medical", "clinical", "pharmaceutical"]
  },
  {
    "name": "Technology & AI",
    "description": "Templates for technology and AI research",
    "templateCount": 18,
    "tags": ["ai", "machine learning", "technology", "software"]
  }
]
```

### Search Templates

Search for templates using various criteria.

**Tauri Command:**
```typescript
const searchResults = await invoke<TemplateSearchResult[]>('search_templates', {
  query: 'machine learning healthcare',
  filters: {
    tags: ['ai', 'healthcare'],
    methodology: ['hybrid', 'don_lim'],
    minRating: 4.0,
    isPublic: true
  },
  sortBy: 'relevance',
  limit: 20
})
```

## 🔧 Template Validation & Testing

### Validate Template

Validate a template configuration before saving.

**Tauri Command:**
```typescript
const validation = await invoke<TemplateValidation>('validate_template', {
  template: {
    name: 'Test Template',
    queryTemplates: [
      'Research on {topic} in {domain}'
    ],
    defaultParameters: {
      maxSources: 50
    }
  }
})
```

**Response:**
```json
{
  "isValid": true,
  "errors": [],
  "warnings": [
    {
      "field": "queryTemplates",
      "message": "Consider adding more query variations for better coverage",
      "severity": "low"
    }
  ],
  "suggestions": [
    "Add more specific query templates",
    "Include domain-specific parameters"
  ]
}
```

### Test Template Execution

Test a template with sample data to verify it works correctly.

**Tauri Command:**
```typescript
const testResult = await invoke<TemplateTestResult>('test_template_execution', {
  templateId: '550e8400-e29b-41d4-a716-446655440000',
  testData: {
    variables: {
      topic: 'test topic',
      application: 'test application'
    }
  },
  dryRun: true
})
```

## 🚨 Error Handling

Common template management errors:

```typescript
try {
  const template = await invoke('create_research_template', templateData)
} catch (error) {
  if (error.includes('TEMPLATE_NAME_EXISTS')) {
    // Handle duplicate template name
  } else if (error.includes('INVALID_TEMPLATE_FORMAT')) {
    // Handle invalid template structure
  } else if (error.includes('TEMPLATE_NOT_FOUND')) {
    // Handle missing template
  } else if (error.includes('PERMISSION_DENIED')) {
    // Handle insufficient permissions
  }
}
```

## 📚 Related Documentation

- [Research Workflow API](./research-workflow.md)
- [Configuration API](./configuration.md)
- [Analytics API](./analytics.md)

---

**Next**: Learn about [Output Processing API](./output-processing.md) for result formatting and export.
