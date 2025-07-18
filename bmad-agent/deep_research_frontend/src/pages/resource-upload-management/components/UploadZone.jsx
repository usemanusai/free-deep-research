import React, { useCallback, useState } from 'react';
import { useDropzone } from 'react-dropzone';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const UploadZone = ({ 
  onFileDrop, 
  acceptedTypes, 
  isUploading, 
  uploadProgress, 
  resourceType 
}) => {
  const [dragActive, setDragActive] = useState(false);
  const [validationError, setValidationError] = useState('');

  const validateFile = (file) => {
    const maxSize = 50 * 1024 * 1024; // 50MB
    const allowedTypes = {
      knowledge: ['.pdf', '.doc', '.docx', '.txt', '.md', '.rtf'],
      files: ['.pdf', '.doc', '.docx', '.txt', '.md', '.rtf', '.jpg', '.jpeg', '.png', '.gif', '.xlsx', '.xls', '.ppt', '.pptx']
    };

    if (file.size > maxSize) {
      return 'File size exceeds 50MB limit';
    }

    const fileExtension = '.' + file.name.split('.').pop().toLowerCase();
    const typeExtensions = allowedTypes[resourceType] || allowedTypes.files;
    
    if (!typeExtensions.includes(fileExtension)) {
      return `File type ${fileExtension} is not supported for ${resourceType}`;
    }

    return null;
  };

  const onDrop = useCallback((acceptedFiles, rejectedFiles) => {
    setValidationError('');
    
    if (rejectedFiles.length > 0) {
      setValidationError('Some files were rejected. Please check file types and sizes.');
      return;
    }

    // Validate each file
    for (const file of acceptedFiles) {
      const error = validateFile(file);
      if (error) {
        setValidationError(error);
        return;
      }
    }

    onFileDrop(acceptedFiles);
  }, [onFileDrop, resourceType]);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    multiple: true,
    accept: acceptedTypes ? acceptedTypes.split(',').reduce((acc, type) => ({
      ...acc,
      [type]: []
    }), {}) : undefined
  });

  const getResourceTypeInfo = () => {
    switch (resourceType) {
      case 'knowledge':
        return {
          title: 'Upload Knowledge Resources',
          description: 'Drag and drop research papers, articles, and documents',
          icon: 'BookOpen',
          supportedFormats: 'PDF, DOC, DOCX, TXT, MD, RTF'
        };
      case 'files':
        return {
          title: 'Upload Local Files',
          description: 'Drag and drop any supported file type',
          icon: 'FileText',
          supportedFormats: 'PDF, DOC, DOCX, TXT, MD, RTF, JPG, PNG, GIF, XLSX, PPTX'
        };
      default:
        return {
          title: 'Upload Files',
          description: 'Drag and drop files here',
          icon: 'Upload',
          supportedFormats: 'Multiple formats supported'
        };
    }
  };

  const typeInfo = getResourceTypeInfo();

  return (
    <div className="space-y-6">
      <div className="research-card p-6">
        <h3 className="text-lg font-semibold text-foreground mb-4">
          {typeInfo.title}
        </h3>
        
        <div
          {...getRootProps()}
          className={`border-2 border-dashed rounded-lg p-8 text-center transition-all duration-200 cursor-pointer ${
            isDragActive || dragActive
              ? 'border-primary bg-primary/10 shadow-lg'
              : 'border-border hover:border-primary/50 hover:bg-card/80'
          }`}
        >
          <input {...getInputProps()} />
          
          <div className="flex flex-col items-center space-y-4">
            <div className={`p-4 rounded-full ${
              isDragActive || dragActive
                ? 'bg-primary/20 text-primary' :'bg-muted/20 text-muted-foreground'
            }`}>
              <Icon name={typeInfo.icon} size={32} />
            </div>
            
            <div className="space-y-2">
              <div className="text-lg font-medium text-foreground">
                {isDragActive ? 'Drop files here' : typeInfo.description}
              </div>
              
              <div className="text-sm text-muted-foreground">
                or <span className="text-primary font-medium">browse files</span>
              </div>
              
              <div className="text-xs text-muted-foreground">
                Supported formats: {typeInfo.supportedFormats}
              </div>
            </div>
            
            <Button
              variant="outline"
              size="sm"
              className="mt-4"
              disabled={isUploading}
            >
              Select Files
            </Button>
          </div>
        </div>
        
        {/* Upload Progress */}
        {isUploading && (
          <div className="mt-4 space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Uploading...</span>
              <span className="text-primary font-medium">{uploadProgress}%</span>
            </div>
            <div className="w-full bg-muted rounded-full h-2">
              <div
                className="bg-primary h-2 rounded-full transition-all duration-300"
                style={{ width: `${uploadProgress}%` }}
              />
            </div>
          </div>
        )}
        
        {/* Validation Error */}
        {validationError && (
          <div className="mt-4 p-3 bg-destructive/10 border border-destructive/20 rounded-md">
            <div className="flex items-center space-x-2">
              <Icon name="AlertTriangle" size={16} className="text-destructive" />
              <span className="text-sm text-destructive">{validationError}</span>
            </div>
          </div>
        )}
      </div>
      
      {/* Bulk Upload Options */}
      <div className="research-card p-6">
        <h4 className="text-md font-semibold text-foreground mb-3">
          Bulk Upload Options
        </h4>
        
        <div className="space-y-3">
          <Button
            variant="outline"
            size="sm"
            iconName="FolderOpen"
            fullWidth
            disabled={isUploading}
            className="justify-start"
          >
            Select Multiple Files
          </Button>
          
          <Button
            variant="outline"
            size="sm"
            iconName="Archive"
            fullWidth
            disabled={isUploading}
            className="justify-start"
          >
            Upload ZIP Archive
          </Button>
        </div>
      </div>
    </div>
  );
};

export default UploadZone;