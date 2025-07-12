import { useQuery } from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/tauri'

export interface SystemHealthStatus {
  security: ServiceStatus
  data_persistence: ServiceStatus
  monitoring: ServiceStatus
  api_manager: ServiceStatus
  research_engine: ServiceStatus
}

export type ServiceStatus = 'Healthy' | 'Unhealthy' | 'Unknown'

export function useSystemHealth() {
  return useQuery({
    queryKey: ['system-health'],
    queryFn: async (): Promise<SystemHealthStatus> => {
      try {
        // For now, we'll use the basic health check
        // In the future, this will call get_service_health command
        await invoke<string>('health_check')
        
        // Mock response for development
        return {
          security: 'Healthy',
          data_persistence: 'Healthy',
          monitoring: 'Healthy',
          api_manager: 'Healthy',
          research_engine: 'Healthy',
        }
      } catch (error) {
        console.error('Health check failed:', error)
        throw error
      }
    },
    refetchInterval: 30000, // Refetch every 30 seconds
    retry: 3,
    retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
  })
}
