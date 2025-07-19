# ⚙️ Configuration API

## Overview

The Configuration API provides comprehensive system and user configuration management, including settings for research parameters, security preferences, and system behavior.

## 🔧 System Configuration

### Get System Configuration

Retrieve current system configuration settings.

**Tauri Command:**
```typescript
const config = await invoke<SystemConfiguration>('get_configuration')
```

**REST Endpoint:**
```http
GET /api/config/system
Authorization: Bearer YOUR_API_KEY
```

**Response:**
```json
{
  "system": {
    "version": "3.0.0",
    "environment": "production",
    "features": {
      "bmadIntegration": true,
      "globalIntelligenceNetwork": true,
      "quantumReady": true,
      "advancedAnalytics": true
    },
    "limits": {
      "maxConcurrentWorkflows": 100,
      "maxWorkflowDuration": "2h",
      "maxResultSize": "100MB",
      "rateLimitPerMinute": 1000
    }
  },
  "research": {
    "defaultMethodology": "hybrid",
    "defaultDepth": "standard",
    "maxSources": 50,
    "timeoutMinutes": 30,
    "retryAttempts": 3
  },
  "security": {
    "encryptionEnabled": true,
    "auditLoggingEnabled": true,
    "sessionTimeoutMinutes": 60,
    "passwordPolicy": {
      "minLength": 12,
      "requireSpecialChars": true,
      "requireNumbers": true,
      "requireUppercase": true
    }
  }
}
```

### Update System Configuration

Update system-wide configuration settings (admin only).

**Tauri Command:**
```typescript
const updatedConfig = await invoke<SystemConfiguration>('update_configuration', {
  section: 'research',
  settings: {
    defaultMethodology: 'don_lim',
    maxSources: 75,
    timeoutMinutes: 45
  }
})
```

**REST Endpoint:**
```http
PATCH /api/config/system
Authorization: Bearer YOUR_ADMIN_KEY
Content-Type: application/json

{
  "section": "research",
  "settings": {
    "defaultMethodology": "don_lim",
    "maxSources": 75,
    "timeoutMinutes": 45
  }
}
```

### Reset Configuration

Reset configuration to default values.

**Tauri Command:**
```typescript
await invoke('reset_configuration', {
  section: 'research', // Optional: reset specific section
  confirmReset: true
})
```

## 👤 User Configuration

### Get User Preferences

Retrieve user-specific configuration and preferences.

**Tauri Command:**
```typescript
const userConfig = await invoke<UserConfiguration>('get_user_configuration')
```

**Response:**
```json
{
  "user": {
    "id": "user_123",
    "email": "user@example.com",
    "preferences": {
      "theme": "dark",
      "language": "en",
      "timezone": "UTC",
      "notifications": {
        "email": true,
        "push": false,
        "desktop": true
      }
    }
  },
  "research": {
    "defaultMethodology": "hybrid",
    "preferredDepth": "comprehensive",
    "autoSaveResults": true,
    "defaultBudget": 20.00,
    "preferredLanguages": ["en", "es"],
    "excludedDomains": ["social_media"]
  },
  "ui": {
    "dashboardLayout": "grid",
    "resultsPerPage": 25,
    "autoRefresh": true,
    "showAdvancedOptions": false
  }
}
```

### Update User Preferences

Update user-specific settings and preferences.

**Tauri Command:**
```typescript
const updatedConfig = await invoke<UserConfiguration>('update_user_configuration', {
  section: 'research',
  preferences: {
    defaultMethodology: 'don_lim',
    preferredDepth: 'comprehensive',
    defaultBudget: 30.00
  }
})
```

### Export User Configuration

Export user configuration for backup or migration.

**Tauri Command:**
```typescript
const exportData = await invoke<ConfigurationExport>('export_user_configuration', {
  includeApiKeys: false, // Security: exclude sensitive data
  format: 'json'
})
```

### Import User Configuration

Import previously exported user configuration.

**Tauri Command:**
```typescript
const importResult = await invoke<ConfigurationImportResult>('import_user_configuration', {
  configData: exportData,
  overwriteExisting: false,
  validateBeforeImport: true
})
```

## 🔐 Security Configuration

### Get Security Settings

Retrieve current security configuration.

**Tauri Command:**
```typescript
const securityConfig = await invoke<SecurityConfiguration>('get_security_configuration')
```

**Response:**
```json
{
  "encryption": {
    "algorithm": "AES-256-GCM",
    "keyRotationDays": 90,
    "backupEncryption": true
  },
  "authentication": {
    "mfaEnabled": true,
    "sessionTimeout": 3600,
    "maxLoginAttempts": 5,
    "lockoutDuration": 900
  },
  "audit": {
    "enabled": true,
    "retentionDays": 365,
    "logLevel": "info",
    "includeApiCalls": true
  },
  "privacy": {
    "dataRetentionDays": 730,
    "anonymizeOldData": true,
    "allowDataExport": true,
    "allowDataDeletion": true
  }
}
```

### Update Security Settings

Update security configuration (admin only).

**Tauri Command:**
```typescript
const updatedSecurity = await invoke<SecurityConfiguration>('update_security_configuration', {
  section: 'authentication',
  settings: {
    sessionTimeout: 7200,
    maxLoginAttempts: 3
  }
})
```

## 🌐 Integration Configuration

### Get Integration Settings

Retrieve configuration for external service integrations.

**Tauri Command:**
```typescript
const integrations = await invoke<IntegrationConfiguration>('get_integration_configuration')
```

**Response:**
```json
{
  "apiServices": {
    "openrouter": {
      "enabled": true,
      "baseUrl": "https://openrouter.ai/api/v1",
      "timeout": 30000,
      "retryAttempts": 3,
      "rateLimits": {
        "requestsPerMinute": 60,
        "requestsPerHour": 1000
      }
    },
    "serpapi": {
      "enabled": true,
      "baseUrl": "https://serpapi.com",
      "timeout": 15000,
      "retryAttempts": 2
    }
  },
  "bmadIntegration": {
    "enabled": true,
    "agentTimeout": 1800,
    "maxConcurrentAgents": 5,
    "evidenceThreshold": 0.8
  },
  "globalIntelligenceNetwork": {
    "enabled": true,
    "federatedResearch": true,
    "aiMarketplace": true,
    "knowledgeGraph": true
  }
}
```

### Update Integration Settings

Update integration configuration.

**Tauri Command:**
```typescript
const updatedIntegrations = await invoke<IntegrationConfiguration>('update_integration_configuration', {
  service: 'bmadIntegration',
  settings: {
    maxConcurrentAgents: 8,
    evidenceThreshold: 0.85
  }
})
```

## 📊 Performance Configuration

### Get Performance Settings

Retrieve performance and optimization settings.

**Tauri Command:**
```typescript
const perfConfig = await invoke<PerformanceConfiguration>('get_performance_configuration')
```

**Response:**
```json
{
  "caching": {
    "enabled": true,
    "ttlSeconds": 3600,
    "maxSize": "1GB",
    "compressionEnabled": true
  },
  "concurrency": {
    "maxConcurrentRequests": 100,
    "maxConcurrentWorkflows": 50,
    "threadPoolSize": 16
  },
  "optimization": {
    "enableGzip": true,
    "enableBrotli": true,
    "minifyResponses": true,
    "enableCdn": true
  },
  "monitoring": {
    "metricsEnabled": true,
    "tracingEnabled": true,
    "samplingRate": 0.1
  }
}
```

### Update Performance Settings

Update performance configuration.

**Tauri Command:**
```typescript
const updatedPerf = await invoke<PerformanceConfiguration>('update_performance_configuration', {
  section: 'concurrency',
  settings: {
    maxConcurrentWorkflows: 75,
    threadPoolSize: 24
  }
})
```

## 🔄 Configuration Validation

### Validate Configuration

Validate configuration settings before applying.

**Tauri Command:**
```typescript
const validation = await invoke<ConfigurationValidation>('validate_configuration', {
  section: 'research',
  settings: {
    maxSources: 200,
    timeoutMinutes: 120
  }
})
```

**Response:**
```json
{
  "valid": false,
  "errors": [
    {
      "field": "maxSources",
      "message": "Maximum sources cannot exceed 100",
      "code": "VALUE_TOO_HIGH"
    }
  ],
  "warnings": [
    {
      "field": "timeoutMinutes", 
      "message": "High timeout values may impact performance",
      "code": "PERFORMANCE_WARNING"
    }
  ]
}
```

## 📋 Configuration Templates

### Get Configuration Templates

Retrieve predefined configuration templates.

**Tauri Command:**
```typescript
const templates = await invoke<ConfigurationTemplate[]>('get_configuration_templates')
```

**Response:**
```json
[
  {
    "id": "academic_research",
    "name": "Academic Research",
    "description": "Optimized for academic and scientific research",
    "settings": {
      "research": {
        "defaultMethodology": "don_lim",
        "preferredDepth": "comprehensive",
        "maxSources": 75,
        "preferredDomains": ["academic", "scientific"]
      }
    }
  },
  {
    "id": "business_intelligence",
    "name": "Business Intelligence",
    "description": "Optimized for business and market research",
    "settings": {
      "research": {
        "defaultMethodology": "nick_scamara",
        "preferredDepth": "standard",
        "maxSources": 50,
        "preferredDomains": ["industry", "news", "business"]
      }
    }
  }
]
```

### Apply Configuration Template

Apply a predefined configuration template.

**Tauri Command:**
```typescript
const appliedConfig = await invoke<UserConfiguration>('apply_configuration_template', {
  templateId: 'academic_research',
  overwriteExisting: false
})
```

## 🔍 Configuration Monitoring

### Get Configuration History

Retrieve configuration change history.

**Tauri Command:**
```typescript
const history = await invoke<ConfigurationHistory[]>('get_configuration_history', {
  section: 'research',
  limit: 50
})
```

### Monitor Configuration Changes

Set up real-time monitoring for configuration changes.

**Tauri Command:**
```typescript
const monitor = await invoke<ConfigurationMonitor>('setup_configuration_monitoring', {
  sections: ['security', 'research'],
  notifyOnChange: true
})
```

## 🚨 Error Handling

Common configuration errors:

```typescript
try {
  const config = await invoke('update_configuration', params)
} catch (error) {
  if (error.includes('INVALID_CONFIGURATION')) {
    // Handle invalid configuration
  } else if (error.includes('PERMISSION_DENIED')) {
    // Handle insufficient permissions
  } else if (error.includes('VALIDATION_FAILED')) {
    // Handle validation errors
  }
}
```

## 📚 Related Documentation

- [Authentication API](./authentication.md)
- [Security Architecture](../architecture/security-architecture.md)
- [Deployment Configuration](../deployment/README.md)

---

**Next**: Learn about [Monitoring API](./monitoring.md) for system health and performance tracking.
