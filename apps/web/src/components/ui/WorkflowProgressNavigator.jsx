import React, { useState, useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import Icon from '../AppIcon';
import Button from './Button';

const WorkflowProgressNavigator = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const [workflowState, setWorkflowState] = useState({
    currentStage: 0,
    completedStages: [],
    stageData: {}
  });

  const workflowStages = [
    {
      id: 'dashboard',
      title: 'Research Dashboard',
      description: 'Overview and workflow orchestration',
      path: '/deep-research-workflow-dashboard',
      icon: 'LayoutDashboard',
      status: 'active'
    },
    {
      id: 'upload',
      title: 'Resource Upload',
      description: 'Upload and manage research materials',
      path: '/resource-upload-management',
      icon: 'Upload',
      status: 'pending'
    },
    {
      id: 'tracking',
      title: 'Progress Tracking',
      description: 'Monitor research workflow progress',
      path: '/research-progress-tracking',
      icon: 'TrendingUp',
      status: 'pending'
    },
    {
      id: 'report',
      title: 'Final Report',
      description: 'Generate and export research findings',
      path: '/final-report-generation',
      icon: 'FileText',
      status: 'pending'
    }
  ];

  useEffect(() => {
    // Determine current stage based on location
    const currentStageIndex = workflowStages.findIndex(
      stage => stage.path === location.pathname
    );
    
    if (currentStageIndex !== -1) {
      setWorkflowState(prev => ({
        ...prev,
        currentStage: currentStageIndex
      }));
    }
  }, [location.pathname]);

  const handleStageClick = (stageIndex, stagePath) => {
    // Allow navigation to completed stages or next stage
    const canNavigate = stageIndex <= workflowState.currentStage + 1 || 
                       workflowState.completedStages.includes(stageIndex);
    
    if (canNavigate) {
      navigate(stagePath);
    }
  };

  const getStageStatus = (stageIndex) => {
    if (workflowState.completedStages.includes(stageIndex)) {
      return 'completed';
    } else if (stageIndex === workflowState.currentStage) {
      return 'active';
    } else if (stageIndex === workflowState.currentStage + 1) {
      return 'next';
    } else {
      return 'pending';
    }
  };

  const getStageIcon = (stage, status) => {
    if (status === 'completed') {
      return 'CheckCircle2';
    } else if (status === 'active') {
      return stage.icon;
    } else {
      return stage.icon;
    }
  };

  const getStageStyles = (status) => {
    switch (status) {
      case 'completed':
        return 'bg-success/10 border-success/20 text-success-foreground';
      case 'active':
        return 'bg-primary/10 border-primary/20 text-primary-foreground';
      case 'next':
        return 'bg-card border-border text-card-foreground hover:bg-muted/50 cursor-pointer';
      default:
        return 'bg-muted/20 border-muted/20 text-muted-foreground cursor-not-allowed';
    }
  };

  const progressPercentage = ((workflowState.currentStage + 1) / workflowStages.length) * 100;

  return (
    <div className="workflow-section">
      {/* Progress Header */}
      <div className="flex items-center justify-between mb-8">
        <div>
          <h2 className="text-2xl font-semibold text-foreground mb-2">
            Research Workflow Progress
          </h2>
          <p className="text-muted-foreground">
            Complete each stage systematically to build comprehensive research
          </p>
        </div>
        <div className="text-right">
          <div className="text-sm text-muted-foreground mb-1">Overall Progress</div>
          <div className="text-2xl font-semibold text-primary">
            {Math.round(progressPercentage)}%
          </div>
        </div>
      </div>

      {/* Progress Bar */}
      <div className="mb-8">
        <div className="w-full bg-muted/20 rounded-full h-2">
          <div 
            className="bg-primary h-2 rounded-full progress-indicator"
            style={{ width: `${progressPercentage}%` }}
          />
        </div>
      </div>

      {/* Workflow Stages */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {workflowStages.map((stage, index) => {
          const status = getStageStatus(index);
          const iconName = getStageIcon(stage, status);
          const canNavigate = status === 'completed' || status === 'active' || status === 'next';

          return (
            <div
              key={stage.id}
              className={`research-card p-6 research-hover transition-all duration-200 ${getStageStyles(status)}`}
              onClick={() => canNavigate && handleStageClick(index, stage.path)}
            >
              {/* Stage Header */}
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center space-x-3">
                  <div className={`
                    w-10 h-10 rounded-lg flex items-center justify-center
                    ${status === 'completed' ? 'bg-success' : 
                      status === 'active' ? 'bg-primary' : 
                      status === 'next' ? 'bg-muted' : 'bg-muted/50'}
                  `}>
                    <Icon 
                      name={iconName} 
                      size={20} 
                      color={status === 'pending' ? 'var(--color-muted-foreground)' : 'white'} 
                    />
                  </div>
                  <div className="text-sm font-medium text-muted-foreground">
                    Stage {index + 1}
                  </div>
                </div>
                
                {status === 'active' && (
                  <div className="w-2 h-2 bg-primary rounded-full animate-pulse" />
                )}
              </div>

              {/* Stage Content */}
              <div className="space-y-2">
                <h3 className="font-semibold text-lg leading-tight">
                  {stage.title}
                </h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  {stage.description}
                </p>
              </div>

              {/* Stage Action */}
              <div className="mt-6">
                {status === 'completed' && (
                  <Button
                    variant="outline"
                    size="sm"
                    iconName="Eye"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleStageClick(index, stage.path);
                    }}
                    className="w-full"
                  >
                    Review
                  </Button>
                )}
                
                {status === 'active' && (
                  <Button
                    variant="default"
                    size="sm"
                    iconName="ArrowRight"
                    iconPosition="right"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleStageClick(index, stage.path);
                    }}
                    className="w-full"
                  >
                    Continue
                  </Button>
                )}
                
                {status === 'next' && (
                  <Button
                    variant="secondary"
                    size="sm"
                    iconName="Play"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleStageClick(index, stage.path);
                    }}
                    className="w-full"
                  >
                    Start
                  </Button>
                )}
                
                {status === 'pending' && (
                  <Button
                    variant="ghost"
                    size="sm"
                    disabled
                    className="w-full cursor-not-allowed"
                  >
                    Locked
                  </Button>
                )}
              </div>
            </div>
          );
        })}
      </div>

      {/* Quick Actions */}
      <div className="mt-8 flex flex-wrap gap-4 justify-center">
        <Button
          variant="outline"
          iconName="RotateCcw"
          onClick={() => navigate('/deep-research-workflow-dashboard')}
          className="research-hover"
        >
          Reset Workflow
        </Button>
        
        <Button
          variant="ghost"
          iconName="BookOpen"
          onClick={() => window.open('/docs/workflow-guide', '_blank')}
          className="research-hover"
        >
          Workflow Guide
        </Button>
        
        <Button
          variant="ghost"
          iconName="HelpCircle"
          onClick={() => window.open('/support', '_blank')}
          className="research-hover"
        >
          Get Help
        </Button>
      </div>
    </div>
  );
};

export default WorkflowProgressNavigator;