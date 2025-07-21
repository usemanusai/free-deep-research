# 🚀 Phase 4.4: API Gateway & GraphQL

## Overview

Phase 4.4 implements a unified GraphQL API Gateway that consolidates all REST endpoints into a single, efficient, and type-safe GraphQL interface. This phase provides:

- **Unified GraphQL API**: Single endpoint for all functionality
- **Real-time Subscriptions**: WebSocket-based live updates
- **Schema Federation**: Distributed schema composition
- **Performance Optimization**: Caching, DataLoader, and query optimization
- **Production Security**: Rate limiting, query validation, and authorization

## 📋 Prerequisites

Before deploying Phase 4.4, ensure:

1. **Phase 4.1** (Event Sourcing) is completed ✅
2. **Phase 4.2** (CQRS) is completed ✅
3. **Phase 4.3** (Infrastructure Modernization) is completed ✅
4. **Kubernetes cluster** with Istio service mesh
5. **kubectl** and **istioctl** are installed and configured

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    GraphQL API Gateway                      │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   GraphQL   │  │ Schema      │  │ Federation  │        │
│  │   Server    │  │ Registry    │  │ Gateway     │        │
│  │     3x      │  │     2x      │  │     2x      │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│         │                │                │                │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Backend   │  │    Redis    │  │ PostgreSQL  │        │
│  │   (CQRS)    │  │  (Cache)    │  │ (Database)  │        │
│  │     3x      │  │     3x      │  │     1x      │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### 1. Deploy Phase 4.4

```bash
# Navigate to Kubernetes directory
cd infrastructure/kubernetes

# Run the deployment script
./deploy-phase-4.4.sh deploy
```

### 2. Verify Deployment

```bash
# Check deployment status
./deploy-phase-4.4.sh verify

# Test GraphQL endpoints
./deploy-phase-4.4.sh test

# Check GraphQL Gateway pods
kubectl get pods -n free-deep-research -l app.kubernetes.io/name=graphql-gateway
```

### 3. Access GraphQL Endpoints

- **Production GraphQL**: https://graphql.freedeepresearch.org/graphql
- **API v4 Endpoint**: https://api-v4.freedeepresearch.org/graphql
- **GraphQL Playground**: https://graphql.freedeepresearch.org/playground
- **Development**: https://graphql-dev.freedeepresearch.org/graphql

## 📁 File Structure

```
infrastructure/kubernetes/
├── deploy-phase-4.4.sh           # Main deployment script
├── README-PHASE-4.4.md           # This documentation
├── deployments/
│   └── graphql-gateway.yaml      # GraphQL Gateway deployment
├── istio/
│   ├── graphql-gateway.yaml      # Istio Gateway for GraphQL
│   └── graphql-virtual-service.yaml # GraphQL routing configuration
└── packages/ai-orchestrator/graphql/
    ├── src/
    │   ├── lib.rs                 # Main GraphQL server
    │   ├── resolvers/             # Query, Mutation, Subscription resolvers
    │   ├── types/                 # GraphQL type definitions
    │   └── dataloaders/           # DataLoader implementations
    └── schema/
        └── schema.graphql         # Complete GraphQL schema
```

## 🔧 Configuration

### GraphQL Server Configuration

The GraphQL server is configured via ConfigMap:

```yaml
graphql:
  enable_playground: true
  enable_introspection: true
  max_query_depth: 15
  max_query_complexity: 1000
  enable_subscriptions: true
  enable_caching: true
  cache_ttl: 300
```

### Rate Limiting Configuration

```yaml
rate_limiting:
  requests_per_minute: 1000
  burst_size: 2000
  enable_per_user_limits: true
  enable_query_complexity_limits: true
  complexity_limit: 1000
  depth_limit: 15
```

### Federation Configuration

```yaml
federation:
  enable_federation: true
  gateway_url: "https://graphql.freedeepresearch.org"
  service_name: "free-deep-research-gateway"
  enable_schema_composition: true
  enable_query_planning: true
```

## 📊 GraphQL Schema Overview

### Core Types

```graphql
type Query {
  # Authentication & Users
  me: User
  users(filter: UserFilter, pagination: PaginationInput): UserConnection!
  
  # API Key Management
  apiKeys(filter: ApiKeyFilter): ApiKeyConnection!
  apiKeyUsageStats(keyId: UUID!, timeRange: TimeRange): UsageStats!
  
  # Research Workflows
  researchWorkflows(filter: WorkflowFilter): WorkflowConnection!
  researchWorkflow(id: UUID!): ResearchWorkflow
  
  # System Monitoring
  systemMetrics(timeRange: TimeRange): SystemMetrics!
  performanceMetrics(service: String): PerformanceMetrics!
  
  # V3.0.0 Features
  federatedResearch(filter: FederatedFilter): [FederatedResearchNode!]!
  aiMarketplace(category: MarketplaceCategory): [MarketplaceItem!]!
  knowledgeGraph(graphId: UUID!): KnowledgeGraph
  
  # BMAD Integration
  bmadAgents: [BMadAgent!]!
  bmadWorkflows(agentId: String): [BMadWorkflow!]!
}

type Mutation {
  # Authentication
  login(input: LoginInput!): AuthPayload!
  logout: Boolean!
  
  # API Key Management
  createApiKey(input: CreateApiKeyInput!): ApiKey!
  updateApiKey(id: UUID!, input: UpdateApiKeyInput!): ApiKey!
  deleteApiKey(id: UUID!): Boolean!
  
  # Research Workflows
  createResearchWorkflow(input: CreateWorkflowInput!): ResearchWorkflow!
  executeResearchWorkflow(id: UUID!): WorkflowExecution!
  
  # V3.0.0 Features
  createFederatedResearch(input: FederatedResearchInput!): FederatedResearchNode!
  createKnowledgeGraph(input: CreateKnowledgeGraphInput!): KnowledgeGraph!
}

type Subscription {
  # Real-time Updates
  workflowExecutionUpdates(workflowId: UUID!): WorkflowExecutionUpdate!
  systemMetricsUpdates: SystemMetricsUpdate!
  apiKeyUsageUpdates(keyId: UUID!): ApiKeyUsageUpdate!
  federatedResearchUpdates(networkId: UUID!): FederatedResearchUpdate!
  knowledgeGraphUpdates(graphId: UUID!): KnowledgeGraphUpdate!
}
```

## 🔄 Migration from REST to GraphQL

### REST to GraphQL Mapping

| REST Endpoint | GraphQL Operation |
|---------------|-------------------|
| `GET /api/auth/me` | `query { me { ... } }` |
| `POST /api/auth/login` | `mutation { login(input: {...}) { ... } }` |
| `GET /api/workflows` | `query { researchWorkflows { ... } }` |
| `POST /api/workflows` | `mutation { createResearchWorkflow(input: {...}) { ... } }` |
| `GET /api/metrics` | `query { systemMetrics { ... } }` |
| `WebSocket /ws/workflows` | `subscription { workflowExecutionUpdates { ... } }` |

### Client Migration Example

**Before (REST):**
```javascript
// Multiple requests needed
const user = await fetch('/api/auth/me').then(r => r.json());
const workflows = await fetch('/api/workflows').then(r => r.json());
const metrics = await fetch('/api/metrics').then(r => r.json());
```

**After (GraphQL):**
```javascript
// Single request with exact data needed
const { data } = await graphqlClient.query({
  query: gql`
    query DashboardData {
      me {
        id
        username
        email
      }
      researchWorkflows(first: 10) {
        edges {
          node {
            id
            name
            status
            progress
          }
        }
      }
      systemMetrics {
        cpuUsage
        memoryUsage
        activeWorkflows
      }
    }
  `
});
```

## 🔒 Security Features

### Query Security

- **Depth Limiting**: Prevents deeply nested queries (max 15 levels)
- **Complexity Analysis**: Prevents expensive operations (max 1000 points)
- **Rate Limiting**: Per-user and per-query limits
- **Query Validation**: Syntax and semantic validation
- **Persisted Queries**: Only allow pre-approved queries in production

### Authentication & Authorization

- **JWT Authentication**: Secure token-based authentication
- **Field-level Authorization**: Granular access control per field
- **Operation-level Authorization**: Control access to queries/mutations
- **Role-based Access Control**: User roles and permissions

### Production Security

- **Error Masking**: Hide internal errors from clients
- **Introspection Control**: Disable schema introspection in production
- **CORS Configuration**: Proper cross-origin resource sharing
- **Request Validation**: Input sanitization and validation

## 📈 Performance Optimization

### Caching Strategies

- **Query Result Caching**: Cache frequently accessed data
- **DataLoader Batching**: Prevent N+1 query problems
- **Automatic Persisted Queries**: Reduce query size and improve security
- **Redis Integration**: Distributed caching across instances

### Query Optimization

- **Query Complexity Analysis**: Prevent expensive operations
- **Query Planning**: Optimize federated query execution
- **Connection Pooling**: Efficient database connections
- **Lazy Loading**: Load data only when needed

## 🚨 Troubleshooting

### Common Issues

1. **GraphQL Gateway not starting**
   ```bash
   kubectl describe pod -l app.kubernetes.io/name=graphql-gateway -n free-deep-research
   kubectl logs -l app.kubernetes.io/name=graphql-gateway -n free-deep-research
   ```

2. **GraphQL queries failing**
   ```bash
   # Check GraphQL endpoint
   curl -X POST https://graphql.freedeepresearch.org/graphql \
     -H "Content-Type: application/json" \
     -d '{"query":"query { __schema { queryType { name } } }"}'
   ```

3. **Subscriptions not working**
   ```bash
   # Check WebSocket connectivity
   kubectl logs -l app.kubernetes.io/name=graphql-gateway -n free-deep-research | grep -i websocket
   ```

4. **Federation issues**
   ```bash
   # Check schema registry
   kubectl get pods -l app.kubernetes.io/name=schema-registry -n free-deep-research
   kubectl logs -l app.kubernetes.io/name=schema-registry -n free-deep-research
   ```

### Debug Commands

```bash
# Check GraphQL Gateway status
kubectl get pods -l app.kubernetes.io/name=graphql-gateway -n free-deep-research

# Check GraphQL service
kubectl get service graphql-gateway-service -n free-deep-research

# Check GraphQL ingress
kubectl get ingress -n free-deep-research | grep graphql

# Check Istio GraphQL configuration
kubectl get gateway,virtualservice -n free-deep-research | grep graphql

# Test GraphQL introspection
kubectl exec -n free-deep-research deployment/graphql-gateway -- \
  curl -X POST http://localhost:4000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"query { __schema { queryType { name } } }"}'
```

## 🎯 Success Criteria

Phase 4.4 is considered successful when:

- ✅ GraphQL Gateway deployed and running
- ✅ All REST endpoints accessible via GraphQL
- ✅ Real-time subscriptions working
- ✅ Query performance optimized
- ✅ Security measures implemented
- ✅ Federation capabilities available

## 📚 Next Steps

After Phase 4.4 completion:

1. **Phase 4.5**: Serverless & Edge Computing
2. **Phase 4.6**: AI/ML Pipeline enhancement
3. **Client Migration**: Gradually migrate frontend to GraphQL
4. **Performance Monitoring**: Monitor GraphQL query performance
5. **Schema Evolution**: Evolve schema based on usage patterns

---

**Phase 4.4 API Gateway & GraphQL** provides a modern, unified API layer that improves developer experience, reduces over-fetching, and enables real-time features! 🚀
