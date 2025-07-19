import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';
import Input from '../../../components/ui/Input';

const ResearchTopicsSection = ({ 
  isExpanded, 
  onToggle, 
  researchTopics, 
  onTopicsChange, 
  onAddResource 
}) => {
  const [showResourceOptions, setShowResourceOptions] = useState(false);

  const handleResourceClick = (type) => {
    onAddResource(type);
    setShowResourceOptions(false);
  };

  return (
    <div className="research-card">
      <div 
        className="flex items-center justify-between p-6 cursor-pointer research-hover"
        onClick={onToggle}
      >
        <div className="flex items-center space-x-4">
          <div className="flex items-center justify-center w-8 h-8 bg-primary rounded-full text-white font-semibold">
            1
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">Research Topics</h3>
            <p className="text-sm text-muted-foreground">
              Define your research focus and add relevant resources
            </p>
          </div>
        </div>
        <Icon 
          name={isExpanded ? "ChevronUp" : "ChevronDown"} 
          size={20} 
          color="var(--color-muted-foreground)" 
        />
      </div>

      {isExpanded && (
        <div className="px-6 pb-6 space-y-6 animate-fade-in">
          <div className="border-t border-border pt-6">
            <Input
              label="Research Topics"
              type="text"
              placeholder="Enter your research topics, questions, or areas of interest..."
              value={researchTopics}
              onChange={(e) => onTopicsChange(e.target.value)}
              description="Describe what you want to research. Be specific to get better results."
              className="mb-4"
            />

            <div className="flex flex-col sm:flex-row gap-4 items-start">
              <Button
                variant="outline"
                iconName="Plus"
                onClick={() => setShowResourceOptions(!showResourceOptions)}
                className="research-hover"
              >
                Add Resource
              </Button>

              {showResourceOptions && (
                <div className="flex flex-wrap gap-2">
                  <Button
                    variant="secondary"
                    size="sm"
                    iconName="Brain"
                    onClick={() => handleResourceClick('knowledge')}
                    className="research-hover"
                  >
                    Knowledge
                  </Button>
                  <Button
                    variant="secondary"
                    size="sm"
                    iconName="Upload"
                    onClick={() => handleResourceClick('file')}
                    className="research-hover"
                  >
                    Local File
                  </Button>
                  <Button
                    variant="secondary"
                    size="sm"
                    iconName="Globe"
                    onClick={() => handleResourceClick('webpage')}
                    className="research-hover"
                  >
                    Web Page
                  </Button>
                </div>
              )}
            </div>

            {researchTopics && (
              <div className="mt-4 p-4 bg-muted/20 rounded-lg border border-border">
                <div className="flex items-center space-x-2 mb-2">
                  <Icon name="CheckCircle2" size={16} color="var(--color-success)" />
                  <span className="text-sm font-medium text-success">Topics Defined</span>
                </div>
                <p className="text-sm text-muted-foreground">
                  Ready to proceed to question formulation
                </p>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default ResearchTopicsSection;