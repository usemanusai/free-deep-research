import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { toast } from 'react-hot-toast'
import {
  ChartBarIcon,
  CogIcon,
  PlayIcon,
  DocumentTextIcon,
  UserGroupIcon,
  ClockIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  PlusIcon,
  BeakerIcon,
  EyeIcon,
  InformationCircleIcon
} from '@heroicons/react/24/outline'

// Types
interface IntegrationHealthStatus {
  overall_status: string
  research_engine_status: string
  ai_orchestration_status: string
  api_manager_status: string
  integration_enabled: boolean
  active_research_count: number
  error_messages: string[]
}

interface BMadAgent {
  id: string
  name: string
  title: string
  description: string
  research_capabilities: string[]
  available_tasks: string[]
}

interface IntegrationStatistics {
  service_status: string
  active_research_count: number
  integration_enabled: boolean
  error_count: number
  uptime_seconds: number
  total_research_conducted: number
  average_research_duration_minutes: number
  success_rate: number
  last_health_check: string
}

interface DocumentationModeRequest {
  project_description: string
  requirements: string[]
  target_audience: string
  research_depth: string
  cost_limit?: number
  timeline_minutes?: number
}

const BMadIntegrationDashboard: React.FC = () => {
  const [healthStatus, setHealthStatus] = useState<IntegrationHealthStatus | null>(null)
  const [agents, setAgents] = useState<BMadAgent[]>([])
  const [statistics, setStatistics] = useState<IntegrationStatistics | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const [activeTab, setActiveTab] = useState<'overview' | 'agents' | 'documentation' | 'research'>('overview')

  // Documentation Mode Form State
  const [docRequest, setDocRequest] = useState<DocumentationModeRequest>({
    project_description: '',
    requirements: [''],
    target_audience: '',
    research_depth: 'Standard',
    cost_limit: 25,
    timeline_minutes: 45
  })
  const [isGeneratingDocs, setIsGeneratingDocs] = useState(false)

  useEffect(() => {
    loadIntegrationData()
    const interval = setInterval(loadIntegrationData, 30000) // Refresh every 30 seconds
    return () => clearInterval(interval)
  }, [])

  const loadIntegrationData = async () => {
    try {
      const [healthData, agentsData, statsData] = await Promise.all([
        invoke<IntegrationHealthStatus>('get_integration_health_status'),
        invoke<BMadAgent[]>('get_bmad_agents'),
        invoke<IntegrationStatistics>('get_integration_statistics')
      ])

      setHealthStatus(healthData)
      setAgents(agentsData)
      setStatistics(statsData)
    } catch (error) {
      console.error('Failed to load integration data:', error)
      toast.error('Failed to load BMAD integration data')
    } finally {
      setIsLoading(false)
    }
  }

  const testIntegration = async () => {
    try {
      const result = await invoke<any>('test_bmad_integration')
      if (result.test_successful) {
        toast.success('BMAD integration test passed!')
      } else {
        toast.error('BMAD integration test failed')
      }
    } catch (error) {
      console.error('Integration test failed:', error)
      toast.error('Integration test failed')
    }
  }

  const generateDocumentation = async () => {
    if (!docRequest.project_description.trim()) {
      toast.error('Please provide a project description')
      return
    }

    setIsGeneratingDocs(true)
    try {
      const result = await invoke<any>('execute_research_enhanced_documentation_mode', {
        request: {
          ...docRequest,
          requirements: docRequest.requirements.filter(req => req.trim() !== '')
        }
      })

      toast.success('Documentation generated successfully!')
      console.log('Generated documentation:', result)
      
      // Here you could open a modal or navigate to show the results
      // For now, we'll just log it
    } catch (error) {
      console.error('Documentation generation failed:', error)
      toast.error('Failed to generate documentation')
    } finally {
      setIsGeneratingDocs(false)
    }
  }

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'healthy': return 'text-green-600 bg-green-100'
      case 'degraded': return 'text-yellow-600 bg-yellow-100'
      case 'unhealthy': return 'text-red-600 bg-red-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const addRequirement = () => {
    setDocRequest(prev => ({
      ...prev,
      requirements: [...prev.requirements, '']
    }))
  }

  const updateRequirement = (index: number, value: string) => {
    setDocRequest(prev => ({
      ...prev,
      requirements: prev.requirements.map((req, i) => i === index ? value : req)
    }))
  }

  const removeRequirement = (index: number) => {
    setDocRequest(prev => ({
      ...prev,
      requirements: prev.requirements.filter((_, i) => i !== index)
    }))
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        <span className="ml-2 text-gray-600">Loading BMAD integration...</span>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="bg-white shadow rounded-lg p-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">BMAD AI Agent Integration</h1>
            <p className="text-gray-600 mt-1">
              Manage and monitor the BMAD AI Agent Orchestrator integration
            </p>
          </div>
          <div className="flex space-x-3">
            <button
              onClick={testIntegration}
              className="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
            >
              <PlayIcon className="h-4 w-4 mr-2" />
              Test Integration
            </button>
            <button
              onClick={loadIntegrationData}
              className="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700"
            >
              <ChartBarIcon className="h-4 w-4 mr-2" />
              Refresh
            </button>
          </div>
        </div>

        {/* Status Overview */}
        {healthStatus && (
          <div className="mt-6 grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="bg-gray-50 rounded-lg p-4">
              <div className="flex items-center">
                <div className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(healthStatus.overall_status)}`}>
                  {healthStatus.overall_status === 'healthy' ? (
                    <CheckCircleIcon className="h-4 w-4 mr-1" />
                  ) : (
                    <ExclamationTriangleIcon className="h-4 w-4 mr-1" />
                  )}
                  {healthStatus.overall_status}
                </div>
              </div>
              <p className="text-sm text-gray-600 mt-1">Overall Status</p>
            </div>
            
            <div className="bg-gray-50 rounded-lg p-4">
              <div className="text-2xl font-bold text-gray-900">{healthStatus.active_research_count}</div>
              <p className="text-sm text-gray-600">Active Research</p>
            </div>
            
            <div className="bg-gray-50 rounded-lg p-4">
              <div className="text-2xl font-bold text-gray-900">{statistics?.total_research_conducted || 0}</div>
              <p className="text-sm text-gray-600">Total Research</p>
            </div>
            
            <div className="bg-gray-50 rounded-lg p-4">
              <div className="text-2xl font-bold text-gray-900">{statistics?.success_rate.toFixed(1) || 0}%</div>
              <p className="text-sm text-gray-600">Success Rate</p>
            </div>
          </div>
        )}
      </div>

      {/* Tabs */}
      <div className="bg-white shadow rounded-lg">
        <div className="border-b border-gray-200">
          <nav className="-mb-px flex space-x-8 px-6">
            {[
              { id: 'overview', name: 'Overview', icon: ChartBarIcon },
              { id: 'agents', name: 'AI Agents', icon: UserGroupIcon },
              { id: 'documentation', name: 'Documentation Mode', icon: DocumentTextIcon },
              { id: 'research', name: 'Research', icon: CogIcon }
            ].map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`${
                  activeTab === tab.id
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
          {activeTab === 'overview' && (
            <div className="space-y-6">
              {/* Service Status */}
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-4">Service Status</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  {healthStatus && [
                    { name: 'Research Engine', status: healthStatus.research_engine_status },
                    { name: 'AI Orchestration', status: healthStatus.ai_orchestration_status },
                    { name: 'API Manager', status: healthStatus.api_manager_status }
                  ].map((service) => (
                    <div key={service.name} className="border rounded-lg p-4">
                      <div className="flex items-center justify-between">
                        <span className="text-sm font-medium text-gray-900">{service.name}</span>
                        <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(service.status)}`}>
                          {service.status}
                        </span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Error Messages */}
              {healthStatus?.error_messages && healthStatus.error_messages.length > 0 && (
                <div>
                  <h3 className="text-lg font-medium text-gray-900 mb-4">Error Messages</h3>
                  <div className="bg-red-50 border border-red-200 rounded-lg p-4">
                    <ul className="list-disc list-inside space-y-1">
                      {healthStatus.error_messages.map((error, index) => (
                        <li key={index} className="text-sm text-red-700">{error}</li>
                      ))}
                    </ul>
                  </div>
                </div>
              )}
            </div>
          )}

          {activeTab === 'agents' && (
            <div className="space-y-4">
              <h3 className="text-lg font-medium text-gray-900">Available AI Agents</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {agents.map((agent) => (
                  <div key={agent.id} className="border rounded-lg p-6 hover:shadow-md transition-shadow">
                    <div className="flex items-center mb-4">
                      <div className="h-10 w-10 bg-primary-100 rounded-full flex items-center justify-center">
                        <UserGroupIcon className="h-6 w-6 text-primary-600" />
                      </div>
                      <div className="ml-3">
                        <h4 className="text-lg font-medium text-gray-900">{agent.name}</h4>
                        <p className="text-sm text-gray-500">{agent.title}</p>
                      </div>
                    </div>
                    
                    <p className="text-sm text-gray-600 mb-4">{agent.description}</p>
                    
                    <div className="space-y-3">
                      <div>
                        <h5 className="text-sm font-medium text-gray-900 mb-2">Research Capabilities</h5>
                        <div className="flex flex-wrap gap-1">
                          {agent.research_capabilities.map((capability) => (
                            <span key={capability} className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                              {capability}
                            </span>
                          ))}
                        </div>
                      </div>
                      
                      <div>
                        <h5 className="text-sm font-medium text-gray-900 mb-2">Available Tasks</h5>
                        <div className="flex flex-wrap gap-1">
                          {agent.available_tasks.map((task) => (
                            <span key={task} className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                              {task}
                            </span>
                          ))}
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {activeTab === 'documentation' && (
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-4">Research-Enhanced Documentation Mode</h3>
                <p className="text-sm text-gray-600 mb-6">
                  Generate comprehensive documentation with AI-powered research integration. This mode produces 
                  three professional documents: PRD, Architecture, and Development Checklist.
                </p>
              </div>

              <div className="space-y-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Project Description *
                  </label>
                  <textarea
                    value={docRequest.project_description}
                    onChange={(e) => setDocRequest(prev => ({ ...prev, project_description: e.target.value }))}
                    rows={4}
                    className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                    placeholder="Describe your project in detail..."
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Requirements
                  </label>
                  {docRequest.requirements.map((req, index) => (
                    <div key={index} className="flex items-center space-x-2 mb-2">
                      <input
                        type="text"
                        value={req}
                        onChange={(e) => updateRequirement(index, e.target.value)}
                        className="flex-1 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                        placeholder="Enter a requirement..."
                      />
                      {docRequest.requirements.length > 1 && (
                        <button
                          onClick={() => removeRequirement(index)}
                          className="text-red-600 hover:text-red-800"
                        >
                          Remove
                        </button>
                      )}
                    </div>
                  ))}
                  <button
                    onClick={addRequirement}
                    className="text-primary-600 hover:text-primary-800 text-sm"
                  >
                    + Add Requirement
                  </button>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Target Audience
                    </label>
                    <input
                      type="text"
                      value={docRequest.target_audience}
                      onChange={(e) => setDocRequest(prev => ({ ...prev, target_audience: e.target.value }))}
                      className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                      placeholder="e.g., Developers, End Users, Stakeholders"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Research Depth
                    </label>
                    <select
                      value={docRequest.research_depth}
                      onChange={(e) => setDocRequest(prev => ({ ...prev, research_depth: e.target.value }))}
                      className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                    >
                      <option value="Basic">Basic</option>
                      <option value="Standard">Standard</option>
                      <option value="Comprehensive">Comprehensive</option>
                      <option value="Expert">Expert</option>
                    </select>
                  </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Cost Limit ($)
                    </label>
                    <input
                      type="number"
                      value={docRequest.cost_limit}
                      onChange={(e) => setDocRequest(prev => ({ ...prev, cost_limit: Number(e.target.value) }))}
                      className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                      min="1"
                      max="100"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Timeline (minutes)
                    </label>
                    <input
                      type="number"
                      value={docRequest.timeline_minutes}
                      onChange={(e) => setDocRequest(prev => ({ ...prev, timeline_minutes: Number(e.target.value) }))}
                      className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                      min="15"
                      max="120"
                    />
                  </div>
                </div>

                <div className="flex justify-end">
                  <button
                    onClick={generateDocumentation}
                    disabled={isGeneratingDocs || !docRequest.project_description.trim()}
                    className="inline-flex items-center px-6 py-3 border border-transparent rounded-md shadow-sm text-base font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {isGeneratingDocs ? (
                      <>
                        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                        Generating...
                      </>
                    ) : (
                      <>
                        <DocumentTextIcon className="h-5 w-5 mr-2" />
                        Generate Documentation
                      </>
                    )}
                  </button>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'research' && (
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-4">Research Management</h3>
                <p className="text-sm text-gray-600 mb-6">
                  Manage and monitor individual research tasks conducted by AI agents.
                </p>
              </div>

              <div className="space-y-6">
                <div className="flex justify-between items-center">
                  <h3 className="text-lg font-medium text-gray-900">Research Tasks</h3>
                  <button
                    className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    onClick={() => {
                      // TODO: Implement new task creation
                      console.log('Create new research task');
                    }}
                  >
                    <PlusIcon className="h-4 w-4 mr-2" />
                    New Task
                  </button>
                </div>

                <div className="bg-white shadow overflow-hidden sm:rounded-md">
                  <ul className="divide-y divide-gray-200">
                    {/* Sample research tasks - replace with actual data */}
                    <li className="px-6 py-4">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center">
                          <div className="flex-shrink-0">
                            <BeakerIcon className="h-6 w-6 text-indigo-600" />
                          </div>
                          <div className="ml-4">
                            <div className="text-sm font-medium text-gray-900">
                              Market Analysis: AI Research Tools
                            </div>
                            <div className="text-sm text-gray-500">
                              Comprehensive analysis of AI-powered research platforms
                            </div>
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                            Completed
                          </span>
                          <button className="text-indigo-600 hover:text-indigo-900">
                            <EyeIcon className="h-4 w-4" />
                          </button>
                        </div>
                      </div>
                    </li>

                    <li className="px-6 py-4">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center">
                          <div className="flex-shrink-0">
                            <BeakerIcon className="h-6 w-6 text-blue-600" />
                          </div>
                          <div className="ml-4">
                            <div className="text-sm font-medium text-gray-900">
                              Technical Architecture Review
                            </div>
                            <div className="text-sm text-gray-500">
                              Evaluating scalable research system architectures
                            </div>
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                            In Progress
                          </span>
                          <button className="text-indigo-600 hover:text-indigo-900">
                            <EyeIcon className="h-4 w-4" />
                          </button>
                        </div>
                      </div>
                    </li>

                    <li className="px-6 py-4">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center">
                          <div className="flex-shrink-0">
                            <BeakerIcon className="h-6 w-6 text-yellow-600" />
                          </div>
                          <div className="ml-4">
                            <div className="text-sm font-medium text-gray-900">
                              Competitive Intelligence Analysis
                            </div>
                            <div className="text-sm text-gray-500">
                              Research on competing research automation platforms
                            </div>
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                            Queued
                          </span>
                          <button className="text-indigo-600 hover:text-indigo-900">
                            <EyeIcon className="h-4 w-4" />
                          </button>
                        </div>
                      </div>
                    </li>
                  </ul>
                </div>

                <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
                  <div className="flex">
                    <div className="flex-shrink-0">
                      <InformationCircleIcon className="h-5 w-5 text-blue-400" />
                    </div>
                    <div className="ml-3">
                      <h3 className="text-sm font-medium text-blue-800">
                        Research Task Management
                      </h3>
                      <div className="mt-2 text-sm text-blue-700">
                        <p>
                          Create, monitor, and manage individual research tasks. Each task can be assigned to specific AI agents and tracked through completion.
                        </p>
                      </div>
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

export default BMadIntegrationDashboard
