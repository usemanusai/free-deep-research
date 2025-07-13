import React, { useState, useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/tauri';
import { Line, Bar, Gauge } from 'react-chartjs-2';

interface PerformanceMetrics {
  response_times: {
    api_average_ms: number;
    ui_average_ms: number;
    database_average_ms: number;
    p95_response_time: number;
    p99_response_time: number;
  };
  throughput: {
    requests_per_second: number;
    research_sessions_per_hour: number;
    api_calls_per_minute: number;
    concurrent_users: number;
  };
  resource_usage: {
    cpu_usage_percent: number;
    memory_usage_percent: number;
    disk_usage_percent: number;
    network_usage_mbps: number;
  };
  bottlenecks: Array<{
    id: string;
    bottleneck_type: string;
    severity: string;
    description: string;
    affected_components: string[];
    detected_at: string;
    estimated_impact: string;
    suggested_solutions: string[];
  }>;
  optimization_opportunities: Array<{
    id: string;
    title: string;
    description: string;
    category: string;
    potential_improvement: string;
    implementation_suggestions: string[];
  }>;
}

interface PerformanceTrends {
  response_time_trend: Array<{
    timestamp: string;
    value: number;
  }>;
  throughput_trend: Array<{
    timestamp: string;
    value: number;
  }>;
  resource_usage_trend: Array<{
    timestamp: string;
    value: number;
  }>;
  trend_analysis: {
    response_time_trend: string;
    throughput_trend: string;
    resource_usage_trend: string;
    overall_health: {
      score: number;
      status: string;
    };
  };
}

const PerformanceMonitor: React.FC = () => {
  const [selectedView, setSelectedView] = useState('realtime');
  const [autoRefresh, setAutoRefresh] = useState(true);

  // Fetch current performance metrics
  const { data: performanceData, isLoading, error, refetch } = useQuery<PerformanceMetrics>({
    queryKey: ['performance-metrics'],
    queryFn: async () => {
      const result = await invoke('get_performance_metrics');
      return result as PerformanceMetrics;
    },
    refetchInterval: autoRefresh ? 5000 : false, // Refresh every 5 seconds
  });

  // Fetch performance trends
  const { data: trendsData } = useQuery<PerformanceTrends>({
    queryKey: ['performance-trends'],
    queryFn: async () => {
      const result = await invoke('get_performance_trends');
      return result as PerformanceTrends;
    },
    refetchInterval: 30000, // Refresh every 30 seconds
  });

  // Auto-refresh effect
  useEffect(() => {
    if (autoRefresh) {
      const interval = setInterval(() => {
        refetch();
      }, 5000);
      return () => clearInterval(interval);
    }
  }, [autoRefresh, refetch]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-red-50 border border-red-200 rounded-lg p-4">
        <h3 className="text-red-800 font-medium">Error Loading Performance Data</h3>
        <p className="text-red-600 text-sm mt-1">
          Failed to load performance monitoring data. Please try again.
        </p>
        <button
          onClick={() => refetch()}
          className="mt-2 px-3 py-1 bg-red-600 text-white rounded text-sm hover:bg-red-700"
        >
          Retry
        </button>
      </div>
    );
  }

  if (!performanceData) {
    return <div>No performance data available</div>;
  }

  // Chart data for response times
  const responseTimeData = {
    labels: ['Average', 'P95', 'P99'],
    datasets: [
      {
        label: 'API Response Time (ms)',
        data: [
          performanceData.response_times.api_average_ms,
          performanceData.response_times.p95_response_time,
          performanceData.response_times.p99_response_time,
        ],
        backgroundColor: 'rgba(59, 130, 246, 0.6)',
        borderColor: 'rgb(59, 130, 246)',
        borderWidth: 1,
      },
      {
        label: 'UI Response Time (ms)',
        data: [
          performanceData.response_times.ui_average_ms,
          performanceData.response_times.ui_average_ms * 1.2, // Estimated P95
          performanceData.response_times.ui_average_ms * 1.5, // Estimated P99
        ],
        backgroundColor: 'rgba(16, 185, 129, 0.6)',
        borderColor: 'rgb(16, 185, 129)',
        borderWidth: 1,
      },
    ],
  };

  // Chart data for throughput
  const throughputData = {
    labels: ['Requests/sec', 'Sessions/hour', 'API calls/min', 'Concurrent Users'],
    datasets: [
      {
        label: 'Throughput Metrics',
        data: [
          performanceData.throughput.requests_per_second,
          performanceData.throughput.research_sessions_per_hour,
          performanceData.throughput.api_calls_per_minute,
          performanceData.throughput.concurrent_users,
        ],
        backgroundColor: [
          'rgba(59, 130, 246, 0.6)',
          'rgba(16, 185, 129, 0.6)',
          'rgba(245, 158, 11, 0.6)',
          'rgba(139, 92, 246, 0.6)',
        ],
        borderColor: [
          'rgb(59, 130, 246)',
          'rgb(16, 185, 129)',
          'rgb(245, 158, 11)',
          'rgb(139, 92, 246)',
        ],
        borderWidth: 1,
      },
    ],
  };

  // Chart data for trends
  const trendsChartData = trendsData ? {
    labels: trendsData.response_time_trend.map(point => 
      new Date(point.timestamp).toLocaleTimeString()
    ),
    datasets: [
      {
        label: 'Response Time (ms)',
        data: trendsData.response_time_trend.map(point => point.value),
        borderColor: 'rgb(59, 130, 246)',
        backgroundColor: 'rgba(59, 130, 246, 0.1)',
        tension: 0.4,
        yAxisID: 'y',
      },
      {
        label: 'Throughput (RPS)',
        data: trendsData.throughput_trend.map(point => point.value),
        borderColor: 'rgb(16, 185, 129)',
        backgroundColor: 'rgba(16, 185, 129, 0.1)',
        tension: 0.4,
        yAxisID: 'y1',
      },
    ],
  } : null;

  const getHealthColor = (score: number) => {
    if (score >= 90) return 'text-green-600 bg-green-50 border-green-200';
    if (score >= 70) return 'text-yellow-600 bg-yellow-50 border-yellow-200';
    if (score >= 50) return 'text-orange-600 bg-orange-50 border-orange-200';
    return 'text-red-600 bg-red-50 border-red-200';
  };

  const getSeverityColor = (severity: string) => {
    switch (severity.toLowerCase()) {
      case 'critical': return 'text-red-600 bg-red-50 border-red-200';
      case 'high': return 'text-orange-600 bg-orange-50 border-orange-200';
      case 'medium': return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'low': return 'text-green-600 bg-green-50 border-green-200';
      default: return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  const renderRealTimeView = () => (
    <div className="space-y-6">
      {/* Real-time Metrics Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">Response Time</p>
              <p className="text-2xl font-bold text-gray-900">
                {performanceData.response_times.api_average_ms.toFixed(0)}ms
              </p>
            </div>
            <div className={`w-3 h-3 rounded-full ${
              performanceData.response_times.api_average_ms < 500 ? 'bg-green-500' :
              performanceData.response_times.api_average_ms < 1000 ? 'bg-yellow-500' :
              'bg-red-500'
            }`}></div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">Throughput</p>
              <p className="text-2xl font-bold text-gray-900">
                {performanceData.throughput.requests_per_second.toFixed(1)} RPS
              </p>
            </div>
            <div className={`w-3 h-3 rounded-full ${
              performanceData.throughput.requests_per_second > 5 ? 'bg-green-500' :
              performanceData.throughput.requests_per_second > 1 ? 'bg-yellow-500' :
              'bg-red-500'
            }`}></div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">CPU Usage</p>
              <p className="text-2xl font-bold text-gray-900">
                {performanceData.resource_usage.cpu_usage_percent.toFixed(1)}%
              </p>
            </div>
            <div className={`w-3 h-3 rounded-full ${
              performanceData.resource_usage.cpu_usage_percent < 70 ? 'bg-green-500' :
              performanceData.resource_usage.cpu_usage_percent < 85 ? 'bg-yellow-500' :
              'bg-red-500'
            }`}></div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">Memory Usage</p>
              <p className="text-2xl font-bold text-gray-900">
                {performanceData.resource_usage.memory_usage_percent.toFixed(1)}%
              </p>
            </div>
            <div className={`w-3 h-3 rounded-full ${
              performanceData.resource_usage.memory_usage_percent < 70 ? 'bg-green-500' :
              performanceData.resource_usage.memory_usage_percent < 85 ? 'bg-yellow-500' :
              'bg-red-500'
            }`}></div>
          </div>
        </div>
      </div>

      {/* Resource Usage Gauges */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Resource Utilization</h3>
          <div className="space-y-4">
            <div>
              <div className="flex justify-between text-sm text-gray-600 mb-1">
                <span>CPU</span>
                <span>{performanceData.resource_usage.cpu_usage_percent.toFixed(1)}%</span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div
                  className={`h-2 rounded-full ${
                    performanceData.resource_usage.cpu_usage_percent < 70 ? 'bg-green-500' :
                    performanceData.resource_usage.cpu_usage_percent < 85 ? 'bg-yellow-500' :
                    'bg-red-500'
                  }`}
                  style={{ width: `${performanceData.resource_usage.cpu_usage_percent}%` }}
                ></div>
              </div>
            </div>
            <div>
              <div className="flex justify-between text-sm text-gray-600 mb-1">
                <span>Memory</span>
                <span>{performanceData.resource_usage.memory_usage_percent.toFixed(1)}%</span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div
                  className={`h-2 rounded-full ${
                    performanceData.resource_usage.memory_usage_percent < 70 ? 'bg-green-500' :
                    performanceData.resource_usage.memory_usage_percent < 85 ? 'bg-yellow-500' :
                    'bg-red-500'
                  }`}
                  style={{ width: `${performanceData.resource_usage.memory_usage_percent}%` }}
                ></div>
              </div>
            </div>
            <div>
              <div className="flex justify-between text-sm text-gray-600 mb-1">
                <span>Disk</span>
                <span>{performanceData.resource_usage.disk_usage_percent.toFixed(1)}%</span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div
                  className={`h-2 rounded-full ${
                    performanceData.resource_usage.disk_usage_percent < 70 ? 'bg-green-500' :
                    performanceData.resource_usage.disk_usage_percent < 85 ? 'bg-yellow-500' :
                    'bg-red-500'
                  }`}
                  style={{ width: `${performanceData.resource_usage.disk_usage_percent}%` }}
                ></div>
              </div>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">System Health</h3>
          {trendsData && (
            <div className={`p-4 rounded-lg border ${getHealthColor(trendsData.trend_analysis.overall_health.score)}`}>
              <div className="flex items-center justify-between">
                <div>
                  <p className="font-medium">Overall Health Score</p>
                  <p className="text-2xl font-bold">
                    {trendsData.trend_analysis.overall_health.score.toFixed(0)}%
                  </p>
                  <p className="text-sm opacity-75">
                    Status: {trendsData.trend_analysis.overall_health.status}
                  </p>
                </div>
                <div className="text-right">
                  <p className="text-sm">Trends:</p>
                  <p className="text-xs">Response: {trendsData.trend_analysis.response_time_trend}</p>
                  <p className="text-xs">Throughput: {trendsData.trend_analysis.throughput_trend}</p>
                  <p className="text-xs">Resources: {trendsData.trend_analysis.resource_usage_trend}</p>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Response Time Distribution</h3>
          <div className="h-64">
            <Bar
              data={responseTimeData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                  legend: {
                    position: 'top' as const,
                  },
                },
                scales: {
                  y: {
                    beginAtZero: true,
                    title: {
                      display: true,
                      text: 'Response Time (ms)',
                    },
                  },
                },
              }}
            />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Throughput Metrics</h3>
          <div className="h-64">
            <Bar
              data={throughputData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                  legend: {
                    display: false,
                  },
                },
                scales: {
                  y: {
                    beginAtZero: true,
                  },
                },
              }}
            />
          </div>
        </div>
      </div>
    </div>
  );

  const renderTrendsView = () => (
    <div className="space-y-6">
      {/* Performance Trends Chart */}
      {trendsChartData && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Performance Trends</h3>
          <div className="h-80">
            <Line
              data={trendsChartData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
                interaction: {
                  mode: 'index' as const,
                  intersect: false,
                },
                plugins: {
                  legend: {
                    position: 'top' as const,
                  },
                },
                scales: {
                  x: {
                    display: true,
                    title: {
                      display: true,
                      text: 'Time',
                    },
                  },
                  y: {
                    type: 'linear' as const,
                    display: true,
                    position: 'left' as const,
                    title: {
                      display: true,
                      text: 'Response Time (ms)',
                    },
                  },
                  y1: {
                    type: 'linear' as const,
                    display: true,
                    position: 'right' as const,
                    title: {
                      display: true,
                      text: 'Throughput (RPS)',
                    },
                    grid: {
                      drawOnChartArea: false,
                    },
                  },
                },
              }}
            />
          </div>
        </div>
      )}

      {/* Trend Analysis Summary */}
      {trendsData && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Trend Analysis</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="text-center">
              <p className="text-sm text-gray-600">Response Time Trend</p>
              <p className={`text-lg font-semibold ${
                trendsData.trend_analysis.response_time_trend === 'Decreasing' ? 'text-green-600' :
                trendsData.trend_analysis.response_time_trend === 'Increasing' ? 'text-red-600' :
                'text-gray-600'
              }`}>
                {trendsData.trend_analysis.response_time_trend}
              </p>
            </div>
            <div className="text-center">
              <p className="text-sm text-gray-600">Throughput Trend</p>
              <p className={`text-lg font-semibold ${
                trendsData.trend_analysis.throughput_trend === 'Increasing' ? 'text-green-600' :
                trendsData.trend_analysis.throughput_trend === 'Decreasing' ? 'text-red-600' :
                'text-gray-600'
              }`}>
                {trendsData.trend_analysis.throughput_trend}
              </p>
            </div>
            <div className="text-center">
              <p className="text-sm text-gray-600">Resource Usage Trend</p>
              <p className={`text-lg font-semibold ${
                trendsData.trend_analysis.resource_usage_trend === 'Decreasing' ? 'text-green-600' :
                trendsData.trend_analysis.resource_usage_trend === 'Increasing' ? 'text-red-600' :
                'text-gray-600'
              }`}>
                {trendsData.trend_analysis.resource_usage_trend}
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );

  const renderBottlenecksView = () => (
    <div className="space-y-6">
      {/* Active Bottlenecks */}
      {performanceData.bottlenecks.length > 0 ? (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Active Bottlenecks</h3>
          <div className="space-y-4">
            {performanceData.bottlenecks.map((bottleneck) => (
              <div
                key={bottleneck.id}
                className={`p-4 rounded-lg border ${getSeverityColor(bottleneck.severity)}`}
              >
                <div className="flex justify-between items-start mb-2">
                  <h4 className="font-medium">{bottleneck.bottleneck_type}</h4>
                  <span className={`px-2 py-1 rounded text-xs font-medium ${getSeverityColor(bottleneck.severity)}`}>
                    {bottleneck.severity}
                  </span>
                </div>
                <p className="text-sm mb-2">{bottleneck.description}</p>
                <p className="text-xs text-gray-600 mb-2">
                  Impact: {bottleneck.estimated_impact}
                </p>
                <p className="text-xs text-gray-600 mb-2">
                  Affected: {bottleneck.affected_components.join(', ')}
                </p>
                <div className="mt-3">
                  <p className="text-xs font-medium text-gray-700 mb-1">Suggested Solutions:</p>
                  <ul className="text-xs text-gray-600 list-disc list-inside">
                    {bottleneck.suggested_solutions.map((solution, index) => (
                      <li key={index}>{solution}</li>
                    ))}
                  </ul>
                </div>
              </div>
            ))}
          </div>
        </div>
      ) : (
        <div className="bg-green-50 border border-green-200 rounded-lg p-6">
          <div className="flex items-center">
            <svg className="w-6 h-6 text-green-600 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <div>
              <h3 className="text-green-800 font-medium">No Active Bottlenecks</h3>
              <p className="text-green-600 text-sm">System is performing optimally with no detected bottlenecks.</p>
            </div>
          </div>
        </div>
      )}

      {/* Optimization Opportunities */}
      {performanceData.optimization_opportunities.length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Optimization Opportunities</h3>
          <div className="space-y-4">
            {performanceData.optimization_opportunities.map((opportunity) => (
              <div key={opportunity.id} className="border border-gray-200 rounded-lg p-4">
                <h4 className="font-medium text-gray-900 mb-2">{opportunity.title}</h4>
                <p className="text-sm text-gray-600 mb-2">{opportunity.description}</p>
                <div className="mb-2">
                  <span className="inline-flex px-2 py-1 text-xs font-semibold rounded-full bg-blue-100 text-blue-800">
                    {opportunity.category}
                  </span>
                </div>
                <p className="text-sm text-green-600 mb-2">
                  Potential improvement: {opportunity.potential_improvement}
                </p>
                <div>
                  <p className="text-xs font-medium text-gray-700 mb-1">Implementation Suggestions:</p>
                  <ul className="text-xs text-gray-600 list-disc list-inside">
                    {opportunity.implementation_suggestions.map((suggestion, index) => (
                      <li key={index}>{suggestion}</li>
                    ))}
                  </ul>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Performance Monitor</h1>
          <p className="text-gray-600 mt-1">
            Real-time system performance monitoring and optimization
          </p>
        </div>
        <div className="flex items-center space-x-4">
          <label className="flex items-center">
            <input
              type="checkbox"
              checked={autoRefresh}
              onChange={(e) => setAutoRefresh(e.target.checked)}
              className="mr-2"
            />
            Auto-refresh (5s)
          </label>
          <button
            onClick={() => refetch()}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            Refresh
          </button>
        </div>
      </div>

      {/* Navigation Tabs */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          {[
            { id: 'realtime', name: 'Real-time' },
            { id: 'trends', name: 'Trends' },
            { id: 'bottlenecks', name: 'Bottlenecks' },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setSelectedView(tab.id)}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                selectedView === tab.id
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              {tab.name}
            </button>
          ))}
        </nav>
      </div>

      {/* Content */}
      {selectedView === 'realtime' && renderRealTimeView()}
      {selectedView === 'trends' && renderTrendsView()}
      {selectedView === 'bottlenecks' && renderBottlenecksView()}
    </div>
  );
};

export default PerformanceMonitor;
