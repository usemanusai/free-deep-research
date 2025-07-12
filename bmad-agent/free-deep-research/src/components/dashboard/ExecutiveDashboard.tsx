import React from 'react'
import { 
  KeyIcon, 
  BeakerIcon, 
  ChartBarIcon, 
  CheckCircleIcon,
  ExclamationTriangleIcon,
  ClockIcon
} from '@heroicons/react/24/outline'
import { useQuery } from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/tauri'

// Mock data for development
const mockStats = {
  totalApiKeys: 8,
  activeApiKeys: 6,
  totalResearch: 24,
  activeResearch: 2,
  systemHealth: 'healthy' as const,
  uptime: '2h 34m',
}

const mockRecentActivity = [
  { id: 1, type: 'research', message: 'Research workflow "AI Market Analysis" completed', time: '2 minutes ago' },
  { id: 2, type: 'api', message: 'OpenRouter API key rotated successfully', time: '15 minutes ago' },
  { id: 3, type: 'system', message: 'Automatic backup completed', time: '30 minutes ago' },
  { id: 4, type: 'research', message: 'New research workflow "Blockchain Trends" started', time: '1 hour ago' },
]

export default function ExecutiveDashboard() {
  // In a real implementation, these would fetch from the backend
  const { data: healthStatus } = useQuery({
    queryKey: ['health'],
    queryFn: () => invoke<string>('health_check'),
    refetchInterval: 30000, // Refetch every 30 seconds
  })

  const stats = mockStats // Replace with real data from backend

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Executive Dashboard</h1>
        <p className="mt-1 text-sm text-gray-500">
          Overview of your Free Deep Research System
        </p>
      </div>

      {/* Stats Grid */}
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
                    {stats.activeApiKeys} / {stats.totalApiKeys}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm">
              <span className="text-green-600 font-medium">{stats.activeApiKeys} active</span>
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
                  <dd className="text-lg font-medium text-gray-900">{stats.totalResearch}</dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm">
              <span className="text-blue-600 font-medium">{stats.activeResearch} running</span>
            </div>
          </div>
        </div>

        {/* System Health */}
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                {stats.systemHealth === 'healthy' ? (
                  <CheckCircleIcon className="h-6 w-6 text-green-400" />
                ) : (
                  <ExclamationTriangleIcon className="h-6 w-6 text-yellow-400" />
                )}
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">System Health</dt>
                  <dd className="text-lg font-medium text-gray-900 capitalize">
                    {stats.systemHealth}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm">
              <span className="text-gray-600">All services operational</span>
            </div>
          </div>
        </div>

        {/* Uptime */}
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <ClockIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Uptime</dt>
                  <dd className="text-lg font-medium text-gray-900">{stats.uptime}</dd>
                </dl>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-5 py-3">
            <div className="text-sm">
              <span className="text-gray-600">Since last restart</span>
            </div>
          </div>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-4 py-5 sm:p-6">
          <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">Quick Actions</h3>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <button className="btn btn-primary">
              <BeakerIcon className="h-4 w-4 mr-2" />
              Start Research
            </button>
            <button className="btn btn-secondary">
              <KeyIcon className="h-4 w-4 mr-2" />
              Manage API Keys
            </button>
            <button className="btn btn-secondary">
              <ChartBarIcon className="h-4 w-4 mr-2" />
              View Analytics
            </button>
            <button className="btn btn-secondary">
              Export Data
            </button>
          </div>
        </div>
      </div>

      {/* Recent Activity */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-4 py-5 sm:p-6">
          <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">Recent Activity</h3>
          <div className="flow-root">
            <ul className="-mb-8">
              {mockRecentActivity.map((activity, activityIdx) => (
                <li key={activity.id}>
                  <div className="relative pb-8">
                    {activityIdx !== mockRecentActivity.length - 1 ? (
                      <span
                        className="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200"
                        aria-hidden="true"
                      />
                    ) : null}
                    <div className="relative flex space-x-3">
                      <div>
                        <span className={`h-8 w-8 rounded-full flex items-center justify-center ring-8 ring-white ${
                          activity.type === 'research' ? 'bg-blue-500' :
                          activity.type === 'api' ? 'bg-green-500' : 'bg-gray-500'
                        }`}>
                          {activity.type === 'research' ? (
                            <BeakerIcon className="h-4 w-4 text-white" />
                          ) : activity.type === 'api' ? (
                            <KeyIcon className="h-4 w-4 text-white" />
                          ) : (
                            <ChartBarIcon className="h-4 w-4 text-white" />
                          )}
                        </span>
                      </div>
                      <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                        <div>
                          <p className="text-sm text-gray-500">{activity.message}</p>
                        </div>
                        <div className="text-right text-sm whitespace-nowrap text-gray-500">
                          {activity.time}
                        </div>
                      </div>
                    </div>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        </div>
      </div>
    </div>
  )
}
