import { useState, useCallback, useEffect } from 'react'
import { 
  rateLimiter, 
  sessionManager, 
  csrfTokenManager, 
  logSecurityEvent, 
  secureApiCall,
  initializeSecurity,
  type SecurityConfig,
  type AuditEvent
} from '@/utils/security'
import { 
  validateApiKey, 
  validateEmail, 
  validateUrl, 
  validateWorkflowParameters,
  validateTemplateData,
  sanitizeHtml,
  sanitizeString,
  type ValidationResult
} from '@/utils/validation'

// Security context interface
export interface SecurityContext {
  isAuthenticated: boolean
  sessionId: string | null
  csrfToken: string | null
  permissions: string[]
  riskLevel: 'low' | 'medium' | 'high' | 'critical'
}

// Security hook return type
export interface SecurityHookReturn {
  // Authentication state
  securityContext: SecurityContext
  isSecurityInitialized: boolean
  
  // Rate limiting
  checkRateLimit: (key: string) => boolean
  getRemainingRequests: (key: string) => number
  
  // Session management
  createSession: (data: any) => string
  getSession: (sessionId: string) => any | null
  updateSession: (sessionId: string, data: any) => boolean
  destroySession: (sessionId: string) => boolean
  
  // CSRF protection
  generateCSRFToken: () => string
  validateCSRFToken: (token: string) => boolean
  
  // Input validation
  validateInput: {
    apiKey: (key: string) => ValidationResult
    email: (email: string) => ValidationResult
    url: (url: string) => ValidationResult
    workflowParams: (params: Record<string, any>) => ValidationResult
    templateData: (template: any) => ValidationResult
  }
  
  // Sanitization
  sanitize: {
    html: (input: string) => string
    string: (input: string, maxLength?: number) => string
  }
  
  // Secure API calls
  secureInvoke: <T>(command: string, args?: Record<string, any>, options?: {
    requireAuth?: boolean
    rateLimitKey?: string
    auditEvent?: Partial<AuditEvent>
  }) => Promise<T>
  
  // Audit logging
  logEvent: (event: Omit<AuditEvent, 'timestamp'>) => Promise<void>
  
  // Security configuration
  updateSecurityConfig: (config: Partial<SecurityConfig>) => void
}

// Default security context
const DEFAULT_SECURITY_CONTEXT: SecurityContext = {
  isAuthenticated: false,
  sessionId: null,
  csrfToken: null,
  permissions: [],
  riskLevel: 'low'
}

export function useSecurity(initialConfig?: Partial<SecurityConfig>): SecurityHookReturn {
  const [securityContext, setSecurityContext] = useState<SecurityContext>(DEFAULT_SECURITY_CONTEXT)
  const [isSecurityInitialized, setIsSecurityInitialized] = useState(false)

  // Initialize security on mount
  useEffect(() => {
    const initialize = async () => {
      try {
        initializeSecurity(initialConfig)
        
        // Generate initial CSRF token
        const csrfToken = csrfTokenManager.generateToken()
        
        setSecurityContext(prev => ({
          ...prev,
          csrfToken
        }))
        
        setIsSecurityInitialized(true)
        
        await logSecurityEvent({
          event_type: 'security_hook_initialized',
          action: 'initialize_security_hook',
          result: 'success',
          risk_level: 'low'
        })
      } catch (error) {
        console.error('Failed to initialize security:', error)
        
        await logSecurityEvent({
          event_type: 'security_initialization_failed',
          action: 'initialize_security_hook',
          result: 'error',
          risk_level: 'high',
          details: {
            error: error instanceof Error ? error.message : 'Unknown error'
          }
        })
      }
    }

    initialize()
  }, [initialConfig])

  // Rate limiting functions
  const checkRateLimit = useCallback((key: string): boolean => {
    const allowed = rateLimiter.isAllowed(key)
    
    if (!allowed) {
      logSecurityEvent({
        event_type: 'rate_limit_exceeded',
        action: 'check_rate_limit',
        result: 'failure',
        risk_level: 'medium',
        details: { rateLimitKey: key }
      })
    }
    
    return allowed
  }, [])

  const getRemainingRequests = useCallback((key: string): number => {
    return rateLimiter.getRemaining(key)
  }, [])

  // Session management functions
  const createSession = useCallback((data: any): string => {
    const sessionId = `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
    sessionManager.createSession(sessionId, data)
    
    setSecurityContext(prev => ({
      ...prev,
      sessionId,
      isAuthenticated: true
    }))
    
    logSecurityEvent({
      event_type: 'session_created',
      action: 'create_session',
      result: 'success',
      risk_level: 'low',
      details: { sessionId }
    })
    
    return sessionId
  }, [])

  const getSession = useCallback((sessionId: string): any | null => {
    return sessionManager.getSession(sessionId)
  }, [])

  const updateSession = useCallback((sessionId: string, data: any): boolean => {
    const success = sessionManager.updateSession(sessionId, data)
    
    if (success) {
      logSecurityEvent({
        event_type: 'session_updated',
        action: 'update_session',
        result: 'success',
        risk_level: 'low',
        details: { sessionId }
      })
    }
    
    return success
  }, [])

  const destroySession = useCallback((sessionId: string): boolean => {
    const success = sessionManager.deleteSession(sessionId)
    
    if (success && securityContext.sessionId === sessionId) {
      setSecurityContext(prev => ({
        ...prev,
        sessionId: null,
        isAuthenticated: false,
        permissions: []
      }))
    }
    
    logSecurityEvent({
      event_type: 'session_destroyed',
      action: 'destroy_session',
      result: success ? 'success' : 'failure',
      risk_level: 'low',
      details: { sessionId }
    })
    
    return success
  }, [securityContext.sessionId])

  // CSRF protection functions
  const generateCSRFToken = useCallback((): string => {
    const token = csrfTokenManager.generateToken()
    
    setSecurityContext(prev => ({
      ...prev,
      csrfToken: token
    }))
    
    return token
  }, [])

  const validateCSRFToken = useCallback((token: string): boolean => {
    const isValid = csrfTokenManager.validateToken(token)
    
    if (!isValid) {
      logSecurityEvent({
        event_type: 'csrf_validation_failed',
        action: 'validate_csrf_token',
        result: 'failure',
        risk_level: 'high',
        details: { token: token.substring(0, 8) + '...' }
      })
    }
    
    return isValid
  }, [])

  // Input validation functions
  const validateInput = {
    apiKey: validateApiKey,
    email: validateEmail,
    url: validateUrl,
    workflowParams: validateWorkflowParameters,
    templateData: validateTemplateData
  }

  // Sanitization functions
  const sanitize = {
    html: sanitizeHtml,
    string: sanitizeString
  }

  // Secure API call wrapper
  const secureInvoke = useCallback(async <T>(
    command: string,
    args?: Record<string, any>,
    options?: {
      requireAuth?: boolean
      rateLimitKey?: string
      auditEvent?: Partial<AuditEvent>
    }
  ): Promise<T> => {
    const { requireAuth = true, rateLimitKey, auditEvent } = options || {}
    
    // Check authentication if required
    if (requireAuth && !securityContext.isAuthenticated) {
      const error = new Error('Authentication required')
      
      await logSecurityEvent({
        event_type: 'unauthorized_access_attempt',
        action: command,
        result: 'failure',
        risk_level: 'high',
        details: { command, args }
      })
      
      throw error
    }
    
    // Use rate limiting key based on session or command
    const effectiveRateLimitKey = rateLimitKey || securityContext.sessionId || command
    
    return secureApiCall<T>(command, args, {
      requireAuth,
      rateLimitKey: effectiveRateLimitKey,
      auditEvent: {
        ...auditEvent,
        user_id: securityContext.sessionId || undefined
      }
    })
  }, [securityContext])

  // Audit logging function
  const logEvent = useCallback(async (event: Omit<AuditEvent, 'timestamp'>): Promise<void> => {
    return logSecurityEvent({
      ...event,
      user_id: event.user_id || securityContext.sessionId || undefined
    })
  }, [securityContext.sessionId])

  // Security configuration update
  const updateSecurityConfig = useCallback((config: Partial<SecurityConfig>): void => {
    initializeSecurity(config)
    
    logSecurityEvent({
      event_type: 'security_config_updated',
      action: 'update_security_config',
      result: 'success',
      risk_level: 'medium',
      details: { config }
    })
  }, [])

  return {
    securityContext,
    isSecurityInitialized,
    checkRateLimit,
    getRemainingRequests,
    createSession,
    getSession,
    updateSession,
    destroySession,
    generateCSRFToken,
    validateCSRFToken,
    validateInput,
    sanitize,
    secureInvoke,
    logEvent,
    updateSecurityConfig
  }
}

// Higher-order component for securing components
export function withSecurity<P extends object>(
  Component: React.ComponentType<P>,
  securityOptions?: {
    requireAuth?: boolean
    requiredPermissions?: string[]
    rateLimitKey?: string
  }
) {
  const SecuredComponent = (props: P) => {
    const { securityContext, secureInvoke, logEvent } = useSecurity()
    const { requireAuth = true, requiredPermissions = [], rateLimitKey } = securityOptions || {}

    // Check authentication
    if (requireAuth && !securityContext.isAuthenticated) {
      logEvent({
        event_type: 'unauthorized_component_access',
        action: 'access_secured_component',
        result: 'failure',
        risk_level: 'medium',
        details: { component: Component.displayName || Component.name }
      })
      
      return (
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <h3 className="text-lg font-medium text-gray-900 mb-2">Authentication Required</h3>
            <p className="text-sm text-gray-500">Please authenticate to access this component.</p>
          </div>
        </div>
      )
    }

    // Check permissions
    if (requiredPermissions.length > 0) {
      const hasPermissions = requiredPermissions.every(permission => 
        securityContext.permissions.includes(permission)
      )
      
      if (!hasPermissions) {
        logEvent({
          event_type: 'insufficient_permissions',
          action: 'access_secured_component',
          result: 'failure',
          risk_level: 'medium',
          details: { 
            component: Component.displayName || Component.name,
            requiredPermissions,
            userPermissions: securityContext.permissions
          }
        })
        
        return (
          <div className="flex items-center justify-center h-64">
            <div className="text-center">
              <h3 className="text-lg font-medium text-gray-900 mb-2">Insufficient Permissions</h3>
              <p className="text-sm text-gray-500">You don't have permission to access this component.</p>
            </div>
          </div>
        )
      }
    }

    return <Component {...props} />
  }

  SecuredComponent.displayName = `withSecurity(${Component.displayName || Component.name})`
  
  return SecuredComponent
}
