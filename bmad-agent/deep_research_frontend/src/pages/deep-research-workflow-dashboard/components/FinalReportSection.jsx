import React from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const FinalReportSection = ({ 
  isExpanded, 
  onToggle, 
  status,
  reportData,
  onGenerateReport,
  onViewReport 
}) => {
  const getStatusIcon = () => {
    switch (status) {
      case 'waiting':
        return 'Clock';
      case 'generating':
        return 'FileText';
      case 'completed':
        return 'CheckCircle2';
      default:
        return 'FileText';
    }
  };

  const getStatusColor = () => {
    switch (status) {
      case 'waiting':
        return 'var(--color-warning)';
      case 'generating':
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
        return 'Waiting for information collection';
      case 'generating':
        return 'Generating comprehensive report...';
      case 'completed':
        return 'Report ready for review';
      default:
        return 'Ready to generate report';
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
            4
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">Final Report</h3>
            <p className="text-sm text-muted-foreground">
              Comprehensive research report with findings and insights
            </p>
          </div>
        </div>
        <div className="flex items-center space-x-3">
          <div className="flex items-center space-x-2">
            <Icon 
              name={getStatusIcon()} 
              size={16} 
              color={getStatusColor()}
              className={status === 'generating' ? 'animate-pulse' : ''}
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
                  Waiting for Data Collection
                </h4>
                <p className="text-muted-foreground">
                  Information collection must be completed before generating the final report
                </p>
              </div>
            )}

            {status === 'ready' && (
              <div className="text-center py-8">
                <Icon name="FileText" size={48} color="var(--color-primary)" className="mx-auto mb-4" />
                <h4 className="text-lg font-medium text-foreground mb-2">
                  Ready to Generate Report
                </h4>
                <p className="text-muted-foreground mb-6">
                  All information has been collected. Generate your comprehensive research report.
                </p>
                <Button
                  variant="default"
                  iconName="FileText"
                  onClick={onGenerateReport}
                  className="research-hover"
                >
                  Generate Report
                </Button>
              </div>
            )}

            {status === 'generating' && (
              <div className="text-center py-8">
                <Icon name="FileText" size={48} color="var(--color-primary)" className="mx-auto mb-4 animate-pulse" />
                <h4 className="text-lg font-medium text-foreground mb-2">
                  Generating Report
                </h4>
                <p className="text-muted-foreground mb-4">
                  Analyzing data and creating comprehensive research report...
                </p>
                <div className="w-full bg-muted/20 rounded-full h-2 max-w-md mx-auto">
                  <div className="bg-primary h-2 rounded-full progress-indicator animate-pulse" style={{ width: '75%' }} />
                </div>
              </div>
            )}

            {status === 'completed' && reportData && (
              <div className="space-y-6">
                <div className="text-center py-4">
                  <Icon name="CheckCircle2" size={48} color="var(--color-success)" className="mx-auto mb-4" />
                  <h4 className="text-lg font-medium text-foreground mb-2">
                    Report Generated Successfully
                  </h4>
                  <p className="text-muted-foreground">
                    Your comprehensive research report is ready for review
                  </p>
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                  <div className="p-4 bg-muted/20 rounded-lg border border-border text-center">
                    <Icon name="FileText" size={24} color="var(--color-primary)" className="mx-auto mb-2" />
                    <p className="text-sm font-medium text-foreground">Pages</p>
                    <p className="text-2xl font-semibold text-primary">{reportData.pages}</p>
                  </div>
                  <div className="p-4 bg-muted/20 rounded-lg border border-border text-center">
                    <Icon name="Globe" size={24} color="var(--color-accent)" className="mx-auto mb-2" />
                    <p className="text-sm font-medium text-foreground">Sources</p>
                    <p className="text-2xl font-semibold text-accent">{reportData.sources}</p>
                  </div>
                  <div className="p-4 bg-muted/20 rounded-lg border border-border text-center">
                    <Icon name="TrendingUp" size={24} color="var(--color-success)" className="mx-auto mb-2" />
                    <p className="text-sm font-medium text-foreground">Insights</p>
                    <p className="text-2xl font-semibold text-success">{reportData.insights}</p>
                  </div>
                </div>

                <div className="flex flex-col sm:flex-row gap-4 justify-center">
                  <Button
                    variant="default"
                    iconName="Eye"
                    onClick={onViewReport}
                    className="research-hover"
                  >
                    View Report
                  </Button>
                  <Button
                    variant="outline"
                    iconName="Download"
                    onClick={() => window.open('/api/download-report', '_blank')}
                    className="research-hover"
                  >
                    Download PDF
                  </Button>
                  <Button
                    variant="ghost"
                    iconName="Share2"
                    onClick={() => navigator.share && navigator.share({
                      title: 'Research Report',
                      text: 'Check out my research findings',
                      url: window.location.href
                    })}
                    className="research-hover"
                  >
                    Share
                  </Button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default FinalReportSection;