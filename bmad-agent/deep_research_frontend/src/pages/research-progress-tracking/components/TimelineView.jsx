import React from 'react';
import Icon from '../../../components/AppIcon';

const TimelineView = ({ milestones }) => {
  const getStatusColor = (status) => {
    switch (status) {
      case 'completed': return 'bg-success border-success';
      case 'active': return 'bg-primary border-primary';
      case 'pending': return 'bg-muted border-muted';
      case 'error': return 'bg-error border-error';
      default: return 'bg-muted border-muted';
    }
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

  const formatTime = (timestamp) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diffInMinutes = Math.floor((now - date) / (1000 * 60));
    
    if (diffInMinutes < 1) return 'Just now';
    if (diffInMinutes < 60) return `${diffInMinutes}m ago`;
    if (diffInMinutes < 1440) return `${Math.floor(diffInMinutes / 60)}h ago`;
    return date.toLocaleDateString();
  };

  return (
    <div className="space-y-6">
      <div className="research-card p-6">
        <div className="flex items-center justify-between mb-6">
          <h3 className="text-lg font-semibold text-foreground">Research Timeline</h3>
          <div className="flex items-center space-x-2">
            <Icon name="Clock" size={16} className="text-muted-foreground" />
            <span className="text-sm text-muted-foreground">
              Last updated: {formatTime(Date.now())}
            </span>
          </div>
        </div>

        <div className="relative">
          {/* Timeline Line */}
          <div className="absolute left-6 top-0 bottom-0 w-0.5 bg-border"></div>
          
          <div className="space-y-6">
            {milestones.map((milestone, index) => (
              <div key={milestone.id} className="relative flex items-start space-x-4">
                {/* Timeline Dot */}
                <div className={`
                  relative z-10 w-12 h-12 rounded-full border-2 flex items-center justify-center
                  ${getStatusColor(milestone.status)}
                `}>
                  <Icon 
                    name={getStatusIcon(milestone.status)} 
                    size={20} 
                    color="white" 
                  />
                  {milestone.status === 'active' && (
                    <div className="absolute inset-0 rounded-full bg-primary/20 animate-ping"></div>
                  )}
                </div>

                {/* Timeline Content */}
                <div className="flex-1 min-w-0 pb-6">
                  <div className="research-card p-4">
                    <div className="flex items-start justify-between mb-2">
                      <div>
                        <h4 className="font-medium text-foreground">{milestone.title}</h4>
                        <p className="text-sm text-muted-foreground">{milestone.description}</p>
                      </div>
                      <div className="text-right">
                        <span className="text-xs text-muted-foreground">
                          {formatTime(milestone.timestamp)}
                        </span>
                        {milestone.duration && (
                          <p className="text-xs text-muted-foreground">
                            Duration: {milestone.duration}
                          </p>
                        )}
                      </div>
                    </div>

                    {/* Milestone Details */}
                    {milestone.details && (
                      <div className="mt-3 space-y-2">
                        {milestone.details.map((detail, detailIndex) => (
                          <div key={detailIndex} className="flex items-center space-x-2 text-sm">
                            <Icon name="ArrowRight" size={12} className="text-muted-foreground" />
                            <span className="text-muted-foreground">{detail}</span>
                          </div>
                        ))}
                      </div>
                    )}

                    {/* Progress Metrics */}
                    {milestone.metrics && (
                      <div className="mt-3 grid grid-cols-2 gap-4">
                        {Object.entries(milestone.metrics).map(([key, value]) => (
                          <div key={key} className="text-center p-2 bg-muted/10 rounded">
                            <div className="text-lg font-semibold text-foreground">{value}</div>
                            <div className="text-xs text-muted-foreground capitalize">
                              {key.replace(/([A-Z])/g, ' $1').trim()}
                            </div>
                          </div>
                        ))}
                      </div>
                    )}

                    {/* Status Badge */}
                    <div className="mt-3 flex items-center justify-between">
                      <div className="flex items-center space-x-2">
                        <span className={`
                          px-2 py-1 text-xs font-medium rounded-full
                          ${milestone.status === 'completed' ? 'bg-success/10 text-success' :
                            milestone.status === 'active' ? 'bg-primary/10 text-primary' :
                            milestone.status === 'pending' ? 'bg-muted/10 text-muted-foreground' :
                            'bg-error/10 text-error'}
                        `}>
                          {milestone.status.charAt(0).toUpperCase() + milestone.status.slice(1)}
                        </span>
                        {milestone.stage && (
                          <span className="text-xs text-muted-foreground">
                            Stage {milestone.stage}
                          </span>
                        )}
                      </div>
                      
                      {milestone.progress !== undefined && (
                        <div className="flex items-center space-x-2">
                          <div className="w-16 bg-muted/20 rounded-full h-1.5">
                            <div 
                              className={`h-1.5 rounded-full transition-all duration-300 ${
                                milestone.status === 'completed' ? 'bg-success' :
                                milestone.status === 'active'? 'bg-primary' : 'bg-muted'
                              }`}
                              style={{ width: `${milestone.progress}%` }}
                            />
                          </div>
                          <span className="text-xs text-muted-foreground">
                            {milestone.progress}%
                          </span>
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Timeline Statistics */}
      <div className="research-card p-6">
        <h4 className="font-medium text-foreground mb-4">Timeline Statistics</h4>
        <div className="grid grid-cols-2 gap-4">
          <div className="text-center p-3 bg-success/10 rounded-lg">
            <div className="text-2xl font-bold text-success">
              {milestones.filter(m => m.status === 'completed').length}
            </div>
            <div className="text-sm text-muted-foreground">Completed</div>
          </div>
          <div className="text-center p-3 bg-primary/10 rounded-lg">
            <div className="text-2xl font-bold text-primary">
              {milestones.filter(m => m.status === 'active').length}
            </div>
            <div className="text-sm text-muted-foreground">Active</div>
          </div>
        </div>
        
        <div className="mt-4 text-center">
          <p className="text-sm text-muted-foreground">
            Total research time: {Math.floor(Math.random() * 120 + 30)} minutes
          </p>
        </div>
      </div>
    </div>
  );
};

export default TimelineView;