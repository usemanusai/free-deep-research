# 🔌 API Integration Guide

## Overview

The Free Deep Research System provides comprehensive APIs for integrating research capabilities into your applications, workflows, and business processes. This guide covers everything from basic setup to advanced integration patterns.

## 🎯 Integration Approaches

### Desktop Application Integration (Tauri Commands)
- **Best for**: Desktop applications, local development, offline capabilities
- **Technology**: Tauri commands with TypeScript/JavaScript
- **Authentication**: Local API key management
- **Performance**: Direct system access, fastest response times

### REST API Integration (Distributed)
- **Best for**: Web applications, microservices, cloud deployments
- **Technology**: HTTP REST endpoints with JSON
- **Authentication**: Bearer token authentication
- **Performance**: Network-dependent, highly scalable

### Webhook Integration
- **Best for**: Event-driven workflows, automation, notifications
- **Technology**: HTTP POST callbacks
- **Authentication**: Signature verification
- **Performance**: Real-time event processing

## 🚀 Quick Start Setup

### Prerequisites

Before integrating, ensure you have:
- ✅ Free Deep Research system installed and running
- ✅ Valid API keys for external services (OpenRouter, SerpAPI, etc.)
- ✅ Development environment set up
- ✅ Basic understanding of REST APIs or Tauri commands

### API Key Configuration

#### **Desktop Application Setup**
```typescript
// Configure API keys in desktop app
import { invoke } from '@tauri-apps/api/core'

// Set API keys
await invoke('set_api_key', {
  service: 'openrouter',
  key: 'your-openrouter-api-key'
})

await invoke('set_api_key', {
  service: 'serpapi',
  key: 'your-serpapi-key'
})

// Verify configuration
const health = await invoke('health_check')
console.log('System health:', health)
```

#### **REST API Setup**
```bash
# Set environment variables
export OPENROUTER_API_KEY="your-openrouter-api-key"
export SERPAPI_KEY="your-serpapi-key"
export TAVILY_API_KEY="your-tavily-key"

# Start the API server
./free-deep-research-server --port 8080
```

### Authentication

#### **Tauri Commands (Desktop)**
```typescript
// Authentication is handled automatically
// API keys are stored securely in the desktop app
const result = await invoke('create_research_workflow', {
  name: 'My Research',
  query: 'AI in healthcare'
})
```

#### **REST API Authentication**
```javascript
// Generate API token
const response = await fetch('http://localhost:8080/api/auth/token', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    username: 'your-username',
    password: 'your-password'
  })
})

const { token } = await response.json()

// Use token for subsequent requests
const headers = {
  'Authorization': `Bearer ${token}`,
  'Content-Type': 'application/json'
}
```

## 🔬 Core Research Operations

### Creating Research Workflows

#### **Tauri Command Example**
```typescript
import { invoke } from '@tauri-apps/api/core'

// Create a new research workflow
const workflowId = await invoke('create_research_workflow', {
  name: 'Market Analysis - AI Tools',
  query: 'artificial intelligence customer service tools market trends 2024',
  methodology: 'hybrid', // 'academic', 'business', 'hybrid'
  parameters: {
    maxSources: 30,
    qualityThreshold: 0.8,
    timeRange: 'last_2_years',
    languages: ['en'],
    sourceTypes: ['academic', 'industry_reports', 'news']
  }
})

console.log('Created workflow:', workflowId)
```

#### **REST API Example**
```javascript
// Create research workflow via REST API
const createWorkflow = async () => {
  const response = await fetch('http://localhost:8080/api/research/workflows', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      name: 'Competitive Analysis',
      query: 'SaaS customer support tools competitive landscape',
      methodology: 'business',
      parameters: {
        maxSources: 25,
        qualityThreshold: 0.75,
        budgetLimit: 10.00,
        outputFormat: 'detailed_report'
      }
    })
  })
  
  const workflow = await response.json()
  return workflow.id
}
```

### Executing Research

#### **Synchronous Execution**
```typescript
// Execute research and wait for completion
const results = await invoke('execute_research_sync', {
  workflowId: workflowId,
  timeout: 300 // 5 minutes
})

console.log('Research completed:', results)
```

#### **Asynchronous Execution with Monitoring**
```typescript
// Start research asynchronously
await invoke('execute_research_async', { workflowId })

// Monitor progress
const monitorProgress = async () => {
  const interval = setInterval(async () => {
    const status = await invoke('get_research_status', { workflowId })
    
    console.log(`Progress: ${status.progress}%`)
    console.log(`Phase: ${status.currentPhase}`)
    console.log(`Sources found: ${status.sourcesFound}`)
    
    if (status.status === 'completed') {
      clearInterval(interval)
      const results = await invoke('get_research_results', { workflowId })
      console.log('Research completed:', results)
    } else if (status.status === 'failed') {
      clearInterval(interval)
      console.error('Research failed:', status.error)
    }
  }, 5000) // Check every 5 seconds
}

monitorProgress()
```

### Retrieving Results

#### **Get Research Results**
```typescript
// Get complete research results
const results = await invoke('get_research_results', {
  workflowId: workflowId,
  includeRawData: false,
  format: 'structured' // 'structured', 'summary', 'raw'
})

// Results structure
interface ResearchResults {
  id: string
  name: string
  status: 'completed' | 'failed' | 'in_progress'
  summary: {
    keyFindings: string[]
    recommendations: string[]
    confidenceScore: number
    sourceCount: number
  }
  sources: Array<{
    title: string
    url: string
    type: 'academic' | 'industry' | 'news'
    qualityScore: number
    relevanceScore: number
    publishedDate: string
  }>
  analysis: {
    methodology: string
    processingTime: number
    costBreakdown: {
      total: number
      apiCosts: Record<string, number>
    }
  }
}
```

#### **Export Research Results**
```typescript
// Export results in various formats
const exportResult = await invoke('export_research', {
  workflowId: workflowId,
  format: 'pdf', // 'pdf', 'docx', 'html', 'json', 'csv'
  template: 'business_report', // 'academic', 'business_report', 'executive_summary'
  options: {
    includeCitations: true,
    includeCharts: true,
    citationStyle: 'apa'
  }
})

// exportResult contains file path or base64 data
console.log('Export completed:', exportResult.filePath)
```

## 📊 Advanced Integration Patterns

### Real-Time Research Monitoring

#### **WebSocket Integration**
```typescript
// Connect to real-time updates
import { WebSocketManager } from './websocket-manager'

const ws = new WebSocketManager('ws://localhost:8080/ws')

ws.on('research_progress', (data) => {
  console.log(`Research ${data.workflowId}: ${data.progress}%`)
  updateProgressBar(data.progress)
})

ws.on('source_found', (data) => {
  console.log(`New source: ${data.title} (Quality: ${data.qualityScore})`)
  addSourceToUI(data)
})

ws.on('research_completed', (data) => {
  console.log(`Research completed: ${data.workflowId}`)
  displayResults(data.results)
})
```

#### **Polling-Based Monitoring**
```typescript
// Alternative polling approach
class ResearchMonitor {
  private intervalId: NodeJS.Timeout | null = null
  
  async startMonitoring(workflowId: string, callback: (status: any) => void) {
    this.intervalId = setInterval(async () => {
      try {
        const status = await invoke('get_research_status', { workflowId })
        callback(status)
        
        if (status.status === 'completed' || status.status === 'failed') {
          this.stopMonitoring()
        }
      } catch (error) {
        console.error('Monitoring error:', error)
        this.stopMonitoring()
      }
    }, 3000)
  }
  
  stopMonitoring() {
    if (this.intervalId) {
      clearInterval(this.intervalId)
      this.intervalId = null
    }
  }
}
```

### Batch Processing

#### **Multiple Research Sessions**
```typescript
// Process multiple research topics
const batchResearch = async (topics: string[]) => {
  const workflows = await Promise.all(
    topics.map(topic => 
      invoke('create_research_workflow', {
        name: `Batch Research: ${topic}`,
        query: topic,
        methodology: 'hybrid',
        parameters: { maxSources: 20, qualityThreshold: 0.8 }
      })
    )
  )
  
  // Execute all research sessions
  const results = await Promise.all(
    workflows.map(workflowId =>
      invoke('execute_research_sync', { workflowId, timeout: 600 })
    )
  )
  
  return results
}

// Usage
const topics = [
  'AI in healthcare 2024',
  'Remote work productivity trends',
  'Cybersecurity threats cloud computing'
]

const batchResults = await batchResearch(topics)
```

### Template-Based Integration

#### **Using Research Templates**
```typescript
// Get available templates
const templates = await invoke('get_research_templates', {
  category: 'business', // 'academic', 'business', 'technical'
  tags: ['market-analysis', 'competitive-intelligence']
})

// Create research from template
const workflowId = await invoke('create_research_from_template', {
  templateId: templates[0].id,
  parameters: {
    research_topic: 'AI-powered customer service tools',
    geographic_scope: 'global',
    time_range: 'last_2_years',
    competitor_list: ['Zendesk', 'Intercom', 'Freshworks']
  }
})
```

## 🔗 External System Integration

### CRM Integration Example

#### **Salesforce Integration**
```typescript
// Integrate with Salesforce
class SalesforceResearchIntegration {
  constructor(private salesforceClient: any) {}
  
  async researchAccount(accountId: string) {
    // Get account details from Salesforce
    const account = await this.salesforceClient.sobject('Account').retrieve(accountId)
    
    // Create research workflow
    const workflowId = await invoke('create_research_workflow', {
      name: `Account Research: ${account.Name}`,
      query: `${account.Name} ${account.Industry} market analysis competitive landscape`,
      methodology: 'business',
      parameters: {
        maxSources: 25,
        qualityThreshold: 0.8,
        sourceTypes: ['industry_reports', 'news', 'company_data']
      }
    })
    
    // Execute research
    const results = await invoke('execute_research_sync', { workflowId })
    
    // Update Salesforce with research insights
    await this.salesforceClient.sobject('Account').update({
      Id: accountId,
      Market_Analysis__c: results.summary.keyFindings.join('\n'),
      Competitive_Position__c: results.analysis.competitivePosition,
      Research_Date__c: new Date().toISOString()
    })
    
    return results
  }
}
```

### Slack Integration Example

#### **Automated Research Notifications**
```typescript
// Slack webhook integration
class SlackResearchNotifier {
  constructor(private webhookUrl: string) {}
  
  async notifyResearchCompletion(workflowId: string) {
    const results = await invoke('get_research_results', { workflowId })
    
    const message = {
      text: `Research Completed: ${results.name}`,
      blocks: [
        {
          type: 'section',
          text: {
            type: 'mrkdwn',
            text: `*Research:* ${results.name}\n*Quality Score:* ${results.summary.confidenceScore}%\n*Sources:* ${results.summary.sourceCount}`
          }
        },
        {
          type: 'section',
          text: {
            type: 'mrkdwn',
            text: `*Key Findings:*\n${results.summary.keyFindings.slice(0, 3).map(f => `• ${f}`).join('\n')}`
          }
        },
        {
          type: 'actions',
          elements: [
            {
              type: 'button',
              text: { type: 'plain_text', text: 'View Results' },
              url: `https://your-app.com/research/${workflowId}`
            }
          ]
        }
      ]
    }
    
    await fetch(this.webhookUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(message)
    })
  }
}
```

## 🛡️ Security and Best Practices

### API Key Management

#### **Secure Storage**
```typescript
// Never hardcode API keys
// ❌ Bad
const apiKey = 'sk-1234567890abcdef'

// ✅ Good - Use environment variables
const apiKey = process.env.OPENROUTER_API_KEY

// ✅ Better - Use secure key management
const apiKey = await invoke('get_secure_api_key', { service: 'openrouter' })
```

#### **Key Rotation**
```typescript
// Implement key rotation
class APIKeyManager {
  async rotateKey(service: string, newKey: string) {
    // Validate new key
    const isValid = await this.validateKey(service, newKey)
    if (!isValid) {
      throw new Error('Invalid API key')
    }
    
    // Update key
    await invoke('set_api_key', { service, key: newKey })
    
    // Log rotation
    console.log(`API key rotated for service: ${service}`)
  }
  
  private async validateKey(service: string, key: string): Promise<boolean> {
    // Implement service-specific validation
    return true
  }
}
```

### Error Handling

#### **Robust Error Handling**
```typescript
// Comprehensive error handling
class ResearchAPIClient {
  async executeResearch(workflowId: string): Promise<any> {
    try {
      const results = await invoke('execute_research_sync', { workflowId })
      return results
    } catch (error) {
      if (error.message.includes('API_QUOTA_EXCEEDED')) {
        throw new APIQuotaExceededError('API quota exceeded. Please upgrade your plan.')
      } else if (error.message.includes('INVALID_API_KEY')) {
        throw new InvalidAPIKeyError('Invalid API key. Please check your configuration.')
      } else if (error.message.includes('NETWORK_ERROR')) {
        throw new NetworkError('Network error. Please check your connection.')
      } else {
        throw new ResearchError(`Research failed: ${error.message}`)
      }
    }
  }
}

// Custom error classes
class APIQuotaExceededError extends Error {
  constructor(message: string) {
    super(message)
    this.name = 'APIQuotaExceededError'
  }
}
```

### Rate Limiting

#### **Implement Rate Limiting**
```typescript
// Rate limiting for API calls
class RateLimiter {
  private requests: number[] = []
  
  constructor(
    private maxRequests: number,
    private windowMs: number
  ) {}
  
  async checkLimit(): Promise<boolean> {
    const now = Date.now()
    
    // Remove old requests outside the window
    this.requests = this.requests.filter(time => now - time < this.windowMs)
    
    if (this.requests.length >= this.maxRequests) {
      return false // Rate limit exceeded
    }
    
    this.requests.push(now)
    return true
  }
  
  async waitForAvailability(): Promise<void> {
    while (!(await this.checkLimit())) {
      await new Promise(resolve => setTimeout(resolve, 1000))
    }
  }
}

// Usage
const rateLimiter = new RateLimiter(10, 60000) // 10 requests per minute

await rateLimiter.waitForAvailability()
const results = await invoke('execute_research_sync', { workflowId })
```

## 🔄 Advanced Workflow Automation

### Event-Driven Research Automation

#### **Webhook-Based Triggers**
```javascript
// Advanced webhook configuration
class ResearchAutomationEngine {
  constructor(config) {
    this.webhookEndpoints = config.webhooks
    this.researchTemplates = config.templates
    this.notificationChannels = config.notifications
  }

  async setupEventTriggers() {
    // CRM integration - new lead research
    await this.registerWebhook({
      source: 'salesforce',
      event: 'lead.created',
      handler: this.researchNewLead.bind(this),
      template: 'company_research'
    })

    // Market monitoring - competitor mentions
    await this.registerWebhook({
      source: 'mention_tracker',
      event: 'competitor.mentioned',
      handler: this.analyzeCompetitorMention.bind(this),
      template: 'competitive_analysis'
    })

    // Content monitoring - industry news
    await this.registerWebhook({
      source: 'news_feed',
      event: 'industry.news',
      handler: this.analyzeIndustryNews.bind(this),
      template: 'trend_analysis'
    })
  }

  async researchNewLead(leadData) {
    const workflowId = await invoke('create_research_workflow', {
      name: `Lead Research: ${leadData.company}`,
      query: `${leadData.company} ${leadData.industry} market position competitive analysis`,
      methodology: 'business',
      parameters: {
        maxSources: 20,
        qualityThreshold: 0.8,
        urgency: 'high'
      }
    })

    // Execute research asynchronously
    await invoke('execute_research_async', { workflowId })

    // Set up completion notification
    this.scheduleNotification(workflowId, 'lead_research_complete')

    return workflowId
  }
}
```

#### **Scheduled Research Pipelines**
```javascript
// Complex scheduling system
class ResearchScheduler {
  constructor() {
    this.schedules = new Map()
    this.dependencies = new Map()
  }

  async createResearchPipeline(pipelineConfig) {
    const pipeline = {
      id: generateId(),
      name: pipelineConfig.name,
      stages: pipelineConfig.stages,
      schedule: pipelineConfig.schedule,
      dependencies: pipelineConfig.dependencies
    }

    // Example: Weekly market intelligence pipeline
    const marketIntelligencePipeline = {
      stages: [
        {
          name: 'competitor_monitoring',
          template: 'competitor_analysis',
          schedule: 'monday_6am',
          duration: '2_hours'
        },
        {
          name: 'industry_trends',
          template: 'trend_analysis',
          schedule: 'tuesday_6am',
          depends_on: 'competitor_monitoring'
        },
        {
          name: 'market_synthesis',
          template: 'market_summary',
          schedule: 'wednesday_6am',
          depends_on: ['competitor_monitoring', 'industry_trends']
        },
        {
          name: 'executive_briefing',
          template: 'executive_summary',
          schedule: 'friday_8am',
          depends_on: 'market_synthesis',
          distribution: ['executives', 'strategy_team']
        }
      ]
    }

    return this.schedulePipeline(marketIntelligencePipeline)
  }
}
```

### Multi-Tenant and Enterprise Integration

#### **Multi-Organization Support**
```typescript
// Enterprise multi-tenant architecture
interface OrganizationConfig {
  orgId: string
  apiQuotas: {
    monthly_budget: number
    max_concurrent_research: number
    priority_level: 'standard' | 'premium' | 'enterprise'
  }
  integrations: {
    crm: 'salesforce' | 'hubspot' | 'pipedrive'
    communication: 'slack' | 'teams' | 'discord'
    storage: 'aws_s3' | 'google_drive' | 'sharepoint'
  }
  compliance: {
    data_retention_days: number
    encryption_required: boolean
    audit_logging: boolean
    gdpr_compliance: boolean
  }
}

class EnterpriseResearchManager {
  async createOrganizationResearch(orgId: string, researchConfig: any) {
    // Validate organization permissions
    const org = await this.getOrganization(orgId)
    await this.validateQuotas(org, researchConfig)

    // Create research with organization context
    const workflowId = await invoke('create_research_workflow', {
      ...researchConfig,
      organizationId: orgId,
      complianceSettings: org.compliance,
      budgetLimits: org.apiQuotas
    })

    // Apply organization-specific processing
    await this.applyOrgPolicies(workflowId, org)

    return workflowId
  }

  async validateQuotas(org: OrganizationConfig, config: any) {
    const currentUsage = await this.getCurrentUsage(org.orgId)

    if (currentUsage.monthly_spend + config.estimatedCost > org.apiQuotas.monthly_budget) {
      throw new Error('Monthly budget exceeded')
    }

    if (currentUsage.active_research >= org.apiQuotas.max_concurrent_research) {
      throw new Error('Concurrent research limit reached')
    }
  }
}
```

#### **SSO and Advanced Authentication**
```typescript
// Enterprise SSO integration
class EnterpriseAuth {
  async authenticateWithSSO(provider: 'okta' | 'azure_ad' | 'google_workspace') {
    switch (provider) {
      case 'okta':
        return this.authenticateOkta()
      case 'azure_ad':
        return this.authenticateAzureAD()
      case 'google_workspace':
        return this.authenticateGoogleWorkspace()
    }
  }

  async authenticateOkta() {
    const oktaConfig = {
      domain: process.env.OKTA_DOMAIN,
      clientId: process.env.OKTA_CLIENT_ID,
      redirectUri: process.env.OKTA_REDIRECT_URI
    }

    // Implement OIDC flow
    const authUrl = `https://${oktaConfig.domain}/oauth2/v1/authorize?` +
      `client_id=${oktaConfig.clientId}&` +
      `response_type=code&` +
      `scope=openid profile email&` +
      `redirect_uri=${oktaConfig.redirectUri}`

    // Handle authentication flow
    return this.handleOIDCFlow(authUrl)
  }

  async validateJWT(token: string) {
    // Validate JWT token with provider
    const decoded = jwt.verify(token, process.env.JWT_SECRET)

    // Check user permissions and organization access
    const userPermissions = await this.getUserPermissions(decoded.sub)

    return {
      userId: decoded.sub,
      organizationId: decoded.org,
      permissions: userPermissions,
      roles: decoded.roles
    }
  }
}
```

## 📊 Analytics and Monitoring Integration

### Research Analytics API

#### **Comprehensive Analytics Collection**
```typescript
// Analytics data collection
interface ResearchAnalytics {
  researchMetrics: {
    totalResearchSessions: number
    averageCompletionTime: number
    averageQualityScore: number
    averageCostPerResearch: number
    successRate: number
  }
  userBehavior: {
    mostUsedTemplates: string[]
    preferredMethodologies: string[]
    peakUsageHours: number[]
    averageSessionDuration: number
  }
  systemPerformance: {
    apiResponseTimes: Record<string, number>
    errorRates: Record<string, number>
    resourceUtilization: {
      cpu: number
      memory: number
      storage: number
    }
  }
  businessMetrics: {
    costEfficiency: number
    userSatisfaction: number
    featureAdoption: Record<string, number>
    retentionRate: number
  }
}

class AnalyticsCollector {
  async collectResearchMetrics(workflowId: string) {
    const research = await invoke('get_research_details', { workflowId })

    // Collect comprehensive metrics
    const metrics = {
      duration: research.completionTime - research.startTime,
      sourceCount: research.sources.length,
      qualityScore: research.qualityMetrics.overallScore,
      costBreakdown: research.costAnalysis,
      userInteractions: research.userActions,
      systemPerformance: research.performanceMetrics
    }

    // Send to analytics platform
    await this.sendToAnalytics('research_completed', metrics)

    // Update user behavior patterns
    await this.updateUserProfile(research.userId, metrics)

    return metrics
  }

  async generateAnalyticsReport(timeRange: string, filters: any) {
    const data = await this.queryAnalyticsData(timeRange, filters)

    return {
      summary: this.calculateSummaryMetrics(data),
      trends: this.identifyTrends(data),
      insights: this.generateInsights(data),
      recommendations: this.generateRecommendations(data)
    }
  }
}
```

#### **Real-Time Dashboard Integration**
```javascript
// Real-time dashboard data streaming
class DashboardStreamer {
  constructor(dashboardConfig) {
    this.websocket = new WebSocket(dashboardConfig.wsEndpoint)
    this.metrics = new Map()
    this.subscribers = new Set()
  }

  async streamResearchMetrics() {
    // Stream real-time research progress
    this.websocket.on('research_progress', (data) => {
      this.broadcastToSubscribers('progress_update', {
        workflowId: data.workflowId,
        progress: data.progress,
        currentPhase: data.phase,
        estimatedCompletion: data.eta,
        qualityScore: data.currentQuality
      })
    })

    // Stream system health metrics
    setInterval(async () => {
      const systemHealth = await invoke('get_system_health')
      this.broadcastToSubscribers('system_health', systemHealth)
    }, 5000)

    // Stream cost and usage metrics
    setInterval(async () => {
      const usage = await invoke('get_usage_metrics')
      this.broadcastToSubscribers('usage_update', usage)
    }, 30000)
  }

  subscribe(callback) {
    this.subscribers.add(callback)
    return () => this.subscribers.delete(callback)
  }

  broadcastToSubscribers(event, data) {
    this.subscribers.forEach(callback => {
      try {
        callback(event, data)
      } catch (error) {
        console.error('Subscriber callback error:', error)
      }
    })
  }
}
```

### Business Intelligence Integration

#### **Data Warehouse Integration**
```sql
-- Data warehouse schema for research analytics
CREATE TABLE research_sessions (
  id UUID PRIMARY KEY,
  organization_id UUID NOT NULL,
  user_id UUID NOT NULL,
  template_id UUID,
  methodology VARCHAR(50),
  start_time TIMESTAMP,
  completion_time TIMESTAMP,
  status VARCHAR(20),
  quality_score DECIMAL(3,2),
  source_count INTEGER,
  total_cost DECIMAL(10,2),
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE research_sources (
  id UUID PRIMARY KEY,
  research_session_id UUID REFERENCES research_sessions(id),
  source_url TEXT,
  source_type VARCHAR(50),
  quality_score DECIMAL(3,2),
  relevance_score DECIMAL(3,2),
  processing_time INTEGER,
  cost DECIMAL(8,4),
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE user_behavior (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL,
  session_id UUID,
  action_type VARCHAR(100),
  action_data JSONB,
  timestamp TIMESTAMP DEFAULT NOW()
);

-- Analytics views
CREATE VIEW research_performance AS
SELECT
  DATE_TRUNC('day', completion_time) as date,
  COUNT(*) as total_research,
  AVG(quality_score) as avg_quality,
  AVG(EXTRACT(EPOCH FROM (completion_time - start_time))/60) as avg_duration_minutes,
  AVG(total_cost) as avg_cost,
  SUM(total_cost) as total_cost
FROM research_sessions
WHERE status = 'completed'
GROUP BY DATE_TRUNC('day', completion_time)
ORDER BY date;
```

#### **Business Intelligence Dashboards**
```javascript
// Tableau/Power BI integration
class BIDashboardIntegration {
  async syncToTableau(data) {
    const tableauConfig = {
      server: process.env.TABLEAU_SERVER,
      site: process.env.TABLEAU_SITE,
      datasource: 'research_analytics'
    }

    // Prepare data for Tableau
    const tableauData = this.formatForTableau(data)

    // Upload to Tableau Server
    await this.uploadToTableau(tableauConfig, tableauData)
  }

  async createPowerBIReport(analyticsData) {
    const powerBIConfig = {
      workspaceId: process.env.POWERBI_WORKSPACE_ID,
      datasetId: process.env.POWERBI_DATASET_ID
    }

    // Transform data for Power BI
    const powerBIData = this.formatForPowerBI(analyticsData)

    // Push to Power BI dataset
    await this.pushToPowerBI(powerBIConfig, powerBIData)
  }

  formatForTableau(data) {
    return data.map(record => ({
      research_id: record.id,
      completion_date: record.completionTime,
      quality_score: record.qualityScore,
      duration_minutes: record.duration / 60000,
      cost_usd: record.totalCost,
      methodology: record.methodology,
      source_count: record.sourceCount,
      user_satisfaction: record.userRating
    }))
  }
}
```

## 🔐 Enterprise Security and Compliance

### Advanced Security Features

#### **End-to-End Encryption**
```typescript
// Enterprise encryption implementation
class EnterpriseEncryption {
  private encryptionKey: string
  private keyRotationSchedule: number = 90 // days

  async encryptResearchData(data: any, organizationId: string) {
    // Get organization-specific encryption key
    const orgKey = await this.getOrganizationKey(organizationId)

    // Encrypt sensitive data
    const encryptedData = {
      id: data.id,
      organizationId: organizationId,
      encryptedContent: await this.encrypt(JSON.stringify(data.content), orgKey),
      encryptedSources: await this.encryptArray(data.sources, orgKey),
      metadata: data.metadata, // Non-sensitive metadata
      encryptionVersion: this.getCurrentEncryptionVersion(),
      encryptedAt: new Date().toISOString()
    }

    return encryptedData
  }

  async decryptResearchData(encryptedData: any, organizationId: string) {
    // Validate access permissions
    await this.validateDecryptionPermissions(organizationId)

    // Get appropriate decryption key
    const orgKey = await this.getOrganizationKey(organizationId)

    // Decrypt data
    const decryptedContent = await this.decrypt(encryptedData.encryptedContent, orgKey)
    const decryptedSources = await this.decryptArray(encryptedData.encryptedSources, orgKey)

    return {
      id: encryptedData.id,
      content: JSON.parse(decryptedContent),
      sources: decryptedSources,
      metadata: encryptedData.metadata
    }
  }

  async rotateEncryptionKeys(organizationId: string) {
    // Generate new encryption key
    const newKey = await this.generateEncryptionKey()

    // Re-encrypt all data with new key
    await this.reencryptOrganizationData(organizationId, newKey)

    // Update key in secure storage
    await this.updateOrganizationKey(organizationId, newKey)

    // Log key rotation event
    await this.logSecurityEvent('key_rotation', organizationId)
  }
}
```

#### **Audit Logging and Compliance**
```typescript
// Comprehensive audit logging
interface AuditEvent {
  eventId: string
  timestamp: string
  userId: string
  organizationId: string
  eventType: string
  resourceId?: string
  action: string
  result: 'success' | 'failure' | 'partial'
  ipAddress: string
  userAgent: string
  details: Record<string, any>
  complianceFlags: string[]
}

class ComplianceAuditor {
  async logResearchEvent(event: Partial<AuditEvent>) {
    const auditEvent: AuditEvent = {
      eventId: generateUUID(),
      timestamp: new Date().toISOString(),
      userId: event.userId!,
      organizationId: event.organizationId!,
      eventType: 'research_activity',
      action: event.action!,
      result: event.result || 'success',
      ipAddress: event.ipAddress!,
      userAgent: event.userAgent!,
      details: event.details || {},
      complianceFlags: this.determineComplianceFlags(event)
    }

    // Store in immutable audit log
    await this.storeAuditEvent(auditEvent)

    // Check for compliance violations
    await this.checkComplianceViolations(auditEvent)

    // Send to SIEM if configured
    if (this.siemEnabled) {
      await this.sendToSIEM(auditEvent)
    }
  }

  async generateComplianceReport(organizationId: string, timeRange: string) {
    const auditEvents = await this.getAuditEvents(organizationId, timeRange)

    return {
      summary: {
        totalEvents: auditEvents.length,
        successfulEvents: auditEvents.filter(e => e.result === 'success').length,
        failedEvents: auditEvents.filter(e => e.result === 'failure').length,
        complianceViolations: auditEvents.filter(e => e.complianceFlags.length > 0).length
      },
      violations: this.analyzeComplianceViolations(auditEvents),
      recommendations: this.generateComplianceRecommendations(auditEvents),
      certificationStatus: await this.checkCertificationCompliance(organizationId)
    }
  }
}
```

### Data Governance and Privacy

#### **GDPR Compliance Implementation**
```typescript
// GDPR compliance features
class GDPRCompliance {
  async handleDataSubjectRequest(requestType: 'access' | 'rectification' | 'erasure' | 'portability', userId: string) {
    switch (requestType) {
      case 'access':
        return this.generateDataAccessReport(userId)
      case 'rectification':
        return this.enableDataRectification(userId)
      case 'erasure':
        return this.executeRightToBeForgotten(userId)
      case 'portability':
        return this.generateDataPortabilityExport(userId)
    }
  }

  async executeRightToBeForgotten(userId: string) {
    // Identify all user data across the system
    const userDataLocations = await this.identifyUserData(userId)

    // Create anonymization plan
    const anonymizationPlan = await this.createAnonymizationPlan(userDataLocations)

    // Execute data deletion/anonymization
    const results = await Promise.all([
      this.anonymizeResearchData(userId),
      this.deletePersonalInformation(userId),
      this.updateAuditLogs(userId, 'anonymized'),
      this.notifyIntegratedSystems(userId, 'deletion_request')
    ])

    // Generate compliance certificate
    const certificate = await this.generateDeletionCertificate(userId, results)

    return {
      status: 'completed',
      deletionId: generateUUID(),
      certificate: certificate,
      completedAt: new Date().toISOString()
    }
  }

  async generateDataAccessReport(userId: string) {
    const userData = await this.collectAllUserData(userId)

    return {
      personalInformation: userData.profile,
      researchHistory: userData.research.map(r => this.sanitizeResearchData(r)),
      systemInteractions: userData.interactions,
      dataProcessingActivities: userData.processing,
      thirdPartySharing: userData.sharing,
      retentionSchedule: userData.retention,
      generatedAt: new Date().toISOString()
    }
  }
}
```

---

**Next Steps**: Explore advanced integration patterns, set up webhooks for real-time notifications, or dive into [Template Management](./templates.md) for custom research workflows.

**Advanced Topics**:
- **Enterprise Architecture**: Design scalable, multi-tenant research systems
- **Security Implementation**: Implement enterprise-grade security and compliance
- **Analytics Integration**: Build comprehensive research analytics and BI dashboards
- **Workflow Automation**: Create sophisticated automated research pipelines

**Need Help?** Check our [FAQ](./faq.md), review the [Desktop App Guide](./desktop-app.md), or visit the [Community Forum](https://community.freedeepresearch.org) for integration support.
