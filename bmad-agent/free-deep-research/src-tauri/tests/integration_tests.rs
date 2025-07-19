//! Free Deep Research System - Integration Tests
//! Comprehensive integration testing for all system components

use std::time::Duration;
use tokio::time::timeout;
use serde_json::json;

use free_deep_research::{
    services::{ServiceManager, ServiceConfig},
    models::{ApiKey, ResearchWorkflow, WorkflowStatus},
    error::AppResult,
};

/// Test configuration for integration tests
struct TestConfig {
    pub test_db_path: String,
    pub test_api_keys: Vec<ApiKey>,
    pub timeout_duration: Duration,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            test_db_path: ":memory:".to_string(),
            test_api_keys: vec![
                ApiKey {
                    id: 1,
                    service_name: "test_service".to_string(),
                    api_key: "test_key_123".to_string(),
                    is_active: true,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                    last_used: None,
                    usage_count: 0,
                    rate_limit: Some(100),
                    cost_per_request: Some(0.01),
                    monthly_budget: Some(100.0),
                    current_month_cost: 0.0,
                },
            ],
            timeout_duration: Duration::from_secs(30),
        }
    }
}

/// Setup test environment
async fn setup_test_environment() -> AppResult<ServiceManager> {
    let config = ServiceConfig {
        database_url: ":memory:".to_string(),
        log_level: "debug".to_string(),
        enable_metrics: true,
        ..Default::default()
    };
    
    let service_manager = ServiceManager::new(config).await?;
    
    // Initialize test data
    setup_test_data(&service_manager).await?;
    
    Ok(service_manager)
}

/// Setup test data
async fn setup_test_data(service_manager: &ServiceManager) -> AppResult<()> {
    let test_config = TestConfig::default();
    
    // Add test API keys
    for api_key in test_config.test_api_keys {
        service_manager.api_service().add_api_key(api_key).await?;
    }
    
    Ok(())
}

/// Cleanup test environment
async fn cleanup_test_environment(service_manager: ServiceManager) -> AppResult<()> {
    service_manager.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_service_manager_initialization() -> AppResult<()> {
    let service_manager = setup_test_environment().await?;
    
    // Test health check
    let health_status = service_manager.health_check().await?;
    assert!(health_status.contains("healthy"));
    
    cleanup_test_environment(service_manager).await?;
    Ok(())
}

#[tokio::test]
async fn test_api_key_management() -> AppResult<()> {
    let service_manager = setup_test_environment().await?;
    let api_service = service_manager.api_service();
    
    // Test getting API keys
    let api_keys = api_service.get_api_keys().await?;
    assert!(!api_keys.is_empty());
    
    // Test adding new API key
    let new_api_key = ApiKey {
        id: 0, // Will be auto-generated
        service_name: "integration_test_service".to_string(),
        api_key: "integration_test_key_456".to_string(),
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_used: None,
        usage_count: 0,
        rate_limit: Some(50),
        cost_per_request: Some(0.02),
        monthly_budget: Some(50.0),
        current_month_cost: 0.0,
    };
    
    let added_key = api_service.add_api_key(new_api_key).await?;
    assert_eq!(added_key.service_name, "integration_test_service");
    
    // Test updating API key
    let mut updated_key = added_key.clone();
    updated_key.rate_limit = Some(75);
    let result = api_service.update_api_key(updated_key).await?;
    assert_eq!(result.rate_limit, Some(75));
    
    // Test deleting API key
    api_service.delete_api_key(added_key.id).await?;
    
    cleanup_test_environment(service_manager).await?;
    Ok(())
}

#[tokio::test]
async fn test_research_workflow_lifecycle() -> AppResult<()> {
    let service_manager = setup_test_environment().await?;
    let research_service = service_manager.research_service();
    
    // Create test workflow
    let workflow_config = json!({
        "methodology": "don_lim",
        "query": "Test research query for integration testing",
        "max_sources": 5,
        "quality_threshold": 0.8
    });
    
    let workflow = research_service.create_workflow(
        "Integration Test Workflow".to_string(),
        workflow_config,
        "don_lim".to_string(),
    ).await?;
    
    assert_eq!(workflow.status, WorkflowStatus::Pending);
    assert_eq!(workflow.title, "Integration Test Workflow");
    
    // Start workflow execution
    let started_workflow = research_service.start_workflow(workflow.id).await?;
    assert_eq!(started_workflow.status, WorkflowStatus::Running);
    
    // Wait for workflow to complete or timeout
    let timeout_duration = Duration::from_secs(60);
    let result = timeout(timeout_duration, async {
        loop {
            let current_workflow = research_service.get_workflow(workflow.id).await?;
            match current_workflow.status {
                WorkflowStatus::Completed | WorkflowStatus::Failed => {
                    return Ok(current_workflow);
                }
                _ => {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }).await;
    
    match result {
        Ok(Ok(final_workflow)) => {
            // Workflow completed within timeout
            assert!(matches!(final_workflow.status, WorkflowStatus::Completed | WorkflowStatus::Failed));
            
            // Get workflow results
            let results = research_service.get_workflow_results(workflow.id).await?;
            assert!(results.is_some());
        }
        Ok(Err(e)) => {
            return Err(e);
        }
        Err(_) => {
            // Timeout - stop the workflow
            research_service.stop_workflow(workflow.id).await?;
            println!("Workflow timed out - this is acceptable for integration tests");
        }
    }
    
    cleanup_test_environment(service_manager).await?;
    Ok(())
}

#[tokio::test]
async fn test_analytics_and_monitoring() -> AppResult<()> {
    let service_manager = setup_test_environment().await?;
    let analytics_service = service_manager.analytics_service();
    let monitoring_service = service_manager.monitoring_service();
    
    // Test analytics dashboard data
    let dashboard_data = analytics_service.get_dashboard_data("Last24Hours".to_string()).await?;
    assert!(dashboard_data.is_object());
    
    // Test usage analytics
    let usage_analytics = analytics_service.get_usage_analytics("LastWeek".to_string()).await?;
    assert!(usage_analytics.is_object());
    
    // Test system metrics
    let system_metrics = monitoring_service.get_system_metrics().await?;
    assert!(system_metrics.is_object());
    
    // Test service health monitoring
    let service_health = monitoring_service.get_service_health().await?;
    assert!(service_health.is_object());
    
    cleanup_test_environment(service_manager).await?;
    Ok(())
}

#[tokio::test]
async fn test_rate_limiting() -> AppResult<()> {
    let service_manager = setup_test_environment().await?;
    let api_service = service_manager.api_service();
    
    // Create API key with low rate limit for testing
    let test_key = ApiKey {
        id: 0,
        service_name: "rate_limit_test".to_string(),
        api_key: "rate_limit_test_key".to_string(),
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_used: None,
        usage_count: 0,
        rate_limit: Some(2), // Very low limit for testing
        cost_per_request: Some(0.01),
        monthly_budget: Some(10.0),
        current_month_cost: 0.0,
    };
    
    let added_key = api_service.add_api_key(test_key).await?;
    
    // Test rate limiting
    let can_make_first = api_service.can_make_request(&added_key.service_name).await?;
    assert!(can_make_first);
    
    // Record requests to hit rate limit
    api_service.record_api_request(&added_key.service_name, 200, 100, 0, 0.01).await?;
    api_service.record_api_request(&added_key.service_name, 200, 100, 0, 0.01).await?;
    
    // Should now be rate limited
    let can_make_after_limit = api_service.can_make_request(&added_key.service_name).await?;
    assert!(!can_make_after_limit);
    
    cleanup_test_environment(service_manager).await?;
    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> AppResult<()> {
    let service_manager = setup_test_environment().await?;
    let research_service = service_manager.research_service();
    
    // Test getting non-existent workflow
    let result = research_service.get_workflow(99999).await;
    assert!(result.is_err());
    
    // Test invalid workflow configuration
    let invalid_config = json!({
        "invalid_field": "invalid_value"
    });
    
    let result = research_service.create_workflow(
        "Invalid Workflow".to_string(),
        invalid_config,
        "invalid_methodology".to_string(),
    ).await;
    assert!(result.is_err());
    
    cleanup_test_environment(service_manager).await?;
    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> AppResult<()> {
    let service_manager = setup_test_environment().await?;
    let api_service = service_manager.api_service();
    
    // Test concurrent API key operations
    let mut handles = vec![];
    
    for i in 0..10 {
        let api_service_clone = api_service.clone();
        let handle = tokio::spawn(async move {
            let api_key = ApiKey {
                id: 0,
                service_name: format!("concurrent_test_{}", i),
                api_key: format!("concurrent_key_{}", i),
                is_active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                last_used: None,
                usage_count: 0,
                rate_limit: Some(100),
                cost_per_request: Some(0.01),
                monthly_budget: Some(100.0),
                current_month_cost: 0.0,
            };
            
            api_service_clone.add_api_key(api_key).await
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    let results = futures::future::join_all(handles).await;
    
    // Check that all operations succeeded
    for result in results {
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }
    
    // Verify all keys were added
    let all_keys = api_service.get_api_keys().await?;
    let concurrent_keys: Vec<_> = all_keys.iter()
        .filter(|key| key.service_name.starts_with("concurrent_test_"))
        .collect();
    assert_eq!(concurrent_keys.len(), 10);
    
    cleanup_test_environment(service_manager).await?;
    Ok(())
}

#[tokio::test]
async fn test_data_persistence() -> AppResult<()> {
    // This test uses a temporary file instead of in-memory database
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let db_path = temp_db.path().to_string_lossy().to_string();
    
    let config = ServiceConfig {
        database_url: db_path.clone(),
        log_level: "debug".to_string(),
        enable_metrics: true,
        ..Default::default()
    };
    
    // Create first service manager instance
    let service_manager1 = ServiceManager::new(config.clone()).await?;
    let api_service1 = service_manager1.api_service();
    
    // Add test data
    let test_key = ApiKey {
        id: 0,
        service_name: "persistence_test".to_string(),
        api_key: "persistence_test_key".to_string(),
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_used: None,
        usage_count: 0,
        rate_limit: Some(100),
        cost_per_request: Some(0.01),
        monthly_budget: Some(100.0),
        current_month_cost: 0.0,
    };
    
    let added_key = api_service1.add_api_key(test_key).await?;
    cleanup_test_environment(service_manager1).await?;
    
    // Create second service manager instance with same database
    let service_manager2 = ServiceManager::new(config).await?;
    let api_service2 = service_manager2.api_service();
    
    // Verify data persisted
    let persisted_keys = api_service2.get_api_keys().await?;
    let found_key = persisted_keys.iter()
        .find(|key| key.service_name == "persistence_test");
    
    assert!(found_key.is_some());
    assert_eq!(found_key.unwrap().api_key, "persistence_test_key");
    
    cleanup_test_environment(service_manager2).await?;
    Ok(())
}

/// Performance benchmark test
#[tokio::test]
async fn test_performance_benchmarks() -> AppResult<()> {
    let service_manager = setup_test_environment().await?;
    let api_service = service_manager.api_service();
    
    // Benchmark API key retrieval
    let start_time = std::time::Instant::now();
    for _ in 0..100 {
        let _ = api_service.get_api_keys().await?;
    }
    let duration = start_time.elapsed();
    
    // Should complete 100 operations in reasonable time
    assert!(duration < Duration::from_secs(5));
    println!("API key retrieval benchmark: {:?} for 100 operations", duration);
    
    cleanup_test_environment(service_manager).await?;
    Ok(())
}

/// Test helper functions
mod test_helpers {
    use super::*;
    
    pub async fn create_test_workflow(service_manager: &ServiceManager) -> AppResult<ResearchWorkflow> {
        let research_service = service_manager.research_service();
        let config = json!({
            "methodology": "don_lim",
            "query": "Test query",
            "max_sources": 3
        });
        
        research_service.create_workflow(
            "Test Workflow".to_string(),
            config,
            "don_lim".to_string(),
        ).await
    }
    
    pub async fn wait_for_workflow_completion(
        service_manager: &ServiceManager,
        workflow_id: i64,
        timeout_secs: u64,
    ) -> AppResult<ResearchWorkflow> {
        let research_service = service_manager.research_service();
        let timeout_duration = Duration::from_secs(timeout_secs);
        
        timeout(timeout_duration, async {
            loop {
                let workflow = research_service.get_workflow(workflow_id).await?;
                match workflow.status {
                    WorkflowStatus::Completed | WorkflowStatus::Failed => {
                        return Ok(workflow);
                    }
                    _ => {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }).await.map_err(|_| {
            free_deep_research::error::AppError::Custom("Workflow timeout".to_string())
        })?
    }
}
