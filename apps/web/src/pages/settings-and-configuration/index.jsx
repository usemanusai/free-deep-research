import React, { useState } from 'react';
import Header from '../../components/ui/Header';
import SettingsSidebar from './components/SettingsSidebar';
import AccountSettings from './components/AccountSettings';
import ResearchPreferences from './components/ResearchPreferences';
import IntegrationSettings from './components/IntegrationSettings';
import NotificationSettings from './components/NotificationSettings';
import PrivacySettings from './components/PrivacySettings';

const SettingsAndConfiguration = () => {
  const [activeSection, setActiveSection] = useState('account');
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const handleSectionChange = (section) => {
    setActiveSection(section);
  };

  const handleMobileMenuToggle = (isOpen) => {
    if (typeof isOpen === 'boolean') {
      setIsMobileMenuOpen(isOpen);
    } else {
      setIsMobileMenuOpen(!isMobileMenuOpen);
    }
  };

  const renderActiveSection = () => {
    switch (activeSection) {
      case 'account':
        return <AccountSettings />;
      case 'research':
        return <ResearchPreferences />;
      case 'integrations':
        return <IntegrationSettings />;
      case 'notifications':
        return <NotificationSettings />;
      case 'privacy':
        return <PrivacySettings />;
      default:
        return <AccountSettings />;
    }
  };

  const getSectionTitle = () => {
    switch (activeSection) {
      case 'account':
        return 'Account Settings';
      case 'research':
        return 'Research Preferences';
      case 'integrations':
        return 'Integration Settings';
      case 'notifications':
        return 'Notification Settings';
      case 'privacy':
        return 'Privacy & Security';
      default:
        return 'Account Settings';
    }
  };

  return (
    <div className="min-h-screen bg-background">
      <Header />
      
      <div className="max-w-7xl mx-auto px-6 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
          {/* Sidebar */}
          <div className="lg:col-span-3">
            <SettingsSidebar
              activeSection={activeSection}
              onSectionChange={handleSectionChange}
              isMobileMenuOpen={isMobileMenuOpen}
              onMobileMenuToggle={handleMobileMenuToggle}
            />
          </div>

          {/* Main Content */}
          <div className="lg:col-span-9">
            <div className="mb-8">
              <h1 className="text-3xl font-bold text-foreground mb-2">
                {getSectionTitle()}
              </h1>
              <p className="text-muted-foreground">
                Last updated: {new Date().toLocaleDateString('en-US', {
                  year: 'numeric',
                  month: 'long',
                  day: 'numeric'
                })}
              </p>
            </div>

            {/* Active Section Content */}
            <div className="space-y-8">
              {renderActiveSection()}
            </div>
          </div>
        </div>
      </div>

      {/* Footer */}
      <footer className="border-t border-border bg-card mt-16">
        <div className="max-w-7xl mx-auto px-6 py-8">
          <div className="flex flex-col md:flex-row items-center justify-between">
            <div className="text-sm text-muted-foreground mb-4 md:mb-0">
              Created with ❤️ by U14App team
            </div>
            <div className="text-sm text-muted-foreground">
              © {new Date().getFullYear()} Deep Research Frontend. All rights reserved.
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
};

export default SettingsAndConfiguration;