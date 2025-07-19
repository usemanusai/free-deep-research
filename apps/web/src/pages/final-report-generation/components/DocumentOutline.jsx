import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const DocumentOutline = ({ sections, activeSection, onSectionClick, onAddSection }) => {
  const [expandedSections, setExpandedSections] = useState(new Set(['executive-summary', 'methodology']));

  const toggleSection = (sectionId) => {
    const newExpanded = new Set(expandedSections);
    if (newExpanded.has(sectionId)) {
      newExpanded.delete(sectionId);
    } else {
      newExpanded.add(sectionId);
    }
    setExpandedSections(newExpanded);
  };

  const getSectionIcon = (section) => {
    const iconMap = {
      'executive-summary': 'FileText',
      'methodology': 'Settings',
      'findings': 'TrendingUp',
      'analysis': 'BarChart3',
      'conclusions': 'CheckCircle2',
      'references': 'BookOpen',
      'appendices': 'Paperclip'
    };
    return iconMap[section.id] || 'File';
  };

  return (
    <div className="h-full bg-card border-r border-border">
      <div className="p-4 border-b border-border">
        <div className="flex items-center justify-between mb-3">
          <h3 className="font-semibold text-foreground">Document Outline</h3>
          <Button
            variant="ghost"
            size="icon"
            iconName="Plus"
            onClick={onAddSection}
            className="h-8 w-8"
          />
        </div>
        <div className="text-sm text-muted-foreground">
          {sections.length} sections • {sections.reduce((acc, s) => acc + s.wordCount, 0)} words
        </div>
      </div>

      <div className="p-2 overflow-y-auto h-[calc(100vh-200px)]">
        {sections.map((section, index) => {
          const isExpanded = expandedSections.has(section.id);
          const isActive = activeSection === section.id;
          
          return (
            <div key={section.id} className="mb-1">
              <div
                className={`
                  flex items-center justify-between p-3 rounded-lg cursor-pointer research-hover
                  ${isActive ? 'bg-primary/10 border border-primary/20' : 'hover:bg-muted/50'}
                `}
                onClick={() => onSectionClick(section.id)}
              >
                <div className="flex items-center space-x-3 flex-1 min-w-0">
                  <div className="flex items-center space-x-2">
                    <span className="text-xs font-mono text-muted-foreground w-6">
                      {index + 1}.
                    </span>
                    <Icon 
                      name={getSectionIcon(section)} 
                      size={16} 
                      color={isActive ? 'var(--color-primary)' : 'var(--color-muted-foreground)'} 
                    />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className={`font-medium text-sm truncate ${isActive ? 'text-primary' : 'text-foreground'}`}>
                      {section.title}
                    </div>
                    <div className="text-xs text-muted-foreground">
                      {section.wordCount} words • {section.status}
                    </div>
                  </div>
                </div>
                
                <div className="flex items-center space-x-1">
                  {section.hasSubsections && (
                    <Button
                      variant="ghost"
                      size="icon"
                      iconName={isExpanded ? "ChevronDown" : "ChevronRight"}
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleSection(section.id);
                      }}
                      className="h-6 w-6"
                    />
                  )}
                  <div className={`w-2 h-2 rounded-full ${
                    section.status === 'completed' ? 'bg-success' :
                    section.status === 'in-progress'? 'bg-warning' : 'bg-muted'
                  }`} />
                </div>
              </div>

              {/* Subsections */}
              {isExpanded && section.subsections && (
                <div className="ml-6 mt-1 space-y-1">
                  {section.subsections.map((subsection, subIndex) => (
                    <div
                      key={subsection.id}
                      className={`
                        flex items-center space-x-3 p-2 rounded cursor-pointer research-hover
                        ${activeSection === subsection.id ? 'bg-primary/5 text-primary' : 'hover:bg-muted/30 text-muted-foreground'}
                      `}
                      onClick={() => onSectionClick(subsection.id)}
                    >
                      <span className="text-xs font-mono w-8">
                        {index + 1}.{subIndex + 1}
                      </span>
                      <Icon name="FileText" size={14} />
                      <span className="text-sm truncate">{subsection.title}</span>
                      <span className="text-xs ml-auto">{subsection.wordCount}w</span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          );
        })}
      </div>

      {/* Quick Actions */}
      <div className="p-4 border-t border-border">
        <div className="space-y-2">
          <Button
            variant="outline"
            size="sm"
            iconName="Search"
            fullWidth
            className="justify-start"
          >
            Search Content
          </Button>
          <Button
            variant="ghost"
            size="sm"
            iconName="RotateCcw"
            fullWidth
            className="justify-start"
          >
            Reset Outline
          </Button>
        </div>
      </div>
    </div>
  );
};

export default DocumentOutline;