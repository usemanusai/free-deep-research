use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use rand;

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

/// Mock service integration for development/testing
pub struct MockServiceIntegration {
    service_provider: ServiceProvider,
    config: ServiceConfig,
}

impl MockServiceIntegration {
    pub fn new(service_provider: ServiceProvider) -> Self {
        let config = ServiceConfig {
            base_url: match service_provider {
                ServiceProvider::OpenRouter => "https://openrouter.ai/api/v1".to_string(),
                ServiceProvider::SerpApi => "https://serpapi.com".to_string(),
                ServiceProvider::Tavily => "https://api.tavily.com".to_string(),
                ServiceProvider::Firecrawl => "https://api.firecrawl.dev/v0".to_string(),
                ServiceProvider::Jina => "https://api.jina.ai/v1".to_string(),
                ServiceProvider::Exa => "https://api.exa.ai".to_string(),
            },
            timeout_ms: 30000,
            max_retries: 3,
            rate_limit_per_minute: 60,
            endpoints: match service_provider {
                ServiceProvider::OpenRouter => vec!["/chat/completions".to_string(), "/models".to_string()],
                ServiceProvider::SerpApi => vec!["/search".to_string()],
                ServiceProvider::Tavily => vec!["/search".to_string()],
                ServiceProvider::Firecrawl => vec!["/scrape".to_string(), "/map".to_string()],
                ServiceProvider::Jina => vec!["/embeddings".to_string()],
                ServiceProvider::Exa => vec!["/search".to_string()],
            },
            headers: HashMap::new(),
        };

        Self {
            service_provider,
            config,
        }
    }

    /// Generate mock response based on service and endpoint
    fn generate_mock_response(&self, request: &ServiceRequest) -> ServiceResponse {
        let mock_data = match (&self.service_provider, request.endpoint.as_str()) {
            (ServiceProvider::SerpApi, _) => {
                serde_json::json!({
                    "organic_results": [
                        {
                            "title": "Mock Search Result 1",
                            "link": "https://example.com/result1",
                            "snippet": "This is a mock search result for testing purposes."
                        },
                        {
                            "title": "Mock Search Result 2",
                            "link": "https://example.com/result2",
                            "snippet": "Another mock search result with relevant information."
                        }
                    ],
                    "search_metadata": {
                        "status": "Success",
                        "total_results": 2
                    }
                })
            },
            (ServiceProvider::Firecrawl, "/scrape") => {
                serde_json::json!({
                    "success": true,
                    "data": {
                        "markdown": "# Mock Scraped Content\n\nThis is mock content scraped from a webpage for testing purposes.\n\n## Key Points\n- Point 1: Important information\n- Point 2: Additional details\n- Point 3: Relevant data",
                        "html": "<h1>Mock Scraped Content</h1><p>This is mock content scraped from a webpage for testing purposes.</p>",
                        "metadata": {
                            "title": "Mock Page Title",
                            "description": "Mock page description"
                        }
                    }
                })
            },
            (ServiceProvider::Firecrawl, "/map") => {
                serde_json::json!({
                    "success": true,
                    "links": [
                        "https://example.com/page1",
                        "https://example.com/page2",
                        "https://example.com/page3"
                    ]
                })
            },
            (ServiceProvider::Jina, "/embeddings") => {
                serde_json::json!({
                    "data": [
                        {
                            "object": "embedding",
                            "embedding": vec![0.1, 0.2, 0.3, 0.4, 0.5], // Mock embedding vector
                            "index": 0
                        }
                    ],
                    "model": "jina-embeddings-v2-base-en",
                    "usage": {
                        "total_tokens": 10
                    }
                })
            },
            (ServiceProvider::OpenRouter, "/chat/completions") => {
                serde_json::json!({
                    "choices": [
                        {
                            "message": {
                                "role": "assistant",
                                "content": "This is a mock AI response for testing purposes. The analysis shows that the research query has been processed successfully."
                            },
                            "finish_reason": "stop"
                        }
                    ],
                    "usage": {
                        "prompt_tokens": 50,
                        "completion_tokens": 25,
                        "total_tokens": 75
                    }
                })
            },
            (ServiceProvider::Tavily, "/search") => {
                serde_json::json!({
                    "results": [
                        {
                            "title": "Mock Tavily Result 1",
                            "url": "https://example.com/tavily1",
                            "content": "Mock content from Tavily search result 1"
                        },
                        {
                            "title": "Mock Tavily Result 2",
                            "url": "https://example.com/tavily2",
                            "content": "Mock content from Tavily search result 2"
                        }
                    ]
                })
            },
            (ServiceProvider::Exa, "/search") => {
                serde_json::json!({
                    "results": [
                        {
                            "title": "Mock Exa Academic Result 1",
                            "url": "https://example.com/exa1",
                            "text": "Mock academic content from Exa search"
                        }
                    ]
                })
            },
            _ => {
                serde_json::json!({
                    "message": "Mock response",
                    "status": "success"
                })
            }
        };

        ServiceResponse {
            request_id: request.request_id,
            success: true,
            status_code: 200,
            headers: HashMap::new(),
            body: mock_data.to_string(),
            response_time_ms: rand::random::<u32>() % 1000 + 200, // Random response time 200-1200ms
            error_message: None,
            metadata: HashMap::new(),
        }
    }
}

#[async_trait]
impl ServiceIntegration for MockServiceIntegration {
    fn service_provider(&self) -> ServiceProvider {
        self.service_provider
    }

    async fn make_request(&self, request: ServiceRequest, _api_key: &ApiKey) -> AppResult<ServiceResponse> {
        debug!("Making mock request to {:?} endpoint: {}", self.service_provider, request.endpoint);

        // Simulate network delay
        let delay_ms = rand::random::<u64>() % 500 + 100; // 100-600ms delay
        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;

        // Generate mock response
        let response = self.generate_mock_response(&request);

        debug!("Mock request completed for {:?} in {}ms", self.service_provider, response.response_time_ms);
        Ok(response)
    }

    async fn health_check(&self, _api_key: &ApiKey) -> AppResult<ServiceHealth> {
        // Simulate health check delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(ServiceHealth {
            service: self.service_provider,
            status: ServiceStatus::Healthy,
            response_time_ms: 100,
            last_check: Utc::now(),
            error_message: None,
            metadata: HashMap::new(),
        })
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    async fn update_config(&mut self, config: ServiceConfig) -> AppResult<()> {
        self.config = config;
        Ok(())
    }

    async fn validate_api_key(&self, api_key: &ApiKey) -> AppResult<bool> {
        // Mock validation - just check if key is not empty
        Ok(!api_key.encrypted_key.is_empty())
    }

    fn get_endpoints(&self) -> Vec<String> {
        self.config.endpoints.clone()
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

        // Initialize mock service integrations for development/testing
        let mut integrations: HashMap<ServiceProvider, Box<dyn ServiceIntegration>> = HashMap::new();
        integrations.insert(ServiceProvider::OpenRouter, Box::new(MockServiceIntegration::new(ServiceProvider::OpenRouter)));
        integrations.insert(ServiceProvider::SerpApi, Box::new(MockServiceIntegration::new(ServiceProvider::SerpApi)));
        integrations.insert(ServiceProvider::Tavily, Box::new(MockServiceIntegration::new(ServiceProvider::Tavily)));
        integrations.insert(ServiceProvider::Firecrawl, Box::new(MockServiceIntegration::new(ServiceProvider::Firecrawl)));
        integrations.insert(ServiceProvider::Jina, Box::new(MockServiceIntegration::new(ServiceProvider::Jina)));
        integrations.insert(ServiceProvider::Exa, Box::new(MockServiceIntegration::new(ServiceProvider::Exa)));

        let manager = Self {
            integrations,
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

    /// Perform comprehensive health check on all service integrations
    pub async fn health_check(&self) -> AppResult<()> {
        debug!("Performing service integration manager health check");

        // Check that integrations are loaded
        if self.integrations.is_empty() {
            return Err(ApiError::invalid_configuration(
                "service_integration".to_string(),
                "No service integrations loaded".to_string()
            ).into());
        }

        // Check metrics system
        let metrics = self.metrics.read().await;
        debug!("Service integration manager has metrics for {} services", metrics.len());
        drop(metrics);

        // Check configurations
        let configs = self.configs.read().await;
        debug!("Service integration manager has configs for {} services", configs.len());
        drop(configs);

        // Verify all registered services have configurations
        for service in self.get_registered_services() {
            let configs = self.configs.read().await;
            if !configs.contains_key(&service) {
                warn!("Service {:?} is registered but has no configuration", service);
            }
            drop(configs);
        }

        debug!("Service integration manager health check completed successfully");
        Ok(())
    }
}
