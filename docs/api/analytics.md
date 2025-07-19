# 📊 Analytics API

## Overview

The Analytics API provides comprehensive business intelligence, usage analytics, and performance insights for the Free Deep Research System.

## 📈 Usage Analytics

### Get Usage Statistics

Retrieve detailed usage statistics and metrics.

**Tauri Command:**
```typescript
const stats = await invoke<UsageStatistics>('get_usage_analytics', {
  timeframe: 'last_30_days', // 'last_7_days', 'last_30_days', 'last_90_days', 'custom'
  granularity: 'daily', // 'hourly', 'daily', 'weekly', 'monthly'
  metrics: ['workflows', 'api_calls', 'costs', 'performance']
})
```

**Response:**
```json
{
  "timeframe": "last_30_days",
  "summary": {
    "totalWorkflows": 1250,
    "successfulWorkflows": 1187,
    "failedWorkflows": 63,
    "averageWorkflowDuration": "18m 32s",
    "totalCost": "$2,845.50",
    "averageCostPerWorkflow": "$2.28"
  },
  "trends": {
    "workflowGrowth": 0.15,
    "costEfficiency": 0.08,
    "successRateImprovement": 0.03
  },
  "dailyBreakdown": [
    {
      "date": "2025-01-20",
      "workflows": 45,
      "successRate": 0.96,
      "averageDuration": "17m 45s",
      "totalCost": "$98.50"
    }
  ]
}
```

### Get Performance Metrics

Retrieve system performance analytics.

**Tauri Command:**
```typescript
const performance = await invoke<PerformanceMetrics>('get_performance_metrics', {
  timeframe: 'last_24_hours',
  includeBreakdown: true
})
```

**Response:**
```json
{
  "system": {
    "averageResponseTime": "185ms",
    "p95ResponseTime": "450ms",
    "p99ResponseTime": "1.2s",
    "uptime": "99.97%",
    "errorRate": "0.03%"
  },
  "research": {
    "averageWorkflowTime": "16m 45s",
    "successRate": "96.8%",
    "averageSourcesPerWorkflow": 42,
    "averageRelevanceScore": 0.87
  },
  "api": {
    "totalRequests": 125000,
    "requestsPerSecond": 145,
    "cacheHitRate": "78%",
    "rateLimitHits": 23
  }
}
```

## 💰 Cost Analytics

### Get Cost Breakdown

Analyze costs across different services and time periods.

**Tauri Command:**
```typescript
const costs = await invoke<CostAnalytics>('get_cost_analytics', {
  timeframe: 'last_30_days',
  groupBy: 'service', // 'service', 'user', 'methodology', 'date'
  includePredictions: true
})
```

**Response:**
```json
{
  "totalCost": "$2,845.50",
  "breakdown": {
    "openrouter": "$1,245.20",
    "serpapi": "$856.30",
    "jina_ai": "$445.80",
    "firecrawl": "$298.20"
  },
  "trends": {
    "monthOverMonth": -0.08,
    "costPerWorkflow": "$2.28",
    "costEfficiencyImprovement": 0.12
  },
  "predictions": {
    "nextMonth": "$2,650.00",
    "confidence": 0.85,
    "factors": ["seasonal_trends", "usage_growth", "efficiency_improvements"]
  }
}
```

### Get Budget Tracking

Monitor budget usage and alerts.

**Tauri Command:**
```typescript
const budget = await invoke<BudgetTracking>('get_budget_tracking')
```

**Response:**
```json
{
  "currentPeriod": {
    "budget": "$5000.00",
    "spent": "$2845.50",
    "remaining": "$2154.50",
    "utilizationRate": 0.569
  },
  "alerts": [
    {
      "type": "warning",
      "message": "Approaching 60% of monthly budget",
      "threshold": 0.6,
      "current": 0.569
    }
  ],
  "projections": {
    "endOfPeriod": "$4,250.00",
    "overBudgetRisk": "low"
  }
}
```

## 👥 User Analytics

### Get User Activity

Analyze user behavior and engagement patterns.

**Tauri Command:**
```typescript
const userActivity = await invoke<UserActivityAnalytics>('get_user_activity', {
  timeframe: 'last_30_days',
  includeSegmentation: true
})
```

**Response:**
```json
{
  "activeUsers": {
    "total": 245,
    "new": 23,
    "returning": 222,
    "powerUsers": 45
  },
  "engagement": {
    "averageSessionDuration": "45m 20s",
    "workflowsPerUser": 5.1,
    "returnRate": 0.78
  },
  "segmentation": {
    "byRole": {
      "researchers": 145,
      "analysts": 67,
      "managers": 33
    },
    "byUsage": {
      "light": 123,
      "moderate": 89,
      "heavy": 33
    }
  }
}
```

### Get Feature Usage

Track feature adoption and usage patterns.

**Tauri Command:**
```typescript
const featureUsage = await invoke<FeatureUsageAnalytics>('get_feature_usage', {
  timeframe: 'last_30_days'
})
```

**Response:**
```json
{
  "features": {
    "bmadIntegration": {
      "usage": 0.68,
      "trend": 0.15,
      "userAdoption": 167
    },
    "advancedAnalytics": {
      "usage": 0.45,
      "trend": 0.23,
      "userAdoption": 110
    },
    "templateManagement": {
      "usage": 0.72,
      "trend": 0.08,
      "userAdoption": 176
    }
  },
  "adoption": {
    "newFeatures": ["quantum_ready", "knowledge_graph"],
    "growingFeatures": ["bmad_integration", "advanced_analytics"],
    "matureFeatures": ["research_workflow", "api_management"]
  }
}
```

## 🔍 Research Analytics

### Get Research Insights

Analyze research patterns and effectiveness.

**Tauri Command:**
```typescript
const insights = await invoke<ResearchInsights>('get_research_insights', {
  timeframe: 'last_90_days',
  includeMethodologyComparison: true
})
```

**Response:**
```json
{
  "methodologies": {
    "hybrid": {
      "usage": 0.52,
      "successRate": 0.97,
      "averageDuration": "18m 30s",
      "averageCost": "$14.50",
      "userSatisfaction": 4.6
    },
    "don_lim": {
      "usage": 0.31,
      "successRate": 0.95,
      "averageDuration": "22m 15s",
      "averageCost": "$18.20",
      "userSatisfaction": 4.7
    },
    "nick_scamara": {
      "usage": 0.17,
      "successRate": 0.94,
      "averageDuration": "16m 45s",
      "averageCost": "$12.80",
      "userSatisfaction": 4.5
    }
  },
  "qualityMetrics": {
    "averageRelevanceScore": 0.87,
    "averageCredibilityScore": 0.89,
    "sourceDiversity": 0.82,
    "evidenceStrength": 0.85
  }
}
```

### Get Topic Analysis

Analyze research topics and trends.

**Tauri Command:**
```typescript
const topics = await invoke<TopicAnalysis>('get_topic_analysis', {
  timeframe: 'last_30_days',
  minFrequency: 5
})
```

**Response:**
```json
{
  "trendingTopics": [
    {
      "topic": "AI in healthcare",
      "frequency": 89,
      "growth": 0.34,
      "averageQuality": 0.91
    },
    {
      "topic": "Machine learning ethics",
      "frequency": 67,
      "growth": 0.28,
      "averageQuality": 0.88
    }
  ],
  "emergingTopics": [
    "Quantum computing applications",
    "Sustainable AI development",
    "Edge AI deployment"
  ],
  "topicClusters": {
    "artificial_intelligence": ["machine learning", "deep learning", "neural networks"],
    "healthcare_technology": ["medical AI", "telemedicine", "health informatics"]
  }
}
```

## 📊 Business Intelligence

### Get Executive Dashboard

Retrieve high-level business metrics for executives.

**Tauri Command:**
```typescript
const dashboard = await invoke<ExecutiveDashboard>('get_executive_dashboard', {
  timeframe: 'last_quarter'
})
```

**Response:**
```json
{
  "kpis": {
    "totalRevenue": "$45,600.00",
    "userGrowth": 0.23,
    "churnRate": 0.05,
    "nps": 8.4,
    "systemUptime": "99.97%"
  },
  "trends": {
    "userEngagement": "increasing",
    "costEfficiency": "improving",
    "featureAdoption": "strong",
    "customerSatisfaction": "high"
  },
  "alerts": [
    {
      "type": "opportunity",
      "message": "BMAD integration showing 34% growth in adoption",
      "priority": "medium"
    }
  ]
}
```

### Generate Business Report

Create comprehensive business intelligence reports.

**Tauri Command:**
```typescript
const report = await invoke<BusinessReport>('generate_business_report', {
  reportType: 'monthly_summary', // 'weekly', 'monthly_summary', 'quarterly_review'
  includeCharts: true,
  format: 'pdf'
})
```

## 🎯 Predictive Analytics

### Get Usage Predictions

Predict future usage patterns and resource needs.

**Tauri Command:**
```typescript
const predictions = await invoke<UsagePredictions>('get_usage_predictions', {
  horizon: '30_days', // '7_days', '30_days', '90_days'
  confidence: 0.85
})
```

**Response:**
```json
{
  "predictions": {
    "workflowVolume": {
      "predicted": 1450,
      "confidence": 0.87,
      "trend": "increasing"
    },
    "resourceNeeds": {
      "cpu": "15% increase",
      "memory": "12% increase",
      "storage": "8% increase"
    },
    "costs": {
      "predicted": "$3,200.00",
      "confidence": 0.82,
      "factors": ["usage_growth", "efficiency_improvements"]
    }
  }
}
```

## 📈 Custom Analytics

### Create Custom Dashboard

Build custom analytics dashboards.

**Tauri Command:**
```typescript
const dashboard = await invoke<CustomDashboard>('create_custom_dashboard', {
  name: 'Research Team Dashboard',
  widgets: [
    {
      "type": "metric",
      "title": "Success Rate",
      "query": "research_success_rate",
      "timeframe": "last_7_days"
    },
    {
      "type": "chart",
      "title": "Daily Workflows",
      "query": "daily_workflow_count",
      "chartType": "line"
    }
  ]
})
```

### Export Analytics Data

Export analytics data for external analysis.

**Tauri Command:**
```typescript
const exportData = await invoke<AnalyticsExport>('export_analytics_data', {
  metrics: ['usage', 'performance', 'costs'],
  timeframe: 'last_30_days',
  format: 'csv', // 'csv', 'json', 'excel'
  includeMetadata: true
})
```

## 🚨 Error Handling

Common analytics errors:

```typescript
try {
  const analytics = await invoke('get_usage_analytics', params)
} catch (error) {
  if (error.includes('INSUFFICIENT_DATA')) {
    // Handle insufficient data for analysis
  } else if (error.includes('INVALID_TIMEFRAME')) {
    // Handle invalid timeframe
  } else if (error.includes('PERMISSION_DENIED')) {
    // Handle access restrictions
  }
}
```

## 📚 Related Documentation

- [Research Workflow API](./research-workflow.md)
- [BMAD Integration API](./bmad-integration.md)
- [Monitoring API](./monitoring.md)

---

**Next**: Explore [Monitoring API](./monitoring.md) for system health and performance monitoring.
