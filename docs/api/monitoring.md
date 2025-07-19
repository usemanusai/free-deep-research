# 🔍 Monitoring API

## Overview

The Monitoring API provides comprehensive system health monitoring, performance tracking, and alerting capabilities for the Free Deep Research System.

## 🏥 System Health

### Get System Health

Retrieve overall system health status and metrics.

**Tauri Command:**
```typescript
const health = await invoke<SystemHealth>('get_system_health')
```

**REST Endpoint:**
```http
GET /api/monitoring/health
Authorization: Bearer YOUR_API_KEY
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2025-01-20T15:30:00Z",
  "uptime": "15d 8h 32m",
  "version": "3.0.0",
  "services": {
    "researchEngine": {
      "status": "healthy",
      "responseTime": "145ms",
      "activeWorkflows": 23,
      "queuedWorkflows": 5
    },
    "aiOrchestrator": {
      "status": "healthy",
      "responseTime": "89ms",
      "activeAgents": 8,
      "queuedTasks": 12
    },
    "database": {
      "status": "healthy",
      "responseTime": "12ms",
      "connections": 45,
      "maxConnections": 100
    },
    "cache": {
      "status": "healthy",
      "responseTime": "3ms",
      "hitRate": "87%",
      "memoryUsage": "68%"
    }
  },
  "resources": {
    "cpu": {
      "usage": "34%",
      "cores": 16,
      "loadAverage": [1.2, 1.5, 1.8]
    },
    "memory": {
      "usage": "67%",
      "total": "32GB",
      "available": "10.5GB"
    },
    "disk": {
      "usage": "45%",
      "total": "1TB",
      "available": "550GB"
    }
  }
}
```

### Get Service Health

Check health status of individual services.

**Tauri Command:**
```typescript
const serviceHealth = await invoke<ServiceHealth>('get_service_health', {
  service: 'research_engine' // 'research_engine', 'ai_orchestrator', 'database', 'cache'
})
```

**Response:**
```json
{
  "service": "research_engine",
  "status": "healthy",
  "details": {
    "responseTime": "145ms",
    "successRate": "98.5%",
    "activeConnections": 23,
    "queueDepth": 5,
    "lastHealthCheck": "2025-01-20T15:29:45Z"
  },
  "dependencies": {
    "database": "healthy",
    "cache": "healthy",
    "externalApis": "healthy"
  },
  "metrics": {
    "requestsPerSecond": 12.5,
    "averageProcessingTime": "18m 32s",
    "errorRate": "1.5%"
  }
}
```

## 📊 Performance Monitoring

### Get System Metrics

Retrieve detailed system performance metrics.

**Tauri Command:**
```typescript
const metrics = await invoke<SystemMetrics>('get_system_metrics', {
  timeframe: 'last_1_hour', // 'last_5_minutes', 'last_1_hour', 'last_24_hours'
  granularity: 'minute' // 'second', 'minute', 'hour'
})
```

**Response:**
```json
{
  "timeframe": "last_1_hour",
  "metrics": {
    "cpu": {
      "average": 34.5,
      "peak": 67.2,
      "trend": "stable"
    },
    "memory": {
      "average": 67.8,
      "peak": 78.9,
      "trend": "increasing"
    },
    "network": {
      "inbound": "125 MB/s",
      "outbound": "89 MB/s",
      "latency": "12ms"
    },
    "disk": {
      "readOps": 1250,
      "writeOps": 890,
      "ioWait": "2.3%"
    }
  },
  "timeSeries": [
    {
      "timestamp": "2025-01-20T15:00:00Z",
      "cpu": 32.1,
      "memory": 65.4,
      "networkLatency": 11
    }
  ]
}
```

### Get API Usage Stats

Monitor API usage patterns and performance.

**Tauri Command:**
```typescript
const apiStats = await invoke<ApiUsageStats>('get_api_usage_stats', {
  timeframe: 'last_24_hours',
  includeEndpoints: true
})
```

**Response:**
```json
{
  "summary": {
    "totalRequests": 125000,
    "successfulRequests": 123250,
    "failedRequests": 1750,
    "averageResponseTime": "185ms",
    "requestsPerSecond": 145
  },
  "endpoints": {
    "/api/research/workflows": {
      "requests": 25000,
      "averageResponseTime": "245ms",
      "successRate": "98.9%"
    },
    "/api/auth/keys": {
      "requests": 15000,
      "averageResponseTime": "89ms",
      "successRate": "99.8%"
    }
  },
  "errors": {
    "4xx": 1200,
    "5xx": 550,
    "timeouts": 125,
    "rateLimits": 75
  }
}
```

## 🚨 Alerting & Notifications

### Get Active Alerts

Retrieve current system alerts and warnings.

**Tauri Command:**
```typescript
const alerts = await invoke<SystemAlert[]>('get_active_alerts')
```

**Response:**
```json
[
  {
    "id": "alert_001",
    "severity": "warning",
    "type": "performance",
    "title": "High Memory Usage",
    "message": "Memory usage has exceeded 75% for the last 10 minutes",
    "timestamp": "2025-01-20T15:25:00Z",
    "source": "system_monitor",
    "acknowledged": false,
    "actions": [
      "Scale up memory",
      "Restart services",
      "Clear cache"
    ]
  },
  {
    "id": "alert_002",
    "severity": "info",
    "type": "maintenance",
    "title": "Scheduled Maintenance",
    "message": "Database maintenance scheduled for tonight at 2 AM UTC",
    "timestamp": "2025-01-20T14:00:00Z",
    "source": "maintenance_scheduler",
    "acknowledged": true
  }
]
```

### Configure Alert Rules

Set up custom alerting rules and thresholds.

**Tauri Command:**
```typescript
const alertRule = await invoke<AlertRule>('create_alert_rule', {
  name: 'High CPU Usage',
  condition: {
    metric: 'cpu_usage',
    operator: 'greater_than',
    threshold: 80,
    duration: '5m'
  },
  severity: 'warning',
  notifications: ['email', 'slack'],
  actions: ['scale_up', 'notify_admin']
})
```

### Acknowledge Alert

Acknowledge and manage system alerts.

**Tauri Command:**
```typescript
await invoke('acknowledge_alert', {
  alertId: 'alert_001',
  acknowledgedBy: 'admin@example.com',
  notes: 'Investigating memory usage spike'
})
```

## 📋 Audit Logging

### Get Audit Logs

Retrieve system audit logs for security and compliance.

**Tauri Command:**
```typescript
const auditLogs = await invoke<AuditLog[]>('get_audit_logs', {
  timeframe: 'last_24_hours',
  eventTypes: ['authentication', 'api_access', 'configuration_change'],
  userId: 'user_123', // Optional: filter by user
  limit: 100
})
```

**Response:**
```json
[
  {
    "id": "audit_001",
    "timestamp": "2025-01-20T15:30:00Z",
    "eventType": "authentication",
    "userId": "user_123",
    "userEmail": "user@example.com",
    "action": "login_success",
    "ipAddress": "192.168.1.100",
    "userAgent": "Mozilla/5.0...",
    "details": {
      "method": "api_key",
      "sessionId": "session_456"
    }
  },
  {
    "id": "audit_002",
    "timestamp": "2025-01-20T15:28:00Z",
    "eventType": "api_access",
    "userId": "user_123",
    "action": "create_workflow",
    "resource": "workflow_789",
    "details": {
      "endpoint": "/api/research/workflows",
      "method": "POST",
      "responseCode": 201
    }
  }
]
```

### Export Audit Logs

Export audit logs for compliance reporting.

**Tauri Command:**
```typescript
const exportData = await invoke<AuditLogExport>('export_audit_logs', {
  timeframe: 'last_30_days',
  format: 'csv', // 'csv', 'json', 'pdf'
  includeMetadata: true,
  encryptExport: true
})
```

## 🔧 Resource Monitoring

### Get Resource Usage

Monitor system resource utilization.

**Tauri Command:**
```typescript
const resources = await invoke<ResourceUsage>('get_resource_usage', {
  includeHistory: true,
  timeframe: 'last_6_hours'
})
```

**Response:**
```json
{
  "current": {
    "cpu": {
      "usage": 34.5,
      "cores": 16,
      "processes": 245
    },
    "memory": {
      "usage": 67.8,
      "total": "32GB",
      "available": "10.3GB",
      "cached": "8.2GB"
    },
    "disk": {
      "usage": 45.2,
      "total": "1TB",
      "available": "548GB",
      "iops": 1250
    },
    "network": {
      "inbound": "125 MB/s",
      "outbound": "89 MB/s",
      "connections": 1250
    }
  },
  "history": [
    {
      "timestamp": "2025-01-20T15:00:00Z",
      "cpu": 32.1,
      "memory": 65.4,
      "disk": 44.8,
      "network": 98.5
    }
  ]
}
```

### Get Database Metrics

Monitor database performance and health.

**Tauri Command:**
```typescript
const dbMetrics = await invoke<DatabaseMetrics>('get_database_metrics')
```

**Response:**
```json
{
  "connections": {
    "active": 45,
    "idle": 15,
    "max": 100,
    "utilization": 0.6
  },
  "performance": {
    "averageQueryTime": "12ms",
    "slowQueries": 3,
    "cacheHitRate": "94%",
    "transactionsPerSecond": 125
  },
  "storage": {
    "size": "125GB",
    "growth": "2.3GB/week",
    "indexSize": "15GB",
    "tableCount": 45
  }
}
```

## 📈 Real-time Monitoring

### Subscribe to Real-time Metrics

Set up real-time monitoring with WebSocket connections.

**Tauri Command:**
```typescript
const subscription = await invoke<MonitoringSubscription>('subscribe_to_metrics', {
  metrics: ['cpu', 'memory', 'api_requests'],
  interval: 5000, // 5 seconds
  callback: (data) => {
    console.log('Real-time metrics:', data)
  }
})
```

### Get Live Dashboard Data

Retrieve live data for monitoring dashboards.

**Tauri Command:**
```typescript
const dashboardData = await invoke<LiveDashboardData>('get_live_dashboard_data')
```

**Response:**
```json
{
  "timestamp": "2025-01-20T15:30:00Z",
  "summary": {
    "status": "healthy",
    "activeUsers": 245,
    "activeWorkflows": 23,
    "systemLoad": 0.34
  },
  "alerts": {
    "critical": 0,
    "warning": 2,
    "info": 5
  },
  "performance": {
    "responseTime": "185ms",
    "throughput": "145 req/s",
    "errorRate": "0.03%"
  }
}
```

## 🔍 Troubleshooting

### Get System Diagnostics

Run comprehensive system diagnostics.

**Tauri Command:**
```typescript
const diagnostics = await invoke<SystemDiagnostics>('run_system_diagnostics')
```

**Response:**
```json
{
  "overall": "healthy",
  "checks": {
    "database_connectivity": "passed",
    "external_api_access": "passed",
    "disk_space": "warning",
    "memory_usage": "passed",
    "service_health": "passed"
  },
  "recommendations": [
    "Consider increasing disk space allocation",
    "Monitor memory usage trends",
    "Update external API configurations"
  ],
  "details": {
    "disk_space": {
      "status": "warning",
      "message": "Disk usage at 85%, consider cleanup or expansion"
    }
  }
}
```

## 🚨 Error Handling

Common monitoring errors:

```typescript
try {
  const health = await invoke('get_system_health')
} catch (error) {
  if (error.includes('SERVICE_UNAVAILABLE')) {
    // Handle service unavailability
  } else if (error.includes('INSUFFICIENT_PERMISSIONS')) {
    // Handle permission errors
  } else if (error.includes('MONITORING_DISABLED')) {
    // Handle disabled monitoring
  }
}
```

## 📚 Related Documentation

- [Analytics API](./analytics.md)
- [Configuration API](./configuration.md)
- [System Architecture](../architecture/system-overview.md)

---

**Next**: Learn about [Template Management API](./template-management.md) for research template operations.
