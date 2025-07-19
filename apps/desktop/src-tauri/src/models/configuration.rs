use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// System configuration model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfiguration {
    pub id: Uuid,
    pub backup_interval: u32, // Backup interval in seconds
    pub encryption_enabled: bool,
    pub rate_limit_buffer: u32, // Buffer percentage (0-50)
    pub monitoring_enabled: bool,
    pub log_level: LogLevel,
    pub ui_theme: UiTheme,
    pub auto_start_monitoring: bool,
    pub max_concurrent_research: u32,
    pub data_retention_days: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for SystemConfiguration {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            backup_interval: 30, // 30 seconds
            encryption_enabled: true,
            rate_limit_buffer: 10, // 10% buffer
            monitoring_enabled: true,
            log_level: LogLevel::Info,
            ui_theme: UiTheme::Auto,
            auto_start_monitoring: true,
            max_concurrent_research: 5,
            data_retention_days: 90,
            created_at: now,
            updated_at: now,
        }
    }
}

impl SystemConfiguration {
    /// Create a new system configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update the configuration
    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }
    
    /// Validate configuration values
    pub fn validate(&self) -> Result<(), String> {
        if self.backup_interval == 0 {
            return Err("Backup interval must be greater than 0".to_string());
        }
        
        if self.rate_limit_buffer > 50 {
            return Err("Rate limit buffer must be between 0 and 50".to_string());
        }
        
        if self.max_concurrent_research == 0 {
            return Err("Max concurrent research must be greater than 0".to_string());
        }
        
        if self.data_retention_days == 0 {
            return Err("Data retention days must be greater than 0".to_string());
        }
        
        Ok(())
    }
}

/// Log level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Get the display name for the log level
    pub fn display_name(&self) -> &'static str {
        match self {
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warn => "Warning",
            LogLevel::Error => "Error",
        }
    }
    
    /// Get all available log levels
    pub fn all() -> Vec<LogLevel> {
        vec![
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
        ]
    }
}

/// UI theme enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UiTheme {
    Light,
    Dark,
    Auto,
}

impl UiTheme {
    /// Get the display name for the UI theme
    pub fn display_name(&self) -> &'static str {
        match self {
            UiTheme::Light => "Light",
            UiTheme::Dark => "Dark",
            UiTheme::Auto => "Auto",
        }
    }
    
    /// Get all available UI themes
    pub fn all() -> Vec<UiTheme> {
        vec![UiTheme::Light, UiTheme::Dark, UiTheme::Auto]
    }
}

/// Configuration update request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfigurationRequest {
    pub backup_interval: Option<u32>,
    pub encryption_enabled: Option<bool>,
    pub rate_limit_buffer: Option<u32>,
    pub monitoring_enabled: Option<bool>,
    pub log_level: Option<LogLevel>,
    pub ui_theme: Option<UiTheme>,
    pub auto_start_monitoring: Option<bool>,
    pub max_concurrent_research: Option<u32>,
    pub data_retention_days: Option<u32>,
}

impl UpdateConfigurationRequest {
    /// Apply the update request to a configuration
    pub fn apply_to(&self, config: &mut SystemConfiguration) {
        if let Some(backup_interval) = self.backup_interval {
            config.backup_interval = backup_interval;
        }
        if let Some(encryption_enabled) = self.encryption_enabled {
            config.encryption_enabled = encryption_enabled;
        }
        if let Some(rate_limit_buffer) = self.rate_limit_buffer {
            config.rate_limit_buffer = rate_limit_buffer;
        }
        if let Some(monitoring_enabled) = self.monitoring_enabled {
            config.monitoring_enabled = monitoring_enabled;
        }
        if let Some(log_level) = &self.log_level {
            config.log_level = log_level.clone();
        }
        if let Some(ui_theme) = &self.ui_theme {
            config.ui_theme = ui_theme.clone();
        }
        if let Some(auto_start_monitoring) = self.auto_start_monitoring {
            config.auto_start_monitoring = auto_start_monitoring;
        }
        if let Some(max_concurrent_research) = self.max_concurrent_research {
            config.max_concurrent_research = max_concurrent_research;
        }
        if let Some(data_retention_days) = self.data_retention_days {
            config.data_retention_days = data_retention_days;
        }
        
        config.update();
    }
}
