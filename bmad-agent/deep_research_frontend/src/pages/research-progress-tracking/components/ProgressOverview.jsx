import React from 'react';
import Icon from '../../../components/AppIcon';

const ProgressOverview = ({ workflowStages, overallProgress }) => {
  const getProgressColor = (progress) => {
    if (progress >= 100) return 'text-success';
    if (progress >= 75) return 'text-primary';
    if (progress >= 50) return 'text-warning';
    return 'text-muted-foreground';
  };

  const getProgressBgColor = (progress) => {
    if (progress >= 100) return 'bg-success';
    if (progress >= 75) return 'bg-primary';
    if (progress >= 50) return 'bg-warning';
    return 'bg-muted';
  };

  return (
    <div className="space-y-6">
      {/* Overall Progress Card */}
      <div className="research-card p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-foreground">Overall Progress</h3>
          <div className="flex items-center space-x-2">
            <Icon name="Clock" size={16} className="text-muted-foreground" />
            <span className="text-sm text-muted-foreground">
              Est. {overallProgress.estimatedTimeRemaining}
            </span>
          </div>
        </div>
        
        <div className="relative w-32 h-32 mx-auto mb-4">
          <svg className="w-32 h-32 transform -rotate-90" viewBox="0 0 120 120">
            <circle
              cx="60"
              cy="60"
              r="50"
              stroke="var(--color-muted)"
              strokeWidth="8"
              fill="none"
              className="opacity-20"
            />
            <circle
              cx="60"
              cy="60"
              r="50"
              stroke="var(--color-primary)"
              strokeWidth="8"
              fill="none"
              strokeDasharray={`${2 * Math.PI * 50}`}
              strokeDashoffset={`${2 * Math.PI * 50 * (1 - overallProgress.percentage / 100)}`}
              className="transition-all duration-500 ease-out"
            />
          </svg>
          <div className="absolute inset-0 flex items-center justify-center">
            <span className="text-2xl font-bold text-primary">
              {overallProgress.percentage}%
            </span>
          </div>
        </div>
        
        <div className="text-center">
          <p className="text-sm text-muted-foreground mb-2">
            {overallProgress.completedStages} of {workflowStages.length} stages completed
          </p>
          <div className="flex items-center justify-center space-x-2">
            <div className="w-2 h-2 bg-primary rounded-full animate-pulse"></div>
            <span className="text-sm text-foreground font-medium">
              {overallProgress.currentActivity}
            </span>
          </div>
        </div>
      </div>

      {/* Stage Progress Cards */}
      <div className="space-y-4">
        <h4 className="text-md font-medium text-foreground">Stage Progress</h4>
        {workflowStages.map((stage, index) => (
          <div key={stage.id} className="research-card p-4">
            <div className="flex items-center justify-between mb-3">
              <div className="flex items-center space-x-3">
                <div className={`
                  w-8 h-8 rounded-lg flex items-center justify-center
                  ${stage.status === 'completed' ? 'bg-success' : 
                    stage.status === 'active' ? 'bg-primary' : 
                    stage.status === 'pending' ? 'bg-muted' : 'bg-error'}
                `}>
                  <Icon 
                    name={stage.status === 'completed' ? 'CheckCircle2' : stage.icon} 
                    size={16} 
                    color="white" 
                  />
                </div>
                <div>
                  <h5 className="font-medium text-foreground">{stage.title}</h5>
                  <p className="text-xs text-muted-foreground">{stage.description}</p>
                </div>
              </div>
              <div className="text-right">
                <span className={`text-sm font-medium ${getProgressColor(stage.progress)}`}>
                  {stage.progress}%
                </span>
                {stage.estimatedTime && (
                  <p className="text-xs text-muted-foreground">
                    {stage.estimatedTime}
                  </p>
                )}
              </div>
            </div>
            
            <div className="w-full bg-muted/20 rounded-full h-2">
              <div 
                className={`h-2 rounded-full transition-all duration-500 ${getProgressBgColor(stage.progress)}`}
                style={{ width: `${stage.progress}%` }}
              />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ProgressOverview;