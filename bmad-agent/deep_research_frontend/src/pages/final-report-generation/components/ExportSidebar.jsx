import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const ExportSidebar = ({ onExport, onShare, reportStats }) => {
  const [exportFormat, setExportFormat] = useState('pdf');
  const [shareSettings, setShareSettings] = useState({
    isPublic: false,
    allowComments: true,
    allowDownload: true,
    expiresIn: '30days'
  });
  const [isExporting, setIsExporting] = useState(false);

  const exportFormats = [
    { id: 'pdf', name: 'PDF Document', icon: 'FileText', description: 'Professional document format' },
    { id: 'docx', name: 'Word Document', icon: 'FileType', description: 'Editable Microsoft Word format' },
    { id: 'html', name: 'Web Page', icon: 'Globe', description: 'Interactive web format' },
    { id: 'markdown', name: 'Markdown', icon: 'Hash', description: 'Plain text with formatting' }
  ];

  const handleExport = async (format) => {
    setIsExporting(true);
    try {
      // Simulate export process
      await new Promise(resolve => setTimeout(resolve, 2000));
      onExport(format);
    } finally {
      setIsExporting(false);
    }
  };

  const handleShare = () => {
    onShare(shareSettings);
  };

  const collaborators = [
    { id: 1, name: "Sarah Chen", email: "sarah.chen@research.com", role: "Editor", avatar: "https://randomuser.me/api/portraits/women/32.jpg" },
    { id: 2, name: "Michael Rodriguez", email: "m.rodriguez@research.com", role: "Reviewer", avatar: "https://randomuser.me/api/portraits/men/45.jpg" },
    { id: 3, name: "Dr. Emily Watson", email: "e.watson@research.com", role: "Supervisor", avatar: "https://randomuser.me/api/portraits/women/67.jpg" }
  ];

  return (
    <div className="h-full bg-card border-l border-border overflow-y-auto">
      {/* Export Section */}
      <div className="p-4 border-b border-border">
        <h3 className="font-semibold text-foreground mb-4">Export Options</h3>
        
        <div className="space-y-3 mb-4">
          {exportFormats.map((format) => (
            <div
              key={format.id}
              className={`
                p-3 rounded-lg border cursor-pointer research-hover transition-all
                ${exportFormat === format.id 
                  ? 'border-primary bg-primary/10' :'border-border hover:border-primary/50'
                }
              `}
              onClick={() => setExportFormat(format.id)}
            >
              <div className="flex items-center space-x-3">
                <Icon 
                  name={format.icon} 
                  size={20} 
                  color={exportFormat === format.id ? 'var(--color-primary)' : 'var(--color-muted-foreground)'} 
                />
                <div className="flex-1">
                  <div className={`font-medium text-sm ${exportFormat === format.id ? 'text-primary' : 'text-foreground'}`}>
                    {format.name}
                  </div>
                  <div className="text-xs text-muted-foreground">
                    {format.description}
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>

        <Button
          variant="default"
          fullWidth
          iconName="Download"
          loading={isExporting}
          onClick={() => handleExport(exportFormat)}
          className="research-hover"
        >
          {isExporting ? 'Exporting...' : 'Export Report'}
        </Button>
      </div>

      {/* Share Section */}
      <div className="p-4 border-b border-border">
        <h3 className="font-semibold text-foreground mb-4">Share & Collaborate</h3>
        
        <div className="space-y-4 mb-4">
          <div className="flex items-center justify-between">
            <span className="text-sm text-foreground">Public Access</span>
            <button
              className={`
                relative w-11 h-6 rounded-full transition-colors
                ${shareSettings.isPublic ? 'bg-primary' : 'bg-muted'}
              `}
              onClick={() => setShareSettings(prev => ({ ...prev, isPublic: !prev.isPublic }))}
            >
              <div className={`
                absolute top-0.5 w-5 h-5 bg-white rounded-full transition-transform
                ${shareSettings.isPublic ? 'translate-x-5' : 'translate-x-0.5'}
              `} />
            </button>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-sm text-foreground">Allow Comments</span>
            <button
              className={`
                relative w-11 h-6 rounded-full transition-colors
                ${shareSettings.allowComments ? 'bg-primary' : 'bg-muted'}
              `}
              onClick={() => setShareSettings(prev => ({ ...prev, allowComments: !prev.allowComments }))}
            >
              <div className={`
                absolute top-0.5 w-5 h-5 bg-white rounded-full transition-transform
                ${shareSettings.allowComments ? 'translate-x-5' : 'translate-x-0.5'}
              `} />
            </button>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-sm text-foreground">Allow Download</span>
            <button
              className={`
                relative w-11 h-6 rounded-full transition-colors
                ${shareSettings.allowDownload ? 'bg-primary' : 'bg-muted'}
              `}
              onClick={() => setShareSettings(prev => ({ ...prev, allowDownload: !prev.allowDownload }))}
            >
              <div className={`
                absolute top-0.5 w-5 h-5 bg-white rounded-full transition-transform
                ${shareSettings.allowDownload ? 'translate-x-5' : 'translate-x-0.5'}
              `} />
            </button>
          </div>
        </div>

        <Button
          variant="outline"
          fullWidth
          iconName="Share2"
          onClick={handleShare}
          className="research-hover mb-3"
        >
          Generate Share Link
        </Button>

        <Button
          variant="ghost"
          fullWidth
          iconName="Copy"
          className="research-hover"
        >
          Copy Link
        </Button>
      </div>

      {/* Collaborators Section */}
      <div className="p-4 border-b border-border">
        <div className="flex items-center justify-between mb-4">
          <h3 className="font-semibold text-foreground">Collaborators</h3>
          <Button
            variant="ghost"
            size="icon"
            iconName="UserPlus"
            className="h-8 w-8 research-hover"
          />
        </div>

        <div className="space-y-3">
          {collaborators.map((collaborator) => (
            <div key={collaborator.id} className="flex items-center space-x-3">
              <img
                src={collaborator.avatar}
                alt={collaborator.name}
                className="w-8 h-8 rounded-full object-cover"
              />
              <div className="flex-1 min-w-0">
                <div className="text-sm font-medium text-foreground truncate">
                  {collaborator.name}
                </div>
                <div className="text-xs text-muted-foreground">
                  {collaborator.role}
                </div>
              </div>
              <Button
                variant="ghost"
                size="icon"
                iconName="MoreHorizontal"
                className="h-6 w-6 research-hover"
              />
            </div>
          ))}
        </div>
      </div>

      {/* Report Statistics */}
      <div className="p-4">
        <h3 className="font-semibold text-foreground mb-4">Report Statistics</h3>
        
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Total Words</span>
            <span className="text-sm font-medium text-foreground">
              {reportStats?.totalWords || '12,847'}
            </span>
          </div>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Pages</span>
            <span className="text-sm font-medium text-foreground">
              {reportStats?.totalPages || '47'}
            </span>
          </div>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Citations</span>
            <span className="text-sm font-medium text-foreground">
              {reportStats?.totalCitations || '156'}
            </span>
          </div>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Charts</span>
            <span className="text-sm font-medium text-foreground">
              {reportStats?.totalCharts || '23'}
            </span>
          </div>
          
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Last Updated</span>
            <span className="text-sm font-medium text-foreground">
              {new Date().toLocaleDateString()}
            </span>
          </div>
        </div>

        <div className="mt-6 pt-4 border-t border-border">
          <Button
            variant="outline"
            fullWidth
            iconName="Eye"
            className="research-hover mb-2"
          >
            Preview Report
          </Button>
          
          <Button
            variant="ghost"
            fullWidth
            iconName="Printer"
            className="research-hover"
          >
            Print Preview
          </Button>
        </div>
      </div>
    </div>
  );
};

export default ExportSidebar;