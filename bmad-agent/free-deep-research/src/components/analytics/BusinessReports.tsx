import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/tauri';
import { Line, Bar, Pie } from 'react-chartjs-2';

interface BusinessReport {
  report_type: string;
  generated_at: string;
  period: string;
  summary: {
    title: string;
    key_metrics: Array<[string, string]>;
    highlights: string[];
  };
  sections: Array<{
    title: string;
    content: string;
    charts: Array<{
      chart_type: string;
      title: string;
      data: any;
    }>;
  }>;
  charts: Array<{
    chart_type: string;
    title: string;
    data: any;
  }>;
  recommendations: Array<{
    title: string;
    description: string;
    priority: string;
    estimated_impact: string;
    implementation_effort: string;
  }>;
}

const BusinessReports: React.FC = () => {
  const [selectedReportType, setSelectedReportType] = useState('ExecutiveSummary');
  const [isGenerating, setIsGenerating] = useState(false);

  // Fetch business report
  const { data: reportData, isLoading, error, refetch } = useQuery<BusinessReport>({
    queryKey: ['business-report', selectedReportType],
    queryFn: async () => {
      setIsGenerating(true);
      try {
        const result = await invoke('generate_business_report', {
          reportType: selectedReportType,
        });
        return result as BusinessReport;
      } finally {
        setIsGenerating(false);
      }
    },
    enabled: false, // Don't auto-fetch, only on manual trigger
  });

  const generateReport = () => {
    refetch();
  };

  const exportReport = async (format: string) => {
    try {
      await invoke('export_analytics_data', {
        exportType: 'business_report',
        timePeriod: 'current',
        format: format,
      });
      // Show success message
      alert(`Report exported successfully in ${format} format`);
    } catch (error) {
      console.error('Export failed:', error);
      alert('Export failed. Please try again.');
    }
  };

  const renderChart = (chart: any) => {
    const chartProps = {
      data: chart.data,
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: 'top' as const,
          },
          title: {
            display: true,
            text: chart.title,
          },
        },
      },
    };

    switch (chart.chart_type) {
      case 'Line':
        return <Line {...chartProps} />;
      case 'Bar':
        return <Bar {...chartProps} />;
      case 'Pie':
        return <Pie {...chartProps} />;
      default:
        return <div className="text-gray-500">Unsupported chart type: {chart.chart_type}</div>;
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority.toLowerCase()) {
      case 'critical': return 'text-red-600 bg-red-50 border-red-200';
      case 'high': return 'text-orange-600 bg-orange-50 border-orange-200';
      case 'medium': return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'low': return 'text-green-600 bg-green-50 border-green-200';
      default: return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Business Intelligence Reports</h1>
          <p className="text-gray-600 mt-1">
            Comprehensive business analysis and insights
          </p>
        </div>
        <div className="flex items-center space-x-4">
          <select
            value={selectedReportType}
            onChange={(e) => setSelectedReportType(e.target.value)}
            className="border border-gray-300 rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          >
            <option value="ExecutiveSummary">Executive Summary</option>
            <option value="UsageReport">Usage Report</option>
            <option value="PerformanceReport">Performance Report</option>
            <option value="CostAnalysis">Cost Analysis</option>
            <option value="TrendAnalysis">Trend Analysis</option>
          </select>
          <button
            onClick={generateReport}
            disabled={isGenerating}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isGenerating ? 'Generating...' : 'Generate Report'}
          </button>
        </div>
      </div>

      {/* Loading State */}
      {(isLoading || isGenerating) && (
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
            <p className="text-gray-600">Generating business intelligence report...</p>
          </div>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <h3 className="text-red-800 font-medium">Error Generating Report</h3>
          <p className="text-red-600 text-sm mt-1">
            Failed to generate business report. Please try again.
          </p>
          <button
            onClick={generateReport}
            className="mt-2 px-3 py-1 bg-red-600 text-white rounded text-sm hover:bg-red-700"
          >
            Retry
          </button>
        </div>
      )}

      {/* Report Content */}
      {reportData && !isLoading && !isGenerating && (
        <div className="space-y-6">
          {/* Report Header */}
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex justify-between items-start mb-4">
              <div>
                <h2 className="text-2xl font-bold text-gray-900">{reportData.summary.title}</h2>
                <p className="text-gray-600 mt-1">
                  Generated on {new Date(reportData.generated_at).toLocaleString()}
                </p>
                <p className="text-gray-600">
                  Period: {reportData.period}
                </p>
              </div>
              <div className="flex space-x-2">
                <button
                  onClick={() => exportReport('pdf')}
                  className="px-3 py-1 bg-gray-600 text-white rounded text-sm hover:bg-gray-700"
                >
                  Export PDF
                </button>
                <button
                  onClick={() => exportReport('excel')}
                  className="px-3 py-1 bg-green-600 text-white rounded text-sm hover:bg-green-700"
                >
                  Export Excel
                </button>
                <button
                  onClick={() => exportReport('csv')}
                  className="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700"
                >
                  Export CSV
                </button>
              </div>
            </div>

            {/* Key Metrics */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
              {reportData.summary.key_metrics.map(([label, value], index) => (
                <div key={index} className="bg-gray-50 rounded-lg p-4">
                  <p className="text-sm text-gray-600">{label}</p>
                  <p className="text-xl font-bold text-gray-900">{value}</p>
                </div>
              ))}
            </div>

            {/* Highlights */}
            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-3">Key Highlights</h3>
              <ul className="space-y-2">
                {reportData.summary.highlights.map((highlight, index) => (
                  <li key={index} className="flex items-start">
                    <svg className="w-5 h-5 text-green-600 mr-2 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span className="text-gray-700">{highlight}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>

          {/* Report Sections */}
          {reportData.sections.map((section, sectionIndex) => (
            <div key={sectionIndex} className="bg-white rounded-lg shadow p-6">
              <h3 className="text-xl font-semibold text-gray-900 mb-4">{section.title}</h3>
              
              {/* Section Content */}
              <div className="prose max-w-none mb-6">
                <p className="text-gray-700 leading-relaxed">{section.content}</p>
              </div>

              {/* Section Charts */}
              {section.charts.length > 0 && (
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  {section.charts.map((chart, chartIndex) => (
                    <div key={chartIndex} className="bg-gray-50 rounded-lg p-4">
                      <h4 className="text-lg font-medium text-gray-900 mb-3">{chart.title}</h4>
                      <div className="h-64">
                        {renderChart(chart)}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          ))}

          {/* Global Charts */}
          {reportData.charts.length > 0 && (
            <div className="bg-white rounded-lg shadow p-6">
              <h3 className="text-xl font-semibold text-gray-900 mb-4">Visual Analytics</h3>
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {reportData.charts.map((chart, chartIndex) => (
                  <div key={chartIndex} className="bg-gray-50 rounded-lg p-4">
                    <h4 className="text-lg font-medium text-gray-900 mb-3">{chart.title}</h4>
                    <div className="h-64">
                      {renderChart(chart)}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Recommendations */}
          {reportData.recommendations.length > 0 && (
            <div className="bg-white rounded-lg shadow p-6">
              <h3 className="text-xl font-semibold text-gray-900 mb-4">Strategic Recommendations</h3>
              <div className="space-y-4">
                {reportData.recommendations.map((recommendation, index) => (
                  <div key={index} className="border border-gray-200 rounded-lg p-4">
                    <div className="flex justify-between items-start mb-2">
                      <h4 className="font-medium text-gray-900">{recommendation.title}</h4>
                      <span className={`px-2 py-1 rounded text-xs font-medium ${getPriorityColor(recommendation.priority)}`}>
                        {recommendation.priority}
                      </span>
                    </div>
                    <p className="text-sm text-gray-600 mb-3">{recommendation.description}</p>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
                      <div>
                        <span className="font-medium text-gray-700">Estimated Impact:</span>
                        <p className="text-gray-600 mt-1">{recommendation.estimated_impact}</p>
                      </div>
                      <div>
                        <span className="font-medium text-gray-700">Implementation Effort:</span>
                        <p className="text-gray-600 mt-1">{recommendation.implementation_effort}</p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Report Footer */}
          <div className="bg-gray-50 rounded-lg p-6 text-center">
            <p className="text-sm text-gray-600">
              This report was automatically generated by the Free Deep Research System Analytics Engine.
            </p>
            <p className="text-xs text-gray-500 mt-1">
              Report ID: {reportData.report_type}-{new Date(reportData.generated_at).getTime()}
            </p>
          </div>
        </div>
      )}

      {/* Empty State */}
      {!reportData && !isLoading && !isGenerating && !error && (
        <div className="bg-white rounded-lg shadow p-12 text-center">
          <svg className="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <h3 className="text-lg font-medium text-gray-900 mb-2">No Report Generated</h3>
          <p className="text-gray-600 mb-4">
            Select a report type and click "Generate Report" to create a comprehensive business intelligence report.
          </p>
          <button
            onClick={generateReport}
            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            Generate Your First Report
          </button>
        </div>
      )}
    </div>
  );
};

export default BusinessReports;
