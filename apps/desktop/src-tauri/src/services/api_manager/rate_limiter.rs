use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ApiError};
use crate::models::api_key::{ServiceProvider, ResetPeriod};
use crate::services::DataPersistenceService;

/// Rate limit configuration for a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub service: ServiceProvider,
    pub requests_per_period: u32,
    pub period_duration_hours: u32,
    pub warning_threshold_percent: f64,
    pub emergency_threshold_percent: f64,
    pub buffer_zone_percent: f64,
}

impl RateLimitConfig {
    /// Get default configuration for a service
    pub fn default_for_service(service: ServiceProvider) -> Self {
        match service {
            ServiceProvider::OpenRouter => Self {
                service,
                requests_per_period: 50,
                period_duration_hours: 24,
                warning_threshold_percent: 70.0,
                emergency_threshold_percent: 90.0,
                buffer_zone_percent: 5.0,
            },
            ServiceProvider::SerpApi => Self {
                service,
                requests_per_period: 100,
                period_duration_hours: 24 * 30, // Monthly
                warning_threshold_percent: 80.0,
                emergency_threshold_percent: 95.0,
                buffer_zone_percent: 3.0,
            },
            ServiceProvider::Jina => Self {
                service,
                requests_per_period: 1000,
                period_duration_hours: 24 * 30, // Monthly
                warning_threshold_percent: 75.0,
                emergency_threshold_percent: 90.0,
                buffer_zone_percent: 5.0,
            },
            ServiceProvider::Firecrawl => Self {
                service,
                requests_per_period: 500,
                period_duration_hours: 24 * 30, // Monthly
                warning_threshold_percent: 80.0,
                emergency_threshold_percent: 95.0,
                buffer_zone_percent: 3.0,
            },
            ServiceProvider::Tavily => Self {
                service,
                requests_per_period: 1000,
                period_duration_hours: 24 * 30, // Monthly
                warning_threshold_percent: 75.0,
                emergency_threshold_percent: 90.0,
                buffer_zone_percent: 5.0,
            },
            ServiceProvider::Exa => Self {
                service,
                requests_per_period: 1000,
                period_duration_hours: 24 * 30, // Monthly
                warning_threshold_percent: 75.0,
                emergency_threshold_percent: 90.0,
                buffer_zone_percent: 5.0,
            },
        }
    }
}

/// Current usage status for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatus {
    pub current_usage: u32,
    pub limit: u32,
    pub usage_percentage: f64,
    pub remaining_requests: u32,
    pub reset_time: DateTime<Utc>,
    pub time_until_reset: Duration,
    pub status: LimitStatus,
}

/// Rate limit status levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LimitStatus {
    Safe,
    Warning,
    Emergency,
    Exhausted,
    Blocked,
}

/// Rate limit violation alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitAlert {
    pub id: Uuid,
    pub api_key_id: Uuid,
    pub service: ServiceProvider,
    pub alert_type: AlertType,
    pub message: String,
    pub usage_percentage: f64,
    pub current_usage: u32,
    pub limit: u32,
    pub timestamp: DateTime<Utc>,
}

/// Types of rate limit alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    Warning,
    Emergency,
    Exhausted,
    Violation,
    Reset,
}

/// Usage forecast data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageForecast {
    pub api_key_id: Uuid,
    pub service: ServiceProvider,
    pub current_usage: u32,
    pub predicted_usage_24h: u32,
    pub predicted_usage_7d: u32,
    pub predicted_exhaustion_time: Option<DateTime<Utc>>,
    pub confidence_level: f64,
    pub recommendations: Vec<String>,
}

/// Rate limiter service for tracking and preventing limit violations
pub struct RateLimiter {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    configs: Arc<RwLock<HashMap<ServiceProvider, RateLimitConfig>>>,
    alerts: Arc<RwLock<Vec<RateLimitAlert>>>,
    emergency_stop_enabled: Arc<RwLock<bool>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub async fn new(data_persistence: Arc<RwLock<DataPersistenceService>>) -> AppResult<Self> {
        info!("Initializing rate limiter...");

        let mut configs = HashMap::new();
        
        // Initialize default configurations for all services
        for service in [
            ServiceProvider::OpenRouter,
            ServiceProvider::SerpApi,
            ServiceProvider::Jina,
            ServiceProvider::Firecrawl,
            ServiceProvider::Tavily,
            ServiceProvider::Exa,
        ] {
            configs.insert(service.clone(), RateLimitConfig::default_for_service(service));
        }

        let rate_limiter = Self {
            data_persistence,
            configs: Arc::new(RwLock::new(configs)),
            alerts: Arc::new(RwLock::new(Vec::new())),
            emergency_stop_enabled: Arc::new(RwLock::new(false)),
        };

        info!("Rate limiter initialized successfully");
        Ok(rate_limiter)
    }

    /// Check if a request can be made for a specific API key
    pub async fn can_make_request(&self, api_key_id: Uuid) -> AppResult<bool> {
        debug!("Checking if request can be made for API key: {}", api_key_id);

        // Check emergency stop
        let emergency_stop = *self.emergency_stop_enabled.read().await;
        if emergency_stop {
            warn!("Emergency stop is enabled - blocking all requests");
            return Ok(false);
        }

        let usage_status = self.get_usage_status(api_key_id).await?;
        
        match usage_status.status {
            LimitStatus::Safe | LimitStatus::Warning => Ok(true),
            LimitStatus::Emergency => {
                // Allow with caution in emergency status
                let remaining_percentage = 100.0 - usage_status.usage_percentage;
                Ok(remaining_percentage > 1.0) // Allow if more than 1% remaining
            },
            LimitStatus::Exhausted | LimitStatus::Blocked => Ok(false),
        }
    }

    /// Get current usage status for an API key
    pub async fn get_usage_status(&self, api_key_id: Uuid) -> AppResult<UsageStatus> {
        debug!("Getting usage status for API key: {}", api_key_id);

        let data_persistence = self.data_persistence.read().await;
        let api_key = data_persistence.get_api_key_by_id(api_key_id).await?
            .ok_or_else(|| ApiError::key_not_found(api_key_id.to_string()))?;
        drop(data_persistence);

        let configs = self.configs.read().await;
        let config = configs.get(&api_key.service)
            .ok_or_else(|| ApiError::invalid_configuration(
                format!("{:?}", api_key.service),
                "No rate limit configuration found".to_string()
            ))?;

        let current_usage = api_key.usage_count;
        let limit = api_key.rate_limit;
        let usage_percentage = if limit > 0 { (current_usage as f64 / limit as f64) * 100.0 } else { 0.0 };
        let remaining_requests = limit.saturating_sub(current_usage);

        // Calculate reset time based on reset period
        let reset_time = match api_key.reset_period {
            ResetPeriod::Daily => {
                let next_day = (api_key.last_reset + Duration::days(1))
                    .date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc();
                next_day
            },
            ResetPeriod::Monthly => {
                let next_month = if api_key.last_reset.month() == 12 {
                    api_key.last_reset.with_year(api_key.last_reset.year() + 1).unwrap().with_month(1).unwrap()
                } else {
                    api_key.last_reset.with_month(api_key.last_reset.month() + 1).unwrap()
                };
                next_month.with_day(1).unwrap().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap()
            },
        };

        let time_until_reset = reset_time - Utc::now();

        // Determine status based on thresholds
        let status = if current_usage >= limit {
            LimitStatus::Exhausted
        } else if usage_percentage >= config.emergency_threshold_percent {
            LimitStatus::Emergency
        } else if usage_percentage >= config.warning_threshold_percent {
            LimitStatus::Warning
        } else {
            LimitStatus::Safe
        };

        Ok(UsageStatus {
            current_usage,
            limit,
            usage_percentage,
            remaining_requests,
            reset_time,
            time_until_reset,
            status,
        })
    }

    /// Record a request and check for threshold violations
    pub async fn record_request(&self, api_key_id: Uuid, success: bool) -> AppResult<Option<RateLimitAlert>> {
        debug!("Recording request for API key: {}", api_key_id);

        let usage_status = self.get_usage_status(api_key_id).await?;
        let mut alert = None;

        // Check for threshold violations and create alerts
        if usage_status.status == LimitStatus::Exhausted && success {
            alert = Some(self.create_alert(
                api_key_id,
                AlertType::Exhausted,
                format!("API key has reached its rate limit ({}/{})", usage_status.current_usage, usage_status.limit),
                usage_status.usage_percentage,
                usage_status.current_usage,
                usage_status.limit,
            ).await?);
        } else if usage_status.status == LimitStatus::Emergency {
            alert = Some(self.create_alert(
                api_key_id,
                AlertType::Emergency,
                format!("API key is approaching rate limit ({:.1}% used)", usage_status.usage_percentage),
                usage_status.usage_percentage,
                usage_status.current_usage,
                usage_status.limit,
            ).await?);
        } else if usage_status.status == LimitStatus::Warning {
            alert = Some(self.create_alert(
                api_key_id,
                AlertType::Warning,
                format!("API key usage is high ({:.1}% used)", usage_status.usage_percentage),
                usage_status.usage_percentage,
                usage_status.current_usage,
                usage_status.limit,
            ).await?);
        }

        if let Some(ref alert) = alert {
            let mut alerts = self.alerts.write().await;
            alerts.push(alert.clone());

            // Keep only last 1000 alerts
            if alerts.len() > 1000 {
                alerts.drain(0..alerts.len() - 1000);
            }
        }

        Ok(alert)
    }

    /// Create a rate limit alert
    async fn create_alert(
        &self,
        api_key_id: Uuid,
        alert_type: AlertType,
        message: String,
        usage_percentage: f64,
        current_usage: u32,
        limit: u32,
    ) -> AppResult<RateLimitAlert> {
        let data_persistence = self.data_persistence.read().await;
        let api_key = data_persistence.get_api_key_by_id(api_key_id).await?
            .ok_or_else(|| ApiError::key_not_found(api_key_id.to_string()))?;
        drop(data_persistence);

        Ok(RateLimitAlert {
            id: Uuid::new_v4(),
            api_key_id,
            service: api_key.service,
            alert_type,
            message,
            usage_percentage,
            current_usage,
            limit,
            timestamp: Utc::now(),
        })
    }

    /// Enable or disable emergency stop
    pub async fn set_emergency_stop(&self, enabled: bool) -> AppResult<()> {
        let mut emergency_stop = self.emergency_stop_enabled.write().await;
        *emergency_stop = enabled;

        if enabled {
            warn!("Emergency stop ENABLED - all API requests will be blocked");
        } else {
            info!("Emergency stop DISABLED - API requests will resume normal operation");
        }

        Ok(())
    }

    /// Check if emergency stop is enabled
    pub async fn is_emergency_stop_enabled(&self) -> bool {
        *self.emergency_stop_enabled.read().await
    }

    /// Get recent alerts
    pub async fn get_recent_alerts(&self, limit: usize) -> Vec<RateLimitAlert> {
        let alerts = self.alerts.read().await;
        alerts.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Clear old alerts
    pub async fn clear_old_alerts(&self, older_than_hours: u32) -> AppResult<u32> {
        let cutoff_time = Utc::now() - Duration::hours(older_than_hours as i64);
        let mut alerts = self.alerts.write().await;

        let initial_count = alerts.len();
        alerts.retain(|alert| alert.timestamp > cutoff_time);
        let removed_count = initial_count - alerts.len();

        if removed_count > 0 {
            info!("Cleared {} old alerts (older than {} hours)", removed_count, older_than_hours);
        }

        Ok(removed_count as u32)
    }

    /// Update rate limit configuration for a service
    pub async fn update_config(&self, service: ServiceProvider, config: RateLimitConfig) -> AppResult<()> {
        let mut configs = self.configs.write().await;
        configs.insert(service, config);
        info!("Updated rate limit configuration for service: {:?}", service);
        Ok(())
    }

    /// Get rate limit configuration for a service
    pub async fn get_config(&self, service: ServiceProvider) -> AppResult<RateLimitConfig> {
        let configs = self.configs.read().await;
        configs.get(&service)
            .cloned()
            .ok_or_else(|| ApiError::invalid_configuration(
                format!("{:?}", service),
                "No rate limit configuration found".to_string()
            ).into())
    }

    /// Get all rate limit configurations
    pub async fn get_all_configs(&self) -> HashMap<ServiceProvider, RateLimitConfig> {
        self.configs.read().await.clone()
    }

    /// Generate usage forecast for an API key
    pub async fn generate_usage_forecast(&self, api_key_id: Uuid) -> AppResult<UsageForecast> {
        debug!("Generating usage forecast for API key: {}", api_key_id);

        let data_persistence = self.data_persistence.read().await;
        let api_key = data_persistence.get_api_key_by_id(api_key_id).await?
            .ok_or_else(|| ApiError::key_not_found(api_key_id.to_string()))?;

        // Get usage statistics for the last 30 days
        let usage_stats = data_persistence.get_api_key_usage_stats(api_key_id, 30).await?;
        drop(data_persistence);

        let current_usage = api_key.usage_count;
        let mut recommendations = Vec::new();

        // Simple forecasting based on recent usage patterns
        let (predicted_24h, predicted_7d, confidence) = if usage_stats.is_empty() {
            // No historical data - use conservative estimates
            recommendations.push("No historical data available for accurate forecasting".to_string());
            (current_usage + 5, current_usage + 20, 0.3)
        } else {
            // Calculate average daily usage from recent data
            let total_requests: u32 = usage_stats.iter().map(|(_, requests, _, _, _)| *requests).sum();
            let days_with_data = usage_stats.len() as u32;
            let avg_daily_usage = if days_with_data > 0 { total_requests / days_with_data } else { 0 };

            let predicted_24h = current_usage + avg_daily_usage;
            let predicted_7d = current_usage + (avg_daily_usage * 7);

            // Higher confidence with more data points
            let confidence = (days_with_data as f64 / 30.0).min(0.9);

            // Generate recommendations based on usage patterns
            let usage_percentage = if api_key.rate_limit > 0 {
                (current_usage as f64 / api_key.rate_limit as f64) * 100.0
            } else {
                0.0
            };

            if usage_percentage > 80.0 {
                recommendations.push("Consider adding additional API keys for this service".to_string());
            }

            if avg_daily_usage > (api_key.rate_limit / 10) {
                recommendations.push("High daily usage detected - monitor closely".to_string());
            }

            if predicted_7d > api_key.rate_limit {
                recommendations.push("Predicted to exceed rate limit within 7 days".to_string());
            }

            (predicted_24h, predicted_7d, confidence)
        };

        // Calculate predicted exhaustion time
        let predicted_exhaustion_time = if predicted_24h >= api_key.rate_limit {
            Some(Utc::now() + Duration::hours(24))
        } else if predicted_7d >= api_key.rate_limit {
            // Estimate based on current usage rate
            let remaining = api_key.rate_limit.saturating_sub(current_usage);
            let daily_rate = (predicted_24h - current_usage).max(1);
            let days_until_exhaustion = remaining / daily_rate;
            Some(Utc::now() + Duration::days(days_until_exhaustion as i64))
        } else {
            None
        };

        Ok(UsageForecast {
            api_key_id,
            service: api_key.service,
            current_usage,
            predicted_usage_24h: predicted_24h,
            predicted_usage_7d: predicted_7d,
            predicted_exhaustion_time,
            confidence_level: confidence,
            recommendations,
        })
    }

    /// Get usage analytics for all API keys
    pub async fn get_usage_analytics(&self) -> AppResult<Vec<(Uuid, UsageStatus, UsageForecast)>> {
        debug!("Generating usage analytics for all API keys");

        let data_persistence = self.data_persistence.read().await;
        let api_keys = data_persistence.get_all_api_keys().await?;
        drop(data_persistence);

        let mut analytics = Vec::new();

        for api_key in api_keys {
            let usage_status = self.get_usage_status(api_key.id).await?;
            let forecast = self.generate_usage_forecast(api_key.id).await?;
            analytics.push((api_key.id, usage_status, forecast));
        }

        debug!("Generated analytics for {} API keys", analytics.len());
        Ok(analytics)
    }

    /// Check all API keys for threshold violations
    pub async fn check_all_thresholds(&self) -> AppResult<Vec<RateLimitAlert>> {
        debug!("Checking all API keys for threshold violations");

        let data_persistence = self.data_persistence.read().await;
        let api_keys = data_persistence.get_all_api_keys().await?;
        drop(data_persistence);

        let mut new_alerts = Vec::new();

        for api_key in api_keys {
            let usage_status = self.get_usage_status(api_key.id).await?;

            // Only create alerts for warning and above
            if matches!(usage_status.status, LimitStatus::Warning | LimitStatus::Emergency | LimitStatus::Exhausted) {
                if let Some(alert) = self.record_request(api_key.id, false).await? {
                    new_alerts.push(alert);
                }
            }
        }

        if !new_alerts.is_empty() {
            info!("Generated {} threshold violation alerts", new_alerts.len());
        }

        Ok(new_alerts)
    }

    /// Generate automated usage report
    pub async fn generate_usage_report(&self) -> AppResult<String> {
        debug!("Generating automated usage report");

        let analytics = self.get_usage_analytics().await?;
        let recent_alerts = self.get_recent_alerts(50).await;

        let mut report = String::new();
        report.push_str("# API Usage Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Summary statistics
        let total_keys = analytics.len();
        let safe_keys = analytics.iter().filter(|(_, status, _)| status.status == LimitStatus::Safe).count();
        let warning_keys = analytics.iter().filter(|(_, status, _)| status.status == LimitStatus::Warning).count();
        let emergency_keys = analytics.iter().filter(|(_, status, _)| status.status == LimitStatus::Emergency).count();
        let exhausted_keys = analytics.iter().filter(|(_, status, _)| status.status == LimitStatus::Exhausted).count();

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- Total API Keys: {}\n", total_keys));
        report.push_str(&format!("- Safe: {} ({:.1}%)\n", safe_keys, (safe_keys as f64 / total_keys as f64) * 100.0));
        report.push_str(&format!("- Warning: {} ({:.1}%)\n", warning_keys, (warning_keys as f64 / total_keys as f64) * 100.0));
        report.push_str(&format!("- Emergency: {} ({:.1}%)\n", emergency_keys, (emergency_keys as f64 / total_keys as f64) * 100.0));
        report.push_str(&format!("- Exhausted: {} ({:.1}%)\n", exhausted_keys, (exhausted_keys as f64 / total_keys as f64) * 100.0));
        report.push_str("\n");

        // Recent alerts
        report.push_str("## Recent Alerts\n\n");
        if recent_alerts.is_empty() {
            report.push_str("No recent alerts.\n\n");
        } else {
            for alert in recent_alerts.iter().take(10) {
                report.push_str(&format!("- **{:?}**: {} ({:.1}% usage)\n",
                    alert.alert_type, alert.message, alert.usage_percentage));
            }
            report.push_str("\n");
        }

        // High usage keys
        report.push_str("## High Usage Keys\n\n");
        let mut high_usage: Vec<_> = analytics.iter()
            .filter(|(_, status, _)| status.usage_percentage > 50.0)
            .collect();
        high_usage.sort_by(|a, b| b.1.usage_percentage.partial_cmp(&a.1.usage_percentage).unwrap());

        if high_usage.is_empty() {
            report.push_str("No high usage keys detected.\n\n");
        } else {
            for (key_id, status, forecast) in high_usage.iter().take(10) {
                report.push_str(&format!("- Key {}: {:.1}% usage ({}/{}) - Service: {:?}\n",
                    key_id, status.usage_percentage, status.current_usage, status.limit, forecast.service));
            }
            report.push_str("\n");
        }

        info!("Generated usage report with {} keys and {} alerts", total_keys, recent_alerts.len());
        Ok(report)
    }

    /// Perform health check on the rate limiter
    pub async fn health_check(&self) -> AppResult<()> {
        debug!("Performing rate limiter health check");

        // Check data persistence connection
        let data_persistence = self.data_persistence.read().await;
        data_persistence.health_check().await?;
        drop(data_persistence);

        // Check configurations are loaded
        let configs = self.configs.read().await;
        if configs.is_empty() {
            return Err(ApiError::invalid_configuration(
                "rate_limiter".to_string(),
                "No rate limit configurations loaded".to_string()
            ).into());
        }
        debug!("Rate limiter has {} service configurations", configs.len());
        drop(configs);

        // Check alerts system
        let alerts = self.alerts.read().await;
        debug!("Rate limiter has {} alerts in memory", alerts.len());
        drop(alerts);

        // Test basic functionality by checking emergency stop status
        let emergency_stop = self.is_emergency_stop_enabled().await;
        debug!("Emergency stop status: {}", emergency_stop);

        debug!("Rate limiter health check completed successfully");
        Ok(())
    }

    /// Get recent alerts from the rate limiter
    pub async fn get_recent_alerts(&self) -> AppResult<Vec<RateLimitAlert>> {
        debug!("Getting recent alerts from rate limiter");

        let alerts = self.alerts.read().await;

        // Filter alerts from the last hour
        let one_hour_ago = chrono::Utc::now() - chrono::Duration::hours(1);
        let recent_alerts: Vec<RateLimitAlert> = alerts
            .iter()
            .filter(|alert| alert.timestamp > one_hour_ago)
            .cloned()
            .collect();

        debug!("Found {} recent alerts", recent_alerts.len());
        Ok(recent_alerts)
    }
}
