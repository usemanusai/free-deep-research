import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';
import Input from '../../../components/ui/Input';
import { Checkbox } from '../../../components/ui/Checkbox';

const AccountSettings = () => {
  const [profileData, setProfileData] = useState({
    fullName: "Dr. Sarah Mitchell",
    email: "sarah.mitchell@research.edu",
    organization: "Stanford Research Institute",
    title: "Senior Research Analyst",
    bio: "Specialized in computational linguistics and AI-driven research methodologies with 8+ years of experience in academic research.",
    timezone: "America/Los_Angeles",
    language: "English (US)"
  });

  const [passwordData, setPasswordData] = useState({
    currentPassword: "",
    newPassword: "",
    confirmPassword: ""
  });

  const [subscriptionData] = useState({
    plan: "Professional",
    status: "Active",
    nextBilling: "2025-08-18",
    usage: {
      researches: 47,
      limit: 100,
      storage: "2.3 GB",
      storageLimit: "10 GB"
    }
  });

  const [twoFactorEnabled, setTwoFactorEnabled] = useState(true);
  const [emailNotifications, setEmailNotifications] = useState(true);

  const handleProfileUpdate = (field, value) => {
    setProfileData(prev => ({
      ...prev,
      [field]: value
    }));
  };

  const handlePasswordChange = (field, value) => {
    setPasswordData(prev => ({
      ...prev,
      [field]: value
    }));
  };

  const handleSaveProfile = () => {
    // Mock save functionality
    console.log('Profile saved:', profileData);
  };

  const handleChangePassword = () => {
    // Mock password change functionality
    console.log('Password change requested');
  };

  return (
    <div className="space-y-8">
      {/* Profile Information */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
            <Icon name="User" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Profile Information
            </h3>
            <p className="text-sm text-muted-foreground">
              Update your personal details and preferences
            </p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Input
            label="Full Name"
            type="text"
            value={profileData.fullName}
            onChange={(e) => handleProfileUpdate('fullName', e.target.value)}
            required
          />
          
          <Input
            label="Email Address"
            type="email"
            value={profileData.email}
            onChange={(e) => handleProfileUpdate('email', e.target.value)}
            required
          />
          
          <Input
            label="Organization"
            type="text"
            value={profileData.organization}
            onChange={(e) => handleProfileUpdate('organization', e.target.value)}
          />
          
          <Input
            label="Job Title"
            type="text"
            value={profileData.title}
            onChange={(e) => handleProfileUpdate('title', e.target.value)}
          />
        </div>

        <div className="mt-6">
          <label className="block text-sm font-medium text-foreground mb-2">
            Bio
          </label>
          <textarea
            className="w-full p-3 bg-input border border-border rounded-lg text-foreground placeholder-muted-foreground resize-none"
            rows={4}
            value={profileData.bio}
            onChange={(e) => handleProfileUpdate('bio', e.target.value)}
            placeholder="Tell us about your research background..."
          />
        </div>

        <div className="flex justify-end mt-6">
          <Button
            variant="default"
            iconName="Save"
            onClick={handleSaveProfile}
          >
            Save Changes
          </Button>
        </div>
      </div>

      {/* Security Settings */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-warning rounded-lg flex items-center justify-center">
            <Icon name="Shield" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Security Settings
            </h3>
            <p className="text-sm text-muted-foreground">
              Manage your account security and authentication
            </p>
          </div>
        </div>

        <div className="space-y-6">
          {/* Password Change */}
          <div className="space-y-4">
            <h4 className="font-medium text-foreground">Change Password</h4>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <Input
                label="Current Password"
                type="password"
                value={passwordData.currentPassword}
                onChange={(e) => handlePasswordChange('currentPassword', e.target.value)}
                placeholder="Enter current password"
              />
              
              <Input
                label="New Password"
                type="password"
                value={passwordData.newPassword}
                onChange={(e) => handlePasswordChange('newPassword', e.target.value)}
                placeholder="Enter new password"
              />
              
              <Input
                label="Confirm Password"
                type="password"
                value={passwordData.confirmPassword}
                onChange={(e) => handlePasswordChange('confirmPassword', e.target.value)}
                placeholder="Confirm new password"
              />
            </div>
            <Button
              variant="outline"
              iconName="Key"
              onClick={handleChangePassword}
            >
              Update Password
            </Button>
          </div>

          {/* Two-Factor Authentication */}
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Two-Factor Authentication</h4>
              <p className="text-sm text-muted-foreground">
                Add an extra layer of security to your account
              </p>
            </div>
            <div className="flex items-center space-x-3">
              <span className={`text-sm ${twoFactorEnabled ? 'text-success' : 'text-muted-foreground'}`}>
                {twoFactorEnabled ? 'Enabled' : 'Disabled'}
              </span>
              <Checkbox
                checked={twoFactorEnabled}
                onChange={(e) => setTwoFactorEnabled(e.target.checked)}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Subscription Details */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-success rounded-lg flex items-center justify-center">
            <Icon name="CreditCard" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Subscription & Billing
            </h3>
            <p className="text-sm text-muted-foreground">
              Manage your subscription and usage details
            </p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <span className="text-sm text-muted-foreground">Current Plan</span>
              <span className="font-medium text-foreground">{subscriptionData.plan}</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-sm text-muted-foreground">Status</span>
              <span className="px-2 py-1 bg-success/10 text-success text-xs rounded-full">
                {subscriptionData.status}
              </span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-sm text-muted-foreground">Next Billing</span>
              <span className="font-medium text-foreground">{subscriptionData.nextBilling}</span>
            </div>
          </div>

          <div className="space-y-4">
            <div>
              <div className="flex justify-between items-center mb-2">
                <span className="text-sm text-muted-foreground">Research Usage</span>
                <span className="text-sm font-medium text-foreground">
                  {subscriptionData.usage.researches}/{subscriptionData.usage.limit}
                </span>
              </div>
              <div className="w-full bg-muted/20 rounded-full h-2">
                <div 
                  className="bg-primary h-2 rounded-full"
                  style={{ width: `${(subscriptionData.usage.researches / subscriptionData.usage.limit) * 100}%` }}
                />
              </div>
            </div>
            
            <div>
              <div className="flex justify-between items-center mb-2">
                <span className="text-sm text-muted-foreground">Storage Used</span>
                <span className="text-sm font-medium text-foreground">
                  {subscriptionData.usage.storage} / {subscriptionData.usage.storageLimit}
                </span>
              </div>
              <div className="w-full bg-muted/20 rounded-full h-2">
                <div 
                  className="bg-accent h-2 rounded-full"
                  style={{ width: '23%' }}
                />
              </div>
            </div>
          </div>
        </div>

        <div className="flex space-x-4 mt-6">
          <Button variant="outline" iconName="CreditCard">
            Update Payment Method
          </Button>
          <Button variant="ghost" iconName="Download">
            Download Invoice
          </Button>
        </div>
      </div>
    </div>
  );
};

export default AccountSettings;