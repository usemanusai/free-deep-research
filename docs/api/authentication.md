# 🔐 Authentication API

## Overview

The Free Deep Research System uses API key-based authentication for all operations. This document covers API key management, authentication flows, and security best practices.

## 🔑 API Key Management

### Get All API Keys

Retrieve all configured API keys for the current user.

**Tauri Command:**
```typescript
const apiKeys = await invoke<ApiKey[]>('get_api_keys')
```

**REST Endpoint:**
```http
GET /api/auth/keys
Authorization: Bearer YOUR_MASTER_KEY
```

**Response:**
```json
{
  "keys": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "service": "openrouter",
      "name": "OpenRouter Production",
      "status": "active",
      "created_at": "2025-01-15T10:30:00Z",
      "last_used": "2025-01-20T14:22:00Z",
      "usage_stats": {
        "total_requests": 1250,
        "successful_requests": 1200,
        "failed_requests": 50,
        "rate_limit_hits": 5
      }
    }
  ]
}
```

### Create API Key

Add a new API key for a service.

**Tauri Command:**
```typescript
const newKey = await invoke<ApiKey>('create_api_key', {
  service: 'openrouter',
  name: 'My OpenRouter Key',
  key: 'sk-or-v1-...',
  metadata: {
    environment: 'production',
    team: 'research'
  }
})
```

**REST Endpoint:**
```http
POST /api/auth/keys
Authorization: Bearer YOUR_MASTER_KEY
Content-Type: application/json

{
  "service": "openrouter",
  "name": "My OpenRouter Key",
  "key": "sk-or-v1-...",
  "metadata": {
    "environment": "production",
    "team": "research"
  }
}
```

### Update API Key

Update an existing API key's configuration.

**Tauri Command:**
```typescript
const updatedKey = await invoke<ApiKey>('update_api_key', {
  keyId: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Updated Key Name',
  metadata: {
    environment: 'staging'
  }
})
```

### Delete API Key

Remove an API key from the system.

**Tauri Command:**
```typescript
await invoke('delete_api_key', {
  keyId: '550e8400-e29b-41d4-a716-446655440000'
})
```

### Test API Key

Validate an API key by making a test request.

**Tauri Command:**
```typescript
const testResult = await invoke<ApiKeyTestResult>('test_api_key', {
  keyId: '550e8400-e29b-41d4-a716-446655440000'
})
```

**Response:**
```json
{
  "success": true,
  "response_time_ms": 245,
  "service_status": "operational",
  "rate_limit_remaining": 4750,
  "rate_limit_reset": "2025-01-20T15:00:00Z"
}
```

## 🔒 Security Features

### Key Rotation

Automatically rotate API keys based on configurable policies.

**Tauri Command:**
```typescript
const rotationResult = await invoke<KeyRotationResult>('rotate_api_key', {
  keyId: '550e8400-e29b-41d4-a716-446655440000',
  newKey: 'sk-or-v1-new-key...'
})
```

### Encryption

All API keys are encrypted at rest using AES-256-GCM encryption.

**Get Encryption Status:**
```typescript
const encryptionStatus = await invoke<EncryptionStatus>('get_encryption_status')
```

### Audit Logging

All API key operations are logged for security auditing.

**Get Audit Logs:**
```typescript
const auditLogs = await invoke<AuditLog[]>('get_api_key_audit_logs', {
  keyId: '550e8400-e29b-41d4-a716-446655440000',
  startDate: '2025-01-01T00:00:00Z',
  endDate: '2025-01-20T23:59:59Z'
})
```

## 📊 Usage Analytics

### Get Usage Statistics

Retrieve detailed usage statistics for API keys.

**Tauri Command:**
```typescript
const usageStats = await invoke<UsageStatistics>('get_api_usage_stats', {
  keyId: '550e8400-e29b-41d4-a716-446655440000',
  timeframe: 'last_30_days'
})
```

**Response:**
```json
{
  "total_requests": 5420,
  "successful_requests": 5180,
  "failed_requests": 240,
  "average_response_time_ms": 320,
  "cost_breakdown": {
    "total_cost_usd": 12.45,
    "cost_per_request": 0.0023
  },
  "daily_usage": [
    {
      "date": "2025-01-20",
      "requests": 180,
      "cost_usd": 0.41
    }
  ]
}
```

### Rate Limit Monitoring

Monitor rate limits across all services.

**Tauri Command:**
```typescript
const rateLimits = await invoke<RateLimitStatus[]>('get_rate_limit_status')
```

## 🚨 Rate Limit Management

### Get Rate Limit Alerts

Retrieve active rate limit alerts.

**Tauri Command:**
```typescript
const alerts = await invoke<RateLimitAlert[]>('get_rate_limit_alerts')
```

### Configure Rate Limit Thresholds

Set custom thresholds for rate limit warnings.

**Tauri Command:**
```typescript
await invoke('configure_rate_limit_thresholds', {
  keyId: '550e8400-e29b-41d4-a716-446655440000',
  warningThreshold: 80, // Warn at 80% of limit
  criticalThreshold: 95 // Critical at 95% of limit
})
```

## 🔧 Service Integration

### Supported Services

| Service | Authentication Type | Rate Limits | Cost Tracking |
|---------|-------------------|-------------|---------------|
| **OpenRouter** | API Key | ✅ | ✅ |
| **SerpApi** | API Key | ✅ | ✅ |
| **Jina AI** | API Key | ✅ | ✅ |
| **Firecrawl** | API Key | ✅ | ✅ |
| **Tavily** | API Key | ✅ | ✅ |
| **Exa AI** | API Key | ✅ | ✅ |

### Get Service Endpoints

Retrieve available endpoints for a service.

**Tauri Command:**
```typescript
const endpoints = await invoke<string[]>('get_service_endpoints', {
  service: 'openrouter'
})
```

### Service Health Check

Check the health status of integrated services.

**Tauri Command:**
```typescript
const healthStatus = await invoke<ServiceHealthStatus>('check_service_health', {
  service: 'openrouter'
})
```

## 🛡️ Best Practices

### Security Recommendations

1. **Use Environment Variables**: Store API keys in environment variables, not in code
2. **Regular Rotation**: Rotate API keys every 90 days
3. **Principle of Least Privilege**: Use service-specific keys with minimal required permissions
4. **Monitor Usage**: Set up alerts for unusual usage patterns
5. **Secure Storage**: Enable encryption for API key storage

### Error Handling

```typescript
try {
  const apiKeys = await invoke<ApiKey[]>('get_api_keys')
} catch (error) {
  if (error.includes('UNAUTHORIZED')) {
    // Handle authentication error
    console.error('Authentication failed:', error)
  } else if (error.includes('RATE_LIMITED')) {
    // Handle rate limit
    console.error('Rate limit exceeded:', error)
  } else {
    // Handle other errors
    console.error('API error:', error)
  }
}
```

## 📚 Related Documentation

- [Research Workflow API](./research-workflow.md)
- [Configuration API](./configuration.md)
- [Monitoring API](./monitoring.md)

---

**Next**: Learn about [Research Workflow API](./research-workflow.md) to start executing research.
