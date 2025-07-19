# 📄 Output Processing API

## Overview

The Output Processing API provides comprehensive functionality for formatting, transforming, and exporting research results in various formats. It supports multiple output formats, custom styling, and advanced processing options.

## 📋 Core Output Operations

### Process Research Output

Transform raw research results into formatted output.

**Tauri Command:**
```typescript
const processedOutput = await invoke<ProcessedOutput>('process_research_output', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000',
  format: 'comprehensive_report', // 'summary', 'detailed', 'comprehensive_report', 'executive_brief'
  options: {
    includeMetadata: true,
    includeSources: true,
    includeMethodology: true,
    includeVisualizations: true,
    customStyling: {
      theme: 'professional',
      brandColors: ['#1f2937', '#3b82f6'],
      logoUrl: 'https://example.com/logo.png'
    }
  }
})
```

**Response:**
```json
{
  "outputId": "output_123",
  "workflowId": "550e8400-e29b-41d4-a716-446655440000",
  "format": "comprehensive_report",
  "content": {
    "title": "AI in Healthcare: Comprehensive Research Analysis",
    "executiveSummary": "This research provides comprehensive insights...",
    "sections": [
      {
        "title": "Key Findings",
        "content": "The research identified several key trends...",
        "subsections": []
      }
    ],
    "appendices": [
      {
        "title": "Sources",
        "content": "Complete list of analyzed sources..."
      }
    ]
  },
  "metadata": {
    "generatedAt": "2025-01-20T15:30:00Z",
    "processingTime": "2.3s",
    "wordCount": 2450,
    "pageCount": 12,
    "sourceCount": 45
  }
}
```

### Export Research Results

Export processed research results to various file formats.

**Tauri Command:**
```typescript
const exportResult = await invoke<ExportResult>('export_research_results', {
  outputId: 'output_123',
  format: 'pdf', // 'pdf', 'docx', 'html', 'markdown', 'json', 'csv', 'xlsx'
  options: {
    includeTableOfContents: true,
    includePageNumbers: true,
    includeWatermark: false,
    compression: 'medium',
    password: 'optional_password',
    metadata: {
      author: 'Research Team',
      title: 'AI in Healthcare Analysis',
      subject: 'Research Report',
      keywords: ['AI', 'Healthcare', 'Research']
    }
  }
})
```

**Response:**
```json
{
  "exportId": "export_456",
  "filePath": "/exports/ai_healthcare_analysis_2025.pdf",
  "fileSize": "2.4MB",
  "format": "pdf",
  "downloadUrl": "https://api.example.com/exports/export_456/download",
  "expiresAt": "2025-01-27T15:30:00Z",
  "checksum": "sha256:abc123...",
  "metadata": {
    "pages": 12,
    "wordCount": 2450,
    "exportedAt": "2025-01-20T15:32:00Z"
  }
}
```

### Generate Custom Report

Create a custom report with specific sections and formatting.

**Tauri Command:**
```typescript
const customReport = await invoke<CustomReport>('generate_custom_report', {
  workflowId: '550e8400-e29b-41d4-a716-446655440000',
  template: {
    title: 'Executive Summary: AI in Healthcare',
    sections: [
      {
        type: 'executive_summary',
        title: 'Executive Summary',
        maxLength: 500
      },
      {
        type: 'key_findings',
        title: 'Key Findings',
        maxItems: 5
      },
      {
        type: 'recommendations',
        title: 'Strategic Recommendations',
        maxItems: 3
      },
      {
        type: 'sources',
        title: 'Key Sources',
        maxItems: 10,
        sortBy: 'relevance'
      }
    ],
    styling: {
      theme: 'executive',
      includeCharts: true,
      includeLogos: true
    }
  }
})
```

## 📊 Output Formatting Options

### Get Available Formats

Retrieve all available output formats and their capabilities.

**Tauri Command:**
```typescript
const formats = await invoke<OutputFormat[]>('get_available_output_formats')
```

**Response:**
```json
[
  {
    "id": "pdf",
    "name": "PDF Document",
    "description": "Professional PDF with full formatting support",
    "capabilities": {
      "supportsImages": true,
      "supportsCharts": true,
      "supportsTableOfContents": true,
      "supportsWatermarks": true,
      "supportsPassword": true,
      "maxFileSize": "50MB"
    },
    "extensions": [".pdf"]
  },
  {
    "id": "docx",
    "name": "Microsoft Word Document",
    "description": "Editable Word document with full formatting",
    "capabilities": {
      "supportsImages": true,
      "supportsCharts": true,
      "supportsComments": true,
      "supportsTrackedChanges": true,
      "maxFileSize": "25MB"
    },
    "extensions": [".docx"]
  }
]
```

### Get Output Templates

Retrieve available output templates for different use cases.

**Tauri Command:**
```typescript
const templates = await invoke<OutputTemplate[]>('get_output_templates', {
  category: 'business', // 'academic', 'business', 'technical', 'executive'
  format: 'pdf'
})
```

**Response:**
```json
[
  {
    "id": "executive_brief",
    "name": "Executive Brief",
    "description": "Concise executive summary format",
    "category": "business",
    "sections": [
      "executive_summary",
      "key_findings",
      "recommendations",
      "next_steps"
    ],
    "estimatedPages": "2-4",
    "targetAudience": "C-level executives"
  }
]
```

## 🎨 Styling & Customization

### Apply Custom Styling

Apply custom styling and branding to research outputs.

**Tauri Command:**
```typescript
const styledOutput = await invoke<StyledOutput>('apply_custom_styling', {
  outputId: 'output_123',
  styling: {
    theme: 'corporate',
    colors: {
      primary: '#1f2937',
      secondary: '#3b82f6',
      accent: '#10b981'
    },
    fonts: {
      heading: 'Arial Bold',
      body: 'Arial',
      monospace: 'Courier New'
    },
    branding: {
      logoUrl: 'https://company.com/logo.png',
      companyName: 'Research Corp',
      footerText: 'Confidential Research Report'
    },
    layout: {
      margins: '1in',
      lineSpacing: 1.5,
      pageOrientation: 'portrait'
    }
  }
})
```

### Create Custom Template

Create a reusable output template for consistent formatting.

**Tauri Command:**
```typescript
const template = await invoke<OutputTemplate>('create_custom_template', {
  name: 'Company Research Template',
  description: 'Standard template for company research reports',
  sections: [
    {
      type: 'cover_page',
      title: 'Cover Page',
      required: true
    },
    {
      type: 'executive_summary',
      title: 'Executive Summary',
      maxLength: 750,
      required: true
    },
    {
      type: 'methodology',
      title: 'Research Methodology',
      required: false
    }
  ],
  styling: {
    theme: 'professional',
    includePageNumbers: true,
    includeTableOfContents: true
  }
})
```

## 📈 Output Analytics

### Get Output Statistics

Retrieve statistics about output generation and usage.

**Tauri Command:**
```typescript
const stats = await invoke<OutputStatistics>('get_output_statistics', {
  timeframe: 'last_30_days',
  groupBy: 'format' // 'format', 'template', 'user', 'date'
})
```

**Response:**
```json
{
  "totalOutputs": 245,
  "formatBreakdown": {
    "pdf": 145,
    "docx": 67,
    "html": 23,
    "markdown": 10
  },
  "averageProcessingTime": "2.1s",
  "averageFileSize": "1.8MB",
  "popularTemplates": [
    {
      "templateId": "executive_brief",
      "usage": 89,
      "satisfaction": 4.7
    }
  ],
  "trends": {
    "monthOverMonth": 0.15,
    "formatTrends": {
      "pdf": 0.08,
      "docx": 0.12
    }
  }
}
```

## 🔄 Batch Processing

### Process Multiple Outputs

Process multiple research results in a single batch operation.

**Tauri Command:**
```typescript
const batchResult = await invoke<BatchProcessingResult>('process_batch_outputs', {
  workflowIds: [
    '550e8400-e29b-41d4-a716-446655440000',
    '660f9511-f3ac-52e5-b827-557766551111'
  ],
  format: 'pdf',
  options: {
    combineIntoSingle: false,
    applyConsistentStyling: true,
    includeComparison: true
  }
})
```

### Export Batch Results

Export multiple outputs as a single archive or separate files.

**Tauri Command:**
```typescript
const batchExport = await invoke<BatchExportResult>('export_batch_results', {
  outputIds: ['output_123', 'output_456', 'output_789'],
  archiveFormat: 'zip', // 'zip', 'tar', 'separate'
  includeManifest: true,
  compression: 'high'
})
```

## 🔍 Output Validation & Quality

### Validate Output Quality

Check the quality and completeness of generated outputs.

**Tauri Command:**
```typescript
const validation = await invoke<OutputValidation>('validate_output_quality', {
  outputId: 'output_123',
  criteria: {
    minWordCount: 1000,
    minSourceCount: 10,
    requireExecutiveSummary: true,
    checkGrammar: true,
    checkFactualConsistency: true
  }
})
```

**Response:**
```json
{
  "isValid": true,
  "qualityScore": 0.92,
  "checks": {
    "wordCount": {
      "passed": true,
      "actual": 2450,
      "required": 1000
    },
    "sourceCount": {
      "passed": true,
      "actual": 45,
      "required": 10
    },
    "grammarCheck": {
      "passed": true,
      "score": 0.95,
      "issues": []
    }
  },
  "recommendations": [
    "Consider adding more visual elements",
    "Include additional data visualizations"
  ]
}
```

## 🔧 Advanced Processing

### Apply AI Enhancement

Use AI to enhance and improve output quality.

**Tauri Command:**
```typescript
const enhancedOutput = await invoke<EnhancedOutput>('apply_ai_enhancement', {
  outputId: 'output_123',
  enhancements: {
    improveReadability: true,
    addVisualizations: true,
    enhanceStructure: true,
    generateSummaries: true,
    checkFactualAccuracy: true
  }
})
```

### Generate Output Comparison

Compare multiple research outputs to identify differences and similarities.

**Tauri Command:**
```typescript
const comparison = await invoke<OutputComparison>('generate_output_comparison', {
  outputIds: ['output_123', 'output_456'],
  comparisonType: 'detailed', // 'summary', 'detailed', 'statistical'
  includeVisualizations: true
})
```

## 🚨 Error Handling

Common output processing errors:

```typescript
try {
  const output = await invoke('process_research_output', params)
} catch (error) {
  if (error.includes('WORKFLOW_NOT_FOUND')) {
    // Handle missing workflow
  } else if (error.includes('INVALID_FORMAT')) {
    // Handle unsupported format
  } else if (error.includes('PROCESSING_FAILED')) {
    // Handle processing errors
  } else if (error.includes('EXPORT_FAILED')) {
    // Handle export errors
  }
}
```

## 📚 Related Documentation

- [Research Workflow API](./research-workflow.md)
- [Template Management API](./template-management.md)
- [Analytics API](./analytics.md)

---

**Next**: Explore [Federated Research API](./federated-research.md) for cross-organization collaboration.
