# 📱 Mobile APIs

## Overview

The Mobile APIs provide comprehensive support for iOS, Android, and Web mobile platforms. Part of Phase 4 Advanced Features, these APIs enable full research functionality on mobile devices with optimized performance and offline capabilities.

## 📲 Platform Support

### Get Platform Capabilities

Retrieve platform-specific capabilities and limitations.

**Mobile Command:**
```typescript
const capabilities = await invoke<PlatformCapabilities>('get_mobile_platform_capabilities')
```

**Response:**
```json
{
  "platform": "ios", // 'ios', 'android', 'web'
  "version": "17.2",
  "capabilities": {
    "offlineMode": true,
    "backgroundProcessing": true,
    "pushNotifications": true,
    "biometricAuth": true,
    "fileSystem": true,
    "camera": true,
    "location": true,
    "networkDetection": true
  },
  "limitations": {
    "maxConcurrentRequests": 5,
    "maxStorageSize": "2GB",
    "backgroundTimeLimit": "30s",
    "memoryLimit": "1GB"
  },
  "optimizations": {
    "batteryOptimized": true,
    "dataCompressionEnabled": true,
    "adaptiveQuality": true
  }
}
```

### Configure Mobile Settings

Configure mobile-specific settings and optimizations.

**Mobile Command:**
```typescript
const mobileConfig = await invoke<MobileConfiguration>('configure_mobile_settings', {
  settings: {
    offlineMode: {
      enabled: true,
      syncInterval: 300, // 5 minutes
      maxOfflineStorage: "1GB",
      autoSync: true
    },
    performance: {
      adaptiveQuality: true,
      batteryOptimization: true,
      dataCompression: true,
      backgroundProcessing: "limited"
    },
    notifications: {
      researchComplete: true,
      errorAlerts: true,
      syncUpdates: false,
      quietHours: {
        enabled: true,
        start: "22:00",
        end: "07:00"
      }
    }
  }
})
```

## 🔄 Offline Capabilities

### Sync Offline Data

Synchronize offline data with the server when connection is available.

**Mobile Command:**
```typescript
const syncResult = await invoke<OfflineSyncResult>('sync_offline_data', {
  syncType: 'incremental', // 'full', 'incremental', 'priority_only'
  includeResults: true,
  includeTemplates: true,
  includeSettings: false
})
```

**Response:**
```json
{
  "syncId": "sync_123",
  "status": "completed",
  "startedAt": "2025-01-20T15:30:00Z",
  "completedAt": "2025-01-20T15:32:15Z",
  "summary": {
    "itemsSynced": 45,
    "itemsUploaded": 12,
    "itemsDownloaded": 33,
    "conflictsResolved": 2,
    "errors": 0
  },
  "details": {
    "researchResults": {
      "uploaded": 5,
      "downloaded": 8,
      "conflicts": 1
    },
    "templates": {
      "uploaded": 2,
      "downloaded": 15,
      "conflicts": 0
    },
    "userSettings": {
      "uploaded": 5,
      "downloaded": 10,
      "conflicts": 1
    }
  },
  "nextSyncRecommended": "2025-01-20T20:30:00Z"
}
```

### Get Offline Status

Check offline capabilities and current sync status.

**Mobile Command:**
```typescript
const offlineStatus = await invoke<OfflineStatus>('get_offline_status')
```

**Response:**
```json
{
  "isOnline": false,
  "lastSyncAt": "2025-01-20T14:15:00Z",
  "offlineCapabilities": {
    "canCreateWorkflows": true,
    "canExecuteBasicResearch": true,
    "canAccessTemplates": true,
    "canViewResults": true,
    "canExportResults": false
  },
  "offlineStorage": {
    "used": "456MB",
    "available": "1.5GB",
    "utilizationRate": 0.23
  },
  "pendingSync": {
    "itemsToUpload": 3,
    "itemsToDownload": 7,
    "estimatedSyncTime": "45s"
  }
}
```

## 📊 Mobile Research Operations

### Execute Mobile Research

Execute research workflows optimized for mobile devices.

**Mobile Command:**
```typescript
const mobileWorkflow = await invoke<MobileResearchWorkflow>('execute_mobile_research', {
  query: 'AI applications in mobile healthcare',
  methodology: 'mobile_optimized', // 'mobile_optimized', 'lightweight', 'standard'
  mobileOptions: {
    maxDuration: 600, // 10 minutes
    maxDataUsage: "50MB",
    batteryOptimized: true,
    backgroundExecution: false,
    qualityLevel: 'balanced' // 'fast', 'balanced', 'comprehensive'
  },
  offlineMode: false
})
```

**Response:**
```json
{
  "workflowId": "mobile_workflow_456",
  "status": "created",
  "mobileOptimizations": {
    "estimatedDuration": "8-12 minutes",
    "estimatedDataUsage": "35MB",
    "batteryImpact": "low",
    "backgroundCapable": false
  },
  "adaptiveSettings": {
    "sourcesReduced": true,
    "compressionEnabled": true,
    "priorityFiltering": true
  },
  "fallbackOptions": {
    "offlineMode": true,
    "cloudProcessing": true,
    "deferredExecution": true
  }
}
```

### Get Mobile Research Status

Monitor mobile research progress with mobile-specific metrics.

**Mobile Command:**
```typescript
const mobileStatus = await invoke<MobileResearchStatus>('get_mobile_research_status', {
  workflowId: 'mobile_workflow_456'
})
```

**Response:**
```json
{
  "workflowId": "mobile_workflow_456",
  "status": "running",
  "progress": {
    "percentage": 65,
    "currentPhase": "source_analysis",
    "estimatedTimeRemaining": "4m 30s"
  },
  "mobileMetrics": {
    "dataUsed": "22MB",
    "batteryUsed": "3%",
    "networkRequests": 45,
    "cacheHitRate": "78%"
  },
  "adaptations": {
    "qualityReduced": false,
    "sourcesFiltered": true,
    "compressionApplied": true
  },
  "warnings": [
    {
      "type": "battery_low",
      "message": "Battery below 20%, consider enabling battery optimization",
      "severity": "medium"
    }
  ]
}
```

## 📱 Mobile UI Components

### Get Mobile Dashboard Data

Retrieve dashboard data optimized for mobile display.

**Mobile Command:**
```typescript
const dashboardData = await invoke<MobileDashboardData>('get_mobile_dashboard_data', {
  layout: 'compact', // 'compact', 'detailed', 'minimal'
  includeCharts: false,
  maxItems: 10
})
```

**Response:**
```json
{
  "layout": "compact",
  "summary": {
    "activeWorkflows": 2,
    "completedToday": 5,
    "totalCostToday": "$12.50",
    "successRate": "96%"
  },
  "recentActivity": [
    {
      "id": "activity_001",
      "type": "workflow_completed",
      "title": "AI Healthcare Research",
      "timestamp": "2025-01-20T15:30:00Z",
      "status": "success",
      "duration": "12m 45s"
    }
  ],
  "quickActions": [
    {
      "id": "quick_research",
      "title": "Quick Research",
      "icon": "search",
      "action": "create_workflow"
    },
    {
      "id": "templates",
      "title": "Templates",
      "icon": "template",
      "action": "view_templates"
    }
  ],
  "notifications": [
    {
      "id": "notif_001",
      "type": "sync_available",
      "message": "7 items ready to sync",
      "priority": "medium"
    }
  ]
}
```

### Configure Mobile UI

Configure mobile user interface preferences.

**Mobile Command:**
```typescript
const uiConfig = await invoke<MobileUIConfiguration>('configure_mobile_ui', {
  preferences: {
    theme: 'auto', // 'light', 'dark', 'auto'
    fontSize: 'medium', // 'small', 'medium', 'large'
    compactMode: true,
    gestureNavigation: true,
    hapticFeedback: true,
    animations: 'reduced' // 'none', 'reduced', 'full'
  },
  accessibility: {
    highContrast: false,
    largeText: false,
    voiceOver: false,
    reduceMotion: true
  }
})
```

## 🔔 Push Notifications

### Configure Push Notifications

Set up and configure push notifications for mobile devices.

**Mobile Command:**
```typescript
const notificationConfig = await invoke<PushNotificationConfig>('configure_push_notifications', {
  deviceToken: 'device_token_abc123',
  preferences: {
    researchComplete: true,
    errorAlerts: true,
    syncUpdates: false,
    weeklyReports: true,
    marketplaceUpdates: false
  },
  schedule: {
    quietHours: {
      enabled: true,
      start: "22:00",
      end: "07:00",
      timezone: "UTC"
    },
    workingDays: ["monday", "tuesday", "wednesday", "thursday", "friday"]
  }
})
```

### Send Test Notification

Send a test push notification to verify configuration.

**Mobile Command:**
```typescript
const testNotification = await invoke<NotificationTestResult>('send_test_notification', {
  deviceToken: 'device_token_abc123',
  message: {
    title: 'Test Notification',
    body: 'This is a test notification from Free Deep Research',
    data: {
      type: 'test',
      timestamp: '2025-01-20T15:30:00Z'
    }
  }
})
```

## 📍 Location & Context

### Get Location Context

Retrieve location-based context for research optimization.

**Mobile Command:**
```typescript
const locationContext = await invoke<LocationContext>('get_location_context', {
  includeTimezone: true,
  includeRegionalSettings: true,
  includeNetworkInfo: true
})
```

**Response:**
```json
{
  "location": {
    "country": "US",
    "region": "California",
    "city": "San Francisco",
    "timezone": "America/Los_Angeles",
    "coordinates": {
      "latitude": 37.7749,
      "longitude": -122.4194
    }
  },
  "regionalSettings": {
    "language": "en-US",
    "currency": "USD",
    "dateFormat": "MM/DD/YYYY",
    "timeFormat": "12h"
  },
  "networkInfo": {
    "type": "wifi", // 'wifi', 'cellular', 'ethernet'
    "speed": "high",
    "dataLimited": false,
    "roaming": false
  },
  "optimizations": {
    "preferredServers": ["us-west-1", "us-west-2"],
    "localizedContent": true,
    "regionalCompliance": ["CCPA"]
  }
}
```

## 🔋 Battery & Performance

### Get Battery Status

Monitor battery status and optimize operations accordingly.

**Mobile Command:**
```typescript
const batteryStatus = await invoke<BatteryStatus>('get_battery_status')
```

**Response:**
```json
{
  "level": 0.65,
  "isCharging": false,
  "chargingType": null,
  "estimatedTimeRemaining": "4h 30m",
  "batteryHealth": "good",
  "powerSavingMode": false,
  "recommendations": [
    {
      "action": "enable_battery_optimization",
      "reason": "Battery below 70%",
      "impact": "Extend usage by 30 minutes"
    }
  ],
  "optimizations": {
    "backgroundProcessingReduced": false,
    "networkRequestsOptimized": true,
    "displayBrightnessReduced": false
  }
}
```

### Optimize for Battery

Enable battery optimization features.

**Mobile Command:**
```typescript
const batteryOptimization = await invoke<BatteryOptimizationResult>('optimize_for_battery', {
  level: 'aggressive', // 'minimal', 'moderate', 'aggressive'
  preserveFeatures: ['notifications', 'sync'],
  temporaryMode: true,
  duration: 3600 // 1 hour
})
```

## 📱 Device Integration

### Access Device Features

Access device-specific features like camera, files, etc.

**Mobile Command:**
```typescript
const deviceAccess = await invoke<DeviceFeatureAccess>('access_device_feature', {
  feature: 'camera', // 'camera', 'files', 'contacts', 'calendar'
  purpose: 'document_scan',
  permissions: ['read', 'write']
})
```

### Share Content

Share research results using native sharing capabilities.

**Mobile Command:**
```typescript
const shareResult = await invoke<ShareResult>('share_mobile_content', {
  workflowId: 'mobile_workflow_456',
  shareType: 'native', // 'native', 'email', 'message', 'social'
  format: 'summary', // 'summary', 'full_report', 'link'
  includeAttachments: false
})
```

## 🚨 Error Handling

Common mobile API errors:

```typescript
try {
  const workflow = await invoke('execute_mobile_research', params)
} catch (error) {
  if (error.includes('INSUFFICIENT_BATTERY')) {
    // Handle low battery
  } else if (error.includes('NETWORK_UNAVAILABLE')) {
    // Handle offline mode
  } else if (error.includes('STORAGE_FULL')) {
    // Handle storage issues
  } else if (error.includes('PERMISSION_DENIED')) {
    // Handle permission issues
  }
}
```

## 📚 Related Documentation

- [Research Workflow API](./research-workflow.md)
- [Configuration API](./configuration.md)
- [Analytics API](./analytics.md)

---

**Next**: Explore [Advanced Analytics API](./advanced-analytics.md) for predictive analytics and insights.
