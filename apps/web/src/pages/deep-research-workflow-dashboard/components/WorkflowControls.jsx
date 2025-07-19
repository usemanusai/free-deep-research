import React from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const WorkflowControls = ({ 
  isWorkflowActive, 
  onStartWorkflow, 
  onPauseWorkflow, 
  onResetWorkflow,
  workflowProgress 
}) => {
  return (
    <div className="research-card p-6">
      <div className="flex flex-col lg:flex-row items-center justify-between space-y-4 lg:space-y-0">
        <div className="flex items-center space-x-4">
          <div className="flex items-center justify-center w-12 h-12 bg-primary/10 rounded-full">
            <Icon name="Zap" size={24} color="var(--color-primary)" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">Workflow Control</h3>
            <p className="text-sm text-muted-foreground">
              {isWorkflowActive ? 
                `Research in progress... ${workflowProgress}% complete` : 
                'Start your deep research workflow'
              }
            </p>
          </div>
        </div>

        <div className="flex items-center space-x-3">
          {!isWorkflowActive ? (
            <Button
              variant="default"
              size="lg"
              iconName="Play"
              onClick={onStartWorkflow}
              className="research-hover bg-primary hover:bg-primary/90"
            >
              Start Thinking
            </Button>
          ) : (
            <>
              <Button
                variant="outline"
                iconName="Pause"
                onClick={onPauseWorkflow}
                className="research-hover"
              >
                Pause
              </Button>
              <Button
                variant="ghost"
                iconName="RotateCcw"
                onClick={onResetWorkflow}
                className="research-hover"
              >
                Reset
              </Button>
            </>
          )}
        </div>
      </div>

      {isWorkflowActive && (
        <div className="mt-6 pt-6 border-t border-border">
          <div className="flex items-center justify-between text-sm mb-2">
            <span className="text-muted-foreground">Overall Progress</span>
            <span className="text-primary font-medium">{workflowProgress}%</span>
          </div>
          <div className="w-full bg-muted/20 rounded-full h-2">
            <div 
              className="bg-primary h-2 rounded-full progress-indicator"
              style={{ width: `${workflowProgress}%` }}
            />
          </div>
        </div>
      )}
    </div>
  );
};

export default WorkflowControls;