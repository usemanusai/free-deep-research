use tauri::State;
use uuid::Uuid;
use tracing::{info, debug, error};

use crate::services::ServiceManager;
use crate::models::nlp_engine::*;

#[tauri::command]
pub async fn register_nlp_model(
    service_manager: State<'_, ServiceManager>,
    model: NLPModel,
) -> Result<NLPModel, String> {
    info!("API: Registering NLP model: {}", model.name);
    match service_manager.nlp_engine_service.register_nlp_model(model).await {
        Ok(registered_model) => Ok(registered_model),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn process_semantic_query(
    service_manager: State<'_, ServiceManager>,
    query: String,
    model_id: String,
) -> Result<SemanticQuery, String> {
    debug!("API: Processing semantic query with model: {}", model_id);
    let mid = Uuid::parse_str(&model_id).map_err(|e| format!("Invalid model ID: {}", e))?;
    match service_manager.nlp_engine_service.process_semantic_query(query, mid).await {
        Ok(semantic_query) => Ok(semantic_query),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn conduct_literature_review(
    service_manager: State<'_, ServiceManager>,
    query: String,
    model_id: String,
    params: SearchParameters,
) -> Result<LiteratureReview, String> {
    info!("API: Conducting literature review for query: {}", query);
    let mid = Uuid::parse_str(&model_id).map_err(|e| format!("Invalid model ID: {}", e))?;
    match service_manager.nlp_engine_service.conduct_literature_review(query, mid, params).await {
        Ok(review) => Ok(review),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn expand_query(
    service_manager: State<'_, ServiceManager>,
    query: String,
    model_id: String,
    strategy: ExpansionStrategy,
) -> Result<QueryExpansion, String> {
    debug!("API: Expanding query with strategy: {:?}", strategy);
    let mid = Uuid::parse_str(&model_id).map_err(|e| format!("Invalid model ID: {}", e))?;
    match service_manager.nlp_engine_service.expand_query(query, mid, strategy).await {
        Ok(expansion) => Ok(expansion),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn analyze_text(
    service_manager: State<'_, ServiceManager>,
    request: NLPProcessingRequest,
) -> Result<NLPProcessingResult, String> {
    debug!("API: Analyzing text with model: {}", request.model_id);
    match service_manager.nlp_engine_service.analyze_text(request).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_available_nlp_models(
    service_manager: State<'_, ServiceManager>,
    model_type: Option<ModelType>,
) -> Result<Vec<NLPModel>, String> {
    debug!("API: Getting available NLP models");
    match service_manager.nlp_engine_service.get_available_models(model_type).await {
        Ok(models) => Ok(models),
        Err(e) => Err(e.to_string())
    }
}
