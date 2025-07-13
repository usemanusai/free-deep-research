import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'

interface ApiKey {
  id: string
  service: string
  name: string
  usage_count: number
  rate_limit: number
  status: string
  created_at: string
  updated_at: string
}

interface ImportResult {
  successful_count: number
  failed_count: number
  errors: string[]
}

export default function ApiKeyManager() {
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([])
  const [loading, setLoading] = useState(true)
  const [showAddForm, setShowAddForm] = useState(false)
  const [showImportModal, setShowImportModal] = useState(false)
  const [importType, setImportType] = useState<'csv' | 'json'>('csv')
  const [importContent, setImportContent] = useState('')
  const [importResult, setImportResult] = useState<ImportResult | null>(null)

  // Form state for adding new API key
  const [newKey, setNewKey] = useState({
    service: 'openrouter',
    name: '',
    api_key: '',
    rate_limit: ''
  })

  useEffect(() => {
    loadApiKeys()
  }, [])

  const loadApiKeys = async () => {
    try {
      setLoading(true)
      const keys = await invoke<ApiKey[]>('get_api_keys')
      setApiKeys(keys)
    } catch (error) {
      console.error('Failed to load API keys:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleAddKey = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      const request = {
        service: newKey.service,
        name: newKey.name,
        api_key: newKey.api_key,
        rate_limit: newKey.rate_limit ? parseInt(newKey.rate_limit) : null
      }

      await invoke('add_api_key', { request })
      setNewKey({ service: 'openrouter', name: '', api_key: '', rate_limit: '' })
      setShowAddForm(false)
      loadApiKeys()
    } catch (error) {
      console.error('Failed to add API key:', error)
      alert('Failed to add API key: ' + error)
    }
  }

  const handleDeleteKey = async (keyId: string) => {
    if (!confirm('Are you sure you want to delete this API key?')) return

    try {
      await invoke('delete_api_key', { keyId })
      loadApiKeys()
    } catch (error) {
      console.error('Failed to delete API key:', error)
      alert('Failed to delete API key: ' + error)
    }
  }

  const handleTestKey = async (keyId: string) => {
    const startTime = Date.now()

    try {
      // Check if request can be made first
      const canMake = await invoke<boolean>('can_make_request', { keyId })
      if (!canMake) {
        alert('Request blocked by rate limiter. Check rate limit dashboard for details.')
        return
      }

      const result = await invoke('test_api_key', { keyId })
      const responseTime = Date.now() - startTime

      // Record the API request and performance
      await Promise.all([
        invoke('record_api_request', { keyId, success: true }),
        invoke('record_key_performance', { keyId, success: true, responseTimeMs: responseTime })
      ])

      alert(JSON.stringify(result, null, 2))
    } catch (error) {
      console.error('Failed to test API key:', error)
      const responseTime = Date.now() - startTime

      // Record failed request and performance
      try {
        await Promise.all([
          invoke('record_api_request', { keyId, success: false }),
          invoke('record_key_performance', { keyId, success: false, responseTimeMs: responseTime })
        ])
      } catch (recordError) {
        console.error('Failed to record failed request:', recordError)
      }

      alert('Failed to test API key: ' + error)
    }
  }

  const handleImport = async () => {
    try {
      let result: ImportResult

      if (importType === 'csv') {
        result = await invoke<ImportResult>('import_api_keys_csv', { csvContent: importContent })
      } else {
        result = await invoke<ImportResult>('import_api_keys_json', { jsonContent: importContent })
      }

      setImportResult(result)
      if (result.successful_count > 0) {
        loadApiKeys()
      }
    } catch (error) {
      console.error('Failed to import API keys:', error)
      alert('Failed to import API keys: ' + error)
    }
  }

  const handleExport = async (format: 'csv' | 'json') => {
    try {
      let content: string
      let filename: string

      if (format === 'csv') {
        content = await invoke<string>('export_api_keys_csv')
        filename = 'api_keys.csv'
      } else {
        content = await invoke<string>('export_api_keys_json')
        filename = 'api_keys.json'
      }

      // Create download link
      const blob = new Blob([content], { type: format === 'csv' ? 'text/csv' : 'application/json' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = filename
      a.click()
      URL.revokeObjectURL(url)
    } catch (error) {
      console.error('Failed to export API keys:', error)
      alert('Failed to export API keys: ' + error)
    }
  }

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'active': return 'text-green-600 bg-green-100'
      case 'exhausted': return 'text-yellow-600 bg-yellow-100'
      case 'error': return 'text-red-600 bg-red-100'
      case 'disabled': return 'text-gray-600 bg-gray-100'
      default: return 'text-gray-600 bg-gray-100'
    }
  }

  const getUsagePercentage = (usage: number, limit: number) => {
    return limit > 0 ? (usage / limit) * 100 : 0
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">API Key Management</h1>
          <p className="mt-1 text-sm text-gray-500">
            Manage your API keys for external services
          </p>
        </div>

        <div className="flex space-x-3">
          <a
            href="/rate-limit-dashboard"
            className="bg-purple-600 text-white px-4 py-2 rounded-md hover:bg-purple-700 inline-flex items-center"
          >
            <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
            Rate Limits
          </a>
          <a
            href="/key-rotation-dashboard"
            className="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700 inline-flex items-center"
          >
            <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            Key Rotation
          </a>
          <button
            onClick={() => setShowImportModal(true)}
            className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
          >
            Import Keys
          </button>
          <div className="relative">
            <button className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700">
              Export Keys
            </button>
            <div className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg hidden group-hover:block">
              <button
                onClick={() => handleExport('csv')}
                className="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
              >
                Export as CSV
              </button>
              <button
                onClick={() => handleExport('json')}
                className="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
              >
                Export as JSON
              </button>
            </div>
          </div>
          <button
            onClick={() => setShowAddForm(true)}
            className="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700"
          >
            Add API Key
          </button>
        </div>
      </div>

      {/* API Keys Table */}
      <div className="bg-white shadow rounded-lg overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">API Keys ({apiKeys.length})</h3>
        </div>

        {loading ? (
          <div className="p-6 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600 mx-auto"></div>
            <p className="mt-2 text-gray-500">Loading API keys...</p>
          </div>
        ) : apiKeys.length === 0 ? (
          <div className="p-6 text-center">
            <p className="text-gray-500">No API keys found. Add your first API key to get started.</p>
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
                    Created
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {apiKeys.map((key) => {
                  const usagePercentage = getUsagePercentage(key.usage_count, key.rate_limit)
                  return (
                    <tr key={key.id}>
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
                            className={`h-2 rounded-full ${
                              usagePercentage >= 90 ? 'bg-red-500' :
                              usagePercentage >= 70 ? 'bg-yellow-500' : 'bg-green-500'
                            }`}
                            style={{ width: `${Math.min(usagePercentage, 100)}%` }}
                          ></div>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(key.status)}`}>
                          {key.status}
                        </span>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {new Date(key.created_at).toLocaleDateString()}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                        <button
                          onClick={() => handleTestKey(key.id)}
                          className="text-indigo-600 hover:text-indigo-900 mr-3"
                        >
                          Test
                        </button>
                        <button
                          onClick={() => handleDeleteKey(key.id)}
                          className="text-red-600 hover:text-red-900"
                        >
                          Delete
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

      {/* Add API Key Modal */}
      {showAddForm && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <h3 className="text-lg font-medium text-gray-900 mb-4">Add New API Key</h3>
              <form onSubmit={handleAddKey} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700">Service</label>
                  <select
                    value={newKey.service}
                    onChange={(e) => setNewKey({ ...newKey, service: e.target.value })}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    required
                  >
                    <option value="openrouter">OpenRouter</option>
                    <option value="serpapi">SerpAPI</option>
                    <option value="jina">Jina AI</option>
                    <option value="firecrawl">Firecrawl</option>
                    <option value="tavily">Tavily</option>
                    <option value="exa">Exa AI</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Name</label>
                  <input
                    type="text"
                    value={newKey.name}
                    onChange={(e) => setNewKey({ ...newKey, name: e.target.value })}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    placeholder="e.g., Primary OpenRouter Key"
                    required
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">API Key</label>
                  <input
                    type="password"
                    value={newKey.api_key}
                    onChange={(e) => setNewKey({ ...newKey, api_key: e.target.value })}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    placeholder="Enter your API key"
                    required
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700">Rate Limit (optional)</label>
                  <input
                    type="number"
                    value={newKey.rate_limit}
                    onChange={(e) => setNewKey({ ...newKey, rate_limit: e.target.value })}
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
                    placeholder="Leave empty for default"
                  />
                </div>

                <div className="flex justify-end space-x-3 pt-4">
                  <button
                    type="button"
                    onClick={() => setShowAddForm(false)}
                    className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
                  >
                    Cancel
                  </button>
                  <button
                    type="submit"
                    className="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-md hover:bg-indigo-700"
                  >
                    Add Key
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
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2"
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
                    className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 h-32"
                    placeholder={
                      importType === 'csv'
                        ? 'service,name,key,rate_limit\nopenrouter,Primary Key,sk-...,50'
                        : '[{"service": "openrouter", "name": "Primary Key", "api_key": "sk-...", "rate_limit": 50}]'
                    }
                  />
                </div>

                {importResult && (
                  <div className="p-3 bg-gray-50 rounded-md">
                    <p className="text-sm">
                      <span className="text-green-600">Success: {importResult.successful_count}</span>
                      {importResult.failed_count > 0 && (
                        <span className="text-red-600 ml-2">Failed: {importResult.failed_count}</span>
                      )}
                    </p>
                    {importResult.errors.length > 0 && (
                      <div className="mt-2">
                        <p className="text-sm font-medium text-red-600">Errors:</p>
                        <ul className="text-xs text-red-500 mt-1">
                          {importResult.errors.map((error, index) => (
                            <li key={index}>• {error}</li>
                          ))}
                        </ul>
                      </div>
                    )}
                  </div>
                )}

                <div className="flex justify-end space-x-3 pt-4">
                  <button
                    type="button"
                    onClick={() => {
                      setShowImportModal(false)
                      setImportContent('')
                      setImportResult(null)
                    }}
                    className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
                  >
                    Cancel
                  </button>
                  <button
                    onClick={handleImport}
                    disabled={!importContent.trim()}
                    className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:bg-gray-400"
                  >
                    Import
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
