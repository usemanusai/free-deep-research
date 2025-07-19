import React, { useState, useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import {
  ChartBarIcon,
  CpuChipIcon,
  LightBulbIcon,
  TrendingUpIcon,
  ClockIcon,
  UserGroupIcon,
  CurrencyDollarIcon,
  ShieldCheckIcon
} from '@heroicons/react/24/outline';
import {
  LineChart,
  Line,
  AreaChart,
  Area,
  BarChart,
  Bar,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer
} from 'recharts';

interface AnalyticsData {
  performance_metrics: {
    avg_response_time: number;
    success_rate: number;
    total_requests: number;
    error_rate: number;
  };
  usage_patterns: {
    hourly_usage: Array<{ hour: number; requests: number }>;
    methodology_distribution: Array<{ methodology: string; count: number; percentage: number }>;
    api_usage: Array<{ api: string; usage: number; cost: number }>;
  };
  ml_insights: {
    pattern_discoveries: Array<{
      pattern_type: string;
      confidence: number;
      impact_score: number;
      description: string;
    }>;
    recommendations: Array<{
      type: string;
      title: string;
      description: string;
      potential_savings: string;
    }>;
  };
  cost_analysis: {
    monthly_cost: number;
    cost_breakdown: Array<{ category: string; amount: number }>;
    savings_opportunities: Array<{ opportunity: string; potential_savings: number }>;
  };
  predictive_analytics: {
    usage_forecast: Array<{ date: string; predicted_usage: number; confidence: number }>;
    performance_trends: Array<{ metric: string; trend: string; prediction: number }>;
  };
}

const AdvancedAnalyticsDashboard: React.FC = () => {
  const [selectedTimeRange, setSelectedTimeRange] = useState('7d');
  const [selectedMetric, setSelectedMetric] = useState('performance');

  // Fetch analytics data
  const { data: analyticsData, isLoading, error } = useQuery<AnalyticsData>({
    queryKey: ['advanced-analytics', selectedTimeRange],
    queryFn: async () => {
      return await invoke('get_advanced_analytics', {
        timeRange: selectedTimeRange,
        includeMLInsights: true,
        includePredictions: true
      });
    },
    refetchInterval: 30000, // Refresh every 30 seconds
  });

  const timeRangeOptions = [
    { value: '1d', label: 'Last 24 Hours' },
    { value: '7d', label: 'Last 7 Days' },
    { value: '30d', label: 'Last 30 Days' },
    { value: '90d', label: 'Last 90 Days' },
  ];

  const metricCategories = [
    { id: 'performance', label: 'Performance', icon: TrendingUpIcon },
    { id: 'usage', label: 'Usage Patterns', icon: ChartBarIcon },
    { id: 'ml-insights', label: 'ML Insights', icon: CpuChipIcon },
    { id: 'cost', label: 'Cost Analysis', icon: CurrencyDollarIcon },
    { id: 'predictions', label: 'Predictions', icon: LightBulbIcon },
  ];

  const COLORS = ['#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#06B6D4'];

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
        <div className="flex">
          <ShieldCheckIcon className="h-5 w-5 text-red-400" />
          <div className="ml-3">
            <h3 className="text-sm font-medium text-red-800">Analytics Error</h3>
            <p className="mt-1 text-sm text-red-700">
              Failed to load analytics data. Please try again later.
            </p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="bg-white shadow rounded-lg p-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">Advanced Analytics</h1>
            <p className="mt-1 text-sm text-gray-500">
              AI-powered insights and predictive analytics for your research platform
            </p>
          </div>
          <div className="flex space-x-4">
            <select
              value={selectedTimeRange}
              onChange={(e) => setSelectedTimeRange(e.target.value)}
              className="rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            >
              {timeRangeOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>
        </div>
      </div>

      {/* Metric Categories */}
      <div className="grid grid-cols-5 gap-4">
        {metricCategories.map((category) => {
          const Icon = category.icon;
          return (
            <button
              key={category.id}
              onClick={() => setSelectedMetric(category.id)}
              className={`p-4 rounded-lg border-2 transition-colors ${
                selectedMetric === category.id
                  ? 'border-blue-500 bg-blue-50 text-blue-700'
                  : 'border-gray-200 bg-white text-gray-600 hover:border-gray-300'
              }`}
            >
              <Icon className="h-6 w-6 mx-auto mb-2" />
              <span className="text-sm font-medium">{category.label}</span>
            </button>
          );
        })}
      </div>

      {/* Performance Metrics */}
      {selectedMetric === 'performance' && analyticsData && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Performance Overview</h3>
            <div className="grid grid-cols-2 gap-4">
              <div className="bg-blue-50 rounded-lg p-4">
                <div className="flex items-center">
                  <ClockIcon className="h-8 w-8 text-blue-600" />
                  <div className="ml-3">
                    <p className="text-sm font-medium text-blue-600">Avg Response Time</p>
                    <p className="text-2xl font-bold text-blue-900">
                      {analyticsData.performance_metrics.avg_response_time.toFixed(1)}s
                    </p>
                  </div>
                </div>
              </div>
              <div className="bg-green-50 rounded-lg p-4">
                <div className="flex items-center">
                  <TrendingUpIcon className="h-8 w-8 text-green-600" />
                  <div className="ml-3">
                    <p className="text-sm font-medium text-green-600">Success Rate</p>
                    <p className="text-2xl font-bold text-green-900">
                      {(analyticsData.performance_metrics.success_rate * 100).toFixed(1)}%
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Usage Trends</h3>
            <ResponsiveContainer width="100%" height={200}>
              <LineChart data={analyticsData.usage_patterns.hourly_usage}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="hour" />
                <YAxis />
                <Tooltip />
                <Line type="monotone" dataKey="requests" stroke="#3B82F6" strokeWidth={2} />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>
      )}

      {/* Usage Patterns */}
      {selectedMetric === 'usage' && analyticsData && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Methodology Distribution</h3>
            <ResponsiveContainer width="100%" height={300}>
              <PieChart>
                <Pie
                  data={analyticsData.usage_patterns.methodology_distribution}
                  cx="50%"
                  cy="50%"
                  labelLine={false}
                  label={({ methodology, percentage }) => `${methodology} (${percentage.toFixed(1)}%)`}
                  outerRadius={80}
                  fill="#8884d8"
                  dataKey="count"
                >
                  {analyticsData.usage_patterns.methodology_distribution.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
                <Tooltip />
              </PieChart>
            </ResponsiveContainer>
          </div>

          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">API Usage & Costs</h3>
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={analyticsData.usage_patterns.api_usage}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="api" />
                <YAxis yAxisId="left" />
                <YAxis yAxisId="right" orientation="right" />
                <Tooltip />
                <Legend />
                <Bar yAxisId="left" dataKey="usage" fill="#3B82F6" name="Usage Count" />
                <Bar yAxisId="right" dataKey="cost" fill="#10B981" name="Cost ($)" />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      )}

      {/* ML Insights */}
      {selectedMetric === 'ml-insights' && analyticsData && (
        <div className="space-y-6">
          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Pattern Discoveries</h3>
            <div className="space-y-4">
              {analyticsData.ml_insights.pattern_discoveries.map((pattern, index) => (
                <div key={index} className="border border-gray-200 rounded-lg p-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="font-medium text-gray-900">{pattern.pattern_type}</h4>
                      <p className="text-sm text-gray-600 mt-1">{pattern.description}</p>
                    </div>
                    <div className="text-right">
                      <div className="text-sm text-gray-500">Confidence</div>
                      <div className="text-lg font-bold text-blue-600">
                        {(pattern.confidence * 100).toFixed(0)}%
                      </div>
                    </div>
                  </div>
                  <div className="mt-3">
                    <div className="flex items-center">
                      <span className="text-sm text-gray-500 mr-2">Impact Score:</span>
                      <div className="flex-1 bg-gray-200 rounded-full h-2">
                        <div
                          className="bg-blue-600 h-2 rounded-full"
                          style={{ width: `${pattern.impact_score * 100}%` }}
                        ></div>
                      </div>
                      <span className="text-sm text-gray-700 ml-2">
                        {(pattern.impact_score * 100).toFixed(0)}%
                      </span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">AI Recommendations</h3>
            <div className="space-y-4">
              {analyticsData.ml_insights.recommendations.map((rec, index) => (
                <div key={index} className="bg-blue-50 border border-blue-200 rounded-lg p-4">
                  <div className="flex items-start">
                    <LightBulbIcon className="h-5 w-5 text-blue-600 mt-0.5" />
                    <div className="ml-3 flex-1">
                      <h4 className="font-medium text-blue-900">{rec.title}</h4>
                      <p className="text-sm text-blue-700 mt-1">{rec.description}</p>
                      <div className="mt-2 text-sm text-blue-600 font-medium">
                        Potential Impact: {rec.potential_savings}
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Cost Analysis */}
      {selectedMetric === 'cost' && analyticsData && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Cost Breakdown</h3>
            <div className="mb-4">
              <div className="text-3xl font-bold text-gray-900">
                ${analyticsData.cost_analysis.monthly_cost.toFixed(2)}
              </div>
              <div className="text-sm text-gray-500">Monthly Total</div>
            </div>
            <ResponsiveContainer width="100%" height={200}>
              <PieChart>
                <Pie
                  data={analyticsData.cost_analysis.cost_breakdown}
                  cx="50%"
                  cy="50%"
                  outerRadius={80}
                  fill="#8884d8"
                  dataKey="amount"
                  label={({ category, amount }) => `${category}: $${amount.toFixed(2)}`}
                >
                  {analyticsData.cost_analysis.cost_breakdown.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
                <Tooltip />
              </PieChart>
            </ResponsiveContainer>
          </div>

          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Savings Opportunities</h3>
            <div className="space-y-3">
              {analyticsData.cost_analysis.savings_opportunities.map((opportunity, index) => (
                <div key={index} className="flex items-center justify-between p-3 bg-green-50 rounded-lg">
                  <span className="text-sm text-green-800">{opportunity.opportunity}</span>
                  <span className="font-medium text-green-900">
                    ${opportunity.potential_savings.toFixed(2)}/mo
                  </span>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Predictive Analytics */}
      {selectedMetric === 'predictions' && analyticsData && (
        <div className="space-y-6">
          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Usage Forecast</h3>
            <ResponsiveContainer width="100%" height={300}>
              <AreaChart data={analyticsData.predictive_analytics.usage_forecast}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="date" />
                <YAxis />
                <Tooltip />
                <Area
                  type="monotone"
                  dataKey="predicted_usage"
                  stroke="#3B82F6"
                  fill="#3B82F6"
                  fillOpacity={0.3}
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>

          <div className="bg-white shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Performance Predictions</h3>
            <div className="space-y-4">
              {analyticsData.predictive_analytics.performance_trends.map((trend, index) => (
                <div key={index} className="flex items-center justify-between p-4 border border-gray-200 rounded-lg">
                  <div>
                    <h4 className="font-medium text-gray-900">{trend.metric}</h4>
                    <p className="text-sm text-gray-600">Trend: {trend.trend}</p>
                  </div>
                  <div className="text-right">
                    <div className="text-lg font-bold text-blue-600">
                      {trend.prediction.toFixed(1)}
                    </div>
                    <div className="text-sm text-gray-500">Predicted</div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default AdvancedAnalyticsDashboard;
