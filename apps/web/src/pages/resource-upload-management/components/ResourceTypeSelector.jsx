import React from 'react';
import Icon from '../../../components/AppIcon';

const ResourceTypeSelector = ({ selectedType, onTypeChange }) => {
  const resourceTypes = [
    {
      id: 'knowledge',
      name: 'Knowledge',
      description: 'Research papers, articles, and academic documents',
      icon: 'BookOpen',
      color: 'text-blue-400'
    },
    {
      id: 'files',
      name: 'Local Files',
      description: 'Documents, images, spreadsheets, and presentations',
      icon: 'FileText',
      color: 'text-green-400'
    },
    {
      id: 'webpages',
      name: 'Web Pages',
      description: 'URLs, web articles, and online resources',
      icon: 'Globe',
      color: 'text-purple-400'
    }
  ];

  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold text-foreground mb-4">
        Resource Type
      </h3>
      
      <div className="space-y-3">
        {resourceTypes.map((type) => (
          <button
            key={type.id}
            onClick={() => onTypeChange(type.id)}
            className={`w-full p-4 rounded-lg border transition-all duration-200 text-left ${
              selectedType === type.id
                ? 'border-primary bg-primary/10 shadow-lg'
                : 'border-border bg-card hover:border-primary/50 hover:bg-card/80'
            }`}
          >
            <div className="flex items-start space-x-3">
              <div className={`flex-shrink-0 ${type.color}`}>
                <Icon name={type.icon} size={24} />
              </div>
              
              <div className="flex-1 min-w-0">
                <div className="font-medium text-foreground mb-1">
                  {type.name}
                </div>
                <div className="text-sm text-muted-foreground">
                  {type.description}
                </div>
              </div>
              
              {selectedType === type.id && (
                <div className="flex-shrink-0 text-primary">
                  <Icon name="Check" size={16} />
                </div>
              )}
            </div>
          </button>
        ))}
      </div>
    </div>
  );
};

export default ResourceTypeSelector;