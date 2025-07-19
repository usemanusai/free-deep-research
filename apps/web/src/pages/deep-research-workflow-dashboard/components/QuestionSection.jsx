import React from 'react';
import Icon from '../../../components/AppIcon';

const QuestionSection = ({ 
  isExpanded, 
  onToggle, 
  status,
  questions 
}) => {
  const getStatusIcon = () => {
    switch (status) {
      case 'waiting':
        return 'Clock';
      case 'processing':
        return 'Loader';
      case 'completed':
        return 'CheckCircle2';
      default:
        return 'HelpCircle';
    }
  };

  const getStatusColor = () => {
    switch (status) {
      case 'waiting':
        return 'var(--color-warning)';
      case 'processing':
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
        return 'Waiting for topics to be defined';
      case 'processing':
        return 'Generating research questions...';
      case 'completed':
        return `${questions.length} questions generated`;
      default:
        return 'Ready to generate questions';
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
            2
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">Ask Question</h3>
            <p className="text-sm text-muted-foreground">
              AI-generated research questions based on your topics
            </p>
          </div>
        </div>
        <div className="flex items-center space-x-3">
          <div className="flex items-center space-x-2">
            <Icon 
              name={getStatusIcon()} 
              size={16} 
              color={getStatusColor()}
              className={status === 'processing' ? 'animate-spin' : ''}
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
                  Waiting for Research Topics
                </h4>
                <p className="text-muted-foreground">
                  Please define your research topics in the section above to continue
                </p>
              </div>
            )}

            {status === 'processing' && (
              <div className="text-center py-8">
                <Icon name="Loader" size={48} color="var(--color-primary)" className="mx-auto mb-4 animate-spin" />
                <h4 className="text-lg font-medium text-foreground mb-2">
                  Generating Questions
                </h4>
                <p className="text-muted-foreground">
                  AI is analyzing your topics and creating relevant research questions...
                </p>
              </div>
            )}

            {status === 'completed' && questions.length > 0 && (
              <div className="space-y-3">
                <h4 className="font-medium text-foreground mb-4">Generated Research Questions:</h4>
                {questions.map((question, index) => (
                  <div key={index} className="p-4 bg-muted/20 rounded-lg border border-border">
                    <div className="flex items-start space-x-3">
                      <div className="flex items-center justify-center w-6 h-6 bg-primary rounded-full text-white text-sm font-medium mt-0.5">
                        {index + 1}
                      </div>
                      <p className="text-foreground flex-1">{question}</p>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default QuestionSection;