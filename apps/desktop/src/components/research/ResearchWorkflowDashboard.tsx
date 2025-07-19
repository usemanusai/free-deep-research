import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface ResearchWorkflow {
  id: string
  name: string
  query: string
  status: 'created' | 'pending' | 'running' | 'paused' | 'completed' | 'failed' | 'cancelled'
  progress: number
  methodology: 'don_lim' | 'nick_scamara' | 'hybrid'
  created_at: string
  started_at?: string
  completed_at?: string
  created_by: string
  tags: string[]
}

interface WorkflowStatistics {
  total: number
  active: number
  completed: number
  failed: number
  cancelled: number
  average_execution_time_ms?: number
}

interface CreateWorkflowData {
  name: string
  query: string
  methodology: string
  created_by: string
}

export default function ResearchWorkflowDashboard() {
  const [workflows, setWorkflows] = useState<ResearchWorkflow[]>([])
  const [statistics, setStatistics] = useState<WorkflowStatistics | null>(null)
  const [loading, setLoading] = useState(true)
  const [showCreateModal, setShowCreateModal] = useState(false)
  const [selectedStatus, setSelectedStatus] = useState<string>('all')
  const [createData, setCreateData] = useState<CreateWorkflowData>({
    name: '',
    query: '',
    methodology: 'hybrid',
    created_by: 'user'
  })

  useEffect(() => {
    loadDashboardData()
    const interval = setInterval(loadDashboardData, 5000) // Refresh every 5 seconds
    return () => clearInterval(interval)
  }, [selectedStatus])

  const loadDashboardData = async () => {
    try {
      setLoading(true)
      
      // Load workflows and statistics
      const [workflowsData, statsData] = await Promise.all([
        selectedStatus === 'all' 
          ? invoke<ResearchWorkflow[]>('get_all_research_workflows')
          : invoke<ResearchWorkflow[]>('get_research_workflows_by_status', { status: selectedStatus }),
        invoke<WorkflowStatistics>('get_workflow_statistics')
      ])

      setWorkflows(workflowsData)
      setStatistics(statsData)
    } catch (error) {
      console.error('Failed to load dashboard data:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleCreateWorkflow = async () => {
    try {
      if (!createData.name || !createData.query) {
        alert('Please fill in all required fields')
        return
      }

      const workflow = await invoke<ResearchWorkflow>('create_research_workflow', createData)
      
      setShowCreateModal(false)
      setCreateData({
        name: '',
        query: '',
        methodology: 'hybrid',
        created_by: 'user'
      })
      
      loadDashboardData()
      alert(`Workflow "${workflow.name}" created successfully!`)
    } catch (error) {
      console.error('Failed to create workflow:', error)
      alert('Failed to create workflow: ' + error)
    }
  }

  const handleStartWorkflow = async (workflowId: string) => {
    try {
      await invoke('start_research_workflow', { workflowId })
      loadDashboardData()
      alert('Workflow started successfully!')
    } catch (error) {
      console.error('Failed to start workflow:', error)
      alert('Failed to start workflow: ' + error)
    }
  }

  const handlePauseWorkflow = async (workflowId: string) => {
    try {
      await invoke('pause_research_workflow', { workflowId })
      loadDashboardData()
      alert('Workflow paused successfully!')
    } catch (error) {
      console.error('Failed to pause workflow:', error)
      alert('Failed to pause workflow: ' + error)
    }
  }

  const handleResumeWorkflow = async (workflowId: string) => {
    try {
      await invoke('resume_research_workflow', { workflowId })
      loadDashboardData()
      alert('Workflow resumed successfully!')
    } catch (error) {
      console.error('Failed to resume workflow:', error)
      alert('Failed to resume workflow: ' + error)
    }
  }

  const handleCancelWorkflow = async (workflowId: string) => {
    try {
      if (confirm('Are you sure you want to cancel this workflow?')) {
        await invoke('cancel_research_workflow', { workflowId })
        loadDashboardData()
        alert('Workflow cancelled successfully!')
      }
    } catch (error) {
      console.error('Failed to cancel workflow:', error)
      alert('Failed to cancel workflow: ' + error)
    }
  }

  const handleDeleteWorkflow = async (workflowId: string) => {
    try {
      if (confirm('Are you sure you want to delete this workflow? This action cannot be undone.')) {
        await invoke('delete_research_workflow', { workflowId })
        loadDashboardData()
        alert('Workflow deleted successfully!')
      }
    } catch (error) {
      console.error('Failed to delete workflow:', error)
      alert('Failed to delete workflow: ' + error)
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'completed': return 'text-green-600 bg-green-100'
      case 'running': return 'text-blue-600 bg-blue-100'
      case 'paused': return 'text-yellow-600 bg-yellow-100'
      case 'failed': return 'text-red-600 bg-red-100'
      case 'cancelled': return 'text-gray-600 bg-gray-100'
      case 'created': return 'text-purple-600 bg-purple-100'
      case 'pending': return 'text-indigo-600 bg-indigo-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed': return '✅'
      case 'running': return '🔄'
      case 'paused': return '⏸️'
      case 'failed': return '❌'
      case 'cancelled': return '🚫'
      case 'created': return '📝'
      case 'pending': return '⏳'
      default: return '❓'
    }
  }

  const getMethodologyColor = (methodology: string) => {
    switch (methodology) {
      case 'don_lim': return 'text-blue-600 bg-blue-100'
      case 'nick_scamara': return 'text-green-600 bg-green-100'
      case 'hybrid': return 'text-purple-600 bg-purple-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const formatDuration = (ms?: number) => {
    if (!ms) return 'N/A'
    if (ms < 1000) return `${ms}ms`
    if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`
    if (ms < 3600000) return `${(ms / 60000).toFixed(1)}m`
    return `${(ms / 3600000).toFixed(1)}h`
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Research Workflow Dashboard</h1>
          <p className="mt-1 text-sm text-gray-500">
            Manage and monitor research workflows with intelligent orchestration
          </p>
        </div>
        
        <div className="flex space-x-3">
          <select
            value={selectedStatus}
            onChange={(e) => setSelectedStatus(e.target.value)}
            className="border border-gray-300 rounded-md px-3 py-2 text-sm"
          >
            <option value="all">All Workflows</option>
            <option value="created">Created</option>
            <option value="pending">Pending</option>
            <option value="running">Running</option>
            <option value="paused">Paused</option>
            <option value="completed">Completed</option>
            <option value="failed">Failed</option>
            <option value="cancelled">Cancelled</option>
          </select>
          <button
            onClick={() => setShowCreateModal(true)}
            className="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700"
          >
            Create Workflow
          </button>
          <button
            onClick={loadDashboardData}
            className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
          >
            Refresh
          </button>
        </div>
      </div>

      {/* Statistics Cards */}
      {statistics && (
        <div className="grid grid-cols-1 md:grid-cols-6 gap-4">
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
                    <dt className="text-sm font-medium text-gray-500 truncate">Total</dt>
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
                    <span className="text-white text-sm font-medium">{statistics.active}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Active</dt>
                    <dd className="text-lg font-medium text-gray-900">{statistics.active}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-emerald-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-sm font-medium">{statistics.completed}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Completed</dt>
                    <dd className="text-lg font-medium text-gray-900">{statistics.completed}</dd>
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
                    <span className="text-white text-sm font-medium">{statistics.failed}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Failed</dt>
                    <dd className="text-lg font-medium text-gray-900">{statistics.failed}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-gray-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-sm font-medium">{statistics.cancelled}</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Cancelled</dt>
                    <dd className="text-lg font-medium text-gray-900">{statistics.cancelled}</dd>
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
                    <span className="text-white text-xs font-medium">⏱️</span>
                  </div>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Avg Time</dt>
                    <dd className="text-lg font-medium text-gray-900">
                      {formatDuration(statistics.average_execution_time_ms)}
                    </dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Workflows Table */}
      <div className="bg-white shadow rounded-lg overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">
            Research Workflows ({workflows.length})
          </h3>
        </div>

        {loading ? (
          <div className="p-6 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600 mx-auto"></div>
            <p className="mt-2 text-gray-500">Loading workflows...</p>
          </div>
        ) : workflows.length === 0 ? (
          <div className="p-6 text-center">
            <p className="text-gray-500">No workflows found.</p>
            <button
              onClick={() => setShowCreateModal(true)}
              className="mt-2 text-indigo-600 hover:text-indigo-500"
            >
              Create your first workflow
            </button>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Workflow
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Progress
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Methodology
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Created
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {workflows.map((workflow) => (
                  <tr key={workflow.id}>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div>
                          <div className="text-sm font-medium text-gray-900">
                            {workflow.name}
                          </div>
                          <div className="text-sm text-gray-500 max-w-xs truncate">
                            {workflow.query}
                          </div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <span className="mr-2">{getStatusIcon(workflow.status)}</span>
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(workflow.status)}`}>
                          {workflow.status}
                        </span>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div className="text-sm text-gray-900">
                          {workflow.progress.toFixed(1)}%
                        </div>
                        <div className="ml-2 w-16 bg-gray-200 rounded-full h-2">
                          <div
                            className={`h-2 rounded-full ${
                              workflow.progress === 100 ? 'bg-green-500' :
                              workflow.progress > 0 ? 'bg-blue-500' : 'bg-gray-300'
                            }`}
                            style={{ width: `${workflow.progress}%` }}
                          ></div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getMethodologyColor(workflow.methodology)}`}>
                        {workflow.methodology.replace('_', ' ')}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {new Date(workflow.created_at).toLocaleDateString()}
                      <div className="text-xs text-gray-400">
                        by {workflow.created_by}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      <div className="flex justify-end space-x-2">
                        {workflow.status === 'created' && (
                          <button
                            onClick={() => handleStartWorkflow(workflow.id)}
                            className="text-green-600 hover:text-green-900"
                            title="Start Workflow"
                          >
                            ▶️
                          </button>
                        )}
                        {workflow.status === 'running' && (
                          <button
                            onClick={() => handlePauseWorkflow(workflow.id)}
                            className="text-yellow-600 hover:text-yellow-900"
                            title="Pause Workflow"
                          >
                            ⏸️
                          </button>
                        )}
                        {workflow.status === 'paused' && (
                          <button
                            onClick={() => handleResumeWorkflow(workflow.id)}
                            className="text-blue-600 hover:text-blue-900"
                            title="Resume Workflow"
                          >
                            ▶️
                          </button>
                        )}
                        {(workflow.status === 'running' || workflow.status === 'paused') && (
                          <button
                            onClick={() => handleCancelWorkflow(workflow.id)}
                            className="text-red-600 hover:text-red-900"
                            title="Cancel Workflow"
                          >
                            🚫
                          </button>
                        )}
                        {(workflow.status === 'completed' || workflow.status === 'failed' || workflow.status === 'cancelled') && (
                          <button
                            onClick={() => handleDeleteWorkflow(workflow.id)}
                            className="text-red-600 hover:text-red-900"
                            title="Delete Workflow"
                          >
                            🗑️
                          </button>
                        )}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Create Workflow Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <h3 className="text-lg font-medium text-gray-900 mb-4">
                Create New Research Workflow
              </h3>

              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700">Workflow Name</label>
                  <input
                    type="text"
                    value={createData.name}
                    onChange={(e) => setCreateData({...createData, name: e.target.value})}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    placeholder="Enter workflow name"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Research Query</label>
                  <textarea
                    value={createData.query}
                    onChange={(e) => setCreateData({...createData, query: e.target.value})}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    rows={3}
                    placeholder="Enter your research question or topic"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Methodology</label>
                  <select
                    value={createData.methodology}
                    onChange={(e) => setCreateData({...createData, methodology: e.target.value})}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                  >
                    <option value="hybrid">Hybrid (Recommended)</option>
                    <option value="don_lim">Don Lim (Cost-Optimized)</option>
                    <option value="nick_scamara">Nick Scamara (Professional)</option>
                  </select>
                  <p className="mt-1 text-xs text-gray-500">
                    {createData.methodology === 'hybrid' && 'Combines both approaches for maximum coverage'}
                    {createData.methodology === 'don_lim' && 'OpenRouter + SerpApi + Jina AI'}
                    {createData.methodology === 'nick_scamara' && 'Firecrawl + AI SDK'}
                  </p>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Created By</label>
                  <input
                    type="text"
                    value={createData.created_by}
                    onChange={(e) => setCreateData({...createData, created_by: e.target.value})}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    placeholder="Your name or identifier"
                  />
                </div>
              </div>

              <div className="flex justify-end space-x-3 pt-4">
                <button
                  onClick={() => {
                    setShowCreateModal(false)
                    setCreateData({
                      name: '',
                      query: '',
                      methodology: 'hybrid',
                      created_by: 'user'
                    })
                  }}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
                >
                  Cancel
                </button>
                <button
                  onClick={handleCreateWorkflow}
                  className="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-md hover:bg-indigo-700"
                >
                  Create Workflow
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
