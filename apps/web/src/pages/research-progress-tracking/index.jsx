import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import Header from '../../components/ui/Header';
import WorkflowProgressNavigator from '../../components/ui/WorkflowProgressNavigator';
import ProgressOverview from './components/ProgressOverview';
import DetailedStatus from './components/DetailedStatus';
import TimelineView from './components/TimelineView';
import WorkflowControls from './components/WorkflowControls';
import Icon from '../../components/AppIcon';
import Button from '../../components/ui/Button';

const ResearchProgressTracking = () => {
  const navigate = useNavigate();
  const [activeView, setActiveView] = useState('overview');
  const [refreshInterval, setRefreshInterval] = useState(5000);
  const [lastUpdated, setLastUpdated] = useState(new Date());

  // Mock data for workflow stages
  const workflowStages = [
    {
      id: 'research-topics',
      title: 'Research Topics',
      description: 'Define and validate research topics',
      icon: 'Search',
      status: 'completed',
      progress: 100,
      estimatedTime: 'Completed',
      currentActivity: 'Topic validation complete'
    },
    {
      id: 'ask-question',
      title: 'Ask Question',
      description: 'Formulate research questions',
      icon: 'HelpCircle',
      status: 'completed',
      progress: 100,
      estimatedTime: 'Completed',
      currentActivity: 'Questions formulated'
    },
    {
      id: 'information-collection',
      title: 'Information Collection',
      description: 'Gather and analyze data',
      icon: 'Database',
      status: 'active',
      progress: 65,
      estimatedTime: '15 min remaining',
      currentActivity: 'Processing web sources'
    },
    {
      id: 'final-report',
      title: 'Final Report',
      description: 'Generate comprehensive report',
      icon: 'FileText',
      status: 'pending',
      progress: 0,
      estimatedTime: '25 min',
      currentActivity: 'Waiting for data collection'
    }
  ];

  // Mock data for overall progress
  const overallProgress = {
    percentage: 67,
    completedStages: 2,
    estimatedTimeRemaining: '40 min',
    currentActivity: 'Analyzing research data from multiple sources'
  };

  // Mock data for current stage details
  const currentStage = workflowStages.find(stage => stage.status === 'active') || workflowStages[2];

  const stageDetails = {
    currentActivities: [
      {
        name: 'Web Source Analysis',
        description: 'Extracting relevant information from web pages',
        progress: 78
      },
      {
        name: 'Document Processing',
        description: 'Analyzing uploaded research documents',
        progress: 45
      },
      {
        name: 'Data Validation',
        description: 'Verifying information accuracy and relevance',
        progress: 23
      }
    ],
    completedTasks: [
      {
        name: 'Topic Definition',
        completedAt: '2 hours ago'
      },
      {
        name: 'Question Formulation',
        completedAt: '1 hour ago'
      },
      {
        name: 'Source Identification',
        completedAt: '45 min ago'
      },
      {
        name: 'Initial Data Collection',
        completedAt: '30 min ago'
      }
    ],
    pendingActions: [
      {
        id: 'validate-sources',
        name: 'Source Validation',
        description: 'Review and validate collected sources',
        status: 'pending',
        actionRequired: true
      },
      {
        id: 'quality-check',
        name: 'Quality Assessment',
        description: 'Assess information quality and relevance',
        status: 'pending',
        actionRequired: false
      }
    ],
    errors: [
      {
        id: 'api-timeout',
        message: 'API timeout while processing external source',
        timestamp: '5 min ago'
      }
    ]
  };

  // Mock data for timeline milestones
  const milestones = [
    {
      id: 'workflow-start',
      title: 'Workflow Initiated',
      description: 'Deep research workflow started successfully',
      timestamp: Date.now() - 7200000, // 2 hours ago
      status: 'completed',
      stage: 1,
      duration: '2 min',
      progress: 100,
      details: [
        'Research parameters configured',
        'Initial validation completed',
        'Resources allocated'
      ],
      metrics: {
        tasksCompleted: 3,
        resourcesAllocated: 5
      }
    },
    {
      id: 'topics-defined',
      title: 'Research Topics Defined',
      description: 'Successfully identified and validated research topics',
      timestamp: Date.now() - 6900000, // 1h 55m ago
      status: 'completed',
      stage: 1,
      duration: '15 min',
      progress: 100,
      details: [
        'Primary topics identified',
        'Secondary topics mapped',
        'Topic relevance validated'
      ],
      metrics: {
        topicsIdentified: 8,
        validationScore: 94
      }
    },
    {
      id: 'questions-formulated',
      title: 'Research Questions Formulated',
      description: 'Generated comprehensive research questions',
      timestamp: Date.now() - 3600000, // 1 hour ago
      status: 'completed',
      stage: 2,
      duration: '20 min',
      progress: 100,
      details: [
        'Primary questions generated',
        'Sub-questions identified',
        'Question hierarchy established'
      ],
      metrics: {
        questionsGenerated: 12,
        complexityScore: 87
      }
    },
    {
      id: 'data-collection-started',
      title: 'Information Collection Started',
      description: 'Began systematic data collection process',
      timestamp: Date.now() - 2700000, // 45 min ago
      status: 'completed',
      stage: 3,
      duration: '5 min',
      progress: 100,
      details: [
        'Collection strategy defined',
        'Source prioritization completed',
        'Data extraction initiated'
      ],
      metrics: {
        sourcesIdentified: 25,
        priorityScore: 91
      }
    },
    {
      id: 'web-analysis-active',
      title: 'Web Source Analysis',
      description: 'Currently analyzing web-based research sources',
      timestamp: Date.now() - 1800000, // 30 min ago
      status: 'active',
      stage: 3,
      progress: 65,
      details: [
        'Processing 15 web sources',
        'Content extraction in progress',
        'Relevance scoring active'
      ],
      metrics: {
        sourcesProcessed: 10,
        relevanceScore: 82
      }
    },
    {
      id: 'report-preparation',
      title: 'Report Generation Preparation',
      description: 'Preparing for final report generation phase',
      timestamp: Date.now() + 1500000, // 25 min from now
      status: 'pending',
      stage: 4,
      progress: 0,
      details: [
        'Data compilation pending',
        'Template selection required',
        'Quality review scheduled'
      ]
    }
  ];

  // Mock workflow status
  const [workflowStatus, setWorkflowStatus] = useState({
    status: 'running',
    apiCalls: 247,
    uptime: '2h 15m'
  });

  // Auto-refresh functionality
  useEffect(() => {
    const interval = setInterval(() => {
      setLastUpdated(new Date());
      // Simulate real-time updates
      if (Math.random() > 0.7) {
        // Randomly update progress
        workflowStages[2].progress = Math.min(100, workflowStages[2].progress + Math.floor(Math.random() * 5));
      }
    }, refreshInterval);

    return () => clearInterval(interval);
  }, [refreshInterval]);

  const handleStageControl = (action) => {
    console.log('Stage control action:', action);
    
    switch (action) {
      case 'pause':
        setWorkflowStatus(prev => ({ ...prev, status: 'paused' }));
        break;
      case 'resume':
        setWorkflowStatus(prev => ({ ...prev, status: 'running' }));
        break;
      case 'stop':
        setWorkflowStatus(prev => ({ ...prev, status: 'stopped' }));
        break;
      case 'restart':
        setWorkflowStatus(prev => ({ ...prev, status: 'running' }));
        // Reset progress
        workflowStages.forEach(stage => {
          if (stage.id !== 'research-topics') {
            stage.status = 'pending';
            stage.progress = 0;
          }
        });
        break;
      default:
        console.log('Unhandled action:', action);
    }
  };

  const handleViewChange = (view) => {
    setActiveView(view);
  };

  const handleRefreshIntervalChange = (interval) => {
    setRefreshInterval(interval);
  };

  return (
    <div className="min-h-screen bg-background">
      <Header />
      
      <div className="max-w-7xl mx-auto px-6 py-8">
        {/* Page Header */}
        <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between mb-8">
          <div>
            <h1 className="text-3xl font-bold text-foreground mb-2">
              Research Progress Tracking
            </h1>
            <p className="text-muted-foreground">
              Monitor your research workflow in real-time with detailed progress insights
            </p>
          </div>
          
          <div className="flex items-center space-x-4 mt-4 lg:mt-0">
            <div className="flex items-center space-x-2 text-sm text-muted-foreground">
              <Icon name="Clock" size={16} />
              <span>Last updated: {lastUpdated.toLocaleTimeString()}</span>
            </div>
            
            <Button
              variant="outline"
              iconName="RefreshCw"
              onClick={() => setLastUpdated(new Date())}
              className="research-hover"
            >
              Refresh
            </Button>
          </div>
        </div>

        {/* View Toggle */}
        <div className="flex flex-wrap gap-2 mb-8">
          <Button
            variant={activeView === 'overview' ? 'default' : 'outline'}
            iconName="BarChart3"
            onClick={() => handleViewChange('overview')}
            className="research-hover"
          >
            Overview
          </Button>
          <Button
            variant={activeView === 'detailed' ? 'default' : 'outline'}
            iconName="List"
            onClick={() => handleViewChange('detailed')}
            className="research-hover"
          >
            Detailed Status
          </Button>
          <Button
            variant={activeView === 'timeline' ? 'default' : 'outline'}
            iconName="Clock"
            onClick={() => handleViewChange('timeline')}
            className="research-hover"
          >
            Timeline
          </Button>
          <Button
            variant={activeView === 'controls' ? 'default' : 'outline'}
            iconName="Settings"
            onClick={() => handleViewChange('controls')}
            className="research-hover"
          >
            Controls
          </Button>
        </div>

        {/* Main Content Grid */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
          {/* Desktop: 3-column layout, Mobile: Single column */}
          {activeView === 'overview' && (
            <>
              {/* Left Column - Progress Overview */}
              <div className="lg:col-span-4">
                <ProgressOverview 
                  workflowStages={workflowStages}
                  overallProgress={overallProgress}
                />
              </div>
              
              {/* Center Column - Detailed Status */}
              <div className="lg:col-span-5">
                <DetailedStatus 
                  currentStage={currentStage}
                  stageDetails={stageDetails}
                />
              </div>
              
              {/* Right Column - Timeline */}
              <div className="lg:col-span-3">
                <TimelineView milestones={milestones.slice(0, 4)} />
              </div>
            </>
          )}

          {activeView === 'detailed' && (
            <div className="lg:col-span-12">
              <DetailedStatus 
                currentStage={currentStage}
                stageDetails={stageDetails}
              />
            </div>
          )}

          {activeView === 'timeline' && (
            <div className="lg:col-span-12">
              <TimelineView milestones={milestones} />
            </div>
          )}

          {activeView === 'controls' && (
            <div className="lg:col-span-12">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <WorkflowControls 
                  currentStage={currentStage}
                  onStageControl={handleStageControl}
                  workflowStatus={workflowStatus}
                />
                <div>
                  <ProgressOverview 
                    workflowStages={workflowStages}
                    overallProgress={overallProgress}
                  />
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Workflow Navigator */}
        <div className="mt-12">
          <WorkflowProgressNavigator />
        </div>

        {/* Quick Navigation */}
        <div className="mt-8 flex flex-wrap gap-4 justify-center">
          <Button
            variant="outline"
            iconName="ArrowLeft"
            onClick={() => navigate('/resource-upload-management')}
            className="research-hover"
          >
            Back to Upload
          </Button>
          
          <Button
            variant="default"
            iconName="ArrowRight"
            iconPosition="right"
            onClick={() => navigate('/final-report-generation')}
            disabled={overallProgress.percentage < 100}
            className="research-hover"
          >
            View Final Report
          </Button>
          
          <Button
            variant="ghost"
            iconName="Home"
            onClick={() => navigate('/deep-research-workflow-dashboard')}
            className="research-hover"
          >
            Dashboard
          </Button>
        </div>
      </div>

      {/* Footer */}
      <footer className="border-t border-border mt-16">
        <div className="max-w-7xl mx-auto px-6 py-8">
          <div className="text-center">
            <p className="text-muted-foreground">
              Created with ❤️ by U14App team • Deep Research v0.9.18 • {new Date().getFullYear()}
            </p>
          </div>
        </div>
      </footer>
    </div>
  );
};

export default ResearchProgressTracking;