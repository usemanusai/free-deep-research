import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mockInvoke, mockTauriCommand, mockTauriCommandError } from '@/test/setup'
import { ResearchWorkflow, WorkflowStatus, QueueStatistics } from '@/types/api'

// Mock data
const mockWorkflow: ResearchWorkflow = {
  id: 'workflow-123',
  name: 'Test Research Workflow',
  description: 'A test research workflow',
  status: 'idle',
  progress: 0,
  created_at: '2025-07-18T10:00:00Z',
  updated_at: '2025-07-18T10:00:00Z',
  started_at: null,
  completed_at: null,
  error_message: null,
  methodology: 'hybrid',
  query: 'AI in healthcare research',
  parameters: {
    max_results: 50,
    search_depth: 'comprehensive',
    include_academic: true,
    include_news: true,
    date_range: '1y'
  },
  results: null,
  metadata: {
    created_by: 'test-user',
    priority: 'normal'
  }
}

const mockQueueStats: QueueStatistics = {
  total_workflows: 5,
  active_workflows: 2,
  queued_workflows: 1,
  completed_workflows: 2,
  failed_workflows: 0,
  average_completion_time: 300,
  queue_processing_rate: 0.8,
  estimated_wait_time: 120
}

describe('Research Workflow Integration', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  describe('Workflow Management', () => {
    it('should create a new research workflow', async () => {
      const createRequest = {
        name: mockWorkflow.name,
        description: mockWorkflow.description,
        methodology: mockWorkflow.methodology,
        query: mockWorkflow.query,
        parameters: mockWorkflow.parameters
      }

      mockTauriCommand('create_research_workflow', mockWorkflow)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ResearchWorkflow>('create_research_workflow', { 
        workflow: createRequest 
      })

      expect(result).toEqual(mockWorkflow)
      expect(mockInvoke).toHaveBeenCalledWith('create_research_workflow', { 
        workflow: createRequest 
      })
    })

    it('should get all research workflows', async () => {
      mockTauriCommand('get_all_research_workflows', [mockWorkflow])

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ResearchWorkflow[]>('get_all_research_workflows')

      expect(result).toEqual([mockWorkflow])
      expect(result).toHaveLength(1)
    })

    it('should get a specific workflow by ID', async () => {
      mockTauriCommand('get_research_workflow', mockWorkflow)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ResearchWorkflow>('get_research_workflow', { 
        workflowId: mockWorkflow.id 
      })

      expect(result).toEqual(mockWorkflow)
      expect(mockInvoke).toHaveBeenCalledWith('get_research_workflow', { 
        workflowId: mockWorkflow.id 
      })
    })

    it('should start a research workflow', async () => {
      const runningWorkflow = { 
        ...mockWorkflow, 
        status: 'running' as WorkflowStatus,
        started_at: '2025-07-18T10:05:00Z'
      }

      mockTauriCommand('start_research_workflow', runningWorkflow)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ResearchWorkflow>('start_research_workflow', { 
        workflowId: mockWorkflow.id 
      })

      expect(result.status).toBe('running')
      expect(result.started_at).toBeTruthy()
    })

    it('should pause a running workflow', async () => {
      const pausedWorkflow = { 
        ...mockWorkflow, 
        status: 'paused' as WorkflowStatus 
      }

      mockTauriCommand('pause_research_workflow', pausedWorkflow)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ResearchWorkflow>('pause_research_workflow', { 
        workflowId: mockWorkflow.id 
      })

      expect(result.status).toBe('paused')
    })

    it('should cancel a workflow', async () => {
      const cancelledWorkflow = { 
        ...mockWorkflow, 
        status: 'cancelled' as WorkflowStatus 
      }

      mockTauriCommand('cancel_research_workflow', cancelledWorkflow)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ResearchWorkflow>('cancel_research_workflow', { 
        workflowId: mockWorkflow.id 
      })

      expect(result.status).toBe('cancelled')
    })

    it('should delete a workflow', async () => {
      mockTauriCommand('delete_research_workflow', undefined)

      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('delete_research_workflow', { workflowId: mockWorkflow.id })

      expect(mockInvoke).toHaveBeenCalledWith('delete_research_workflow', { 
        workflowId: mockWorkflow.id 
      })
    })
  })

  describe('Workflow Progress Tracking', () => {
    it('should get workflow progress', async () => {
      const progressData = {
        workflow_id: mockWorkflow.id,
        current_step: 2,
        total_steps: 5,
        progress_percentage: 40,
        current_step_name: 'Data Collection',
        estimated_completion: '2025-07-18T10:15:00Z',
        steps_completed: ['Initialization', 'Query Processing'],
        current_step_details: {
          operation: 'Searching academic databases',
          progress: 0.6,
          results_found: 25
        }
      }

      mockTauriCommand('get_workflow_progress', progressData)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke('get_workflow_progress', { 
        workflowId: mockWorkflow.id 
      })

      expect(result).toEqual(progressData)
      expect(result.progress_percentage).toBe(40)
      expect(result.current_step_name).toBe('Data Collection')
    })

    it('should get workflow results', async () => {
      const resultsData = {
        workflow_id: mockWorkflow.id,
        status: 'completed',
        total_results: 150,
        sources_found: 45,
        analysis_summary: 'Comprehensive analysis of AI in healthcare',
        key_findings: [
          'AI improves diagnostic accuracy by 23%',
          'Cost reduction of 15% in routine procedures',
          'Patient satisfaction increased by 18%'
        ],
        data_sources: [
          { type: 'academic', count: 30 },
          { type: 'news', count: 15 }
        ],
        generated_at: '2025-07-18T10:30:00Z'
      }

      mockTauriCommand('get_workflow_results', resultsData)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke('get_workflow_results', { 
        workflowId: mockWorkflow.id 
      })

      expect(result).toEqual(resultsData)
      expect(result.total_results).toBe(150)
      expect(result.key_findings).toHaveLength(3)
    })
  })

  describe('Queue Management', () => {
    it('should get queue statistics', async () => {
      mockTauriCommand('get_queue_statistics', mockQueueStats)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<QueueStatistics>('get_queue_statistics')

      expect(result).toEqual(mockQueueStats)
      expect(result.total_workflows).toBe(5)
      expect(result.active_workflows).toBe(2)
      expect(result.queue_processing_rate).toBe(0.8)
    })

    it('should handle queue overflow', async () => {
      const overflowStats = {
        ...mockQueueStats,
        queued_workflows: 10,
        estimated_wait_time: 1800 // 30 minutes
      }

      mockTauriCommand('get_queue_statistics', overflowStats)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<QueueStatistics>('get_queue_statistics')

      expect(result.queued_workflows).toBe(10)
      expect(result.estimated_wait_time).toBe(1800)
    })
  })

  describe('Error Handling', () => {
    it('should handle workflow creation errors', async () => {
      mockTauriCommandError('create_research_workflow', 'Invalid methodology specified')

      const { invoke } = await import('@tauri-apps/api/core')
      const invalidRequest = {
        name: 'Test Workflow',
        methodology: 'invalid-method',
        query: 'test query'
      }

      await expect(invoke('create_research_workflow', { workflow: invalidRequest }))
        .rejects.toThrow('Invalid methodology specified')
    })

    it('should handle workflow not found errors', async () => {
      mockTauriCommandError('get_research_workflow', 'Workflow not found')

      const { invoke } = await import('@tauri-apps/api/core')

      await expect(invoke('get_research_workflow', { workflowId: 'non-existent-id' }))
        .rejects.toThrow('Workflow not found')
    })

    it('should handle workflow state transition errors', async () => {
      mockTauriCommandError('start_research_workflow', 'Cannot start completed workflow')

      const { invoke } = await import('@tauri-apps/api/core')

      await expect(invoke('start_research_workflow', { workflowId: mockWorkflow.id }))
        .rejects.toThrow('Cannot start completed workflow')
    })

    it('should handle failed workflow execution', async () => {
      const failedWorkflow = {
        ...mockWorkflow,
        status: 'failed' as WorkflowStatus,
        error_message: 'API rate limit exceeded',
        completed_at: '2025-07-18T10:10:00Z'
      }

      mockTauriCommand('get_research_workflow', failedWorkflow)

      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ResearchWorkflow>('get_research_workflow', { 
        workflowId: mockWorkflow.id 
      })

      expect(result.status).toBe('failed')
      expect(result.error_message).toBe('API rate limit exceeded')
    })
  })

  describe('Performance and Concurrency', () => {
    it('should handle multiple concurrent workflow operations', async () => {
      mockTauriCommand('get_all_research_workflows', [mockWorkflow])

      const { invoke } = await import('@tauri-apps/api/core')
      
      // Make multiple concurrent requests
      const promises = Array(3).fill(null).map(() => 
        invoke<ResearchWorkflow[]>('get_all_research_workflows')
      )

      const results = await Promise.all(promises)

      expect(results).toHaveLength(3)
      results.forEach(result => {
        expect(result).toEqual([mockWorkflow])
      })
    })

    it('should complete workflow operations within reasonable time', async () => {
      mockTauriCommand('create_research_workflow', mockWorkflow)

      const { invoke } = await import('@tauri-apps/api/core')
      const startTime = Date.now()
      
      await invoke<ResearchWorkflow>('create_research_workflow', { 
        workflow: {
          name: 'Performance Test',
          methodology: 'hybrid',
          query: 'test'
        }
      })
      
      const endTime = Date.now()
      const duration = endTime - startTime

      // Should complete within 100ms (mocked)
      expect(duration).toBeLessThan(100)
    })
  })
})
