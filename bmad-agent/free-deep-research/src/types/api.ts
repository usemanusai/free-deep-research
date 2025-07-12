// API Key types
export interface ApiKey {
  id: string
  service: ServiceProvider
  name: string
  encrypted_key: string
  usage_count: number
  rate_limit: number
  reset_period: ResetPeriod
  last_used?: string
  last_reset: string
  status: ApiKeyStatus
  created_at: string
  updated_at: string
}

export type ServiceProvider = 'openrouter' | 'serpapi' | 'jina' | 'firecrawl'

export type ResetPeriod = 'daily' | 'monthly'

export type ApiKeyStatus = 'active' | 'exhausted' | 'error' | 'disabled'

export interface CreateApiKeyRequest {
  service: ServiceProvider
  name: string
  api_key: string
  rate_limit?: number
}

export interface UpdateApiKeyRequest {
  name?: string
  api_key?: string
  rate_limit?: number
  status?: ApiKeyStatus
}

export interface ApiKeyTestResult {
  key_id: string
  success: boolean
  message: string
  response_time_ms?: number
  tested_at: string
}

export interface ApiKeyImport {
  service: ServiceProvider
  name: string
  api_key: string
  rate_limit?: number
}

export interface ApiKeyExport {
  service: ServiceProvider
  name: string
  rate_limit: number
  usage_count: number
  status: ApiKeyStatus
  created_at: string
}

export interface ImportResult {
  successful_count: number
  failed_count: number
  errors: ImportError[]
}

export interface ImportError {
  index: number
  service: string
  name: string
  error: string
}
