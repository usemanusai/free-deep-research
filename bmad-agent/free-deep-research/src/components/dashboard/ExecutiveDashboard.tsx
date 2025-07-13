import React from 'react'
import {
  KeyIcon,
  BeakerIcon,
  ChartBarIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  ClockIcon,
  ArrowPathIcon,
  PlayIcon,
  PauseIcon,
  StopIcon,
  CpuChipIcon,
  ServerIcon
} from '@heroicons/react/24/outline'
import {
  useDashboardStats,
  useRealTimeMonitoring,
  useApiKeyStats,
  useWorkflowStats
} from '@/hooks/useDashboardData'
import LoadingSpinner from '@/components/common/LoadingSpinner'
import ErrorAlert from '@/components/common/ErrorAlert'

export default function ExecutiveDashboard() {
  const { data: dashboardStats, isLoading, error, refetch } = useDashboardStats()
  const { health, resources, queue } = useRealTimeMonitoring()
  const { data: apiKeyStats } = useApiKeyStats()
  const { data: workflowStats } = useWorkflowStats()

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" message="Loading dashboard data..." />
      </div>
    )
  }

  if (error) {
    return (
      <ErrorAlert
        title="Dashboard Error"
        message="Failed to load dashboard data. Please try again."
        onRetry={refetch}
      />
    )
  }

  if (!dashboardStats) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500">No dashboard data available</p>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* Header with Real-time Status */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Executive Dashboard</h1>
          <p className="mt-1 text-sm text-gray-500">
            Real-time overview of your Free Deep Research System
          </p>
        </div>
        <div className="flex items-center space-x-4">
          {/* System Health Indicator */}
          <div className="flex items-center space-x-2">
            {health?.overall_status === 'healthy' ? (
              <CheckCircleIcon className="h-5 w-5 text-green-500" />
            ) : health?.overall_status === 'degraded' ? (
              <ExclamationTriangleIcon className="h-5 w-5 text-yellow-500" />
            ) : (
              <ExclamationTriangleIcon className="h-5 w-5 text-red-500" />
            )}
            <span className="text-sm font-medium text-gray-700 capitalize">
              {health?.overall_status || 'Unknown'}
            </span>
          </div>

          {/* Refresh Button */}
          <button
            onClick={refetch}
            className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <ArrowPathIcon className="h-4 w-4 mr-1" />
            Refresh
          </button>
        </div>
      </div>

      {/* Enhanced Stats Grid */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        {/* API Keys */}
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <KeyIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">API Keys</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {dashboardStats.active_api_keys} / {dashboardStats.total_api_keys}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm">
              <span className="text-green-600 font-medium">{dashboardStats.active_api_keys} active</span>
              {apiKeyStats && apiKeyStats.exhausted > 0 && (
                <span className="text-red-600 font-medium ml-2">
                  {apiKeyStats.exhausted} exhausted
                </span>
              )}
            </div>
          </div>
        </div>

        {/* Research Workflows */}
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <BeakerIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Research</dt>
                  <dd className="text-lg font-medium text-gray-900">{dashboardStats.total_research}</dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm space-x-4">
              <span className="text-blue-600 font-medium">{dashboardStats.active_research} running</span>
              {queue && queue.total_queued > 0 && (
                <span className="text-yellow-600 font-medium">{queue.total_queued} queued</span>
              )}
            </div>
          </div>
        </div>

        {/* System Resources */}
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <CpuChipIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">System Resources</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {resources ? `${Math.round(resources.cpu_usage)}%` : 'N/A'}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm">
              {resources && (
                <>
                  <span className="text-gray-600">
                    CPU: {Math.round(resources.cpu_usage)}% |
                    Memory: {Math.round(resources.memory_usage)}%
                  </span>
                </>
              )}
            </div>
          </div>
        </div>

        {/* System Health */}
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                {health?.overall_status === 'healthy' ? (
                  <CheckCircleIcon className="h-6 w-6 text-green-400" />
                ) : health?.overall_status === 'degraded' ? (
                  <ExclamationTriangleIcon className="h-6 w-6 text-yellow-400" />
                ) : (
                  <ExclamationTriangleIcon className="h-6 w-6 text-red-400" />
                )}
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">System Health</dt>
                  <dd className="text-lg font-medium text-gray-900 capitalize">
                    {health?.overall_status || 'Unknown'}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm">
              {health && (
                <span className="text-gray-600">
                  {Object.keys(health.services).length} services monitored
                </span>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* Enhanced Quick Actions */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-4 py-5 sm:p-6">
          <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">Quick Actions</h3>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <button
              className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              onClick={() => window.location.href = '/research'}
            >
              <BeakerIcon className="h-4 w-4 mr-2" />
              Start Research
            </button>
            <button
              className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md shadow-sm text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              onClick={() => window.location.href = '/api-keys'}
            >
              <KeyIcon className="h-4 w-4 mr-2" />
              Manage API Keys
            </button>
            <button
              className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md shadow-sm text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              onClick={() => window.location.href = '/monitoring'}
            >
              <ChartBarIcon className="h-4 w-4 mr-2" />
              View Analytics
            </button>
            <button
              className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md shadow-sm text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              onClick={() => {
                // TODO: Implement export functionality
                console.log('Export data clicked')
              }}
            >
              <ServerIcon className="h-4 w-4 mr-2" />
              Export Data
            </button>
          </div>
        </div>
      </div>

      {/* Queue Management Panel */}
      {queue && (
        <div className="bg-white shadow rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg leading-6 font-medium text-gray-900">Queue Management</h3>
              <div className="flex items-center space-x-2">
                <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                  queue.currently_running > 0 ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
                }`}>
                  {queue.currently_running > 0 ? 'Processing' : 'Idle'}
                </span>
              </div>
            </div>
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
              <div className="text-center">
                <div className="text-2xl font-bold text-gray-900">{queue.total_queued}</div>
                <div className="text-sm text-gray-500">Queued</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">{queue.currently_running}</div>
                <div className="text-sm text-gray-500">Running</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-green-600">{queue.completed_today}</div>
                <div className="text-sm text-gray-500">Completed Today</div>
              </div>
            </div>
            {queue.estimated_queue_time > 0 && (
              <div className="mt-4 p-3 bg-blue-50 rounded-md">
                <p className="text-sm text-blue-700">
                  Estimated queue time: {Math.round(queue.estimated_queue_time / 60)} minutes
                </p>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Enhanced Recent Activity */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-4 py-5 sm:p-6">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg leading-6 font-medium text-gray-900">Recent Activity</h3>
            <button
              onClick={() => window.location.href = '/monitoring'}
              className="text-sm text-primary-600 hover:text-primary-500"
            >
              View all →
            </button>
          </div>
          <div className="flow-root">
            {dashboardStats.recent_activity.length > 0 ? (
              <ul className="-mb-8">
                {dashboardStats.recent_activity.map((activity, activityIdx) => (
                  <li key={activity.id}>
                    <div className="relative pb-8">
                      {activityIdx !== dashboardStats.recent_activity.length - 1 ? (
                        <span
                          className="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200"
                          aria-hidden="true"
                        />
                      ) : null}
                      <div className="relative flex space-x-3">
                        <div>
                          <span className={`h-8 w-8 rounded-full flex items-center justify-center ring-8 ring-white ${
                            activity.type === 'research' ? 'bg-blue-500' :
                            activity.type === 'api' ? 'bg-green-500' :
                            activity.type === 'security' ? 'bg-red-500' : 'bg-gray-500'
                          }`}>
                            {activity.type === 'research' ? (
                              <BeakerIcon className="h-4 w-4 text-white" />
                            ) : activity.type === 'api' ? (
                              <KeyIcon className="h-4 w-4 text-white" />
                            ) : activity.type === 'security' ? (
                              <ExclamationTriangleIcon className="h-4 w-4 text-white" />
                            ) : (
                              <ChartBarIcon className="h-4 w-4 text-white" />
                            )}
                          </span>
                        </div>
                        <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                          <div>
                            <p className="text-sm text-gray-500">{activity.message}</p>
                            {activity.severity !== 'info' && (
                              <span className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium mt-1 ${
                                activity.severity === 'error' ? 'bg-red-100 text-red-800' :
                                activity.severity === 'warning' ? 'bg-yellow-100 text-yellow-800' :
                                'bg-green-100 text-green-800'
                              }`}>
                                {activity.severity}
                              </span>
                            )}
                          </div>
                          <div className="text-right text-sm whitespace-nowrap text-gray-500">
                            {new Date(activity.timestamp).toLocaleTimeString()}
                          </div>
                        </div>
                      </div>
                    </div>
                  </li>
                ))}
              </ul>
            ) : (
              <div className="text-center py-6">
                <p className="text-sm text-gray-500">No recent activity</p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}
