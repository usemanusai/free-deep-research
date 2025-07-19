/**
 * Free Deep Research Frontend - API Client
 * Comprehensive API client for communicating with backend services
 */

import axios from 'axios';

// API Configuration
const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080';
const TAURI_API_URL = process.env.REACT_APP_TAURI_API_URL || 'http://localhost:1420';

// Create axios instances
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
});

const tauriClient = axios.create({
  baseURL: TAURI_API_URL,
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptors
apiClient.interceptors.request.use(
  (config) => {
    // Add authentication token if available
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    
    // Add request timestamp
    config.metadata = { startTime: new Date() };
    
    console.log(`[API] ${config.method?.toUpperCase()} ${config.url}`);
    return config;
  },
  (error) => {
    console.error('[API] Request error:', error);
    return Promise.reject(error);
  }
);

// Response interceptors
apiClient.interceptors.response.use(
  (response) => {
    // Calculate response time
    const endTime = new Date();
    const duration = endTime - response.config.metadata.startTime;
    console.log(`[API] ${response.config.method?.toUpperCase()} ${response.config.url} - ${response.status} (${duration}ms)`);
    
    return response;
  },
  (error) => {
    const duration = error.config?.metadata ? new Date() - error.config.metadata.startTime : 0;
    console.error(`[API] ${error.config?.method?.toUpperCase()} ${error.config?.url} - Error (${duration}ms):`, error.response?.data || error.message);
    
    // Handle authentication errors
    if (error.response?.status === 401) {
      localStorage.removeItem('auth_token');
      window.location.href = '/login';
    }
    
    return Promise.reject(error);
  }
);

// Health Check API
export const healthAPI = {
  // Basic health check
  checkHealth: async () => {
    try {
      const response = await apiClient.get('/health');
      return response.data;
    } catch (error) {
      // Fallback to Tauri health check
      try {
        const tauriResponse = await tauriClient.get('/health');
        return tauriResponse.data;
      } catch (tauriError) {
        throw new Error('Both backend and Tauri services are unavailable');
      }
    }
  },
  
  // Detailed health check
  checkDetailedHealth: async () => {
    const response = await apiClient.get('/health/detailed');
    return response.data;
  },
  
  // System health check
  checkSystemHealth: async () => {
    const response = await apiClient.get('/health/system');
    return response.data;
  },
  
  // Database health check
  checkDatabaseHealth: async () => {
    const response = await apiClient.get('/health/database');
    return response.data;
  },
};

// Research Workflow API
export const researchAPI = {
  // Get all workflows
  getWorkflows: async () => {
    const response = await apiClient.get('/api/workflows');
    return response.data;
  },
  
  // Create new workflow
  createWorkflow: async (workflowData) => {
    const response = await apiClient.post('/api/workflows', workflowData);
    return response.data;
  },
  
  // Get workflow by ID
  getWorkflow: async (workflowId) => {
    const response = await apiClient.get(`/api/workflows/${workflowId}`);
    return response.data;
  },
  
  // Update workflow
  updateWorkflow: async (workflowId, workflowData) => {
    const response = await apiClient.put(`/api/workflows/${workflowId}`, workflowData);
    return response.data;
  },
  
  // Delete workflow
  deleteWorkflow: async (workflowId) => {
    const response = await apiClient.delete(`/api/workflows/${workflowId}`);
    return response.data;
  },
  
  // Start workflow execution
  startWorkflow: async (workflowId) => {
    const response = await apiClient.post(`/api/workflows/${workflowId}/start`);
    return response.data;
  },
  
  // Stop workflow execution
  stopWorkflow: async (workflowId) => {
    const response = await apiClient.post(`/api/workflows/${workflowId}/stop`);
    return response.data;
  },
  
  // Get workflow results
  getWorkflowResults: async (workflowId) => {
    const response = await apiClient.get(`/api/workflows/${workflowId}/results`);
    return response.data;
  },
  
  // Get workflow progress
  getWorkflowProgress: async (workflowId) => {
    const response = await apiClient.get(`/api/workflows/${workflowId}/progress`);
    return response.data;
  },
};

// API Keys Management API
export const apiKeysAPI = {
  // Get all API keys
  getApiKeys: async () => {
    const response = await apiClient.get('/api/keys');
    return response.data;
  },
  
  // Add new API key
  addApiKey: async (keyData) => {
    const response = await apiClient.post('/api/keys', keyData);
    return response.data;
  },
  
  // Update API key
  updateApiKey: async (keyId, keyData) => {
    const response = await apiClient.put(`/api/keys/${keyId}`, keyData);
    return response.data;
  },
  
  // Delete API key
  deleteApiKey: async (keyId) => {
    const response = await apiClient.delete(`/api/keys/${keyId}`);
    return response.data;
  },
  
  // Test API key
  testApiKey: async (keyId) => {
    const response = await apiClient.post(`/api/keys/${keyId}/test`);
    return response.data;
  },
  
  // Get API key usage stats
  getApiKeyUsage: async (keyId) => {
    const response = await apiClient.get(`/api/keys/${keyId}/usage`);
    return response.data;
  },
};

// Analytics API
export const analyticsAPI = {
  // Get dashboard data
  getDashboardData: async (timeRange = 'Last24Hours') => {
    const response = await apiClient.get(`/api/analytics/dashboard?timeRange=${timeRange}`);
    return response.data;
  },
  
  // Get usage analytics
  getUsageAnalytics: async (period = 'LastWeek') => {
    const response = await apiClient.get(`/api/analytics/usage?period=${period}`);
    return response.data;
  },
  
  // Get performance metrics
  getPerformanceMetrics: async () => {
    const response = await apiClient.get('/api/analytics/performance');
    return response.data;
  },
  
  // Get predictive analytics
  getPredictiveAnalytics: async () => {
    const response = await apiClient.get('/api/analytics/predictive');
    return response.data;
  },
  
  // Generate business report
  generateBusinessReport: async (reportType = 'ExecutiveSummary') => {
    const response = await apiClient.post('/api/analytics/reports', { reportType });
    return response.data;
  },
  
  // Export analytics data
  exportAnalyticsData: async (exportType, timePeriod, format = 'json') => {
    const response = await apiClient.post('/api/analytics/export', {
      exportType,
      timePeriod,
      format
    });
    return response.data;
  },
};

// System Monitoring API
export const monitoringAPI = {
  // Get system metrics
  getSystemMetrics: async () => {
    const response = await apiClient.get('/api/monitoring/system');
    return response.data;
  },
  
  // Get API usage stats
  getApiUsageStats: async () => {
    const response = await apiClient.get('/api/monitoring/api-usage');
    return response.data;
  },
  
  // Get service health
  getServiceHealth: async () => {
    const response = await apiClient.get('/api/monitoring/service-health');
    return response.data;
  },
  
  // Get audit logs
  getAuditLogs: async (limit = 100) => {
    const response = await apiClient.get(`/api/monitoring/audit-logs?limit=${limit}`);
    return response.data;
  },
};

// Configuration API
export const configAPI = {
  // Get system configuration
  getSystemConfig: async () => {
    const response = await apiClient.get('/api/config/system');
    return response.data;
  },
  
  // Update system configuration
  updateSystemConfig: async (configData) => {
    const response = await apiClient.put('/api/config/system', configData);
    return response.data;
  },
  
  // Get user preferences
  getUserPreferences: async () => {
    const response = await apiClient.get('/api/config/preferences');
    return response.data;
  },
  
  // Update user preferences
  updateUserPreferences: async (preferences) => {
    const response = await apiClient.put('/api/config/preferences', preferences);
    return response.data;
  },
};

// Templates API
export const templatesAPI = {
  // Get all templates
  getTemplates: async () => {
    const response = await apiClient.get('/api/templates');
    return response.data;
  },
  
  // Get template by ID
  getTemplate: async (templateId) => {
    const response = await apiClient.get(`/api/templates/${templateId}`);
    return response.data;
  },
  
  // Create new template
  createTemplate: async (templateData) => {
    const response = await apiClient.post('/api/templates', templateData);
    return response.data;
  },
  
  // Update template
  updateTemplate: async (templateId, templateData) => {
    const response = await apiClient.put(`/api/templates/${templateId}`, templateData);
    return response.data;
  },
  
  // Delete template
  deleteTemplate: async (templateId) => {
    const response = await apiClient.delete(`/api/templates/${templateId}`);
    return response.data;
  },
};

// WebSocket connection for real-time updates
export class WebSocketClient {
  constructor() {
    this.ws = null;
    this.reconnectAttempts = 0;
    this.maxReconnectAttempts = 5;
    this.reconnectDelay = 1000;
    this.listeners = new Map();
  }
  
  connect() {
    const wsUrl = process.env.REACT_APP_WS_URL || 'ws://localhost:8080';
    
    try {
      this.ws = new WebSocket(wsUrl);
      
      this.ws.onopen = () => {
        console.log('[WebSocket] Connected');
        this.reconnectAttempts = 0;
        this.emit('connected');
      };
      
      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          console.log('[WebSocket] Message received:', data);
          this.emit('message', data);
          
          // Emit specific event types
          if (data.type) {
            this.emit(data.type, data);
          }
        } catch (error) {
          console.error('[WebSocket] Failed to parse message:', error);
        }
      };
      
      this.ws.onclose = () => {
        console.log('[WebSocket] Disconnected');
        this.emit('disconnected');
        this.attemptReconnect();
      };
      
      this.ws.onerror = (error) => {
        console.error('[WebSocket] Error:', error);
        this.emit('error', error);
      };
    } catch (error) {
      console.error('[WebSocket] Failed to connect:', error);
      this.attemptReconnect();
    }
  }
  
  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
  
  send(data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    } else {
      console.warn('[WebSocket] Cannot send message - not connected');
    }
  }
  
  on(event, callback) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event).push(callback);
  }
  
  off(event, callback) {
    if (this.listeners.has(event)) {
      const callbacks = this.listeners.get(event);
      const index = callbacks.indexOf(callback);
      if (index > -1) {
        callbacks.splice(index, 1);
      }
    }
  }
  
  emit(event, data) {
    if (this.listeners.has(event)) {
      this.listeners.get(event).forEach(callback => {
        try {
          callback(data);
        } catch (error) {
          console.error(`[WebSocket] Error in event listener for ${event}:`, error);
        }
      });
    }
  }
  
  attemptReconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
      
      console.log(`[WebSocket] Attempting to reconnect in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
      
      setTimeout(() => {
        this.connect();
      }, delay);
    } else {
      console.error('[WebSocket] Max reconnection attempts reached');
      this.emit('maxReconnectAttemptsReached');
    }
  }
}

// Create singleton WebSocket client
export const wsClient = new WebSocketClient();

// Error handling utilities
export const handleApiError = (error) => {
  if (error.response) {
    // Server responded with error status
    const { status, data } = error.response;
    return {
      type: 'server_error',
      status,
      message: data?.message || data?.error || `Server error: ${status}`,
      details: data
    };
  } else if (error.request) {
    // Request was made but no response received
    return {
      type: 'network_error',
      message: 'Network error - please check your connection',
      details: error.message
    };
  } else {
    // Something else happened
    return {
      type: 'client_error',
      message: error.message || 'An unexpected error occurred',
      details: error
    };
  }
};

// API client configuration
export const configureApiClient = (config) => {
  if (config.baseURL) {
    apiClient.defaults.baseURL = config.baseURL;
  }
  if (config.timeout) {
    apiClient.defaults.timeout = config.timeout;
  }
  if (config.headers) {
    Object.assign(apiClient.defaults.headers, config.headers);
  }
};

// Export default API client
export default apiClient;
