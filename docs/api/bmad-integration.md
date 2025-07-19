# 🤖 BMAD Integration API

## Overview

The BMAD (BMad Master of the BMAD Method) Integration API provides comprehensive AI agent orchestration capabilities, enabling evidence-based documentation generation and multi-agent research coordination.

## 🎯 Core BMAD Operations

### Execute Research Enhanced Documentation Mode

Execute BMAD's Documentation Mode to generate professional handoff documents.

**Tauri Command:**
```typescript
const result = await invoke<BMadDocumentationResult>('execute_research_enhanced_documentation_mode', {
  projectDescription: 'AI-powered healthcare platform',
  requirements: [
    'Patient data management',
    'AI diagnosis assistance',
    'HIPAA compliance'
  ],
  targetAudience: 'development_team',
  outputFormat: 'professional_handoff',
  researchDepth: 'comprehensive',
  includeEvidence: true
})
```

**Response:**
```json
{
  "sessionId": "bmad_session_001",
  "status": "completed",
  "documents": {
    "prd": {
      "filename": "prd.md",
      "content": "# Product Requirements Document...",
      "evidenceSources": 15,
      "confidenceScore": 0.92
    },
    "architecture": {
      "filename": "architecture.md", 
      "content": "# Technical Architecture...",
      "evidenceSources": 22,
      "confidenceScore": 0.89
    },
    "checklist": {
      "filename": "checklist.md",
      "content": "# Development Checklist...",
      "evidenceSources": 8,
      "confidenceScore": 0.95
    }
  },
  "metadata": {
    "totalResearchTime": "18m 32s",
    "totalCost": "$14.50",
    "agentsInvolved": ["Product Manager AI", "Architect AI", "Task Breakdown Specialist AI"],
    "evidenceQuality": "high",
    "readinessScore": 0.94
  }
}
```

### Conduct Agent Research

Execute research using specific BMAD AI agents.

**Tauri Command:**
```typescript
const research = await invoke<AgentResearchResult>('conduct_agent_research', {
  agentType: 'product_manager', // 'product_manager', 'architect', 'platform_engineer'
  query: 'Best practices for HIPAA-compliant AI systems',
  researchType: 'comprehensive_analysis',
  evidenceRequirements: {
    minSources: 10,
    sourceTypes: ['academic', 'industry', 'regulatory'],
    confidenceThreshold: 0.8
  }
})
```

**Response:**
```json
{
  "agentId": "pm_agent_001",
  "agentName": "Product Manager AI (John)",
  "research": {
    "summary": "Comprehensive analysis of HIPAA compliance for AI systems...",
    "keyFindings": [
      "Technical safeguards require encryption at rest and in transit",
      "Administrative safeguards need role-based access controls",
      "Physical safeguards require secure data center practices"
    ],
    "recommendations": [
      "Implement zero-trust architecture",
      "Use federated learning for privacy preservation",
      "Establish comprehensive audit logging"
    ],
    "evidence": [
      {
        "source": "HHS HIPAA Security Rule",
        "relevance": 0.95,
        "credibility": 0.98,
        "citation": "45 CFR § 164.312(a)(1)"
      }
    ]
  },
  "metadata": {
    "researchDuration": "12m 45s",
    "sourcesAnalyzed": 18,
    "confidenceScore": 0.91,
    "costBreakdown": {
      "searchCosts": "$3.20",
      "analysisCosts": "$5.80",
      "total": "$9.00"
    }
  }
}
```

### Get Integration Health Status

Monitor the health and status of BMAD integration.

**Tauri Command:**
```typescript
const health = await invoke<BMadHealthStatus>('get_integration_health_status')
```

**Response:**
```json
{
  "status": "healthy",
  "services": {
    "agentOrchestrator": {
      "status": "operational",
      "activeAgents": 3,
      "queuedTasks": 2
    },
    "researchBridge": {
      "status": "operational",
      "connectionLatency": "45ms",
      "successRate": 0.98
    },
    "evidenceProcessor": {
      "status": "operational",
      "processingQueue": 1,
      "averageProcessingTime": "2.3s"
    }
  },
  "performance": {
    "averageResponseTime": "1.8s",
    "successRate": 0.97,
    "uptime": "99.8%"
  }
}
```

## 🧠 AI Agent Management

### Get Available BMAD Agents

Retrieve list of available AI agents and their capabilities.

**Tauri Command:**
```typescript
const agents = await invoke<BMadAgent[]>('get_bmad_agents')
```

**Response:**
```json
[
  {
    "id": "product_manager_john",
    "name": "Product Manager AI (John)",
    "type": "product_manager",
    "capabilities": [
      "requirements_analysis",
      "stakeholder_management",
      "market_research",
      "feature_prioritization"
    ],
    "specializations": [
      "B2B SaaS",
      "Healthcare Technology",
      "Enterprise Software"
    ],
    "status": "available",
    "currentLoad": 0.3,
    "averageTaskTime": "15m"
  },
  {
    "id": "architect_fred",
    "name": "Architect AI (Fred)",
    "type": "architect",
    "capabilities": [
      "system_design",
      "technology_selection",
      "scalability_planning",
      "security_architecture"
    ],
    "specializations": [
      "Microservices",
      "Cloud Architecture",
      "Security Design"
    ],
    "status": "available",
    "currentLoad": 0.1,
    "averageTaskTime": "20m"
  }
]
```

### Get Research Methodologies

Retrieve available research methodologies for BMAD agents.

**Tauri Command:**
```typescript
const methodologies = await invoke<ResearchMethodology[]>('get_research_methodologies')
```

### Get Research Types

Get available research types and their descriptions.

**Tauri Command:**
```typescript
const researchTypes = await invoke<ResearchType[]>('get_research_types')
```

**Response:**
```json
[
  {
    "id": "comprehensive_analysis",
    "name": "Comprehensive Analysis",
    "description": "Deep dive analysis with extensive evidence gathering",
    "estimatedDuration": "15-25 minutes",
    "evidenceRequirements": {
      "minSources": 15,
      "confidenceThreshold": 0.85
    }
  },
  {
    "id": "rapid_assessment",
    "name": "Rapid Assessment", 
    "description": "Quick analysis for time-sensitive decisions",
    "estimatedDuration": "5-10 minutes",
    "evidenceRequirements": {
      "minSources": 8,
      "confidenceThreshold": 0.75
    }
  }
]
```

## 📊 Integration Configuration

### Get Integration Configuration

Retrieve current BMAD integration settings.

**Tauri Command:**
```typescript
const config = await invoke<BMadIntegrationConfig>('get_integration_config')
```

**Response:**
```json
{
  "version": "3.0.0",
  "settings": {
    "defaultResearchDepth": "comprehensive",
    "evidenceThreshold": 0.8,
    "maxConcurrentAgents": 5,
    "costLimits": {
      "perSession": 25.00,
      "perAgent": 10.00,
      "daily": 100.00
    }
  },
  "agentConfigurations": {
    "product_manager": {
      "researchFocus": ["market_analysis", "user_needs", "competitive_landscape"],
      "outputStyle": "business_focused",
      "evidencePreference": "industry_reports"
    },
    "architect": {
      "researchFocus": ["technical_patterns", "scalability", "security"],
      "outputStyle": "technical_detailed",
      "evidencePreference": "technical_documentation"
    }
  }
}
```

### Test BMAD Integration

Perform integration health check and connectivity test.

**Tauri Command:**
```typescript
const testResult = await invoke<BMadIntegrationTest>('test_bmad_integration')
```

**Response:**
```json
{
  "success": true,
  "tests": {
    "agentConnectivity": {
      "status": "passed",
      "responseTime": "120ms"
    },
    "researchBridge": {
      "status": "passed", 
      "responseTime": "85ms"
    },
    "evidenceProcessing": {
      "status": "passed",
      "processingTime": "1.2s"
    }
  },
  "overallHealth": "excellent",
  "recommendations": []
}
```

## 📈 Integration Statistics

### Get Integration Statistics

Retrieve usage statistics and performance metrics.

**Tauri Command:**
```typescript
const stats = await invoke<BMadIntegrationStats>('get_integration_statistics', {
  timeframe: 'last_30_days'
})
```

**Response:**
```json
{
  "timeframe": "last_30_days",
  "usage": {
    "totalSessions": 145,
    "documentationModeUsage": 89,
    "agentResearchSessions": 56,
    "averageSessionDuration": "16m 32s"
  },
  "performance": {
    "averageResponseTime": "1.9s",
    "successRate": 0.96,
    "evidenceQuality": 0.89
  },
  "costs": {
    "totalCost": "$1,245.50",
    "averageCostPerSession": "$8.59",
    "costBreakdown": {
      "research": "$856.20",
      "processing": "$389.30"
    }
  },
  "agentUsage": {
    "product_manager": 45,
    "architect": 38,
    "platform_engineer": 23,
    "task_breakdown": 39
  }
}
```

## 🔧 Advanced Features

### Multi-Agent Collaboration

Execute collaborative research with multiple agents.

**Tauri Command:**
```typescript
const collaboration = await invoke<MultiAgentCollaboration>('execute_multi_agent_research', {
  agents: ['product_manager', 'architect', 'security_engineer'],
  query: 'Design secure AI healthcare platform',
  collaborationMode: 'sequential', // 'parallel', 'sequential', 'consensus'
  evidenceSharing: true
})
```

### Evidence-Based Validation

Validate research findings with additional evidence.

**Tauri Command:**
```typescript
const validation = await invoke<EvidenceValidation>('validate_research_evidence', {
  researchId: 'research_001',
  validationCriteria: {
    sourceCredibility: 0.9,
    sourceDiversity: 0.8,
    recencyWeight: 0.7
  }
})
```

## 🚨 Error Handling

Common BMAD integration errors:

```typescript
try {
  const result = await invoke('execute_research_enhanced_documentation_mode', params)
} catch (error) {
  if (error.includes('AGENT_UNAVAILABLE')) {
    // Handle agent unavailability
  } else if (error.includes('INSUFFICIENT_EVIDENCE')) {
    // Handle low evidence quality
  } else if (error.includes('BUDGET_EXCEEDED')) {
    // Handle cost limits
  }
}
```

## 📚 Related Documentation

- [Research Workflow API](./research-workflow.md)
- [AI Marketplace API](./ai-marketplace.md)
- [Analytics API](./analytics.md)

---

**Next**: Explore [AI Marketplace API](./ai-marketplace.md) for community AI agents and models.
