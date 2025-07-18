import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const WorkflowControls = ({ currentStage, onStageControl, workflowStatus }) => {
  const [showConfirmDialog, setShowConfirmDialog] = useState(null);

  const handleControlAction = (action) => {
    if (action === 'restart') {
      setShowConfirmDialog('restart');
    } else {
      onStageControl(action);
    }
  };

  const confirmAction = (action) => {
    onStageControl(action);
    setShowConfirmDialog(null);
  };

  const getStatusColor = (status) => {
    switch (status) {
      case 'running': return 'text-success';
      case 'paused': return 'text-warning';
      case 'error': return 'text-error';
      default: return 'text-muted-foreground';
    }
  };

  const getStatusIcon = (status) => {
    switch (status) {
      case 'running': return 'Play';
      case 'paused': return 'Pause';
      case 'error': return 'AlertCircle';
      default: return 'Clock';
    }
  };

  return (
    <div className="space-y-6">
      {/* Workflow Status Card */}
      <div className="research-card p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-foreground">Workflow Controls</h3>
          <div className="flex items-center space-x-2">
            <Icon 
              name={getStatusIcon(workflowStatus.status)} 
              size={16} 
              className={getStatusColor(workflowStatus.status)} 
            />
            <span className={`text-sm font-medium ${getStatusColor(workflowStatus.status)}`}>
              {workflowStatus.status.charAt(0).toUpperCase() + workflowStatus.status.slice(1)}
            </span>
          </div>
        </div>

        <div className="bg-muted/10 rounded-lg p-4 mb-4">
          <div className="flex items-center space-x-2 mb-2">
            <Icon name="Target" size={16} className="text-primary" />
            <span className="text-sm font-medium text-foreground">Current Stage</span>
          </div>
          <p className="text-sm text-muted-foreground ml-6">
            {currentStage.title} - {currentStage.description}
          </p>
        </div>

        {/* Control Buttons */}
        <div className="grid grid-cols-2 gap-3">
          {workflowStatus.status === 'running' ? (
            <Button
              variant="outline"
              iconName="Pause"
              onClick={() => handleControlAction('pause')}
              className="research-hover"
            >
              Pause Workflow
            </Button>
          ) : (
            <Button
              variant="default"
              iconName="Play"
              onClick={() => handleControlAction('resume')}
              className="research-hover"
            >
              Resume Workflow
            </Button>
          )}
          
          <Button
            variant="outline"
            iconName="Square"
            onClick={() => handleControlAction('stop')}
            className="research-hover"
          >
            Stop Workflow
          </Button>
          
          <Button
            variant="secondary"
            iconName="SkipForward"
            onClick={() => handleControlAction('skip')}
            disabled={currentStage.status === 'completed'}
            className="research-hover"
          >
            Skip Stage
          </Button>
          
          <Button
            variant="destructive"
            iconName="RotateCcw"
            onClick={() => handleControlAction('restart')}
            className="research-hover"
          >
            Restart Workflow
          </Button>
        </div>
      </div>

      {/* Stage-Specific Controls */}
      <div className="research-card p-6">
        <h4 className="font-medium text-foreground mb-4">Stage Controls</h4>
        
        <div className="space-y-3">
          <Button
            variant="outline"
            iconName="RefreshCw"
            onClick={() => handleControlAction('retry-stage')}
            fullWidth
            disabled={currentStage.status !== 'error'}
            className="justify-start research-hover"
          >
            Retry Current Stage
          </Button>
          
          <Button
            variant="ghost"
            iconName="Settings"
            onClick={() => handleControlAction('configure-stage')}
            fullWidth
            className="justify-start research-hover"
          >
            Configure Stage Parameters
          </Button>
          
          <Button
            variant="ghost"
            iconName="Eye"
            onClick={() => handleControlAction('view-logs')}
            fullWidth
            className="justify-start research-hover"
          >
            View Stage Logs
          </Button>
        </div>
      </div>

      {/* Performance Metrics */}
      <div className="research-card p-6">
        <h4 className="font-medium text-foreground mb-4">Performance Metrics</h4>
        
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Processing Speed</span>
            <div className="flex items-center space-x-2">
              <div className="w-16 bg-muted/20 rounded-full h-2">
                <div className="bg-success h-2 rounded-full" style={{ width: '78%' }}></div>
              </div>
              <span className="text-sm text-success font-medium">78%</span>
            </div>
          </div>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Memory Usage</span>
            <div className="flex items-center space-x-2">
              <div className="w-16 bg-muted/20 rounded-full h-2">
                <div className="bg-warning h-2 rounded-full" style={{ width: '45%' }}></div>
              </div>
              <span className="text-sm text-warning font-medium">45%</span>
            </div>
          </div>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">API Calls</span>
            <span className="text-sm text-foreground font-medium">
              {workflowStatus.apiCalls || 0} / 1000
            </span>
          </div>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Uptime</span>
            <span className="text-sm text-foreground font-medium">
              {workflowStatus.uptime || '0h 0m'}
            </span>
          </div>
        </div>
      </div>

      {/* Confirmation Dialog */}
      {showConfirmDialog && (
        <div className="fixed inset-0 bg-background/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
          <div className="research-card p-6 max-w-md w-full">
            <div className="flex items-center space-x-3 mb-4">
              <Icon name="AlertTriangle" size={24} className="text-warning" />
              <h3 className="text-lg font-semibold text-foreground">Confirm Action</h3>
            </div>
            
            <p className="text-sm text-muted-foreground mb-6">
              Are you sure you want to restart the workflow? This will reset all progress and start from the beginning.
            </p>
            
            <div className="flex space-x-3">
              <Button
                variant="destructive"
                onClick={() => confirmAction('restart')}
                className="flex-1"
              >
                Restart Workflow
              </Button>
              <Button
                variant="outline"
                onClick={() => setShowConfirmDialog(null)}
                className="flex-1"
              >
                Cancel
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Quick Actions */}
      <div className="research-card p-6">
        <h4 className="font-medium text-foreground mb-4">Quick Actions</h4>
        
        <div className="grid grid-cols-2 gap-3">
          <Button
            variant="ghost"
            iconName="Download"
            size="sm"
            onClick={() => handleControlAction('export-progress')}
            className="research-hover"
          >
            Export Progress
          </Button>
          
          <Button
            variant="ghost"
            iconName="Share2"
            size="sm"
            onClick={() => handleControlAction('share-status')}
            className="research-hover"
          >
            Share Status
          </Button>
          
          <Button
            variant="ghost"
            iconName="Bell"
            size="sm"
            onClick={() => handleControlAction('setup-notifications')}
            className="research-hover"
          >
            Notifications
          </Button>
          
          <Button
            variant="ghost"
            iconName="HelpCircle"
            size="sm"
            onClick={() => handleControlAction('get-help')}
            className="research-hover"
          >
            Get Help
          </Button>
        </div>
      </div>
    </div>
  );
};

export default WorkflowControls;