import React from 'react';
import Button from '../../../components/ui/Button';
import Icon from '../../../components/AppIcon';

const ResourceList = ({ resources, onResourceRemove }) => {
  const formatFileSize = (bytes) => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const getResourceIcon = (resource) => {
    if (resource.url) return 'Globe';
    if (resource.type?.includes('image')) return 'Image';
    if (resource.type?.includes('pdf')) return 'FileText';
    if (resource.type?.includes('document')) return 'FileText';
    return 'File';
  };

  const getResourceTypeColor = (category) => {
    switch (category) {
      case 'knowledge':
        return 'text-blue-400';
      case 'files':
        return 'text-green-400';
      case 'webpages':
        return 'text-purple-400';
      default:
        return 'text-muted-foreground';
    }
  };

  const formatDate = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  if (resources.length === 0) {
    return (
      <div className="research-card p-6">
        <h3 className="text-lg font-semibold text-foreground mb-4">
          Uploaded Resources
        </h3>
        
        <div className="text-center py-8">
          <div className="flex justify-center mb-4">
            <div className="p-4 bg-muted/20 rounded-full">
              <Icon name="FileText" size={32} className="text-muted-foreground" />
            </div>
          </div>
          <div className="text-sm text-muted-foreground">
            No resources uploaded yet
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="research-card p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-foreground">
          Uploaded Resources
        </h3>
        <div className="text-sm text-muted-foreground">
          {resources.length} item{resources.length !== 1 ? 's' : ''}
        </div>
      </div>
      
      <div className="space-y-3 max-h-96 overflow-y-auto">
        {resources.map((resource) => (
          <div
            key={resource.id}
            className="p-3 bg-muted/10 border border-border rounded-lg hover:bg-muted/20 transition-colors"
          >
            <div className="flex items-start space-x-3">
              {/* Thumbnail or Icon */}
              <div className="flex-shrink-0">
                {resource.thumbnail ? (
                  <img
                    src={resource.thumbnail}
                    alt={resource.name}
                    className="w-10 h-10 object-cover rounded-md"
                  />
                ) : (
                  <div className={`p-2 rounded-md bg-muted/20 ${getResourceTypeColor(resource.category)}`}>
                    <Icon name={getResourceIcon(resource)} size={16} />
                  </div>
                )}
              </div>
              
              {/* Resource Info */}
              <div className="flex-1 min-w-0">
                <div className="font-medium text-foreground text-sm mb-1 truncate">
                  {resource.name}
                </div>
                
                <div className="flex items-center space-x-2 text-xs text-muted-foreground">
                  {resource.size && (
                    <span>{formatFileSize(resource.size)}</span>
                  )}
                  {resource.size && <span>•</span>}
                  <span>{formatDate(resource.uploadDate)}</span>
                </div>
                
                {resource.url && (
                  <div className="text-xs text-muted-foreground mt-1 truncate">
                    {resource.url}
                  </div>
                )}
              </div>
              
              {/* Remove Button */}
              <Button
                variant="ghost"
                size="icon"
                iconName="Trash2"
                onClick={() => onResourceRemove(resource.id)}
                className="flex-shrink-0 h-8 w-8 text-muted-foreground hover:text-destructive"
              />
            </div>
          </div>
        ))}
      </div>
      
      {/* Bulk Actions */}
      {resources.length > 0 && (
        <div className="pt-4 border-t border-border mt-4">
          <div className="flex space-x-2">
            <Button
              variant="outline"
              size="sm"
              iconName="Download"
              className="flex-1"
            >
              Export All
            </Button>
            <Button
              variant="outline"
              size="sm"
              iconName="Trash2"
              className="text-destructive hover:text-destructive"
            >
              Clear All
            </Button>
          </div>
        </div>
      )}
    </div>
  );
};

export default ResourceList;