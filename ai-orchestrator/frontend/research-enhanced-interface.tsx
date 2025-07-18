// Research-Enhanced BMAD Interface Component
import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { 
  Card, 
  CardContent, 
  CardHeader, 
  CardTitle 
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { 
  Brain, 
  Search, 
  FileText, 
  CheckCircle, 
  AlertCircle,
  TrendingUp,
  Shield,
  Server,
  Users
} from 'lucide-react';

// Type definitions for integration
interface DocumentationModeRequest {
  project_description: string;
  requirements: string[];
  target_audience: string;
  research_depth: 'Basic' | 'Standard' | 'Comprehensive' | 'Expert';
  cost_limit?: number;
  timeline_minutes?: number;
}

interface DocumentationModeResponse {
  session_id: string;
  status: string;
  deliverables: DocumentationDeliverables;
  research_summary: ResearchSummary;
  quality_metrics: QualityMetrics;
  cost_breakdown: CostBreakdown;
}

interface DocumentationDeliverables {
  prd: string;
  architecture: string;
  checklist: string;
  research_appendix: string;
}

interface ResearchSummary {
  total_research_conducted: number;
  research_confidence_average: number;
  sources_analyzed: number;
  evidence_items_collected: number;
  research_duration_minutes: number;
}

interface QualityMetrics {
  overall_confidence_score: number;
  source_diversity_score: number;
  evidence_completeness_score: number;
  research_coverage_score: number;
  quality_gates_passed: number;
  quality_gates_total: number;
}

interface CostBreakdown {
  total_cost: number;
  research_cost: number;
  api_cost: number;
  processing_cost: number;
  cost_per_deliverable: number;
}

interface IntegrationHealthStatus {
  overall_status: string;
  research_bridge_status: string;
  agent_enhancer_status: string;
  workflow_coordinator_status: string;
  integration_enabled: boolean;
  active_research_count: number;
  error_messages: string[];
}

export const ResearchEnhancedBMADInterface: React.FC = () => {
  // State management
  const [selectedMode, setSelectedMode] = useState<'documentation' | 'development' | null>(null);
  const [isExecuting, setIsExecuting] = useState(false);
  const [executionProgress, setExecutionProgress] = useState(0);
  const [currentPhase, setCurrentPhase] = useState('');
  const [results, setResults] = useState<DocumentationModeResponse | null>(null);
  const [healthStatus, setHealthStatus] = useState<IntegrationHealthStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  // Form state
  const [projectDescription, setProjectDescription] = useState('');
  const [requirements, setRequirements] = useState<string[]>(['']);
  const [targetAudience, setTargetAudience] = useState('');
  const [researchDepth, setResearchDepth] = useState<'Basic' | 'Standard' | 'Comprehensive' | 'Expert'>('Standard');
  const [costLimit, setCostLimit] = useState<number>(25);

  // Load health status on component mount
  useEffect(() => {
    checkIntegrationHealth();
  }, []);

  const checkIntegrationHealth = async () => {
    try {
      const health = await invoke<IntegrationHealthStatus>('get_integration_health_status');
      setHealthStatus(health);
    } catch (err) {
      console.error('Failed to check integration health:', err);
      setError('Failed to connect to BMAD-Research integration');
    }
  };

  const executeDocumentationMode = async () => {
    if (!projectDescription.trim()) {
      setError('Project description is required');
      return;
    }

    setIsExecuting(true);
    setError(null);
    setExecutionProgress(0);
    setCurrentPhase('Initializing research-enhanced documentation mode...');

    try {
      const request: DocumentationModeRequest = {
        project_description: projectDescription,
        requirements: requirements.filter(req => req.trim()),
        target_audience: targetAudience,
        research_depth: researchDepth,
        cost_limit: costLimit,
        timeline_minutes: 120,
      };

      // Simulate progress updates
      const progressInterval = setInterval(() => {
        setExecutionProgress(prev => {
          if (prev < 90) return prev + 10;
          return prev;
        });
      }, 5000);

      // Update phases
      setTimeout(() => setCurrentPhase('Conducting market research...'), 2000);
      setTimeout(() => setCurrentPhase('Analyzing technology options...'), 15000);
      setTimeout(() => setCurrentPhase('Researching infrastructure requirements...'), 30000);
      setTimeout(() => setCurrentPhase('Synthesizing research findings...'), 45000);
      setTimeout(() => setCurrentPhase('Generating documentation...'), 60000);
      setTimeout(() => setCurrentPhase('Validating quality gates...'), 75000);

      const response = await invoke<DocumentationModeResponse>(
        'execute_research_enhanced_documentation_mode',
        { request }
      );

      clearInterval(progressInterval);
      setExecutionProgress(100);
      setCurrentPhase('Documentation generation completed!');
      setResults(response);
    } catch (err) {
      setError(`Documentation mode failed: ${err}`);
      console.error('Documentation mode error:', err);
    } finally {
      setIsExecuting(false);
    }
  };

  const addRequirement = () => {
    setRequirements([...requirements, '']);
  };

  const updateRequirement = (index: number, value: string) => {
    const updated = [...requirements];
    updated[index] = value;
    setRequirements(updated);
  };

  const removeRequirement = (index: number) => {
    setRequirements(requirements.filter((_, i) => i !== index));
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'healthy': return 'bg-green-500';
      case 'degraded': return 'bg-yellow-500';
      case 'unhealthy': return 'bg-red-500';
      default: return 'bg-gray-500';
    }
  };

  const getQualityColor = (score: number) => {
    if (score >= 0.8) return 'text-green-600';
    if (score >= 0.6) return 'text-yellow-600';
    return 'text-red-600';
  };

  if (!selectedMode) {
    return (
      <div className="max-w-4xl mx-auto p-6 space-y-6">
        <div className="text-center space-y-4">
          <div className="flex items-center justify-center space-x-2">
            <Brain className="h-8 w-8 text-blue-600" />
            <h1 className="text-3xl font-bold">BMAD AI Agent System</h1>
          </div>
          <p className="text-gray-600">Research-Powered AI Agent Orchestrator</p>
        </div>

        {/* Integration Health Status */}
        {healthStatus && (
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Shield className="h-5 w-5" />
                <span>Integration Status</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="text-center">
                  <div className={`w-3 h-3 rounded-full mx-auto mb-2 ${getStatusColor(healthStatus.overall_status)}`} />
                  <p className="text-sm font-medium">Overall</p>
                  <p className="text-xs text-gray-500">{healthStatus.overall_status}</p>
                </div>
                <div className="text-center">
                  <div className={`w-3 h-3 rounded-full mx-auto mb-2 ${getStatusColor(healthStatus.research_bridge_status)}`} />
                  <p className="text-sm font-medium">Research Bridge</p>
                  <p className="text-xs text-gray-500">{healthStatus.research_bridge_status}</p>
                </div>
                <div className="text-center">
                  <div className={`w-3 h-3 rounded-full mx-auto mb-2 ${getStatusColor(healthStatus.agent_enhancer_status)}`} />
                  <p className="text-sm font-medium">Agent Enhancer</p>
                  <p className="text-xs text-gray-500">{healthStatus.agent_enhancer_status}</p>
                </div>
                <div className="text-center">
                  <div className={`w-3 h-3 rounded-full mx-auto mb-2 ${getStatusColor(healthStatus.workflow_coordinator_status)}`} />
                  <p className="text-sm font-medium">Workflow Coordinator</p>
                  <p className="text-xs text-gray-500">{healthStatus.workflow_coordinator_status}</p>
                </div>
              </div>
              {healthStatus.error_messages.length > 0 && (
                <div className="mt-4 p-3 bg-red-50 rounded-lg">
                  <p className="text-sm text-red-600 font-medium">Issues detected:</p>
                  <ul className="text-sm text-red-600 mt-1">
                    {healthStatus.error_messages.map((msg, idx) => (
                      <li key={idx}>• {msg}</li>
                    ))}
                  </ul>
                </div>
              )}
            </CardContent>
          </Card>
        )}

        {/* Mode Selection */}
        <Card>
          <CardHeader>
            <CardTitle>🎯 Mode Selection Required</CardTitle>
            <p className="text-gray-600">Please choose your workflow mode:</p>
          </CardHeader>
          <CardContent className="space-y-4">
            <div 
              className="p-6 border-2 border-blue-200 rounded-lg cursor-pointer hover:border-blue-400 transition-colors"
              onClick={() => setSelectedMode('documentation')}
            >
              <div className="flex items-start space-x-4">
                <FileText className="h-8 w-8 text-blue-600 mt-1" />
                <div>
                  <h3 className="text-xl font-semibold">1. Documentation Mode (Recommended)</h3>
                  <p className="text-gray-600 mt-2">
                    📋 Generate exactly 3 complete, final documents ready for developer handoff:
                  </p>
                  <ul className="text-sm text-gray-600 mt-2 space-y-1">
                    <li>• <code>prd.md</code> - Product Requirements Document (complete final product specifications)</li>
                    <li>• <code>architecture.md</code> - Technical architecture document (system design & implementation approach)</li>
                    <li>• <code>checklist.md</code> - Development checklist (acceptance criteria & implementation steps)</li>
                  </ul>
                  <div className="mt-3 space-y-1">
                    <p className="text-sm text-green-600">✅ Perfect for: Sending specifications to developers working in professional environments</p>
                    <p className="text-sm text-green-600">✅ Output: Standalone documents requiring no additional clarification</p>
                    <p className="text-sm text-blue-600">🔬 Enhanced with: Comprehensive research and evidence-based insights</p>
                  </div>
                </div>
              </div>
            </div>

            <div 
              className="p-6 border-2 border-gray-200 rounded-lg cursor-pointer hover:border-gray-400 transition-colors"
              onClick={() => setSelectedMode('development')}
            >
              <div className="flex items-start space-x-4">
                <Server className="h-8 w-8 text-gray-600 mt-1" />
                <div>
                  <h3 className="text-xl font-semibold">2. Full Development Mode</h3>
                  <p className="text-gray-600 mt-2">
                    🚀 Build the entire project within this chat session
                  </p>
                  <ul className="text-sm text-gray-600 mt-2 space-y-1">
                    <li>• Complete application development with AI agents</li>
                    <li>• Interactive development workflow</li>
                    <li>• Full implementation and testing</li>
                    <li>• Real-time research capabilities</li>
                  </ul>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (selectedMode === 'documentation') {
    return (
      <div className="max-w-6xl mx-auto p-6 space-y-6">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <FileText className="h-6 w-6 text-blue-600" />
            <h1 className="text-2xl font-bold">Research-Enhanced Documentation Mode</h1>
          </div>
          <Button variant="outline" onClick={() => setSelectedMode(null)}>
            Change Mode
          </Button>
        </div>

        {!isExecuting && !results && (
          <Card>
            <CardHeader>
              <CardTitle>Project Configuration</CardTitle>
              <p className="text-gray-600">Configure your project for research-enhanced documentation generation</p>
            </CardHeader>
            <CardContent className="space-y-6">
              <div>
                <label className="block text-sm font-medium mb-2">Project Description *</label>
                <textarea
                  className="w-full p-3 border rounded-lg"
                  rows={4}
                  placeholder="Describe your project, its goals, and key features..."
                  value={projectDescription}
                  onChange={(e) => setProjectDescription(e.target.value)}
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Requirements</label>
                {requirements.map((req, index) => (
                  <div key={index} className="flex space-x-2 mb-2">
                    <input
                      className="flex-1 p-2 border rounded"
                      placeholder="Enter a requirement..."
                      value={req}
                      onChange={(e) => updateRequirement(index, e.target.value)}
                    />
                    {requirements.length > 1 && (
                      <Button variant="outline" size="sm" onClick={() => removeRequirement(index)}>
                        Remove
                      </Button>
                    )}
                  </div>
                ))}
                <Button variant="outline" size="sm" onClick={addRequirement}>
                  Add Requirement
                </Button>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-2">Target Audience</label>
                  <input
                    className="w-full p-2 border rounded"
                    placeholder="e.g., Enterprise users, Developers"
                    value={targetAudience}
                    onChange={(e) => setTargetAudience(e.target.value)}
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium mb-2">Research Depth</label>
                  <select
                    className="w-full p-2 border rounded"
                    value={researchDepth}
                    onChange={(e) => setResearchDepth(e.target.value as any)}
                  >
                    <option value="Basic">Basic</option>
                    <option value="Standard">Standard</option>
                    <option value="Comprehensive">Comprehensive</option>
                    <option value="Expert">Expert</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium mb-2">Cost Limit ($)</label>
                  <input
                    type="number"
                    className="w-full p-2 border rounded"
                    value={costLimit}
                    onChange={(e) => setCostLimit(Number(e.target.value))}
                    min="5"
                    max="100"
                  />
                </div>
              </div>

              {error && (
                <div className="p-3 bg-red-50 border border-red-200 rounded-lg">
                  <div className="flex items-center space-x-2">
                    <AlertCircle className="h-4 w-4 text-red-600" />
                    <p className="text-red-600">{error}</p>
                  </div>
                </div>
              )}

              <Button 
                onClick={executeDocumentationMode}
                className="w-full"
                disabled={!projectDescription.trim()}
              >
                <Search className="h-4 w-4 mr-2" />
                Generate Research-Enhanced Documentation
              </Button>
            </CardContent>
          </Card>
        )}

        {isExecuting && (
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Brain className="h-5 w-5 animate-pulse" />
                <span>Executing Research-Enhanced Documentation Mode</span>
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span>{currentPhase}</span>
                  <span>{executionProgress}%</span>
                </div>
                <Progress value={executionProgress} className="w-full" />
              </div>
              
              <div className="grid grid-cols-3 gap-4 text-center">
                <div className="p-3 bg-blue-50 rounded-lg">
                  <TrendingUp className="h-6 w-6 mx-auto text-blue-600 mb-2" />
                  <p className="text-sm font-medium">Market Research</p>
                  <p className="text-xs text-gray-600">Product Manager (John)</p>
                </div>
                <div className="p-3 bg-green-50 rounded-lg">
                  <Server className="h-6 w-6 mx-auto text-green-600 mb-2" />
                  <p className="text-sm font-medium">Technology Analysis</p>
                  <p className="text-xs text-gray-600">Architect (Fred)</p>
                </div>
                <div className="p-3 bg-purple-50 rounded-lg">
                  <Shield className="h-6 w-6 mx-auto text-purple-600 mb-2" />
                  <p className="text-sm font-medium">Infrastructure Research</p>
                  <p className="text-xs text-gray-600">Platform Engineer (Alex)</p>
                </div>
              </div>
            </CardContent>
          </Card>
        )}

        {results && (
          <div className="space-y-6">
            {/* Results Summary */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center space-x-2">
                  <CheckCircle className="h-5 w-5 text-green-600" />
                  <span>Documentation Generation Completed</span>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                  <div className="text-center">
                    <p className="text-2xl font-bold text-blue-600">{results.research_summary.total_research_conducted}</p>
                    <p className="text-sm text-gray-600">Research Sessions</p>
                  </div>
                  <div className="text-center">
                    <p className="text-2xl font-bold text-green-600">{Math.round(results.research_summary.research_confidence_average * 100)}%</p>
                    <p className="text-sm text-gray-600">Avg Confidence</p>
                  </div>
                  <div className="text-center">
                    <p className="text-2xl font-bold text-purple-600">{results.research_summary.sources_analyzed}</p>
                    <p className="text-sm text-gray-600">Sources Analyzed</p>
                  </div>
                  <div className="text-center">
                    <p className="text-2xl font-bold text-orange-600">${results.cost_breakdown.total_cost.toFixed(2)}</p>
                    <p className="text-sm text-gray-600">Total Cost</p>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Quality Metrics */}
            <Card>
              <CardHeader>
                <CardTitle>Quality Metrics</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                  <div>
                    <p className="text-sm font-medium">Overall Confidence</p>
                    <p className={`text-lg font-bold ${getQualityColor(results.quality_metrics.overall_confidence_score)}`}>
                      {Math.round(results.quality_metrics.overall_confidence_score * 100)}%
                    </p>
                  </div>
                  <div>
                    <p className="text-sm font-medium">Source Diversity</p>
                    <p className={`text-lg font-bold ${getQualityColor(results.quality_metrics.source_diversity_score)}`}>
                      {Math.round(results.quality_metrics.source_diversity_score * 100)}%
                    </p>
                  </div>
                  <div>
                    <p className="text-sm font-medium">Evidence Completeness</p>
                    <p className={`text-lg font-bold ${getQualityColor(results.quality_metrics.evidence_completeness_score)}`}>
                      {Math.round(results.quality_metrics.evidence_completeness_score * 100)}%
                    </p>
                  </div>
                  <div>
                    <p className="text-sm font-medium">Quality Gates</p>
                    <p className="text-lg font-bold text-blue-600">
                      {results.quality_metrics.quality_gates_passed}/{results.quality_metrics.quality_gates_total}
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Deliverables */}
            <Card>
              <CardHeader>
                <CardTitle>Generated Deliverables</CardTitle>
              </CardHeader>
              <CardContent>
                <Tabs defaultValue="prd" className="w-full">
                  <TabsList className="grid w-full grid-cols-4">
                    <TabsTrigger value="prd">PRD</TabsTrigger>
                    <TabsTrigger value="architecture">Architecture</TabsTrigger>
                    <TabsTrigger value="checklist">Checklist</TabsTrigger>
                    <TabsTrigger value="research">Research</TabsTrigger>
                  </TabsList>
                  <TabsContent value="prd" className="mt-4">
                    <div className="p-4 bg-gray-50 rounded-lg">
                      <pre className="whitespace-pre-wrap text-sm">{results.deliverables.prd}</pre>
                    </div>
                  </TabsContent>
                  <TabsContent value="architecture" className="mt-4">
                    <div className="p-4 bg-gray-50 rounded-lg">
                      <pre className="whitespace-pre-wrap text-sm">{results.deliverables.architecture}</pre>
                    </div>
                  </TabsContent>
                  <TabsContent value="checklist" className="mt-4">
                    <div className="p-4 bg-gray-50 rounded-lg">
                      <pre className="whitespace-pre-wrap text-sm">{results.deliverables.checklist}</pre>
                    </div>
                  </TabsContent>
                  <TabsContent value="research" className="mt-4">
                    <div className="p-4 bg-gray-50 rounded-lg">
                      <pre className="whitespace-pre-wrap text-sm">{results.deliverables.research_appendix}</pre>
                    </div>
                  </TabsContent>
                </Tabs>
              </CardContent>
            </Card>
          </div>
        )}
      </div>
    );
  }

  // Development Mode UI would go here
  return (
    <div className="max-w-4xl mx-auto p-6">
      <Card>
        <CardHeader>
          <CardTitle>Development Mode</CardTitle>
        </CardHeader>
        <CardContent>
          <p>Development Mode interface coming soon...</p>
          <Button onClick={() => setSelectedMode(null)} className="mt-4">
            Back to Mode Selection
          </Button>
        </CardContent>
      </Card>
    </div>
  );
};
