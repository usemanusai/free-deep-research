import { invoke } from '@tauri-apps/api/core'
import React from 'react'

// Performance monitoring utilities
export interface PerformanceMetrics {
  id: string
  name: string
  startTime: number
  endTime?: number
  duration?: number
  metadata?: Record<string, any>
  category: 'api' | 'component' | 'navigation' | 'render' | 'user-interaction'
  severity: 'low' | 'medium' | 'high' | 'critical'
}

export interface PerformanceBudget {
  api_calls: number // milliseconds
  component_render: number
  navigation: number
  user_interaction: number
  memory_usage: number // MB
  bundle_size: number // KB
}

export interface PerformanceReport {
  timestamp: string
  metrics: PerformanceMetrics[]
  budget_violations: string[]
  recommendations: string[]
  overall_score: number
}

class PerformanceMonitor {
  private metrics: Map<string, PerformanceMetrics> = new Map()
  private observers: PerformanceObserver[] = []
  private budget: PerformanceBudget = {
    api_calls: 2000,
    component_render: 16,
    navigation: 1000,
    user_interaction: 100,
    memory_usage: 100,
    bundle_size: 1000
  }

  constructor() {
    this.initializeObservers()
  }

  private initializeObservers() {
    if (typeof window === 'undefined') return

    // Observe navigation timing
    if ('PerformanceObserver' in window) {
      const navObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.entryType === 'navigation') {
            this.recordMetric({
              id: `nav-${Date.now()}`,
              name: 'page-navigation',
              startTime: entry.startTime,
              endTime: entry.startTime + entry.duration,
              duration: entry.duration,
              category: 'navigation',
              severity: entry.duration > this.budget.navigation ? 'high' : 'low',
              metadata: {
                type: entry.name,
                transferSize: (entry as any).transferSize,
                domContentLoaded: (entry as any).domContentLoadedEventEnd - (entry as any).domContentLoadedEventStart
              }
            })
          }
        }
      })

      try {
        navObserver.observe({ entryTypes: ['navigation'] })
        this.observers.push(navObserver)
      } catch (error) {
        console.warn('Navigation observer not supported:', error)
      }

      // Observe resource timing
      const resourceObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.entryType === 'resource') {
            this.recordMetric({
              id: `resource-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
              name: `resource-${entry.name.split('/').pop() || 'unknown'}`,
              startTime: entry.startTime,
              endTime: entry.startTime + entry.duration,
              duration: entry.duration,
              category: 'api',
              severity: entry.duration > this.budget.api_calls ? 'high' : 'low',
              metadata: {
                url: entry.name,
                transferSize: (entry as any).transferSize,
                initiatorType: (entry as any).initiatorType
              }
            })
          }
        }
      })

      try {
        resourceObserver.observe({ entryTypes: ['resource'] })
        this.observers.push(resourceObserver)
      } catch (error) {
        console.warn('Resource observer not supported:', error)
      }

      // Observe long tasks
      const longTaskObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.entryType === 'longtask') {
            this.recordMetric({
              id: `longtask-${Date.now()}`,
              name: 'long-task',
              startTime: entry.startTime,
              endTime: entry.startTime + entry.duration,
              duration: entry.duration,
              category: 'render',
              severity: 'critical',
              metadata: {
                attribution: (entry as any).attribution
              }
            })
          }
        }
      })

      try {
        longTaskObserver.observe({ entryTypes: ['longtask'] })
        this.observers.push(longTaskObserver)
      } catch (error) {
        console.warn('Long task observer not supported:', error)
      }
    }
  }

  // Start measuring a custom metric
  startMeasurement(name: string, category: PerformanceMetrics['category'], metadata?: Record<string, any>): string {
    const id = `${name}-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
    const metric: PerformanceMetrics = {
      id,
      name,
      startTime: performance.now(),
      category,
      severity: 'low',
      metadata
    }
    
    this.metrics.set(id, metric)
    return id
  }

  // End measuring a custom metric
  endMeasurement(id: string): PerformanceMetrics | null {
    const metric = this.metrics.get(id)
    if (!metric) return null

    const endTime = performance.now()
    const duration = endTime - metric.startTime

    const updatedMetric: PerformanceMetrics = {
      ...metric,
      endTime,
      duration,
      severity: this.calculateSeverity(duration, metric.category)
    }

    this.metrics.set(id, updatedMetric)
    this.recordMetric(updatedMetric)
    
    return updatedMetric
  }

  private calculateSeverity(duration: number, category: PerformanceMetrics['category']): PerformanceMetrics['severity'] {
    const threshold = this.budget[category] || 1000
    
    if (duration > threshold * 2) return 'critical'
    if (duration > threshold * 1.5) return 'high'
    if (duration > threshold) return 'medium'
    return 'low'
  }

  private recordMetric(metric: PerformanceMetrics) {
    // Log to backend if enabled
    if (process.env.NODE_ENV === 'production') {
      invoke('log_performance_metric', { metric }).catch(console.error)
    }

    // Log to console in development
    if (process.env.NODE_ENV === 'development') {
      const color = {
        low: 'green',
        medium: 'yellow',
        high: 'orange',
        critical: 'red'
      }[metric.severity]

      console.log(
        `%c[PERF] ${metric.name}: ${metric.duration?.toFixed(2)}ms`,
        `color: ${color}; font-weight: bold`,
        metric
      )
    }
  }

  // Get performance report
  getReport(): PerformanceReport {
    const metrics = Array.from(this.metrics.values()).filter(m => m.duration !== undefined)
    const budgetViolations: string[] = []
    const recommendations: string[] = []

    // Check budget violations
    metrics.forEach(metric => {
      const threshold = this.budget[metric.category]
      if (metric.duration && metric.duration > threshold) {
        budgetViolations.push(`${metric.name} exceeded budget: ${metric.duration.toFixed(2)}ms > ${threshold}ms`)
      }
    })

    // Generate recommendations
    if (metrics.filter(m => m.category === 'api' && m.severity === 'high').length > 0) {
      recommendations.push('Consider implementing request caching or optimizing API calls')
    }
    
    if (metrics.filter(m => m.category === 'render' && m.severity === 'high').length > 0) {
      recommendations.push('Consider implementing React.memo or useMemo for expensive renders')
    }

    if (metrics.filter(m => m.category === 'component' && m.severity === 'high').length > 0) {
      recommendations.push('Consider code splitting or lazy loading for large components')
    }

    // Calculate overall score (0-100)
    const totalMetrics = metrics.length
    const goodMetrics = metrics.filter(m => m.severity === 'low').length
    const overallScore = totalMetrics > 0 ? Math.round((goodMetrics / totalMetrics) * 100) : 100

    return {
      timestamp: new Date().toISOString(),
      metrics,
      budget_violations: budgetViolations,
      recommendations,
      overall_score: overallScore
    }
  }

  // Clear metrics
  clearMetrics() {
    this.metrics.clear()
  }

  // Update performance budget
  updateBudget(budget: Partial<PerformanceBudget>) {
    this.budget = { ...this.budget, ...budget }
  }

  // Cleanup observers
  disconnect() {
    this.observers.forEach(observer => observer.disconnect())
    this.observers = []
  }
}

// Singleton instance
export const performanceMonitor = new PerformanceMonitor()

// React hook for performance monitoring
export function usePerformanceMonitoring() {
  const startMeasurement = (name: string, category: PerformanceMetrics['category'], metadata?: Record<string, any>) => {
    return performanceMonitor.startMeasurement(name, category, metadata)
  }

  const endMeasurement = (id: string) => {
    return performanceMonitor.endMeasurement(id)
  }

  const getReport = () => {
    return performanceMonitor.getReport()
  }

  return {
    startMeasurement,
    endMeasurement,
    getReport,
    clearMetrics: () => performanceMonitor.clearMetrics(),
    updateBudget: (budget: Partial<PerformanceBudget>) => performanceMonitor.updateBudget(budget)
  }
}

// Higher-order component for measuring component performance
export function withPerformanceMonitoring<P extends object>(
  Component: React.ComponentType<P>,
  componentName?: string
) {
  const WrappedComponent = (props: P) => {
    const { startMeasurement, endMeasurement } = usePerformanceMonitoring()
    
    React.useEffect(() => {
      const measurementId = startMeasurement(
        componentName || Component.displayName || Component.name || 'Unknown',
        'component'
      )
      
      return () => {
        endMeasurement(measurementId)
      }
    }, [])

    return <Component {...props} />
  }

  WrappedComponent.displayName = `withPerformanceMonitoring(${componentName || Component.displayName || Component.name})`
  
  return WrappedComponent
}
