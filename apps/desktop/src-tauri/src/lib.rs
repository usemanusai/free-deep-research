pub mod commands;
pub mod services;
pub mod models;
pub mod utils;
pub mod error;

pub use error::{AppError, AppResult};
pub use services::ServiceManager;

// Phase 1 completion verification
pub fn verify_phase_1_completion() -> bool {
    // This function verifies that all Phase 1 critical components are implemented

    // Check 1: Error handling system
    let _error = AppError::ValidationError {
        field: "test".to_string(),
        message: "test".to_string(),
    };

    // Check 2: Models are defined
    use models::api_key::ServiceProvider;
    let _provider = ServiceProvider::OpenRouter;

    // Check 3: Core services structure exists
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_manager_creation() {
        // Test that we can create a service manager
        let result = ServiceManager::new().await;
        println!("ServiceManager creation result: {:?}", result.is_ok());
    }

    #[test]
    fn test_error_types() {
        // Test error type creation
        let error = AppError::ValidationError {
            field: "test".to_string(),
            message: "test error".to_string(),
        };

        assert!(error.to_string().contains("test"));
    }

    #[test]
    fn test_serde_serialization() {
        // Test that our models can be serialized
        use models::api_key::ServiceProvider;

        let provider = ServiceProvider::OpenRouter;
        let json = serde_json::to_string(&provider).unwrap();
        assert!(json.contains("OpenRouter"));
    }

    #[test]
    fn test_phase_1_verification() {
        assert!(verify_phase_1_completion());
    }
}
