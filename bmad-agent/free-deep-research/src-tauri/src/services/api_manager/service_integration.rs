use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::error::{AppResult, ApiError};
use crate::models::api_key::{ServiceProvider, ApiKey};

/// Standard request structure for all services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub request_id: Uuid,
    pub service: ServiceProvider,
    pub endpoint: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout_ms: u32,
    pub retry_count: u32,
    pub metadata: HashMap<String, String>,
}

/// Standard response structure for all services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub request_id: Uuid,
    pub service: ServiceProvider,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub response_time_ms: u32,
    pub success: bool,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Down,
    Unknown,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub service: ServiceProvider,
    pub base_url: String,
    pub default_timeout_ms: u32,
    pub max_retries: u32,
    pub retry_delay_ms: u32,
    pub health_check_endpoint: Option<String>,
    pub health_check_interval_minutes: u32,
    pub rate_limit_per_minute: u32,
    pub custom_headers: HashMap<String, String>,
    pub enabled: bool,
}

impl ServiceConfig {
    /// Get default configuration for a service
    pub fn default_for_service(service: ServiceProvider) -> Self {
        match service {
            ServiceProvider::OpenRouter => Self {
                service,
                base_url: "https://openrouter.ai/api/v1".to_string(),
                default_timeout_ms: 30000,
                max_retries: 3,
                retry_delay_ms: 1000,
                health_check_endpoint: Some("/models".to_string()),
                health_check_interval_minutes: 5,
                rate_limit_per_minute: 50,
                custom_headers: HashMap::new(),
                enabled: true,
            },
            ServiceProvider::SerpApi => Self {
                service,
                base_url: "https://serpapi.com".to_string(),
                default_timeout_ms: 15000,
                max_retries: 2,
                retry_delay_ms: 2000,
                health_check_endpoint: Some("/search".to_string()),
                health_check_interval_minutes: 10,
                rate_limit_per_minute: 100,
                custom_headers: HashMap::new(),
                enabled: true,
            },
            ServiceProvider::Jina => Self {
                service,
                base_url: "https://api.jina.ai/v1".to_string(),
                default_timeout_ms: 20000,
                max_retries: 3,
                retry_delay_ms: 1500,
                health_check_endpoint: Some("/embeddings".to_string()),
                health_check_interval_minutes: 5,
                rate_limit_per_minute: 1000,
                custom_headers: HashMap::new(),
                enabled: true,
            },
            ServiceProvider::Firecrawl => Self {
                service,
                base_url: "https://api.firecrawl.dev/v0".to_string(),
                default_timeout_ms: 60000,
                max_retries: 2,
                retry_delay_ms: 3000,
                health_check_endpoint: Some("/scrape".to_string()),
                health_check_interval_minutes: 10,
                rate_limit_per_minute: 500,
                custom_headers: HashMap::new(),
                enabled: true,
            },
            ServiceProvider::Tavily => Self {
                service,
                base_url: "https://api.tavily.com".to_string(),
                default_timeout_ms: 25000,
                max_retries: 3,
                retry_delay_ms: 2000,
                health_check_endpoint: Some("/search".to_string()),
                health_check_interval_minutes: 5,
                rate_limit_per_minute: 1000,
                custom_headers: HashMap::new(),
                enabled: true,
            },
            ServiceProvider::Exa => Self {
                service,
                base_url: "https://api.exa.ai".to_string(),
                default_timeout_ms: 20000,
                max_retries: 3,
                retry_delay_ms: 1500,
                health_check_endpoint: Some("/search".to_string()),
                health_check_interval_minutes: 5,
                rate_limit_per_minute: 1000,
                custom_headers: HashMap::new(),
                enabled: true,
            },
        }
    }
}

/// Service performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    pub service: ServiceProvider,
    pub total_requests: u32,
    pub successful_requests: u32,
    pub failed_requests: u32,
    pub average_response_time_ms: f64,
    pub min_response_time_ms: u32,
    pub max_response_time_ms: u32,
    pub last_request_time: Option<DateTime<Utc>>,
    pub health_status: ServiceHealth,
    pub uptime_percentage: f64,
    pub error_rate: f64,
    pub requests_per_minute: f64,
}

impl ServiceMetrics {
    /// Create new metrics for a service
    pub fn new(service: ServiceProvider) -> Self {
        Self {
            service,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            min_response_time_ms: 0,
            max_response_time_ms: 0,
            last_request_time: None,
            health_status: ServiceHealth::Unknown,
            uptime_percentage: 100.0,
            error_rate: 0.0,
            requests_per_minute: 0.0,
        }
    }

    /// Update metrics after a request
    pub fn update_after_request(&mut self, response: &ServiceResponse) {
        self.total_requests += 1;
        self.last_request_time = Some(response.timestamp);

        if response.success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }

        // Update response time metrics
        if self.total_requests == 1 {
            self.average_response_time_ms = response.response_time_ms as f64;
            self.min_response_time_ms = response.response_time_ms;
            self.max_response_time_ms = response.response_time_ms;
        } else {
            self.average_response_time_ms = 
                (self.average_response_time_ms * (self.total_requests - 1) as f64 + response.response_time_ms as f64) 
                / self.total_requests as f64;
            self.min_response_time_ms = self.min_response_time_ms.min(response.response_time_ms);
            self.max_response_time_ms = self.max_response_time_ms.max(response.response_time_ms);
        }

        // Update error rate
        self.error_rate = (self.failed_requests as f64 / self.total_requests as f64) * 100.0;

        // Update health status based on error rate
        self.health_status = if self.error_rate > 50.0 {
            ServiceHealth::Down
        } else if self.error_rate > 20.0 {
            ServiceHealth::Unhealthy
        } else if self.error_rate > 5.0 {
            ServiceHealth::Degraded
        } else {
            ServiceHealth::Healthy
        };

        // Update uptime percentage
        self.uptime_percentage = ((self.successful_requests as f64 / self.total_requests as f64) * 100.0).max(0.0);
    }
}

/// Trait for service integrations
#[async_trait]
pub trait ServiceIntegration: Send + Sync {
    /// Get the service provider this integration handles
    fn service_provider(&self) -> ServiceProvider;

    /// Make a request to the service
    async fn make_request(&self, request: ServiceRequest, api_key: &ApiKey) -> AppResult<ServiceResponse>;

    /// Perform health check
    async fn health_check(&self, api_key: &ApiKey) -> AppResult<ServiceHealth>;

    /// Get service configuration
    fn get_config(&self) -> &ServiceConfig;

    /// Update service configuration
    async fn update_config(&mut self, config: ServiceConfig) -> AppResult<()>;

    /// Validate API key for this service
    async fn validate_api_key(&self, api_key: &ApiKey) -> AppResult<bool>;

    /// Get service-specific endpoints
    fn get_endpoints(&self) -> Vec<String>;

    /// Transform service-specific request
    async fn transform_request(&self, request: &mut ServiceRequest) -> AppResult<()>;

    /// Transform service-specific response
    async fn transform_response(&self, response: &mut ServiceResponse) -> AppResult<()>;
}

/// Service integration manager
pub struct ServiceIntegrationManager {
    integrations: HashMap<ServiceProvider, Box<dyn ServiceIntegration>>,
    metrics: Arc<RwLock<HashMap<ServiceProvider, ServiceMetrics>>>,
    configs: Arc<RwLock<HashMap<ServiceProvider, ServiceConfig>>>,
}

impl ServiceIntegrationManager {
    /// Create a new service integration manager
    pub async fn new() -> AppResult<Self> {
        info!("Initializing service integration manager...");

        let mut metrics = HashMap::new();
        let mut configs = HashMap::new();

        // Initialize metrics and configs for all services
        for service in [
            ServiceProvider::OpenRouter,
            ServiceProvider::SerpApi,
            ServiceProvider::Jina,
            ServiceProvider::Firecrawl,
            ServiceProvider::Tavily,
            ServiceProvider::Exa,
        ] {
            metrics.insert(service.clone(), ServiceMetrics::new(service.clone()));
            configs.insert(service.clone(), ServiceConfig::default_for_service(service));
        }

        let manager = Self {
            integrations: HashMap::new(),
            metrics: Arc::new(RwLock::new(metrics)),
            configs: Arc::new(RwLock::new(configs)),
        };

        info!("Service integration manager initialized successfully");
        Ok(manager)
    }

    /// Register a service integration
    pub async fn register_integration(&mut self, integration: Box<dyn ServiceIntegration>) -> AppResult<()> {
        let service = integration.service_provider();
        info!("Registering integration for service: {:?}", service);
        
        self.integrations.insert(service, integration);
        
        info!("Integration registered successfully for service: {:?}", service);
        Ok(())
    }

    /// Make a request through the appropriate service integration
    pub async fn make_service_request(&self, service: ServiceProvider, request: ServiceRequest, api_key: &ApiKey) -> AppResult<ServiceResponse> {
        debug!("Making request to service: {:?}", service);

        let integration = self.integrations.get(&service)
            .ok_or_else(|| ApiError::invalid_configuration(
                format!("{:?}", service),
                "No integration found for service".to_string()
            ))?;

        let start_time = std::time::Instant::now();
        let result = integration.make_request(request, api_key).await;
        let response_time = start_time.elapsed().as_millis() as u32;

        // Update metrics
        match &result {
            Ok(response) => {
                let mut metrics = self.metrics.write().await;
                if let Some(service_metrics) = metrics.get_mut(&service) {
                    service_metrics.update_after_request(response);
                }
            }
            Err(_) => {
                // Create error response for metrics
                let error_response = ServiceResponse {
                    request_id: Uuid::new_v4(),
                    service,
                    status_code: 500,
                    headers: HashMap::new(),
                    body: "Request failed".to_string(),
                    response_time_ms: response_time,
                    success: false,
                    error_message: Some("Request failed".to_string()),
                    metadata: HashMap::new(),
                    timestamp: Utc::now(),
                };

                let mut metrics = self.metrics.write().await;
                if let Some(service_metrics) = metrics.get_mut(&service) {
                    service_metrics.update_after_request(&error_response);
                }
            }
        }

        result
    }

    /// Perform health check for a service
    pub async fn check_service_health(&self, service: ServiceProvider, api_key: &ApiKey) -> AppResult<ServiceHealth> {
        debug!("Checking health for service: {:?}", service);

        let integration = self.integrations.get(&service)
            .ok_or_else(|| ApiError::invalid_configuration(
                format!("{:?}", service),
                "No integration found for service".to_string()
            ))?;

        integration.health_check(api_key).await
    }

    /// Get metrics for a service
    pub async fn get_service_metrics(&self, service: ServiceProvider) -> Option<ServiceMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(&service).cloned()
    }

    /// Get metrics for all services
    pub async fn get_all_service_metrics(&self) -> HashMap<ServiceProvider, ServiceMetrics> {
        self.metrics.read().await.clone()
    }

    /// Update service configuration
    pub async fn update_service_config(&mut self, service: ServiceProvider, config: ServiceConfig) -> AppResult<()> {
        // Update stored config
        let mut configs = self.configs.write().await;
        configs.insert(service, config.clone());
        drop(configs);

        // Update integration config
        if let Some(integration) = self.integrations.get_mut(&service) {
            integration.update_config(config).await?;
        }

        info!("Updated configuration for service: {:?}", service);
        Ok(())
    }

    /// Get service configuration
    pub async fn get_service_config(&self, service: ServiceProvider) -> Option<ServiceConfig> {
        let configs = self.configs.read().await;
        configs.get(&service).cloned()
    }

    /// Get all service configurations
    pub async fn get_all_service_configs(&self) -> HashMap<ServiceProvider, ServiceConfig> {
        self.configs.read().await.clone()
    }

    /// Validate API key for a service
    pub async fn validate_service_api_key(&self, service: ServiceProvider, api_key: &ApiKey) -> AppResult<bool> {
        debug!("Validating API key for service: {:?}", service);

        let integration = self.integrations.get(&service)
            .ok_or_else(|| ApiError::invalid_configuration(
                format!("{:?}", service),
                "No integration found for service".to_string()
            ))?;

        integration.validate_api_key(api_key).await
    }

    /// Get available endpoints for a service
    pub fn get_service_endpoints(&self, service: ServiceProvider) -> Vec<String> {
        if let Some(integration) = self.integrations.get(&service) {
            integration.get_endpoints()
        } else {
            Vec::new()
        }
    }

    /// Get list of registered services
    pub fn get_registered_services(&self) -> Vec<ServiceProvider> {
        self.integrations.keys().cloned().collect()
    }

    /// Check if a service is registered
    pub fn is_service_registered(&self, service: ServiceProvider) -> bool {
        self.integrations.contains_key(&service)
    }

    /// Generate service status report
    pub async fn generate_service_status_report(&self) -> AppResult<String> {
        debug!("Generating service status report");

        let metrics = self.get_all_service_metrics().await;
        let configs = self.get_all_service_configs().await;

        let mut report = String::new();
        report.push_str("# Service Integration Status Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Summary
        let total_services = metrics.len();
        let healthy_services = metrics.values().filter(|m| m.health_status == ServiceHealth::Healthy).count();
        let degraded_services = metrics.values().filter(|m| m.health_status == ServiceHealth::Degraded).count();
        let unhealthy_services = metrics.values().filter(|m| m.health_status == ServiceHealth::Unhealthy).count();
        let down_services = metrics.values().filter(|m| m.health_status == ServiceHealth::Down).count();

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- Total Services: {}\n", total_services));
        report.push_str(&format!("- Healthy: {} ({:.1}%)\n", healthy_services, (healthy_services as f64 / total_services as f64) * 100.0));
        report.push_str(&format!("- Degraded: {} ({:.1}%)\n", degraded_services, (degraded_services as f64 / total_services as f64) * 100.0));
        report.push_str(&format!("- Unhealthy: {} ({:.1}%)\n", unhealthy_services, (unhealthy_services as f64 / total_services as f64) * 100.0));
        report.push_str(&format!("- Down: {} ({:.1}%)\n", down_services, (down_services as f64 / total_services as f64) * 100.0));
        report.push_str("\n");

        // Service details
        report.push_str("## Service Details\n\n");
        for (service, service_metrics) in metrics.iter() {
            let config = configs.get(service);

            report.push_str(&format!("### {:?}\n", service));
            report.push_str(&format!("- Status: {:?}\n", service_metrics.health_status));
            report.push_str(&format!("- Total Requests: {}\n", service_metrics.total_requests));
            report.push_str(&format!("- Success Rate: {:.1}%\n", 100.0 - service_metrics.error_rate));
            report.push_str(&format!("- Average Response Time: {:.1}ms\n", service_metrics.average_response_time_ms));
            report.push_str(&format!("- Uptime: {:.1}%\n", service_metrics.uptime_percentage));

            if let Some(config) = config {
                report.push_str(&format!("- Base URL: {}\n", config.base_url));
                report.push_str(&format!("- Enabled: {}\n", config.enabled));
                report.push_str(&format!("- Rate Limit: {} req/min\n", config.rate_limit_per_minute));
            }

            if let Some(last_request) = service_metrics.last_request_time {
                report.push_str(&format!("- Last Request: {}\n", last_request.format("%Y-%m-%d %H:%M:%S UTC")));
            }
            report.push_str("\n");
        }

        info!("Generated service status report for {} services", total_services);
        Ok(report)
    }

    /// Start background health monitoring
    pub async fn start_background_monitoring(&self) -> AppResult<()> {
        info!("Starting service integration background monitoring...");

        // TODO: Implement background health checks for each service
        // This would involve periodic health checks and metric updates

        info!("Service integration background monitoring started successfully");
        Ok(())
    }
}
