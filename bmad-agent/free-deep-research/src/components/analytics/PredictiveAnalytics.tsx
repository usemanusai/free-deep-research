import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/tauri';
import { Line, Bar, Gauge } from 'react-chartjs-2';

interface PredictiveAnalyticsData {
  usage_predictions: Array<{
    prediction_type: string;
    forecast_period: string;
    forecast_points: Array<{
      timestamp: string;
      predicted_value: number;
      confidence_interval: {
        lower_bound: number;
        upper_bound: number;
      };
    }>;
    accuracy_score: number;
    generated_at: string;
  }>;
  quota_forecasts: Array<{
    service_name: string;
    current_usage: number;
    quota_limit: number;
    forecast_points: Array<{
      timestamp: string;
      predicted_usage: number;
      quota_utilization_percent: number;
      estimated_time_to_limit?: string;
    }>;
    risk_level: string;
    generated_at: string;
  }>;
  capacity_planning: {
    current_capacity: {
      cpu_utilization: number;
      memory_utilization: number;
      storage_utilization: number;
      network_utilization: number;
    };
    growth_projections: Array<{
      time_horizon: string;
      projected_users: number;
      projected_usage: number;
      projected_resource_needs: {
        cpu_needs: number;
        memory_needs: number;
        storage_needs: number;
        network_needs: number;
      };
    }>;
    scaling_recommendations: Array<{
      resource_type: string;
      current_utilization: number;
      projected_utilization: number;
      recommended_action: string;
      time_horizon: string;
      priority: string;
    }>;
    cost_projections: Array<{
      time_horizon: string;
      projected_cost: number;
      theoretical_commercial_cost: number;
      savings: number;
    }>;
    recommended_actions: string[];
  };
  early_warnings: Array<{
    warning_type: string;
    severity: string;
    message: string;
    predicted_occurrence: string;
    confidence: number;
    recommended_actions: string[];
  }>;
  model_accuracy: {
    overall_accuracy: number;
    model_accuracies: Record<string, number>;
    last_training_date: string;
    next_training_scheduled: string;
  };
}

const PredictiveAnalytics: React.FC = () => {
  const [selectedView, setSelectedView] = useState('forecasts');
  const [selectedTimeHorizon, setSelectedTimeHorizon] = useState('30d');

  // Fetch predictive analytics data
  const { data: predictiveData, isLoading, error, refetch } = useQuery<PredictiveAnalyticsData>({
    queryKey: ['predictive-analytics', selectedTimeHorizon],
    queryFn: async () => {
      const result = await invoke('get_predictive_analytics');
      return result as PredictiveAnalyticsData;
    },
    refetchInterval: 300000, // Refresh every 5 minutes
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
        <h3 className="text-red-800 font-medium">Error Loading Predictive Analytics</h3>
        <p className="text-red-600 text-sm mt-1">
          Failed to load predictive analytics data. Please try again.
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

  if (!predictiveData) {
    return <div>No predictive analytics data available</div>;
  }

  // Prepare chart data for usage predictions
  const usagePredictionData = predictiveData.usage_predictions.length > 0 ? {
    labels: predictiveData.usage_predictions[0].forecast_points.map(point => 
      new Date(point.timestamp).toLocaleDateString()
    ),
    datasets: predictiveData.usage_predictions.map((prediction, index) => ({
      label: prediction.prediction_type.replace(/([A-Z])/g, ' $1').trim(),
      data: prediction.forecast_points.map(point => point.predicted_value),
      borderColor: [
        'rgb(59, 130, 246)',
        'rgb(16, 185, 129)',
        'rgb(245, 158, 11)',
        'rgb(239, 68, 68)',
      ][index % 4],
      backgroundColor: [
        'rgba(59, 130, 246, 0.1)',
        'rgba(16, 185, 129, 0.1)',
        'rgba(245, 158, 11, 0.1)',
        'rgba(239, 68, 68, 0.1)',
      ][index % 4],
      tension: 0.4,
    })),
  } : null;

  // Prepare chart data for quota forecasts
  const quotaForecastData = {
    labels: predictiveData.quota_forecasts.map(forecast => forecast.service_name),
    datasets: [
      {
        label: 'Current Usage %',
        data: predictiveData.quota_forecasts.map(forecast => 
          (forecast.current_usage / forecast.quota_limit) * 100
        ),
        backgroundColor: 'rgba(59, 130, 246, 0.6)',
        borderColor: 'rgb(59, 130, 246)',
        borderWidth: 1,
      },
      {
        label: 'Projected Usage %',
        data: predictiveData.quota_forecasts.map(forecast => {
          const lastPoint = forecast.forecast_points[forecast.forecast_points.length - 1];
          return lastPoint ? lastPoint.quota_utilization_percent : 0;
        }),
        backgroundColor: 'rgba(239, 68, 68, 0.6)',
        borderColor: 'rgb(239, 68, 68)',
        borderWidth: 1,
      },
    ],
  };

  // Prepare chart data for capacity planning
  const capacityData = {
    labels: ['CPU', 'Memory', 'Storage', 'Network'],
    datasets: [
      {
        label: 'Current Utilization %',
        data: [
          predictiveData.capacity_planning.current_capacity.cpu_utilization,
          predictiveData.capacity_planning.current_capacity.memory_utilization,
          predictiveData.capacity_planning.current_capacity.storage_utilization,
          predictiveData.capacity_planning.current_capacity.network_utilization,
        ],
        backgroundColor: 'rgba(59, 130, 246, 0.6)',
        borderColor: 'rgb(59, 130, 246)',
        borderWidth: 1,
      },
    ],
  };

  const getRiskColor = (riskLevel: string) => {
    switch (riskLevel.toLowerCase()) {
      case 'critical': return 'text-red-600 bg-red-50 border-red-200';
      case 'high': return 'text-orange-600 bg-orange-50 border-orange-200';
      case 'medium': return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'low': return 'text-green-600 bg-green-50 border-green-200';
      default: return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity.toLowerCase()) {
      case 'critical': return 'text-red-600 bg-red-50 border-red-200';
      case 'warning': return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'info': return 'text-blue-600 bg-blue-50 border-blue-200';
      default: return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  const renderForecastsView = () => (
    <div className="space-y-6">
      {/* Usage Predictions Chart */}
      {usagePredictionData && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Usage Predictions</h3>
          <div className="h-64">
            <Line
              data={usagePredictionData}
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
      )}

      {/* Quota Forecasts */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Quota Forecasts</h3>
        <div className="h-64 mb-6">
          <Bar
            data={quotaForecastData}
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
                  max: 100,
                  title: {
                    display: true,
                    text: 'Usage Percentage',
                  },
                },
              },
            }}
          />
        </div>

        {/* Quota Risk Details */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {predictiveData.quota_forecasts.map((forecast) => (
            <div
              key={forecast.service_name}
              className={`p-4 rounded-lg border ${getRiskColor(forecast.risk_level)}`}
            >
              <div className="flex justify-between items-start mb-2">
                <h4 className="font-medium capitalize">{forecast.service_name}</h4>
                <span className={`px-2 py-1 rounded text-xs font-medium ${getRiskColor(forecast.risk_level)}`}>
                  {forecast.risk_level}
                </span>
              </div>
              <div className="space-y-1 text-sm">
                <p>Current: {forecast.current_usage.toLocaleString()} / {forecast.quota_limit.toLocaleString()}</p>
                <p>Utilization: {((forecast.current_usage / forecast.quota_limit) * 100).toFixed(1)}%</p>
                {forecast.forecast_points.length > 0 && (
                  <p>Projected: {forecast.forecast_points[forecast.forecast_points.length - 1].quota_utilization_percent.toFixed(1)}%</p>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Model Accuracy */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Model Accuracy</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <div className="text-center mb-4">
              <p className="text-3xl font-bold text-blue-600">
                {(predictiveData.model_accuracy.overall_accuracy * 100).toFixed(1)}%
              </p>
              <p className="text-sm text-gray-600">Overall Accuracy</p>
            </div>
            <div className="space-y-2">
              {Object.entries(predictiveData.model_accuracy.model_accuracies).map(([model, accuracy]) => (
                <div key={model} className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">{model}</span>
                  <span className="text-sm font-medium">{(accuracy * 100).toFixed(1)}%</span>
                </div>
              ))}
            </div>
          </div>
          <div className="space-y-3">
            <div>
              <p className="text-sm text-gray-600">Last Training</p>
              <p className="font-medium">
                {new Date(predictiveData.model_accuracy.last_training_date).toLocaleDateString()}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600">Next Training</p>
              <p className="font-medium">
                {new Date(predictiveData.model_accuracy.next_training_scheduled).toLocaleDateString()}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );

  const renderCapacityView = () => (
    <div className="space-y-6">
      {/* Current Capacity */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Current Resource Utilization</h3>
        <div className="h-64">
          <Bar
            data={capacityData}
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
                  max: 100,
                  title: {
                    display: true,
                    text: 'Utilization %',
                  },
                },
              },
            }}
          />
        </div>
      </div>

      {/* Growth Projections */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Growth Projections</h3>
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Time Horizon
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Projected Users
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Projected Usage
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  CPU Needs
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Memory Needs
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {predictiveData.capacity_planning.growth_projections.map((projection, index) => (
                <tr key={index}>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {projection.time_horizon}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {projection.projected_users.toFixed(0)}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {projection.projected_usage.toFixed(0)}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {projection.projected_resource_needs.cpu_needs.toFixed(1)}%
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {projection.projected_resource_needs.memory_needs.toFixed(1)}%
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Scaling Recommendations */}
      {predictiveData.capacity_planning.scaling_recommendations.length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Scaling Recommendations</h3>
          <div className="space-y-4">
            {predictiveData.capacity_planning.scaling_recommendations.map((recommendation, index) => (
              <div key={index} className="border border-gray-200 rounded-lg p-4">
                <div className="flex justify-between items-start mb-2">
                  <h4 className="font-medium text-gray-900">{recommendation.resource_type}</h4>
                  <span className={`px-2 py-1 rounded text-xs font-medium ${
                    recommendation.priority === 'Critical' ? 'bg-red-100 text-red-800' :
                    recommendation.priority === 'High' ? 'bg-orange-100 text-orange-800' :
                    'bg-yellow-100 text-yellow-800'
                  }`}>
                    {recommendation.priority}
                  </span>
                </div>
                <p className="text-sm text-gray-600 mb-2">{recommendation.recommended_action}</p>
                <div className="flex justify-between text-xs text-gray-500">
                  <span>Current: {recommendation.current_utilization.toFixed(1)}%</span>
                  <span>Projected: {recommendation.projected_utilization.toFixed(1)}%</span>
                  <span>Timeline: {recommendation.time_horizon}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Cost Projections */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Cost Projections</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {predictiveData.capacity_planning.cost_projections.map((projection, index) => (
            <div key={index} className="bg-green-50 border border-green-200 rounded-lg p-4">
              <h4 className="font-medium text-green-800 mb-2">{projection.time_horizon}</h4>
              <div className="space-y-1 text-sm">
                <div className="flex justify-between">
                  <span className="text-green-600">Projected Cost:</span>
                  <span className="font-medium">${projection.projected_cost.toFixed(2)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-green-600">Commercial Cost:</span>
                  <span className="font-medium">${projection.theoretical_commercial_cost.toFixed(2)}</span>
                </div>
                <div className="flex justify-between border-t border-green-200 pt-1">
                  <span className="text-green-700 font-medium">Savings:</span>
                  <span className="font-bold text-green-800">${projection.savings.toFixed(2)}</span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );

  const renderWarningsView = () => (
    <div className="space-y-6">
      {/* Early Warnings */}
      {predictiveData.early_warnings.length > 0 ? (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Early Warning System</h3>
          <div className="space-y-4">
            {predictiveData.early_warnings.map((warning, index) => (
              <div
                key={index}
                className={`p-4 rounded-lg border ${getSeverityColor(warning.severity)}`}
              >
                <div className="flex justify-between items-start mb-2">
                  <h4 className="font-medium">{warning.warning_type.replace(/([A-Z])/g, ' $1').trim()}</h4>
                  <span className={`px-2 py-1 rounded text-xs font-medium ${getSeverityColor(warning.severity)}`}>
                    {warning.severity}
                  </span>
                </div>
                <p className="text-sm mb-2">{warning.message}</p>
                <div className="flex justify-between items-center text-xs text-gray-600 mb-3">
                  <span>Predicted: {new Date(warning.predicted_occurrence).toLocaleString()}</span>
                  <span>Confidence: {(warning.confidence * 100).toFixed(0)}%</span>
                </div>
                <div>
                  <p className="text-xs font-medium text-gray-700 mb-1">Recommended Actions:</p>
                  <ul className="text-xs text-gray-600 list-disc list-inside">
                    {warning.recommended_actions.map((action, actionIndex) => (
                      <li key={actionIndex}>{action}</li>
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
              <h3 className="text-green-800 font-medium">No Active Warnings</h3>
              <p className="text-green-600 text-sm">All systems are operating within normal parameters.</p>
            </div>
          </div>
        </div>
      )}

      {/* Recommended Actions */}
      {predictiveData.capacity_planning.recommended_actions.length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Recommended Actions</h3>
          <div className="space-y-2">
            {predictiveData.capacity_planning.recommended_actions.map((action, index) => (
              <div key={index} className="flex items-start">
                <svg className="w-5 h-5 text-blue-600 mr-2 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <p className="text-sm text-gray-700">{action}</p>
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
          <h1 className="text-3xl font-bold text-gray-900">Predictive Analytics</h1>
          <p className="text-gray-600 mt-1">
            AI-powered forecasting and capacity planning insights
          </p>
        </div>
        <div className="flex items-center space-x-4">
          <select
            value={selectedTimeHorizon}
            onChange={(e) => setSelectedTimeHorizon(e.target.value)}
            className="border border-gray-300 rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          >
            <option value="7d">7 Days</option>
            <option value="30d">30 Days</option>
            <option value="90d">90 Days</option>
            <option value="1y">1 Year</option>
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
            { id: 'forecasts', name: 'Forecasts' },
            { id: 'capacity', name: 'Capacity Planning' },
            { id: 'warnings', name: 'Early Warnings' },
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
      {selectedView === 'forecasts' && renderForecastsView()}
      {selectedView === 'capacity' && renderCapacityView()}
      {selectedView === 'warnings' && renderWarningsView()}
    </div>
  );
};

export default PredictiveAnalytics;
