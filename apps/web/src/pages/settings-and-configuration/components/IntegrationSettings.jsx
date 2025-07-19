import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';
import Input from '../../../components/ui/Input';
import { Checkbox } from '../../../components/ui/Checkbox';
import AIProviderSettings from './AIProviderSettings';

const IntegrationSettings = () => {
  const [showAISettings, setShowAISettings] = useState(false);
  const [integrations, setIntegrations] = useState({
    openai: {
      enabled: true,
      apiKey: "sk-proj-***************",
      model: "gpt-4",
      status: "connected"
    },
    googleScholar: {
      enabled: true,
      apiKey: "",
      status: "connected"
    },
    mendeley: {
      enabled: false,
      apiKey: "",
      status: "disconnected"
    },
    zotero: {
      enabled: false,
      apiKey: "",
      status: "disconnected"
    }
  });

  const [exportSettings, setExportSettings] = useState({
    autoExportEnabled: true,
    exportFormat: 'json',
    exportFrequency: 'weekly',
    includeMetadata: true,
    compressFiles: false
  });

  const availableIntegrations = [
    {
      id: 'openai',
      name: 'OpenAI GPT',
      description: 'AI-powered research assistance and content generation',
      icon: 'Brain',
      color: 'bg-primary',
      features: ['Content Generation', 'Research Assistance', 'Summarization'],
      setupRequired: true
    },
    {
      id: 'googleScholar',
      name: 'Google Scholar',
      description: 'Access to academic papers and scholarly articles',
      icon: 'GraduationCap',
      color: 'bg-accent',
      features: ['Academic Search', 'Citation Tracking', 'Paper Discovery'],
      setupRequired: false
    },
    {
      id: 'mendeley',
      name: 'Mendeley',
      description: 'Reference management and academic collaboration',
      icon: 'BookOpen',
      color: 'bg-secondary',
      features: ['Reference Management', 'PDF Annotation', 'Collaboration'],
      setupRequired: true
    },
    {
      id: 'zotero',
      name: 'Zotero',
      description: 'Research collection and organization tool',
      icon: 'Archive',
      color: 'bg-warning',
      features: ['Research Collection', 'Citation Management', 'Group Libraries'],
      setupRequired: true
    }
  ];

  const handleIntegrationToggle = (integrationId) => {
    setIntegrations(prev => ({
      ...prev,
      [integrationId]: {
        ...prev[integrationId],
        enabled: !prev[integrationId].enabled
      }
    }));
  };

  const handleApiKeyUpdate = (integrationId, apiKey) => {
    setIntegrations(prev => ({
      ...prev,
      [integrationId]: {
        ...prev[integrationId],
        apiKey: apiKey
      }
    }));
  };

  const handleTestConnection = (integrationId) => {
    console.log(`Testing connection for ${integrationId}`);
    // Mock connection test
    setIntegrations(prev => ({
      ...prev,
      [integrationId]: {
        ...prev[integrationId],
        status: 'connected'
      }
    }));
  };

  const handleExportSettingChange = (key, value) => {
    setExportSettings(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleAISettingsSave = (settings) => {
    console.log('AI Settings saved:', settings);
    // Here you would typically save to localStorage or send to backend
  };

  return (
    <div className="space-y-8">
      {/* AI Provider Settings */}
      <div className="research-card p-6">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center space-x-3">
            <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
              <Icon name="Settings" size={20} color="white" />
            </div>
            <div>
              <h3 className="text-lg font-semibold text-foreground">
                AI Provider Settings
              </h3>
              <p className="text-sm text-muted-foreground">
                Configure AI providers and models for research assistance
              </p>
            </div>
          </div>
          <Button
            variant="default"
            onClick={() => setShowAISettings(true)}
            iconName="Settings"
          >
            Configure AI Settings
          </Button>
        </div>
      </div>

      {/* Available Integrations */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
            <Icon name="Plug" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              External Integrations
            </h3>
            <p className="text-sm text-muted-foreground">
              Connect with external services to enhance your research capabilities
            </p>
          </div>
        </div>

        <div className="space-y-6">
          {availableIntegrations.map((integration) => {
            const integrationData = integrations[integration.id];
            const isConnected = integrationData?.status === 'connected';
            
            return (
              <div
                key={integration.id}
                className="border border-border rounded-lg p-6 transition-all duration-200"
              >
                <div className="flex items-start justify-between mb-4">
                  <div className="flex items-start space-x-4">
                    <div className={`w-12 h-12 ${integration.color} rounded-lg flex items-center justify-center`}>
                      <Icon name={integration.icon} size={24} color="white" />
                    </div>
                    <div className="flex-1">
                      <div className="flex items-center space-x-3 mb-2">
                        <h4 className="font-semibold text-foreground">{integration.name}</h4>
                        <span className={`
                          px-2 py-1 text-xs rounded-full
                          ${isConnected 
                            ? 'bg-success/10 text-success' :'bg-muted/20 text-muted-foreground'
                          }
                        `}>
                          {isConnected ? 'Connected' : 'Disconnected'}
                        </span>
                      </div>
                      <p className="text-sm text-muted-foreground mb-3">
                        {integration.description}
                      </p>
                      <div className="flex flex-wrap gap-2">
                        {integration.features.map((feature, index) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-muted/20 text-muted-foreground text-xs rounded-full"
                          >
                            {feature}
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>
                  
                  <div className="flex items-center space-x-2">
                    <Checkbox
                      checked={integrationData?.enabled || false}
                      onChange={() => handleIntegrationToggle(integration.id)}
                    />
                  </div>
                </div>

                {integrationData?.enabled && integration.setupRequired && (
                  <div className="mt-4 p-4 bg-muted/20 rounded-lg">
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4 items-end">
                      <div className="md:col-span-2">
                        <Input
                          label="API Key"
                          type="password"
                          value={integrationData.apiKey}
                          onChange={(e) => handleApiKeyUpdate(integration.id, e.target.value)}
                          placeholder="Enter your API key"
                          description="Your API key will be encrypted and stored securely"
                        />
                      </div>
                      <Button
                        variant="outline"
                        iconName="Zap"
                        onClick={() => handleTestConnection(integration.id)}
                        disabled={!integrationData.apiKey}
                      >
                        Test Connection
                      </Button>
                    </div>
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>

      {/* Data Export Settings */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-accent rounded-lg flex items-center justify-center">
            <Icon name="Download" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Data Export Settings
            </h3>
            <p className="text-sm text-muted-foreground">
              Configure how your research data is exported and shared
            </p>
          </div>
        </div>

        <div className="space-y-6">
          {/* Auto Export Settings */}
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Automatic Export</h4>
              <p className="text-sm text-muted-foreground">
                Automatically export research data at regular intervals
              </p>
            </div>
            <Checkbox
              checked={exportSettings.autoExportEnabled}
              onChange={(e) => handleExportSettingChange('autoExportEnabled', e.target.checked)}
            />
          </div>

          {exportSettings.autoExportEnabled && (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label className="block text-sm font-medium text-foreground mb-2">
                  Export Format
                </label>
                <select
                  className="w-full p-3 bg-input border border-border rounded-lg text-foreground"
                  value={exportSettings.exportFormat}
                  onChange={(e) => handleExportSettingChange('exportFormat', e.target.value)}
                >
                  <option value="json">JSON Format</option>
                  <option value="csv">CSV Format</option>
                  <option value="xml">XML Format</option>
                  <option value="pdf">PDF Report</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-foreground mb-2">
                  Export Frequency
                </label>
                <select
                  className="w-full p-3 bg-input border border-border rounded-lg text-foreground"
                  value={exportSettings.exportFrequency}
                  onChange={(e) => handleExportSettingChange('exportFrequency', e.target.value)}
                >
                  <option value="daily">Daily</option>
                  <option value="weekly">Weekly</option>
                  <option value="monthly">Monthly</option>
                  <option value="custom">Custom Schedule</option>
                </select>
              </div>
            </div>
          )}

          {/* Export Options */}
          <div className="space-y-4">
            <h4 className="font-medium text-foreground">Export Options</h4>
            
            <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
              <div>
                <h5 className="font-medium text-foreground">Include Metadata</h5>
                <p className="text-sm text-muted-foreground">
                  Include research metadata and timestamps in exports
                </p>
              </div>
              <Checkbox
                checked={exportSettings.includeMetadata}
                onChange={(e) => handleExportSettingChange('includeMetadata', e.target.checked)}
              />
            </div>

            <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
              <div>
                <h5 className="font-medium text-foreground">Compress Files</h5>
                <p className="text-sm text-muted-foreground">
                  Compress exported files to reduce file size
                </p>
              </div>
              <Checkbox
                checked={exportSettings.compressFiles}
                onChange={(e) => handleExportSettingChange('compressFiles', e.target.checked)}
              />
            </div>
          </div>
        </div>
      </div>

      {/* API Configuration */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-secondary rounded-lg flex items-center justify-center">
            <Icon name="Code" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              API Configuration
            </h3>
            <p className="text-sm text-muted-foreground">
              Configure API endpoints and authentication for custom integrations
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <Input
            label="Custom API Endpoint"
            type="url"
            placeholder="https://api.example.com/v1"
            description="Enter your custom API endpoint URL"
          />
          
          <Input
            label="API Authentication Token"
            type="password"
            placeholder="Enter your authentication token"
            description="Token will be used for API authentication"
          />
          
          <div className="flex space-x-4">
            <Button variant="outline" iconName="TestTube">
              Test API Connection
            </Button>
            <Button variant="ghost" iconName="BookOpen">
              View API Documentation
            </Button>
          </div>
        </div>
      </div>

      {/* Save Changes */}
      <div className="flex justify-end">
        <Button variant="default" iconName="Save">
          Save Integration Settings
        </Button>
      </div>

      {/* AI Provider Settings Modal */}
      <AIProviderSettings
        isOpen={showAISettings}
        onClose={() => setShowAISettings(false)}
        onSave={handleAISettingsSave}
      />
    </div>
  );
};

export default IntegrationSettings;