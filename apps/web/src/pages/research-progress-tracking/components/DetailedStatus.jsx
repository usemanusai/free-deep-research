import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const DetailedStatus = ({ currentStage, stageDetails }) => {
  const [expandedSections, setExpandedSections] = useState({
    current: true,
    completed: false,
    pending: false
  });

  const toggleSection = (section) => {
    setExpandedSections(prev => ({
      ...prev,
      [section]: !prev[section]
    }));
  };

  const getStatusIcon = (status) => {
    switch (status) {
      case 'completed': return 'CheckCircle2';
      case 'active': return 'Play';
      case 'pending': return 'Clock';
      case 'error': return 'AlertCircle';
      default: return 'Circle';
    }
  };

  const getStatusColor = (status) => {
    switch (status) {
      case 'completed': return 'text-success';
      case 'active': return 'text-primary';
      case 'pending': return 'text-muted-foreground';
      case 'error': return 'text-error';
      default: return 'text-muted-foreground';
    }
  };

  return (
    <div className="space-y-6">
      {/* Current Stage Header */}
      <div className="research-card p-6">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center space-x-3">
            <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
              <Icon name={currentStage.icon} size={20} color="white" />
            </div>
            <div>
              <h3 className="text-lg font-semibold text-foreground">{currentStage.title}</h3>
              <p className="text-sm text-muted-foreground">{currentStage.description}</p>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 bg-primary rounded-full animate-pulse"></div>
            <span className="text-sm text-primary font-medium">Active</span>
          </div>
        </div>
        
        <div className="bg-muted/10 rounded-lg p-4">
          <div className="flex items-center space-x-2 mb-2">
            <Icon name="Activity" size={16} className="text-primary" />
            <span className="text-sm font-medium text-foreground">Current Activity</span>
          </div>
          <p className="text-sm text-muted-foreground ml-6">
            {currentStage.currentActivity}
          </p>
        </div>
      </div>

      {/* Current Activities Section */}
      <div className="research-card">
        <button
          onClick={() => toggleSection('current')}
          className="w-full p-4 flex items-center justify-between text-left research-hover"
        >
          <div className="flex items-center space-x-3">
            <Icon name="Play" size={20} className="text-primary" />
            <h4 className="font-medium text-foreground">Current Activities</h4>
            <span className="bg-primary/10 text-primary text-xs px-2 py-1 rounded-full">
              {stageDetails.currentActivities.length}
            </span>
          </div>
          <Icon 
            name={expandedSections.current ? "ChevronUp" : "ChevronDown"} 
            size={20} 
            className="text-muted-foreground" 
          />
        </button>
        
        {expandedSections.current && (
          <div className="px-4 pb-4 space-y-3">
            {stageDetails.currentActivities.map((activity, index) => (
              <div key={index} className="flex items-start space-x-3 p-3 bg-muted/5 rounded-lg">
                <div className="w-6 h-6 bg-primary/10 rounded-full flex items-center justify-center mt-0.5">
                  <Icon name="Zap" size={12} className="text-primary" />
                </div>
                <div className="flex-1">
                  <p className="text-sm font-medium text-foreground">{activity.name}</p>
                  <p className="text-xs text-muted-foreground">{activity.description}</p>
                  <div className="flex items-center space-x-2 mt-2">
                    <div className="w-full bg-muted/20 rounded-full h-1.5">
                      <div 
                        className="bg-primary h-1.5 rounded-full transition-all duration-300"
                        style={{ width: `${activity.progress}%` }}
                      />
                    </div>
                    <span className="text-xs text-primary font-medium">
                      {activity.progress}%
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Completed Tasks Section */}
      <div className="research-card">
        <button
          onClick={() => toggleSection('completed')}
          className="w-full p-4 flex items-center justify-between text-left research-hover"
        >
          <div className="flex items-center space-x-3">
            <Icon name="CheckCircle2" size={20} className="text-success" />
            <h4 className="font-medium text-foreground">Completed Tasks</h4>
            <span className="bg-success/10 text-success text-xs px-2 py-1 rounded-full">
              {stageDetails.completedTasks.length}
            </span>
          </div>
          <Icon 
            name={expandedSections.completed ? "ChevronUp" : "ChevronDown"} 
            size={20} 
            className="text-muted-foreground" 
          />
        </button>
        
        {expandedSections.completed && (
          <div className="px-4 pb-4 space-y-2">
            {stageDetails.completedTasks.map((task, index) => (
              <div key={index} className="flex items-center space-x-3 p-2">
                <Icon name="Check" size={16} className="text-success" />
                <span className="text-sm text-foreground">{task.name}</span>
                <span className="text-xs text-muted-foreground ml-auto">
                  {task.completedAt}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Pending Actions Section */}
      <div className="research-card">
        <button
          onClick={() => toggleSection('pending')}
          className="w-full p-4 flex items-center justify-between text-left research-hover"
        >
          <div className="flex items-center space-x-3">
            <Icon name="Clock" size={20} className="text-warning" />
            <h4 className="font-medium text-foreground">Pending Actions</h4>
            <span className="bg-warning/10 text-warning text-xs px-2 py-1 rounded-full">
              {stageDetails.pendingActions.length}
            </span>
          </div>
          <Icon 
            name={expandedSections.pending ? "ChevronUp" : "ChevronDown"} 
            size={20} 
            className="text-muted-foreground" 
          />
        </button>
        
        {expandedSections.pending && (
          <div className="px-4 pb-4 space-y-3">
            {stageDetails.pendingActions.map((action, index) => (
              <div key={index} className="flex items-start justify-between p-3 bg-muted/5 rounded-lg">
                <div className="flex items-start space-x-3">
                  <Icon 
                    name={getStatusIcon(action.status)} 
                    size={16} 
                    className={getStatusColor(action.status)} 
                  />
                  <div>
                    <p className="text-sm font-medium text-foreground">{action.name}</p>
                    <p className="text-xs text-muted-foreground">{action.description}</p>
                  </div>
                </div>
                {action.actionRequired && (
                  <Button
                    variant="outline"
                    size="xs"
                    iconName="Play"
                    onClick={() => console.log('Action triggered:', action.id)}
                  >
                    Start
                  </Button>
                )}
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Error Handling */}
      {stageDetails.errors && stageDetails.errors.length > 0 && (
        <div className="research-card p-4 border-error/20 bg-error/5">
          <div className="flex items-center space-x-2 mb-3">
            <Icon name="AlertTriangle" size={20} className="text-error" />
            <h4 className="font-medium text-error">Issues Detected</h4>
          </div>
          <div className="space-y-2">
            {stageDetails.errors.map((error, index) => (
              <div key={index} className="flex items-start justify-between">
                <div className="flex-1">
                  <p className="text-sm text-foreground">{error.message}</p>
                  <p className="text-xs text-muted-foreground">{error.timestamp}</p>
                </div>
                <Button
                  variant="outline"
                  size="xs"
                  iconName="RotateCcw"
                  onClick={() => console.log('Retry action:', error.id)}
                >
                  Retry
                </Button>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default DetailedStatus;