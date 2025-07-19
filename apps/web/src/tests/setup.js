/**
 * Free Deep Research Frontend - Test Setup
 * Comprehensive testing configuration and utilities
 */

import '@testing-library/jest-dom';
import { configure } from '@testing-library/react';
import { server } from './mocks/server';

// Configure testing library
configure({
  testIdAttribute: 'data-testid',
  asyncUtilTimeout: 5000,
});

// Mock environment variables
process.env.REACT_APP_API_URL = 'http://localhost:8080';
process.env.REACT_APP_WS_URL = 'ws://localhost:8080';
process.env.REACT_APP_VERSION = '3.0.0';

// Mock localStorage
const localStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
global.localStorage = localStorageMock;

// Mock sessionStorage
const sessionStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
global.sessionStorage = sessionStorageMock;

// Mock WebSocket
global.WebSocket = jest.fn(() => ({
  close: jest.fn(),
  send: jest.fn(),
  addEventListener: jest.fn(),
  removeEventListener: jest.fn(),
  readyState: 1,
  CONNECTING: 0,
  OPEN: 1,
  CLOSING: 2,
  CLOSED: 3,
}));

// Mock IntersectionObserver
global.IntersectionObserver = jest.fn(() => ({
  observe: jest.fn(),
  disconnect: jest.fn(),
  unobserve: jest.fn(),
}));

// Mock ResizeObserver
global.ResizeObserver = jest.fn(() => ({
  observe: jest.fn(),
  disconnect: jest.fn(),
  unobserve: jest.fn(),
}));

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: jest.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: jest.fn(), // deprecated
    removeListener: jest.fn(), // deprecated
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
});

// Mock scrollTo
Object.defineProperty(window, 'scrollTo', {
  writable: true,
  value: jest.fn(),
});

// Mock fetch if not available
if (!global.fetch) {
  global.fetch = jest.fn();
}

// Setup MSW server
beforeAll(() => {
  server.listen({
    onUnhandledRequest: 'warn',
  });
});

afterEach(() => {
  server.resetHandlers();
  jest.clearAllMocks();
  localStorageMock.clear();
  sessionStorageMock.clear();
});

afterAll(() => {
  server.close();
});

// Global test utilities
global.testUtils = {
  // Mock user data
  mockUser: {
    id: 1,
    email: 'test@example.com',
    name: 'Test User',
    roles: ['user'],
    permissions: ['read', 'write'],
  },
  
  // Mock API responses
  mockApiResponse: (data, status = 200) => ({
    ok: status >= 200 && status < 300,
    status,
    json: () => Promise.resolve(data),
    text: () => Promise.resolve(JSON.stringify(data)),
  }),
  
  // Mock workflow data
  mockWorkflow: {
    id: 1,
    title: 'Test Workflow',
    status: 'pending',
    methodology: 'don_lim',
    created_at: '2025-07-19T00:00:00Z',
    updated_at: '2025-07-19T00:00:00Z',
    config: {
      query: 'Test research query',
      max_sources: 5,
    },
  },
  
  // Mock API key data
  mockApiKey: {
    id: 1,
    service_name: 'test_service',
    api_key: 'test_key_123',
    is_active: true,
    created_at: '2025-07-19T00:00:00Z',
    updated_at: '2025-07-19T00:00:00Z',
    usage_count: 0,
    rate_limit: 100,
  },
  
  // Wait for async operations
  waitFor: (callback, options = {}) => {
    return new Promise((resolve, reject) => {
      const timeout = options.timeout || 5000;
      const interval = options.interval || 100;
      const startTime = Date.now();
      
      const check = () => {
        try {
          const result = callback();
          if (result) {
            resolve(result);
            return;
          }
        } catch (error) {
          // Continue waiting
        }
        
        if (Date.now() - startTime > timeout) {
          reject(new Error('Timeout waiting for condition'));
          return;
        }
        
        setTimeout(check, interval);
      };
      
      check();
    });
  },
  
  // Simulate user interactions
  simulateUserInput: (element, value) => {
    element.focus();
    element.value = value;
    element.dispatchEvent(new Event('input', { bubbles: true }));
    element.dispatchEvent(new Event('change', { bubbles: true }));
  },
  
  // Create mock event
  createMockEvent: (type, properties = {}) => {
    const event = new Event(type, { bubbles: true, cancelable: true });
    Object.assign(event, properties);
    return event;
  },
};

// Console error suppression for known issues
const originalError = console.error;
console.error = (...args) => {
  // Suppress specific warnings that are expected in tests
  const suppressedWarnings = [
    'Warning: ReactDOM.render is deprecated',
    'Warning: componentWillReceiveProps has been renamed',
    'Warning: componentWillMount has been renamed',
  ];
  
  const message = args[0];
  if (typeof message === 'string' && suppressedWarnings.some(warning => message.includes(warning))) {
    return;
  }
  
  originalError.apply(console, args);
};

// Add custom matchers
expect.extend({
  toBeWithinRange(received, floor, ceiling) {
    const pass = received >= floor && received <= ceiling;
    if (pass) {
      return {
        message: () => `expected ${received} not to be within range ${floor} - ${ceiling}`,
        pass: true,
      };
    } else {
      return {
        message: () => `expected ${received} to be within range ${floor} - ${ceiling}`,
        pass: false,
      };
    }
  },
  
  toHaveBeenCalledWithObject(received, expected) {
    const pass = received.mock.calls.some(call => {
      const arg = call[0];
      return Object.keys(expected).every(key => arg[key] === expected[key]);
    });
    
    if (pass) {
      return {
        message: () => `expected function not to have been called with object containing ${JSON.stringify(expected)}`,
        pass: true,
      };
    } else {
      return {
        message: () => `expected function to have been called with object containing ${JSON.stringify(expected)}`,
        pass: false,
      };
    }
  },
});

// Performance monitoring for tests
const performanceObserver = {
  measurements: [],
  mark: (name) => {
    performanceObserver.measurements.push({
      name,
      timestamp: Date.now(),
      type: 'mark',
    });
  },
  measure: (name, startMark, endMark) => {
    const start = performanceObserver.measurements.find(m => m.name === startMark);
    const end = performanceObserver.measurements.find(m => m.name === endMark);
    
    if (start && end) {
      const duration = end.timestamp - start.timestamp;
      performanceObserver.measurements.push({
        name,
        duration,
        type: 'measure',
      });
      return duration;
    }
    return 0;
  },
  clear: () => {
    performanceObserver.measurements = [];
  },
  getEntries: () => performanceObserver.measurements,
};

global.testPerformance = performanceObserver;

// Test data factories
global.testFactories = {
  createUser: (overrides = {}) => ({
    id: Math.floor(Math.random() * 1000),
    email: `user${Math.floor(Math.random() * 1000)}@example.com`,
    name: `Test User ${Math.floor(Math.random() * 1000)}`,
    roles: ['user'],
    permissions: ['read'],
    created_at: new Date().toISOString(),
    ...overrides,
  }),
  
  createWorkflow: (overrides = {}) => ({
    id: Math.floor(Math.random() * 1000),
    title: `Test Workflow ${Math.floor(Math.random() * 1000)}`,
    status: 'pending',
    methodology: 'don_lim',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    config: {
      query: 'Test research query',
      max_sources: 5,
    },
    ...overrides,
  }),
  
  createApiKey: (overrides = {}) => ({
    id: Math.floor(Math.random() * 1000),
    service_name: `test_service_${Math.floor(Math.random() * 1000)}`,
    api_key: `test_key_${Math.floor(Math.random() * 1000)}`,
    is_active: true,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    usage_count: 0,
    rate_limit: 100,
    ...overrides,
  }),
};

// Accessibility testing helpers
global.a11yUtils = {
  checkAriaLabels: (container) => {
    const elementsNeedingLabels = container.querySelectorAll('input, button, select, textarea');
    const issues = [];
    
    elementsNeedingLabels.forEach(element => {
      const hasLabel = element.getAttribute('aria-label') || 
                      element.getAttribute('aria-labelledby') ||
                      container.querySelector(`label[for="${element.id}"]`);
      
      if (!hasLabel) {
        issues.push(`Element ${element.tagName} missing accessible label`);
      }
    });
    
    return issues;
  },
  
  checkKeyboardNavigation: (container) => {
    const focusableElements = container.querySelectorAll(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    
    return Array.from(focusableElements).every(element => {
      return element.tabIndex >= 0 || element.getAttribute('tabindex') !== '-1';
    });
  },
  
  checkColorContrast: (element) => {
    // Simplified color contrast check
    const style = window.getComputedStyle(element);
    const backgroundColor = style.backgroundColor;
    const color = style.color;
    
    // This is a simplified check - in real tests you'd use a proper contrast ratio calculator
    return backgroundColor !== color;
  },
};

// Error boundary for tests
global.TestErrorBoundary = class extends React.Component {
  constructor(props) {
    super(props);
    this.state = { hasError: false, error: null };
  }
  
  static getDerivedStateFromError(error) {
    return { hasError: true, error };
  }
  
  componentDidCatch(error, errorInfo) {
    console.error('Test Error Boundary caught an error:', error, errorInfo);
  }
  
  render() {
    if (this.state.hasError) {
      return React.createElement('div', {
        'data-testid': 'error-boundary',
        children: `Error: ${this.state.error?.message || 'Unknown error'}`
      });
    }
    
    return this.props.children;
  }
};
