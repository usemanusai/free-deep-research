import React from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';

const SettingsSidebar = ({ activeSection, onSectionChange, isMobileMenuOpen, onMobileMenuToggle }) => {
  const settingsCategories = [
    {
      id: 'account',
      title: 'Account',
      icon: 'User',
      description: 'Profile and account management'
    },
    {
      id: 'research',
      title: 'Research Preferences',
      icon: 'Search',
      description: 'Workflow and output settings'
    },
    {
      id: 'integrations',
      title: 'Integrations',
      icon: 'Plug',
      description: 'External services and APIs'
    },
    {
      id: 'notifications',
      title: 'Notifications',
      icon: 'Bell',
      description: 'Alerts and communication'
    },
    {
      id: 'privacy',
      title: 'Privacy & Security',
      icon: 'Shield',
      description: 'Data and privacy controls'
    }
  ];

  return (
    <>
      {/* Mobile Menu Toggle */}
      <div className="lg:hidden mb-6">
        <Button
          variant="outline"
          iconName={isMobileMenuOpen ? "X" : "Menu"}
          onClick={onMobileMenuToggle}
          className="w-full justify-start"
        >
          {isMobileMenuOpen ? 'Close Menu' : 'Settings Menu'}
        </Button>
      </div>

      {/* Sidebar Navigation */}
      <div className={`
        lg:block space-y-2
        ${isMobileMenuOpen ? 'block' : 'hidden'}
      `}>
        <div className="mb-6">
          <h2 className="text-lg font-semibold text-foreground mb-2">
            Settings & Configuration
          </h2>
          <p className="text-sm text-muted-foreground">
            Customize your research workflow and preferences
          </p>
        </div>

        {settingsCategories.map((category) => (
          <button
            key={category.id}
            onClick={() => {
              onSectionChange(category.id);
              onMobileMenuToggle(false);
            }}
            className={`
              w-full text-left p-4 rounded-lg border transition-all duration-200
              ${activeSection === category.id
                ? 'bg-primary/10 border-primary/20 text-primary-foreground'
                : 'bg-card border-border text-card-foreground hover:bg-muted/50'
              }
            `}
          >
            <div className="flex items-start space-x-3">
              <div className={`
                w-8 h-8 rounded-lg flex items-center justify-center mt-0.5
                ${activeSection === category.id ? 'bg-primary' : 'bg-muted'}
              `}>
                <Icon 
                  name={category.icon} 
                  size={16} 
                  color={activeSection === category.id ? 'white' : 'var(--color-muted-foreground)'} 
                />
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="font-medium text-sm mb-1">
                  {category.title}
                </h3>
                <p className="text-xs text-muted-foreground leading-relaxed">
                  {category.description}
                </p>
              </div>
            </div>
          </button>
        ))}

        {/* Quick Actions */}
        <div className="mt-8 pt-6 border-t border-border">
          <div className="space-y-2">
            <Button
              variant="ghost"
              size="sm"
              iconName="Download"
              className="w-full justify-start text-sm"
            >
              Export Settings
            </Button>
            <Button
              variant="ghost"
              size="sm"
              iconName="Upload"
              className="w-full justify-start text-sm"
            >
              Import Settings
            </Button>
            <Button
              variant="ghost"
              size="sm"
              iconName="RotateCcw"
              className="w-full justify-start text-sm"
            >
              Reset to Defaults
            </Button>
          </div>
        </div>
      </div>
    </>
  );
};

export default SettingsSidebar;