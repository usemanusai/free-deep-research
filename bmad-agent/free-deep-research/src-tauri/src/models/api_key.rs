use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Supported API service providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceProvider {
    OpenRouter,
    SerpApi,
    Jina,
    Firecrawl,
}

impl ServiceProvider {
    /// Get the display name for the service
    pub fn display_name(&self) -> &'static str {
        match self {
            ServiceProvider::OpenRouter => "OpenRouter.ai",
            ServiceProvider::SerpApi => "SerpApi",
            ServiceProvider::Jina => "Jina AI",
            ServiceProvider::Firecrawl => "Firecrawl",
        }
    }
    
    /// Get the default rate limit for the service
    pub fn default_rate_limit(&self) -> u32 {
        match self {
            ServiceProvider::OpenRouter => 50,  // 50 messages/day
            ServiceProvider::SerpApi => 100,    // 100 searches/month
            ServiceProvider::Jina => 1000,      // 1000 requests/month
            ServiceProvider::Firecrawl => 500,  // 500 requests/month
        }
    }
    
    /// Get the reset period for the service
    pub fn reset_period(&self) -> ResetPeriod {
        match self {
            ServiceProvider::OpenRouter => ResetPeriod::Daily,
            ServiceProvider::SerpApi => ResetPeriod::Monthly,
            ServiceProvider::Jina => ResetPeriod::Monthly,
            ServiceProvider::Firecrawl => ResetPeriod::Monthly,
        }
    }
}

/// Rate limit reset periods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResetPeriod {
    Daily,
    Monthly,
}

/// API key status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyStatus {
    Active,
    Exhausted,
    Error,
    Disabled,
}

/// API key model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub service: ServiceProvider,
    pub name: String,
    pub encrypted_key: String,
    pub usage_count: u32,
    pub rate_limit: u32,
    pub reset_period: ResetPeriod,
    pub last_used: Option<DateTime<Utc>>,
    pub last_reset: DateTime<Utc>,
    pub status: ApiKeyStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiKey {
    /// Create a new API key
    pub fn new(
        service: ServiceProvider,
        name: String,
        encrypted_key: String,
    ) -> Self {
        let now = Utc::now();
        let rate_limit = service.default_rate_limit();
        let reset_period = service.reset_period();
        
        Self {
            id: Uuid::new_v4(),
            service,
            name,
            encrypted_key,
            usage_count: 0,
            rate_limit,
            reset_period,
            last_used: None,
            last_reset: now,
            status: ApiKeyStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Check if the key is available for use
    pub fn is_available(&self) -> bool {
        matches!(self.status, ApiKeyStatus::Active) && self.usage_count < self.rate_limit
    }
    
    /// Check if the key needs to be reset
    pub fn needs_reset(&self) -> bool {
        let now = Utc::now();
        match self.reset_period {
            ResetPeriod::Daily => {
                now.date_naive() > self.last_reset.date_naive()
            }
            ResetPeriod::Monthly => {
                now.format("%Y-%m").to_string() > self.last_reset.format("%Y-%m").to_string()
            }
        }
    }
    
    /// Reset the usage count
    pub fn reset_usage(&mut self) {
        self.usage_count = 0;
        self.last_reset = Utc::now();
        self.updated_at = Utc::now();
        if matches!(self.status, ApiKeyStatus::Exhausted) {
            self.status = ApiKeyStatus::Active;
        }
    }
    
    /// Increment usage count
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
        self.last_used = Some(Utc::now());
        self.updated_at = Utc::now();
        
        if self.usage_count >= self.rate_limit {
            self.status = ApiKeyStatus::Exhausted;
        }
    }
    
    /// Get usage percentage
    pub fn usage_percentage(&self) -> f64 {
        if self.rate_limit == 0 {
            0.0
        } else {
            (self.usage_count as f64 / self.rate_limit as f64) * 100.0
        }
    }
    
    /// Get remaining requests
    pub fn remaining_requests(&self) -> u32 {
        self.rate_limit.saturating_sub(self.usage_count)
    }
}

/// API key creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub service: ServiceProvider,
    pub name: String,
    pub api_key: String,
    pub rate_limit: Option<u32>,
}

/// API key update request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateApiKeyRequest {
    pub name: Option<String>,
    pub api_key: Option<String>,
    pub rate_limit: Option<u32>,
    pub status: Option<ApiKeyStatus>,
}

/// API key import data
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyImport {
    pub service: ServiceProvider,
    pub name: String,
    pub api_key: String,
    pub rate_limit: Option<u32>,
}

/// API key export data
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyExport {
    pub service: ServiceProvider,
    pub name: String,
    pub rate_limit: u32,
    pub usage_count: u32,
    pub status: ApiKeyStatus,
    pub created_at: DateTime<Utc>,
}

/// API key test result
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyTestResult {
    pub key_id: Uuid,
    pub success: bool,
    pub message: String,
    pub response_time_ms: Option<u64>,
    pub tested_at: DateTime<Utc>,
}
