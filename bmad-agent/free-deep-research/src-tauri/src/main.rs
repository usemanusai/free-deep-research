// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tracing::{info, error};
use tracing_subscriber;

mod commands;
mod services;
mod models;
mod utils;
mod error;

use commands::*;
use services::ServiceManager;
use error::AppResult;

/// Initialize the application logging system
fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into())
        )
        .init();
}

/// Initialize the application services
async fn init_services() -> AppResult<ServiceManager> {
    info!("Initializing application services...");
    
    let service_manager = ServiceManager::new().await?;
    
    info!("Application services initialized successfully");
    Ok(service_manager)
}

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();
    info!("Starting Free Deep Research System...");

    // Initialize services
    let service_manager = match init_services().await {
        Ok(sm) => sm,
        Err(e) => {
            error!("Failed to initialize services: {}", e);
            std::process::exit(1);
        }
    };

    // Build and run the Tauri application
    tauri::Builder::default()
        .manage(service_manager)
        .invoke_handler(tauri::generate_handler![
            // API Management commands
            api_management::get_api_keys,
            api_management::add_api_key,
            api_management::update_api_key,
            api_management::delete_api_key,
            api_management::test_api_key,
            api_management::import_api_keys,
            api_management::export_api_keys,
            api_management::import_api_keys_csv,
            api_management::import_api_keys_json,
            api_management::export_api_keys_csv,
            api_management::export_api_keys_json,
            api_management::get_api_key_usage_stats,
            api_management::get_api_keys_with_status,
            // Rate limiting commands
            api_management::can_make_request,
            api_management::get_key_usage_status,
            api_management::record_api_request,
            api_management::set_emergency_stop,
            api_management::is_emergency_stop_enabled,
            api_management::get_recent_alerts,
            api_management::generate_usage_forecast,
            api_management::get_usage_analytics,
            api_management::check_all_thresholds,
            api_management::generate_usage_report,
            // Key rotation commands
            api_management::select_best_key_for_service,
            api_management::record_key_performance,
            api_management::get_key_performance_metrics,
            api_management::get_service_performance_metrics,
            api_management::get_all_performance_metrics,
            api_management::get_rotation_analytics,
            api_management::perform_health_check,
            api_management::reactivate_cooled_down_keys,
            api_management::get_keys_needing_attention,
            api_management::generate_rotation_report,
            // Service integration commands
            api_management::make_service_request,
            api_management::check_service_health,
            api_management::get_service_metrics,
            api_management::get_all_service_metrics,
            api_management::get_service_config,
            api_management::get_all_service_configs,
            api_management::update_service_config,
            api_management::get_service_endpoints,
            api_management::get_registered_services,
            api_management::generate_service_status_report,
            // Research workflow commands
            commands::research_workflow::create_research_workflow,
            commands::research_workflow::start_research_workflow,
            commands::research_workflow::pause_research_workflow,
            commands::research_workflow::resume_research_workflow,
            commands::research_workflow::cancel_research_workflow,
            commands::research_workflow::get_research_workflow,
            commands::research_workflow::get_all_research_workflows,
            commands::research_workflow::get_research_workflows_by_status,
            commands::research_workflow::delete_research_workflow,
            commands::research_workflow::get_workflow_status,
            commands::research_workflow::get_workflow_progress,
            commands::research_workflow::get_workflow_results,
            commands::research_workflow::get_workflow_statistics,
            // Template management commands
            commands::template_management::create_research_template,
            commands::template_management::get_research_template,
            commands::template_management::get_all_research_templates,
            commands::template_management::get_research_templates_by_category,
            commands::template_management::get_featured_research_templates,
            commands::template_management::get_public_research_templates,
            commands::template_management::search_research_templates,
            commands::template_management::update_research_template,
            commands::template_management::delete_research_template,
            commands::template_management::execute_research_template,
            commands::template_management::preview_template_execution,
            commands::template_management::rate_research_template,
            commands::template_management::get_template_metrics,
            commands::template_management::get_all_template_metrics,
            commands::template_management::get_template_recommendations,
            commands::template_management::get_template_statistics,
            
            // Research commands
            research::create_research_workflow,
            research::execute_research,
            research::get_research_status,
            research::cancel_research,
            research::get_research_results,
            
            // Configuration commands
            commands::config::get_configuration,
            commands::config::update_configuration,
            commands::config::reset_configuration,
            
            // Monitoring commands
            monitoring::get_system_metrics,
            monitoring::get_api_usage_stats,
            monitoring::get_service_health,
            monitoring::get_audit_logs,
            
            // Health check
            health_check
        ])
        .setup(|app| {
            info!("Tauri application setup complete");
            
            // Set up application window
            let window = app.get_window("main").unwrap();
            
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Health check command for basic application status
#[tauri::command]
async fn health_check() -> AppResult<String> {
    info!("Health check requested");
    Ok("Free Deep Research System is running".to_string())
}
