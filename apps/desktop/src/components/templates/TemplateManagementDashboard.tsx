import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import {
  DocumentTextIcon,
  PlusIcon,
  MagnifyingGlassIcon,
  FunnelIcon,
  StarIcon,
  PlayIcon,
  EyeIcon,
  PencilIcon,
  TrashIcon,
  ChartBarIcon,
  UserIcon,
  TagIcon,
  CalendarIcon,
  ClockIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon
} from '@heroicons/react/24/outline'
import { StarIcon as StarIconSolid } from '@heroicons/react/24/solid'
import {
  ResearchTemplate,
  TemplateParameter,
  TemplateStep,
  ParameterValidation
} from '@/types/api'
import LoadingSpinner from '@/components/common/LoadingSpinner'
import ErrorAlert from '@/components/common/ErrorAlert'

interface TemplateParameter {
  id: string
  name: string
  description: string
  parameter_type: 'string' | 'number' | 'boolean' | 'select' | 'multiselect' | 'date' | 'url' | 'email' | 'text'
  required: boolean
  default_value?: any
  options?: string[]
  placeholder?: string
  help_text?: string
  order: number
}

interface TemplateStep {
  id: string
  name: string
  description: string
  service_provider?: string
  endpoint?: string
  depends_on: string[]
  input_template: Record<string, string>
  output_mapping: Record<string, string>
  conditional?: string
  retry_count: number
  timeout_ms: number
  order: number
}

interface TemplateStatistics {
  total: number
  public: number
  featured: number
  categories: number
  total_usage: number
  average_rating: number
  total_executions: number
  average_success_rate: number
}

interface TemplateExecutionContext {
  template_id: string
  parameters: Record<string, any>
  workflow_name: string
  created_by: string
  execution_metadata: Record<string, any>
}

interface TemplateCategory {
  id: string
  name: string
  description: string
  count: number
  icon: string
}

export default function TemplateManagementDashboard() {
  const [templates, setTemplates] = useState<ResearchTemplate[]>([])
  const [statistics, setStatistics] = useState<TemplateStatistics | null>(null)
  const [recommendations, setRecommendations] = useState<ResearchTemplate[]>([])
  const [loading, setLoading] = useState(true)
  const [selectedCategory, setSelectedCategory] = useState<string>('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [showExecuteModal, setShowExecuteModal] = useState(false)
  const [selectedTemplate, setSelectedTemplate] = useState<ResearchTemplate | null>(null)
  const [executionContext, setExecutionContext] = useState<TemplateExecutionContext>({
    template_id: '',
    parameters: {},
    workflow_name: '',
    created_by: 'user',
    execution_metadata: {}
  })

  useEffect(() => {
    loadDashboardData()
    const interval = setInterval(loadDashboardData, 30000) // Refresh every 30 seconds
    return () => clearInterval(interval)
  }, [selectedCategory, searchQuery])

  const loadDashboardData = async () => {
    try {
      setLoading(true)
      
      // Load templates based on filter
      let templatesData: ResearchTemplate[]
      if (selectedCategory === 'all') {
        if (searchQuery.trim()) {
          templatesData = await invoke<ResearchTemplate[]>('search_research_templates', { query: searchQuery })
        } else {
          templatesData = await invoke<ResearchTemplate[]>('get_all_research_templates')
        }
      } else if (selectedCategory === 'featured') {
        templatesData = await invoke<ResearchTemplate[]>('get_featured_research_templates')
      } else if (selectedCategory === 'public') {
        templatesData = await invoke<ResearchTemplate[]>('get_public_research_templates')
      } else {
        templatesData = await invoke<ResearchTemplate[]>('get_research_templates_by_category', { category: selectedCategory })
      }

      // Load statistics and recommendations
      const [statsData, recommendationsData] = await Promise.all([
        invoke<TemplateStatistics>('get_template_statistics'),
        invoke<ResearchTemplate[]>('get_template_recommendations', { limit: 5 })
      ])

      setTemplates(templatesData)
      setStatistics(statsData)
      setRecommendations(recommendationsData)
    } catch (error) {
      console.error('Failed to load dashboard data:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleExecuteTemplate = (template: ResearchTemplate) => {
    setSelectedTemplate(template)
    setExecutionContext({
      template_id: template.id,
      parameters: {},
      workflow_name: `${template.name} - ${new Date().toLocaleDateString()}`,
      created_by: 'user',
      execution_metadata: {}
    })
    setShowExecuteModal(true)
  }

  const handleConfirmExecution = async () => {
    try {
      if (!selectedTemplate) return

      // Validate required parameters
      const missingParams = selectedTemplate.parameters
        .filter(p => p.required && !executionContext.parameters[p.id])
        .map(p => p.name)

      if (missingParams.length > 0) {
        alert(`Please fill in required parameters: ${missingParams.join(', ')}`)
        return
      }

      const workflow = await invoke('execute_research_template', { context: executionContext })
      
      setShowExecuteModal(false)
      setSelectedTemplate(null)
      alert(`Workflow "${executionContext.workflow_name}" created successfully!`)
      
      // Refresh data to update usage counts
      loadDashboardData()
    } catch (error) {
      console.error('Failed to execute template:', error)
      alert('Failed to execute template: ' + error)
    }
  }

  const handleRateTemplate = async (templateId: string, rating: number) => {
    try {
      await invoke('rate_research_template', { templateId, rating })
      loadDashboardData()
      alert('Template rated successfully!')
    } catch (error) {
      console.error('Failed to rate template:', error)
      alert('Failed to rate template: ' + error)
    }
  }

  const getCategoryColor = (category: string) => {
    switch (category) {
      case 'academic': return 'text-blue-600 bg-blue-100'
      case 'business': return 'text-green-600 bg-green-100'
      case 'technical': return 'text-purple-600 bg-purple-100'
      case 'market': return 'text-orange-600 bg-orange-100'
      case 'competitive': return 'text-red-600 bg-red-100'
      case 'scientific': return 'text-indigo-600 bg-indigo-100'
      case 'legal': return 'text-gray-600 bg-gray-100'
      case 'medical': return 'text-pink-600 bg-pink-100'
      case 'financial': return 'text-yellow-600 bg-yellow-100'
      case 'custom': return 'text-teal-600 bg-teal-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const getMethodologyColor = (methodology: string) => {
    switch (methodology) {
      case 'don_lim': return 'text-blue-600 bg-blue-100'
      case 'nick_scamara': return 'text-green-600 bg-green-100'
      case 'hybrid': return 'text-purple-600 bg-purple-100'
      case 'custom': return 'text-gray-600 bg-gray-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const renderStars = (rating: number, onRate?: (rating: number) => void) => {
    return (
      <div className="flex items-center">
        {[1, 2, 3, 4, 5].map((star) => (
          <button
            key={star}
            onClick={() => onRate && onRate(star)}
            className={`text-lg ${star <= rating ? 'text-yellow-400' : 'text-gray-300'} ${onRate ? 'hover:text-yellow-500 cursor-pointer' : ''}`}
            disabled={!onRate}
          >
            ★
          </button>
        ))}
        <span className="ml-2 text-sm text-gray-600">({rating.toFixed(1)})</span>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Template Management Dashboard</h1>
          <p className="mt-1 text-sm text-gray-500">
            Manage research templates and create standardized workflows
          </p>
        </div>
        
        <div className="flex space-x-3">
          <button
            onClick={loadDashboardData}
            className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
          >
            Refresh
          </button>
        </div>
      </div>

      {/* Search and Filters */}
      <div className="bg-white p-4 rounded-lg shadow">
        <div className="flex space-x-4">
          <div className="flex-1">
            <input
              type="text"
              placeholder="Search templates..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full border border-gray-300 rounded-md px-3 py-2"
            />
          </div>
          <select
            value={selectedCategory}
            onChange={(e) => setSelectedCategory(e.target.value)}
            className="border border-gray-300 rounded-md px-3 py-2"
          >
            <option value="all">All Templates</option>
            <option value="featured">Featured</option>
            <option value="public">Public</option>
            <option value="academic">Academic</option>
            <option value="business">Business</option>
            <option value="technical">Technical</option>
            <option value="market">Market</option>
            <option value="competitive">Competitive</option>
            <option value="scientific">Scientific</option>
            <option value="legal">Legal</option>
            <option value="medical">Medical</option>
            <option value="financial">Financial</option>
            <option value="custom">Custom</option>
          </select>
        </div>
      </div>

      {/* Statistics Cards */}
      {statistics && (
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-sm font-medium">{statistics.total}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Total Templates</dt>
                    <dd className="text-lg font-medium text-gray-900">{statistics.total}</dd>
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
                    <span className="text-white text-sm font-medium">{statistics.public}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Public</dt>
                    <dd className="text-lg font-medium text-gray-900">{statistics.public}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-sm font-medium">{statistics.total_usage}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Total Usage</dt>
                    <dd className="text-lg font-medium text-gray-900">{statistics.total_usage}</dd>
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
                    <span className="text-white text-sm font-medium">★</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Avg Rating</dt>
                    <dd className="text-lg font-medium text-gray-900">{statistics.average_rating.toFixed(1)}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Recommendations Section */}
      {recommendations.length > 0 && (
        <div className="bg-white shadow rounded-lg overflow-hidden">
          <div className="px-6 py-4 border-b border-gray-200">
            <h3 className="text-lg font-medium text-gray-900">Recommended Templates</h3>
          </div>
          <div className="p-6">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {recommendations.map((template) => (
                <div key={template.id} className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
                  <div className="flex justify-between items-start mb-2">
                    <h4 className="font-medium text-gray-900 truncate">{template.name}</h4>
                    {template.is_featured && <span className="text-yellow-500">⭐</span>}
                  </div>
                  <p className="text-sm text-gray-600 mb-3 line-clamp-2">{template.description}</p>
                  <div className="flex justify-between items-center">
                    <div className="flex space-x-2">
                      <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getCategoryColor(template.category)}`}>
                        {template.category}
                      </span>
                      <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getMethodologyColor(template.methodology)}`}>
                        {template.methodology.replace('_', ' ')}
                      </span>
                    </div>
                    <button
                      onClick={() => handleExecuteTemplate(template)}
                      className="text-indigo-600 hover:text-indigo-900 text-sm font-medium"
                    >
                      Use Template
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Templates Table */}
      <div className="bg-white shadow rounded-lg overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">
            Research Templates ({templates.length})
          </h3>
        </div>

        {loading ? (
          <div className="p-6 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600 mx-auto"></div>
            <p className="mt-2 text-gray-500">Loading templates...</p>
          </div>
        ) : templates.length === 0 ? (
          <div className="p-6 text-center">
            <p className="text-gray-500">No templates found.</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Template
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Category
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Methodology
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Rating
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Usage
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Author
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {templates.map((template) => (
                  <tr key={template.id}>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div>
                          <div className="flex items-center">
                            <div className="text-sm font-medium text-gray-900">
                              {template.name}
                            </div>
                            {template.is_featured && <span className="ml-2 text-yellow-500">⭐</span>}
                            {template.is_public && <span className="ml-1 text-green-500">🌐</span>}
                          </div>
                          <div className="text-sm text-gray-500 max-w-xs truncate">
                            {template.description}
                          </div>
                          <div className="flex space-x-1 mt-1">
                            {template.tags.slice(0, 3).map((tag) => (
                              <span key={tag} className="inline-flex px-2 py-1 text-xs text-gray-600 bg-gray-100 rounded">
                                {tag}
                              </span>
                            ))}
                            {template.tags.length > 3 && (
                              <span className="text-xs text-gray-500">+{template.tags.length - 3} more</span>
                            )}
                          </div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getCategoryColor(template.category)}`}>
                        {template.category}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getMethodologyColor(template.methodology)}`}>
                        {template.methodology.replace('_', ' ')}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      {renderStars(template.rating, (rating) => handleRateTemplate(template.id, rating))}
                      <div className="text-xs text-gray-500">
                        {template.rating_count} reviews
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {template.usage_count} times
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {template.author}
                      {template.organization && (
                        <div className="text-xs text-gray-400">{template.organization}</div>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      <div className="flex justify-end space-x-2">
                        <button
                          onClick={() => handleExecuteTemplate(template)}
                          className="text-indigo-600 hover:text-indigo-900"
                          title="Execute Template"
                        >
                          ▶️
                        </button>
                        <button
                          onClick={() => {/* TODO: Implement template details view */}}
                          className="text-blue-600 hover:text-blue-900"
                          title="View Details"
                        >
                          👁️
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Execute Template Modal */}
      {showExecuteModal && selectedTemplate && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-4/5 max-w-4xl shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-medium text-gray-900">
                  Execute Template: {selectedTemplate.name}
                </h3>
                <button
                  onClick={() => {
                    setShowExecuteModal(false)
                    setSelectedTemplate(null)
                  }}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>

              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700">Workflow Name</label>
                  <input
                    type="text"
                    value={executionContext.workflow_name}
                    onChange={(e) => setExecutionContext({...executionContext, workflow_name: e.target.value})}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    placeholder="Enter workflow name"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Created By</label>
                  <input
                    type="text"
                    value={executionContext.created_by}
                    onChange={(e) => setExecutionContext({...executionContext, created_by: e.target.value})}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    placeholder="Your name or identifier"
                  />
                </div>

                {/* Template Parameters */}
                {selectedTemplate.parameters.length > 0 && (
                  <div>
                    <h4 className="text-md font-medium text-gray-900 mb-3">Template Parameters</h4>
                    <div className="space-y-3 max-h-64 overflow-y-auto">
                      {selectedTemplate.parameters
                        .sort((a, b) => a.order - b.order)
                        .map((param) => (
                          <div key={param.id}>
                            <label className="block text-sm font-medium text-gray-700">
                              {param.name}
                              {param.required && <span className="text-red-500 ml-1">*</span>}
                            </label>
                            <p className="text-xs text-gray-500 mb-1">{param.description}</p>

                            {param.parameter_type === 'string' || param.parameter_type === 'url' || param.parameter_type === 'email' ? (
                              <input
                                type={param.parameter_type === 'email' ? 'email' : param.parameter_type === 'url' ? 'url' : 'text'}
                                value={executionContext.parameters[param.id] || ''}
                                onChange={(e) => setExecutionContext({
                                  ...executionContext,
                                  parameters: {...executionContext.parameters, [param.id]: e.target.value}
                                })}
                                className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                                placeholder={param.placeholder}
                                required={param.required}
                              />
                            ) : param.parameter_type === 'text' ? (
                              <textarea
                                value={executionContext.parameters[param.id] || ''}
                                onChange={(e) => setExecutionContext({
                                  ...executionContext,
                                  parameters: {...executionContext.parameters, [param.id]: e.target.value}
                                })}
                                className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                                rows={3}
                                placeholder={param.placeholder}
                                required={param.required}
                              />
                            ) : param.parameter_type === 'number' ? (
                              <input
                                type="number"
                                value={executionContext.parameters[param.id] || ''}
                                onChange={(e) => setExecutionContext({
                                  ...executionContext,
                                  parameters: {...executionContext.parameters, [param.id]: parseFloat(e.target.value)}
                                })}
                                className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                                required={param.required}
                              />
                            ) : param.parameter_type === 'boolean' ? (
                              <div className="mt-1">
                                <label className="flex items-center">
                                  <input
                                    type="checkbox"
                                    checked={executionContext.parameters[param.id] || false}
                                    onChange={(e) => setExecutionContext({
                                      ...executionContext,
                                      parameters: {...executionContext.parameters, [param.id]: e.target.checked}
                                    })}
                                    className="mr-2"
                                  />
                                  <span className="text-sm text-gray-700">{param.help_text || 'Enable this option'}</span>
                                </label>
                              </div>
                            ) : param.parameter_type === 'select' && param.options ? (
                              <select
                                value={executionContext.parameters[param.id] || ''}
                                onChange={(e) => setExecutionContext({
                                  ...executionContext,
                                  parameters: {...executionContext.parameters, [param.id]: e.target.value}
                                })}
                                className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                                required={param.required}
                              >
                                <option value="">Select an option</option>
                                {param.options.map((option) => (
                                  <option key={option} value={option}>{option}</option>
                                ))}
                              </select>
                            ) : null}

                            {param.help_text && param.parameter_type !== 'boolean' && (
                              <p className="text-xs text-gray-400 mt-1">{param.help_text}</p>
                            )}
                          </div>
                        ))}
                    </div>
                  </div>
                )}
              </div>

              <div className="flex justify-end space-x-3 pt-4">
                <button
                  onClick={() => {
                    setShowExecuteModal(false)
                    setSelectedTemplate(null)
                  }}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
                >
                  Cancel
                </button>
                <button
                  onClick={handleConfirmExecution}
                  className="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-md hover:bg-indigo-700"
                >
                  Execute Template
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
