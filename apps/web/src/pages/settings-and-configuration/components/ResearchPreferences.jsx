import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';
import Select from '../../../components/ui/Select';
import { Checkbox } from '../../../components/ui/Checkbox';

const ResearchPreferences = () => {
  const [preferences, setPreferences] = useState({
    defaultResourceTypes: ['knowledge', 'web'],
    outputFormat: 'markdown',
    citationStyle: 'apa',
    autoSave: true,
    autoBackup: true,
    collaborativeMode: false,
    advancedAnalytics: true
  });

  const resourceTypeOptions = [
    { value: 'knowledge', label: 'Knowledge Base', description: 'Built-in research database' },
    { value: 'web', label: 'Web Pages', description: 'Online articles and resources' },
    { value: 'local', label: 'Local Files', description: 'Uploaded documents and files' },
    { value: 'academic', label: 'Academic Papers', description: 'Scholarly articles and journals' }
  ];

  const outputFormatOptions = [
    { value: 'markdown', label: 'Markdown (.md)', description: 'Structured text format' },
    { value: 'pdf', label: 'PDF Document', description: 'Portable document format' },
    { value: 'docx', label: 'Word Document', description: 'Microsoft Word format' },
    { value: 'html', label: 'HTML Page', description: 'Web page format' }
  ];

  const citationStyleOptions = [
    { value: 'apa', label: 'APA Style', description: 'American Psychological Association' },
    { value: 'mla', label: 'MLA Style', description: 'Modern Language Association' },
    { value: 'chicago', label: 'Chicago Style', description: 'Chicago Manual of Style' },
    { value: 'harvard', label: 'Harvard Style', description: 'Harvard referencing system' },
    { value: 'ieee', label: 'IEEE Style', description: 'Institute of Electrical and Electronics Engineers' }
  ];

  const workflowTemplates = [
    {
      id: 'academic',
      name: 'Academic Research',
      description: 'Comprehensive research with peer-reviewed sources and detailed citations',
      steps: ['Literature Review', 'Data Collection', 'Analysis', 'Report Generation'],
      estimatedTime: '2-4 hours'
    },
    {
      id: 'business',
      name: 'Business Intelligence',
      description: 'Market research and competitive analysis for business decisions',
      steps: ['Market Analysis', 'Competitor Research', 'Trend Analysis', 'Executive Summary'],
      estimatedTime: '1-2 hours'
    },
    {
      id: 'technical',
      name: 'Technical Documentation',
      description: 'In-depth technical research with code examples and implementation guides',
      steps: ['Technology Overview', 'Implementation Details', 'Best Practices', 'Documentation'],
      estimatedTime: '3-5 hours'
    }
  ];

  const handlePreferenceChange = (key, value) => {
    setPreferences(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleResourceTypeChange = (selectedTypes) => {
    setPreferences(prev => ({
      ...prev,
      defaultResourceTypes: selectedTypes
    }));
  };

  const handleSavePreferences = () => {
    console.log('Preferences saved:', preferences);
  };

  return (
    <div className="space-y-8">
      {/* Default Resource Types */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
            <Icon name="Database" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Default Resource Types
            </h3>
            <p className="text-sm text-muted-foreground">
              Choose which resource types to include by default in new research projects
            </p>
          </div>
        </div>

        <Select
          label="Preferred Resource Types"
          description="Select multiple resource types that will be automatically enabled for new research"
          options={resourceTypeOptions}
          value={preferences.defaultResourceTypes}
          onChange={handleResourceTypeChange}
          multiple
          searchable
          className="mb-4"
        />

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mt-6">
          {resourceTypeOptions.map((type) => (
            <div
              key={type.value}
              className={`
                p-4 rounded-lg border transition-all duration-200
                ${preferences.defaultResourceTypes.includes(type.value)
                  ? 'bg-primary/10 border-primary/20' :'bg-card border-border'
                }
              `}
            >
              <div className="flex items-center space-x-3 mb-2">
                <div className={`
                  w-8 h-8 rounded-lg flex items-center justify-center
                  ${preferences.defaultResourceTypes.includes(type.value) ? 'bg-primary' : 'bg-muted'}
                `}>
                  <Icon 
                    name={type.value === 'knowledge' ? 'Brain' : 
                          type.value === 'web' ? 'Globe' :
                          type.value === 'local' ? 'FolderOpen' : 'GraduationCap'} 
                    size={16} 
                    color={preferences.defaultResourceTypes.includes(type.value) ? 'white' : 'var(--color-muted-foreground)'} 
                  />
                </div>
                <h4 className="font-medium text-sm">{type.label}</h4>
              </div>
              <p className="text-xs text-muted-foreground">{type.description}</p>
            </div>
          ))}
        </div>
      </div>

      {/* Output Preferences */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-accent rounded-lg flex items-center justify-center">
            <Icon name="FileText" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Output Preferences
            </h3>
            <p className="text-sm text-muted-foreground">
              Configure how your research reports are formatted and cited
            </p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Select
            label="Default Output Format"
            description="Choose the default format for generated reports"
            options={outputFormatOptions}
            value={preferences.outputFormat}
            onChange={(value) => handlePreferenceChange('outputFormat', value)}
          />

          <Select
            label="Citation Style"
            description="Select your preferred citation format"
            options={citationStyleOptions}
            value={preferences.citationStyle}
            onChange={(value) => handlePreferenceChange('citationStyle', value)}
            searchable
          />
        </div>
      </div>

      {/* Workflow Automation */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-secondary rounded-lg flex items-center justify-center">
            <Icon name="Zap" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Workflow Automation
            </h3>
            <p className="text-sm text-muted-foreground">
              Enable automatic features to streamline your research process
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Auto-Save Progress</h4>
              <p className="text-sm text-muted-foreground">
                Automatically save your research progress every 30 seconds
              </p>
            </div>
            <Checkbox
              checked={preferences.autoSave}
              onChange={(e) => handlePreferenceChange('autoSave', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Auto-Backup</h4>
              <p className="text-sm text-muted-foreground">
                Create automatic backups of your research data daily
              </p>
            </div>
            <Checkbox
              checked={preferences.autoBackup}
              onChange={(e) => handlePreferenceChange('autoBackup', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Collaborative Mode</h4>
              <p className="text-sm text-muted-foreground">
                Enable real-time collaboration features for team research
              </p>
            </div>
            <Checkbox
              checked={preferences.collaborativeMode}
              onChange={(e) => handlePreferenceChange('collaborativeMode', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Advanced Analytics</h4>
              <p className="text-sm text-muted-foreground">
                Track detailed analytics and insights about your research patterns
              </p>
            </div>
            <Checkbox
              checked={preferences.advancedAnalytics}
              onChange={(e) => handlePreferenceChange('advancedAnalytics', e.target.checked)}
            />
          </div>
        </div>
      </div>

      {/* Workflow Templates */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-warning rounded-lg flex items-center justify-center">
            <Icon name="Layout" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Workflow Templates
            </h3>
            <p className="text-sm text-muted-foreground">
              Pre-configured research workflows for different use cases
            </p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {workflowTemplates.map((template) => (
            <div
              key={template.id}
              className="research-card p-4 hover:bg-muted/50 transition-all duration-200 cursor-pointer"
            >
              <div className="flex items-center justify-between mb-3">
                <h4 className="font-medium text-foreground">{template.name}</h4>
                <Icon name="Clock" size={16} color="var(--color-muted-foreground)" />
              </div>
              
              <p className="text-sm text-muted-foreground mb-4 leading-relaxed">
                {template.description}
              </p>
              
              <div className="space-y-2 mb-4">
                <div className="text-xs text-muted-foreground">Workflow Steps:</div>
                <div className="flex flex-wrap gap-1">
                  {template.steps.map((step, index) => (
                    <span
                      key={index}
                      className="px-2 py-1 bg-primary/10 text-primary text-xs rounded-full"
                    >
                      {step}
                    </span>
                  ))}
                </div>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-xs text-muted-foreground">
                  Est. {template.estimatedTime}
                </span>
                <Button variant="ghost" size="sm" iconName="ArrowRight">
                  Use Template
                </Button>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Save Changes */}
      <div className="flex justify-end">
        <Button
          variant="default"
          iconName="Save"
          onClick={handleSavePreferences}
        >
          Save Preferences
        </Button>
      </div>
    </div>
  );
};

export default ResearchPreferences;