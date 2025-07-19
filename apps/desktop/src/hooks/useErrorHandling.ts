import { useState, useCallback, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

export interface ErrorInfo {
  id: string
  message: string
  code?: string
  severity: 'low' | 'medium' | 'high' | 'critical'
  timestamp: string
  context?: Record<string, any>
  stack?: string
  component?: string
  action?: string
  retryable: boolean
  retryCount: number
  maxRetries: number
}

export interface ErrorHandlingOptions {
  maxRetries?: number
  retryDelay?: number
  logToConsole?: boolean
  logToBackend?: boolean
  showNotification?: boolean
  autoRetry?: boolean
  fallbackValue?: any
}

const DEFAULT_OPTIONS: ErrorHandlingOptions = {
  maxRetries: 3,
  retryDelay: 1000,
  logToConsole: true,
  logToBackend: true,
  showNotification: true,
  autoRetry: false,
  fallbackValue: null
}

// ============================================================================
// ERROR HANDLING HOOK
// ============================================================================

export function useErrorHandling(options: ErrorHandlingOptions = {}) {
  const [errors, setErrors] = useState<ErrorInfo[]>([])
  const [isRetrying, setIsRetrying] = useState(false)

  const config = { ...DEFAULT_OPTIONS, ...options }

  // Generate unique error ID
  const generateErrorId = useCallback(() => {
    return `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }, [])

  // Log error to backend
  const logErrorToBackend = useCallback(async (error: ErrorInfo) => {
    if (!config.logToBackend) return

    try {
      await invoke('log_error', {
        error: {
          id: error.id,
          message: error.message,
          code: error.code,
          severity: error.severity,
          timestamp: error.timestamp,
          context: error.context,
          stack: error.stack,
          component: error.component,
          action: error.action
        }
      })
    } catch (logError) {
      console.error('Failed to log error to backend:', logError)
    }
  }, [config.logToBackend])

  // Show notification
  const showErrorNotification = useCallback((error: ErrorInfo) => {
    if (!config.showNotification) return

    // In a real implementation, you might use a toast library
    // For now, we'll use browser notifications if available
    if ('Notification' in window && Notification.permission === 'granted') {
      new Notification('Application Error', {
        body: error.message,
        icon: '/error-icon.png',
        tag: error.id
      })
    }
  }, [config.showNotification])

  // Handle error with full context
  const handleError = useCallback(async (
    error: Error | string,
    context?: {
      component?: string
      action?: string
      severity?: ErrorInfo['severity']
      retryable?: boolean
      additionalContext?: Record<string, any>
    }
  ) => {
    const errorMessage = typeof error === 'string' ? error : error.message
    const errorStack = typeof error === 'string' ? undefined : error.stack

    const errorInfo: ErrorInfo = {
      id: generateErrorId(),
      message: errorMessage,
      code: typeof error === 'object' && 'code' in error ? (error as any).code : undefined,
      severity: context?.severity || 'medium',
      timestamp: new Date().toISOString(),
      context: context?.additionalContext,
      stack: errorStack,
      component: context?.component,
      action: context?.action,
      retryable: context?.retryable ?? false,
      retryCount: 0,
      maxRetries: config.maxRetries || 3
    }

    // Add to errors list
    setErrors(prev => [...prev, errorInfo])

    // Log to console
    if (config.logToConsole) {
      console.error(`[${errorInfo.severity.toUpperCase()}] ${errorInfo.message}`, {
        error: errorInfo,
        originalError: error
      })
    }

    // Log to backend
    await logErrorToBackend(errorInfo)

    // Show notification
    showErrorNotification(errorInfo)

    return errorInfo
  }, [generateErrorId, config, logErrorToBackend, showErrorNotification])

  // Retry function with exponential backoff
  const retryWithBackoff = useCallback(async (
    fn: () => Promise<any>,
    errorInfo: ErrorInfo
  ): Promise<any> => {
    if (errorInfo.retryCount >= errorInfo.maxRetries) {
      throw new Error(`Max retries (${errorInfo.maxRetries}) exceeded for: ${errorInfo.message}`)
    }

    const delay = (config.retryDelay || 1000) * Math.pow(2, errorInfo.retryCount)
    
    setIsRetrying(true)
    
    try {
      await new Promise(resolve => setTimeout(resolve, delay))
      const result = await fn()
      
      // Update error info to mark as resolved
      setErrors(prev => prev.map(err => 
        err.id === errorInfo.id 
          ? { ...err, retryCount: err.retryCount + 1 }
          : err
      ))
      
      setIsRetrying(false)
      return result
    } catch (retryError) {
      const updatedError = {
        ...errorInfo,
        retryCount: errorInfo.retryCount + 1,
        timestamp: new Date().toISOString()
      }

      setErrors(prev => prev.map(err => 
        err.id === errorInfo.id ? updatedError : err
      ))

      if (updatedError.retryCount >= updatedError.maxRetries) {
        setIsRetrying(false)
        throw retryError
      }

      return retryWithBackoff(fn, updatedError)
    }
  }, [config.retryDelay])

  // Retry specific error
  const retryError = useCallback(async (errorId: string, retryFn: () => Promise<any>) => {
    const error = errors.find(err => err.id === errorId)
    if (!error || !error.retryable) {
      throw new Error('Error not found or not retryable')
    }

    return retryWithBackoff(retryFn, error)
  }, [errors, retryWithBackoff])

  // Clear specific error
  const clearError = useCallback((errorId: string) => {
    setErrors(prev => prev.filter(err => err.id !== errorId))
  }, [])

  // Clear all errors
  const clearAllErrors = useCallback(() => {
    setErrors([])
  }, [])

  // Clear errors by severity
  const clearErrorsBySeverity = useCallback((severity: ErrorInfo['severity']) => {
    setErrors(prev => prev.filter(err => err.severity !== severity))
  }, [])

  // Get errors by severity
  const getErrorsBySeverity = useCallback((severity: ErrorInfo['severity']) => {
    return errors.filter(err => err.severity === severity)
  }, [errors])

  // Get retryable errors
  const getRetryableErrors = useCallback(() => {
    return errors.filter(err => err.retryable && err.retryCount < err.maxRetries)
  }, [errors])

  // Async error handler wrapper
  const withErrorHandling = useCallback(<T extends any[], R>(
    fn: (...args: T) => Promise<R>,
    context?: {
      component?: string
      action?: string
      severity?: ErrorInfo['severity']
      retryable?: boolean
      fallbackValue?: R
    }
  ) => {
    return async (...args: T): Promise<R> => {
      try {
        return await fn(...args)
      } catch (error) {
        const errorInfo = await handleError(error as Error, context)
        
        if (context?.retryable && config.autoRetry) {
          try {
            return await retryWithBackoff(() => fn(...args), errorInfo)
          } catch (retryError) {
            if (context.fallbackValue !== undefined) {
              return context.fallbackValue
            }
            throw retryError
          }
        }
        
        if (context?.fallbackValue !== undefined) {
          return context.fallbackValue
        }
        
        throw error
      }
    }
  }, [handleError, retryWithBackoff, config.autoRetry])

  // Effect to request notification permission
  useEffect(() => {
    if (config.showNotification && 'Notification' in window && Notification.permission === 'default') {
      Notification.requestPermission()
    }
  }, [config.showNotification])

  return {
    // State
    errors,
    isRetrying,
    hasErrors: errors.length > 0,
    hasCriticalErrors: errors.some(err => err.severity === 'critical'),
    hasRetryableErrors: getRetryableErrors().length > 0,
    
    // Error handling
    handleError,
    withErrorHandling,
    
    // Retry functionality
    retryError,
    retryWithBackoff,
    
    // Error management
    clearError,
    clearAllErrors,
    clearErrorsBySeverity,
    
    // Error queries
    getErrorsBySeverity,
    getRetryableErrors,
    
    // Computed values
    criticalErrors: getErrorsBySeverity('critical'),
    highErrors: getErrorsBySeverity('high'),
    mediumErrors: getErrorsBySeverity('medium'),
    lowErrors: getErrorsBySeverity('low'),
    retryableErrors: getRetryableErrors()
  }
}
