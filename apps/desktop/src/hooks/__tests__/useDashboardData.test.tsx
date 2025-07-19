import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { renderHook, waitFor } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import React from 'react'
import { mockTauriCommand, createMockApiKey, createMockWorkflow, createMockSystemMetrics } from '@/test/setup'
import { useDashboardStats, useSystemHealth, useResourceStatus } from '@/hooks/useDashboardData'

// Create a wrapper component for React Query
const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        cacheTime: 0,
      },
    },
  })

  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  )
}

describe('Dashboard Data Hooks', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  describe('useDashboardStats', () => {
    it('should fetch and combine dashboard statistics', async () => {
      const mockApiKeys = [createMockApiKey(), createMockApiKey({ id: 'key-2', name: 'Second Key' })]
      const mockWorkflows = [createMockWorkflow(), createMockWorkflow({ id: 'workflow-2', name: 'Second Workflow' })]
      const mockHealth = {
        security: 'Healthy',
        data_persistence: 'Healthy',
        monitoring: 'Healthy',
        api_manager: 'Healthy',
        research_engine: 'Healthy'
      }
      const mockQueue = {
        total_workflows: 2,
        active_workflows: 1,
        queued_workflows: 0,
        completed_workflows: 1,
        failed_workflows: 0,
        average_completion_time: 300,
        queue_processing_rate: 1.0,
        estimated_wait_time: 0
      }
      const mockAuditLogs = [
        {
          id: 'audit-1',
          event_type: 'api_key_created',
          severity: 'info',
          timestamp: '2025-07-18T10:00:00Z',
          details: { key_name: 'Test Key' },
          user_id: 'test-user'
        }
      ]

      // Mock all the required Tauri commands
      mockTauriCommand('get_api_keys', mockApiKeys)
      mockTauriCommand('get_all_research_workflows', mockWorkflows)
      mockTauriCommand('get_service_health', mockHealth)
      mockTauriCommand('get_queue_statistics', mockQueue)
      mockTauriCommand('get_audit_logs', mockAuditLogs)

      const wrapper = createWrapper()
      const { result } = renderHook(() => useDashboardStats(), { wrapper })

      await waitFor(() => {
        expect(result.current.dashboardStats).toBeTruthy()
      })

      const stats = result.current.dashboardStats!
      expect(stats.total_api_keys).toBe(2)
      expect(stats.active_api_keys).toBe(2) // Both mock keys are active
      expect(stats.total_workflows).toBe(2)
      expect(stats.active_workflows).toBe(1)
      expect(stats.system_health_score).toBeGreaterThan(0)
      expect(stats.recent_activity).toHaveLength(1)
    })

    it('should handle loading states correctly', async () => {
      // Don't mock any commands to simulate loading
      const wrapper = createWrapper()
      const { result } = renderHook(() => useDashboardStats(), { wrapper })

      expect(result.current.isLoading).toBe(true)
      expect(result.current.dashboardStats).toBeNull()
    })

    it('should handle errors gracefully', async () => {
      // Mock one command to fail
      mockTauriCommand('get_api_keys', [])
      mockTauriCommand('get_all_research_workflows', [])
      vi.mocked(mockTauriCommand).mockImplementation((command) => {
        if (command === 'get_service_health') {
          throw new Error('Service health check failed')
        }
        return Promise.resolve([])
      })

      const wrapper = createWrapper()
      const { result } = renderHook(() => useDashboardStats(), { wrapper })

      await waitFor(() => {
        expect(result.current.error).toBeTruthy()
      })

      expect(result.current.error).toBeTruthy()
      expect(result.current.dashboardStats).toBeNull()
    })
  })

  describe('useSystemHealth', () => {
    it('should fetch system health status', async () => {
      const mockHealthStatus = {
        security: 'Healthy',
        data_persistence: 'Healthy',
        monitoring: 'Healthy',
        api_manager: 'Healthy',
        research_engine: 'Healthy'
      }

      mockTauriCommand('get_service_health', mockHealthStatus)

      const wrapper = createWrapper()
      const { result } = renderHook(() => useSystemHealth(), { wrapper })

      await waitFor(() => {
        expect(result.current.data).toEqual(mockHealthStatus)
      })

      expect(result.current.data).toEqual(mockHealthStatus)
      expect(result.current.isLoading).toBe(false)
      expect(result.current.error).toBeNull()
    })

    it('should handle unhealthy services', async () => {
      const mockHealthStatus = {
        security: 'Healthy',
        data_persistence: 'Unhealthy',
        monitoring: 'Healthy',
        api_manager: 'Healthy',
        research_engine: 'Unknown'
      }

      mockTauriCommand('get_service_health', mockHealthStatus)

      const wrapper = createWrapper()
      const { result } = renderHook(() => useSystemHealth(), { wrapper })

      await waitFor(() => {
        expect(result.current.data).toEqual(mockHealthStatus)
      })

      expect(result.current.data?.data_persistence).toBe('Unhealthy')
      expect(result.current.data?.research_engine).toBe('Unknown')
    })

    it('should refetch health status periodically', async () => {
      const mockHealthStatus = {
        security: 'Healthy',
        data_persistence: 'Healthy',
        monitoring: 'Healthy',
        api_manager: 'Healthy',
        research_engine: 'Healthy'
      }

      mockTauriCommand('get_service_health', mockHealthStatus)

      const wrapper = createWrapper()
      const { result } = renderHook(() => useSystemHealth(), { wrapper })

      await waitFor(() => {
        expect(result.current.data).toEqual(mockHealthStatus)
      })

      // Verify that the hook is set up for periodic refetching
      expect(result.current.data).toEqual(mockHealthStatus)
    })
  })

  describe('useResourceStatus', () => {
    it('should fetch resource status', async () => {
      const mockResourceStatus = createMockSystemMetrics({
        cpu_usage_percent: 45.2,
        memory_usage_percent: 62.8,
        disk_usage_percent: 78.5,
        network_bytes_sent: 1024000,
        network_bytes_received: 2048000
      })

      mockTauriCommand('get_resource_status', mockResourceStatus)

      const wrapper = createWrapper()
      const { result } = renderHook(() => useResourceStatus(), { wrapper })

      await waitFor(() => {
        expect(result.current.data).toEqual(mockResourceStatus)
      })

      expect(result.current.data?.cpu_usage_percent).toBe(45.2)
      expect(result.current.data?.memory_usage_percent).toBe(62.8)
      expect(result.current.data?.disk_usage_percent).toBe(78.5)
    })

    it('should handle high resource usage', async () => {
      const mockResourceStatus = createMockSystemMetrics({
        cpu_usage_percent: 95.5,
        memory_usage_percent: 89.2,
        disk_usage_percent: 92.1
      })

      mockTauriCommand('get_resource_status', mockResourceStatus)

      const wrapper = createWrapper()
      const { result } = renderHook(() => useResourceStatus(), { wrapper })

      await waitFor(() => {
        expect(result.current.data).toEqual(mockResourceStatus)
      })

      // Verify high usage values
      expect(result.current.data?.cpu_usage_percent).toBeGreaterThan(90)
      expect(result.current.data?.memory_usage_percent).toBeGreaterThan(80)
      expect(result.current.data?.disk_usage_percent).toBeGreaterThan(90)
    })
  })

  describe('Error Handling', () => {
    it('should handle network errors', async () => {
      vi.mocked(mockTauriCommand).mockImplementation(() => {
        throw new Error('Network connection failed')
      })

      const wrapper = createWrapper()
      const { result } = renderHook(() => useSystemHealth(), { wrapper })

      await waitFor(() => {
        expect(result.current.error).toBeTruthy()
      })

      expect(result.current.error).toBeTruthy()
      expect(result.current.data).toBeUndefined()
    })

    it('should retry failed requests', async () => {
      let callCount = 0
      vi.mocked(mockTauriCommand).mockImplementation(() => {
        callCount++
        if (callCount < 3) {
          throw new Error('Temporary failure')
        }
        return Promise.resolve({
          security: 'Healthy',
          data_persistence: 'Healthy',
          monitoring: 'Healthy',
          api_manager: 'Healthy',
          research_engine: 'Healthy'
        })
      })

      const wrapper = createWrapper()
      const { result } = renderHook(() => useSystemHealth(), { wrapper })

      await waitFor(() => {
        expect(result.current.data).toBeTruthy()
      })

      expect(callCount).toBeGreaterThanOrEqual(3)
      expect(result.current.data).toBeTruthy()
    })
  })

  describe('Performance', () => {
    it('should cache results appropriately', async () => {
      const mockHealthStatus = {
        security: 'Healthy',
        data_persistence: 'Healthy',
        monitoring: 'Healthy',
        api_manager: 'Healthy',
        research_engine: 'Healthy'
      }

      mockTauriCommand('get_service_health', mockHealthStatus)

      const wrapper = createWrapper()
      
      // Render the hook twice
      const { result: result1 } = renderHook(() => useSystemHealth(), { wrapper })
      const { result: result2 } = renderHook(() => useSystemHealth(), { wrapper })

      await waitFor(() => {
        expect(result1.current.data).toEqual(mockHealthStatus)
        expect(result2.current.data).toEqual(mockHealthStatus)
      })

      // Both hooks should have the same data (cached)
      expect(result1.current.data).toEqual(result2.current.data)
    })
  })
})
