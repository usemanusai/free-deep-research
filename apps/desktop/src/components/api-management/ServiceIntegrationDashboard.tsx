import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface ServiceMetrics {
  service: string
  total_requests: number
  successful_requests: number
  failed_requests: number
  average_response_time_ms: number
  min_response_time_ms: number
  max_response_time_ms: number
  last_request_time?: string
  health_status: 'Healthy' | 'Degraded' | 'Unhealthy' | 'Down' | 'Unknown'
  uptime_percentage: number
  error_rate: number
  requests_per_minute: number
}

interface ServiceConfig {
  service: string
  base_url: string
  default_timeout_ms: number
  max_retries: number
  retry_delay_ms: number
  health_check_endpoint?: string
  health_check_interval_minutes: number
  rate_limit_per_minute: number
  custom_headers: Record<string, string>
  enabled: boolean
}

export default function ServiceIntegrationDashboard() {
  const [serviceMetrics, setServiceMetrics] = useState<Array<[string, ServiceMetrics]>>([])
  const [serviceConfigs, setServiceConfigs] = useState<Array<[string, ServiceConfig]>>([])
  const [registeredServices, setRegisteredServices] = useState<string[]>([])
  const [loading, setLoading] = useState(true)
  const [statusReport, setStatusReport] = useState<string>('')
  const [showReport, setShowReport] = useState(false)
  const [selectedService, setSelectedService] = useState<string>('')
  const [showConfigModal, setShowConfigModal] = useState(false)
  const [editingConfig, setEditingConfig] = useState<ServiceConfig | null>(null)

  useEffect(() => {
    loadDashboardData()
    const interval = setInterval(loadDashboardData, 30000) // Refresh every 30 seconds
    return () => clearInterval(interval)
  }, [])

  const loadDashboardData = async () => {
    try {
      setLoading(true)
      
      // Load metrics, configs, and registered services
      const [metricsData, configsData, servicesData] = await Promise.all([
        invoke<Array<[string, ServiceMetrics]>>('get_all_service_metrics'),
        invoke<Array<[string, ServiceConfig]>>('get_all_service_configs'),
        invoke<string[]>('get_registered_services')
      ])

      setServiceMetrics(metricsData)
      setServiceConfigs(configsData)
      setRegisteredServices(servicesData)
    } catch (error) {
      console.error('Failed to load dashboard data:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleHealthCheck = async (service: string) => {
    try {
      const health = await invoke<string>('check_service_health', { service })
      alert(`Health check for ${service}: ${health}`)
      loadDashboardData()
    } catch (error) {
      console.error('Failed to perform health check:', error)
      alert('Failed to perform health check: ' + error)
    }
  }

  const handleGenerateReport = async () => {
    try {
      const report = await invoke<string>('generate_service_status_report')
      setStatusReport(report)
      setShowReport(true)
    } catch (error) {
      console.error('Failed to generate report:', error)
      alert('Failed to generate report: ' + error)
    }
  }

  const handleEditConfig = (service: string) => {
    const config = serviceConfigs.find(([s]) => s === service)?.[1]
    if (config) {
      setEditingConfig(config)
      setShowConfigModal(true)
    }
  }

  const handleSaveConfig = async () => {
    if (!editingConfig) return

    try {
      await invoke('update_service_config', { 
        service: editingConfig.service, 
        config: editingConfig 
      })
      setShowConfigModal(false)
      setEditingConfig(null)
      loadDashboardData()
      alert('Configuration updated successfully')
    } catch (error) {
      console.error('Failed to update configuration:', error)
      alert('Failed to update configuration: ' + error)
    }
  }

  const getHealthColor = (health: string) => {
    switch (health) {
      case 'Healthy': return 'text-green-600 bg-green-100'
      case 'Degraded': return 'text-yellow-600 bg-yellow-100'
      case 'Unhealthy': return 'text-orange-600 bg-orange-100'
      case 'Down': return 'text-red-600 bg-red-100'
      case 'Unknown': return 'text-gray-600 bg-gray-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const getHealthIcon = (health: string) => {
    switch (health) {
      case 'Healthy': return '✅'
      case 'Degraded': return '⚠️'
      case 'Unhealthy': return '🔶'
      case 'Down': return '❌'
      case 'Unknown': return '❓'
      default: return '❓'
    }
  }

  const formatDuration = (ms: number) => {
    if (ms < 1000) return `${ms.toFixed(0)}ms`
    if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`
    return `${(ms / 60000).toFixed(1)}m`
  }

  // Calculate summary statistics
  const totalServices = serviceMetrics.length
  const healthyServices = serviceMetrics.filter(([_, metrics]) => metrics.health_status === 'Healthy').length
  const degradedServices = serviceMetrics.filter(([_, metrics]) => metrics.health_status === 'Degraded').length
  const unhealthyServices = serviceMetrics.filter(([_, metrics]) => metrics.health_status === 'Unhealthy').length
  const downServices = serviceMetrics.filter(([_, metrics]) => metrics.health_status === 'Down').length

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Service Integration Dashboard</h1>
          <p className="mt-1 text-sm text-gray-500">
            Monitor external service integrations, health, and performance
          </p>
        </div>
        
        <div className="flex space-x-3">
          <button
            onClick={handleGenerateReport}
            className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
          >
            Generate Report
          </button>
          <button
            onClick={loadDashboardData}
            className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
          >
            Refresh Data
          </button>
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
                  <span className="text-white text-sm font-medium">{totalServices}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Total Services</dt>
                  <dd className="text-lg font-medium text-gray-900">{totalServices}</dd>
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
                  <span className="text-white text-sm font-medium">{healthyServices}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Healthy</dt>
                  <dd className="text-lg font-medium text-gray-900">{healthyServices}</dd>
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
                  <span className="text-white text-sm font-medium">{degradedServices}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Degraded</dt>
                  <dd className="text-lg font-medium text-gray-900">{degradedServices}</dd>
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
                  <span className="text-white text-sm font-medium">{unhealthyServices}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Unhealthy</dt>
                  <dd className="text-lg font-medium text-gray-900">{unhealthyServices}</dd>
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
                  <span className="text-white text-sm font-medium">{downServices}</span>
                </div>
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Down</dt>
                  <dd className="text-lg font-medium text-gray-900">{downServices}</dd>
                </dl>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Service Metrics Table */}
      <div className="bg-white shadow rounded-lg overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">Service Performance Metrics</h3>
        </div>

        {loading ? (
          <div className="p-6 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600 mx-auto"></div>
            <p className="mt-2 text-gray-500">Loading service data...</p>
          </div>
        ) : serviceMetrics.length === 0 ? (
          <div className="p-6 text-center">
            <p className="text-gray-500">No service metrics found.</p>
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
                    Health
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Requests
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Success Rate
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Avg Response
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Uptime
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {serviceMetrics.map(([service, metrics]) => {
                  const successRate = 100 - metrics.error_rate
                  return (
                    <tr key={service}>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="flex items-center">
                          <div className="text-sm font-medium text-gray-900 capitalize">
                            {service}
                          </div>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="flex items-center">
                          <span className="mr-2">{getHealthIcon(metrics.health_status)}</span>
                          <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getHealthColor(metrics.health_status)}`}>
                            {metrics.health_status}
                          </span>
                        </div>
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
                          {successRate.toFixed(1)}%
                        </div>
                        <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                          <div
                            className={`h-2 rounded-full ${
                              successRate >= 95 ? 'bg-green-500' :
                              successRate >= 80 ? 'bg-yellow-500' : 'bg-red-500'
                            }`}
                            style={{ width: `${successRate}%` }}
                          ></div>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {formatDuration(metrics.average_response_time_ms)}
                        <div className="text-xs text-gray-400">
                          {formatDuration(metrics.min_response_time_ms)} - {formatDuration(metrics.max_response_time_ms)}
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="text-sm text-gray-900">
                          {metrics.uptime_percentage.toFixed(1)}%
                        </div>
                        <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                          <div
                            className={`h-2 rounded-full ${
                              metrics.uptime_percentage >= 99 ? 'bg-green-500' :
                              metrics.uptime_percentage >= 95 ? 'bg-yellow-500' : 'bg-red-500'
                            }`}
                            style={{ width: `${metrics.uptime_percentage}%` }}
                          ></div>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                        <button
                          onClick={() => handleHealthCheck(service)}
                          className="text-indigo-600 hover:text-indigo-900 mr-3"
                        >
                          Health Check
                        </button>
                        <button
                          onClick={() => handleEditConfig(service)}
                          className="text-blue-600 hover:text-blue-900"
                        >
                          Configure
                        </button>
                      </td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  )
}
