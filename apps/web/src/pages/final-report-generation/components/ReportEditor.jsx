import React, { useState, useRef } from 'react';

import Button from '../../../components/ui/Button';

const ReportEditor = ({ activeSection, content, onContentChange, onSave }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [showToolbar, setShowToolbar] = useState(false);
  const [selectedText, setSelectedText] = useState('');
  const editorRef = useRef(null);

  const toolbarOptions = [
    { name: 'Bold', icon: 'Bold', action: 'bold' },
    { name: 'Italic', icon: 'Italic', action: 'italic' },
    { name: 'Underline', icon: 'Underline', action: 'underline' },
    { name: 'Heading', icon: 'Heading', action: 'heading' },
    { name: 'List', icon: 'List', action: 'list' },
    { name: 'Link', icon: 'Link', action: 'link' },
    { name: 'Quote', icon: 'Quote', action: 'quote' },
    { name: 'Code', icon: 'Code', action: 'code' }
  ];

  const handleTextSelection = () => {
    const selection = window.getSelection();
    const text = selection.toString();
    setSelectedText(text);
    setShowToolbar(text.length > 0);
  };

  const applyFormatting = (action) => {
    document.execCommand(action, false, null);
    editorRef.current?.focus();
  };

  const insertCitation = () => {
    const citation = `[${Math.floor(Math.random() * 50) + 1}]`;
    document.execCommand('insertText', false, citation);
  };

  const insertChart = () => {
    const chartPlaceholder = `\n[CHART: Research Findings Analysis - Generated on ${new Date().toLocaleDateString()}]\n`;
    document.execCommand('insertText', false, chartPlaceholder);
  };

  const mockContent = {
    'executive-summary': `# Executive Summary

This comprehensive research report presents findings from an extensive analysis conducted between January and July 2025. The study examined multiple data sources and employed rigorous methodological approaches to ensure accuracy and reliability.

## Key Findings

Our research identified three primary trends that significantly impact the current landscape:

1. **Digital Transformation Acceleration**: Organizations have increased their digital adoption rates by 340% compared to pre-2024 levels, with particular emphasis on cloud-based solutions and AI integration.

2. **Sustainability Integration**: 78% of surveyed entities have implemented comprehensive sustainability frameworks, representing a 45% increase from the previous year.

3. **Workforce Evolution**: Remote and hybrid work models have stabilized at 65% adoption rate, with productivity metrics showing consistent improvement across all measured parameters.

## Methodology Overview

The research employed a mixed-methods approach combining quantitative analysis of 2,847 data points with qualitative insights from 156 structured interviews. Data collection spanned six months, ensuring seasonal variations were captured and analyzed.

## Implications and Recommendations

Based on our findings, we recommend immediate action in three critical areas: technology infrastructure modernization, sustainability policy implementation, and workforce development programs. These recommendations are supported by statistical evidence and expert validation.`,

    'methodology': `# Research Methodology

## Research Design

This study employed a comprehensive mixed-methods research design to ensure robust data collection and analysis. The methodology was structured around four core phases:

### Phase 1: Literature Review and Framework Development
- Systematic review of 247 peer-reviewed articles
- Analysis of industry reports from 15 leading organizations
- Development of theoretical framework based on established models

### Phase 2: Quantitative Data Collection
- **Sample Size**: 2,847 participants across 12 geographic regions
- **Data Collection Period**: January 15, 2025 - June 30, 2025
- **Response Rate**: 73.2% (above industry standard of 65%)
- **Sampling Method**: Stratified random sampling with demographic controls

### Phase 3: Qualitative Research
- **In-depth Interviews**: 156 structured interviews (45-60 minutes each)
- **Focus Groups**: 24 sessions with 8-12 participants per group
- **Expert Consultations**: 31 subject matter experts across relevant fields

### Phase 4: Data Analysis and Validation
- Statistical analysis using SPSS v29 and R programming language
- Qualitative analysis through thematic coding using NVivo
- Cross-validation through triangulation of multiple data sources

## Data Quality Assurance

Multiple measures were implemented to ensure data integrity:
- Pre-testing of all instruments with pilot groups (n=150)
- Inter-rater reliability testing (Cohen's kappa = 0.87)
- Regular data audits and cleaning procedures
- Bias detection and mitigation protocols

## Ethical Considerations

All research activities were conducted in accordance with institutional review board guidelines. Participants provided informed consent, and data anonymization protocols were strictly followed throughout the study.`,

    'findings': `# Research Findings

## Primary Research Outcomes

### Finding 1: Digital Transformation Acceleration

Our analysis reveals unprecedented acceleration in digital transformation initiatives across all sectors studied.

**Key Statistics:**
- 340% increase in digital adoption rates since 2024
- Average implementation time reduced from 18 months to 7 months
- ROI improvement of 156% within first year of implementation

**Supporting Evidence:**
The data shows consistent patterns across geographic regions, with North American organizations leading at 89% adoption rate, followed by European entities at 82%, and Asia-Pacific at 76%.

### Finding 2: Sustainability Integration Trends

Environmental sustainability has become a core business imperative rather than a peripheral consideration.

**Quantitative Results:**
- 78% of organizations have implemented comprehensive sustainability frameworks
- Carbon footprint reduction averaging 34% across participating entities
- Sustainability-linked performance metrics adopted by 67% of leadership teams

**Qualitative Insights:**
Interview participants consistently emphasized the shift from compliance-driven to value-driven sustainability approaches. As one executive noted: "Sustainability isn't just about meeting regulations anymore—it's about creating competitive advantage and long-term resilience."

### Finding 3: Workforce Evolution Patterns

The nature of work has fundamentally shifted, with new models becoming permanently embedded in organizational structures.

**Work Model Distribution:**
- Remote work: 28% of workforce
- Hybrid arrangements: 37% of workforce  
- Traditional office-based: 35% of workforce

**Productivity Metrics:**
- Overall productivity increased by 23% compared to pre-2024 baselines
- Employee satisfaction scores improved by 31%
- Retention rates increased by 19% in organizations with flexible work policies

## Secondary Findings

### Technology Infrastructure Modernization
Organizations investing in cloud infrastructure showed 45% better performance metrics compared to those maintaining legacy systems.

### Skills Gap Analysis
Critical skills shortages identified in:
- Data analytics and interpretation (67% of organizations affected)
- Cybersecurity expertise (54% reporting gaps)
- Digital marketing and customer experience (48% seeking talent)

### Regional Variations
Significant differences observed across geographic regions, with developing markets showing faster adoption rates for mobile-first solutions while established markets focused on integration and optimization.`
  };

  const currentContent = mockContent[activeSection] || 'Select a section to view content...';

  return (
    <div className="h-full bg-background">
      {/* Editor Header */}
      <div className="flex items-center justify-between p-4 border-b border-border">
        <div className="flex items-center space-x-4">
          <h2 className="text-xl font-semibold text-foreground">
            {activeSection ? activeSection.replace('-', ' ').replace(/\b\w/g, l => l.toUpperCase()) : 'Report Editor'}
          </h2>
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 bg-success rounded-full" />
            <span className="text-sm text-muted-foreground">Auto-saved 2 minutes ago</span>
          </div>
        </div>
        
        <div className="flex items-center space-x-2">
          <Button
            variant="ghost"
            size="sm"
            iconName="Search"
            className="research-hover"
          >
            Find
          </Button>
          <Button
            variant="ghost"
            size="sm"
            iconName="RotateCcw"
            className="research-hover"
          >
            Undo
          </Button>
          <Button
            variant="outline"
            size="sm"
            iconName="Save"
            onClick={onSave}
            className="research-hover"
          >
            Save
          </Button>
        </div>
      </div>

      {/* Formatting Toolbar */}
      {showToolbar && (
        <div className="flex items-center space-x-1 p-2 border-b border-border bg-card">
          {toolbarOptions.map((tool) => (
            <Button
              key={tool.action}
              variant="ghost"
              size="icon"
              iconName={tool.icon}
              onClick={() => applyFormatting(tool.action)}
              className="h-8 w-8 research-hover"
              title={tool.name}
            />
          ))}
          <div className="w-px h-6 bg-border mx-2" />
          <Button
            variant="ghost"
            size="icon"
            iconName="Quote"
            onClick={insertCitation}
            className="h-8 w-8 research-hover"
            title="Insert Citation"
          />
          <Button
            variant="ghost"
            size="icon"
            iconName="BarChart3"
            onClick={insertChart}
            className="h-8 w-8 research-hover"
            title="Insert Chart"
          />
        </div>
      )}

      {/* Editor Content */}
      <div className="flex-1 overflow-y-auto">
        <div
          ref={editorRef}
          className="p-8 max-w-4xl mx-auto min-h-full"
          contentEditable={isEditing}
          onMouseUp={handleTextSelection}
          onKeyUp={handleTextSelection}
          suppressContentEditableWarning={true}
          style={{
            outline: 'none',
            lineHeight: '1.7',
            fontSize: '16px',
            fontFamily: 'Inter, sans-serif'
          }}
        >
          <div className="prose prose-invert max-w-none">
            {currentContent.split('\n').map((line, index) => {
              if (line.startsWith('# ')) {
                return (
                  <h1 key={index} className="text-3xl font-bold text-foreground mb-6 mt-8">
                    {line.substring(2)}
                  </h1>
                );
              } else if (line.startsWith('## ')) {
                return (
                  <h2 key={index} className="text-2xl font-semibold text-foreground mb-4 mt-6">
                    {line.substring(3)}
                  </h2>
                );
              } else if (line.startsWith('### ')) {
                return (
                  <h3 key={index} className="text-xl font-medium text-foreground mb-3 mt-5">
                    {line.substring(4)}
                  </h3>
                );
              } else if (line.startsWith('**') && line.endsWith('**')) {
                return (
                  <p key={index} className="font-semibold text-foreground mb-3">
                    {line.substring(2, line.length - 2)}
                  </p>
                );
              } else if (line.startsWith('- ')) {
                return (
                  <li key={index} className="text-foreground mb-2 ml-4">
                    {line.substring(2)}
                  </li>
                );
              } else if (line.trim() === '') {
                return <br key={index} />;
              } else {
                return (
                  <p key={index} className="text-foreground mb-4 leading-relaxed">
                    {line}
                  </p>
                );
              }
            })}
          </div>
        </div>
      </div>

      {/* Editor Footer */}
      <div className="flex items-center justify-between p-4 border-t border-border bg-card">
        <div className="flex items-center space-x-4 text-sm text-muted-foreground">
          <span>Words: {currentContent.split(' ').length}</span>
          <span>Characters: {currentContent.length}</span>
          <span>Reading time: ~{Math.ceil(currentContent.split(' ').length / 200)} min</span>
        </div>
        
        <div className="flex items-center space-x-2">
          <Button
            variant={isEditing ? "default" : "outline"}
            size="sm"
            iconName={isEditing ? "Check" : "Edit"}
            onClick={() => setIsEditing(!isEditing)}
            className="research-hover"
          >
            {isEditing ? 'Done' : 'Edit'}
          </Button>
        </div>
      </div>
    </div>
  );
};

export default ReportEditor;