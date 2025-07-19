import React, { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/core'
import {
  PlusIcon,
  BeakerIcon,
  PlayIcon,
  PauseIcon,
  StopIcon,
  TrashIcon,
  EyeIcon,
  DocumentTextIcon,
  ClockIcon,
  CheckCircleIcon,
  XCircleIcon,
  ExclamationTriangleIcon,
  ArrowPathIcon,
  ChartBarIcon,
  Cog6ToothIcon
} from '@heroicons/react/24/outline'
import {
  ResearchWorkflow,
  WorkflowStatus,
  ResearchTemplate,
  WorkflowParameters,
  QueueStatistics
} from '@/types/api'
import LoadingSpinner from '@/components/common/LoadingSpinner'
import ErrorAlert from '@/components/common/ErrorAlert'

export default function ResearchWorkflowManager() {
  const queryClient = useQueryClient()
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [selectedWorkflow, setSelectedWorkflow] = useState<ResearchWorkflow | null>(null)
  const [showResults, setShowResults] = useState(false)
  const [filterStatus, setFilterStatus] = useState<WorkflowStatus | 'all'>('all')
  const [searchQuery, setSearchQuery] = useState('')

  // Form state for creating new workflow
  const [workflowForm, setWorkflowForm] = useState<{
    name: string;
    description: string;
    template_id: string | null;
    methodology: 'don_lim' | 'nick_scamara' | 'hybrid';
    parameters: Partial<WorkflowParameters>;
  }>({
    name: '',
    description: '',
    template_id: null,
    methodology: 'hybrid',
    parameters: {
      query: '',
      max_results: 10,
      depth_level: 2,
      include_sources: true,
      output_format: ['markdown'],
      custom_instructions: null,
      filters: {}
    }
  })

  // Fetch research workflows
  const { data: workflows, isLoading, error, refetch } = useQuery({
    queryKey: ['research-workflows'],
    queryFn: () => invoke<ResearchWorkflow[]>('get_all_research_workflows'),
    refetchInterval: 15000,
  })

  // Fetch research templates
  const { data: templates } = useQuery({
    queryKey: ['research-templates'],
    queryFn: () => invoke<ResearchTemplate[]>('get_all_research_templates'),
  })

  // Fetch queue statistics
  const { data: queueStats } = useQuery({
    queryKey: ['queue-statistics'],
    queryFn: () => invoke<QueueStatistics>('get_queue_statistics'),
    refetchInterval: 10000,
  })

  // Mutations for workflow operations
  const createWorkflowMutation = useMutation({
    mutationFn: (workflow: typeof workflowForm) =>
      invoke('create_research_workflow', { workflow }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['research-workflows'] })
      setShowCreateForm(false)
      resetForm()
    },
  })

  const startWorkflowMutation = useMutation({
    mutationFn: (workflowId: string) => invoke('start_research_workflow', { workflowId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['research-workflows'] })
    },
  })

  const pauseWorkflowMutation = useMutation({
    mutationFn: (workflowId: string) => invoke('pause_research_workflow', { workflowId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['research-workflows'] })
    },
  })

  const cancelWorkflowMutation = useMutation({
    mutationFn: (workflowId: string) => invoke('cancel_research_workflow', { workflowId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['research-workflows'] })
    },
  })

  const deleteWorkflowMutation = useMutation({
    mutationFn: (workflowId: string) => invoke('delete_research_workflow', { workflowId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['research-workflows'] })
    },
  })

  const resetForm = () => {
    setWorkflowForm({
      name: '',
      description: '',
      template_id: null,
      methodology: 'hybrid',
      parameters: {
        query: '',
        max_results: 10,
        depth_level: 2,
        include_sources: true,
        output_format: ['markdown'],
        custom_instructions: null,
        filters: {}
      }
    })
  }

  const handleCreateWorkflow = () => {
    createWorkflowMutation.mutate(workflowForm)
  }

  const handleStartWorkflow = (workflowId: string) => {
    startWorkflowMutation.mutate(workflowId)
  }

  const handlePauseWorkflow = (workflowId: string) => {
    pauseWorkflowMutation.mutate(workflowId)
  }

  const handleCancelWorkflow = (workflowId: string) => {
    if (confirm('Are you sure you want to cancel this workflow?')) {
      cancelWorkflowMutation.mutate(workflowId)
    }
  }

  const handleDeleteWorkflow = (workflowId: string) => {
    if (confirm('Are you sure you want to delete this workflow? This action cannot be undone.')) {
      deleteWorkflowMutation.mutate(workflowId)
    }
  }

  const getStatusIcon = (status: WorkflowStatus) => {
    switch (status) {
      case 'running':
        return <PlayIcon className="h-5 w-5 text-blue-500 animate-pulse" />
      case 'completed':
        return <CheckCircleIcon className="h-5 w-5 text-green-500" />
      case 'failed':
        return <XCircleIcon className="h-5 w-5 text-red-500" />
      case 'paused':
        return <PauseIcon className="h-5 w-5 text-yellow-500" />
      case 'cancelled':
        return <StopIcon className="h-5 w-5 text-gray-500" />
      case 'queued':
        return <ClockIcon className="h-5 w-5 text-purple-500" />
      default:
        return <DocumentTextIcon className="h-5 w-5 text-gray-400" />
    }
  }

  const getStatusColor = (status: WorkflowStatus) => {
    switch (status) {
      case 'running':
        return 'bg-blue-100 text-blue-800'
      case 'completed':
        return 'bg-green-100 text-green-800'
      case 'failed':
        return 'bg-red-100 text-red-800'
      case 'paused':
        return 'bg-yellow-100 text-yellow-800'
      case 'cancelled':
        return 'bg-gray-100 text-gray-800'
      case 'queued':
        return 'bg-purple-100 text-purple-800'
      default:
        return 'bg-gray-100 text-gray-800'
    }
  }

  // Filter workflows based on status and search query
  const filteredWorkflows = workflows?.filter(workflow => {
    const matchesStatus = filterStatus === 'all' || workflow.status === filterStatus
    const matchesSearch = !searchQuery ||
      workflow.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      workflow.description.toLowerCase().includes(searchQuery.toLowerCase())
    return matchesStatus && matchesSearch
  }) || []

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" message="Loading research workflows..." />
      </div>
    )
  }

  if (error) {
    return (
      <ErrorAlert
        title="Failed to Load Workflows"
        message="Unable to fetch research workflows. Please try again."
        onRetry={refetch}
      />
    )
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Research Workflows</h1>
          <p className="mt-1 text-sm text-gray-500">
            Create, manage, and monitor your research workflows with AI-powered methodologies
          </p>
        </div>
        <div className="flex items-center space-x-3">
          <button
            onClick={refetch}
            className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <ArrowPathIcon className="h-4 w-4 mr-1" />
            Refresh
          </button>
          <button
            onClick={() => setShowCreateForm(true)}
            className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <PlusIcon className="h-4 w-4 mr-2" />
            Create Workflow
          </button>
        </div>
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <BeakerIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Total Workflows</dt>
                  <dd className="text-lg font-medium text-gray-900">{workflows?.length || 0}</dd>
                </dl>
              </div>
            </div>
          </div>
        </div>

        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <PlayIcon className="h-6 w-6 text-blue-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Running</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {workflows?.filter(w => w.status === 'running').length || 0}
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
                <ClockIcon className="h-6 w-6 text-purple-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Queued</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {queueStats?.total_queued || 0}
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
                <CheckCircleIcon className="h-6 w-6 text-green-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Completed Today</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {queueStats?.completed_today || 0}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Filters and Search */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-medium text-gray-900">Workflows</h3>
            <div className="flex items-center space-x-4">
              {/* Search */}
              <div className="relative">
                <input
                  type="text"
                  placeholder="Search workflows..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="block w-64 pl-3 pr-10 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
                />
              </div>

              {/* Status Filter */}
              <select
                value={filterStatus}
                onChange={(e) => setFilterStatus(e.target.value as WorkflowStatus | 'all')}
                className="block w-40 pl-3 pr-10 py-2 text-base border border-gray-300 focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm rounded-md"
              >
                <option value="all">All Status</option>
                <option value="draft">Draft</option>
                <option value="queued">Queued</option>
                <option value="running">Running</option>
                <option value="paused">Paused</option>
                <option value="completed">Completed</option>
                <option value="failed">Failed</option>
                <option value="cancelled">Cancelled</option>
              </select>
            </div>
          </div>
        </div>

        {/* Workflows Table */}
        {filteredWorkflows.length === 0 ? (
          <div className="p-6 text-center">
            <BeakerIcon className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">No workflows found</h3>
            <p className="mt-1 text-sm text-gray-500">
              {searchQuery || filterStatus !== 'all'
                ? 'Try adjusting your search or filter criteria.'
                : 'Get started by creating your first research workflow.'
              }
            </p>
            {!searchQuery && filterStatus === 'all' && (
              <div className="mt-6">
                <button
                  onClick={() => setShowCreateForm(true)}
                  className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                >
                  <PlusIcon className="h-4 w-4 mr-2" />
                  Create Workflow
                </button>
              </div>
            )}
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
                {filteredWorkflows.map((workflow) => (
                  <tr key={workflow.id} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div>
                          <div className="text-sm font-medium text-gray-900">{workflow.name}</div>
                          <div className="text-sm text-gray-500">{workflow.description}</div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        {getStatusIcon(workflow.status)}
                        <span className={`ml-2 inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(workflow.status)}`}>
                          {workflow.status}
                        </span>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="w-full bg-gray-200 rounded-full h-2">
                        <div
                          className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                          style={{ width: `${workflow.progress.percentage}%` }}
                        />
                      </div>
                      <div className="text-xs text-gray-500 mt-1">
                        {workflow.progress.percentage}% ({workflow.progress.completed_steps}/{workflow.progress.total_steps})
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className="inline-flex px-2 py-1 text-xs font-medium rounded-full bg-gray-100 text-gray-800 capitalize">
                        {workflow.methodology.replace('_', ' ')}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {new Date(workflow.created_at).toLocaleDateString()}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      <div className="flex items-center justify-end space-x-2">
                        {workflow.status === 'draft' && (
                          <button
                            onClick={() => handleStartWorkflow(workflow.id)}
                            disabled={startWorkflowMutation.isPending}
                            className="text-green-600 hover:text-green-900 disabled:text-gray-400"
                            title="Start workflow"
                          >
                            <PlayIcon className="h-4 w-4" />
                          </button>
                        )}

                        {workflow.status === 'running' && (
                          <button
                            onClick={() => handlePauseWorkflow(workflow.id)}
                            disabled={pauseWorkflowMutation.isPending}
                            className="text-yellow-600 hover:text-yellow-900 disabled:text-gray-400"
                            title="Pause workflow"
                          >
                            <PauseIcon className="h-4 w-4" />
                          </button>
                        )}

                        {(workflow.status === 'running' || workflow.status === 'paused' || workflow.status === 'queued') && (
                          <button
                            onClick={() => handleCancelWorkflow(workflow.id)}
                            disabled={cancelWorkflowMutation.isPending}
                            className="text-red-600 hover:text-red-900 disabled:text-gray-400"
                            title="Cancel workflow"
                          >
                            <StopIcon className="h-4 w-4" />
                          </button>
                        )}

                        {workflow.status === 'completed' && (
                          <button
                            onClick={() => {
                              setSelectedWorkflow(workflow)
                              setShowResults(true)
                            }}
                            className="text-blue-600 hover:text-blue-900"
                            title="View results"
                          >
                            <EyeIcon className="h-4 w-4" />
                          </button>
                        )}

                        <button
                          onClick={() => handleDeleteWorkflow(workflow.id)}
                          disabled={deleteWorkflowMutation.isPending}
                          className="text-red-600 hover:text-red-900 disabled:text-gray-400"
                          title="Delete workflow"
                        >
                          <TrashIcon className="h-4 w-4" />
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

      {/* Create Workflow Modal */}
      {showCreateForm && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-2/3 max-w-4xl shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <h3 className="text-lg font-medium text-gray-900 mb-4">Create New Research Workflow</h3>
              <form onSubmit={(e) => {
                e.preventDefault()
                handleCreateWorkflow()
              }} className="space-y-6">
                <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Workflow Name</label>
                    <input
                      type="text"
                      value={workflowForm.name}
                      onChange={(e) => setWorkflowForm({ ...workflowForm, name: e.target.value })}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                      placeholder="e.g., AI Market Analysis"
                      required
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Methodology</label>
                    <select
                      value={workflowForm.methodology}
                      onChange={(e) => setWorkflowForm({ ...workflowForm, methodology: e.target.value as any })}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    >
                      <option value="don_lim">Don Lim (OpenRouter + SerpApi + Jina AI)</option>
                      <option value="nick_scamara">Nick Scamara (Firecrawl + AI SDK)</option>
                      <option value="hybrid">Hybrid (Best of Both)</option>
                    </select>
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Description</label>
                  <textarea
                    value={workflowForm.description}
                    onChange={(e) => setWorkflowForm({ ...workflowForm, description: e.target.value })}
                    rows={3}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    placeholder="Describe what this research workflow will accomplish..."
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Research Query</label>
                  <textarea
                    value={workflowForm.parameters.query}
                    onChange={(e) => setWorkflowForm({
                      ...workflowForm,
                      parameters: { ...workflowForm.parameters, query: e.target.value }
                    })}
                    rows={2}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    placeholder="Enter your research question or topic..."
                    required
                  />
                </div>

                <div className="grid grid-cols-1 gap-6 sm:grid-cols-3">
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Max Results</label>
                    <input
                      type="number"
                      value={workflowForm.parameters.max_results}
                      onChange={(e) => setWorkflowForm({
                        ...workflowForm,
                        parameters: { ...workflowForm.parameters, max_results: parseInt(e.target.value) || 10 }
                      })}
                      min="1"
                      max="100"
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Depth Level</label>
                    <select
                      value={workflowForm.parameters.depth_level}
                      onChange={(e) => setWorkflowForm({
                        ...workflowForm,
                        parameters: { ...workflowForm.parameters, depth_level: parseInt(e.target.value) || 2 }
                      })}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    >
                      <option value={1}>Surface (1)</option>
                      <option value={2}>Medium (2)</option>
                      <option value={3}>Deep (3)</option>
                      <option value={4}>Comprehensive (4)</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Output Format</label>
                    <select
                      value={workflowForm.parameters.output_format?.[0] || 'markdown'}
                      onChange={(e) => setWorkflowForm({
                        ...workflowForm,
                        parameters: { ...workflowForm.parameters, output_format: [e.target.value] }
                      })}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    >
                      <option value="markdown">Markdown</option>
                      <option value="pdf">PDF</option>
                      <option value="html">HTML</option>
                      <option value="json">JSON</option>
                    </select>
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Custom Instructions (Optional)</label>
                  <textarea
                    value={workflowForm.parameters.custom_instructions || ''}
                    onChange={(e) => setWorkflowForm({
                      ...workflowForm,
                      parameters: { ...workflowForm.parameters, custom_instructions: e.target.value || null }
                    })}
                    rows={2}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    placeholder="Any specific instructions for the research process..."
                  />
                </div>

                <div className="flex justify-end space-x-3 pt-4">
                  <button
                    type="button"
                    onClick={() => {
                      setShowCreateForm(false)
                      resetForm()
                    }}
                    className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                  >
                    Cancel
                  </button>
                  <button
                    type="submit"
                    disabled={createWorkflowMutation.isPending}
                    className="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:bg-gray-400"
                  >
                    {createWorkflowMutation.isPending ? 'Creating...' : 'Create Workflow'}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
