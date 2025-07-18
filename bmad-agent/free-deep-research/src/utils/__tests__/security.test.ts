import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import {
  rateLimiter,
  sessionManager,
  csrfTokenManager,
  generateCSPHeader,
  sanitizeInput,
  secureApiCall,
  logSecurityEvent,
  initializeSecurity,
  DEFAULT_SECURITY_CONFIG,
  CSP_DIRECTIVES
} from '../security'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

describe('Security Utilities', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  describe('Rate Limiter', () => {
    it('should allow requests within limit', () => {
      const identifier = 'test-user'
      
      // Should allow first request
      expect(rateLimiter.isAllowed(identifier)).toBe(true)
      
      // Should allow subsequent requests within limit
      for (let i = 0; i < 50; i++) {
        expect(rateLimiter.isAllowed(identifier)).toBe(true)
      }
    })

    it('should deny requests exceeding limit', () => {
      const identifier = 'heavy-user'
      
      // Fill up the rate limit
      for (let i = 0; i < 100; i++) {
        rateLimiter.isAllowed(identifier)
      }
      
      // Next request should be denied
      expect(rateLimiter.isAllowed(identifier)).toBe(false)
    })

    it('should reset after time window', () => {
      const identifier = 'time-test-user'
      
      // Fill up the rate limit
      for (let i = 0; i < 100; i++) {
        rateLimiter.isAllowed(identifier)
      }
      
      // Should be denied
      expect(rateLimiter.isAllowed(identifier)).toBe(false)
      
      // Advance time past the window
      vi.advanceTimersByTime(61000) // 61 seconds
      
      // Should be allowed again
      expect(rateLimiter.isAllowed(identifier)).toBe(true)
    })

    it('should handle concurrent requests correctly', () => {
      const identifier = 'concurrent-user'
      const promises = []
      
      // Make 50 concurrent requests
      for (let i = 0; i < 50; i++) {
        promises.push(Promise.resolve(rateLimiter.isAllowed(identifier)))
      }
      
      return Promise.all(promises).then(results => {
        // All should be allowed since we're within limit
        expect(results.every(result => result === true)).toBe(true)
      })
    })

    it('should track remaining requests correctly', () => {
      const identifier = 'remaining-test'
      
      // Initial remaining should be max
      expect(rateLimiter.getRemaining(identifier)).toBe(100)
      
      // Use some requests
      for (let i = 0; i < 30; i++) {
        rateLimiter.isAllowed(identifier)
      }
      
      // Remaining should be reduced
      expect(rateLimiter.getRemaining(identifier)).toBe(70)
    })

    it('should handle edge case of zero requests', () => {
      const identifier = 'zero-requests'
      
      // Should have full limit available
      expect(rateLimiter.getRemaining(identifier)).toBe(100)
      expect(rateLimiter.getResetTime(identifier)).toBe(0)
    })
  })

  describe('Session Manager', () => {
    it('should create and retrieve sessions', () => {
      const sessionId = 'test-session'
      const sessionData = { userId: 'user123', role: 'admin' }
      
      sessionManager.createSession(sessionId, sessionData)
      
      const retrieved = sessionManager.getSession(sessionId)
      expect(retrieved).toEqual(sessionData)
    })

    it('should expire sessions after timeout', () => {
      const sessionId = 'expiring-session'
      const sessionData = { userId: 'user456' }
      
      sessionManager.createSession(sessionId, sessionData)
      
      // Advance time past session timeout
      vi.advanceTimersByTime(31 * 60 * 1000) // 31 minutes
      
      const retrieved = sessionManager.getSession(sessionId)
      expect(retrieved).toBeNull()
    })

    it('should update session timestamp on access', () => {
      const sessionId = 'active-session'
      const sessionData = { userId: 'user789' }
      
      sessionManager.createSession(sessionId, sessionData)
      
      // Advance time but not past timeout
      vi.advanceTimersByTime(20 * 60 * 1000) // 20 minutes
      
      // Access session (should update timestamp)
      sessionManager.getSession(sessionId)
      
      // Advance time again
      vi.advanceTimersByTime(20 * 60 * 1000) // Another 20 minutes
      
      // Should still be valid (total 40 minutes but timestamp was updated)
      const retrieved = sessionManager.getSession(sessionId)
      expect(retrieved).toEqual(sessionData)
    })

    it('should handle non-existent sessions', () => {
      const retrieved = sessionManager.getSession('non-existent')
      expect(retrieved).toBeNull()
    })

    it('should update session data', () => {
      const sessionId = 'update-session'
      const initialData = { userId: 'user123', role: 'user' }
      const updateData = { role: 'admin', lastLogin: '2025-07-18' }
      
      sessionManager.createSession(sessionId, initialData)
      
      const success = sessionManager.updateSession(sessionId, updateData)
      expect(success).toBe(true)
      
      const retrieved = sessionManager.getSession(sessionId)
      expect(retrieved).toEqual({
        userId: 'user123',
        role: 'admin',
        lastLogin: '2025-07-18'
      })
    })

    it('should delete sessions', () => {
      const sessionId = 'delete-session'
      const sessionData = { userId: 'user999' }
      
      sessionManager.createSession(sessionId, sessionData)
      
      const deleted = sessionManager.deleteSession(sessionId)
      expect(deleted).toBe(true)
      
      const retrieved = sessionManager.getSession(sessionId)
      expect(retrieved).toBeNull()
    })
  })

  describe('CSRF Token Manager', () => {
    it('should generate unique tokens', () => {
      const token1 = csrfTokenManager.generateToken()
      const token2 = csrfTokenManager.generateToken()
      
      expect(token1).not.toBe(token2)
      expect(token1).toHaveLength(32)
      expect(token2).toHaveLength(32)
    })

    it('should validate generated tokens', () => {
      const token = csrfTokenManager.generateToken()
      
      expect(csrfTokenManager.validateToken(token)).toBe(true)
      expect(csrfTokenManager.validateToken('invalid-token')).toBe(false)
    })

    it('should consume tokens after use', () => {
      const token = csrfTokenManager.generateToken()
      
      expect(csrfTokenManager.validateToken(token)).toBe(true)
      
      const consumed = csrfTokenManager.consumeToken(token)
      expect(consumed).toBe(true)
      
      // Token should no longer be valid
      expect(csrfTokenManager.validateToken(token)).toBe(false)
    })

    it('should handle token limit correctly', () => {
      const tokens = []
      
      // Generate more than the limit (100)
      for (let i = 0; i < 150; i++) {
        tokens.push(csrfTokenManager.generateToken())
      }
      
      // First 50 tokens should be invalid (cleaned up)
      for (let i = 0; i < 50; i++) {
        expect(csrfTokenManager.validateToken(tokens[i])).toBe(false)
      }
      
      // Last 100 tokens should be valid
      for (let i = 50; i < 150; i++) {
        expect(csrfTokenManager.validateToken(tokens[i])).toBe(true)
      }
    })
  })

  describe('CSP Header Generation', () => {
    it('should generate valid CSP header', () => {
      const csp = generateCSPHeader()
      
      expect(csp).toContain("default-src 'self'")
      expect(csp).toContain("script-src 'self'")
      expect(csp).toContain("style-src 'self'")
      expect(csp).toContain("object-src 'none'")
      expect(csp).toContain("frame-ancestors 'none'")
    })

    it('should include all required directives', () => {
      const csp = generateCSPHeader()
      
      Object.keys(CSP_DIRECTIVES).forEach(directive => {
        expect(csp).toContain(directive)
      })
    })

    it('should handle empty source arrays', () => {
      const csp = generateCSPHeader()
      
      // upgrade-insecure-requests has no sources
      expect(csp).toContain('upgrade-insecure-requests')
    })
  })

  describe('Input Sanitization', () => {
    it('should remove dangerous HTML tags', () => {
      const maliciousInput = '<script>alert("xss")</script>Hello World'
      const sanitized = sanitizeInput(maliciousInput)
      
      expect(sanitized).not.toContain('<script>')
      expect(sanitized).not.toContain('alert')
      expect(sanitized).toBe('Hello World')
    })

    it('should remove javascript: protocols', () => {
      const maliciousInput = 'javascript:alert("xss")'
      const sanitized = sanitizeInput(maliciousInput)
      
      expect(sanitized).not.toContain('javascript:')
      expect(sanitized).toBe('alert("xss")')
    })

    it('should remove event handlers', () => {
      const maliciousInput = 'onclick=alert("xss") Hello'
      const sanitized = sanitizeInput(maliciousInput)
      
      expect(sanitized).not.toContain('onclick=')
      expect(sanitized).toBe(' Hello')
    })

    it('should handle non-string input', () => {
      expect(sanitizeInput(null as any)).toBe('')
      expect(sanitizeInput(undefined as any)).toBe('')
      expect(sanitizeInput(123 as any)).toBe('')
      expect(sanitizeInput({} as any)).toBe('')
    })

    it('should trim whitespace', () => {
      const input = '  Hello World  '
      const sanitized = sanitizeInput(input)
      
      expect(sanitized).toBe('Hello World')
    })
  })

  describe('Security Event Logging', () => {
    it('should log security events', async () => {
      const { invoke } = await import('@tauri-apps/api/core')
      const mockInvoke = vi.mocked(invoke)
      
      const event = {
        event_type: 'test_event',
        action: 'test_action',
        result: 'success' as const,
        risk_level: 'low' as const
      }
      
      await logSecurityEvent(event)
      
      expect(mockInvoke).toHaveBeenCalledWith('log_security_event', {
        event: expect.objectContaining({
          ...event,
          timestamp: expect.any(String)
        })
      })
    })

    it('should handle logging errors gracefully', async () => {
      const { invoke } = await import('@tauri-apps/api/core')
      const mockInvoke = vi.mocked(invoke)
      
      mockInvoke.mockRejectedValueOnce(new Error('Logging failed'))
      
      const event = {
        event_type: 'test_event',
        action: 'test_action',
        result: 'success' as const,
        risk_level: 'low' as const
      }
      
      // Should not throw
      await expect(logSecurityEvent(event)).resolves.toBeUndefined()
    })
  })

  describe('Secure API Call', () => {
    it('should make successful API calls', async () => {
      const { invoke } = await import('@tauri-apps/api/core')
      const mockInvoke = vi.mocked(invoke)
      
      const expectedResult = { success: true, data: 'test' }
      mockInvoke.mockResolvedValueOnce(expectedResult)
      
      const result = await secureApiCall('test_command', { arg: 'value' })
      
      expect(result).toEqual(expectedResult)
      expect(mockInvoke).toHaveBeenCalledWith('test_command', { arg: 'value' })
    })

    it('should enforce rate limiting', async () => {
      const rateLimitKey = 'test-rate-limit'
      
      // Fill up rate limit
      for (let i = 0; i < 100; i++) {
        rateLimiter.isAllowed(rateLimitKey)
      }
      
      await expect(
        secureApiCall('test_command', {}, { rateLimitKey })
      ).rejects.toThrow('Rate limit exceeded')
    })

    it('should log audit events', async () => {
      const { invoke } = await import('@tauri-apps/api/core')
      const mockInvoke = vi.mocked(invoke)
      
      mockInvoke.mockResolvedValueOnce({ success: true })
      
      await secureApiCall('test_command', {}, {
        auditEvent: {
          event_type: 'test_audit',
          action: 'test_action',
          risk_level: 'medium'
        }
      })
      
      // Should have called both the command and the audit log
      expect(mockInvoke).toHaveBeenCalledWith('test_command', {})
      expect(mockInvoke).toHaveBeenCalledWith('log_security_event', expect.any(Object))
    })
  })

  describe('Security Initialization', () => {
    it('should initialize with default config', () => {
      // Should not throw
      expect(() => initializeSecurity()).not.toThrow()
    })

    it('should initialize with custom config', () => {
      const customConfig = {
        ...DEFAULT_SECURITY_CONFIG,
        rateLimitMax: 200
      }
      
      // Should not throw
      expect(() => initializeSecurity(customConfig)).not.toThrow()
    })

    it('should set CSP meta tag in browser environment', () => {
      // Mock DOM
      const mockMeta = {
        httpEquiv: '',
        content: ''
      }
      const mockAppendChild = vi.fn()
      const mockCreateElement = vi.fn(() => mockMeta)
      
      Object.defineProperty(global, 'document', {
        value: {
          createElement: mockCreateElement,
          head: {
            appendChild: mockAppendChild
          }
        },
        writable: true
      })
      
      Object.defineProperty(global, 'window', {
        value: {},
        writable: true
      })
      
      initializeSecurity({ enableCSP: true })
      
      expect(mockCreateElement).toHaveBeenCalledWith('meta')
      expect(mockMeta.httpEquiv).toBe('Content-Security-Policy')
      expect(mockMeta.content).toContain("default-src 'self'")
      expect(mockAppendChild).toHaveBeenCalledWith(mockMeta)
    })
  })
})
