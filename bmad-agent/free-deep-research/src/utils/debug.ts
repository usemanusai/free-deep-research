/**
 * Debug Utilities for Development
 * 
 * Comprehensive debugging tools for the Free Deep Research System,
 * including logging, state inspection, performance monitoring,
 * and development helpers.
 * 
 * Features:
 * - Structured logging with levels and categories
 * - Component state inspection and debugging
 * - Performance profiling and bottleneck detection
 * - API call monitoring and debugging
 * - Error tracking and stack trace analysis
 * - Development-only utilities and helpers
 * 
 * @author Free Deep Research System
 * @version 1.0.0
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// TYPES AND INTERFACES
// ============================================================================

export type LogLevel = 'debug' | 'info' | 'warn' | 'error' | 'trace'
export type LogCategory = 'api' | 'component' | 'performance' | 'security' | 'workflow' | 'general'

export interface LogEntry {
  timestamp: string
  level: LogLevel
  category: LogCategory
  message: string
  data?: any
  stack?: string
  component?: string
  userId?: string
  sessionId?: string
}

export interface DebugConfig {
  enabled: boolean
  logLevel: LogLevel
  categories: LogCategory[]
  persistLogs: boolean
  maxLogEntries: number
  showStackTrace: boolean
  enablePerformanceLogging: boolean
  enableApiLogging: boolean
}

export interface ComponentDebugInfo {
  name: string
  props: Record<string, any>
  state: Record<string, any>
  renderCount: number
  lastRender: string
  performance: {
    averageRenderTime: number
    slowestRender: number
    fastestRender: number
  }
}

export interface ApiCallDebugInfo {
  id: string
  command: string
  args: any
  startTime: number
  endTime?: number
  duration?: number
  success: boolean
  error?: string
  response?: any
}

// ============================================================================
// DEBUG LOGGER CLASS
// ============================================================================

class DebugLogger {
  private config: DebugConfig
  private logs: LogEntry[] = []
  private componentInfo: Map<string, ComponentDebugInfo> = new Map()
  private apiCalls: Map<string, ApiCallDebugInfo> = new Map()

  constructor(config: Partial<DebugConfig> = {}) {
    this.config = {
      enabled: process.env.NODE_ENV === 'development',
      logLevel: 'debug',
      categories: ['api', 'component', 'performance', 'security', 'workflow', 'general'],
      persistLogs: true,
      maxLogEntries: 1000,
      showStackTrace: true,
      enablePerformanceLogging: true,
      enableApiLogging: true,
      ...config
    }

    // Set up global error handler
    if (typeof window !== 'undefined' && this.config.enabled) {
      window.addEventListener('error', this.handleGlobalError.bind(this))
      window.addEventListener('unhandledrejection', this.handleUnhandledRejection.bind(this))
    }
  }

  /**
   * Log a debug message
   */
  debug(message: string, data?: any, category: LogCategory = 'general', component?: string) {
    this.log('debug', category, message, data, component)
  }

  /**
   * Log an info message
   */
  info(message: string, data?: any, category: LogCategory = 'general', component?: string) {
    this.log('info', category, message, data, component)
  }

  /**
   * Log a warning message
   */
  warn(message: string, data?: any, category: LogCategory = 'general', component?: string) {
    this.log('warn', category, message, data, component)
  }

  /**
   * Log an error message
   */
  error(message: string, data?: any, category: LogCategory = 'general', component?: string) {
    this.log('error', category, message, data, component)
  }

  /**
   * Log a trace message
   */
  trace(message: string, data?: any, category: LogCategory = 'general', component?: string) {
    this.log('trace', category, message, data, component)
  }

  /**
   * Core logging method
   */
  private log(
    level: LogLevel,
    category: LogCategory,
    message: string,
    data?: any,
    component?: string
  ) {
    if (!this.config.enabled) return
    if (!this.shouldLog(level, category)) return

    const entry: LogEntry = {
      timestamp: new Date().toISOString(),
      level,
      category,
      message,
      data,
      component,
      stack: this.config.showStackTrace ? new Error().stack : undefined
    }

    this.logs.push(entry)

    // Trim logs if exceeding max entries
    if (this.logs.length > this.config.maxLogEntries) {
      this.logs = this.logs.slice(-this.config.maxLogEntries)
    }

    // Console output with styling
    this.outputToConsole(entry)

    // Persist to backend if enabled
    if (this.config.persistLogs) {
      this.persistLog(entry)
    }
  }

  /**
   * Check if log should be output based on level and category
   */
  private shouldLog(level: LogLevel, category: LogCategory): boolean {
    const levelPriority = { trace: 0, debug: 1, info: 2, warn: 3, error: 4 }
    const configLevelPriority = levelPriority[this.config.logLevel]
    const logLevelPriority = levelPriority[level]

    return (
      logLevelPriority >= configLevelPriority &&
      this.config.categories.includes(category)
    )
  }

  /**
   * Output log entry to console with styling
   */
  private outputToConsole(entry: LogEntry) {
    const colors = {
      debug: '#6B7280',
      info: '#3B82F6',
      warn: '#F59E0B',
      error: '#EF4444',
      trace: '#8B5CF6'
    }

    const categoryColors = {
      api: '#10B981',
      component: '#F59E0B',
      performance: '#8B5CF6',
      security: '#EF4444',
      workflow: '#3B82F6',
      general: '#6B7280'
    }

    const timestamp = new Date(entry.timestamp).toLocaleTimeString()
    const color = colors[entry.level]
    const categoryColor = categoryColors[entry.category]

    console.log(
      `%c[${timestamp}] %c[${entry.level.toUpperCase()}] %c[${entry.category}] %c${entry.message}`,
      `color: #6B7280`,
      `color: ${color}; font-weight: bold`,
      `color: ${categoryColor}; font-weight: bold`,
      `color: #1F2937`,
      entry.data
    )

    if (entry.component) {
      console.log(`%c  Component: ${entry.component}`, 'color: #6B7280; font-style: italic')
    }

    if (entry.data) {
      console.log('  Data:', entry.data)
    }
  }

  /**
   * Persist log to backend
   */
  private async persistLog(entry: LogEntry) {
    try {
      await invoke('log_debug_entry', { entry })
    } catch (error) {
      // Silently fail to avoid infinite loops
    }
  }

  /**
   * Handle global errors
   */
  private handleGlobalError(event: ErrorEvent) {
    this.error('Global Error', {
      message: event.message,
      filename: event.filename,
      lineno: event.lineno,
      colno: event.colno,
      error: event.error
    }, 'general')
  }

  /**
   * Handle unhandled promise rejections
   */
  private handleUnhandledRejection(event: PromiseRejectionEvent) {
    this.error('Unhandled Promise Rejection', {
      reason: event.reason
    }, 'general')
  }

  /**
   * Get all logs
   */
  getLogs(): LogEntry[] {
    return [...this.logs]
  }

  /**
   * Get logs by category
   */
  getLogsByCategory(category: LogCategory): LogEntry[] {
    return this.logs.filter(log => log.category === category)
  }

  /**
   * Get logs by level
   */
  getLogsByLevel(level: LogLevel): LogEntry[] {
    return this.logs.filter(log => log.level === level)
  }

  /**
   * Clear all logs
   */
  clearLogs() {
    this.logs = []
  }

  /**
   * Export logs as JSON
   */
  exportLogs(): string {
    return JSON.stringify(this.logs, null, 2)
  }

  /**
   * Update debug configuration
   */
  updateConfig(config: Partial<DebugConfig>) {
    this.config = { ...this.config, ...config }
  }

  /**
   * Register component for debugging
   */
  registerComponent(name: string, props: any, state: any) {
    if (!this.config.enabled) return

    const existing = this.componentInfo.get(name)
    const info: ComponentDebugInfo = {
      name,
      props,
      state,
      renderCount: existing ? existing.renderCount + 1 : 1,
      lastRender: new Date().toISOString(),
      performance: existing?.performance || {
        averageRenderTime: 0,
        slowestRender: 0,
        fastestRender: Infinity
      }
    }

    this.componentInfo.set(name, info)
    this.debug(`Component ${name} rendered`, { renderCount: info.renderCount }, 'component', name)
  }

  /**
   * Track API call
   */
  trackApiCall(command: string, args: any): string {
    if (!this.config.enableApiLogging) return ''

    const id = `api_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
    const callInfo: ApiCallDebugInfo = {
      id,
      command,
      args,
      startTime: performance.now(),
      success: false
    }

    this.apiCalls.set(id, callInfo)
    this.debug(`API Call Started: ${command}`, args, 'api')

    return id
  }

  /**
   * Complete API call tracking
   */
  completeApiCall(id: string, success: boolean, response?: any, error?: string) {
    if (!this.config.enableApiLogging || !id) return

    const callInfo = this.apiCalls.get(id)
    if (!callInfo) return

    callInfo.endTime = performance.now()
    callInfo.duration = callInfo.endTime - callInfo.startTime
    callInfo.success = success
    callInfo.response = response
    callInfo.error = error

    const level = success ? 'info' : 'error'
    const message = `API Call ${success ? 'Completed' : 'Failed'}: ${callInfo.command}`
    
    this.log(level, 'api', message, {
      duration: `${callInfo.duration.toFixed(2)}ms`,
      success,
      response: success ? response : undefined,
      error: error || undefined
    })
  }

  /**
   * Get component debug info
   */
  getComponentInfo(): ComponentDebugInfo[] {
    return Array.from(this.componentInfo.values())
  }

  /**
   * Get API call history
   */
  getApiCallHistory(): ApiCallDebugInfo[] {
    return Array.from(this.apiCalls.values())
  }
}

// ============================================================================
// GLOBAL DEBUG INSTANCE
// ============================================================================

export const debugLogger = new DebugLogger()

// ============================================================================
// REACT HOOKS FOR DEBUGGING
// ============================================================================

/**
 * Hook for component debugging
 */
export function useDebugComponent(name: string, props: any, state?: any) {
  React.useEffect(() => {
    debugLogger.registerComponent(name, props, state)
  })

  return {
    log: (message: string, data?: any, level: LogLevel = 'debug') => {
      debugLogger.log(level, 'component', message, data, name)
    }
  }
}

/**
 * Hook for API call debugging
 */
export function useDebugApi() {
  return {
    trackCall: (command: string, args: any) => debugLogger.trackApiCall(command, args),
    completeCall: (id: string, success: boolean, response?: any, error?: string) => 
      debugLogger.completeApiCall(id, success, response, error)
  }
}

// ============================================================================
// DEVELOPMENT UTILITIES
// ============================================================================

/**
 * Add debug information to window object for console access
 */
if (typeof window !== 'undefined' && process.env.NODE_ENV === 'development') {
  (window as any).__DEBUG__ = {
    logger: debugLogger,
    getLogs: () => debugLogger.getLogs(),
    clearLogs: () => debugLogger.clearLogs(),
    exportLogs: () => debugLogger.exportLogs(),
    getComponents: () => debugLogger.getComponentInfo(),
    getApiCalls: () => debugLogger.getApiCallHistory(),
    config: (config: Partial<DebugConfig>) => debugLogger.updateConfig(config)
  }
}

/**
 * Performance measurement utility
 */
export function measurePerformance<T>(name: string, fn: () => T): T {
  const start = performance.now()
  const result = fn()
  const end = performance.now()
  
  debugLogger.debug(`Performance: ${name}`, {
    duration: `${(end - start).toFixed(2)}ms`
  }, 'performance')
  
  return result
}

/**
 * Async performance measurement utility
 */
export async function measureAsyncPerformance<T>(name: string, fn: () => Promise<T>): Promise<T> {
  const start = performance.now()
  const result = await fn()
  const end = performance.now()
  
  debugLogger.debug(`Async Performance: ${name}`, {
    duration: `${(end - start).toFixed(2)}ms`
  }, 'performance')
  
  return result
}

/**
 * Debug wrapper for API calls
 */
export async function debugApiCall<T>(command: string, args?: any): Promise<T> {
  const callId = debugLogger.trackApiCall(command, args)
  
  try {
    const result = await invoke<T>(command, args)
    debugLogger.completeApiCall(callId, true, result)
    return result
  } catch (error) {
    debugLogger.completeApiCall(callId, false, undefined, error instanceof Error ? error.message : String(error))
    throw error
  }
}

export default debugLogger
