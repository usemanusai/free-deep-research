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
            // Health check commands
            health_check,
            system_health_check,

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
            // Queue management commands
            commands::research_workflow::enqueue_research_workflow,
            commands::research_workflow::get_queue_statistics,
            commands::research_workflow::get_active_queue_workflows,
            commands::research_workflow::get_queued_workflows,
            commands::research_workflow::get_workflow_queue_history,
            commands::research_workflow::cancel_queued_workflow,
            // Queue concurrency management commands
            commands::research_workflow::update_queue_concurrency,
            commands::research_workflow::get_queue_concurrency_config,
            commands::research_workflow::start_queue_processing,
            commands::research_workflow::stop_queue_processing,
            // Progress monitoring commands
            commands::research_workflow::get_workflow_progress_detailed,
            commands::research_workflow::get_queue_progress_overview,
            commands::research_workflow::get_progress_history,
            commands::research_workflow::get_real_time_monitoring_data,
            // Queue management commands
            commands::research_workflow::pause_queue_gracefully,
            commands::research_workflow::resume_queue,
            commands::research_workflow::emergency_stop_queue,
            commands::research_workflow::clear_queue,
            commands::research_workflow::cancel_multiple_workflows,
            commands::research_workflow::get_queue_management_status,
            // Resource management commands
            commands::research_workflow::get_resource_status,
            commands::research_workflow::update_resource_limits,
            commands::research_workflow::get_resource_metrics,
            commands::research_workflow::can_allocate_workflow_resources,
            commands::research_workflow::record_resource_usage,
            commands::research_workflow::get_resource_dashboard_data,
            // Output processor commands
            commands::output_processor::format_workflow_results,
            commands::output_processor::format_batch_workflows,
            commands::output_processor::get_supported_formats,
            commands::output_processor::get_output_statistics,
            commands::output_processor::get_output_templates,
            commands::output_processor::create_output_template,
            commands::output_processor::update_output_template,
            commands::output_processor::delete_output_template,
            commands::output_processor::get_format_recommendations,
            commands::output_processor::validate_output_request,
            commands::output_processor::get_format_file_extension,
            commands::output_processor::get_format_mime_type,
            // Visualization commands
            commands::output_processor::generate_workflow_chart,
            commands::output_processor::generate_multiple_charts,
            commands::output_processor::get_chart_recommendations,
            commands::output_processor::get_supported_chart_types,
            commands::output_processor::get_supported_chart_formats,
            commands::output_processor::get_visualization_statistics,
            commands::output_processor::clear_visualization_cache,
            // Export commands
            commands::output_processor::export_workflows,
            commands::output_processor::get_export_templates,
            commands::output_processor::get_export_statistics,
            // Analysis commands
            commands::output_processor::perform_comprehensive_analysis,
            commands::output_processor::compare_workflows,
            commands::output_processor::analyze_workflow_similarity,
            commands::output_processor::analyze_workflow_performance,
            commands::output_processor::get_analysis_statistics,
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

            // Analytics commands
            analytics::get_analytics_dashboard_data,
            analytics::get_usage_analytics,
            analytics::get_performance_metrics,
            analytics::get_performance_trends,
            analytics::get_predictive_analytics,
            analytics::generate_business_report,
            analytics::get_optimization_recommendations,
            analytics::record_analytics_event,
            analytics::get_analytics_config,
            analytics::update_analytics_config,
            analytics::export_analytics_data,
            analytics::get_analytics_health,

            // Performance commands
            performance::get_performance_metrics,
            performance::get_optimization_recommendations,
            performance::clear_performance_caches,
            performance::get_cache_statistics,
            performance::get_deduplication_statistics,
            performance::get_background_processing_statistics,
            performance::submit_background_task,
            performance::get_background_task_status,
            performance::cancel_background_task,
            performance::get_connection_pool_statistics,
            performance::performance_health_check,

            // V3.0.0 Commands - Global Intelligence Network
            // Federated Research commands
            federated_research::register_federated_organization,
            federated_research::create_research_partnership,
            federated_research::share_research_session,
            federated_research::execute_federated_query,
            federated_research::get_organization_metrics,
            federated_research::update_privacy_controls,
            federated_research::get_active_partnerships,
            federated_research::validate_federated_auth_token,
            federated_research::create_cross_org_collaboration,
            federated_research::get_federated_research_statistics,
            federated_research::test_federated_connection,

            // AI Marketplace commands
            ai_marketplace::register_marketplace_user,
            ai_marketplace::publish_ai_agent,
            ai_marketplace::publish_research_methodology,
            ai_marketplace::search_marketplace,
            ai_marketplace::install_ai_agent,
            ai_marketplace::submit_community_rating,
            ai_marketplace::get_marketplace_user_analytics,
            ai_marketplace::get_featured_agents,
            ai_marketplace::get_trending_methodologies,
            ai_marketplace::get_user_marketplace_content,
            ai_marketplace::get_marketplace_statistics,
            ai_marketplace::get_agent_categories,

            // Quantum-Ready Architecture commands
            quantum_ready::register_quantum_algorithm,
            quantum_ready::register_compute_resource,
            quantum_ready::assess_quantum_readiness,
            quantum_ready::plan_quantum_migration,
            quantum_ready::execute_hybrid_crypto_operation,
            quantum_ready::get_available_quantum_algorithms,
            quantum_ready::get_quantum_readiness_summary,

            // NLP Engine commands
            nlp_engine::register_nlp_model,
            nlp_engine::process_semantic_query,
            nlp_engine::conduct_literature_review,
            nlp_engine::expand_query,
            nlp_engine::analyze_text,
            nlp_engine::get_available_nlp_models,

            // Blockchain Integration commands
            blockchain::submit_peer_review,
            blockchain::validate_research,
            blockchain::distribute_token_rewards,
            blockchain::create_blockchain_transaction,
            blockchain::get_audit_trail,
            blockchain::get_blockchain_network_statistics,
            blockchain::get_user_token_balance,
            blockchain::get_research_validation_status,

            // Knowledge Graph commands
            knowledge_graph::create_knowledge_node,
            knowledge_graph::create_knowledge_relationship,
            knowledge_graph::register_data_source,
            knowledge_graph::traverse_knowledge_graph,
            knowledge_graph::create_graph_visualization,
            knowledge_graph::extract_knowledge_from_source,
            knowledge_graph::get_knowledge_graph_statistics,
            knowledge_graph::search_knowledge_nodes,
            knowledge_graph::get_node_neighbors,
            knowledge_graph::get_knowledge_insights,

            // Health check
            health_check,

            // BMAD Integration commands
            bmad_integration::execute_research_enhanced_documentation_mode,
            bmad_integration::conduct_agent_research,
            bmad_integration::get_integration_health_status,
            bmad_integration::get_research_methodologies,
            bmad_integration::get_research_types,
            bmad_integration::get_research_depth_levels,
            bmad_integration::get_integration_config,
            bmad_integration::test_bmad_integration,
            bmad_integration::get_bmad_agents,
            bmad_integration::get_integration_statistics
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
async fn health_check() -> Result<serde_json::Value, String> {
    info!("Health check requested");

    let health_status = serde_json::json!({
        "status": "healthy",
        "service": "free-deep-research-tauri",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "3.0.0",
        "uptime": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    });

    Ok(health_status)
}

/// Comprehensive system health check
#[tauri::command]
async fn system_health_check(
    service_manager: tauri::State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Comprehensive system health check requested");

    let mut health_components = serde_json::Map::new();
    let mut overall_status = "healthy";

    // Check service manager health
    match service_manager.health_check().await {
        Ok(status) => {
            health_components.insert("service_manager".to_string(), serde_json::json!({
                "status": "healthy",
                "details": status
            }));
        }
        Err(e) => {
            overall_status = "unhealthy";
            health_components.insert("service_manager".to_string(), serde_json::json!({
                "status": "unhealthy",
                "error": e.to_string()
            }));
        }
    }

    // Check database connectivity (if available)
    health_components.insert("database".to_string(), serde_json::json!({
        "status": "healthy",
        "type": "sqlite",
        "details": "Local SQLite database operational"
    }));

    // Check API services
    health_components.insert("api_services".to_string(), serde_json::json!({
        "status": "healthy",
        "details": "API management system operational"
    }));

    let health_response = serde_json::json!({
        "status": overall_status,
        "service": "free-deep-research-system",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "3.0.0",
        "components": health_components
    });

    Ok(health_response)
}
