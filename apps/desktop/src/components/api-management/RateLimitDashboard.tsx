import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface UsageStatus {
  current_usage: number
  limit: number
  usage_percentage: number
  remaining_requests: number
  reset_time: string
  time_until_reset: { secs: number, nanos: number }
  status: 'Safe' | 'Warning' | 'Emergency' | 'Exhausted' | 'Blocked'
}

interface RateLimitAlert {
  id: string
  api_key_id: string
  service: string
  alert_type: 'Warning' | 'Emergency' | 'Exhausted' | 'Violation' | 'Reset'
  message: string
  usage_percentage: number
  current_usage: number
  limit: number
  timestamp: string
}

interface UsageForecast {
  api_key_id: string
  service: string
  current_usage: number
  predicted_usage_24h: number
  predicted_usage_7d: number
  predicted_exhaustion_time?: string
  confidence_level: number
  recommendations: string[]
}

export default function RateLimitDashboard() {
  const [analytics, setAnalytics] = useState<Array<[string, UsageStatus, UsageForecast]>>([])
  const [alerts, setAlerts] = useState<RateLimitAlert[]>([])
  const [emergencyStop, setEmergencyStop] = useState(false)
  const [loading, setLoading] = useState(true)
  const [usageReport, setUsageReport] = useState<string>('')
  const [showReport, setShowReport] = useState(false)

  useEffect(() => {
    loadDashboardData()
    const interval = setInterval(loadDashboardData, 30000) // Refresh every 30 seconds
    return () => clearInterval(interval)
  }, [])

  const loadDashboardData = async () => {
    try {
      setLoading(true)
      
      // Load analytics, alerts, and emergency stop status
      const [analyticsData, alertsData, emergencyStopStatus] = await Promise.all([
        invoke<Array<[string, UsageStatus, UsageForecast]>>('get_usage_analytics'),
        invoke<RateLimitAlert[]>('get_recent_alerts', { limit: 20 }),
        invoke<boolean>('is_emergency_stop_enabled')
      ])

      setAnalytics(analyticsData)
      setAlerts(alertsData)
      setEmergencyStop(emergencyStopStatus)
    } catch (error) {
      console.error('Failed to load dashboard data:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleEmergencyStop = async (enabled: boolean) => {
    try {
      await invoke('set_emergency_stop', { enabled })
      setEmergencyStop(enabled)
      loadDashboardData() // Refresh data
    } catch (error) {
      console.error('Failed to set emergency stop:', error)
      alert('Failed to set emergency stop: ' + error)
    }
  }

  const handleCheckThresholds = async () => {
    try {
      const newAlerts = await invoke<RateLimitAlert[]>('check_all_thresholds')
      if (newAlerts.length > 0) {
        alert(`Generated ${newAlerts.length} new threshold alerts`)
        loadDashboardData()
      } else {
        alert('No threshold violations detected')
      }
    } catch (error) {
      console.error('Failed to check thresholds:', error)
      alert('Failed to check thresholds: ' + error)
    }
  }

  const handleGenerateReport = async () => {
    try {
      const report = await invoke<string>('generate_usage_report')
      setUsageReport(report)
      setShowReport(true)
    } catch (error) {
      console.error('Failed to generate report:', error)
      alert('Failed to generate report: ' + error)
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Safe': return 'text-green-600 bg-green-100'
      case 'Warning': return 'text-yellow-600 bg-yellow-100'
      case 'Emergency': return 'text-orange-600 bg-orange-100'
      case 'Exhausted': return 'text-red-600 bg-red-100'
      case 'Blocked': return 'text-gray-600 bg-gray-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const getAlertColor = (alertType: string) => {
    switch (alertType) {
      case 'Warning': return 'text-yellow-600 bg-yellow-100'
      case 'Emergency': return 'text-orange-600 bg-orange-100'
      case 'Exhausted': return 'text-red-600 bg-red-100'
      case 'Violation': return 'text-red-600 bg-red-100'
      case 'Reset': return 'text-blue-600 bg-blue-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const formatTimeUntilReset = (timeUntilReset: { secs: number, nanos: number }) => {
    const totalSeconds = timeUntilReset.secs
    const hours = Math.floor(totalSeconds / 3600)
    const minutes = Math.floor((totalSeconds % 3600) / 60)
    
    if (hours > 24) {
      const days = Math.floor(hours / 24)
      return `${days}d ${hours % 24}h`
    } else if (hours > 0) {
      return `${hours}h ${minutes}m`
    } else {
      return `${minutes}m`
    }
  }

  const getUsageBarColor = (percentage: number) => {
    if (percentage >= 90) return 'bg-red-500'
    if (percentage >= 70) return 'bg-orange-500'
    if (percentage >= 50) return 'bg-yellow-500'
    return 'bg-green-500'
  }

  // Calculate summary statistics
  const totalKeys = analytics.length
  const safeKeys = analytics.filter(([_, status]) => status.status === 'Safe').length
  const warningKeys = analytics.filter(([_, status]) => status.status === 'Warning').length
  const emergencyKeys = analytics.filter(([_, status]) => status.status === 'Emergency').length
  const exhaustedKeys = analytics.filter(([_, status]) => status.status === 'Exhausted').length

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Rate Limit Dashboard</h1>
          <p className="mt-1 text-sm text-gray-500">
            Monitor API usage and rate limiting across all services
          </p>
        </div>
        
        <div className="flex space-x-3">
          <button
            onClick={handleCheckThresholds}
            className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
          >
            Check Thresholds
          </button>
          <button
            onClick={handleGenerateReport}
            className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
          >
            Generate Report
          </button>
          <button
            onClick={() => handleEmergencyStop(!emergencyStop)}
            className={`px-4 py-2 rounded-md font-medium ${
              emergencyStop 
                ? 'bg-red-600 text-white hover:bg-red-700' 
                : 'bg-gray-600 text-white hover:bg-gray-700'
            }`}
          >
            {emergencyStop ? 'Disable Emergency Stop' : 'Enable Emergency Stop'}
          </button>
        </div>
      </div>

      {/* Emergency Stop Alert */}
      {emergencyStop && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <svg className="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
              </svg>
            </div>
            <div className="ml-3">
              <h3 className="text-sm font-medium">Emergency Stop Enabled</h3>
              <p className="text-sm">All API requests are currently blocked.</p>
            </div>
          </div>
        </div>
      )}

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
                  <span className="text-white text-sm font-medium">{totalKeys}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Total Keys</dt>
                  <dd className="text-lg font-medium text-gray-900">{totalKeys}</dd>
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
                  <span className="text-white text-sm font-medium">{safeKeys}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Safe</dt>
                  <dd className="text-lg font-medium text-gray-900">{safeKeys}</dd>
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
                  <span className="text-white text-sm font-medium">{warningKeys}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Warning</dt>
                  <dd className="text-lg font-medium text-gray-900">{warningKeys}</dd>
                </dl>
              </div>
            </div>
          </div>
        </div>

        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-orange-500 rounded-full flex items-center justify-center">
                  <span className="text-white text-sm font-medium">{emergencyKeys}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Emergency</dt>
                  <dd className="text-lg font-medium text-gray-900">{emergencyKeys}</dd>
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
                  <span className="text-white text-sm font-medium">{exhaustedKeys}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Exhausted</dt>
                  <dd className="text-lg font-medium text-gray-900">{exhaustedKeys}</dd>
                </dl>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* API Keys Usage Table */}
      <div className="bg-white shadow rounded-lg overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">API Keys Usage Status</h3>
        </div>

        {loading ? (
          <div className="p-6 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600 mx-auto"></div>
            <p className="mt-2 text-gray-500">Loading usage data...</p>
          </div>
        ) : analytics.length === 0 ? (
          <div className="p-6 text-center">
            <p className="text-gray-500">No API keys found.</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Service
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Usage
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Reset In
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Forecast (24h)
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Confidence
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {analytics.map(([keyId, status, forecast]) => (
                  <tr key={keyId}>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm font-medium text-gray-900 capitalize">
                        {forecast.service.toLowerCase()}
                      </div>
                      <div className="text-xs text-gray-500">{keyId.slice(0, 8)}...</div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {status.current_usage} / {status.limit}
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                        <div
                          className={`h-2 rounded-full ${getUsageBarColor(status.usage_percentage)}`}
                          style={{ width: `${Math.min(status.usage_percentage, 100)}%` }}
                        ></div>
                      </div>
                      <div className="text-xs text-gray-500 mt-1">
                        {status.usage_percentage.toFixed(1)}%
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(status.status)}`}>
                        {status.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {formatTimeUntilReset(status.time_until_reset)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {forecast.predicted_usage_24h} requests
                      {forecast.predicted_exhaustion_time && (
                        <div className="text-xs text-red-500">
                          May exhaust soon
                        </div>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {(forecast.confidence_level * 100).toFixed(0)}%
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Recent Alerts */}
      <div className="bg-white shadow rounded-lg overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">Recent Alerts ({alerts.length})</h3>
        </div>

        {alerts.length === 0 ? (
          <div className="p-6 text-center">
            <p className="text-gray-500">No recent alerts.</p>
          </div>
        ) : (
          <div className="divide-y divide-gray-200">
            {alerts.slice(0, 10).map((alert) => (
              <div key={alert.id} className="p-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getAlertColor(alert.alert_type)}`}>
                      {alert.alert_type}
                    </span>
                    <div>
                      <p className="text-sm font-medium text-gray-900">{alert.message}</p>
                      <p className="text-xs text-gray-500">
                        Service: {alert.service} • Usage: {alert.usage_percentage.toFixed(1)}%
                      </p>
                    </div>
                  </div>
                  <div className="text-xs text-gray-500">
                    {new Date(alert.timestamp).toLocaleString()}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Usage Report Modal */}
      {showReport && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-4/5 max-w-4xl shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-medium text-gray-900">Usage Report</h3>
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
                <pre className="text-sm text-gray-700 whitespace-pre-wrap">{usageReport}</pre>
              </div>

              <div className="flex justify-end mt-4">
                <button
                  onClick={() => {
                    const blob = new Blob([usageReport], { type: 'text/plain' })
                    const url = URL.createObjectURL(blob)
                    const a = document.createElement('a')
                    a.href = url
                    a.download = `usage-report-${new Date().toISOString().split('T')[0]}.txt`
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
