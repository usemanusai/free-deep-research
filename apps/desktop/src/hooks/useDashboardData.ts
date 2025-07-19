import { useQuery, useQueries } from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/core'
import { 
  DashboardStats, 
  ServiceHealthStatus, 
  QueueStatistics, 
  ResourceStatus,
  ApiKey,
  ResearchWorkflow,
  ActivityEvent,
  AuditEvent
} from '@/types/api'

// ============================================================================
// DASHBOARD DATA HOOKS
// ============================================================================

export function useDashboardStats() {
  const queries = useQueries({
    queries: [
      {
        queryKey: ['api-keys'],
        queryFn: () => invoke<ApiKey[]>('get_api_keys'),
        refetchInterval: 30000,
      },
      {
        queryKey: ['research-workflows'],
        queryFn: () => invoke<ResearchWorkflow[]>('get_all_research_workflows'),
        refetchInterval: 15000,
      },
      {
        queryKey: ['service-health'],
        queryFn: () => invoke<ServiceHealthStatus>('get_service_health'),
        refetchInterval: 10000,
      },
      {
        queryKey: ['queue-statistics'],
        queryFn: () => invoke<QueueStatistics>('get_queue_statistics'),
        refetchInterval: 15000,
      },
      {
        queryKey: ['audit-logs'],
        queryFn: () => invoke<AuditEvent[]>('get_audit_logs', { limit: 10 }),
        refetchInterval: 30000,
      }
    ]
  })

  const [apiKeysQuery, workflowsQuery, healthQuery, queueQuery, auditQuery] = queries

  // Calculate dashboard statistics
  const dashboardStats: DashboardStats | null = (() => {
    if (!apiKeysQuery.data || !workflowsQuery.data || !healthQuery.data) {
      return null
    }

    const apiKeys = apiKeysQuery.data
    const workflows = workflowsQuery.data
    const health = healthQuery.data
    const auditLogs = auditQuery.data || []

    // Convert audit logs to activity events
    const recentActivity: ActivityEvent[] = auditLogs.slice(0, 5).map(log => ({
      id: log.id,
      type: getActivityType(log.event_type),
      message: formatActivityMessage(log),
      timestamp: log.timestamp,
      severity: mapSeverity(log.severity),
      metadata: log.details
    }))

    return {
      total_api_keys: apiKeys.length,
      active_api_keys: apiKeys.filter(key => key.status === 'active').length,
      total_research: workflows.length,
      active_research: workflows.filter(w => w.status === 'running').length,
      completed_research: workflows.filter(w => w.status === 'completed').length,
      failed_research: workflows.filter(w => w.status === 'failed').length,
      system_health: health,
      uptime: formatUptime(health.uptime),
      recent_activity: recentActivity
    }
  })()

  return {
    data: dashboardStats,
    isLoading: queries.some(q => q.isLoading),
    error: queries.find(q => q.error)?.error,
    refetch: () => queries.forEach(q => q.refetch())
  }
}

export function useSystemHealth() {
  return useQuery({
    queryKey: ['service-health'],
    queryFn: () => invoke<ServiceHealthStatus>('get_service_health'),
    refetchInterval: 10000, // Refetch every 10 seconds
    retry: 3,
    retryDelay: 1000,
  })
}

export function useResourceStatus() {
  return useQuery({
    queryKey: ['resource-status'],
    queryFn: () => invoke<ResourceStatus>('get_resource_status'),
    refetchInterval: 5000, // Refetch every 5 seconds for real-time monitoring
    retry: 2,
  })
}

export function useQueueStatistics() {
  return useQuery({
    queryKey: ['queue-statistics'],
    queryFn: () => invoke<QueueStatistics>('get_queue_statistics'),
    refetchInterval: 15000,
    retry: 2,
  })
}

export function useRecentActivity(limit: number = 10) {
  return useQuery({
    queryKey: ['recent-activity', limit],
    queryFn: async () => {
      const auditLogs = await invoke<AuditEvent[]>('get_audit_logs', { limit })
      return auditLogs.map(log => ({
        id: log.id,
        type: getActivityType(log.event_type),
        message: formatActivityMessage(log),
        timestamp: log.timestamp,
        severity: mapSeverity(log.severity),
        metadata: log.details
      })) as ActivityEvent[]
    },
    refetchInterval: 30000,
  })
}

export function useApiKeyStats() {
  return useQuery({
    queryKey: ['api-key-stats'],
    queryFn: async () => {
      const apiKeys = await invoke<ApiKey[]>('get_api_keys')
      const usageStats = await invoke('get_api_usage_stats')
      
      return {
        total: apiKeys.length,
        active: apiKeys.filter(key => key.status === 'active').length,
        exhausted: apiKeys.filter(key => key.status === 'exhausted').length,
        error: apiKeys.filter(key => key.status === 'error').length,
        usage_stats: usageStats
      }
    },
    refetchInterval: 30000,
  })
}

export function useWorkflowStats() {
  return useQuery({
    queryKey: ['workflow-stats'],
    queryFn: async () => {
      const workflows = await invoke<ResearchWorkflow[]>('get_all_research_workflows')
      
      return {
        total: workflows.length,
        running: workflows.filter(w => w.status === 'running').length,
        queued: workflows.filter(w => w.status === 'queued').length,
        completed: workflows.filter(w => w.status === 'completed').length,
        failed: workflows.filter(w => w.status === 'failed').length,
        paused: workflows.filter(w => w.status === 'paused').length,
      }
    },
    refetchInterval: 15000,
  })
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

function getActivityType(eventType: string): ActivityEvent['type'] {
  if (eventType.includes('research') || eventType.includes('workflow')) {
    return 'research'
  }
  if (eventType.includes('api') || eventType.includes('key')) {
    return 'api'
  }
  if (eventType.includes('security') || eventType.includes('auth')) {
    return 'security'
  }
  return 'system'
}

function formatActivityMessage(log: AuditEvent): string {
  const action = log.action.replace(/_/g, ' ').toLowerCase()
  const resourceType = log.resource_type.replace(/_/g, ' ').toLowerCase()
  
  return `${action} ${resourceType}${log.resource_id ? ` (${log.resource_id})` : ''}`
}

function mapSeverity(severity: AuditEvent['severity']): ActivityEvent['severity'] {
  switch (severity) {
    case 'low': return 'info'
    case 'medium': return 'warning'
    case 'high': return 'error'
    case 'critical': return 'error'
    default: return 'info'
  }
}

function formatUptime(uptimeSeconds: number): string {
  const hours = Math.floor(uptimeSeconds / 3600)
  const minutes = Math.floor((uptimeSeconds % 3600) / 60)
  
  if (hours > 0) {
    return `${hours}h ${minutes}m`
  }
  return `${minutes}m`
}

// ============================================================================
// REAL-TIME MONITORING HOOKS
// ============================================================================

export function useRealTimeMonitoring() {
  const healthQuery = useSystemHealth()
  const resourceQuery = useResourceStatus()
  const queueQuery = useQueueStatistics()
  
  return {
    health: healthQuery.data,
    resources: resourceQuery.data,
    queue: queueQuery.data,
    isLoading: healthQuery.isLoading || resourceQuery.isLoading || queueQuery.isLoading,
    error: healthQuery.error || resourceQuery.error || queueQuery.error,
    refetch: () => {
      healthQuery.refetch()
      resourceQuery.refetch()
      queueQuery.refetch()
    }
  }
}

export function usePerformanceMetrics() {
  return useQuery({
    queryKey: ['performance-metrics'],
    queryFn: () => invoke('get_all_performance_metrics'),
    refetchInterval: 30000,
  })
}
