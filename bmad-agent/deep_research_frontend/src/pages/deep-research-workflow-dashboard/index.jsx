import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import Header from '../../components/ui/Header';
import WorkflowProgressNavigator from '../../components/ui/WorkflowProgressNavigator';
import ResearchTopicsSection from './components/ResearchTopicsSection';
import QuestionSection from './components/QuestionSection';
import InformationCollectionSection from './components/InformationCollectionSection';
import FinalReportSection from './components/FinalReportSection';
import WorkflowControls from './components/WorkflowControls';

const DeepResearchWorkflowDashboard = () => {
  const navigate = useNavigate();
  
  // Workflow state management
  const [workflowState, setWorkflowState] = useState({
    isActive: false,
    progress: 0,
    currentStage: 0
  });

  // Section expansion states
  const [expandedSections, setExpandedSections] = useState({
    topics: true,
    questions: false,
    collection: false,
    report: false
  });

  // Research data states
  const [researchTopics, setResearchTopics] = useState('');
  const [questionStatus, setQuestionStatus] = useState('waiting');
  const [collectionStatus, setCollectionStatus] = useState('waiting');
  const [reportStatus, setReportStatus] = useState('waiting');
  const [collectionProgress, setCollectionProgress] = useState(0);

  // Mock data
  const [questions] = useState([
    "What are the current market trends and growth projections for artificial intelligence in healthcare?",
    "How do regulatory frameworks impact AI implementation in medical diagnostics?",
    "What are the key challenges and barriers to AI adoption in healthcare institutions?",
    "Which AI technologies show the most promise for improving patient outcomes?",
    "How do healthcare professionals perceive AI integration in their workflow?"
  ]);

  const [collectedSources] = useState([
    {
      title: "AI in Healthcare: Market Analysis 2024",
      description: "Comprehensive market research on AI adoption in healthcare sector",
      type: "web",
      relevance: 95
    },
    {
      title: "FDA Guidelines for AI Medical Devices",
      description: "Official regulatory framework for AI-powered medical devices",
      type: "document",
      relevance: 88
    },
    {
      title: "Healthcare AI Implementation Case Studies",
      description: "Real-world examples of successful AI integration in hospitals",
      type: "web",
      relevance: 92
    },
    {
      title: "Machine Learning in Diagnostics Research Paper",
      description: "Peer-reviewed research on ML applications in medical diagnostics",
      type: "document",
      relevance: 90
    },
    {
      title: "Healthcare Professional AI Survey Results",
      description: "Survey data on healthcare worker attitudes toward AI technology",
      type: "web",
      relevance: 85
    }
  ]);

  const [reportData] = useState({
    pages: 24,
    sources: 15,
    insights: 8
  });

  // Toggle section expansion
  const toggleSection = (section) => {
    setExpandedSections(prev => ({
      ...prev,
      [section]: !prev[section]
    }));
  };

  // Handle workflow start
  const handleStartWorkflow = () => {
    if (!researchTopics.trim()) {
      alert('Please define your research topics first');
      return;
    }

    setWorkflowState(prev => ({ ...prev, isActive: true }));
    
    // Simulate workflow progression
    setTimeout(() => {
      setQuestionStatus('processing');
      setWorkflowState(prev => ({ ...prev, progress: 25 }));
    }, 1000);

    setTimeout(() => {
      setQuestionStatus('completed');
      setCollectionStatus('collecting');
      setWorkflowState(prev => ({ ...prev, progress: 50 }));
      
      // Simulate collection progress
      let progress = 0;
      const interval = setInterval(() => {
        progress += 10;
        setCollectionProgress(progress);
        if (progress >= 100) {
          clearInterval(interval);
          setCollectionStatus('completed');
          setReportStatus('ready');
          setWorkflowState(prev => ({ ...prev, progress: 75 }));
        }
      }, 500);
    }, 3000);
  };

  // Handle workflow pause
  const handlePauseWorkflow = () => {
    setWorkflowState(prev => ({ ...prev, isActive: false }));
  };

  // Handle workflow reset
  const handleResetWorkflow = () => {
    setWorkflowState({ isActive: false, progress: 0, currentStage: 0 });
    setQuestionStatus('waiting');
    setCollectionStatus('waiting');
    setReportStatus('waiting');
    setCollectionProgress(0);
    setResearchTopics('');
  };

  // Handle resource addition
  const handleAddResource = (type) => {
    switch (type) {
      case 'knowledge': alert('Knowledge base integration coming soon');
        break;
      case 'file': navigate('/resource-upload-management');
        break;
      case 'webpage':
        const url = prompt('Enter webpage URL:');
        if (url) {
          alert(`Webpage "${url}" will be added to resources`);
        }
        break;
      default:
        break;
    }
  };

  // Handle report generation
  const handleGenerateReport = () => {
    setReportStatus('generating');
    setTimeout(() => {
      setReportStatus('completed');
      setWorkflowState(prev => ({ ...prev, progress: 100 }));
    }, 3000);
  };

  // Handle view report
  const handleViewReport = () => {
    navigate('/final-report-generation');
  };

  // Update question status based on topics
  useEffect(() => {
    if (researchTopics.trim() && questionStatus === 'waiting' && !workflowState.isActive) {
      setQuestionStatus('ready');
    } else if (!researchTopics.trim() && questionStatus === 'ready') {
      setQuestionStatus('waiting');
    }
  }, [researchTopics, questionStatus, workflowState.isActive]);

  return (
    <div className="min-h-screen bg-background">
      <Header />
      
      <main className="container mx-auto px-6 py-8 max-w-6xl">
        {/* Page Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-foreground mb-2">
            Deep Research Workflow Dashboard
          </h1>
          <p className="text-muted-foreground text-lg">
            Conduct comprehensive research through our structured 4-step process
          </p>
        </div>

        {/* Workflow Progress Navigator */}
        <div className="mb-8">
          <WorkflowProgressNavigator />
        </div>

        {/* Workflow Controls */}
        <div className="mb-8">
          <WorkflowControls
            isWorkflowActive={workflowState.isActive}
            onStartWorkflow={handleStartWorkflow}
            onPauseWorkflow={handlePauseWorkflow}
            onResetWorkflow={handleResetWorkflow}
            workflowProgress={workflowState.progress}
          />
        </div>

        {/* Workflow Sections */}
        <div className="space-y-6">
          {/* Research Topics Section */}
          <ResearchTopicsSection
            isExpanded={expandedSections.topics}
            onToggle={() => toggleSection('topics')}
            researchTopics={researchTopics}
            onTopicsChange={setResearchTopics}
            onAddResource={handleAddResource}
          />

          {/* Question Section */}
          <QuestionSection
            isExpanded={expandedSections.questions}
            onToggle={() => toggleSection('questions')}
            status={questionStatus}
            questions={questions}
          />

          {/* Information Collection Section */}
          <InformationCollectionSection
            isExpanded={expandedSections.collection}
            onToggle={() => toggleSection('collection')}
            status={collectionStatus}
            progress={collectionProgress}
            collectedSources={collectedSources}
          />

          {/* Final Report Section */}
          <FinalReportSection
            isExpanded={expandedSections.report}
            onToggle={() => toggleSection('report')}
            status={reportStatus}
            reportData={reportData}
            onGenerateReport={handleGenerateReport}
            onViewReport={handleViewReport}
          />
        </div>

        {/* Footer */}
        <footer className="mt-16 pt-8 border-t border-border text-center">
          <p className="text-muted-foreground">
            Created with ❤️ by U14App team • {new Date().getFullYear()}
          </p>
        </footer>
      </main>
    </div>
  );
};

export default DeepResearchWorkflowDashboard;