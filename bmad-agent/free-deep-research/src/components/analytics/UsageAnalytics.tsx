import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { Line, Bar, Pie, Doughnut } from 'react-chartjs-2';

interface UsageAnalyticsData {
  total_research_sessions: number;
  methodology_usage: Record<string, number>;
  api_usage_stats: Record<string, {
    total_calls: number;
    successful_calls: number;
    failed_calls: number;
    success_rate: number;
    total_response_time: number;
    average_response_time: number;
    rate_limit_hits: number;
  }>;
  cost_savings: {
    total_theoretical_cost: number;
    actual_cost: number;
    total_savings: number;
    service_costs: Record<string, number>;
    savings_vs_commercial: number;
    cost_per_research_session: number;
  };
  usage_trends: Array<{
    timestamp: string;
    research_sessions: number;
    api_calls: number;
    active_users: number;
  }>;
  peak_usage_times: Array<{
    timestamp: string;
    usage_level: number;
    peak_type: string;
  }>;
}

const UsageAnalytics: React.FC = () => {
  const [selectedPeriod, setSelectedPeriod] = useState('LastWeek');
  const [selectedView, setSelectedView] = useState('overview');

  // Fetch usage analytics data
  const { data: usageData, isLoading, error, refetch } = useQuery<UsageAnalyticsData>({
    queryKey: ['usage-analytics', selectedPeriod],
    queryFn: async () => {
      const result = await invoke('get_usage_analytics', {
        period: selectedPeriod,
      });
      return result as UsageAnalyticsData;
    },
    refetchInterval: 60000, // Refresh every minute
  });

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
        <h3 className="text-red-800 font-medium">Error Loading Usage Analytics</h3>
        <p className="text-red-600 text-sm mt-1">
          Failed to load usage analytics data. Please try again.
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

  if (!usageData) {
    return <div>No usage data available</div>;
  }

  // Prepare chart data
  const methodologyChartData = {
    labels: Object.keys(usageData.methodology_usage),
    datasets: [
      {
        data: Object.values(usageData.methodology_usage),
        backgroundColor: [
          'rgba(59, 130, 246, 0.8)',
          'rgba(16, 185, 129, 0.8)',
          'rgba(245, 158, 11, 0.8)',
          'rgba(239, 68, 68, 0.8)',
          'rgba(139, 92, 246, 0.8)',
        ],
        borderColor: [
          'rgb(59, 130, 246)',
          'rgb(16, 185, 129)',
          'rgb(245, 158, 11)',
          'rgb(239, 68, 68)',
          'rgb(139, 92, 246)',
        ],
        borderWidth: 2,
      },
    ],
  };

  const usageTrendsData = {
    labels: usageData.usage_trends.map(trend => 
      new Date(trend.timestamp).toLocaleDateString()
    ),
    datasets: [
      {
        label: 'Research Sessions',
        data: usageData.usage_trends.map(trend => trend.research_sessions),
        borderColor: 'rgb(59, 130, 246)',
        backgroundColor: 'rgba(59, 130, 246, 0.1)',
        tension: 0.4,
      },
      {
        label: 'API Calls',
        data: usageData.usage_trends.map(trend => trend.api_calls),
        borderColor: 'rgb(16, 185, 129)',
        backgroundColor: 'rgba(16, 185, 129, 0.1)',
        tension: 0.4,
      },
      {
        label: 'Active Users',
        data: usageData.usage_trends.map(trend => trend.active_users),
        borderColor: 'rgb(245, 158, 11)',
        backgroundColor: 'rgba(245, 158, 11, 0.1)',
        tension: 0.4,
      },
    ],
  };

  const apiUsageData = {
    labels: Object.keys(usageData.api_usage_stats),
    datasets: [
      {
        label: 'Total Calls',
        data: Object.values(usageData.api_usage_stats).map(stats => stats.total_calls),
        backgroundColor: 'rgba(59, 130, 246, 0.6)',
        borderColor: 'rgb(59, 130, 246)',
        borderWidth: 1,
      },
      {
        label: 'Failed Calls',
        data: Object.values(usageData.api_usage_stats).map(stats => stats.failed_calls),
        backgroundColor: 'rgba(239, 68, 68, 0.6)',
        borderColor: 'rgb(239, 68, 68)',
        borderWidth: 1,
      },
    ],
  };

  const costSavingsData = {
    labels: Object.keys(usageData.cost_savings.service_costs),
    datasets: [
      {
        data: Object.values(usageData.cost_savings.service_costs),
        backgroundColor: [
          'rgba(59, 130, 246, 0.8)',
          'rgba(16, 185, 129, 0.8)',
          'rgba(245, 158, 11, 0.8)',
          'rgba(239, 68, 68, 0.8)',
          'rgba(139, 92, 246, 0.8)',
        ],
        borderColor: [
          'rgb(59, 130, 246)',
          'rgb(16, 185, 129)',
          'rgb(245, 158, 11)',
          'rgb(239, 68, 68)',
          'rgb(139, 92, 246)',
        ],
        borderWidth: 2,
      },
    ],
  };

  const renderOverview = () => (
    <div className="space-y-6">
      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center">
            <div className="p-2 bg-blue-100 rounded-lg">
              <svg className="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Total Sessions</p>
              <p className="text-2xl font-bold text-gray-900">
                {usageData.total_research_sessions.toLocaleString()}
              </p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center">
            <div className="p-2 bg-green-100 rounded-lg">
              <svg className="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Cost Savings</p>
              <p className="text-2xl font-bold text-gray-900">
                ${usageData.cost_savings.total_savings.toFixed(2)}
              </p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center">
            <div className="p-2 bg-yellow-100 rounded-lg">
              <svg className="w-6 h-6 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">API Calls</p>
              <p className="text-2xl font-bold text-gray-900">
                {Object.values(usageData.api_usage_stats)
                  .reduce((sum, stats) => sum + stats.total_calls, 0)
                  .toLocaleString()}
              </p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center">
            <div className="p-2 bg-purple-100 rounded-lg">
              <svg className="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Success Rate</p>
              <p className="text-2xl font-bold text-gray-900">
                {(Object.values(usageData.api_usage_stats)
                  .reduce((sum, stats) => sum + stats.success_rate, 0) / 
                  Object.keys(usageData.api_usage_stats).length).toFixed(1)}%
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Methodology Usage</h3>
          <div className="h-64">
            <Pie
              data={methodologyChartData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                  legend: {
                    position: 'bottom' as const,
                  },
                },
              }}
            />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Usage Trends</h3>
          <div className="h-64">
            <Line
              data={usageTrendsData}
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
                  },
                },
              }}
            />
          </div>
        </div>
      </div>
    </div>
  );

  const renderAPIAnalysis = () => (
    <div className="space-y-6">
      {/* API Usage Chart */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">API Usage by Service</h3>
        <div className="h-64">
          <Bar
            data={apiUsageData}
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
                },
              },
            }}
          />
        </div>
      </div>

      {/* API Statistics Table */}
      <div className="bg-white rounded-lg shadow overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900">API Service Statistics</h3>
        </div>
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Service
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Total Calls
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Success Rate
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Avg Response Time
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Rate Limit Hits
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {Object.entries(usageData.api_usage_stats).map(([service, stats]) => (
                <tr key={service}>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {service}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {stats.total_calls.toLocaleString()}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${
                      stats.success_rate >= 95 ? 'bg-green-100 text-green-800' :
                      stats.success_rate >= 90 ? 'bg-yellow-100 text-yellow-800' :
                      'bg-red-100 text-red-800'
                    }`}>
                      {stats.success_rate.toFixed(1)}%
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {stats.average_response_time.toFixed(0)}ms
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {stats.rate_limit_hits}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );

  const renderCostAnalysis = () => (
    <div className="space-y-6">
      {/* Cost Savings Summary */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <h4 className="text-lg font-semibold text-gray-900 mb-2">Total Savings</h4>
          <p className="text-3xl font-bold text-green-600">
            ${usageData.cost_savings.total_savings.toFixed(2)}
          </p>
          <p className="text-sm text-gray-600 mt-1">
            vs. theoretical cost of ${usageData.cost_savings.total_theoretical_cost.toFixed(2)}
          </p>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <h4 className="text-lg font-semibold text-gray-900 mb-2">Actual Cost</h4>
          <p className="text-3xl font-bold text-blue-600">
            ${usageData.cost_savings.actual_cost.toFixed(2)}
          </p>
          <p className="text-sm text-gray-600 mt-1">
            Operating on free tiers
          </p>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <h4 className="text-lg font-semibold text-gray-900 mb-2">Cost per Session</h4>
          <p className="text-3xl font-bold text-purple-600">
            ${usageData.cost_savings.cost_per_research_session.toFixed(4)}
          </p>
          <p className="text-sm text-gray-600 mt-1">
            Average theoretical cost
          </p>
        </div>
      </div>

      {/* Cost Distribution Chart */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Cost Distribution by Service</h3>
        <div className="h-64">
          <Doughnut
            data={costSavingsData}
            options={{
              responsive: true,
              maintainAspectRatio: false,
              plugins: {
                legend: {
                  position: 'right' as const,
                },
              },
            }}
          />
        </div>
      </div>

      {/* Commercial Comparison */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Commercial Alternative Comparison</h3>
        <div className="bg-green-50 border border-green-200 rounded-lg p-4">
          <div className="flex items-center">
            <svg className="w-6 h-6 text-green-600 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
            </svg>
            <div>
              <p className="text-green-800 font-medium">
                Savings vs. Commercial: ${usageData.cost_savings.savings_vs_commercial.toFixed(2)}
              </p>
              <p className="text-green-600 text-sm">
                100% cost reduction through optimized free tier usage
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Usage Analytics</h1>
          <p className="text-gray-600 mt-1">
            Comprehensive analysis of system usage patterns and efficiency
          </p>
        </div>
        <div className="flex items-center space-x-4">
          <select
            value={selectedPeriod}
            onChange={(e) => setSelectedPeriod(e.target.value)}
            className="border border-gray-300 rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          >
            <option value="LastHour">Last Hour</option>
            <option value="Last24Hours">Last 24 Hours</option>
            <option value="LastWeek">Last Week</option>
            <option value="LastMonth">Last Month</option>
          </select>
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
            { id: 'overview', name: 'Overview' },
            { id: 'api', name: 'API Analysis' },
            { id: 'cost', name: 'Cost Analysis' },
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
      {selectedView === 'overview' && renderOverview()}
      {selectedView === 'api' && renderAPIAnalysis()}
      {selectedView === 'cost' && renderCostAnalysis()}
    </div>
  );
};

export default UsageAnalytics;
