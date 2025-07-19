import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface KeyPerformanceMetrics {
  api_key_id: string
  service: string
  total_requests: number
  successful_requests: number
  failed_requests: number
  success_rate: number
  average_response_time_ms: number
  last_success?: string
  last_failure?: string
  consecutive_failures: number
  health_status: 'Healthy' | 'Degraded' | 'Unhealthy' | 'Failed' | 'Cooldown'
  priority_score: number
  last_used?: string
  cooldown_until?: string
}

interface RotationAnalytics {
  total_rotations: number
  successful_rotations: number
  failed_rotations: number
  average_rotation_time_ms: number
  keys_in_cooldown: number
  keys_healthy: number
  keys_degraded: number
  keys_unhealthy: number
  keys_failed: number
  last_rotation?: string
  rotation_frequency_per_hour: number
}

export default function KeyRotationDashboard() {
  const [performanceMetrics, setPerformanceMetrics] = useState<Array<[string, KeyPerformanceMetrics]>>([])
  const [rotationAnalytics, setRotationAnalytics] = useState<RotationAnalytics | null>(null)
  const [keysNeedingAttention, setKeysNeedingAttention] = useState<Array<[string, KeyPerformanceMetrics]>>([])
  const [loading, setLoading] = useState(true)
  const [rotationReport, setRotationReport] = useState<string>('')
  const [showReport, setShowReport] = useState(false)
  const [selectedService, setSelectedService] = useState<string>('all')

  useEffect(() => {
    loadDashboardData()
    const interval = setInterval(loadDashboardData, 30000) // Refresh every 30 seconds
    return () => clearInterval(interval)
  }, [])

  const loadDashboardData = async () => {
    try {
      setLoading(true)
      
      // Load performance metrics, analytics, and keys needing attention
      const [metricsData, analyticsData, attentionData] = await Promise.all([
        invoke<Array<[string, KeyPerformanceMetrics]>>('get_all_performance_metrics'),
        invoke<RotationAnalytics>('get_rotation_analytics'),
        invoke<Array<[string, KeyPerformanceMetrics]>>('get_keys_needing_attention')
      ])

      setPerformanceMetrics(metricsData)
      setRotationAnalytics(analyticsData)
      setKeysNeedingAttention(attentionData)
    } catch (error) {
      console.error('Failed to load dashboard data:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleHealthCheck = async () => {
    try {
      const healthStatus = await invoke<Array<[string, string]>>('perform_health_check')
      alert(`Health check completed for ${healthStatus.length} keys`)
      loadDashboardData()
    } catch (error) {
      console.error('Failed to perform health check:', error)
      alert('Failed to perform health check: ' + error)
    }
  }

  const handleReactivateKeys = async () => {
    try {
      const reactivatedKeys = await invoke<string[]>('reactivate_cooled_down_keys')
      if (reactivatedKeys.length > 0) {
        alert(`Reactivated ${reactivatedKeys.length} keys from cooldown`)
        loadDashboardData()
      } else {
        alert('No keys needed reactivation')
      }
    } catch (error) {
      console.error('Failed to reactivate keys:', error)
      alert('Failed to reactivate keys: ' + error)
    }
  }

  const handleGenerateReport = async () => {
    try {
      const report = await invoke<string>('generate_rotation_report')
      setRotationReport(report)
      setShowReport(true)
    } catch (error) {
      console.error('Failed to generate report:', error)
      alert('Failed to generate report: ' + error)
    }
  }

  const getHealthColor = (health: string) => {
    switch (health) {
      case 'Healthy': return 'text-green-600 bg-green-100'
      case 'Degraded': return 'text-yellow-600 bg-yellow-100'
      case 'Unhealthy': return 'text-orange-600 bg-orange-100'
      case 'Failed': return 'text-red-600 bg-red-100'
      case 'Cooldown': return 'text-gray-600 bg-gray-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const getHealthIcon = (health: string) => {
    switch (health) {
      case 'Healthy': return '✅'
      case 'Degraded': return '⚠️'
      case 'Unhealthy': return '🔶'
      case 'Failed': return '❌'
      case 'Cooldown': return '⏸️'
      default: return '❓'
    }
  }

  const formatDuration = (ms: number) => {
    if (ms < 1000) return `${ms.toFixed(0)}ms`
    if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`
    return `${(ms / 60000).toFixed(1)}m`
  }

  // Filter metrics by selected service
  const filteredMetrics = selectedService === 'all' 
    ? performanceMetrics 
    : performanceMetrics.filter(([_, metrics]) => metrics.service.toLowerCase() === selectedService.toLowerCase())

  // Get unique services for filter
  const services = ['all', ...Array.from(new Set(performanceMetrics.map(([_, metrics]) => metrics.service)))]

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Key Rotation Dashboard</h1>
          <p className="mt-1 text-sm text-gray-500">
            Monitor API key performance, health, and intelligent rotation
          </p>
        </div>
        
        <div className="flex space-x-3">
          <select
            value={selectedService}
            onChange={(e) => setSelectedService(e.target.value)}
            className="border border-gray-300 rounded-md px-3 py-2 text-sm"
          >
            {services.map(service => (
              <option key={service} value={service}>
                {service === 'all' ? 'All Services' : service}
              </option>
            ))}
          </select>
          <button
            onClick={handleHealthCheck}
            className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
          >
            Health Check
          </button>
          <button
            onClick={handleReactivateKeys}
            className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
          >
            Reactivate Keys
          </button>
          <button
            onClick={handleGenerateReport}
            className="bg-purple-600 text-white px-4 py-2 rounded-md hover:bg-purple-700"
          >
            Generate Report
          </button>
        </div>
      </div>

      {/* Analytics Summary Cards */}
      {rotationAnalytics && (
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-sm font-medium">{rotationAnalytics.total_rotations}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Total Rotations</dt>
                    <dd className="text-lg font-medium text-gray-900">{rotationAnalytics.total_rotations}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-green-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-sm font-medium">{rotationAnalytics.keys_healthy}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Healthy Keys</dt>
                    <dd className="text-lg font-medium text-gray-900">{rotationAnalytics.keys_healthy}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-yellow-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-sm font-medium">
                      {rotationAnalytics.keys_degraded + rotationAnalytics.keys_unhealthy}
                    </span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Needs Attention</dt>
                    <dd className="text-lg font-medium text-gray-900">
                      {rotationAnalytics.keys_degraded + rotationAnalytics.keys_unhealthy}
                    </dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-red-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-sm font-medium">
                      {rotationAnalytics.keys_failed + rotationAnalytics.keys_in_cooldown}
                    </span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Failed/Cooldown</dt>
                    <dd className="text-lg font-medium text-gray-900">
                      {rotationAnalytics.keys_failed + rotationAnalytics.keys_in_cooldown}
                    </dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Keys Needing Attention */}
      {keysNeedingAttention.length > 0 && (
        <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <svg className="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
              </svg>
            </div>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-yellow-800">
                {keysNeedingAttention.length} API Keys Need Attention
              </h3>
              <div className="mt-2 text-sm text-yellow-700">
                <ul className="list-disc pl-5 space-y-1">
                  {keysNeedingAttention.slice(0, 5).map(([keyId, metrics]) => (
                    <li key={keyId}>
                      <span className="font-medium">{metrics.service}</span> key 
                      ({keyId.slice(0, 8)}...) - {metrics.health_status}
                      {metrics.consecutive_failures > 0 && ` (${metrics.consecutive_failures} failures)`}
                    </li>
                  ))}
                  {keysNeedingAttention.length > 5 && (
                    <li>...and {keysNeedingAttention.length - 5} more</li>
                  )}
                </ul>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Performance Metrics Table */}
      <div className="bg-white shadow rounded-lg overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">
            Key Performance Metrics ({filteredMetrics.length})
          </h3>
        </div>

        {loading ? (
          <div className="p-6 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600 mx-auto"></div>
            <p className="mt-2 text-gray-500">Loading performance data...</p>
          </div>
        ) : filteredMetrics.length === 0 ? (
          <div className="p-6 text-center">
            <p className="text-gray-500">No performance metrics found.</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Service & Key
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Health
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Success Rate
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Avg Response
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Requests
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Priority Score
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Last Used
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {filteredMetrics
                  .sort((a, b) => b[1].priority_score - a[1].priority_score)
                  .map(([keyId, metrics]) => (
                  <tr key={keyId}>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm font-medium text-gray-900 capitalize">
                        {metrics.service.toLowerCase()}
                      </div>
                      <div className="text-xs text-gray-500 font-mono">{keyId.slice(0, 8)}...</div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <span className="mr-2">{getHealthIcon(metrics.health_status)}</span>
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getHealthColor(metrics.health_status)}`}>
                          {metrics.health_status}
                        </span>
                      </div>
                      {metrics.consecutive_failures > 0 && (
                        <div className="text-xs text-red-500 mt-1">
                          {metrics.consecutive_failures} consecutive failures
                        </div>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {metrics.success_rate.toFixed(1)}%
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                        <div
                          className={`h-2 rounded-full ${
                            metrics.success_rate >= 90 ? 'bg-green-500' :
                            metrics.success_rate >= 70 ? 'bg-yellow-500' : 'bg-red-500'
                          }`}
                          style={{ width: `${metrics.success_rate}%` }}
                        ></div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {formatDuration(metrics.average_response_time_ms)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {metrics.total_requests}
                      </div>
                      <div className="text-xs text-gray-500">
                        {metrics.successful_requests}✅ {metrics.failed_requests}❌
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {metrics.priority_score.toFixed(1)}
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                        <div
                          className="h-2 rounded-full bg-blue-500"
                          style={{ width: `${Math.min(metrics.priority_score, 100)}%` }}
                        ></div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {metrics.last_used
                        ? new Date(metrics.last_used).toLocaleString()
                        : 'Never'
                      }
                      {metrics.cooldown_until && (
                        <div className="text-xs text-red-500 mt-1">
                          Cooldown until {new Date(metrics.cooldown_until).toLocaleString()}
                        </div>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Rotation Report Modal */}
      {showReport && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-4/5 max-w-4xl shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-medium text-gray-900">Key Rotation Report</h3>
                <button
                  onClick={() => setShowReport(false)}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>

              <div className="bg-gray-50 p-4 rounded-md max-h-96 overflow-y-auto">
                <pre className="text-sm text-gray-700 whitespace-pre-wrap">{rotationReport}</pre>
              </div>

              <div className="flex justify-end mt-4">
                <button
                  onClick={() => {
                    const blob = new Blob([rotationReport], { type: 'text/plain' })
                    const url = URL.createObjectURL(blob)
                    const a = document.createElement('a')
                    a.href = url
                    a.download = `rotation-report-${new Date().toISOString().split('T')[0]}.txt`
                    a.click()
                    URL.revokeObjectURL(url)
                  }}
                  className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 mr-3"
                >
                  Download Report
                </button>
                <button
                  onClick={() => setShowReport(false)}
                  className="bg-gray-300 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-400"
                >
                  Close
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
