import React, { useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import Icon from '../AppIcon';
import Button from './Button';

const Header = () => {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const location = useLocation();
  const navigate = useNavigate();

  const handleSettingsClick = () => {
    navigate('/settings-and-configuration');
    setMobileMenuOpen(false);
  };

  const handleShareClick = () => {
    // Share functionality - could integrate with Web Share API
    if (navigator.share) {
      navigator.share({
        title: 'Deep Research Frontend',
        text: 'Check out my research progress',
        url: window.location.href,
      });
    } else {
      // Fallback to clipboard
      navigator.clipboard.writeText(window.location.href);
    }
  };

  const handleGitHubClick = () => {
    window.open('https://github.com/deep-research-frontend', '_blank');
  };

  const handleLogoClick = () => {
    navigate('/deep-research-workflow-dashboard');
    setMobileMenuOpen(false);
  };

  const toggleMobileMenu = () => {
    setMobileMenuOpen(!mobileMenuOpen);
  };

  const isOnDashboard = location.pathname === '/deep-research-workflow-dashboard';
  const hasResults = location.pathname === '/final-report-generation';

  return (
    <>
      <header className="fixed top-0 left-0 right-0 z-50 bg-background border-b border-border">
        <div className="flex items-center justify-between h-16 px-6">
          {/* Logo Section */}
          <div 
            className="flex items-center space-x-3 cursor-pointer research-hover hover:opacity-80"
            onClick={handleLogoClick}
          >
            <div className="flex items-center justify-center w-8 h-8 bg-primary rounded-lg">
              <Icon name="Search" size={20} color="white" />
            </div>
            <div className="flex flex-col">
              <h1 className="text-lg font-semibold text-foreground leading-none">
                Deep Research
              </h1>
              <span className="text-xs text-muted-foreground font-mono">
                v0.9.18
              </span>
            </div>
          </div>

          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center space-x-2">
            <Button
              variant="ghost"
              size="sm"
              iconName="Github"
              onClick={handleGitHubClick}
              className="research-hover"
            >
              GitHub
            </Button>
            
            <Button
              variant="ghost"
              size="sm"
              iconName="Share2"
              onClick={handleShareClick}
              disabled={!hasResults}
              className="research-hover"
            >
              Share
            </Button>
            
            <Button
              variant="ghost"
              size="sm"
              iconName="Settings"
              onClick={handleSettingsClick}
              className="research-hover"
            >
              Settings
            </Button>
          </div>

          {/* Mobile Menu Button */}
          <div className="md:hidden">
            <Button
              variant="ghost"
              size="icon"
              iconName={mobileMenuOpen ? "X" : "Menu"}
              onClick={toggleMobileMenu}
              className="research-hover"
            />
          </div>
        </div>

        {/* Mobile Menu Panel */}
        {mobileMenuOpen && (
          <div className="md:hidden bg-card border-t border-border animate-fade-in">
            <div className="px-6 py-4 space-y-3">
              <Button
                variant="ghost"
                size="sm"
                iconName="Github"
                onClick={handleGitHubClick}
                fullWidth
                className="justify-start research-hover"
              >
                GitHub Repository
              </Button>
              
              <Button
                variant="ghost"
                size="sm"
                iconName="Share2"
                onClick={handleShareClick}
                disabled={!hasResults}
                fullWidth
                className="justify-start research-hover"
              >
                Share Research
              </Button>
              
              <Button
                variant="ghost"
                size="sm"
                iconName="Settings"
                onClick={handleSettingsClick}
                fullWidth
                className="justify-start research-hover"
              >
                Settings & Configuration
              </Button>
            </div>
          </div>
        )}
      </header>

      {/* Header Spacer */}
      <div className="h-16" />
    </>
  );
};

export default Header;