import React, { useState } from 'react';
import Button from '../../../components/ui/Button';
import Input from '../../../components/ui/Input';
import Icon from '../../../components/AppIcon';

const WebPageUpload = ({ onWebPageAdd }) => {
  const [url, setUrl] = useState('');
  const [isValidating, setIsValidating] = useState(false);
  const [validationError, setValidationError] = useState('');
  const [preview, setPreview] = useState(null);

  const validateUrl = (urlString) => {
    try {
      const urlObj = new URL(urlString);
      return urlObj.protocol === 'http:' || urlObj.protocol === 'https:';
    } catch {
      return false;
    }
  };

  const fetchPageMetadata = async (url) => {
    setIsValidating(true);
    setValidationError('');
    
    try {
      // Simulate metadata extraction
      // In a real app, this would call an API to extract metadata
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      const metadata = {
        title: `Page Title from ${new URL(url).hostname}`,
        description: 'This is a preview description of the web page content. It provides a summary of what the page contains.',
        favicon: null,
        url: url
      };
      
      setPreview(metadata);
      return metadata;
    } catch (error) {
      setValidationError('Failed to fetch page metadata. Please check the URL and try again.');
      return null;
    } finally {
      setIsValidating(false);
    }
  };

  const handleUrlSubmit = async (e) => {
    e.preventDefault();
    
    if (!url.trim()) {
      setValidationError('Please enter a URL');
      return;
    }

    if (!validateUrl(url)) {
      setValidationError('Please enter a valid HTTP or HTTPS URL');
      return;
    }

    const metadata = await fetchPageMetadata(url);
    if (metadata) {
      setPreview(metadata);
    }
  };

  const handleAddPage = () => {
    if (preview) {
      onWebPageAdd(preview);
      setUrl('');
      setPreview(null);
      setValidationError('');
    }
  };

  const handleCancel = () => {
    setUrl('');
    setPreview(null);
    setValidationError('');
  };

  return (
    <div className="space-y-6">
      <div className="research-card p-6">
        <h3 className="text-lg font-semibold text-foreground mb-4">
          Add Web Page Resource
        </h3>
        
        <form onSubmit={handleUrlSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-foreground mb-2">
              Web Page URL
            </label>
            <div className="flex space-x-2">
              <Input
                type="url"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="https://example.com/article"
                className="flex-1"
                disabled={isValidating}
              />
              <Button
                type="submit"
                variant="outline"
                iconName="Search"
                loading={isValidating}
                disabled={isValidating || !url.trim()}
              >
                {isValidating ? 'Validating...' : 'Validate'}
              </Button>
            </div>
          </div>
          
          {validationError && (
            <div className="p-3 bg-destructive/10 border border-destructive/20 rounded-md">
              <div className="flex items-center space-x-2">
                <Icon name="AlertTriangle" size={16} className="text-destructive" />
                <span className="text-sm text-destructive">{validationError}</span>
              </div>
            </div>
          )}
        </form>
        
        {/* URL Preview */}
        {preview && (
          <div className="mt-6 p-4 bg-muted/10 border border-border rounded-lg">
            <div className="flex items-start space-x-3">
              <div className="flex-shrink-0 p-2 bg-primary/10 rounded-lg">
                <Icon name="Globe" size={20} className="text-primary" />
              </div>
              
              <div className="flex-1 min-w-0">
                <div className="font-medium text-foreground mb-1">
                  {preview.title}
                </div>
                <div className="text-sm text-muted-foreground mb-2">
                  {preview.description}
                </div>
                <div className="text-xs text-muted-foreground font-mono">
                  {preview.url}
                </div>
              </div>
            </div>
            
            <div className="flex space-x-2 mt-4">
              <Button
                onClick={handleAddPage}
                size="sm"
                iconName="Plus"
                className="flex-1"
              >
                Add to Resources
              </Button>
              <Button
                onClick={handleCancel}
                variant="outline"
                size="sm"
                iconName="X"
              >
                Cancel
              </Button>
            </div>
          </div>
        )}
      </div>
      
      {/* Quick Add Options */}
      <div className="research-card p-6">
        <h4 className="text-md font-semibold text-foreground mb-3">
          Quick Add Options
        </h4>
        
        <div className="space-y-3">
          <Button
            variant="outline"
            size="sm"
            iconName="Bookmark"
            fullWidth
            className="justify-start"
          >
            Import from Bookmarks
          </Button>
          
          <Button
            variant="outline"
            size="sm"
            iconName="Link"
            fullWidth
            className="justify-start"
          >
            Batch URL Import
          </Button>
          
          <Button
            variant="outline"
            size="sm"
            iconName="Download"
            fullWidth
            className="justify-start"
          >
            Save Page as PDF
          </Button>
        </div>
      </div>
    </div>
  );
};

export default WebPageUpload;