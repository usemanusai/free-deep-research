import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import Header from '../../components/ui/Header';
import WorkflowProgressNavigator from '../../components/ui/WorkflowProgressNavigator';
import DocumentOutline from './components/DocumentOutline';
import ReportEditor from './components/ReportEditor';
import ExportSidebar from './components/ExportSidebar';
import ReportGenerationProgress from './components/ReportGenerationProgress';
import Icon from '../../components/AppIcon';
import Button from '../../components/ui/Button';

const FinalReportGeneration = () => {
  const navigate = useNavigate();
  const [activeSection, setActiveSection] = useState('executive-summary');
  const [showGenerationProgress, setShowGenerationProgress] = useState(false);
  const [isReportGenerated, setIsReportGenerated] = useState(true);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const documentSections = [
    {
      id: 'executive-summary',
      title: 'Executive Summary',
      wordCount: 847,
      status: 'completed',
      hasSubsections: true,
      subsections: [
        { id: 'key-findings', title: 'Key Findings', wordCount: 234 },
        { id: 'methodology-overview', title: 'Methodology Overview', wordCount: 189 },
        { id: 'recommendations', title: 'Recommendations', wordCount: 424 }
      ]
    },
    {
      id: 'methodology',
      title: 'Research Methodology',
      wordCount: 1456,
      status: 'completed',
      hasSubsections: true,
      subsections: [
        { id: 'research-design', title: 'Research Design', wordCount: 567 },
        { id: 'data-collection', title: 'Data Collection', wordCount: 445 },
        { id: 'analysis-methods', title: 'Analysis Methods', wordCount: 444 }
      ]
    },
    {
      id: 'findings',
      title: 'Research Findings',
      wordCount: 2847,
      status: 'completed',
      hasSubsections: true,
      subsections: [
        { id: 'primary-findings', title: 'Primary Findings', wordCount: 1234 },
        { id: 'secondary-findings', title: 'Secondary Findings', wordCount: 876 },
        { id: 'statistical-analysis', title: 'Statistical Analysis', wordCount: 737 }
      ]
    },
    {
      id: 'analysis',
      title: 'Data Analysis',
      wordCount: 1923,
      status: 'in-progress',
      hasSubsections: false
    },
    {
      id: 'conclusions',
      title: 'Conclusions',
      wordCount: 756,
      status: 'pending',
      hasSubsections: false
    },
    {
      id: 'references',
      title: 'References',
      wordCount: 234,
      status: 'completed',
      hasSubsections: false
    },
    {
      id: 'appendices',
      title: 'Appendices',
      wordCount: 1567,
      status: 'completed',
      hasSubsections: false
    }
  ];

  const reportStats = {
    totalWords: documentSections.reduce((acc, section) => acc + section.wordCount, 0),
    totalPages: 47,
    totalCitations: 156,
    totalCharts: 23,
    completionPercentage: Math.round((documentSections.filter(s => s.status === 'completed').length / documentSections.length) * 100)
  };

  const handleSectionClick = (sectionId) => {
    setActiveSection(sectionId);
    setIsMobileMenuOpen(false);
  };

  const handleAddSection = () => {
    // Logic to add new section
    console.log('Adding new section...');
  };

  const handleContentChange = (content) => {
    // Logic to handle content changes
    console.log('Content changed:', content);
  };

  const handleSave = () => {
    // Logic to save document
    console.log('Saving document...');
  };

  const handleExport = (format) => {
    console.log('Exporting as:', format);
    // Logic to export document
  };

  const handleShare = (settings) => {
    console.log('Sharing with settings:', settings);
    // Logic to share document
  };

  const handleGenerateReport = () => {
    setShowGenerationProgress(true);
  };

  const handleGenerationComplete = () => {
    setShowGenerationProgress(false);
    setIsReportGenerated(true);
  };

  const handleNavigateToWorkflow = () => {
    navigate('/deep-research-workflow-dashboard');
  };

  const handleNavigateToProgress = () => {
    navigate('/research-progress-tracking');
  };

  useEffect(() => {
    // Simulate checking if report needs to be generated
    const needsGeneration = localStorage.getItem('reportGenerated') !== 'true';
    setIsReportGenerated(!needsGeneration);
  }, []);

  return (
    <div className="min-h-screen bg-background">
      <Header />
      
      {/* Generation Progress Modal */}
      <ReportGenerationProgress
        isVisible={showGenerationProgress}
        onComplete={handleGenerationComplete}
      />

      {!isReportGenerated ? (
        /* Report Generation Landing */
        <div className="container mx-auto px-6 py-12">
          <div className="max-w-4xl mx-auto text-center">
            <div className="w-24 h-24 bg-primary/10 rounded-full flex items-center justify-center mx-auto mb-8">
              <Icon name="FileText" size={48} color="var(--color-primary)" />
            </div>
            
            <h1 className="text-4xl font-bold text-foreground mb-4">
              Generate Final Report
            </h1>
            
            <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
              Transform your research data into a comprehensive, professional report with automated analysis, insights, and formatting.
            </p>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
              <div className="research-card p-6 text-center">
                <Icon name="BarChart3" size={32} color="var(--color-primary)" className="mx-auto mb-4" />
                <h3 className="font-semibold text-foreground mb-2">Data Analysis</h3>
                <p className="text-sm text-muted-foreground">
                  Automated statistical analysis and pattern recognition
                </p>
              </div>
              
              <div className="research-card p-6 text-center">
                <Icon name="FileText" size={32} color="var(--color-primary)" className="mx-auto mb-4" />
                <h3 className="font-semibold text-foreground mb-2">Professional Format</h3>
                <p className="text-sm text-muted-foreground">
                  Academic-standard formatting with citations and references
                </p>
              </div>
              
              <div className="research-card p-6 text-center">
                <Icon name="Share2" size={32} color="var(--color-primary)" className="mx-auto mb-4" />
                <h3 className="font-semibold text-foreground mb-2">Easy Export</h3>
                <p className="text-sm text-muted-foreground">
                  Multiple export formats including PDF, Word, and HTML
                </p>
              </div>
            </div>

            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Button
                variant="default"
                size="lg"
                iconName="Play"
                onClick={handleGenerateReport}
                className="research-hover"
              >
                Generate Report
              </Button>
              
              <Button
                variant="outline"
                size="lg"
                iconName="ArrowLeft"
                onClick={handleNavigateToProgress}
                className="research-hover"
              >
                Back to Progress
              </Button>
            </div>
          </div>
        </div>
      ) : (
        /* Report Editor Interface */
        <div className="flex h-[calc(100vh-64px)]">
          {/* Mobile Menu Toggle */}
          <div className="lg:hidden fixed top-20 left-4 z-40">
            <Button
              variant="outline"
              size="icon"
              iconName={isMobileMenuOpen ? "X" : "Menu"}
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              className="research-hover"
            />
          </div>

          {/* Document Outline - Left Panel */}
          <div className={`
            w-80 flex-shrink-0 transition-transform duration-300 lg:translate-x-0
            ${isMobileMenuOpen ? 'translate-x-0' : '-translate-x-full'}
            lg:relative absolute inset-y-0 left-0 z-30
          `}>
            <DocumentOutline
              sections={documentSections}
              activeSection={activeSection}
              onSectionClick={handleSectionClick}
              onAddSection={handleAddSection}
            />
          </div>

          {/* Main Editor - Center Panel */}
          <div className="flex-1 flex flex-col min-w-0">
            <ReportEditor
              activeSection={activeSection}
              onContentChange={handleContentChange}
              onSave={handleSave}
            />
          </div>

          {/* Export Sidebar - Right Panel */}
          <div className="w-80 flex-shrink-0 hidden xl:block">
            <ExportSidebar
              onExport={handleExport}
              onShare={handleShare}
              reportStats={reportStats}
            />
          </div>
        </div>
      )}

      {/* Mobile Export Panel */}
      {isReportGenerated && (
        <div className="xl:hidden fixed bottom-0 left-0 right-0 bg-card border-t border-border p-4">
          <div className="flex items-center justify-between">
            <div className="text-sm text-muted-foreground">
              {reportStats.totalWords} words • {reportStats.completionPercentage}% complete
            </div>
            <div className="flex items-center space-x-2">
              <Button
                variant="outline"
                size="sm"
                iconName="Share2"
                className="research-hover"
              >
                Share
              </Button>
              <Button
                variant="default"
                size="sm"
                iconName="Download"
                className="research-hover"
              >
                Export
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Workflow Navigation */}
      {!isReportGenerated && (
        <div className="border-t border-border">
          <WorkflowProgressNavigator />
        </div>
      )}

      {/* Mobile Menu Overlay */}
      {isMobileMenuOpen && (
        <div 
          className="lg:hidden fixed inset-0 bg-background/80 backdrop-blur-sm z-20"
          onClick={() => setIsMobileMenuOpen(false)}
        />
      )}
    </div>
  );
};

export default FinalReportGeneration;