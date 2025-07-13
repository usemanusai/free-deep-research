import React, { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/core'
import {
  Cog6ToothIcon,
  ShieldCheckIcon,
  ServerIcon,
  ChartBarIcon,
  DocumentArrowDownIcon,
  BellIcon,
  PaintBrushIcon,
  KeyIcon,
  ClockIcon,
  CheckIcon,
  XMarkIcon
} from '@heroicons/react/24/outline'
import { AppConfiguration } from '@/types/api'
import LoadingSpinner from '@/components/common/LoadingSpinner'
import ErrorAlert from '@/components/common/ErrorAlert'

export default function ConfigurationPanel() {
  const queryClient = useQueryClient()
  const [selectedSection, setSelectedSection] = useState<'general' | 'security' | 'api' | 'research' | 'monitoring' | 'backup'>('general')
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false)

  // Fetch current configuration
  const { data: config, isLoading, error, refetch } = useQuery({
    queryKey: ['configuration'],
    queryFn: () => invoke<AppConfiguration>('get_configuration'),
  })

  // Local state for configuration changes
  const [localConfig, setLocalConfig] = useState<AppConfiguration | null>(null)

  // Initialize local config when data loads
  React.useEffect(() => {
    if (config && !localConfig) {
      setLocalConfig(config)
    }
  }, [config, localConfig])

  // Update configuration mutation
  const updateConfigMutation = useMutation({
    mutationFn: (newConfig: AppConfiguration) => invoke('update_configuration', { config: newConfig }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['configuration'] })
      setHasUnsavedChanges(false)
    },
  })

  // Reset configuration mutation
  const resetConfigMutation = useMutation({
    mutationFn: () => invoke('reset_configuration'),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['configuration'] })
      setHasUnsavedChanges(false)
    },
  })

  const handleConfigChange = (section: keyof AppConfiguration, field: string, value: any) => {
    if (!localConfig) return

    setLocalConfig({
      ...localConfig,
      [section]: {
        ...localConfig[section],
        [field]: value
      }
    })
    setHasUnsavedChanges(true)
  }

  const handleSave = () => {
    if (localConfig) {
      updateConfigMutation.mutate(localConfig)
    }
  }

  const handleReset = () => {
    if (confirm('Are you sure you want to reset all settings to defaults? This action cannot be undone.')) {
      resetConfigMutation.mutate()
    }
  }

  const handleDiscard = () => {
    setLocalConfig(config)
    setHasUnsavedChanges(false)
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <LoadingSpinner size="lg" message="Loading configuration..." />
      </div>
    )
  }

  if (error || !localConfig) {
    return (
      <ErrorAlert
        title="Configuration Error"
        message="Failed to load system configuration. Please try again."
        onRetry={refetch}
      />
    )
  }

  const sections = [
    { id: 'general', name: 'General', icon: Cog6ToothIcon },
    { id: 'security', name: 'Security', icon: ShieldCheckIcon },
    { id: 'api', name: 'API Management', icon: KeyIcon },
    { id: 'research', name: 'Research', icon: ChartBarIcon },
    { id: 'monitoring', name: 'Monitoring', icon: ServerIcon },
    { id: 'backup', name: 'Backup', icon: DocumentArrowDownIcon },
  ]

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">System Configuration</h1>
          <p className="mt-1 text-sm text-gray-500">
            Configure system preferences, security settings, and operational parameters
          </p>
        </div>

        {hasUnsavedChanges && (
          <div className="flex items-center space-x-3">
            <span className="text-sm text-yellow-600">You have unsaved changes</span>
            <button
              onClick={handleDiscard}
              className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
            >
              <XMarkIcon className="h-4 w-4 mr-1" />
              Discard
            </button>
            <button
              onClick={handleSave}
              disabled={updateConfigMutation.isPending}
              className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:bg-gray-400"
            >
              <CheckIcon className="h-4 w-4 mr-1" />
              {updateConfigMutation.isPending ? 'Saving...' : 'Save Changes'}
            </button>
          </div>
        )}
      </div>

      <div className="flex">
        {/* Sidebar */}
        <div className="w-64 bg-white shadow rounded-lg mr-6">
          <nav className="space-y-1 p-4">
            {sections.map((section) => (
              <button
                key={section.id}
                onClick={() => setSelectedSection(section.id as any)}
                className={`${
                  selectedSection === section.id
                    ? 'bg-primary-50 border-primary-500 text-primary-700'
                    : 'border-transparent text-gray-600 hover:bg-gray-50 hover:text-gray-900'
                } group w-full flex items-center pl-2 py-2 text-sm font-medium border-l-4 rounded-md`}
              >
                <section.icon className="mr-3 h-5 w-5" />
                {section.name}
              </button>
            ))}
          </nav>
        </div>

        {/* Main Content */}
        <div className="flex-1 bg-white shadow rounded-lg">
          <div className="p-6">
            {/* General Settings */}
            {selectedSection === 'general' && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900">General Settings</h3>

                <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Application Name</label>
                    <input
                      type="text"
                      value={localConfig.general.app_name}
                      onChange={(e) => handleConfigChange('general', 'app_name', e.target.value)}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Environment</label>
                    <select
                      value={localConfig.general.environment}
                      onChange={(e) => handleConfigChange('general', 'environment', e.target.value)}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    >
                      <option value="development">Development</option>
                      <option value="production">Production</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Log Level</label>
                    <select
                      value={localConfig.general.log_level}
                      onChange={(e) => handleConfigChange('general', 'log_level', e.target.value)}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    >
                      <option value="debug">Debug</option>
                      <option value="info">Info</option>
                      <option value="warn">Warning</option>
                      <option value="error">Error</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Theme</label>
                    <select
                      value={localConfig.general.theme}
                      onChange={(e) => handleConfigChange('general', 'theme', e.target.value)}
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    >
                      <option value="light">Light</option>
                      <option value="dark">Dark</option>
                      <option value="auto">Auto</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Auto-save Interval (seconds)</label>
                    <input
                      type="number"
                      value={localConfig.general.auto_save_interval}
                      onChange={(e) => handleConfigChange('general', 'auto_save_interval', parseInt(e.target.value) || 30)}
                      min="10"
                      max="300"
                      className="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                  </div>
                </div>
              </div>
            )}

            {/* Security Settings */}
            {selectedSection === 'security' && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900">Security Settings</h3>

                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="text-sm font-medium text-gray-900">Encryption</h4>
                      <p className="text-sm text-gray-500">Enable AES-256-GCM encryption for sensitive data</p>
                    </div>
                    <input
                      type="checkbox"
                      checked={localConfig.security.encryption_enabled}
                      onChange={(e) => handleConfigChange('security', 'encryption_enabled', e.target.checked)}
                      className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="text-sm font-medium text-gray-900">Master Password Required</h4>
                      <p className="text-sm text-gray-500">Require master password for application access</p>
                    </div>
                    <input
                      type="checkbox"
                      checked={localConfig.security.master_password_required}
                      onChange={(e) => handleConfigChange('security', 'master_password_required', e.target.checked)}
                      className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="text-sm font-medium text-gray-900">Audit Logging</h4>
                      <p className="text-sm text-gray-500">Enable comprehensive audit logging</p>
                    </div>
                    <input
                      type="checkbox"
                      checked={localConfig.security.audit_logging_enabled}
                      onChange={(e) => handleConfigChange('security', 'audit_logging_enabled', e.target.checked)}
                      className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Session Timeout (minutes)</label>
                    <input
                      type="number"
                      value={localConfig.security.session_timeout}
                      onChange={(e) => handleConfigChange('security', 'session_timeout', parseInt(e.target.value) || 30)}
                      min="5"
                      max="480"
                      className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                  </div>
                </div>
              </div>
            )}

            {/* API Management Settings */}
            {selectedSection === 'api' && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900">API Management Settings</h3>

                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="text-sm font-medium text-gray-900">Auto Key Rotation</h4>
                      <p className="text-sm text-gray-500">Automatically rotate API keys based on performance</p>
                    </div>
                    <input
                      type="checkbox"
                      checked={localConfig.api_management.auto_rotation_enabled}
                      onChange={(e) => handleConfigChange('api_management', 'auto_rotation_enabled', e.target.checked)}
                      className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Rate Limit Buffer (%)</label>
                    <input
                      type="number"
                      value={localConfig.api_management.rate_limit_buffer_percentage}
                      onChange={(e) => handleConfigChange('api_management', 'rate_limit_buffer_percentage', parseInt(e.target.value) || 10)}
                      min="0"
                      max="50"
                      className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Health Check Interval (seconds)</label>
                    <input
                      type="number"
                      value={localConfig.api_management.health_check_interval}
                      onChange={(e) => handleConfigChange('api_management', 'health_check_interval', parseInt(e.target.value) || 60)}
                      min="10"
                      max="300"
                      className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Emergency Stop Threshold (%)</label>
                    <input
                      type="number"
                      value={localConfig.api_management.emergency_stop_threshold}
                      onChange={(e) => handleConfigChange('api_management', 'emergency_stop_threshold', parseInt(e.target.value) || 95)}
                      min="80"
                      max="99"
                      className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                  </div>
                </div>
              </div>
            )}

            {/* Research Settings */}
            {selectedSection === 'research' && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900">Research Settings</h3>

                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Default Methodology</label>
                    <select
                      value={localConfig.research.default_methodology}
                      onChange={(e) => handleConfigChange('research', 'default_methodology', e.target.value)}
                      className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    >
                      <option value="don_lim">Don Lim</option>
                      <option value="nick_scamara">Nick Scamara</option>
                      <option value="hybrid">Hybrid</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Max Concurrent Workflows</label>
                    <input
                      type="number"
                      value={localConfig.research.max_concurrent_workflows}
                      onChange={(e) => handleConfigChange('research', 'max_concurrent_workflows', parseInt(e.target.value) || 3)}
                      min="1"
                      max="10"
                      className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="text-sm font-medium text-gray-900">Auto-cleanup Completed Workflows</h4>
                      <p className="text-sm text-gray-500">Automatically clean up old completed workflows</p>
                    </div>
                    <input
                      type="checkbox"
                      checked={localConfig.research.auto_cleanup_completed}
                      onChange={(e) => handleConfigChange('research', 'auto_cleanup_completed', e.target.checked)}
                      className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    />
                  </div>

                  {localConfig.research.auto_cleanup_completed && (
                    <div>
                      <label className="block text-sm font-medium text-gray-700">Cleanup After (days)</label>
                      <input
                        type="number"
                        value={localConfig.research.cleanup_after_days}
                        onChange={(e) => handleConfigChange('research', 'cleanup_after_days', parseInt(e.target.value) || 30)}
                        min="1"
                        max="365"
                        className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                      />
                    </div>
                  )}
                </div>
              </div>
            )}

            {/* Backup Settings */}
            {selectedSection === 'backup' && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900">Backup Settings</h3>

                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="text-sm font-medium text-gray-900">Auto Backup</h4>
                      <p className="text-sm text-gray-500">Enable automatic incremental backups</p>
                    </div>
                    <input
                      type="checkbox"
                      checked={localConfig.backup.auto_backup_enabled}
                      onChange={(e) => handleConfigChange('backup', 'auto_backup_enabled', e.target.checked)}
                      className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    />
                  </div>

                  {localConfig.backup.auto_backup_enabled && (
                    <>
                      <div>
                        <label className="block text-sm font-medium text-gray-700">Backup Interval (seconds)</label>
                        <input
                          type="number"
                          value={localConfig.backup.backup_interval_seconds}
                          onChange={(e) => handleConfigChange('backup', 'backup_interval_seconds', parseInt(e.target.value) || 30)}
                          min="30"
                          max="3600"
                          className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                      </div>

                      <div>
                        <label className="block text-sm font-medium text-gray-700">Max Backup Files</label>
                        <input
                          type="number"
                          value={localConfig.backup.max_backup_files}
                          onChange={(e) => handleConfigChange('backup', 'max_backup_files', parseInt(e.target.value) || 100)}
                          min="10"
                          max="1000"
                          className="mt-1 block w-64 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                      </div>

                      <div className="flex items-center justify-between">
                        <div>
                          <h4 className="text-sm font-medium text-gray-900">Backup Compression</h4>
                          <p className="text-sm text-gray-500">Compress backup files to save space</p>
                        </div>
                        <input
                          type="checkbox"
                          checked={localConfig.backup.backup_compression_enabled}
                          onChange={(e) => handleConfigChange('backup', 'backup_compression_enabled', e.target.checked)}
                          className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                        />
                      </div>

                      <div className="flex items-center justify-between">
                        <div>
                          <h4 className="text-sm font-medium text-gray-900">Backup Encryption</h4>
                          <p className="text-sm text-gray-500">Encrypt backup files with separate keys</p>
                        </div>
                        <input
                          type="checkbox"
                          checked={localConfig.backup.backup_encryption_enabled}
                          onChange={(e) => handleConfigChange('backup', 'backup_encryption_enabled', e.target.checked)}
                          className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                        />
                      </div>
                    </>
                  )}
                </div>
              </div>
            )}

            {/* Reset Section */}
            <div className="mt-8 pt-6 border-t border-gray-200">
              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-sm font-medium text-gray-900">Reset Configuration</h4>
                  <p className="text-sm text-gray-500">Reset all settings to their default values</p>
                </div>
                <button
                  onClick={handleReset}
                  disabled={resetConfigMutation.isPending}
                  className="inline-flex items-center px-4 py-2 border border-red-300 shadow-sm text-sm font-medium rounded-md text-red-700 bg-white hover:bg-red-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 disabled:bg-gray-100"
                >
                  {resetConfigMutation.isPending ? 'Resetting...' : 'Reset to Defaults'}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
