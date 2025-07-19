import React from "react";
import { BrowserRouter, Routes as RouterRoutes, Route } from "react-router-dom";
import ScrollToTop from "components/ScrollToTop";
import ErrorBoundary from "components/ErrorBoundary";
// Add your imports here
import DeepResearchWorkflowDashboard from "pages/deep-research-workflow-dashboard";
import FinalReportGeneration from "pages/final-report-generation";
import ResearchProgressTracking from "pages/research-progress-tracking";
import SettingsAndConfiguration from "pages/settings-and-configuration";
import ResourceUploadManagement from "pages/resource-upload-management";
import NotFound from "pages/NotFound";

const Routes = () => {
  return (
    <BrowserRouter>
      <ErrorBoundary>
      <ScrollToTop />
      <RouterRoutes>
        {/* Define your routes here */}
        <Route path="/" element={<DeepResearchWorkflowDashboard />} />
        <Route path="/deep-research-workflow-dashboard" element={<DeepResearchWorkflowDashboard />} />
        <Route path="/final-report-generation" element={<FinalReportGeneration />} />
        <Route path="/research-progress-tracking" element={<ResearchProgressTracking />} />
        <Route path="/settings-and-configuration" element={<SettingsAndConfiguration />} />
        <Route path="/resource-upload-management" element={<ResourceUploadManagement />} />
        <Route path="*" element={<NotFound />} />
      </RouterRoutes>
      </ErrorBoundary>
    </BrowserRouter>
  );
};

export default Routes;