use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};
use uuid::Uuid;

use crate::error::AppResult;
use crate::services::{Service, DataPersistenceService};
use crate::models::nlp_engine::*;

pub mod model_manager;
pub mod literature_reviewer;
pub mod semantic_processor;
pub mod query_expander;
pub mod text_analyzer;

use model_manager::NLPModelManager;
use literature_reviewer::LiteratureReviewer;
use semantic_processor::SemanticProcessor;
use query_expander::QueryExpander;
use text_analyzer::TextAnalyzer;

/// Advanced NLP Engine Service for natural language processing and literature review
pub struct NLPEngineService {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    model_manager: Arc<RwLock<NLPModelManager>>,
    literature_reviewer: Arc<RwLock<LiteratureReviewer>>,
    semantic_processor: Arc<RwLock<SemanticProcessor>>,
    query_expander: Arc<RwLock<QueryExpander>>,
    text_analyzer: Arc<RwLock<TextAnalyzer>>,
}

impl NLPEngineService {
    pub async fn new(data_persistence: Arc<RwLock<DataPersistenceService>>) -> AppResult<Self> {
        info!("Initializing NLP Engine Service");

        let model_manager = Arc::new(RwLock::new(
            NLPModelManager::new(data_persistence.clone()).await?
        ));

        let literature_reviewer = Arc::new(RwLock::new(
            LiteratureReviewer::new(data_persistence.clone(), model_manager.clone()).await?
        ));

        let semantic_processor = Arc::new(RwLock::new(
            SemanticProcessor::new(model_manager.clone()).await?
        ));

        let query_expander = Arc::new(RwLock::new(
            QueryExpander::new(model_manager.clone()).await?
        ));

        let text_analyzer = Arc::new(RwLock::new(
            TextAnalyzer::new(model_manager.clone()).await?
        ));

        Ok(Self {
            data_persistence,
            model_manager,
            literature_reviewer,
            semantic_processor,
            query_expander,
            text_analyzer,
        })
    }

    pub async fn register_nlp_model(&self, model: NLPModel) -> AppResult<NLPModel> {
        info!("Registering NLP model: {}", model.name);
        let model_manager = self.model_manager.write().await;
        model_manager.register_model(model).await
    }

    pub async fn process_semantic_query(&self, query: String, model_id: Uuid) -> AppResult<SemanticQuery> {
        debug!("Processing semantic query with model: {}", model_id);
        let semantic_processor = self.semantic_processor.read().await;
        semantic_processor.process_query(query, model_id).await
    }

    pub async fn conduct_literature_review(&self, query: String, model_id: Uuid, params: SearchParameters) -> AppResult<LiteratureReview> {
        info!("Conducting literature review for query: {}", query);
        let literature_reviewer = self.literature_reviewer.write().await;
        literature_reviewer.conduct_review(query, model_id, params).await
    }

    pub async fn expand_query(&self, query: String, model_id: Uuid, strategy: ExpansionStrategy) -> AppResult<QueryExpansion> {
        debug!("Expanding query with strategy: {:?}", strategy);
        let query_expander = self.query_expander.read().await;
        query_expander.expand_query(query, model_id, strategy).await
    }

    pub async fn analyze_text(&self, request: NLPProcessingRequest) -> AppResult<NLPProcessingResult> {
        debug!("Analyzing text with model: {}", request.model_id);
        let text_analyzer = self.text_analyzer.read().await;
        text_analyzer.analyze_text(request).await
    }

    pub async fn get_available_models(&self, model_type: Option<ModelType>) -> AppResult<Vec<NLPModel>> {
        debug!("Getting available NLP models");
        let model_manager = self.model_manager.read().await;
        model_manager.get_models(model_type).await
    }

    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting NLP engine background tasks...");
        let model_manager = self.model_manager.read().await;
        model_manager.start_model_monitoring().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for NLPEngineService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing NLP engine service health check");
        let model_manager = self.model_manager.read().await;
        model_manager.health_check().await
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down NLP engine service...");
        let model_manager = self.model_manager.read().await;
        model_manager.shutdown().await?;
        Ok(())
    }
}
