import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';
import { Checkbox } from '../../../components/ui/Checkbox';

const PrivacySettings = () => {
  const [dataRetentionSettings, setDataRetentionSettings] = useState({
    retentionPeriod: '2-years',
    autoDelete: true,
    backupBeforeDelete: true,
    deleteInactiveProjects: false
  });

  const [sharingSettings, setSharingSettings] = useState({
    profileVisibility: 'private',
    researchVisibility: 'private',
    allowCollaboration: true,
    shareAnalytics: false,
    publicProfile: false
  });

  const [accountVisibility, setAccountVisibility] = useState({
    showOnlineStatus: false,
    showRecentActivity: false,
    allowDiscovery: false,
    indexBySearchEngines: false
  });

  const [dataProcessingConsent, setDataProcessingConsent] = useState({
    analyticsConsent: true,
    marketingConsent: false,
    researchConsent: true,
    thirdPartySharing: false
  });

  const handleDataRetentionChange = (key, value) => {
    setDataRetentionSettings(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleSharingChange = (key, value) => {
    setSharingSettings(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleVisibilityChange = (key, value) => {
    setAccountVisibility(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleConsentChange = (key, value) => {
    setDataProcessingConsent(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleExportData = () => {
    console.log('Exporting user data...');
    // Mock data export functionality
  };

  const handleDeleteAccount = () => {
    console.log('Account deletion requested...');
    // Mock account deletion functionality
  };

  const handleSavePrivacySettings = () => {
    console.log('Privacy settings saved:', {
      dataRetentionSettings,
      sharingSettings,
      accountVisibility,
      dataProcessingConsent
    });
  };

  return (
    <div className="space-y-8">
      {/* Data Retention */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
            <Icon name="Database" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Data Retention
            </h3>
            <p className="text-sm text-muted-foreground">
              Control how long your research data is stored
            </p>
          </div>
        </div>

        <div className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-foreground mb-3">
              Data Retention Period
            </label>
            <div className="space-y-2">
              {[
                { value: '6-months', label: '6 Months', description: 'Data deleted after 6 months of inactivity' },
                { value: '1-year', label: '1 Year', description: 'Data deleted after 1 year of inactivity' },
                { value: '2-years', label: '2 Years', description: 'Data deleted after 2 years of inactivity' },
                { value: 'never', label: 'Never Delete', description: 'Keep data indefinitely until manually deleted' }
              ].map((option) => (
                <label
                  key={option.value}
                  className={`
                    flex items-start space-x-3 p-4 rounded-lg border cursor-pointer transition-all duration-200
                    ${dataRetentionSettings.retentionPeriod === option.value
                      ? 'bg-primary/10 border-primary/20' :'bg-card border-border hover:bg-muted/50'
                    }
                  `}
                >
                  <input
                    type="radio"
                    name="retentionPeriod"
                    value={option.value}
                    checked={dataRetentionSettings.retentionPeriod === option.value}
                    onChange={(e) => handleDataRetentionChange('retentionPeriod', e.target.value)}
                    className="mt-1"
                  />
                  <div>
                    <div className="font-medium text-foreground">{option.label}</div>
                    <div className="text-sm text-muted-foreground">{option.description}</div>
                  </div>
                </label>
              ))}
            </div>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
              <div>
                <h4 className="font-medium text-foreground">Auto-Delete Inactive Data</h4>
                <p className="text-sm text-muted-foreground">
                  Automatically delete data based on retention period
                </p>
              </div>
              <Checkbox
                checked={dataRetentionSettings.autoDelete}
                onChange={(e) => handleDataRetentionChange('autoDelete', e.target.checked)}
              />
            </div>

            <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
              <div>
                <h4 className="font-medium text-foreground">Backup Before Delete</h4>
                <p className="text-sm text-muted-foreground">
                  Create backup before automatically deleting data
                </p>
              </div>
              <Checkbox
                checked={dataRetentionSettings.backupBeforeDelete}
                onChange={(e) => handleDataRetentionChange('backupBeforeDelete', e.target.checked)}
              />
            </div>

            <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
              <div>
                <h4 className="font-medium text-foreground">Delete Inactive Projects</h4>
                <p className="text-sm text-muted-foreground">
                  Remove projects that haven't been accessed recently
                </p>
              </div>
              <Checkbox
                checked={dataRetentionSettings.deleteInactiveProjects}
                onChange={(e) => handleDataRetentionChange('deleteInactiveProjects', e.target.checked)}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Sharing Permissions */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-accent rounded-lg flex items-center justify-center">
            <Icon name="Share2" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Sharing Permissions
            </h3>
            <p className="text-sm text-muted-foreground">
              Control who can see and access your research
            </p>
          </div>
        </div>

        <div className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-foreground mb-3">
              Profile Visibility
            </label>
            <select
              className="w-full p-3 bg-input border border-border rounded-lg text-foreground"
              value={sharingSettings.profileVisibility}
              onChange={(e) => handleSharingChange('profileVisibility', e.target.value)}
            >
              <option value="private">Private - Only visible to you</option>
              <option value="team">Team - Visible to team members only</option>
              <option value="organization">Organization - Visible within organization</option>
              <option value="public">Public - Visible to everyone</option>
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-foreground mb-3">
              Research Visibility
            </label>
            <select
              className="w-full p-3 bg-input border border-border rounded-lg text-foreground"
              value={sharingSettings.researchVisibility}
              onChange={(e) => handleSharingChange('researchVisibility', e.target.value)}
            >
              <option value="private">Private - Only accessible by you</option>
              <option value="shared">Shared - Accessible by invited collaborators</option>
              <option value="team">Team - Accessible by team members</option>
              <option value="public">Public - Accessible by anyone with link</option>
            </select>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
              <div>
                <h4 className="font-medium text-foreground">Allow Collaboration</h4>
                <p className="text-sm text-muted-foreground">
                  Enable others to collaborate on your research projects
                </p>
              </div>
              <Checkbox
                checked={sharingSettings.allowCollaboration}
                onChange={(e) => handleSharingChange('allowCollaboration', e.target.checked)}
              />
            </div>

            <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
              <div>
                <h4 className="font-medium text-foreground">Share Analytics</h4>
                <p className="text-sm text-muted-foreground">
                  Share anonymized usage analytics for research improvement
                </p>
              </div>
              <Checkbox
                checked={sharingSettings.shareAnalytics}
                onChange={(e) => handleSharingChange('shareAnalytics', e.target.checked)}
              />
            </div>

            <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
              <div>
                <h4 className="font-medium text-foreground">Public Profile</h4>
                <p className="text-sm text-muted-foreground">
                  Make your profile discoverable in public directories
                </p>
              </div>
              <Checkbox
                checked={sharingSettings.publicProfile}
                onChange={(e) => handleSharingChange('publicProfile', e.target.checked)}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Account Visibility */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-secondary rounded-lg flex items-center justify-center">
            <Icon name="Eye" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Account Visibility
            </h3>
            <p className="text-sm text-muted-foreground">
              Control what information others can see about your account
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Show Online Status</h4>
              <p className="text-sm text-muted-foreground">
                Display when you're online to other users
              </p>
            </div>
            <Checkbox
              checked={accountVisibility.showOnlineStatus}
              onChange={(e) => handleVisibilityChange('showOnlineStatus', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Show Recent Activity</h4>
              <p className="text-sm text-muted-foreground">
                Display your recent research activity to collaborators
              </p>
            </div>
            <Checkbox
              checked={accountVisibility.showRecentActivity}
              onChange={(e) => handleVisibilityChange('showRecentActivity', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Allow Discovery</h4>
              <p className="text-sm text-muted-foreground">
                Allow others to find your profile through search
              </p>
            </div>
            <Checkbox
              checked={accountVisibility.allowDiscovery}
              onChange={(e) => handleVisibilityChange('allowDiscovery', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Search Engine Indexing</h4>
              <p className="text-sm text-muted-foreground">
                Allow search engines to index your public content
              </p>
            </div>
            <Checkbox
              checked={accountVisibility.indexBySearchEngines}
              onChange={(e) => handleVisibilityChange('indexBySearchEngines', e.target.checked)}
            />
          </div>
        </div>
      </div>

      {/* Data Processing Consent */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-warning rounded-lg flex items-center justify-center">
            <Icon name="Shield" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Data Processing Consent
            </h3>
            <p className="text-sm text-muted-foreground">
              Manage your consent for different types of data processing
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Analytics Consent</h4>
              <p className="text-sm text-muted-foreground">
                Allow processing of usage data for analytics and improvements
              </p>
            </div>
            <Checkbox
              checked={dataProcessingConsent.analyticsConsent}
              onChange={(e) => handleConsentChange('analyticsConsent', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Marketing Consent</h4>
              <p className="text-sm text-muted-foreground">
                Allow use of your data for marketing and promotional purposes
              </p>
            </div>
            <Checkbox
              checked={dataProcessingConsent.marketingConsent}
              onChange={(e) => handleConsentChange('marketingConsent', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Research Consent</h4>
              <p className="text-sm text-muted-foreground">
                Allow anonymized data to be used for research purposes
              </p>
            </div>
            <Checkbox
              checked={dataProcessingConsent.researchConsent}
              onChange={(e) => handleConsentChange('researchConsent', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Third-Party Sharing</h4>
              <p className="text-sm text-muted-foreground">
                Allow sharing of anonymized data with trusted partners
              </p>
            </div>
            <Checkbox
              checked={dataProcessingConsent.thirdPartySharing}
              onChange={(e) => handleConsentChange('thirdPartySharing', e.target.checked)}
            />
          </div>
        </div>
      </div>

      {/* Data Rights */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-destructive rounded-lg flex items-center justify-center">
            <Icon name="Download" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Data Rights
            </h3>
            <p className="text-sm text-muted-foreground">
              Exercise your rights regarding your personal data
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="p-4 bg-muted/20 rounded-lg">
            <h4 className="font-medium text-foreground mb-2">Export Your Data</h4>
            <p className="text-sm text-muted-foreground mb-4">
              Download a copy of all your research data, settings, and account information in a portable format.
            </p>
            <Button
              variant="outline"
              iconName="Download"
              onClick={handleExportData}
            >
              Request Data Export
            </Button>
          </div>

          <div className="p-4 bg-destructive/10 border border-destructive/20 rounded-lg">
            <h4 className="font-medium text-destructive mb-2">Delete Account</h4>
            <p className="text-sm text-muted-foreground mb-4">
              Permanently delete your account and all associated data. This action cannot be undone.
            </p>
            <Button
              variant="destructive"
              iconName="Trash2"
              onClick={handleDeleteAccount}
            >
              Delete Account
            </Button>
          </div>
        </div>
      </div>

      {/* Save Changes */}
      <div className="flex justify-end">
        <Button
          variant="default"
          iconName="Save"
          onClick={handleSavePrivacySettings}
        >
          Save Privacy Settings
        </Button>
      </div>
    </div>
  );
};

export default PrivacySettings;