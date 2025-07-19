/**
 * Security Utilities and Configurations
 *
 * This module provides comprehensive security utilities for the Free Deep Research System,
 * including Content Security Policy (CSP) configuration, rate limiting, session management,
 * CSRF protection, and audit logging.
 *
 * Key Features:
 * - Content Security Policy (CSP) generation and management
 * - Rate limiting with configurable windows and limits
 * - Session management with automatic cleanup
 * - CSRF token generation and validation
 * - Security event logging and audit trails
 * - Input sanitization and XSS prevention
 * - Secure API call wrapper with built-in protections
 *
 * Security Best Practices Implemented:
 * - Defense in depth with multiple security layers
 * - Principle of least privilege for permissions
 * - Secure by default configurations
 * - Comprehensive audit logging for security events
 * - Rate limiting to prevent abuse
 * - Input validation and sanitization
 *
 * @author Free Deep Research System
 * @version 1.0.0
 */

import { invoke } from '@tauri-apps/api/core'

// Security configuration interface
export interface SecurityConfig {
  enableCSP: boolean
  enableHSTS: boolean
  enableXFrameOptions: boolean
  enableXContentTypeOptions: boolean
  enableReferrerPolicy: boolean
  maxRequestSize: number
  rateLimitWindow: number
  rateLimitMax: number
  sessionTimeout: number
  enableAuditLogging: boolean
}

// Default security configuration
export const DEFAULT_SECURITY_CONFIG: SecurityConfig = {
  enableCSP: true,
  enableHSTS: true,
  enableXFrameOptions: true,
  enableXContentTypeOptions: true,
  enableReferrerPolicy: true,
  maxRequestSize: 10 * 1024 * 1024, // 10MB
  rateLimitWindow: 60 * 1000, // 1 minute
  rateLimitMax: 100, // 100 requests per minute
  sessionTimeout: 30 * 60 * 1000, // 30 minutes
  enableAuditLogging: true
}

// Content Security Policy configuration
export const CSP_DIRECTIVES = {
  'default-src': ["'self'"],
  'script-src': [
    "'self'",
    "'unsafe-inline'", // Required for Vite in development
    "'unsafe-eval'", // Required for Vite in development
    'https://cdn.jsdelivr.net',
    'https://unpkg.com'
  ],
  'style-src': [
    "'self'",
    "'unsafe-inline'", // Required for CSS-in-JS and Tailwind
    'https://fonts.googleapis.com'
  ],
  'font-src': [
    "'self'",
    'https://fonts.gstatic.com',
    'data:'
  ],
  'img-src': [
    "'self'",
    'data:',
    'blob:',
    'https:'
  ],
  'connect-src': [
    "'self'",
    'https://api.openrouter.ai',
    'https://serpapi.com',
    'https://r.jina.ai',
    'https://api.firecrawl.dev',
    'wss://localhost:*',
    'ws://localhost:*'
  ],
  'media-src': ["'self'", 'data:', 'blob:'],
  'object-src': ["'none'"],
  'base-uri': ["'self'"],
  'form-action': ["'self'"],
  'frame-ancestors': ["'none'"],
  'upgrade-insecure-requests': []
}

// Generate CSP header value
export function generateCSPHeader(): string {
  const directives = Object.entries(CSP_DIRECTIVES)
    .map(([directive, sources]) => {
      if (sources.length === 0) {
        return directive
      }
      return `${directive} ${sources.join(' ')}`
    })
    .join('; ')
  
  return directives
}

// Security headers configuration
export const SECURITY_HEADERS = {
  'Content-Security-Policy': generateCSPHeader(),
  'Strict-Transport-Security': 'max-age=31536000; includeSubDomains; preload',
  'X-Frame-Options': 'DENY',
  'X-Content-Type-Options': 'nosniff',
  'Referrer-Policy': 'strict-origin-when-cross-origin',
  'X-XSS-Protection': '1; mode=block',
  'Permissions-Policy': 'camera=(), microphone=(), geolocation=(), payment=()',
  'Cross-Origin-Embedder-Policy': 'require-corp',
  'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Resource-Policy': 'same-origin'
}

/**
 * Rate Limiting Implementation
 *
 * Implements a sliding window rate limiter to prevent abuse and ensure fair usage
 * of system resources. The rate limiter tracks requests per identifier (e.g., user ID,
 * IP address, session ID) and enforces configurable limits.
 *
 * Features:
 * - Sliding window algorithm for accurate rate limiting
 * - Configurable rate limits and time windows
 * - Automatic cleanup of expired entries
 * - Per-identifier tracking and limits
 * - Memory efficient with periodic cleanup
 *
 * Algorithm:
 * 1. Track timestamps of requests per identifier
 * 2. Filter out requests outside the current window
 * 3. Check if remaining requests exceed the limit
 * 4. Allow or deny the request based on current usage
 *
 * @example
 * ```typescript
 * const limiter = new RateLimiter({
 *   rateLimitMax: 100,
 *   rateLimitWindow: 60000 // 1 minute
 * })
 *
 * if (limiter.isAllowed('user123')) {
 *   // Process request
 * } else {
 *   // Rate limit exceeded
 * }
 * ```
 */
class RateLimiter {
  /** Map storing request timestamps for each identifier */
  private requests: Map<string, number[]> = new Map()

  /** Security configuration containing rate limit settings */
  private config: SecurityConfig

  /**
   * Creates a new RateLimiter instance
   * @param config Security configuration with rate limiting settings
   */
  constructor(config: SecurityConfig = DEFAULT_SECURITY_CONFIG) {
    this.config = config

    // Set up periodic cleanup to prevent memory leaks
    // Cleanup interval matches the rate limit window for efficiency
    setInterval(() => {
      this.cleanup()
    }, this.config.rateLimitWindow)
  }

  // Check if request is allowed
  isAllowed(identifier: string): boolean {
    const now = Date.now()
    const windowStart = now - this.config.rateLimitWindow
    
    // Get existing requests for this identifier
    const requests = this.requests.get(identifier) || []
    
    // Filter out old requests
    const recentRequests = requests.filter(timestamp => timestamp > windowStart)
    
    // Check if under limit
    if (recentRequests.length >= this.config.rateLimitMax) {
      return false
    }
    
    // Add current request
    recentRequests.push(now)
    this.requests.set(identifier, recentRequests)
    
    return true
  }

  // Get remaining requests for identifier
  getRemaining(identifier: string): number {
    const now = Date.now()
    const windowStart = now - this.config.rateLimitWindow
    const requests = this.requests.get(identifier) || []
    const recentRequests = requests.filter(timestamp => timestamp > windowStart)
    
    return Math.max(0, this.config.rateLimitMax - recentRequests.length)
  }

  // Get reset time for identifier
  getResetTime(identifier: string): number {
    const requests = this.requests.get(identifier) || []
    if (requests.length === 0) {
      return 0
    }
    
    const oldestRequest = Math.min(...requests)
    return oldestRequest + this.config.rateLimitWindow
  }

  // Clean up old entries
  private cleanup(): void {
    const now = Date.now()
    const windowStart = now - this.config.rateLimitWindow
    
    for (const [identifier, requests] of this.requests.entries()) {
      const recentRequests = requests.filter(timestamp => timestamp > windowStart)
      
      if (recentRequests.length === 0) {
        this.requests.delete(identifier)
      } else {
        this.requests.set(identifier, recentRequests)
      }
    }
  }
}

// Global rate limiter instance
export const rateLimiter = new RateLimiter()

// Session management
class SessionManager {
  private sessions: Map<string, { timestamp: number; data: any }> = new Map()
  private config: SecurityConfig

  constructor(config: SecurityConfig = DEFAULT_SECURITY_CONFIG) {
    this.config = config
    
    // Clean up expired sessions periodically
    setInterval(() => {
      this.cleanup()
    }, 60000) // Check every minute
  }

  // Create new session
  createSession(sessionId: string, data: any): void {
    this.sessions.set(sessionId, {
      timestamp: Date.now(),
      data
    })
  }

  // Get session data
  getSession(sessionId: string): any | null {
    const session = this.sessions.get(sessionId)
    if (!session) {
      return null
    }
    
    // Check if session is expired
    if (Date.now() - session.timestamp > this.config.sessionTimeout) {
      this.sessions.delete(sessionId)
      return null
    }
    
    // Update timestamp
    session.timestamp = Date.now()
    return session.data
  }

  // Update session
  updateSession(sessionId: string, data: any): boolean {
    const session = this.sessions.get(sessionId)
    if (!session) {
      return false
    }
    
    session.data = { ...session.data, ...data }
    session.timestamp = Date.now()
    return true
  }

  // Delete session
  deleteSession(sessionId: string): boolean {
    return this.sessions.delete(sessionId)
  }

  // Clean up expired sessions
  private cleanup(): void {
    const now = Date.now()
    
    for (const [sessionId, session] of this.sessions.entries()) {
      if (now - session.timestamp > this.config.sessionTimeout) {
        this.sessions.delete(sessionId)
      }
    }
  }
}

// Global session manager instance
export const sessionManager = new SessionManager()

// Audit logging
export interface AuditEvent {
  timestamp: string
  event_type: string
  user_id?: string
  resource_id?: string
  action: string
  result: 'success' | 'failure' | 'error'
  details?: Record<string, any>
  ip_address?: string
  user_agent?: string
  risk_level: 'low' | 'medium' | 'high' | 'critical'
}

// Log security event
export async function logSecurityEvent(event: Omit<AuditEvent, 'timestamp'>): Promise<void> {
  const auditEvent: AuditEvent = {
    ...event,
    timestamp: new Date().toISOString()
  }
  
  try {
    // Log to backend
    await invoke('log_security_event', { event: auditEvent })
    
    // Log to console in development
    if (process.env.NODE_ENV === 'development') {
      console.log('[SECURITY]', auditEvent)
    }
  } catch (error) {
    console.error('Failed to log security event:', error)
  }
}

// Input sanitization for XSS prevention
export function sanitizeInput(input: string): string {
  if (typeof input !== 'string') {
    return ''
  }
  
  return input
    .replace(/[<>]/g, '') // Remove angle brackets
    .replace(/javascript:/gi, '') // Remove javascript: protocol
    .replace(/on\w+=/gi, '') // Remove event handlers
    .replace(/data:/gi, '') // Remove data: protocol
    .trim()
}

// CSRF token management
class CSRFTokenManager {
  private tokens: Set<string> = new Set()
  private readonly tokenLength = 32

  // Generate new CSRF token
  generateToken(): string {
    const token = this.generateRandomString(this.tokenLength)
    this.tokens.add(token)
    
    // Clean up old tokens (keep last 100)
    if (this.tokens.size > 100) {
      const tokensArray = Array.from(this.tokens)
      const tokensToRemove = tokensArray.slice(0, tokensArray.length - 100)
      tokensToRemove.forEach(token => this.tokens.delete(token))
    }
    
    return token
  }

  // Validate CSRF token
  validateToken(token: string): boolean {
    return this.tokens.has(token)
  }

  // Remove token after use
  consumeToken(token: string): boolean {
    return this.tokens.delete(token)
  }

  private generateRandomString(length: number): string {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
    let result = ''
    for (let i = 0; i < length; i++) {
      result += chars.charAt(Math.floor(Math.random() * chars.length))
    }
    return result
  }
}

// Global CSRF token manager
export const csrfTokenManager = new CSRFTokenManager()

// Security middleware for API calls
export async function secureApiCall<T>(
  command: string,
  args?: Record<string, any>,
  options?: {
    requireAuth?: boolean
    rateLimitKey?: string
    auditEvent?: Partial<AuditEvent>
  }
): Promise<T> {
  const { requireAuth = true, rateLimitKey, auditEvent } = options || {}
  
  try {
    // Rate limiting
    if (rateLimitKey && !rateLimiter.isAllowed(rateLimitKey)) {
      const error = new Error('Rate limit exceeded')
      
      if (auditEvent) {
        await logSecurityEvent({
          ...auditEvent,
          event_type: 'rate_limit_exceeded',
          action: command,
          result: 'failure',
          risk_level: 'medium'
        })
      }
      
      throw error
    }
    
    // Make the API call
    const result = await invoke<T>(command, args)
    
    // Log successful operation
    if (auditEvent) {
      await logSecurityEvent({
        ...auditEvent,
        event_type: auditEvent.event_type || 'api_call',
        action: command,
        result: 'success',
        risk_level: auditEvent.risk_level || 'low'
      })
    }
    
    return result
  } catch (error) {
    // Log failed operation
    if (auditEvent) {
      await logSecurityEvent({
        ...auditEvent,
        event_type: auditEvent.event_type || 'api_call',
        action: command,
        result: 'error',
        risk_level: auditEvent.risk_level || 'medium',
        details: {
          error: error instanceof Error ? error.message : 'Unknown error'
        }
      })
    }
    
    throw error
  }
}

// Initialize security configuration
export function initializeSecurity(config: Partial<SecurityConfig> = {}): void {
  const securityConfig = { ...DEFAULT_SECURITY_CONFIG, ...config }
  
  // Set up CSP if in browser environment
  if (typeof window !== 'undefined' && securityConfig.enableCSP) {
    const meta = document.createElement('meta')
    meta.httpEquiv = 'Content-Security-Policy'
    meta.content = generateCSPHeader()
    document.head.appendChild(meta)
  }
  
  // Log security initialization
  logSecurityEvent({
    event_type: 'security_initialized',
    action: 'initialize_security',
    result: 'success',
    risk_level: 'low',
    details: { config: securityConfig }
  })
}
