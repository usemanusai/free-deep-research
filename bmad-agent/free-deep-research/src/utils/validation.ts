// Input validation and sanitization utilities
import DOMPurify from 'isomorphic-dompurify'

// Validation result interface
export interface ValidationResult {
  isValid: boolean
  errors: string[]
  sanitized?: any
}

// Common validation patterns
export const VALIDATION_PATTERNS = {
  email: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  url: /^https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)$/,
  apiKey: /^[a-zA-Z0-9_-]{20,}$/,
  uuid: /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i,
  alphanumeric: /^[a-zA-Z0-9]+$/,
  alphanumericWithSpaces: /^[a-zA-Z0-9\s]+$/,
  noSpecialChars: /^[a-zA-Z0-9\s\-_\.]+$/,
  strongPassword: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/,
  filename: /^[a-zA-Z0-9\-_\.\s]+$/,
  ipAddress: /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/
}

// Security constants
export const SECURITY_LIMITS = {
  maxStringLength: 10000,
  maxArrayLength: 1000,
  maxObjectDepth: 10,
  maxFileSize: 10 * 1024 * 1024, // 10MB
  maxApiKeyLength: 500,
  maxDescriptionLength: 2000,
  maxNameLength: 100,
  maxTagsCount: 20,
  maxParametersCount: 50
}

// Sanitize HTML content
export function sanitizeHtml(input: string): string {
  return DOMPurify.sanitize(input, {
    ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'p', 'br', 'ul', 'ol', 'li'],
    ALLOWED_ATTR: []
  })
}

// Sanitize string input
export function sanitizeString(input: string, maxLength: number = SECURITY_LIMITS.maxStringLength): string {
  if (typeof input !== 'string') {
    throw new Error('Input must be a string')
  }
  
  // Remove null bytes and control characters
  let sanitized = input.replace(/[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]/g, '')
  
  // Trim whitespace
  sanitized = sanitized.trim()
  
  // Limit length
  if (sanitized.length > maxLength) {
    sanitized = sanitized.substring(0, maxLength)
  }
  
  return sanitized
}

// Validate and sanitize API key
export function validateApiKey(apiKey: string): ValidationResult {
  const errors: string[] = []
  
  if (!apiKey || typeof apiKey !== 'string') {
    errors.push('API key is required and must be a string')
    return { isValid: false, errors }
  }
  
  const sanitized = sanitizeString(apiKey, SECURITY_LIMITS.maxApiKeyLength)
  
  if (sanitized.length < 10) {
    errors.push('API key must be at least 10 characters long')
  }
  
  if (sanitized.length > SECURITY_LIMITS.maxApiKeyLength) {
    errors.push(`API key must not exceed ${SECURITY_LIMITS.maxApiKeyLength} characters`)
  }
  
  if (!VALIDATION_PATTERNS.apiKey.test(sanitized)) {
    errors.push('API key contains invalid characters. Only alphanumeric characters, hyphens, and underscores are allowed')
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized
  }
}

// Validate email address
export function validateEmail(email: string): ValidationResult {
  const errors: string[] = []
  
  if (!email || typeof email !== 'string') {
    errors.push('Email is required and must be a string')
    return { isValid: false, errors }
  }
  
  const sanitized = sanitizeString(email, 254) // RFC 5321 limit
  
  if (!VALIDATION_PATTERNS.email.test(sanitized)) {
    errors.push('Invalid email format')
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized
  }
}

// Validate URL
export function validateUrl(url: string): ValidationResult {
  const errors: string[] = []
  
  if (!url || typeof url !== 'string') {
    errors.push('URL is required and must be a string')
    return { isValid: false, errors }
  }
  
  const sanitized = sanitizeString(url, 2048) // Common URL length limit
  
  if (!VALIDATION_PATTERNS.url.test(sanitized)) {
    errors.push('Invalid URL format. Must be a valid HTTP or HTTPS URL')
  }
  
  // Additional security checks
  try {
    const urlObj = new URL(sanitized)
    
    // Block dangerous protocols
    if (!['http:', 'https:'].includes(urlObj.protocol)) {
      errors.push('Only HTTP and HTTPS protocols are allowed')
    }
    
    // Block localhost and private IPs in production
    if (process.env.NODE_ENV === 'production') {
      const hostname = urlObj.hostname.toLowerCase()
      if (hostname === 'localhost' || 
          hostname === '127.0.0.1' || 
          hostname.startsWith('192.168.') ||
          hostname.startsWith('10.') ||
          hostname.startsWith('172.')) {
        errors.push('Private and localhost URLs are not allowed in production')
      }
    }
  } catch (e) {
    errors.push('Invalid URL format')
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized
  }
}

// Validate object depth to prevent prototype pollution
export function validateObjectDepth(obj: any, maxDepth: number = SECURITY_LIMITS.maxObjectDepth): ValidationResult {
  const errors: string[] = []
  
  function checkDepth(value: any, currentDepth: number): number {
    if (currentDepth > maxDepth) {
      errors.push(`Object depth exceeds maximum allowed depth of ${maxDepth}`)
      return currentDepth
    }
    
    if (value && typeof value === 'object' && !Array.isArray(value)) {
      let maxChildDepth = currentDepth
      for (const key in value) {
        if (value.hasOwnProperty(key)) {
          // Check for prototype pollution attempts
          if (key === '__proto__' || key === 'constructor' || key === 'prototype') {
            errors.push(`Dangerous property name detected: ${key}`)
            continue
          }
          
          const childDepth = checkDepth(value[key], currentDepth + 1)
          maxChildDepth = Math.max(maxChildDepth, childDepth)
        }
      }
      return maxChildDepth
    }
    
    return currentDepth
  }
  
  const depth = checkDepth(obj, 0)
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized: obj
  }
}

// Validate array length
export function validateArray(arr: any[], maxLength: number = SECURITY_LIMITS.maxArrayLength): ValidationResult {
  const errors: string[] = []
  
  if (!Array.isArray(arr)) {
    errors.push('Input must be an array')
    return { isValid: false, errors }
  }
  
  if (arr.length > maxLength) {
    errors.push(`Array length exceeds maximum allowed length of ${maxLength}`)
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized: arr
  }
}

// Validate research workflow parameters
export function validateWorkflowParameters(params: Record<string, any>): ValidationResult {
  const errors: string[] = []
  
  // Validate object structure
  const depthResult = validateObjectDepth(params)
  if (!depthResult.isValid) {
    errors.push(...depthResult.errors)
  }
  
  // Validate specific parameters
  if (params.max_results !== undefined) {
    if (typeof params.max_results !== 'number' || params.max_results < 1 || params.max_results > 1000) {
      errors.push('max_results must be a number between 1 and 1000')
    }
  }
  
  if (params.search_depth !== undefined) {
    const validDepths = ['basic', 'comprehensive', 'deep']
    if (!validDepths.includes(params.search_depth)) {
      errors.push(`search_depth must be one of: ${validDepths.join(', ')}`)
    }
  }
  
  if (params.date_range !== undefined) {
    const validRanges = ['1d', '1w', '1m', '3m', '6m', '1y', 'all']
    if (!validRanges.includes(params.date_range)) {
      errors.push(`date_range must be one of: ${validRanges.join(', ')}`)
    }
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized: params
  }
}

// Validate template parameters
export function validateTemplateData(template: any): ValidationResult {
  const errors: string[] = []
  
  // Required fields
  if (!template.name || typeof template.name !== 'string') {
    errors.push('Template name is required and must be a string')
  } else {
    const nameResult = validateString(template.name, SECURITY_LIMITS.maxNameLength)
    if (!nameResult.isValid) {
      errors.push(...nameResult.errors)
    }
  }
  
  if (!template.description || typeof template.description !== 'string') {
    errors.push('Template description is required and must be a string')
  } else {
    const descResult = validateString(template.description, SECURITY_LIMITS.maxDescriptionLength)
    if (!descResult.isValid) {
      errors.push(...descResult.errors)
    }
  }
  
  // Validate tags array
  if (template.tags && Array.isArray(template.tags)) {
    const tagsResult = validateArray(template.tags, SECURITY_LIMITS.maxTagsCount)
    if (!tagsResult.isValid) {
      errors.push(...tagsResult.errors)
    }
    
    // Validate individual tags
    template.tags.forEach((tag: any, index: number) => {
      if (typeof tag !== 'string') {
        errors.push(`Tag at index ${index} must be a string`)
      } else {
        const tagResult = validateString(tag, 50)
        if (!tagResult.isValid) {
          errors.push(`Tag at index ${index}: ${tagResult.errors.join(', ')}`)
        }
      }
    })
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized: template
  }
}

// Generic string validation
export function validateString(
  input: string, 
  maxLength: number = SECURITY_LIMITS.maxStringLength,
  pattern?: RegExp,
  fieldName: string = 'Field'
): ValidationResult {
  const errors: string[] = []
  
  if (typeof input !== 'string') {
    errors.push(`${fieldName} must be a string`)
    return { isValid: false, errors }
  }
  
  const sanitized = sanitizeString(input, maxLength)
  
  if (sanitized.length === 0) {
    errors.push(`${fieldName} cannot be empty`)
  }
  
  if (sanitized.length > maxLength) {
    errors.push(`${fieldName} must not exceed ${maxLength} characters`)
  }
  
  if (pattern && !pattern.test(sanitized)) {
    errors.push(`${fieldName} contains invalid characters`)
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized
  }
}

// Rate limiting validation
export function validateRateLimit(limit: number): ValidationResult {
  const errors: string[] = []
  
  if (typeof limit !== 'number' || !Number.isInteger(limit)) {
    errors.push('Rate limit must be an integer')
    return { isValid: false, errors }
  }
  
  if (limit < 1 || limit > 10000) {
    errors.push('Rate limit must be between 1 and 10000 requests per minute')
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized: limit
  }
}

// File upload validation
export function validateFileUpload(file: File, allowedTypes: string[] = [], maxSize: number = SECURITY_LIMITS.maxFileSize): ValidationResult {
  const errors: string[] = []
  
  if (!file || !(file instanceof File)) {
    errors.push('Invalid file object')
    return { isValid: false, errors }
  }
  
  // Check file size
  if (file.size > maxSize) {
    errors.push(`File size exceeds maximum allowed size of ${Math.round(maxSize / 1024 / 1024)}MB`)
  }
  
  // Check file type
  if (allowedTypes.length > 0 && !allowedTypes.includes(file.type)) {
    errors.push(`File type ${file.type} is not allowed. Allowed types: ${allowedTypes.join(', ')}`)
  }
  
  // Check filename
  const filenameResult = validateString(file.name, 255, VALIDATION_PATTERNS.filename, 'Filename')
  if (!filenameResult.isValid) {
    errors.push(...filenameResult.errors)
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    sanitized: file
  }
}
