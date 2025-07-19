import React, { useState } from 'react';
import Button from '../../../components/ui/Button';
import Input from '../../../components/ui/Input';
import Select from '../../../components/ui/Select';
import Icon from '../../../components/AppIcon';

const AIProviderSettings = ({ isOpen, onClose, onSave }) => {
  const [activeTab, setActiveTab] = useState('model');
  const [settings, setSettings] = useState({
    apiMode: 'Local',
    aiProvider: 'Google AI Studio',
    apiKey: '',
    apiBaseUrl: '',
    thinkingModel: '',
    taskModel: '',
    customThinkingModel: '',
    customTaskModel: '',
    useCustomThinkingModel: false,
    useCustomTaskModel: false,
    // Search settings
    webSearchEnabled: 'Enable',
    searchProvider: 'Model built-in',
    parallelSearch: 1,
    searchResults: 5
  });

  const [errors, setErrors] = useState({});

  const apiModeOptions = [
    { value: 'Local', label: 'Local' },
    { value: 'Cloud', label: 'Cloud' },
    { value: 'Hybrid', label: 'Hybrid' }
  ];

  const aiProviderOptions = [
    { value: 'Google AI Studio', label: 'Google AI Studio' },
    { value: 'OpenAI', label: 'OpenAI' },
    { value: 'Anthropic', label: 'Anthropic' },
    { value: 'DeepSeek', label: 'DeepSeek' },
    { value: 'xAI Grok', label: 'xAI Grok' },
    { value: 'Mistral', label: 'Mistral' },
    { value: 'Azure OpenAI', label: 'Azure OpenAI' },
    { value: 'OpenRouter', label: 'OpenRouter' },
    { value: 'OpenAI Compatible', label: 'OpenAI Compatible' },
    { value: 'Pollinations (Free)', label: 'Pollinations (Free)' },
    { value: 'Ollama', label: 'Ollama' }
  ];

  const webSearchOptions = [
    { value: 'Enable', label: 'Enable' },
    { value: 'Disable', label: 'Disable' }
  ];

  const searchProviderOptions = [
    { value: 'Model built-in', label: 'Model built-in' },
    { value: 'Google Search', label: 'Google Search' },
    { value: 'Bing Search', label: 'Bing Search' },
    { value: 'DuckDuckGo', label: 'DuckDuckGo' }
  ];

  const getModelOptions = (provider) => {
    const modelsByProvider = {
      'Google AI Studio': [
        { value: 'gemini-pro', label: 'Gemini Pro' },
        { value: 'gemini-pro-vision', label: 'Gemini Pro Vision' },
        { value: 'gemini-ultra', label: 'Gemini Ultra' }
      ],
      'OpenAI': [
        { value: 'gpt-4', label: 'GPT-4' },
        { value: 'gpt-4-turbo', label: 'GPT-4 Turbo' },
        { value: 'gpt-3.5-turbo', label: 'GPT-3.5 Turbo' },
        { value: 'gpt-4o', label: 'GPT-4o' }
      ],
      'Anthropic': [
        { value: 'claude-3-opus', label: 'Claude 3 Opus' },
        { value: 'claude-3-sonnet', label: 'Claude 3 Sonnet' },
        { value: 'claude-3-haiku', label: 'Claude 3 Haiku' }
      ],
      'DeepSeek': [
        { value: 'deepseek-chat', label: 'DeepSeek Chat' },
        { value: 'deepseek-coder', label: 'DeepSeek Coder' }
      ],
      'xAI Grok': [
        { value: 'grok-1', label: 'Grok-1' },
        { value: 'grok-1.5', label: 'Grok-1.5' }
      ],
      'Mistral': [
        { value: 'mistral-large', label: 'Mistral Large' },
        { value: 'mistral-medium', label: 'Mistral Medium' },
        { value: 'mistral-small', label: 'Mistral Small' }
      ],
      'Azure OpenAI': [
        { value: 'gpt-4', label: 'GPT-4' },
        { value: 'gpt-35-turbo', label: 'GPT-3.5 Turbo' }
      ],
      'OpenRouter': [
        { value: 'openrouter/auto', label: 'Auto (Best)' },
        { value: 'anthropic/claude-3-opus', label: 'Claude 3 Opus' },
        { value: 'openai/gpt-4', label: 'GPT-4' },
        { value: 'custom', label: 'Custom Model' }
      ],
      'OpenAI Compatible': [
        { value: 'custom-model', label: 'Custom Model' }
      ],
      'Pollinations (Free)': [
        { value: 'poll-gpt', label: 'Poll GPT' }
      ],
      'Ollama': [
        { value: 'llama2', label: 'Llama 2' },
        { value: 'codellama', label: 'Code Llama' },
        { value: 'mistral', label: 'Mistral' }
      ]
    };

    return modelsByProvider[provider] || [];
  };

  const validateForm = () => {
    const newErrors = {};

    if (activeTab === 'model') {
      if (!settings.apiKey.trim()) {
        newErrors.apiKey = 'API Key is required';
      }

      if (!settings.useCustomThinkingModel && !settings.thinkingModel.trim()) {
        newErrors.thinkingModel = 'Thinking Model is required';
      }

      if (settings.useCustomThinkingModel && !settings.customThinkingModel.trim()) {
        newErrors.customThinkingModel = 'Custom Thinking Model is required';
      }

      if (!settings.useCustomTaskModel && !settings.taskModel.trim()) {
        newErrors.taskModel = 'Task Model is required';
      }

      if (settings.useCustomTaskModel && !settings.customTaskModel.trim()) {
        newErrors.customTaskModel = 'Custom Task Model is required';
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleInputChange = (field, value) => {
    setSettings(prev => ({
      ...prev,
      [field]: value
    }));

    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({
        ...prev,
        [field]: ''
      }));
    }

    // Reset models when provider changes
    if (field === 'aiProvider') {
      setSettings(prev => ({
        ...prev,
        thinkingModel: '',
        taskModel: '',
        customThinkingModel: '',
        customTaskModel: '',
        useCustomThinkingModel: false,
        useCustomTaskModel: false
      }));
    }
  };

  const handleCustomModelToggle = (modelType) => {
    const field = `useCustom${modelType}Model`;
    setSettings(prev => ({
      ...prev,
      [field]: !prev[field],
      [modelType.toLowerCase() + 'Model']: '',
      [`custom${modelType}Model`]: ''
    }));
  };

  const handleSave = () => {
    if (validateForm()) {
      onSave?.(settings);
      onClose?.();
    }
  };

  const handleCancel = () => {
    setSettings({
      apiMode: 'Local',
      aiProvider: 'Google AI Studio',
      apiKey: '',
      apiBaseUrl: '',
      thinkingModel: '',
      taskModel: '',
      customThinkingModel: '',
      customTaskModel: '',
      useCustomThinkingModel: false,
      useCustomTaskModel: false,
      webSearchEnabled: 'Enable',
      searchProvider: 'Model built-in',
      parallelSearch: 1,
      searchResults: 5
    });
    setErrors({});
    onClose?.();
  };

  if (!isOpen) return null;

  const modelOptions = getModelOptions(settings.aiProvider);

  const SliderInput = ({ label, value, onChange, min = 1, max = 10, step = 1 }) => (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <label className="text-sm font-medium text-foreground">{label}</label>
        <span className="text-sm text-muted-foreground">{value}</span>
      </div>
      <div className="flex items-center space-x-3">
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={value}
          onChange={(e) => onChange(parseInt(e.target.value))}
          className="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
        />
      </div>
    </div>
  );

  const CustomModelInput = ({ label, value, onChange, options, useCustom, onToggleCustom, customValue, onCustomChange, error, customError }) => (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <label className="text-sm font-medium text-foreground">
          {label}
          <span className="text-destructive ml-1">*</span>
        </label>
        <button
          type="button"
          onClick={onToggleCustom}
          className="text-xs text-primary hover:text-primary/80 font-medium"
        >
          {useCustom ? 'Use predefined' : 'Use custom'}
        </button>
      </div>
      
      {useCustom ? (
        <Input
          type="text"
          value={customValue}
          onChange={(e) => onCustomChange(e.target.value)}
          placeholder="e.g., deepseek/deepseek-r1-0528:free"
          error={customError}
          className="w-full"
        />
      ) : (
        <Select
          value={value}
          onChange={onChange}
          options={options}
          placeholder={`Select ${label.toLowerCase()}`}
          error={error}
          className="w-full"
        />
      )}
    </div>
  );

  const renderTabContent = () => {
    switch (activeTab) {
      case 'model':
        return (
          <div className="space-y-6">
            {/* API Mode */}
            <div>
              <Select
                label="API Mode"
                value={settings.apiMode}
                onChange={(value) => handleInputChange('apiMode', value)}
                options={apiModeOptions}
                className="w-full"
              />
            </div>

            {/* AI Provider */}
            <div>
              <Select
                label="AI Provider"
                value={settings.aiProvider}
                onChange={(value) => handleInputChange('aiProvider', value)}
                options={aiProviderOptions}
                className="w-full"
              />
            </div>

            {/* API Key */}
            <div>
              <Input
                label="Api Key"
                type="password"
                value={settings.apiKey}
                onChange={(e) => handleInputChange('apiKey', e.target.value)}
                placeholder="Enter your API key"
                required
                error={errors.apiKey}
                className="w-full"
              />
            </div>

            {/* API Base URL */}
            <div>
              <Input
                label="Api Base Url"
                type="url"
                value={settings.apiBaseUrl}
                onChange={(e) => handleInputChange('apiBaseUrl', e.target.value)}
                placeholder="Enter API base URL (optional)"
                className="w-full"
              />
            </div>

            {/* Thinking Model */}
            <div>
              <CustomModelInput
                label="Thinking Model"
                value={settings.thinkingModel}
                onChange={(value) => handleInputChange('thinkingModel', value)}
                options={modelOptions}
                useCustom={settings.useCustomThinkingModel}
                onToggleCustom={() => handleCustomModelToggle('Thinking')}
                customValue={settings.customThinkingModel}
                onCustomChange={(value) => handleInputChange('customThinkingModel', value)}
                error={errors.thinkingModel}
                customError={errors.customThinkingModel}
              />
            </div>

            {/* Task Model */}
            <div>
              <CustomModelInput
                label="Task Model"
                value={settings.taskModel}
                onChange={(value) => handleInputChange('taskModel', value)}
                options={modelOptions}
                useCustom={settings.useCustomTaskModel}
                onToggleCustom={() => handleCustomModelToggle('Task')}
                customValue={settings.customTaskModel}
                onCustomChange={(value) => handleInputChange('customTaskModel', value)}
                error={errors.taskModel}
                customError={errors.customTaskModel}
              />
            </div>
          </div>
        );

      case 'search':
        return (
          <div className="space-y-6">
            {/* Web Search */}
            <div>
              <Select
                label="Web Search"
                value={settings.webSearchEnabled}
                onChange={(value) => handleInputChange('webSearchEnabled', value)}
                options={webSearchOptions}
                className="w-full"
              />
            </div>

            {/* Search Provider */}
            <div>
              <Select
                label="Search Provider"
                value={settings.searchProvider}
                onChange={(value) => handleInputChange('searchProvider', value)}
                options={searchProviderOptions}
                className="w-full"
              />
            </div>

            {/* Parallel Search */}
            <div>
              <SliderInput
                label="Parallel Search"
                value={settings.parallelSearch}
                onChange={(value) => handleInputChange('parallelSearch', value)}
                min={1}
                max={5}
                step={1}
              />
            </div>

            {/* Search Results */}
            <div>
              <SliderInput
                label="Search Results"
                value={settings.searchResults}
                onChange={(value) => handleInputChange('searchResults', value)}
                min={1}
                max={20}
                step={1}
              />
            </div>
          </div>
        );

      case 'system':
        return (
          <div className="space-y-6">
            <div className="text-center text-muted-foreground py-8">
              <Icon name="Settings" size={48} className="mx-auto mb-4 opacity-50" />
              <p>System settings coming soon...</p>
            </div>
          </div>
        );

      case 'experimental':
        return (
          <div className="space-y-6">
            <div className="text-center text-muted-foreground py-8">
              <Icon name="Flask" size={48} className="mx-auto mb-4 opacity-50" />
              <p>Experimental features coming soon...</p>
            </div>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="research-modal w-full max-w-2xl max-h-[90vh] overflow-y-auto m-4 animate-fade-in">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-border">
          <div>
            <h2 className="text-xl font-semibold text-foreground">Setting</h2>
            <p className="text-sm text-muted-foreground mt-1">
              All settings are saved in the user's browser
            </p>
          </div>
          <Button
            variant="ghost"
            size="icon"
            onClick={onClose}
            className="h-8 w-8"
          >
            <Icon name="X" size={16} />
          </Button>
        </div>

        {/* Tab Navigation */}
        <div className="flex border-b border-border px-6">
          <button
            onClick={() => setActiveTab('model')}
            className={`px-4 py-3 text-sm font-medium ${
              activeTab === 'model' ?'text-primary border-b-2 border-primary' :'text-muted-foreground hover:text-foreground'
            }`}
          >
            Model
          </button>
          <button
            onClick={() => setActiveTab('search')}
            className={`px-4 py-3 text-sm font-medium ${
              activeTab === 'search' ?'text-primary border-b-2 border-primary' :'text-muted-foreground hover:text-foreground'
            }`}
          >
            Search
          </button>
          <button
            onClick={() => setActiveTab('system')}
            className={`px-4 py-3 text-sm font-medium ${
              activeTab === 'system' ?'text-primary border-b-2 border-primary' :'text-muted-foreground hover:text-foreground'
            }`}
          >
            System
          </button>
          <button
            onClick={() => setActiveTab('experimental')}
            className={`px-4 py-3 text-sm font-medium ${
              activeTab === 'experimental' ?'text-primary border-b-2 border-primary' :'text-muted-foreground hover:text-foreground'
            }`}
          >
            Experimental
          </button>
        </div>

        {/* Form Content */}
        <div className="p-6">
          {renderTabContent()}
        </div>

        {/* Footer */}
        <div className="flex justify-end space-x-3 p-6 border-t border-border">
          <Button
            variant="outline"
            onClick={handleCancel}
          >
            Cancel
          </Button>
          <Button
            variant="default"
            onClick={handleSave}
          >
            Save
          </Button>
        </div>
      </div>
    </div>
  );
};

export default AIProviderSettings;