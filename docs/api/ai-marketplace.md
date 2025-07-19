# 🛒 AI Marketplace API

## Overview

The AI Marketplace API provides access to a community-driven marketplace of AI agents, models, and research tools. Part of the V3.0.0 Global Intelligence Network, it enables sharing, discovery, and deployment of AI capabilities across the research community.

## 👤 User Management

### Register Marketplace User

Register as a user in the AI Marketplace.

**Tauri Command:**
```typescript
const user = await invoke<MarketplaceUser>('register_marketplace_user', {
  username: 'research_scientist',
  email: 'scientist@university.edu',
  displayName: 'Dr. Research Scientist',
  organization: 'Research University',
  profileInfo: {
    bio: 'AI researcher specializing in healthcare applications',
    expertise: ['machine_learning', 'healthcare_ai', 'clinical_research'],
    website: 'https://university.edu/~scientist',
    orcid: '0000-0000-0000-0000'
  }
})
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "research_scientist",
  "email": "scientist@university.edu",
  "displayName": "Dr. Research Scientist",
  "organization": "Research University",
  "profileInfo": {
    "bio": "AI researcher specializing in healthcare applications",
    "expertise": ["machine_learning", "healthcare_ai", "clinical_research"],
    "website": "https://university.edu/~scientist",
    "orcid": "0000-0000-0000-0000"
  },
  "reputation": {
    "score": 0,
    "level": "newcomer",
    "badges": []
  },
  "registeredAt": "2025-01-20T15:30:00Z",
  "status": "active"
}
```

### Get User Profile

Retrieve user profile information and statistics.

**Tauri Command:**
```typescript
const profile = await invoke<UserProfile>('get_marketplace_user_profile', {
  userId: '550e8400-e29b-41d4-a716-446655440000'
})
```

## 🤖 AI Agent Management

### Publish AI Agent

Publish an AI agent to the marketplace.

**Tauri Command:**
```typescript
const publishedAgent = await invoke<AIAgentMarketplace>('publish_ai_agent', {
  creatorId: '550e8400-e29b-41d4-a716-446655440000',
  agent: {
    name: 'Healthcare Research Assistant',
    description: 'Specialized AI agent for healthcare research and analysis',
    category: 'research_assistant',
    capabilities: [
      'medical_literature_analysis',
      'clinical_data_interpretation',
      'regulatory_compliance_checking',
      'research_methodology_guidance'
    ],
    version: '1.0.0',
    pricing: {
      model: 'freemium', // 'free', 'freemium', 'paid', 'subscription'
      freeUsage: {
        requestsPerMonth: 100,
        features: ['basic_analysis', 'literature_search']
      },
      paidTiers: [
        {
          name: 'Professional',
          pricePerMonth: 29.99,
          features: ['advanced_analysis', 'custom_models', 'priority_support'],
          requestsPerMonth: 1000
        }
      ]
    },
    technicalSpecs: {
      modelType: 'transformer',
      inputFormats: ['text', 'pdf', 'json'],
      outputFormats: ['text', 'json', 'structured_report'],
      averageResponseTime: '2.5s',
      accuracy: 0.94,
      languages: ['en', 'es', 'fr']
    },
    requirements: {
      minMemory: '2GB',
      recommendedMemory: '4GB',
      gpuRequired: false,
      internetRequired: true
    }
  }
})
```

**Response:**
```json
{
  "id": "agent_123",
  "name": "Healthcare Research Assistant",
  "description": "Specialized AI agent for healthcare research and analysis",
  "category": "research_assistant",
  "creator": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "research_scientist",
    "displayName": "Dr. Research Scientist"
  },
  "version": "1.0.0",
  "status": "published",
  "publishedAt": "2025-01-20T15:30:00Z",
  "marketplaceUrl": "https://marketplace.ai-research.org/agents/agent_123",
  "downloadCount": 0,
  "rating": {
    "average": 0.0,
    "count": 0
  },
  "verification": {
    "verified": false,
    "verificationLevel": "none"
  }
}
```

### Search AI Agents

Search for AI agents in the marketplace.

**Tauri Command:**
```typescript
const searchResults = await invoke<AIAgentSearchResult[]>('search_ai_agents', {
  query: 'healthcare research',
  filters: {
    categories: ['research_assistant', 'data_analysis'],
    pricing: ['free', 'freemium'],
    capabilities: ['medical_analysis'],
    minRating: 4.0,
    verified: true,
    languages: ['en']
  },
  sortBy: 'popularity', // 'relevance', 'rating', 'downloads', 'recent', 'popularity'
  limit: 20,
  offset: 0
})
```

**Response:**
```json
[
  {
    "id": "agent_123",
    "name": "Healthcare Research Assistant",
    "description": "Specialized AI agent for healthcare research and analysis",
    "category": "research_assistant",
    "creator": {
      "username": "research_scientist",
      "displayName": "Dr. Research Scientist",
      "reputation": 4.8
    },
    "version": "1.0.0",
    "rating": {
      "average": 4.7,
      "count": 156
    },
    "pricing": {
      "model": "freemium",
      "startingPrice": 0.00
    },
    "downloadCount": 2450,
    "lastUpdated": "2025-01-15T10:00:00Z",
    "verification": {
      "verified": true,
      "verificationLevel": "community"
    },
    "tags": ["healthcare", "research", "medical", "analysis"]
  }
]
```

### Install AI Agent

Install an AI agent for use in your research workflows.

**Tauri Command:**
```typescript
const installation = await invoke<AgentInstallationResult>('install_ai_agent', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  request: {
    agentId: 'agent_123',
    version: '1.0.0',
    installationType: 'local', // 'local', 'cloud', 'hybrid'
    configuration: {
      enabledFeatures: ['basic_analysis', 'literature_search'],
      customSettings: {
        responseLanguage: 'en',
        outputFormat: 'structured_report'
      }
    },
    licenseAccepted: true
  }
})
```

**Response:**
```json
{
  "installationId": "install_456",
  "agentId": "agent_123",
  "status": "completed",
  "installedAt": "2025-01-20T15:35:00Z",
  "installationPath": "/agents/healthcare_research_assistant",
  "configuration": {
    "enabledFeatures": ["basic_analysis", "literature_search"],
    "apiEndpoint": "https://api.agent.local:8080",
    "authToken": "agent_token_xyz789"
  },
  "usage": {
    "requestsUsed": 0,
    "requestsRemaining": 100,
    "resetDate": "2025-02-20T00:00:00Z"
  }
}
```

## 📊 Marketplace Analytics

### Get Agent Statistics

Retrieve detailed statistics for an AI agent.

**Tauri Command:**
```typescript
const agentStats = await invoke<AgentStatistics>('get_agent_statistics', {
  agentId: 'agent_123',
  timeframe: 'last_30_days',
  includeUsageMetrics: true
})
```

**Response:**
```json
{
  "agentId": "agent_123",
  "timeframe": "last_30_days",
  "downloads": {
    "total": 2450,
    "newDownloads": 245,
    "growth": 0.11
  },
  "usage": {
    "activeInstallations": 1890,
    "totalRequests": 125000,
    "averageRequestsPerUser": 66,
    "successRate": 0.97
  },
  "ratings": {
    "average": 4.7,
    "distribution": {
      "5_star": 156,
      "4_star": 89,
      "3_star": 23,
      "2_star": 8,
      "1_star": 4
    },
    "recentReviews": 45
  },
  "revenue": {
    "totalRevenue": 4567.89,
    "monthlyRecurring": 2890.45,
    "averageRevenuePerUser": 2.42
  }
}
```

### Get Marketplace Trends

Retrieve trending AI agents and marketplace insights.

**Tauri Command:**
```typescript
const trends = await invoke<MarketplaceTrends>('get_marketplace_trends', {
  timeframe: 'last_7_days',
  categories: ['research_assistant', 'data_analysis', 'nlp']
})
```

**Response:**
```json
{
  "trendingAgents": [
    {
      "agentId": "agent_123",
      "name": "Healthcare Research Assistant",
      "growthRate": 0.34,
      "downloadVelocity": 45
    }
  ],
  "popularCategories": [
    {
      "category": "research_assistant",
      "agentCount": 156,
      "totalDownloads": 45000,
      "growth": 0.23
    }
  ],
  "emergingTechnologies": [
    "multimodal_ai",
    "federated_learning",
    "quantum_ml"
  ],
  "marketInsights": {
    "totalAgents": 2450,
    "activeUsers": 12500,
    "totalDownloads": 450000,
    "averageRating": 4.3
  }
}
```

## 💰 Monetization & Billing

### Get Agent Revenue

Retrieve revenue information for published agents.

**Tauri Command:**
```typescript
const revenue = await invoke<AgentRevenue>('get_agent_revenue', {
  creatorId: '550e8400-e29b-41d4-a716-446655440000',
  agentId: 'agent_123',
  timeframe: 'last_90_days'
})
```

### Configure Agent Pricing

Update pricing configuration for a published agent.

**Tauri Command:**
```typescript
const pricingUpdate = await invoke<AgentPricing>('configure_agent_pricing', {
  agentId: 'agent_123',
  pricing: {
    model: 'subscription',
    tiers: [
      {
        name: 'Basic',
        pricePerMonth: 19.99,
        features: ['basic_analysis', 'standard_support'],
        requestsPerMonth: 500
      },
      {
        name: 'Professional',
        pricePerMonth: 49.99,
        features: ['advanced_analysis', 'priority_support', 'custom_models'],
        requestsPerMonth: 2000
      }
    ],
    trialPeriod: 14
  }
})
```

## 🔍 Quality & Verification

### Submit Agent for Verification

Submit an AI agent for community or official verification.

**Tauri Command:**
```typescript
const verification = await invoke<VerificationSubmission>('submit_agent_verification', {
  agentId: 'agent_123',
  verificationType: 'community', // 'community', 'official', 'enterprise'
  documentation: {
    technicalSpecs: 'https://docs.agent.com/specs',
    securityAudit: 'https://docs.agent.com/security',
    performanceBenchmarks: 'https://docs.agent.com/benchmarks'
  },
  testCases: [
    {
      name: 'Healthcare Literature Analysis',
      description: 'Test agent ability to analyze medical literature',
      expectedOutput: 'Structured analysis with key findings'
    }
  ]
})
```

### Rate and Review Agent

Provide rating and review for an installed agent.

**Tauri Command:**
```typescript
const review = await invoke<AgentReview>('rate_and_review_agent', {
  agentId: 'agent_123',
  userId: '550e8400-e29b-41d4-a716-446655440000',
  rating: 5,
  review: {
    title: 'Excellent for Healthcare Research',
    content: 'This agent has significantly improved our research workflow...',
    pros: ['Accurate analysis', 'Fast response', 'Easy integration'],
    cons: ['Limited customization options'],
    recommendedFor: ['healthcare_researchers', 'clinical_analysts']
  },
  usageContext: {
    usageDuration: '3_months',
    primaryUseCase: 'literature_analysis',
    organizationType: 'academic'
  }
})
```

## 🔧 Agent Development Tools

### Create Agent Template

Create a template for developing new AI agents.

**Tauri Command:**
```typescript
const template = await invoke<AgentTemplate>('create_agent_template', {
  name: 'Research Assistant Template',
  category: 'research_assistant',
  baseCapabilities: [
    'text_analysis',
    'data_processing',
    'report_generation'
  ],
  templateFiles: {
    configTemplate: 'agent_config.json',
    codeTemplate: 'agent_implementation.py',
    documentationTemplate: 'README.md'
  }
})
```

### Test Agent Performance

Test an agent's performance with standardized benchmarks.

**Tauri Command:**
```typescript
const performanceTest = await invoke<AgentPerformanceTest>('test_agent_performance', {
  agentId: 'agent_123',
  testSuite: 'healthcare_research_benchmark',
  testCases: [
    'literature_analysis_accuracy',
    'response_time_benchmark',
    'resource_usage_test'
  ]
})
```

## 🚨 Error Handling

Common AI Marketplace errors:

```typescript
try {
  const agent = await invoke('install_ai_agent', params)
} catch (error) {
  if (error.includes('AGENT_NOT_FOUND')) {
    // Handle missing agent
  } else if (error.includes('INSUFFICIENT_CREDITS')) {
    // Handle payment/credit issues
  } else if (error.includes('INCOMPATIBLE_VERSION')) {
    // Handle version compatibility
  } else if (error.includes('INSTALLATION_FAILED')) {
    // Handle installation errors
  }
}
```

## 📚 Related Documentation

- [Federated Research API](./federated-research.md)
- [BMAD Integration API](./bmad-integration.md)
- [Machine Learning API](./machine-learning.md)

---

**Next**: Explore [Machine Learning API](./machine-learning.md) for ML models and predictions.
