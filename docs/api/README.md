# 🔌 Free Deep Research System - API Documentation

## Overview

The Free Deep Research System provides comprehensive APIs for all major functionality across multiple versions. All APIs are implemented as Tauri commands for the desktop application and REST endpoints for distributed deployments.

## 📚 API Documentation Structure

### Core APIs
- **[Authentication](./authentication.md)** - API key management and authentication
- **[Research Workflow](./research-workflow.md)** - Research execution and management
- **[Configuration](./configuration.md)** - System configuration and settings
- **[Monitoring](./monitoring.md)** - System health and performance monitoring

### Advanced Features
- **[Analytics](./analytics.md)** - Business intelligence and analytics
- **[Template Management](./template-management.md)** - Research template management
- **[Output Processing](./output-processing.md)** - Result formatting and export

### V3.0.0 Global Intelligence Network
- **[Federated Research](./federated-research.md)** - Cross-organization collaboration
- **[AI Marketplace](./ai-marketplace.md)** - Community AI agents and models
- **[Quantum-Ready Architecture](./quantum-ready.md)** - Post-quantum cryptography
- **[NLP Engine](./nlp-engine.md)** - Natural language processing
- **[Blockchain Integration](./blockchain.md)** - Decentralized validation
- **[Knowledge Graph](./knowledge-graph.md)** - Interconnected knowledge management

### BMAD Integration
- **[BMAD Agent Integration](./bmad-integration.md)** - AI agent orchestration
- **[Research Enhancement](./research-enhancement.md)** - Evidence-based documentation

### Phase 4 Advanced Features
- **[Machine Learning](./machine-learning.md)** - ML models and predictions
- **[Mobile APIs](./mobile-apis.md)** - Mobile platform support
- **[Advanced Analytics](./advanced-analytics.md)** - Predictive analytics

## 🚀 Quick Start

### Desktop Application (Tauri Commands)

```typescript
import { invoke } from '@tauri-apps/api/core'

// Health check
const health = await invoke('health_check')

// Create research workflow
const workflowId = await invoke('create_research_workflow', {
  name: 'My Research',
  query: 'AI in healthcare',
  methodology: 'hybrid'
})

// Execute research
await invoke('execute_research', { workflowId })

// Get results
const results = await invoke('get_research_results', { workflowId })
```

### REST API (Distributed Deployment)

```bash
# Health check
curl -X GET http://localhost:8080/api/health

# Create research workflow
curl -X POST http://localhost:8080/api/research/workflows \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Research",
    "query": "AI in healthcare",
    "methodology": "hybrid"
  }'
```

## 🔐 Authentication

All API calls require authentication using API keys:

```typescript
// Tauri commands automatically handle authentication
const apiKeys = await invoke('get_api_keys')

// REST API requires Authorization header
Authorization: Bearer YOUR_API_KEY
```

## 📊 API Versions

| Version | Status | Features |
|---------|--------|----------|
| **1.0.0** | ✅ Stable | Core research functionality |
| **1.1.0** | ✅ Stable | Enhanced AI models, advanced analytics |
| **1.2.0** | ✅ Stable | Template management, batch processing |
| **2.0.0** | ✅ Stable | Distributed computing, real-time collaboration |
| **3.0.0** | ✅ Current | Global Intelligence Network, BMAD integration |
| **3.1.0** | 🚧 Development | Phase 4 Advanced Features |

## 🛠️ Development Tools

### API Testing
- **Postman Collection**: [Download](../tools/postman-collection.json)
- **OpenAPI Spec**: [View](./openapi.yaml)
- **Interactive Docs**: Available at `/api/docs` when running

### SDKs and Libraries
- **TypeScript/JavaScript**: Built-in with Tauri
- **Python**: [Python SDK](../sdks/python/)
- **Rust**: [Rust Crate](../sdks/rust/)

## 📈 Rate Limits

| Tier | Requests/Minute | Requests/Hour | Requests/Day |
|------|----------------|---------------|--------------|
| **Free** | 60 | 1,000 | 10,000 |
| **Pro** | 300 | 10,000 | 100,000 |
| **Enterprise** | 1,000 | 50,000 | 1,000,000 |

## 🔍 Error Handling

All APIs return consistent error responses:

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "The request is invalid",
    "details": "Missing required field: query"
  }
}
```

Common error codes:
- `INVALID_REQUEST` - Request validation failed
- `UNAUTHORIZED` - Authentication required
- `FORBIDDEN` - Insufficient permissions
- `NOT_FOUND` - Resource not found
- `RATE_LIMITED` - Rate limit exceeded
- `INTERNAL_ERROR` - Server error

## 📞 Support

- **Documentation**: [docs.free-deep-research.com](https://docs.free-deep-research.com)
- **GitHub Issues**: [Report bugs](https://github.com/usemanusai/free-deep-research/issues)
- **Discord**: [Join community](https://discord.gg/free-deep-research)
- **Email**: support@free-deep-research.com

## 🔄 Changelog

See [CHANGELOG.md](../../CHANGELOG.md) for detailed version history and breaking changes.

---

**Next**: Start with [Authentication](./authentication.md) to set up API access.
