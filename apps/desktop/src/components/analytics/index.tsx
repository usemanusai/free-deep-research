import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import AnalyticsDashboard from './AnalyticsDashboard';
import UsageAnalytics from './UsageAnalytics';
import PerformanceMonitor from './PerformanceMonitor';
import PredictiveAnalytics from './PredictiveAnalytics';
import BusinessReports from './BusinessReports';

interface AnalyticsHealth {
  status: string;
  last_check: string;
  components?: Record<string, string>;
  error?: string;
}

const Analytics: React.FC = () => {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [analyticsHealth, setAnalyticsHealth] = useState<AnalyticsHealth | null>(null);
  const [isHealthy, setIsHealthy] = useState(true);

  // Check analytics health on component mount
  useEffect(() => {
    checkAnalyticsHealth();
    
    // Set up periodic health checks
    const healthCheckInterval = setInterval(checkAnalyticsHealth, 60000); // Every minute
    
    return () => clearInterval(healthCheckInterval);
  }, []);

  const checkAnalyticsHealth = async () => {
    try {
      const health = await invoke('get_analytics_health') as AnalyticsHealth;
      setAnalyticsHealth(health);
      setIsHealthy(health.status === 'healthy');
    } catch (error) {
      console.error('Failed to check analytics health:', error);
      setIsHealthy(false);
      setAnalyticsHealth({
        status: 'error',
        last_check: new Date().toISOString(),
        error: 'Failed to connect to analytics service',
      });
    }
  };

  // Record analytics event for dashboard views
  useEffect(() => {
    const recordDashboardView = async () => {
      try {
        await invoke('record_analytics_event', {
          eventType: 'DashboardViewed',
          metadata: {
            tab: activeTab,
            timestamp: new Date().toISOString(),
          },
        });
      } catch (error) {
        console.error('Failed to record analytics event:', error);
      }
    };

    recordDashboardView();
  }, [activeTab]);

  const tabs = [
    {
      id: 'dashboard',
      name: 'Dashboard',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
      ),
      component: AnalyticsDashboard,
    },
    {
      id: 'usage',
      name: 'Usage Analytics',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 8v8m-4-5v5m-4-2v2m-2 4h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
      ),
      component: UsageAnalytics,
    },
    {
      id: 'performance',
      name: 'Performance',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
      ),
      component: PerformanceMonitor,
    },
    {
      id: 'predictive',
      name: 'Predictive Analytics',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
        </svg>
      ),
      component: PredictiveAnalytics,
    },
    {
      id: 'reports',
      name: 'Business Reports',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
      ),
      component: BusinessReports,
    },
  ];

  const ActiveComponent = tabs.find(tab => tab.id === activeTab)?.component || AnalyticsDashboard;

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Health Status Banner */}
      {!isHealthy && (
        <div className="bg-red-50 border-b border-red-200 px-6 py-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <svg className="w-5 h-5 text-red-600 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
              </svg>
              <span className="text-red-800 font-medium">Analytics Service Issue</span>
              <span className="text-red-600 ml-2">
                {analyticsHealth?.error || 'Analytics service is not responding properly'}
              </span>
            </div>
            <button
              onClick={checkAnalyticsHealth}
              className="px-3 py-1 bg-red-600 text-white rounded text-sm hover:bg-red-700"
            >
              Retry
            </button>
          </div>
        </div>
      )}

      {/* Navigation Header */}
      <div className="bg-white shadow-sm border-b border-gray-200">
        <div className="px-6 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Analytics & Optimization</h1>
              <p className="text-gray-600 mt-1">
                Comprehensive system analytics, performance monitoring, and business intelligence
              </p>
            </div>
            
            {/* Health Indicator */}
            <div className="flex items-center space-x-4">
              <div className="flex items-center">
                <div className={`w-3 h-3 rounded-full mr-2 ${
                  isHealthy ? 'bg-green-500' : 'bg-red-500'
                }`}></div>
                <span className={`text-sm font-medium ${
                  isHealthy ? 'text-green-700' : 'text-red-700'
                }`}>
                  {isHealthy ? 'Analytics Healthy' : 'Analytics Issues'}
                </span>
              </div>
              
              {analyticsHealth && (
                <div className="text-xs text-gray-500">
                  Last check: {new Date(analyticsHealth.last_check).toLocaleTimeString()}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Tab Navigation */}
        <div className="px-6">
          <nav className="-mb-px flex space-x-8">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`group inline-flex items-center py-2 px-1 border-b-2 font-medium text-sm ${
                  activeTab === tab.id
                    ? 'border-blue-500 text-blue-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                <span className={`mr-2 ${
                  activeTab === tab.id ? 'text-blue-500' : 'text-gray-400 group-hover:text-gray-500'
                }`}>
                  {tab.icon}
                </span>
                {tab.name}
              </button>
            ))}
          </nav>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1">
        {isHealthy ? (
          <ActiveComponent />
        ) : (
          <div className="p-6">
            <div className="bg-white rounded-lg shadow p-12 text-center">
              <svg className="w-16 h-16 text-red-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
              </svg>
              <h3 className="text-lg font-medium text-gray-900 mb-2">Analytics Service Unavailable</h3>
              <p className="text-gray-600 mb-4">
                The analytics service is currently experiencing issues. Please check the system status and try again.
              </p>
              <div className="space-y-2">
                <button
                  onClick={checkAnalyticsHealth}
                  className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                  Check Status
                </button>
                {analyticsHealth?.error && (
                  <div className="mt-4 p-3 bg-red-50 border border-red-200 rounded text-sm text-red-700">
                    Error: {analyticsHealth.error}
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Component Health Details (Debug) */}
      {analyticsHealth?.components && process.env.NODE_ENV === 'development' && (
        <div className="fixed bottom-4 right-4 bg-white rounded-lg shadow-lg p-4 max-w-sm">
          <h4 className="font-medium text-gray-900 mb-2">Component Health</h4>
          <div className="space-y-1 text-xs">
            {Object.entries(analyticsHealth.components).map(([component, status]) => (
              <div key={component} className="flex justify-between">
                <span className="text-gray-600">{component}:</span>
                <span className={`font-medium ${
                  status === 'healthy' ? 'text-green-600' : 'text-red-600'
                }`}>
                  {status}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default Analytics;
