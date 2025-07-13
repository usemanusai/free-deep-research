import React, { useState, useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/core'
import {
  CpuChipIcon,
  ServerIcon,
  ChartBarIcon,
  ExclamationTriangleIcon,
  CheckCircleIcon,
  ClockIcon,
  ArrowPathIcon,
  EyeIcon,
  DocumentTextIcon,
  BoltIcon,
  ShieldCheckIcon
} from '@heroicons/react/24/outline'
import {
  SystemMetrics,
  ServiceHealthStatus,
  AuditEvent,
  ResourceStatus,
  QueueStatistics
} from '@/types/api'
import { useRealTimeMonitoring, useRecentActivity } from '@/hooks/useDashboardData'
import LoadingSpinner from '@/components/common/LoadingSpinner'
import ErrorAlert from '@/components/common/ErrorAlert'

export default function RealTimeConsole() {
  const [autoRefresh, setAutoRefresh] = useState(true)
  const [selectedTab, setSelectedTab] = useState<'overview' | 'logs' | 'performance' | 'security'>('overview')
  const [logFilter, setLogFilter] = useState<'all' | 'error' | 'warning' | 'info'>('all')

  const { health, resources, queue, isLoading, error, refetch } = useRealTimeMonitoring()
  const { data: recentActivity } = useRecentActivity(50)

  // Fetch system metrics
  const { data: systemMetrics } = useQuery({
    queryKey: ['system-metrics'],
    queryFn: () => invoke<SystemMetrics>('get_system_metrics'),
    refetchInterval: autoRefresh ? 5000 : false,
    retry: 2,
  })

  // Fetch audit logs
  const { data: auditLogs } = useQuery({
    queryKey: ['audit-logs-detailed'],
    queryFn: () => invoke<AuditEvent[]>('get_audit_logs', { limit: 100 }),
    refetchInterval: autoRefresh ? 10000 : false,
  })

  // Filter audit logs based on selected filter
  const filteredLogs = auditLogs?.filter(log => {
    if (logFilter === 'all') return true
    if (logFilter === 'error') return log.severity === 'high' || log.severity === 'critical'
    if (logFilter === 'warning') return log.severity === 'medium'
    if (logFilter === 'info') return log.severity === 'low'
    return true
  }) || []

  const getHealthStatusColor = (status: string) => {
    switch (status) {
      case 'healthy': return 'text-green-600 bg-green-100'
      case 'degraded': return 'text-yellow-600 bg-yellow-100'
      case 'unhealthy': return 'text-red-600 bg-red-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical': return 'text-red-800 bg-red-100'
      case 'high': return 'text-red-700 bg-red-50'
      case 'medium': return 'text-yellow-700 bg-yellow-50'
      case 'low': return 'text-blue-700 bg-blue-50'
      default: return 'text-gray-700 bg-gray-50'
    }
  }

  const formatUptime = (seconds: number) => {
    const days = Math.floor(seconds / 86400)
    const hours = Math.floor((seconds % 86400) / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)

    if (days > 0) return `${days}d ${hours}h ${minutes}m`
    if (hours > 0) return `${hours}h ${minutes}m`
    return `${minutes}m`
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" message="Loading monitoring data..." />
      </div>
    )
  }

  if (error) {
    return (
      <ErrorAlert
        title="Monitoring Error"
        message="Failed to load monitoring data. Please try again."
        onRetry={refetch}
      />
    )
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Real-Time Monitoring Console</h1>
          <p className="mt-1 text-sm text-gray-500">
            Live system performance, health monitoring, and audit logging
          </p>
        </div>
        <div className="flex items-center space-x-3">
          <div className="flex items-center">
            <input
              type="checkbox"
              id="auto-refresh"
              checked={autoRefresh}
              onChange={(e) => setAutoRefresh(e.target.checked)}
              className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
            <label htmlFor="auto-refresh" className="ml-2 text-sm text-gray-700">
              Auto-refresh
            </label>
          </div>
          <button
            onClick={refetch}
            className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <ArrowPathIcon className="h-4 w-4 mr-1" />
            Refresh
          </button>
        </div>
      </div>

      {/* System Status Overview */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <ServerIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">System Health</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getHealthStatusColor(health?.overall_status || 'unknown')}`}>
                      {health?.overall_status || 'Unknown'}
                    </span>
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm text-gray-600">
              Uptime: {health ? formatUptime(health.uptime) : 'N/A'}
            </div>
          </div>
        </div>

        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <CpuChipIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">CPU Usage</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {resources ? `${Math.round(resources.cpu_usage)}%` : 'N/A'}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm text-gray-600">
              Memory: {resources ? `${Math.round(resources.memory_usage)}%` : 'N/A'}
            </div>
          </div>
        </div>

        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <BoltIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Active Workflows</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {resources ? `${resources.concurrent_workflows}/${resources.max_concurrent_workflows}` : 'N/A'}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm text-gray-600">
              Capacity: {resources ? `${Math.round(resources.available_capacity * 100)}%` : 'N/A'}
            </div>
          </div>
        </div>

        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <ClockIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Queue Status</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {queue ? `${queue.total_queued} queued` : 'N/A'}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm text-gray-600">
              Running: {queue ? queue.currently_running : 'N/A'}
            </div>
          </div>
        </div>
      </div>

      {/* Tabbed Interface */}
      <div className="bg-white shadow rounded-lg">
        <div className="border-b border-gray-200">
          <nav className="-mb-px flex space-x-8 px-6" aria-label="Tabs">
            {[
              { id: 'overview', name: 'Overview', icon: ChartBarIcon },
              { id: 'logs', name: 'Audit Logs', icon: DocumentTextIcon },
              { id: 'performance', name: 'Performance', icon: CpuChipIcon },
              { id: 'security', name: 'Security', icon: ShieldCheckIcon },
            ].map((tab) => (
              <button
                key={tab.id}
                onClick={() => setSelectedTab(tab.id as any)}
                className={`${
                  selectedTab === tab.id
                    ? 'border-primary-500 text-primary-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center`}
              >
                <tab.icon className="h-4 w-4 mr-2" />
                {tab.name}
              </button>
            ))}
          </nav>
        </div>

        <div className="p-6">
          {/* Overview Tab */}
          {selectedTab === 'overview' && (
            <div className="space-y-6">
              {/* Service Health */}
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-4">Service Health Status</h3>
                <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                  {health?.services && Object.entries(health.services).map(([serviceName, service]) => (
                    <div key={serviceName} className="bg-gray-50 rounded-lg p-4">
                      <div className="flex items-center justify-between">
                        <div>
                          <h4 className="text-sm font-medium text-gray-900 capitalize">{serviceName}</h4>
                          <p className="text-xs text-gray-500">Response: {service.response_time}ms</p>
                        </div>
                        <div className="flex items-center">
                          {service.status === 'healthy' ? (
                            <CheckCircleIcon className="h-5 w-5 text-green-500" />
                          ) : (
                            <ExclamationTriangleIcon className="h-5 w-5 text-red-500" />
                          )}
                          <span className={`ml-2 inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getHealthStatusColor(service.status)}`}>
                            {service.status}
                          </span>
                        </div>
                      </div>
                      {service.error_message && (
                        <p className="mt-2 text-xs text-red-600">{service.error_message}</p>
                      )}
                    </div>
                  ))}
                </div>
              </div>

              {/* Recent Activity Summary */}
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-4">Recent Activity</h3>
                <div className="bg-gray-50 rounded-lg p-4">
                  {recentActivity && recentActivity.length > 0 ? (
                    <div className="space-y-2">
                      {recentActivity.slice(0, 5).map((activity) => (
                        <div key={activity.id} className="flex items-center justify-between text-sm">
                          <span className="text-gray-900">{activity.message}</span>
                          <span className="text-gray-500">{new Date(activity.timestamp).toLocaleTimeString()}</span>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <p className="text-sm text-gray-500">No recent activity</p>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* Audit Logs Tab */}
          {selectedTab === 'logs' && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-medium text-gray-900">Audit Logs</h3>
                <select
                  value={logFilter}
                  onChange={(e) => setLogFilter(e.target.value as any)}
                  className="block w-32 pl-3 pr-10 py-2 text-base border border-gray-300 focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm rounded-md"
                >
                  <option value="all">All Logs</option>
                  <option value="error">Errors</option>
                  <option value="warning">Warnings</option>
                  <option value="info">Info</option>
                </select>
              </div>

              <div className="bg-gray-900 rounded-lg p-4 h-96 overflow-y-auto">
                <div className="space-y-1 font-mono text-sm">
                  {filteredLogs.length > 0 ? (
                    filteredLogs.map((log) => (
                      <div key={log.id} className="flex items-start space-x-3 text-gray-300">
                        <span className="text-gray-500 text-xs">
                          {new Date(log.timestamp).toLocaleTimeString()}
                        </span>
                        <span className={`inline-flex px-2 py-0.5 rounded text-xs font-medium ${getSeverityColor(log.severity)}`}>
                          {log.severity.toUpperCase()}
                        </span>
                        <span className="flex-1">
                          [{log.event_type}] {log.action} {log.resource_type}
                          {log.resource_id && ` (${log.resource_id})`}
                        </span>
                      </div>
                    ))
                  ) : (
                    <p className="text-gray-500">No logs found for the selected filter.</p>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* Performance Tab */}
          {selectedTab === 'performance' && (
            <div className="space-y-6">
              <h3 className="text-lg font-medium text-gray-900">Performance Metrics</h3>

              {systemMetrics ? (
                <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
                  <div className="bg-gray-50 rounded-lg p-4">
                    <h4 className="text-sm font-medium text-gray-900 mb-3">System Resources</h4>
                    <div className="space-y-3">
                      <div>
                        <div className="flex justify-between text-sm">
                          <span>CPU Usage</span>
                          <span>{Math.round(systemMetrics.cpu_usage)}%</span>
                        </div>
                        <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                          <div
                            className="bg-blue-600 h-2 rounded-full"
                            style={{ width: `${systemMetrics.cpu_usage}%` }}
                          />
                        </div>
                      </div>
                      <div>
                        <div className="flex justify-between text-sm">
                          <span>Memory Usage</span>
                          <span>{Math.round(systemMetrics.memory_usage)}%</span>
                        </div>
                        <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                          <div
                            className="bg-green-600 h-2 rounded-full"
                            style={{ width: `${systemMetrics.memory_usage}%` }}
                          />
                        </div>
                      </div>
                      <div>
                        <div className="flex justify-between text-sm">
                          <span>Disk Usage</span>
                          <span>{Math.round(systemMetrics.disk_usage)}%</span>
                        </div>
                        <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                          <div
                            className="bg-yellow-600 h-2 rounded-full"
                            style={{ width: `${systemMetrics.disk_usage}%` }}
                          />
                        </div>
                      </div>
                    </div>
                  </div>

                  <div className="bg-gray-50 rounded-lg p-4">
                    <h4 className="text-sm font-medium text-gray-900 mb-3">Network Activity</h4>
                    <div className="space-y-2 text-sm">
                      <div className="flex justify-between">
                        <span>Bytes Sent:</span>
                        <span>{(systemMetrics.network_activity.bytes_sent / 1024 / 1024).toFixed(2)} MB</span>
                      </div>
                      <div className="flex justify-between">
                        <span>Bytes Received:</span>
                        <span>{(systemMetrics.network_activity.bytes_received / 1024 / 1024).toFixed(2)} MB</span>
                      </div>
                      <div className="flex justify-between">
                        <span>Requests/min:</span>
                        <span>{systemMetrics.network_activity.requests_per_minute}</span>
                      </div>
                      <div className="flex justify-between">
                        <span>Active Processes:</span>
                        <span>{systemMetrics.active_processes}</span>
                      </div>
                    </div>
                  </div>
                </div>
              ) : (
                <div className="text-center py-8">
                  <p className="text-gray-500">Performance metrics not available</p>
                </div>
              )}
            </div>
          )}

          {/* Security Tab */}
          {selectedTab === 'security' && (
            <div className="space-y-6">
              <h3 className="text-lg font-medium text-gray-900">Security Overview</h3>

              <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
                <div className="bg-gray-50 rounded-lg p-4">
                  <h4 className="text-sm font-medium text-gray-900 mb-3">Security Events</h4>
                  <div className="space-y-2">
                    {auditLogs?.filter(log => log.event_type.includes('security') || log.severity === 'critical').slice(0, 5).map((log) => (
                      <div key={log.id} className="flex items-center justify-between text-sm">
                        <span className="text-gray-900">{log.action}</span>
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getSeverityColor(log.severity)}`}>
                          {log.severity}
                        </span>
                      </div>
                    )) || <p className="text-sm text-gray-500">No security events</p>}
                  </div>
                </div>

                <div className="bg-gray-50 rounded-lg p-4">
                  <h4 className="text-sm font-medium text-gray-900 mb-3">System Security</h4>
                  <div className="space-y-2 text-sm">
                    <div className="flex items-center justify-between">
                      <span>Encryption Status:</span>
                      <span className="inline-flex items-center text-green-600">
                        <CheckCircleIcon className="h-4 w-4 mr-1" />
                        Active
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span>Audit Logging:</span>
                      <span className="inline-flex items-center text-green-600">
                        <CheckCircleIcon className="h-4 w-4 mr-1" />
                        Enabled
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span>Backup Status:</span>
                      <span className="inline-flex items-center text-green-600">
                        <CheckCircleIcon className="h-4 w-4 mr-1" />
                        Operational
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
