import React, { useState, useEffect } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const ReportGenerationProgress = ({ isVisible, onComplete }) => {
  const [currentStep, setCurrentStep] = useState(0);
  const [progress, setProgress] = useState(0);
  const [isGenerating, setIsGenerating] = useState(false);

  const generationSteps = [
    {
      id: 'analyzing',
      title: 'Analyzing Research Data',
      description: 'Processing collected information and identifying key patterns',
      icon: 'Search',
      duration: 3000
    },
    {
      id: 'structuring',
      title: 'Structuring Content',
      description: 'Organizing findings into logical sections and subsections',
      icon: 'FileText',
      duration: 2500
    },
    {
      id: 'generating',
      title: 'Generating Insights',
      description: 'Creating analysis and drawing conclusions from data',
      icon: 'Lightbulb',
      duration: 4000
    },
    {
      id: 'formatting',
      title: 'Formatting Document',
      description: 'Applying professional formatting and styling',
      icon: 'Layout',
      duration: 2000
    },
    {
      id: 'finalizing',
      title: 'Finalizing Report',
      description: 'Adding citations, references, and final touches',
      icon: 'CheckCircle2',
      duration: 1500
    }
  ];

  useEffect(() => {
    if (isVisible && !isGenerating) {
      startGeneration();
    }
  }, [isVisible]);

  const startGeneration = async () => {
    setIsGenerating(true);
    setCurrentStep(0);
    setProgress(0);

    for (let i = 0; i < generationSteps.length; i++) {
      setCurrentStep(i);
      
      // Simulate step progress
      const stepDuration = generationSteps[i].duration;
      const progressIncrement = 100 / generationSteps.length;
      const startProgress = i * progressIncrement;
      
      for (let j = 0; j <= 100; j += 2) {
        await new Promise(resolve => setTimeout(resolve, stepDuration / 50));
        setProgress(startProgress + (progressIncrement * j / 100));
      }
    }

    setProgress(100);
    setTimeout(() => {
      setIsGenerating(false);
      onComplete();
    }, 500);
  };

  if (!isVisible) return null;

  return (
    <div className="fixed inset-0 bg-background/80 backdrop-blur-sm z-50 flex items-center justify-center">
      <div className="bg-card border border-border rounded-lg p-8 max-w-md w-full mx-4 research-card">
        {/* Header */}
        <div className="text-center mb-8">
          <div className="w-16 h-16 bg-primary/10 rounded-full flex items-center justify-center mx-auto mb-4">
            <Icon name="FileText" size={32} color="var(--color-primary)" />
          </div>
          <h2 className="text-2xl font-semibold text-foreground mb-2">
            Generating Final Report
          </h2>
          <p className="text-muted-foreground">
            Please wait while we compile your research into a comprehensive report
          </p>
        </div>

        {/* Progress Bar */}
        <div className="mb-8">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm font-medium text-foreground">Overall Progress</span>
            <span className="text-sm text-muted-foreground">{Math.round(progress)}%</span>
          </div>
          <div className="w-full bg-muted/20 rounded-full h-2">
            <div 
              className="bg-primary h-2 rounded-full progress-indicator transition-all duration-300"
              style={{ width: `${progress}%` }}
            />
          </div>
        </div>

        {/* Current Step */}
        <div className="space-y-4 mb-8">
          {generationSteps.map((step, index) => {
            const isActive = index === currentStep;
            const isCompleted = index < currentStep;
            const isPending = index > currentStep;

            return (
              <div
                key={step.id}
                className={`
                  flex items-center space-x-4 p-3 rounded-lg transition-all
                  ${isActive ? 'bg-primary/10 border border-primary/20' : 
                    isCompleted ? 'bg-success/10' : 'bg-muted/20'}
                `}
              >
                <div className={`
                  w-10 h-10 rounded-full flex items-center justify-center
                  ${isActive ? 'bg-primary' : 
                    isCompleted ? 'bg-success' : 'bg-muted'}
                `}>
                  <Icon 
                    name={isCompleted ? 'Check' : step.icon} 
                    size={20} 
                    color="white" 
                  />
                </div>
                
                <div className="flex-1">
                  <div className={`
                    font-medium text-sm
                    ${isActive ? 'text-primary' : 
                      isCompleted ? 'text-success' : 'text-muted-foreground'}
                  `}>
                    {step.title}
                  </div>
                  <div className="text-xs text-muted-foreground">
                    {step.description}
                  </div>
                </div>

                {isActive && (
                  <div className="flex items-center space-x-2">
                    <div className="w-2 h-2 bg-primary rounded-full animate-pulse" />
                    <span className="text-xs text-primary font-medium">Processing...</span>
                  </div>
                )}

                {isCompleted && (
                  <Icon name="CheckCircle2" size={16} color="var(--color-success)" />
                )}
              </div>
            );
          })}
        </div>

        {/* Estimated Time */}
        <div className="text-center">
          <div className="text-sm text-muted-foreground mb-2">
            Estimated time remaining
          </div>
          <div className="text-lg font-semibold text-foreground">
            {isGenerating ? 
              `${Math.max(0, Math.ceil((100 - progress) / 10))} seconds` : 
              'Complete!'
            }
          </div>
        </div>

        {/* Cancel Button */}
        {isGenerating && (
          <div className="mt-6 text-center">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => {
                setIsGenerating(false);
                onComplete();
              }}
              className="research-hover"
            >
              Cancel Generation
            </Button>
          </div>
        )}
      </div>
    </div>
  );
};

export default ReportGenerationProgress;