import React, { Suspense, lazy, ComponentType } from 'react'
import LoadingSpinner from './LoadingSpinner'
import ErrorBoundary from './ErrorBoundary'

interface LazyWrapperProps {
  fallback?: React.ReactNode
  errorFallback?: React.ReactNode
  children: React.ReactNode
}

// Generic lazy wrapper component
export function LazyWrapper({ 
  fallback = <LoadingSpinner message="Loading component..." />, 
  errorFallback,
  children 
}: LazyWrapperProps) {
  return (
    <ErrorBoundary fallback={errorFallback}>
      <Suspense fallback={fallback}>
        {children}
      </Suspense>
    </ErrorBoundary>
  )
}

// Higher-order component for lazy loading
export function withLazyLoading<P extends object>(
  importFn: () => Promise<{ default: ComponentType<P> }>,
  options?: {
    fallback?: React.ReactNode
    errorFallback?: React.ReactNode
    displayName?: string
  }
) {
  const LazyComponent = lazy(importFn)
  
  const WrappedComponent = (props: P) => (
    <LazyWrapper 
      fallback={options?.fallback}
      errorFallback={options?.errorFallback}
    >
      <LazyComponent {...props} />
    </LazyWrapper>
  )

  WrappedComponent.displayName = options?.displayName || 'LazyComponent'
  
  return WrappedComponent
}

// Preload function for lazy components
export function preloadComponent(importFn: () => Promise<any>) {
  return importFn()
}

// Hook for intersection observer-based lazy loading
export function useIntersectionObserver(
  ref: React.RefObject<Element>,
  options?: IntersectionObserverInit
) {
  const [isIntersecting, setIsIntersecting] = React.useState(false)
  const [hasIntersected, setHasIntersected] = React.useState(false)

  React.useEffect(() => {
    const element = ref.current
    if (!element) return

    const observer = new IntersectionObserver(
      ([entry]) => {
        setIsIntersecting(entry.isIntersecting)
        if (entry.isIntersecting && !hasIntersected) {
          setHasIntersected(true)
        }
      },
      {
        threshold: 0.1,
        rootMargin: '50px',
        ...options
      }
    )

    observer.observe(element)

    return () => {
      observer.unobserve(element)
    }
  }, [ref, hasIntersected, options])

  return { isIntersecting, hasIntersected }
}

// Lazy image component with intersection observer
interface LazyImageProps extends React.ImgHTMLAttributes<HTMLImageElement> {
  src: string
  alt: string
  placeholder?: string
  className?: string
  onLoad?: () => void
  onError?: () => void
}

export function LazyImage({ 
  src, 
  alt, 
  placeholder = 'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAwIiBoZWlnaHQ9IjIwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiBmaWxsPSIjZGRkIi8+PHRleHQgeD0iNTAlIiB5PSI1MCUiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzk5OSIgdGV4dC1hbmNob3I9Im1pZGRsZSIgZHk9Ii4zZW0iPkxvYWRpbmcuLi48L3RleHQ+PC9zdmc+',
  className = '',
  onLoad,
  onError,
  ...props 
}: LazyImageProps) {
  const imgRef = React.useRef<HTMLImageElement>(null)
  const { hasIntersected } = useIntersectionObserver(imgRef)
  const [isLoaded, setIsLoaded] = React.useState(false)
  const [hasError, setHasError] = React.useState(false)

  const handleLoad = () => {
    setIsLoaded(true)
    onLoad?.()
  }

  const handleError = () => {
    setHasError(true)
    onError?.()
  }

  return (
    <img
      ref={imgRef}
      src={hasIntersected ? src : placeholder}
      alt={alt}
      className={`transition-opacity duration-300 ${isLoaded ? 'opacity-100' : 'opacity-50'} ${className}`}
      onLoad={handleLoad}
      onError={handleError}
      {...props}
    />
  )
}

// Lazy content component for any content
interface LazyContentProps {
  children: React.ReactNode
  fallback?: React.ReactNode
  className?: string
  threshold?: number
  rootMargin?: string
}

export function LazyContent({ 
  children, 
  fallback = <div className="animate-pulse bg-gray-200 h-32 rounded" />,
  className = '',
  threshold = 0.1,
  rootMargin = '50px'
}: LazyContentProps) {
  const contentRef = React.useRef<HTMLDivElement>(null)
  const { hasIntersected } = useIntersectionObserver(contentRef, { threshold, rootMargin })

  return (
    <div ref={contentRef} className={className}>
      {hasIntersected ? children : fallback}
    </div>
  )
}

// Lazy route component
interface LazyRouteProps {
  component: () => Promise<{ default: ComponentType<any> }>
  fallback?: React.ReactNode
  errorFallback?: React.ReactNode
  preload?: boolean
}

export function LazyRoute({ 
  component, 
  fallback = <LoadingSpinner message="Loading page..." />,
  errorFallback,
  preload = false
}: LazyRouteProps) {
  const LazyComponent = React.useMemo(() => lazy(component), [component])

  // Preload component if requested
  React.useEffect(() => {
    if (preload) {
      preloadComponent(component)
    }
  }, [component, preload])

  return (
    <LazyWrapper fallback={fallback} errorFallback={errorFallback}>
      <LazyComponent />
    </LazyWrapper>
  )
}

// Bundle splitting utilities
export const lazyComponents = {
  // Analytics components
  Analytics: lazy(() => import('@/components/analytics')),
  UsageAnalytics: lazy(() => import('@/components/analytics/UsageAnalytics')),
  BusinessReports: lazy(() => import('@/components/analytics/BusinessReports')),
  PredictiveAnalytics: lazy(() => import('@/components/analytics/PredictiveAnalytics')),
  
  // Research components
  ResearchWorkflow: lazy(() => import('@/components/research/ResearchWorkflow')),
  ResearchWorkflowDashboard: lazy(() => import('@/components/research/ResearchWorkflowDashboard')),
  
  // Template components
  TemplateManagement: lazy(() => import('@/components/templates/TemplateManagementDashboard')),
  
  // Settings components
  Settings: lazy(() => import('@/components/settings/ConfigurationPanel')),
  
  // Monitoring components
  MonitoringConsole: lazy(() => import('@/components/monitoring/RealTimeConsole')),

  // BMAD Integration components
  BMadIntegration: lazy(() => import('@/components/bmad-integration')),
}

// Preload critical components
export function preloadCriticalComponents() {
  // Preload components that are likely to be used soon
  preloadComponent(() => import('@/components/dashboard/ExecutiveDashboard'))
  preloadComponent(() => import('@/components/api-management/ApiKeyManager'))
}

// Preload components on user interaction
export function preloadOnHover(componentImport: () => Promise<any>) {
  return {
    onMouseEnter: () => preloadComponent(componentImport),
    onFocus: () => preloadComponent(componentImport)
  }
}

// Route-based code splitting
export const routes = {
  Dashboard: withLazyLoading(
    () => import('@/components/dashboard/ExecutiveDashboard'),
    { displayName: 'Dashboard' }
  ),
  ApiKeys: withLazyLoading(
    () => import('@/components/api-management/ApiKeyManager'),
    { displayName: 'ApiKeys' }
  ),
  Research: withLazyLoading(
    () => import('@/components/research/ResearchWorkflow'),
    { displayName: 'Research' }
  ),
  ResearchDashboard: withLazyLoading(
    () => import('@/components/research/ResearchWorkflowDashboard'),
    { displayName: 'ResearchDashboard' }
  ),
  Templates: withLazyLoading(
    () => import('@/components/templates/TemplateManagementDashboard'),
    { displayName: 'Templates' }
  ),
  Monitoring: withLazyLoading(
    () => import('@/components/monitoring/RealTimeConsole'),
    { displayName: 'Monitoring' }
  ),
  Analytics: withLazyLoading(
    () => import('@/components/analytics'),
    { displayName: 'Analytics' }
  ),
  Settings: withLazyLoading(
    () => import('@/components/settings/ConfigurationPanel'),
    { displayName: 'Settings' }
  ),
  BMadIntegration: withLazyLoading(
    () => import('@/components/bmad-integration'),
    { displayName: 'BMadIntegration' }
  )
}

export default LazyWrapper
