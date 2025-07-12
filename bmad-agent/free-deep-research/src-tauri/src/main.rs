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
