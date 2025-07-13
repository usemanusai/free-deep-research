import React, { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/core'
import { 
  PlusIcon, 
  KeyIcon, 
  TrashIcon, 
  PencilIcon,
  ArrowUpTrayIcon,
  ArrowDownTrayIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  XCircleIcon,
  ClockIcon,
  ChartBarIcon
} from '@heroicons/react/24/outline'
import { 
  ApiKey, 
  CreateApiKeyRequest, 
  UpdateApiKeyRequest, 
  ImportResult,
  ApiKeyTestResult,
  ServiceProvider 
} from '@/types/api'
import LoadingSpinner from '@/components/common/LoadingSpinner'
import ErrorAlert from '@/components/common/ErrorAlert'

export default function ApiKeyManager() {
  const queryClient = useQueryClient()
  const [showAddForm, setShowAddForm] = useState(false)
  const [showImportModal, setShowImportModal] = useState(false)
  const [editingKey, setEditingKey] = useState<ApiKey | null>(null)
  const [importType, setImportType] = useState<'csv' | 'json'>('csv')
  const [importContent, setImportContent] = useState('')
  const [selectedKeys, setSelectedKeys] = useState<Set<string>>(new Set())

  // Form state for adding/editing API key
  const [keyForm, setKeyForm] = useState<CreateApiKeyRequest>({
    service: 'openrouter',
    name: '',
    api_key: '',
    rate_limit: 50
  })

  // Fetch API keys with real-time updates
  const { data: apiKeys, isLoading, error, refetch } = useQuery({
    queryKey: ['api-keys'],
    queryFn: () => invoke<ApiKey[]>('get_api_keys'),
    refetchInterval: 30000, // Refetch every 30 seconds
  })

  // Mutations for API key operations
  const addKeyMutation = useMutation({
    mutationFn: (newKey: CreateApiKeyRequest) => invoke('add_api_key', newKey),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['api-keys'] })
      setShowAddForm(false)
      resetForm()
    },
  })

  const updateKeyMutation = useMutation({
    mutationFn: ({ id, updates }: { id: string; updates: UpdateApiKeyRequest }) => 
      invoke('update_api_key', { keyId: id, updates }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['api-keys'] })
      setEditingKey(null)
      resetForm()
    },
  })

  const deleteKeyMutation = useMutation({
    mutationFn: (keyId: string) => invoke('delete_api_key', { keyId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['api-keys'] })
    },
  })

  const testKeyMutation = useMutation({
    mutationFn: (keyId: string) => invoke<ApiKeyTestResult>('test_api_key', { keyId }),
  })

  const importKeysMutation = useMutation({
    mutationFn: ({ content, type }: { content: string; type: 'csv' | 'json' }) => {
      if (type === 'csv') {
        return invoke<ImportResult>('import_api_keys_csv', { csvContent: content })
      } else {
        return invoke<ImportResult>('import_api_keys_json', { jsonContent: content })
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['api-keys'] })
      setShowImportModal(false)
      setImportContent('')
    },
  })

  const resetForm = () => {
    setKeyForm({
      service: 'openrouter',
      name: '',
      api_key: '',
      rate_limit: 50
    })
  }

  const handleAddKey = () => {
    addKeyMutation.mutate(keyForm)
  }

  const handleUpdateKey = () => {
    if (!editingKey) return
    updateKeyMutation.mutate({
      id: editingKey.id,
      updates: {
        name: keyForm.name,
        rate_limit: keyForm.rate_limit,
      }
    })
  }

  const handleDeleteKey = (keyId: string) => {
    if (confirm('Are you sure you want to delete this API key?')) {
      deleteKeyMutation.mutate(keyId)
    }
  }

  const handleTestKey = (keyId: string) => {
    testKeyMutation.mutate(keyId)
  }

  const handleImport = () => {
    importKeysMutation.mutate({ content: importContent, type: importType })
  }

  const handleEditKey = (key: ApiKey) => {
    setEditingKey(key)
    setKeyForm({
      service: key.service,
      name: key.name,
      api_key: '', // Don't populate encrypted key
      rate_limit: key.rate_limit
    })
    setShowAddForm(true)
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return <CheckCircleIcon className="h-5 w-5 text-green-500" />
      case 'exhausted':
        return <ClockIcon className="h-5 w-5 text-yellow-500" />
      case 'error':
        return <XCircleIcon className="h-5 w-5 text-red-500" />
      case 'rate_limited':
        return <ExclamationTriangleIcon className="h-5 w-5 text-orange-500" />
      default:
        return <XCircleIcon className="h-5 w-5 text-gray-500" />
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'bg-green-100 text-green-800'
      case 'exhausted':
        return 'bg-yellow-100 text-yellow-800'
      case 'error':
        return 'bg-red-100 text-red-800'
      case 'rate_limited':
        return 'bg-orange-100 text-orange-800'
      default:
        return 'bg-gray-100 text-gray-800'
    }
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" message="Loading API keys..." />
      </div>
    )
  }

  if (error) {
    return (
      <ErrorAlert 
        title="Failed to Load API Keys"
        message="Unable to fetch API keys. Please try again."
        onRetry={refetch}
      />
    )
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">API Key Management</h1>
          <p className="mt-1 text-sm text-gray-500">
            Manage your service API keys with real-time monitoring and intelligent rotation
          </p>
        </div>
        <div className="flex items-center space-x-3">
          <button
            onClick={() => setShowImportModal(true)}
            className="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <ArrowUpTrayIcon className="h-4 w-4 mr-2" />
            Import
          </button>
          <button
            onClick={() => setShowAddForm(true)}
            className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <PlusIcon className="h-4 w-4 mr-2" />
            Add API Key
          </button>
        </div>
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="p-5">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <KeyIcon className="h-6 w-6 text-gray-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Total Keys</dt>
                  <dd className="text-lg font-medium text-gray-900">{apiKeys?.length || 0}</dd>
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
                  <dt className="text-sm font-medium text-gray-500 truncate">Active</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {apiKeys?.filter(key => key.status === 'active').length || 0}
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
                <ExclamationTriangleIcon className="h-6 w-6 text-yellow-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Rate Limited</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {apiKeys?.filter(key => key.status === 'rate_limited' || key.status === 'exhausted').length || 0}
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
                <ChartBarIcon className="h-6 w-6 text-blue-400" />
              </div>
              <div className="ml-5 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">Total Usage</dt>
                  <dd className="text-lg font-medium text-gray-900">
                    {apiKeys?.reduce((sum, key) => sum + key.usage_count, 0) || 0}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* API Keys Table */}
      <div className="bg-white shadow rounded-lg overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">API Keys ({apiKeys?.length || 0})</h3>
        </div>

        {!apiKeys || apiKeys.length === 0 ? (
          <div className="p-6 text-center">
            <KeyIcon className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">No API keys</h3>
            <p className="mt-1 text-sm text-gray-500">Get started by adding your first API key.</p>
            <div className="mt-6">
              <button
                onClick={() => setShowAddForm(true)}
                className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              >
                <PlusIcon className="h-4 w-4 mr-2" />
                Add API Key
              </button>
            </div>
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
                    Name
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Usage
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Last Used
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {apiKeys.map((key) => {
                  const usagePercentage = key.rate_limit > 0 ? (key.usage_count / key.rate_limit) * 100 : 0
                  return (
                    <tr key={key.id} className="hover:bg-gray-50">
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="flex items-center">
                          <div className="text-sm font-medium text-gray-900 capitalize">
                            {key.service}
                          </div>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="text-sm text-gray-900">{key.name}</div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="text-sm text-gray-900">
                          {key.usage_count} / {key.rate_limit}
                        </div>
                        <div className="w-full bg-gray-200 rounded-full h-2 mt-1">
                          <div
                            className={`h-2 rounded-full transition-all duration-300 ${
                              usagePercentage >= 90 ? 'bg-red-500' :
                              usagePercentage >= 70 ? 'bg-yellow-500' : 'bg-green-500'
                            }`}
                            style={{ width: `${Math.min(usagePercentage, 100)}%` }}
                          />
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="flex items-center">
                          {getStatusIcon(key.status)}
                          <span className={`ml-2 inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(key.status)}`}>
                            {key.status}
                          </span>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {key.last_used ? new Date(key.last_used).toLocaleDateString() : 'Never'}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                        <div className="flex items-center justify-end space-x-2">
                          <button
                            onClick={() => handleTestKey(key.id)}
                            disabled={testKeyMutation.isPending}
                            className="text-primary-600 hover:text-primary-900 disabled:text-gray-400"
                          >
                            {testKeyMutation.isPending ? 'Testing...' : 'Test'}
                          </button>
                          <button
                            onClick={() => handleEditKey(key)}
                            className="text-gray-600 hover:text-gray-900"
                          >
                            <PencilIcon className="h-4 w-4" />
                          </button>
                          <button
                            onClick={() => handleDeleteKey(key.id)}
                            disabled={deleteKeyMutation.isPending}
                            className="text-red-600 hover:text-red-900 disabled:text-gray-400"
                          >
                            <TrashIcon className="h-4 w-4" />
                          </button>
                        </div>
                      </td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Add/Edit API Key Modal */}
      {showAddForm && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <h3 className="text-lg font-medium text-gray-900 mb-4">
                {editingKey ? 'Edit API Key' : 'Add New API Key'}
              </h3>
              <form onSubmit={(e) => {
                e.preventDefault()
                editingKey ? handleUpdateKey() : handleAddKey()
              }} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700">Service</label>
                  <select
                    value={keyForm.service}
                    onChange={(e) => setKeyForm({ ...keyForm, service: e.target.value as ServiceProvider })}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    required
                    disabled={!!editingKey}
                  >
                    <option value="openrouter">OpenRouter</option>
                    <option value="serpapi">SerpAPI</option>
                    <option value="jina">Jina AI</option>
                    <option value="firecrawl">Firecrawl</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Name</label>
                  <input
                    type="text"
                    value={keyForm.name}
                    onChange={(e) => setKeyForm({ ...keyForm, name: e.target.value })}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    placeholder="e.g., Primary OpenRouter Key"
                    required
                  />
                </div>

                {!editingKey && (
                  <div>
                    <label className="block text-sm font-medium text-gray-700">API Key</label>
                    <input
                      type="password"
                      value={keyForm.api_key}
                      onChange={(e) => setKeyForm({ ...keyForm, api_key: e.target.value })}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                      placeholder="Enter your API key"
                      required
                    />
                  </div>
                )}

                <div>
                  <label className="block text-sm font-medium text-gray-700">Rate Limit</label>
                  <input
                    type="number"
                    value={keyForm.rate_limit}
                    onChange={(e) => setKeyForm({ ...keyForm, rate_limit: parseInt(e.target.value) || 0 })}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    placeholder="50"
                    min="1"
                  />
                </div>

                <div className="flex justify-end space-x-3 pt-4">
                  <button
                    type="button"
                    onClick={() => {
                      setShowAddForm(false)
                      setEditingKey(null)
                      resetForm()
                    }}
                    className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                  >
                    Cancel
                  </button>
                  <button
                    type="submit"
                    disabled={addKeyMutation.isPending || updateKeyMutation.isPending}
                    className="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:bg-gray-400"
                  >
                    {addKeyMutation.isPending || updateKeyMutation.isPending ? 'Saving...' :
                     editingKey ? 'Update Key' : 'Add Key'}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      )}

      {/* Import API Keys Modal */}
      {showImportModal && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <h3 className="text-lg font-medium text-gray-900 mb-4">Import API Keys</h3>

              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700">Import Format</label>
                  <select
                    value={importType}
                    onChange={(e) => setImportType(e.target.value as 'csv' | 'json')}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                  >
                    <option value="csv">CSV</option>
                    <option value="json">JSON</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">
                    {importType === 'csv' ? 'CSV Content' : 'JSON Content'}
                  </label>
                  <textarea
                    value={importContent}
                    onChange={(e) => setImportContent(e.target.value)}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 h-32 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    placeholder={
                      importType === 'csv'
                        ? 'service,name,key,rate_limit\nopenrouter,Primary Key,sk-...,50'
                        : '[{"service": "openrouter", "name": "Primary Key", "api_key": "sk-...", "rate_limit": 50}]'
                    }
                  />
                </div>

                <div className="flex justify-end space-x-3 pt-4">
                  <button
                    type="button"
                    onClick={() => {
                      setShowImportModal(false)
                      setImportContent('')
                    }}
                    className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                  >
                    Cancel
                  </button>
                  <button
                    onClick={handleImport}
                    disabled={!importContent.trim() || importKeysMutation.isPending}
                    className="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:bg-gray-400"
                  >
                    {importKeysMutation.isPending ? 'Importing...' : 'Import'}
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
