use tauri::State;
use uuid::Uuid;
use tracing::{info, error};
use std::collections::HashMap;

use crate::error::AppResult;
use crate::services::ServiceManager;

/// V1.2.0 Plugin System Commands

/// Install plugin from marketplace
#[tauri::command]
pub async fn install_plugin(
    plugin_id: String,
    user_id: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<String> {
    info!("Installing plugin: {} for user: {}", plugin_id, user_id);
    
    let plugin_system = service_manager.plugin_system.read().await;
    let plugin_uuid = Uuid::parse_str(&plugin_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid plugin ID".to_string()))?;
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    
    let instance = plugin_system.install_plugin(plugin_uuid, user_uuid).await?;
    Ok(instance.id.to_string())
}

/// Execute plugin with context
#[tauri::command]
pub async fn execute_plugin(
    plugin_id: String,
    context: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Executing plugin: {}", plugin_id);
    
    let plugin_system = service_manager.plugin_system.read().await;
    let plugin_uuid = Uuid::parse_str(&plugin_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid plugin ID".to_string()))?;
    
    // Create execution context from JSON
    let execution_context = crate::services::plugin_system::PluginExecutionContext {
        plugin_id: plugin_uuid,
        session_id: Uuid::new_v4(),
        user_id: None, // Would be extracted from context
        permissions: vec![], // Would be determined from plugin config
        configuration: HashMap::new(),
        environment: HashMap::new(),
        resource_limits: crate::services::plugin_system::ResourceLimits {
            max_memory_mb: 512,
            max_cpu_percent: 50,
            max_execution_time_seconds: 300,
            max_network_requests: 100,
            max_file_operations: 50,
        },
    };
    
    let result = plugin_system.execute_plugin(plugin_uuid, execution_context).await?;
    Ok(serde_json::to_value(result)?)
}

/// Get marketplace plugins
#[tauri::command]
pub async fn get_marketplace_plugins(
    category: Option<String>,
    search_query: Option<String>,
    limit: Option<u32>,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Getting marketplace plugins");
    
    let plugin_system = service_manager.plugin_system.read().await;
    
    let category_enum = category.map(|c| match c.as_str() {
        "data_source" => crate::services::plugin_system::PluginCategory::DataSource,
        "analytics" => crate::services::plugin_system::PluginCategory::Analytics,
        "export" => crate::services::plugin_system::PluginCategory::Export,
        "visualization" => crate::services::plugin_system::PluginCategory::Visualization,
        "integration" => crate::services::plugin_system::PluginCategory::Integration,
        "utility" => crate::services::plugin_system::PluginCategory::Utility,
        "security" => crate::services::plugin_system::PluginCategory::Security,
        "workflow" => crate::services::plugin_system::PluginCategory::Workflow,
        _ => crate::services::plugin_system::PluginCategory::Utility,
    });
    
    let plugins = plugin_system.get_marketplace_plugins(category_enum, search_query, limit).await?;
    Ok(serde_json::to_value(plugins)?)
}

/// V1.2.0 Workflow Engine Commands

/// Create new workflow
#[tauri::command]
pub async fn create_workflow(
    user_id: String,
    name: String,
    description: Option<String>,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Creating workflow: {} for user: {}", name, user_id);
    
    let workflow_engine = service_manager.workflow_engine.read().await;
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    
    let definition = workflow_engine.create_workflow(user_uuid, name, description).await?;
    Ok(serde_json::to_value(definition)?)
}

/// Execute workflow
#[tauri::command]
pub async fn execute_workflow(
    workflow_id: String,
    user_id: String,
    parameters: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<String> {
    info!("Executing workflow: {} for user: {}", workflow_id, user_id);
    
    let workflow_engine = service_manager.workflow_engine.read().await;
    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid workflow ID".to_string()))?;
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    
    let request = crate::services::workflow_engine::WorkflowExecutionRequest {
        workflow_id: workflow_uuid,
        user_id: user_uuid,
        parameters: serde_json::from_value(parameters).unwrap_or_default(),
        execution_mode: crate::services::workflow_engine::ExecutionMode::Asynchronous,
        priority: crate::services::workflow_engine::ExecutionPriority::Normal,
        timeout_minutes: Some(60),
        retry_config: None,
    };
    
    let execution_id = workflow_engine.execute_workflow(request).await?;
    Ok(execution_id.to_string())
}

/// Get workflow templates
#[tauri::command]
pub async fn get_workflow_templates(
    category: Option<String>,
    search_query: Option<String>,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Getting workflow templates");
    
    let workflow_engine = service_manager.workflow_engine.read().await;
    
    let category_enum = category.map(|c| match c.as_str() {
        "data_processing" => crate::services::workflow_engine::WorkflowCategory::DataProcessing,
        "research" => crate::services::workflow_engine::WorkflowCategory::Research,
        "analytics" => crate::services::workflow_engine::WorkflowCategory::Analytics,
        "integration" => crate::services::workflow_engine::WorkflowCategory::Integration,
        "automation" => crate::services::workflow_engine::WorkflowCategory::Automation,
        "reporting" => crate::services::workflow_engine::WorkflowCategory::Reporting,
        _ => crate::services::workflow_engine::WorkflowCategory::Custom,
    });
    
    let templates = workflow_engine.get_workflow_templates(category_enum, search_query).await?;
    Ok(serde_json::to_value(templates)?)
}

/// V1.2.0 ML Engine Commands

/// Train ML model
#[tauri::command]
pub async fn train_ml_model(
    model_name: String,
    model_type: String,
    user_id: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<String> {
    info!("Training ML model: {} for user: {}", model_name, user_id);
    
    let ml_engine = service_manager.ml_engine.read().await;
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    
    // Create default training config
    let training_config = crate::services::ml_engine::model_training::TrainingConfig::default();
    let model_type_enum = crate::services::ml_engine::inference_engine::ModelType::Classification; // Default
    
    let job_id = ml_engine.train_model(model_name, model_type_enum, training_config, user_uuid).await?;
    Ok(job_id.to_string())
}

/// Make ML prediction
#[tauri::command]
pub async fn make_ml_prediction(
    model_id: String,
    features: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Making ML prediction with model: {}", model_id);
    
    let ml_engine = service_manager.ml_engine.read().await;
    
    let request = crate::services::ml_engine::PredictionRequest {
        model_id,
        features: serde_json::from_value(features).unwrap_or_default(),
        prediction_type: crate::services::ml_engine::PredictionType::Classification,
        confidence_required: true,
        explanation_required: false,
        user_id: None,
        context: HashMap::new(),
    };
    
    let result = ml_engine.predict(request).await?;
    Ok(serde_json::to_value(result)?)
}

/// V1.2.0 Cloud Sync Commands

/// Start cloud sync
#[tauri::command]
pub async fn start_cloud_sync(
    user_id: String,
    device_id: String,
    provider: String,
    sync_type: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<String> {
    info!("Starting cloud sync for user: {} on device: {}", user_id, device_id);
    
    let cloud_sync = service_manager.cloud_sync.read().await;
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    let device_uuid = Uuid::parse_str(&device_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid device ID".to_string()))?;
    
    let provider_type = match provider.as_str() {
        "aws" => crate::services::cloud_sync::CloudProviderType::AWS,
        "google" => crate::services::cloud_sync::CloudProviderType::GoogleCloud,
        "azure" => crate::services::cloud_sync::CloudProviderType::Azure,
        _ => crate::services::cloud_sync::CloudProviderType::AWS,
    };
    
    let sync_type_enum = match sync_type.as_str() {
        "full" => crate::services::cloud_sync::SyncType::Full,
        "incremental" => crate::services::cloud_sync::SyncType::Incremental,
        "selective" => crate::services::cloud_sync::SyncType::Selective,
        "backup" => crate::services::cloud_sync::SyncType::Backup,
        "restore" => crate::services::cloud_sync::SyncType::Restore,
        _ => crate::services::cloud_sync::SyncType::Incremental,
    };
    
    let request = crate::services::cloud_sync::SyncRequest {
        user_id: user_uuid,
        device_id: device_uuid,
        provider: provider_type,
        sync_type: sync_type_enum,
        data_types: vec![], // Would be specified by user
        selective_paths: None,
        force_sync: false,
        dry_run: false,
    };
    
    let session_id = cloud_sync.start_sync(request).await?;
    Ok(session_id.to_string())
}

/// V1.2.0 Enterprise Commands

/// Create enterprise user
#[tauri::command]
pub async fn create_enterprise_user(
    user_request: serde_json::Value,
    created_by: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Creating enterprise user");
    
    let enterprise = service_manager.enterprise.read().await;
    let created_by_uuid = Uuid::parse_str(&created_by)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid creator ID".to_string()))?;
    
    let request: crate::services::enterprise::EnterpriseUserRequest = 
        serde_json::from_value(user_request)?;
    
    let user = enterprise.create_user(request, created_by_uuid).await?;
    Ok(serde_json::to_value(user)?)
}

/// Check access permissions
#[tauri::command]
pub async fn check_enterprise_access(
    user_id: String,
    resource_type: String,
    resource_id: String,
    action: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Checking enterprise access for user: {}", user_id);
    
    let enterprise = service_manager.enterprise.read().await;
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    
    let request = crate::services::enterprise::AccessRequest {
        user_id: user_uuid,
        resource_type,
        resource_id,
        action,
        context: HashMap::new(),
        tenant_id: None,
    };
    
    let decision = enterprise.check_access(request).await?;
    Ok(serde_json::to_value(decision)?)
}

/// V2.0.0 Distributed System Commands

/// Join cluster
#[tauri::command]
pub async fn join_cluster(
    node_config: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<()> {
    info!("Joining cluster");
    
    let distributed = service_manager.distributed.read().await;
    let node: crate::services::distributed::ClusterNode = 
        serde_json::from_value(node_config)?;
    
    distributed.join_cluster(node).await
}

/// Deploy service to cluster
#[tauri::command]
pub async fn deploy_service_to_cluster(
    deployment_request: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<String> {
    info!("Deploying service to cluster");
    
    let distributed = service_manager.distributed.read().await;
    let request: crate::services::distributed::DeploymentRequest = 
        serde_json::from_value(deployment_request)?;
    
    let deployment_id = distributed.deploy_service(request).await?;
    Ok(deployment_id.to_string())
}

/// V2.0.0 AI Orchestration Commands

/// Register AI agent
#[tauri::command]
pub async fn register_ai_agent(
    agent: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<()> {
    info!("Registering AI agent");
    
    let ai_orchestration = service_manager.ai_orchestration.read().await;
    let agent_config: crate::services::ai_orchestration::AIAgent = 
        serde_json::from_value(agent)?;
    
    ai_orchestration.register_agent(agent_config).await
}

/// Submit AI task
#[tauri::command]
pub async fn submit_ai_task(
    task: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<String> {
    info!("Submitting AI task");
    
    let ai_orchestration = service_manager.ai_orchestration.read().await;
    let task_config: crate::services::ai_orchestration::AITask = 
        serde_json::from_value(task)?;
    
    let task_id = ai_orchestration.submit_task(task_config).await?;
    Ok(task_id.to_string())
}

/// V2.0.0 Real-time Collaboration Commands

/// Start collaboration session
#[tauri::command]
pub async fn start_realtime_collaboration(
    collaboration_request: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<String> {
    info!("Starting real-time collaboration");
    
    let realtime_collaboration = service_manager.realtime_collaboration.read().await;
    let request: crate::services::realtime_collaboration::CollaborationRequest = 
        serde_json::from_value(collaboration_request)?;
    
    let session_id = realtime_collaboration.start_collaboration(request).await?;
    Ok(session_id.to_string())
}

/// Send chat message
#[tauri::command]
pub async fn send_collaboration_message(
    message: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<()> {
    info!("Sending collaboration message");
    
    let realtime_collaboration = service_manager.realtime_collaboration.read().await;
    let chat_message: crate::services::realtime_collaboration::ChatMessage = 
        serde_json::from_value(message)?;
    
    realtime_collaboration.send_chat_message(chat_message).await
}

/// Get collaboration statistics
#[tauri::command]
pub async fn get_collaboration_statistics(
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Getting collaboration statistics");
    
    let realtime_collaboration = service_manager.realtime_collaboration.read().await;
    let stats = realtime_collaboration.get_collaboration_stats().await?;
    Ok(serde_json::to_value(stats)?)
}
