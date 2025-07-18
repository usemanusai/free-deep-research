import { expect, afterEach, beforeAll, afterAll, vi } from 'vitest'
import { cleanup } from '@testing-library/react'
import '@testing-library/jest-dom'

// Mock Tauri API
const mockInvoke = vi.fn()
const mockListen = vi.fn()
const mockEmit = vi.fn()

// Mock @tauri-apps/api/core
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke
}))

// Mock @tauri-apps/api/event
vi.mock('@tauri-apps/api/event', () => ({
  listen: mockListen,
  emit: mockEmit
}))

// Mock window.matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(), // deprecated
    removeListener: vi.fn(), // deprecated
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
})

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))

// Mock IntersectionObserver
global.IntersectionObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))

// Mock localStorage
const localStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
}
global.localStorage = localStorageMock

// Mock sessionStorage
const sessionStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
}
global.sessionStorage = sessionStorageMock

// Mock fetch
global.fetch = vi.fn()

// Mock console methods to reduce noise in tests
global.console = {
  ...console,
  log: vi.fn(),
  debug: vi.fn(),
  info: vi.fn(),
  warn: vi.fn(),
  error: vi.fn(),
}

// Setup and teardown
beforeAll(() => {
  // Global test setup
  process.env.NODE_ENV = 'test'
  process.env.MOCK_APIS = 'true'
})

afterEach(() => {
  // Cleanup after each test
  cleanup()
  vi.clearAllMocks()
  localStorageMock.clear()
  sessionStorageMock.clear()
})

afterAll(() => {
  // Global test cleanup
  vi.restoreAllMocks()
})

// Extend expect with custom matchers
expect.extend({
  toBeInTheDocument: (received) => {
    const pass = received && received.ownerDocument && received.ownerDocument.body.contains(received)
    return {
      pass,
      message: () => pass 
        ? `Expected element not to be in the document`
        : `Expected element to be in the document`
    }
  }
})

// Export test utilities
export {
  mockInvoke,
  mockListen,
  mockEmit,
  localStorageMock,
  sessionStorageMock
}

// Test data factories
export const createMockApiKey = (overrides = {}) => ({
  id: 'test-key-id',
  name: 'Test API Key',
  service: 'openrouter',
  encrypted_key: 'encrypted-test-key',
  created_at: new Date().toISOString(),
  last_used: new Date().toISOString(),
  is_active: true,
  usage_count: 0,
  rate_limit: 60,
  ...overrides
})

export const createMockWorkflow = (overrides = {}) => ({
  id: 'test-workflow-id',
  name: 'Test Workflow',
  description: 'Test workflow description',
  status: 'idle',
  progress: 0,
  created_at: new Date().toISOString(),
  updated_at: new Date().toISOString(),
  ...overrides
})

export const createMockSystemMetrics = (overrides = {}) => ({
  timestamp: new Date().toISOString(),
  uptime_seconds: 3600,
  cpu_usage_percent: 25.5,
  memory_usage_percent: 45.2,
  disk_usage_percent: 60.8,
  network_bytes_sent: 1024000,
  network_bytes_received: 2048000,
  active_connections: 5,
  ...overrides
})

// Test helpers
export const waitFor = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

export const mockTauriCommand = (command: string, response: any) => {
  mockInvoke.mockImplementation((cmd: string, ...args: any[]) => {
    if (cmd === command) {
      return Promise.resolve(response)
    }
    return Promise.reject(new Error(`Unmocked command: ${cmd}`))
  })
}

export const mockTauriCommandError = (command: string, error: string) => {
  mockInvoke.mockImplementation((cmd: string, ...args: any[]) => {
    if (cmd === command) {
      return Promise.reject(new Error(error))
    }
    return Promise.reject(new Error(`Unmocked command: ${cmd}`))
  })
}
