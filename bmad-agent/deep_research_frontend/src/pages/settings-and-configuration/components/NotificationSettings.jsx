import React, { useState } from 'react';
import Icon from '../../../components/AppIcon';
import Button from '../../../components/ui/Button';
import { Checkbox } from '../../../components/ui/Checkbox';

const NotificationSettings = () => {
  const [emailNotifications, setEmailNotifications] = useState({
    researchComplete: true,
    progressUpdates: true,
    weeklyDigest: false,
    collaborationInvites: true,
    systemUpdates: false,
    securityAlerts: true
  });

  const [pushNotifications, setPushNotifications] = useState({
    browserNotifications: true,
    desktopNotifications: false,
    mobileNotifications: true
  });

  const [collaborationNotifications, setCollaborationNotifications] = useState({
    newComments: true,
    documentShared: true,
    teamInvites: true,
    mentionAlerts: true,
    deadlineReminders: false
  });

  const [notificationSchedule, setNotificationSchedule] = useState({
    quietHours: true,
    quietStart: "22:00",
    quietEnd: "08:00",
    timezone: "America/Los_Angeles",
    weekendNotifications: false
  });

  const handleEmailNotificationChange = (key, value) => {
    setEmailNotifications(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handlePushNotificationChange = (key, value) => {
    setPushNotifications(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleCollaborationNotificationChange = (key, value) => {
    setCollaborationNotifications(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleScheduleChange = (key, value) => {
    setNotificationSchedule(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleSaveNotificationSettings = () => {
    console.log('Notification settings saved:', {
      emailNotifications,
      pushNotifications,
      collaborationNotifications,
      notificationSchedule
    });
  };

  const handleTestNotification = () => {
    console.log('Sending test notification...');
    // Mock test notification
    if (Notification.permission === 'granted') {
      new Notification('Deep Research Test', {
        body: 'This is a test notification from Deep Research Frontend',
        icon: '/favicon.ico'
      });
    }
  };

  return (
    <div className="space-y-8">
      {/* Email Notifications */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
            <Icon name="Mail" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Email Notifications
            </h3>
            <p className="text-sm text-muted-foreground">
              Configure which email notifications you want to receive
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Research Completion</h4>
              <p className="text-sm text-muted-foreground">
                Get notified when your research workflow is complete
              </p>
            </div>
            <Checkbox
              checked={emailNotifications.researchComplete}
              onChange={(e) => handleEmailNotificationChange('researchComplete', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Progress Updates</h4>
              <p className="text-sm text-muted-foreground">
                Receive updates on research progress and milestones
              </p>
            </div>
            <Checkbox
              checked={emailNotifications.progressUpdates}
              onChange={(e) => handleEmailNotificationChange('progressUpdates', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Weekly Digest</h4>
              <p className="text-sm text-muted-foreground">
                Weekly summary of your research activities and achievements
              </p>
            </div>
            <Checkbox
              checked={emailNotifications.weeklyDigest}
              onChange={(e) => handleEmailNotificationChange('weeklyDigest', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Collaboration Invites</h4>
              <p className="text-sm text-muted-foreground">
                Notifications when you're invited to collaborate on research
              </p>
            </div>
            <Checkbox
              checked={emailNotifications.collaborationInvites}
              onChange={(e) => handleEmailNotificationChange('collaborationInvites', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">System Updates</h4>
              <p className="text-sm text-muted-foreground">
                Information about new features and system maintenance
              </p>
            </div>
            <Checkbox
              checked={emailNotifications.systemUpdates}
              onChange={(e) => handleEmailNotificationChange('systemUpdates', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Security Alerts</h4>
              <p className="text-sm text-muted-foreground">
                Important security notifications and login alerts
              </p>
            </div>
            <Checkbox
              checked={emailNotifications.securityAlerts}
              onChange={(e) => handleEmailNotificationChange('securityAlerts', e.target.checked)}
            />
          </div>
        </div>
      </div>

      {/* Push Notifications */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-accent rounded-lg flex items-center justify-center">
            <Icon name="Bell" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Push Notifications
            </h3>
            <p className="text-sm text-muted-foreground">
              Configure real-time notifications for immediate updates
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Browser Notifications</h4>
              <p className="text-sm text-muted-foreground">
                Show notifications in your web browser
              </p>
            </div>
            <Checkbox
              checked={pushNotifications.browserNotifications}
              onChange={(e) => handlePushNotificationChange('browserNotifications', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Desktop Notifications</h4>
              <p className="text-sm text-muted-foreground">
                Show system notifications on your desktop
              </p>
            </div>
            <Checkbox
              checked={pushNotifications.desktopNotifications}
              onChange={(e) => handlePushNotificationChange('desktopNotifications', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Mobile Notifications</h4>
              <p className="text-sm text-muted-foreground">
                Push notifications to your mobile device
              </p>
            </div>
            <Checkbox
              checked={pushNotifications.mobileNotifications}
              onChange={(e) => handlePushNotificationChange('mobileNotifications', e.target.checked)}
            />
          </div>
        </div>

        <div className="mt-6">
          <Button
            variant="outline"
            iconName="TestTube"
            onClick={handleTestNotification}
          >
            Send Test Notification
          </Button>
        </div>
      </div>

      {/* Collaboration Notifications */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-secondary rounded-lg flex items-center justify-center">
            <Icon name="Users" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Collaboration Notifications
            </h3>
            <p className="text-sm text-muted-foreground">
              Manage notifications for team collaboration activities
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">New Comments</h4>
              <p className="text-sm text-muted-foreground">
                Notifications when someone comments on your research
              </p>
            </div>
            <Checkbox
              checked={collaborationNotifications.newComments}
              onChange={(e) => handleCollaborationNotificationChange('newComments', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Document Shared</h4>
              <p className="text-sm text-muted-foreground">
                Alerts when research documents are shared with you
              </p>
            </div>
            <Checkbox
              checked={collaborationNotifications.documentShared}
              onChange={(e) => handleCollaborationNotificationChange('documentShared', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Team Invites</h4>
              <p className="text-sm text-muted-foreground">
                Notifications for team and project invitations
              </p>
            </div>
            <Checkbox
              checked={collaborationNotifications.teamInvites}
              onChange={(e) => handleCollaborationNotificationChange('teamInvites', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Mention Alerts</h4>
              <p className="text-sm text-muted-foreground">
                Get notified when you're mentioned in discussions
              </p>
            </div>
            <Checkbox
              checked={collaborationNotifications.mentionAlerts}
              onChange={(e) => handleCollaborationNotificationChange('mentionAlerts', e.target.checked)}
            />
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Deadline Reminders</h4>
              <p className="text-sm text-muted-foreground">
                Reminders for project deadlines and milestones
              </p>
            </div>
            <Checkbox
              checked={collaborationNotifications.deadlineReminders}
              onChange={(e) => handleCollaborationNotificationChange('deadlineReminders', e.target.checked)}
            />
          </div>
        </div>
      </div>

      {/* Notification Schedule */}
      <div className="research-card p-6">
        <div className="flex items-center space-x-3 mb-6">
          <div className="w-10 h-10 bg-warning rounded-lg flex items-center justify-center">
            <Icon name="Clock" size={20} color="white" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Notification Schedule
            </h3>
            <p className="text-sm text-muted-foreground">
              Control when you receive notifications
            </p>
          </div>
        </div>

        <div className="space-y-6">
          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Quiet Hours</h4>
              <p className="text-sm text-muted-foreground">
                Disable notifications during specified hours
              </p>
            </div>
            <Checkbox
              checked={notificationSchedule.quietHours}
              onChange={(e) => handleScheduleChange('quietHours', e.target.checked)}
            />
          </div>

          {notificationSchedule.quietHours && (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-foreground mb-2">
                  Quiet Hours Start
                </label>
                <input
                  type="time"
                  className="w-full p-3 bg-input border border-border rounded-lg text-foreground"
                  value={notificationSchedule.quietStart}
                  onChange={(e) => handleScheduleChange('quietStart', e.target.value)}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-foreground mb-2">
                  Quiet Hours End
                </label>
                <input
                  type="time"
                  className="w-full p-3 bg-input border border-border rounded-lg text-foreground"
                  value={notificationSchedule.quietEnd}
                  onChange={(e) => handleScheduleChange('quietEnd', e.target.value)}
                />
              </div>
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-foreground mb-2">
              Timezone
            </label>
            <select
              className="w-full p-3 bg-input border border-border rounded-lg text-foreground"
              value={notificationSchedule.timezone}
              onChange={(e) => handleScheduleChange('timezone', e.target.value)}
            >
              <option value="America/Los_Angeles">Pacific Time (PT)</option>
              <option value="America/Denver">Mountain Time (MT)</option>
              <option value="America/Chicago">Central Time (CT)</option>
              <option value="America/New_York">Eastern Time (ET)</option>
              <option value="Europe/London">Greenwich Mean Time (GMT)</option>
              <option value="Europe/Paris">Central European Time (CET)</option>
              <option value="Asia/Tokyo">Japan Standard Time (JST)</option>
            </select>
          </div>

          <div className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
            <div>
              <h4 className="font-medium text-foreground">Weekend Notifications</h4>
              <p className="text-sm text-muted-foreground">
                Receive notifications on weekends
              </p>
            </div>
            <Checkbox
              checked={notificationSchedule.weekendNotifications}
              onChange={(e) => handleScheduleChange('weekendNotifications', e.target.checked)}
            />
          </div>
        </div>
      </div>

      {/* Save Changes */}
      <div className="flex justify-end">
        <Button
          variant="default"
          iconName="Save"
          onClick={handleSaveNotificationSettings}
        >
          Save Notification Settings
        </Button>
      </div>
    </div>
  );
};

export default NotificationSettings;