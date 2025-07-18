# Component Documentation

## Overview

This document provides comprehensive documentation for all React components in the Free Deep Research System. Each component is documented with its purpose, props, usage examples, and best practices.

## Table of Contents

1. [Common Components](#common-components)
2. [Dashboard Components](#dashboard-components)
3. [Research Components](#research-components)
4. [Template Components](#template-components)
5. [Monitoring Components](#monitoring-components)
6. [Security Components](#security-components)
7. [Hooks Documentation](#hooks-documentation)
8. [Best Practices](#best-practices)

## Common Components

### ErrorBoundary

A React error boundary component that catches JavaScript errors anywhere in the child component tree.

**Props:**
```typescript
interface Props {
  children: ReactNode
  fallback?: ReactNode
  onError?: (error: Error, errorInfo: ErrorInfo) => void
}
```

**Usage:**
```tsx
import ErrorBoundary from '@/components/common/ErrorBoundary'

function App() {
  return (
    <ErrorBoundary
      fallback={<div>Something went wrong!</div>}
      onError={(error, errorInfo) => {
        console.error('Error caught:', error, errorInfo)
      }}
    >
      <MyComponent />
    </ErrorBoundary>
  )
}
```

### LazyWrapper

A wrapper component for implementing lazy loading with Suspense and error boundaries.

**Props:**
```typescript
interface LazyWrapperProps {
  fallback?: React.ReactNode
  errorFallback?: React.ReactNode
  children: React.ReactNode
}
```

**Usage:**
```tsx
import { LazyWrapper, withLazyLoading } from '@/components/common/LazyWrapper'

// Method 1: Direct wrapper
<LazyWrapper fallback={<LoadingSpinner />}>
  <LazyComponent />
</LazyWrapper>

// Method 2: HOC
const LazyAnalytics = withLazyLoading(
  () => import('@/components/analytics'),
  { displayName: 'Analytics' }
)
```

### LoadingSpinner

A reusable loading spinner component with customizable appearance.

**Props:**
```typescript
interface LoadingSpinnerProps {
  size?: 'sm' | 'md' | 'lg'
  message?: string
  className?: string
}
```

**Usage:**
```tsx
import LoadingSpinner from '@/components/common/LoadingSpinner'

<LoadingSpinner 
  size="lg" 
  message="Loading research data..." 
  className="my-4"
/>
```

### LazyImage

An optimized image component with lazy loading and intersection observer.

**Props:**
```typescript
interface LazyImageProps extends React.ImgHTMLAttributes<HTMLImageElement> {
  src: string
  alt: string
  placeholder?: string
  className?: string
  onLoad?: () => void
  onError?: () => void
}
```

**Usage:**
```tsx
import { LazyImage } from '@/components/common/LazyWrapper'

<LazyImage
  src="/api/chart/performance.png"
  alt="Performance Chart"
  placeholder="data:image/svg+xml;base64,..."
  className="w-full h-64 object-cover"
  onLoad={() => console.log('Image loaded')}
/>
```

## Dashboard Components

### ExecutiveDashboard

The main dashboard component displaying system overview and key metrics.

**Features:**
- Real-time system health monitoring
- API key usage statistics
- Research workflow status
- Performance metrics
- Recent activity feed

**Usage:**
```tsx
import ExecutiveDashboard from '@/components/dashboard/ExecutiveDashboard'

function DashboardPage() {
  return (
    <div className="container mx-auto px-4">
      <ExecutiveDashboard />
    </div>
  )
}
```

**Data Dependencies:**
- `useDashboardStats()` hook
- `useSystemHealth()` hook
- `useRealTimeMonitoring()` hook

## Research Components

### ResearchWorkflow

Component for creating and managing research workflows.

**Props:**
```typescript
interface ResearchWorkflowProps {
  initialWorkflow?: Partial<ResearchWorkflow>
  onWorkflowCreated?: (workflow: ResearchWorkflow) => void
  onWorkflowUpdated?: (workflow: ResearchWorkflow) => void
}
```

**Usage:**
```tsx
import ResearchWorkflow from '@/components/research/ResearchWorkflow'

<ResearchWorkflow
  onWorkflowCreated={(workflow) => {
    console.log('New workflow created:', workflow.id)
    navigate(`/research/${workflow.id}`)
  }}
/>
```

### ResearchWorkflowDashboard

Dashboard for monitoring and managing multiple research workflows.

**Features:**
- Workflow list with status indicators
- Bulk operations (start, pause, cancel)
- Progress tracking
- Results preview
- Queue management

**Usage:**
```tsx
import ResearchWorkflowDashboard from '@/components/research/ResearchWorkflowDashboard'

<ResearchWorkflowDashboard />
```

## Template Components

### TemplateManagementDashboard

Comprehensive template management interface.

**Features:**
- Template CRUD operations
- Category filtering
- Search functionality
- Template execution
- Rating system
- Import/export capabilities

**Usage:**
```tsx
import TemplateManagement from '@/components/templates/TemplateManagementDashboard'

<TemplateManagement />
```

**Key Methods:**
```tsx
const {
  templates,
  createTemplate,
  updateTemplate,
  deleteTemplate,
  executeTemplate,
  rateTemplate
} = useTemplateManagement()
```

## Monitoring Components

### RealTimeConsole

Real-time system monitoring console with live metrics.

**Features:**
- Live system metrics (CPU, memory, disk)
- Service health status
- Network activity monitoring
- Performance alerts
- Resource usage graphs

**Usage:**
```tsx
import RealTimeConsole from '@/components/monitoring/RealTimeConsole'

<RealTimeConsole />
```

## Security Components

### withSecurity HOC

Higher-order component for adding security checks to components.

**Usage:**
```tsx
import { withSecurity } from '@/hooks/useSecurity'

const SecureComponent = withSecurity(MyComponent, {
  requireAuth: true,
  requiredPermissions: ['admin', 'research'],
  rateLimitKey: 'secure-component'
})
```

## Hooks Documentation

### usePerformanceMonitoring

Hook for monitoring component and application performance.

**Returns:**
```typescript
interface PerformanceHookReturn {
  startMeasurement: (name: string, category: string, metadata?: any) => string
  endMeasurement: (id: string) => PerformanceMetrics | null
  getReport: () => PerformanceReport
  clearMetrics: () => void
  updateBudget: (budget: Partial<PerformanceBudget>) => void
}
```

**Usage:**
```tsx
import { usePerformanceMonitoring } from '@/utils/performance'

function MyComponent() {
  const { startMeasurement, endMeasurement } = usePerformanceMonitoring()
  
  useEffect(() => {
    const measurementId = startMeasurement('component-render', 'component')
    
    return () => {
      endMeasurement(measurementId)
    }
  }, [])
  
  return <div>Component content</div>
}
```

### useSecurity

Comprehensive security hook for authentication, validation, and secure API calls.

**Returns:**
```typescript
interface SecurityHookReturn {
  securityContext: SecurityContext
  checkRateLimit: (key: string) => boolean
  validateInput: ValidationMethods
  sanitize: SanitizationMethods
  secureInvoke: <T>(command: string, args?: any, options?: any) => Promise<T>
  logEvent: (event: AuditEvent) => Promise<void>
}
```

**Usage:**
```tsx
import { useSecurity } from '@/hooks/useSecurity'

function SecureForm() {
  const { validateInput, sanitize, secureInvoke } = useSecurity()
  
  const handleSubmit = async (formData: any) => {
    // Validate input
    const validation = validateInput.apiKey(formData.key)
    if (!validation.isValid) {
      setErrors(validation.errors)
      return
    }
    
    // Sanitize and submit
    const sanitizedData = {
      name: sanitize.string(formData.name, 100),
      key: validation.sanitized
    }
    
    await secureInvoke('add_api_key', { key: sanitizedData })
  }
}
```

### useTemplateManagement

Hook for managing research templates with full CRUD operations.

**Usage:**
```tsx
import { useTemplateManagement } from '@/hooks/useTemplateManagement'

function TemplateComponent() {
  const {
    templates,
    isLoading,
    createTemplate,
    updateTemplate,
    deleteTemplate,
    executeTemplate
  } = useTemplateManagement('academic', 'AI research')
  
  const handleCreateTemplate = async (templateData: any) => {
    try {
      const newTemplate = await createTemplate(templateData)
      console.log('Template created:', newTemplate.id)
    } catch (error) {
      console.error('Failed to create template:', error)
    }
  }
}
```

## Best Practices

### Component Design

1. **Single Responsibility**: Each component should have one clear purpose
2. **Prop Validation**: Use TypeScript interfaces for all props
3. **Error Boundaries**: Wrap components that might fail
4. **Loading States**: Always provide loading indicators
5. **Accessibility**: Include proper ARIA labels and keyboard navigation

### Performance

1. **Lazy Loading**: Use lazy loading for heavy components
2. **Memoization**: Use React.memo for expensive renders
3. **Code Splitting**: Split large components into smaller chunks
4. **Performance Monitoring**: Monitor component render times

### Security

1. **Input Validation**: Validate all user inputs
2. **Sanitization**: Sanitize data before display
3. **Authentication**: Check authentication state
4. **Rate Limiting**: Implement rate limiting for API calls

### Example: Complete Component

```tsx
import React, { useState, useCallback } from 'react'
import { useSecurity } from '@/hooks/useSecurity'
import { usePerformanceMonitoring } from '@/utils/performance'
import ErrorBoundary from '@/components/common/ErrorBoundary'
import LoadingSpinner from '@/components/common/LoadingSpinner'

interface SecureFormProps {
  onSubmit: (data: any) => Promise<void>
  initialData?: any
  className?: string
}

const SecureForm: React.FC<SecureFormProps> = React.memo(({
  onSubmit,
  initialData,
  className = ''
}) => {
  const [formData, setFormData] = useState(initialData || {})
  const [isLoading, setIsLoading] = useState(false)
  const [errors, setErrors] = useState<string[]>([])
  
  const { validateInput, sanitize, secureInvoke } = useSecurity()
  const { startMeasurement, endMeasurement } = usePerformanceMonitoring()
  
  const handleSubmit = useCallback(async (e: React.FormEvent) => {
    e.preventDefault()
    
    const measurementId = startMeasurement('form-submission', 'user-interaction')
    setIsLoading(true)
    setErrors([])
    
    try {
      // Validate input
      const validation = validateInput.apiKey(formData.key)
      if (!validation.isValid) {
        setErrors(validation.errors)
        return
      }
      
      // Sanitize data
      const sanitizedData = {
        name: sanitize.string(formData.name, 100),
        key: validation.sanitized
      }
      
      // Submit securely
      await onSubmit(sanitizedData)
    } catch (error) {
      setErrors([error instanceof Error ? error.message : 'Unknown error'])
    } finally {
      setIsLoading(false)
      endMeasurement(measurementId)
    }
  }, [formData, onSubmit, validateInput, sanitize, startMeasurement, endMeasurement])
  
  if (isLoading) {
    return <LoadingSpinner message="Submitting form..." />
  }
  
  return (
    <ErrorBoundary>
      <form onSubmit={handleSubmit} className={`space-y-4 ${className}`}>
        {errors.length > 0 && (
          <div className="bg-red-50 border border-red-200 rounded-md p-4">
            <ul className="text-sm text-red-600">
              {errors.map((error, index) => (
                <li key={index}>{error}</li>
              ))}
            </ul>
          </div>
        )}
        
        {/* Form fields */}
        <div>
          <label htmlFor="name" className="block text-sm font-medium text-gray-700">
            Name
          </label>
          <input
            type="text"
            id="name"
            value={formData.name || ''}
            onChange={(e) => setFormData(prev => ({ ...prev, name: e.target.value }))}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
            required
          />
        </div>
        
        <button
          type="submit"
          disabled={isLoading}
          className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          Submit
        </button>
      </form>
    </ErrorBoundary>
  )
})

SecureForm.displayName = 'SecureForm'

export default SecureForm
```

## Testing Components

### Unit Testing Example

```tsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import SecureForm from './SecureForm'

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } }
  })
  
  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  )
}

describe('SecureForm', () => {
  it('validates input before submission', async () => {
    const mockSubmit = jest.fn()
    
    render(
      <SecureForm onSubmit={mockSubmit} />,
      { wrapper: createWrapper() }
    )
    
    const submitButton = screen.getByRole('button', { name: /submit/i })
    fireEvent.click(submitButton)
    
    await waitFor(() => {
      expect(screen.getByText(/validation error/i)).toBeInTheDocument()
    })
    
    expect(mockSubmit).not.toHaveBeenCalled()
  })
})
```
