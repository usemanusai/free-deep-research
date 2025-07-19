import React, { useState, useCallback } from 'react'
import {
  ExclamationTriangleIcon,
  XCircleIcon,
  InformationCircleIcon,
  CheckCircleIcon,
  ClipboardDocumentIcon,
  ChevronDownIcon,
  ChevronUpIcon,
  ArrowPathIcon,
  BugAntIcon
} from '@heroicons/react/24/outline'
import { cn } from '@/utils/codeQuality'
import { debugLogger } from '@/utils/debug'

// ============================================================================
// TYPES AND INTERFACES
// ============================================================================

export type ErrorSeverity = 'info' | 'warning' | 'error' | 'critical'

export interface ErrorContext {
  component?: string
  action?: string
  timestamp?: string
  userId?: string
  sessionId?: string
  requestId?: string
  stackTrace?: string
  additionalData?: Record<string, any>
}

export interface ErrorSolution {
  title: string
  description: string
  action?: {
    label: string
    onClick: () => void
  }
}

export interface EnhancedErrorAlertProps {
  title: string
  message: string
  severity?: ErrorSeverity
  context?: ErrorContext
  solutions?: ErrorSolution[]
  onRetry?: () => void
  onDismiss?: () => void
  onReport?: (errorData: any) => void
  showDetails?: boolean
  className?: string
  persistent?: boolean
}

// ============================================================================
// ERROR ALERT COMPONENT
// ============================================================================

export function EnhancedErrorAlert({
  title,
  message,
  severity = 'error',
  context,
  solutions = [],
  onRetry,
  onDismiss,
  onReport,
  showDetails = false,
  className = '',
  persistent = false
}: EnhancedErrorAlertProps) {
  const [isExpanded, setIsExpanded] = useState(showDetails)
  const [isCopied, setIsCopied] = useState(false)

  // Get appropriate icon and colors based on severity
  const getIconAndColors = (severity: ErrorSeverity) => {
    switch (severity) {
      case 'info':
        return {
          icon: InformationCircleIcon,
          bgColor: 'bg-blue-50',
          borderColor: 'border-blue-200',
          iconColor: 'text-blue-600',
          textColor: 'text-blue-800'
        }
      case 'warning':
        return {
          icon: ExclamationTriangleIcon,
          bgColor: 'bg-yellow-50',
          borderColor: 'border-yellow-200',
          iconColor: 'text-yellow-600',
          textColor: 'text-yellow-800'
        }
      case 'error':
        return {
          icon: XCircleIcon,
          bgColor: 'bg-red-50',
          borderColor: 'border-red-200',
          iconColor: 'text-red-600',
          textColor: 'text-red-800'
        }
      case 'critical':
        return {
          icon: XCircleIcon,
          bgColor: 'bg-red-100',
          borderColor: 'border-red-300',
          iconColor: 'text-red-700',
          textColor: 'text-red-900'
        }
    }
  }

  const { icon: Icon, bgColor, borderColor, iconColor, textColor } = getIconAndColors(severity)

  // Copy error details to clipboard
  const copyErrorDetails = useCallback(async () => {
    const errorDetails = {
      title,
      message,
      severity,
      context,
      timestamp: new Date().toISOString()
    }

    try {
      await navigator.clipboard.writeText(JSON.stringify(errorDetails, null, 2))
      setIsCopied(true)
      setTimeout(() => setIsCopied(false), 2000)
      
      debugLogger.debug('Error details copied to clipboard', errorDetails, 'general')
    } catch (error) {
      debugLogger.error('Failed to copy error details', error, 'general')
    }
  }, [title, message, severity, context])

  // Report error to development team
  const reportError = useCallback(() => {
    const errorData = {
      title,
      message,
      severity,
      context,
      timestamp: new Date().toISOString(),
      userAgent: navigator.userAgent,
      url: window.location.href
    }

    debugLogger.error('Error reported by user', errorData, 'general')
    
    if (onReport) {
      onReport(errorData)
    }
  }, [title, message, severity, context, onReport])

  // Generate user-friendly error ID
  const errorId = React.useMemo(() => {
    const timestamp = Date.now().toString(36)
    const random = Math.random().toString(36).substr(2, 5)
    return `ERR-${timestamp}-${random}`.toUpperCase()
  }, [])

  return (
    <div className={cn(
      'rounded-lg border p-4',
      bgColor,
      borderColor,
      className
    )}>
      {/* Header */}
      <div className="flex items-start">
        <div className="flex-shrink-0">
          <Icon className={cn('h-5 w-5', iconColor)} />
        </div>
        
        <div className="ml-3 flex-1">
          <h3 className={cn('text-sm font-medium', textColor)}>
            {title}
          </h3>
          
          <div className={cn('mt-2 text-sm', textColor)}>
            <p>{message}</p>
          </div>

          {/* Error ID */}
          <div className="mt-2 text-xs text-gray-500">
            Error ID: {errorId}
          </div>

          {/* Solutions */}
          {solutions.length > 0 && (
            <div className="mt-4">
              <h4 className={cn('text-sm font-medium mb-2', textColor)}>
                Suggested Solutions:
              </h4>
              <ul className="space-y-2">
                {solutions.map((solution, index) => (
                  <li key={index} className="flex items-start">
                    <CheckCircleIcon className="h-4 w-4 text-green-500 mt-0.5 mr-2 flex-shrink-0" />
                    <div className="flex-1">
                      <p className={cn('text-sm font-medium', textColor)}>
                        {solution.title}
                      </p>
                      <p className="text-sm text-gray-600 mt-1">
                        {solution.description}
                      </p>
                      {solution.action && (
                        <button
                          onClick={solution.action.onClick}
                          className="mt-2 text-sm text-blue-600 hover:text-blue-800 underline"
                        >
                          {solution.action.label}
                        </button>
                      )}
                    </div>
                  </li>
                ))}
              </ul>
            </div>
          )}

          {/* Action Buttons */}
          <div className="mt-4 flex flex-wrap gap-2">
            {onRetry && (
              <button
                onClick={onRetry}
                className="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
              >
                <ArrowPathIcon className="h-3 w-3 mr-1" />
                Retry
              </button>
            )}
            
            <button
              onClick={copyErrorDetails}
              className="inline-flex items-center px-3 py-1.5 border border-gray-300 text-xs font-medium rounded text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              <ClipboardDocumentIcon className="h-3 w-3 mr-1" />
              {isCopied ? 'Copied!' : 'Copy Details'}
            </button>
            
            <button
              onClick={reportError}
              className="inline-flex items-center px-3 py-1.5 border border-gray-300 text-xs font-medium rounded text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              <BugAntIcon className="h-3 w-3 mr-1" />
              Report Issue
            </button>
            
            {context && (
              <button
                onClick={() => setIsExpanded(!isExpanded)}
                className="inline-flex items-center px-3 py-1.5 border border-gray-300 text-xs font-medium rounded text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
              >
                {isExpanded ? (
                  <>
                    <ChevronUpIcon className="h-3 w-3 mr-1" />
                    Hide Details
                  </>
                ) : (
                  <>
                    <ChevronDownIcon className="h-3 w-3 mr-1" />
                    Show Details
                  </>
                )}
              </button>
            )}
          </div>

          {/* Expanded Details */}
          {isExpanded && context && (
            <div className="mt-4 p-3 bg-gray-50 rounded border">
              <h4 className="text-sm font-medium text-gray-900 mb-2">
                Error Details
              </h4>
              
              <dl className="space-y-2 text-sm">
                {context.component && (
                  <div>
                    <dt className="font-medium text-gray-700">Component:</dt>
                    <dd className="text-gray-600">{context.component}</dd>
                  </div>
                )}
                
                {context.action && (
                  <div>
                    <dt className="font-medium text-gray-700">Action:</dt>
                    <dd className="text-gray-600">{context.action}</dd>
                  </div>
                )}
                
                {context.timestamp && (
                  <div>
                    <dt className="font-medium text-gray-700">Timestamp:</dt>
                    <dd className="text-gray-600">{new Date(context.timestamp).toLocaleString()}</dd>
                  </div>
                )}
                
                {context.requestId && (
                  <div>
                    <dt className="font-medium text-gray-700">Request ID:</dt>
                    <dd className="text-gray-600 font-mono text-xs">{context.requestId}</dd>
                  </div>
                )}
                
                {context.additionalData && Object.keys(context.additionalData).length > 0 && (
                  <div>
                    <dt className="font-medium text-gray-700">Additional Data:</dt>
                    <dd className="text-gray-600">
                      <pre className="text-xs bg-white p-2 rounded border overflow-auto max-h-32">
                        {JSON.stringify(context.additionalData, null, 2)}
                      </pre>
                    </dd>
                  </div>
                )}
                
                {context.stackTrace && (
                  <div>
                    <dt className="font-medium text-gray-700">Stack Trace:</dt>
                    <dd className="text-gray-600">
                      <pre className="text-xs bg-white p-2 rounded border overflow-auto max-h-32">
                        {context.stackTrace}
                      </pre>
                    </dd>
                  </div>
                )}
              </dl>
            </div>
          )}
        </div>

        {/* Dismiss Button */}
        {!persistent && onDismiss && (
          <div className="ml-auto pl-3">
            <div className="-mx-1.5 -my-1.5">
              <button
                onClick={onDismiss}
                className={cn(
                  'inline-flex rounded-md p-1.5 focus:outline-none focus:ring-2 focus:ring-offset-2',
                  severity === 'error' || severity === 'critical'
                    ? 'text-red-500 hover:bg-red-100 focus:ring-red-600'
                    : severity === 'warning'
                    ? 'text-yellow-500 hover:bg-yellow-100 focus:ring-yellow-600'
                    : 'text-blue-500 hover:bg-blue-100 focus:ring-blue-600'
                )}
              >
                <span className="sr-only">Dismiss</span>
                <XCircleIcon className="h-5 w-5" />
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

// ============================================================================
// ERROR BOUNDARY WITH ENHANCED ALERTS
// ============================================================================

interface EnhancedErrorBoundaryProps {
  children: React.ReactNode
  fallback?: React.ComponentType<{ error: Error; errorInfo: React.ErrorInfo }>
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void
}

interface EnhancedErrorBoundaryState {
  hasError: boolean
  error: Error | null
  errorInfo: React.ErrorInfo | null
}

export class EnhancedErrorBoundary extends React.Component<
  EnhancedErrorBoundaryProps,
  EnhancedErrorBoundaryState
> {
  constructor(props: EnhancedErrorBoundaryProps) {
    super(props)
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null
    }
  }

  static getDerivedStateFromError(error: Error): EnhancedErrorBoundaryState {
    return {
      hasError: true,
      error,
      errorInfo: null
    }
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    this.setState({
      error,
      errorInfo
    })

    // Log to debug system
    debugLogger.error('React Error Boundary caught error', {
      error: error.message,
      stack: error.stack,
      componentStack: errorInfo.componentStack
    }, 'component')

    // Call custom error handler
    if (this.props.onError) {
      this.props.onError(error, errorInfo)
    }
  }

  render() {
    if (this.state.hasError && this.state.error) {
      // Custom fallback component
      if (this.props.fallback) {
        const FallbackComponent = this.props.fallback
        return <FallbackComponent error={this.state.error} errorInfo={this.state.errorInfo!} />
      }

      // Default enhanced error display
      return (
        <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
          <div className="max-w-md w-full">
            <EnhancedErrorAlert
              title="Application Error"
              message="An unexpected error occurred in the application. Please try refreshing the page or contact support if the problem persists."
              severity="critical"
              context={{
                component: 'ErrorBoundary',
                action: 'render',
                timestamp: new Date().toISOString(),
                stackTrace: this.state.error.stack,
                additionalData: {
                  errorMessage: this.state.error.message,
                  componentStack: this.state.errorInfo?.componentStack
                }
              }}
              solutions={[
                {
                  title: 'Refresh the page',
                  description: 'Sometimes a simple page refresh can resolve temporary issues.',
                  action: {
                    label: 'Refresh Now',
                    onClick: () => window.location.reload()
                  }
                },
                {
                  title: 'Clear browser cache',
                  description: 'Clearing your browser cache may resolve issues with cached resources.'
                },
                {
                  title: 'Contact support',
                  description: 'If the problem persists, please report this issue to our support team.'
                }
              ]}
              onRetry={() => {
                this.setState({
                  hasError: false,
                  error: null,
                  errorInfo: null
                })
              }}
              persistent
            />
          </div>
        </div>
      )
    }

    return this.props.children
  }
}

export default EnhancedErrorAlert
