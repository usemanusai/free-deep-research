import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import React from 'react'
import BMadIntegrationDashboard from '../BMadIntegrationDashboard'
import { mockTauriCommand } from '@/test/setup'

// Mock Tauri commands
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

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

describe('BMadIntegrationDashboard', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    
    // Mock successful API responses
    mockTauriCommand('get_bmad_integration_status', {
      is_connected: true,
      last_sync: '2025-07-19T10:00:00Z',
      active_workflows: 3,
      pending_requests: 1
    })
    
    mockTauriCommand('get_bmad_agent_status', {
      agents: [
        { name: 'Product Manager', status: 'active', last_activity: '2025-07-19T09:30:00Z' },
        { name: 'Architect', status: 'active', last_activity: '2025-07-19T09:25:00Z' },
        { name: 'Task Breakdown Specialist', status: 'idle', last_activity: '2025-07-19T09:00:00Z' }
      ]
    })
    
    mockTauriCommand('get_bmad_workflow_stats', {
      total_workflows: 15,
      completed_workflows: 12,
      active_workflows: 3,
      success_rate: 0.85,
      average_completion_time: 1800
    })
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  it('renders dashboard with integration status', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    // Check for main dashboard elements
    expect(screen.getByText('BMAD AI Agent Integration')).toBeInTheDocument()
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Connected')).toBeInTheDocument()
    })
  })

  it('displays research task management section', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    // Check for research tasks section
    expect(screen.getByText('Research Tasks')).toBeInTheDocument()
    expect(screen.getByText('New Task')).toBeInTheDocument()
    
    // Check for sample research tasks
    await waitFor(() => {
      expect(screen.getByText('Market Analysis: AI Research Tools')).toBeInTheDocument()
      expect(screen.getByText('Technical Architecture Review')).toBeInTheDocument()
      expect(screen.getByText('Competitive Intelligence Analysis')).toBeInTheDocument()
    })
  })

  it('shows task status indicators correctly', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    await waitFor(() => {
      // Check for different status badges
      expect(screen.getByText('Completed')).toBeInTheDocument()
      expect(screen.getByText('In Progress')).toBeInTheDocument()
      expect(screen.getByText('Queued')).toBeInTheDocument()
    })
  })

  it('handles new task button click', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    const newTaskButton = screen.getByText('New Task')
    expect(newTaskButton).toBeInTheDocument()

    // Mock console.log to verify click handler
    const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {})
    
    fireEvent.click(newTaskButton)
    
    expect(consoleSpy).toHaveBeenCalledWith('Create new research task')
    
    consoleSpy.mockRestore()
  })

  it('displays informational help section', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    // Check for help section
    expect(screen.getByText('Research Task Management')).toBeInTheDocument()
    expect(screen.getByText(/Create, monitor, and manage individual research tasks/)).toBeInTheDocument()
  })

  it('shows agent status information', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    await waitFor(() => {
      // Check for agent status indicators
      expect(screen.getByText(/3 active workflows/)).toBeInTheDocument()
      expect(screen.getByText(/1 pending requests/)).toBeInTheDocument()
    })
  })

  it('handles integration status errors gracefully', async () => {
    const Wrapper = createWrapper()
    
    // Mock error response
    mockTauriCommand('get_bmad_integration_status', null, new Error('Connection failed'))
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    // Should still render the component structure
    expect(screen.getByText('BMAD AI Agent Integration')).toBeInTheDocument()
    expect(screen.getByText('Research Tasks')).toBeInTheDocument()
  })

  it('displays workflow statistics correctly', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    await waitFor(() => {
      // Check for workflow statistics
      expect(screen.getByText(/15.*total workflows/i)).toBeInTheDocument()
      expect(screen.getByText(/85%.*success rate/i)).toBeInTheDocument()
    })
  })

  it('shows proper loading states', async () => {
    const Wrapper = createWrapper()
    
    // Mock delayed response
    mockTauriCommand('get_bmad_integration_status', new Promise(resolve => 
      setTimeout(() => resolve({
        is_connected: true,
        last_sync: '2025-07-19T10:00:00Z',
        active_workflows: 3,
        pending_requests: 1
      }), 100)
    ))
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    // Should show loading state initially
    expect(screen.getByText('BMAD AI Agent Integration')).toBeInTheDocument()
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Connected')).toBeInTheDocument()
    }, { timeout: 200 })
  })

  it('handles task view button clicks', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    await waitFor(() => {
      const viewButtons = screen.getAllByRole('button', { name: /view/i })
      expect(viewButtons.length).toBeGreaterThan(0)
      
      // Click first view button
      fireEvent.click(viewButtons[0])
      // In a real implementation, this would open a task detail view
    })
  })

  it('displays correct task descriptions', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    await waitFor(() => {
      expect(screen.getByText('Comprehensive analysis of AI-powered research platforms')).toBeInTheDocument()
      expect(screen.getByText('Evaluating scalable research system architectures')).toBeInTheDocument()
      expect(screen.getByText('Research on competing research automation platforms')).toBeInTheDocument()
    })
  })

  it('shows proper icon usage', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    // Check that icons are rendered (they should be in the DOM as SVG elements)
    const svgElements = screen.getAllByRole('img', { hidden: true })
    expect(svgElements.length).toBeGreaterThan(0)
  })

  it('maintains responsive design structure', async () => {
    const Wrapper = createWrapper()
    
    render(
      <Wrapper>
        <BMadIntegrationDashboard />
      </Wrapper>
    )

    // Check for responsive grid classes
    const container = screen.getByText('Research Tasks').closest('div')
    expect(container).toHaveClass('space-y-6')
  })
})
