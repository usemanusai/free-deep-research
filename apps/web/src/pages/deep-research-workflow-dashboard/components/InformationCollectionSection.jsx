import React from 'react';
import Icon from '../../../components/AppIcon';

const InformationCollectionSection = ({ 
  isExpanded, 
  onToggle, 
  status,
  progress,
  collectedSources 
}) => {
  const getStatusIcon = () => {
    switch (status) {
      case 'waiting':
        return 'Clock';
      case 'collecting':
        return 'Search';
      case 'completed':
        return 'CheckCircle2';
      default:
        return 'Database';
    }
  };

  const getStatusColor = () => {
    switch (status) {
      case 'waiting':
        return 'var(--color-warning)';
      case 'collecting':
        return 'var(--color-primary)';
      case 'completed':
        return 'var(--color-success)';
      default:
        return 'var(--color-muted-foreground)';
    }
  };

  const getStatusText = () => {
    switch (status) {
      case 'waiting':
        return 'Waiting for questions to be generated';
      case 'collecting':
        return `Collecting information... ${progress}%`;
      case 'completed':
        return `${collectedSources.length} sources collected`;
      default:
        return 'Ready to collect information';
    }
  };

  return (
    <div className="research-card">
      <div 
        className="flex items-center justify-between p-6 cursor-pointer research-hover"
        onClick={onToggle}
      >
        <div className="flex items-center space-x-4">
          <div className="flex items-center justify-center w-8 h-8 bg-muted rounded-full text-muted-foreground font-semibold">
            3
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">Information Collection</h3>
            <p className="text-sm text-muted-foreground">
              Gathering relevant information from multiple sources
            </p>
          </div>
        </div>
        <div className="flex items-center space-x-3">
          <div className="flex items-center space-x-2">
            <Icon 
              name={getStatusIcon()} 
              size={16} 
              color={getStatusColor()}
              className={status === 'collecting' ? 'animate-pulse' : ''}
            />
            <span className="text-sm" style={{ color: getStatusColor() }}>
              {getStatusText()}
            </span>
          </div>
          <Icon 
            name={isExpanded ? "ChevronUp" : "ChevronDown"} 
            size={20} 
            color="var(--color-muted-foreground)" 
          />
        </div>
      </div>

      {isExpanded && (
        <div className="px-6 pb-6 space-y-4 animate-fade-in">
          <div className="border-t border-border pt-6">
            {status === 'waiting' && (
              <div className="text-center py-8">
                <Icon name="Clock" size={48} color="var(--color-warning)" className="mx-auto mb-4" />
                <h4 className="text-lg font-medium text-foreground mb-2">
                  Waiting for Questions
                </h4>
                <p className="text-muted-foreground">
                  Questions need to be generated before information collection can begin
                </p>
              </div>
            )}

            {status === 'collecting' && (
              <div className="space-y-6">
                <div className="text-center py-4">
                  <Icon name="Search" size={48} color="var(--color-primary)" className="mx-auto mb-4 animate-pulse" />
                  <h4 className="text-lg font-medium text-foreground mb-2">
                    Collecting Information
                  </h4>
                  <p className="text-muted-foreground mb-4">
                    Searching and analyzing sources across the web...
                  </p>
                </div>

                <div className="space-y-3">
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-muted-foreground">Progress</span>
                    <span className="text-primary font-medium">{progress}%</span>
                  </div>
                  <div className="w-full bg-muted/20 rounded-full h-2">
                    <div 
                      className="bg-primary h-2 rounded-full progress-indicator"
                      style={{ width: `${progress}%` }}
                    />
                  </div>
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <div className="p-4 bg-muted/20 rounded-lg border border-border">
                    <div className="flex items-center space-x-2 mb-2">
                      <Icon name="Globe" size={16} color="var(--color-primary)" />
                      <span className="text-sm font-medium text-foreground">Web Sources</span>
                    </div>
                    <p className="text-2xl font-semibold text-primary">
                      {Math.floor(collectedSources.length * 0.6)}
                    </p>
                  </div>
                  <div className="p-4 bg-muted/20 rounded-lg border border-border">
                    <div className="flex items-center space-x-2 mb-2">
                      <Icon name="FileText" size={16} color="var(--color-accent)" />
                      <span className="text-sm font-medium text-foreground">Documents</span>
                    </div>
                    <p className="text-2xl font-semibold text-accent">
                      {Math.floor(collectedSources.length * 0.4)}
                    </p>
                  </div>
                </div>
              </div>
            )}

            {status === 'completed' && collectedSources.length > 0 && (
              <div className="space-y-4">
                <h4 className="font-medium text-foreground mb-4">
                  Information Sources Collected:
                </h4>
                <div className="space-y-3 max-h-64 overflow-y-auto">
                  {collectedSources.map((source, index) => (
                    <div key={index} className="p-4 bg-muted/20 rounded-lg border border-border">
                      <div className="flex items-start space-x-3">
                        <Icon 
                          name={source.type === 'web' ? 'Globe' : 'FileText'} 
                          size={16} 
                          color="var(--color-primary)" 
                          className="mt-1"
                        />
                        <div className="flex-1">
                          <h5 className="font-medium text-foreground text-sm mb-1">
                            {source.title}
                          </h5>
                          <p className="text-xs text-muted-foreground mb-2">
                            {source.description}
                          </p>
                          <div className="flex items-center space-x-4 text-xs text-muted-foreground">
                            <span>Type: {source.type}</span>
                            <span>Relevance: {source.relevance}%</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default InformationCollectionSection;