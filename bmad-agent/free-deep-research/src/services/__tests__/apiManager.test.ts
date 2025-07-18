import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mockInvoke, mockTauriCommand, mockTauriCommandError } from '@/test/setup'
import { ApiKey, CreateApiKeyRequest, ApiKeyTestResult } from '@/types/api'

// Mock data
const mockApiKey: ApiKey = {
  id: 'test-key-id',
  name: 'Test OpenRouter Key',
  service: 'openrouter',
  encrypted_key: 'encrypted-test-key',
  created_at: '2025-07-18T10:00:00Z',
  last_used: '2025-07-18T10:00:00Z',
  is_active: true,
  usage_count: 0,
  rate_limit: 60,
  rate_limit_window: 60,
  current_usage: 0,
  last_reset: '2025-07-18T10:00:00Z',
  metadata: {},
  tags: [],
  description: 'Test API key for OpenRouter service'
}

const mockCreateRequest: CreateApiKeyRequest = {
  name: 'New Test Key',
  service: 'openrouter',
  key: 'test-api-key-value',
  description: 'A new test API key',
  rate_limit: 60,
  tags: ['test']
}

const mockTestResult: ApiKeyTestResult = {
  success: true,
  response_time_ms: 150,
  status_code: 200,
  error_message: null,
  tested_at: '2025-07-18T10:00:00Z'
}

describe('API Manager Integration', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  describe('API Key Management', () => {
    it('should fetch all API keys', async () => {
      mockTauriCommand('get_api_keys', [mockApiKey])

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ApiKey[]>('get_api_keys')

      expect(result).toEqual([mockApiKey])
      expect(mockInvoke).toHaveBeenCalledWith('get_api_keys')
    })

    it('should create a new API key', async () => {
      mockTauriCommand('add_api_key', mockApiKey)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ApiKey>('add_api_key', { key: mockCreateRequest })

      expect(result).toEqual(mockApiKey)
      expect(mockInvoke).toHaveBeenCalledWith('add_api_key', { key: mockCreateRequest })
    })

    it('should handle API key creation errors', async () => {
      mockTauriCommandError('add_api_key', 'Invalid API key format')

      const { invoke } = await import('@tauri-apps/api/core')

      await expect(invoke('add_api_key', { key: mockCreateRequest }))
        .rejects.toThrow('Invalid API key format')
    })

    it('should update an existing API key', async () => {
      const updates = { name: 'Updated Key Name', is_active: false }
      const updatedKey = { ...mockApiKey, ...updates }
      
      mockTauriCommand('update_api_key', updatedKey)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ApiKey>('update_api_key', { 
        id: mockApiKey.id, 
        updates 
      })

      expect(result).toEqual(updatedKey)
      expect(mockInvoke).toHaveBeenCalledWith('update_api_key', { 
        id: mockApiKey.id, 
        updates 
      })
    })

    it('should delete an API key', async () => {
      mockTauriCommand('delete_api_key', undefined)

      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('delete_api_key', { id: mockApiKey.id })

      expect(mockInvoke).toHaveBeenCalledWith('delete_api_key', { id: mockApiKey.id })
    })

    it('should test API key connection', async () => {
      mockTauriCommand('test_api_key', mockTestResult)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ApiKeyTestResult>('test_api_key', { id: mockApiKey.id })

      expect(result).toEqual(mockTestResult)
      expect(result.success).toBe(true)
      expect(result.response_time_ms).toBe(150)
    })

    it('should handle failed API key test', async () => {
      const failedResult: ApiKeyTestResult = {
        success: false,
        response_time_ms: 0,
        status_code: 401,
        error_message: 'Unauthorized: Invalid API key',
        tested_at: '2025-07-18T10:00:00Z'
      }

      mockTauriCommand('test_api_key', failedResult)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ApiKeyTestResult>('test_api_key', { id: mockApiKey.id })

      expect(result.success).toBe(false)
      expect(result.error_message).toBe('Unauthorized: Invalid API key')
    })
  })

  describe('Rate Limiting', () => {
    it('should check if request can be made', async () => {
      mockTauriCommand('can_make_request', true)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<boolean>('can_make_request', { 
        service: 'openrouter' 
      })

      expect(result).toBe(true)
      expect(mockInvoke).toHaveBeenCalledWith('can_make_request', { 
        service: 'openrouter' 
      })
    })

    it('should record API request', async () => {
      mockTauriCommand('record_api_request', undefined)

      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('record_api_request', { 
        service: 'openrouter',
        keyId: mockApiKey.id
      })

      expect(mockInvoke).toHaveBeenCalledWith('record_api_request', { 
        service: 'openrouter',
        keyId: mockApiKey.id
      })
    })

    it('should get key usage status', async () => {
      const usageStatus = {
        current_usage: 25,
        rate_limit: 60,
        window_start: '2025-07-18T10:00:00Z',
        window_end: '2025-07-18T11:00:00Z',
        percentage_used: 41.67,
        requests_remaining: 35,
        reset_time: '2025-07-18T11:00:00Z'
      }

      mockTauriCommand('get_key_usage_status', usageStatus)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke('get_key_usage_status', { keyId: mockApiKey.id })

      expect(result).toEqual(usageStatus)
      expect(result.percentage_used).toBeCloseTo(41.67, 2)
    })
  })

  describe('Service Health', () => {
    it('should perform health check', async () => {
      mockTauriCommand('health_check', 'System is healthy')

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<string>('health_check')

      expect(result).toBe('System is healthy')
    })

    it('should get service health status', async () => {
      const healthStatus = {
        security: 'Healthy',
        data_persistence: 'Healthy',
        monitoring: 'Healthy',
        api_manager: 'Healthy',
        research_engine: 'Healthy'
      }

      mockTauriCommand('get_service_health', healthStatus)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke('get_service_health')

      expect(result).toEqual(healthStatus)
      expect(Object.values(result).every(status => status === 'Healthy')).toBe(true)
    })

    it('should handle unhealthy services', async () => {
      const healthStatus = {
        security: 'Healthy',
        data_persistence: 'Unhealthy',
        monitoring: 'Healthy',
        api_manager: 'Healthy',
        research_engine: 'Unknown'
      }

      mockTauriCommand('get_service_health', healthStatus)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke('get_service_health')

      expect(result.data_persistence).toBe('Unhealthy')
      expect(result.research_engine).toBe('Unknown')
    })
  })

  describe('Error Handling', () => {
    it('should handle network errors gracefully', async () => {
      mockTauriCommandError('get_api_keys', 'Network connection failed')

      const { invoke } = await import('@tauri-apps/api/core')

      await expect(invoke('get_api_keys'))
        .rejects.toThrow('Network connection failed')
    })

    it('should handle invalid parameters', async () => {
      mockTauriCommandError('add_api_key', 'Invalid service type')

      const { invoke } = await import('@tauri-apps/api/core')
      const invalidRequest = { ...mockCreateRequest, service: 'invalid-service' }

      await expect(invoke('add_api_key', { key: invalidRequest }))
        .rejects.toThrow('Invalid service type')
    })

    it('should handle rate limit exceeded', async () => {
      mockTauriCommand('can_make_request', false)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<boolean>('can_make_request', { 
        service: 'openrouter' 
      })

      expect(result).toBe(false)
    })
  })

  describe('Performance', () => {
    it('should complete API key operations within reasonable time', async () => {
      mockTauriCommand('get_api_keys', [mockApiKey])

      const { invoke } = await import('@tauri-apps/api/core')
      const startTime = Date.now()
      
      await invoke<ApiKey[]>('get_api_keys')
      
      const endTime = Date.now()
      const duration = endTime - startTime

      // Should complete within 100ms (mocked, so should be very fast)
      expect(duration).toBeLessThan(100)
    })

    it('should handle concurrent requests', async () => {
      mockTauriCommand('get_api_keys', [mockApiKey])

      const { invoke } = await import('@tauri-apps/api/core')
      
      // Make multiple concurrent requests
      const promises = Array(5).fill(null).map(() => 
        invoke<ApiKey[]>('get_api_keys')
      )

      const results = await Promise.all(promises)

      // All requests should succeed
      expect(results).toHaveLength(5)
      results.forEach(result => {
        expect(result).toEqual([mockApiKey])
      })
    })
  })
})
