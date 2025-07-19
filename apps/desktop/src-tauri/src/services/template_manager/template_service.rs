use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::research_template::{
    ResearchTemplate, TemplateCategory, TemplateMetrics, TemplateExecutionContext
};
use crate::models::research_workflow::ResearchWorkflow;
use crate::services::{DataPersistenceService, ResearchEngineService};
use super::{TemplateExecutor, PredefinedTemplates, TemplateStatistics};

/// Template Manager Service that handles all template operations
pub struct TemplateManagerService {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    research_engine: Arc<RwLock<ResearchEngineService>>,
    template_executor: Arc<TemplateExecutor>,
    template_metrics: Arc<RwLock<HashMap<Uuid, TemplateMetrics>>>,
}

impl TemplateManagerService {
    /// Create a new template manager service
    pub async fn new(
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        research_engine: Arc<RwLock<ResearchEngineService>>,
    ) -> AppResult<Self> {
        info!("Initializing template manager service...");

        // Create template executor
        let template_executor = Arc::new(TemplateExecutor::new(
            data_persistence.clone(),
            research_engine.clone(),
        ).await?);

        let service = Self {
            data_persistence,
            research_engine,
            template_executor,
            template_metrics: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize predefined templates
        service.initialize_predefined_templates().await?;

        // Load existing template metrics
        service.load_template_metrics().await?;

        info!("Template manager service initialized successfully");
        Ok(service)
    }

    /// Create a new research template
    pub async fn create_template(&self, template: ResearchTemplate) -> AppResult<ResearchTemplate> {
        info!("Creating new research template: {}", template.name);

        // Validate template
        self.validate_template(&template)?;

        // Save template to database
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_template(&template).await?;
        drop(data_persistence);

        // Initialize metrics
        let mut metrics = self.template_metrics.write().await;
        metrics.insert(template.id, TemplateMetrics::new(template.id));
        drop(metrics);

        info!("Created research template with ID: {}", template.id);
        Ok(template)
    }

    /// Get template by ID
    pub async fn get_template(&self, template_id: Uuid) -> AppResult<Option<ResearchTemplate>> {
        let data_persistence = self.data_persistence.read().await;
        data_persistence.get_research_template(template_id).await
    }

    /// Get all templates
    pub async fn get_all_templates(&self) -> AppResult<Vec<ResearchTemplate>> {
        let data_persistence = self.data_persistence.read().await;
        data_persistence.get_all_research_templates().await
    }

    /// Get templates by category
    pub async fn get_templates_by_category(&self, category: TemplateCategory) -> AppResult<Vec<ResearchTemplate>> {
        let data_persistence = self.data_persistence.read().await;
        data_persistence.get_research_templates_by_category(category).await
    }

    /// Get featured templates
    pub async fn get_featured_templates(&self) -> AppResult<Vec<ResearchTemplate>> {
        let data_persistence = self.data_persistence.read().await;
        data_persistence.get_featured_research_templates().await
    }

    /// Get public templates
    pub async fn get_public_templates(&self) -> AppResult<Vec<ResearchTemplate>> {
        let data_persistence = self.data_persistence.read().await;
        data_persistence.get_public_research_templates().await
    }

    /// Search templates
    pub async fn search_templates(&self, query: &str) -> AppResult<Vec<ResearchTemplate>> {
        let data_persistence = self.data_persistence.read().await;
        data_persistence.search_research_templates(query).await
    }

    /// Update template
    pub async fn update_template(&self, template: ResearchTemplate) -> AppResult<ResearchTemplate> {
        info!("Updating research template: {}", template.id);

        // Validate template
        self.validate_template(&template)?;

        // Save updated template
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_template(&template).await?;
        drop(data_persistence);

        info!("Updated research template: {}", template.id);
        Ok(template)
    }

    /// Delete template
    pub async fn delete_template(&self, template_id: Uuid) -> AppResult<()> {
        info!("Deleting research template: {}", template_id);

        let data_persistence = self.data_persistence.write().await;
        data_persistence.delete_research_template(template_id).await?;
        drop(data_persistence);

        // Remove metrics
        let mut metrics = self.template_metrics.write().await;
        metrics.remove(&template_id);
        drop(metrics);

        info!("Deleted research template: {}", template_id);
        Ok(())
    }

    /// Execute template to create workflow
    pub async fn execute_template(
        &self,
        context: TemplateExecutionContext,
    ) -> AppResult<ResearchWorkflow> {
        info!("Executing template: {}", context.template_id);

        // Validate execution context
        self.template_executor.validate_execution_context(&context)?;

        let workflow = self.template_executor.execute_template(context).await?;

        // Update template metrics
        self.update_template_usage(workflow.template_id.unwrap_or_default()).await?;

        info!("Template executed successfully, created workflow: {}", workflow.id);
        Ok(workflow)
    }

    /// Preview workflow that would be created from template
    pub async fn preview_template_execution(
        &self,
        context: &TemplateExecutionContext,
    ) -> AppResult<super::template_executor::WorkflowPreview> {
        info!("Previewing template execution: {}", context.template_id);
        self.template_executor.preview_workflow_from_template(context).await
    }

    /// Rate a template
    pub async fn rate_template(&self, template_id: Uuid, rating: f64) -> AppResult<()> {
        info!("Rating template {} with score: {}", template_id, rating);

        if !(1.0..=5.0).contains(&rating) {
            return Err(crate::error::ApiError::invalid_input(
                "Rating must be between 1.0 and 5.0".to_string()
            ));
        }

        // Get and update template
        let data_persistence = self.data_persistence.read().await;
        if let Some(mut template) = data_persistence.get_research_template(template_id).await? {
            drop(data_persistence);
            
            template.update_rating(rating);
            
            let data_persistence = self.data_persistence.write().await;
            data_persistence.save_research_template(&template).await?;
            drop(data_persistence);
            
            info!("Updated template rating: {}", template_id);
        }

        Ok(())
    }

    /// Get template metrics
    pub async fn get_template_metrics(&self, template_id: Uuid) -> Option<TemplateMetrics> {
        let metrics = self.template_metrics.read().await;
        metrics.get(&template_id).cloned()
    }

    /// Get all template metrics
    pub async fn get_all_template_metrics(&self) -> HashMap<Uuid, TemplateMetrics> {
        self.template_metrics.read().await.clone()
    }

    /// Update template execution metrics
    pub async fn update_template_execution_metrics(
        &self,
        template_id: Uuid,
        success: bool,
        execution_time_ms: u32,
    ) -> AppResult<()> {
        let mut metrics = self.template_metrics.write().await;
        if let Some(template_metrics) = metrics.get_mut(&template_id) {
            template_metrics.update_after_execution(success, execution_time_ms);
        } else {
            let mut new_metrics = TemplateMetrics::new(template_id);
            new_metrics.update_after_execution(success, execution_time_ms);
            metrics.insert(template_id, new_metrics);
        }
        drop(metrics);

        // Save metrics to database
        self.save_template_metrics().await?;

        Ok(())
    }

    /// Get template recommendations based on usage and ratings
    pub async fn get_template_recommendations(&self, limit: usize) -> AppResult<Vec<ResearchTemplate>> {
        let templates = self.get_all_templates().await?;
        let metrics = self.get_all_template_metrics().await;

        // Score templates based on usage, rating, and performance
        let mut scored_templates: Vec<_> = templates.into_iter()
            .map(|template| {
                let template_metrics = metrics.get(&template.id);
                let score = self.calculate_recommendation_score(&template, template_metrics);
                (template, score)
            })
            .collect();

        // Sort by score (highest first)
        scored_templates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return top templates
        Ok(scored_templates.into_iter()
            .take(limit)
            .map(|(template, _)| template)
            .collect())
    }

    /// Calculate recommendation score for a template
    fn calculate_recommendation_score(&self, template: &ResearchTemplate, metrics: Option<&TemplateMetrics>) -> f64 {
        let mut score = 0.0;

        // Base score from rating
        if template.rating_count > 0 {
            score += template.rating * 20.0; // Max 100 points from rating
        }

        // Usage popularity
        score += (template.usage_count as f64).ln().max(0.0) * 10.0;

        // Performance score from metrics
        if let Some(m) = metrics {
            score += m.performance_score * 0.5;
        }

        // Featured templates get bonus
        if template.is_featured {
            score += 25.0;
        }

        // Public templates get small bonus
        if template.is_public {
            score += 5.0;
        }

        score
    }

    /// Update template usage count
    async fn update_template_usage(&self, template_id: Uuid) -> AppResult<()> {
        // Update template usage count
        let data_persistence = self.data_persistence.read().await;
        if let Some(mut template) = data_persistence.get_research_template(template_id).await? {
            drop(data_persistence);
            
            template.increment_usage();
            
            let data_persistence = self.data_persistence.write().await;
            data_persistence.save_research_template(&template).await?;
            drop(data_persistence);
        }

        Ok(())
    }

    /// Validate template structure
    fn validate_template(&self, template: &ResearchTemplate) -> AppResult<()> {
        // Validate basic fields
        if template.name.trim().is_empty() {
            return Err(crate::error::ApiError::invalid_input(
                "Template name cannot be empty".to_string()
            ));
        }

        if template.description.trim().is_empty() {
            return Err(crate::error::ApiError::invalid_input(
                "Template description cannot be empty".to_string()
            ));
        }

        // Validate parameters
        for param in &template.parameters {
            if param.id.trim().is_empty() {
                return Err(crate::error::ApiError::invalid_input(
                    "Parameter ID cannot be empty".to_string()
                ));
            }
            if param.name.trim().is_empty() {
                return Err(crate::error::ApiError::invalid_input(
                    "Parameter name cannot be empty".to_string()
                ));
            }
        }

        // Validate steps
        for step in &template.steps {
            if step.id.trim().is_empty() {
                return Err(crate::error::ApiError::invalid_input(
                    "Step ID cannot be empty".to_string()
                ));
            }
            if step.name.trim().is_empty() {
                return Err(crate::error::ApiError::invalid_input(
                    "Step name cannot be empty".to_string()
                ));
            }
        }

        // Validate step dependencies
        let step_ids: std::collections::HashSet<_> = template.steps.iter().map(|s| &s.id).collect();
        for step in &template.steps {
            for dep in &step.depends_on {
                if !step_ids.contains(dep) {
                    return Err(crate::error::ApiError::invalid_input(
                        format!("Step '{}' depends on non-existent step '{}'", step.id, dep)
                    ));
                }
            }
        }

        Ok(())
    }

    /// Initialize predefined templates
    async fn initialize_predefined_templates(&self) -> AppResult<()> {
        info!("Initializing predefined templates...");

        let predefined = PredefinedTemplates::create_all();
        
        for template in predefined {
            // Check if template already exists
            let data_persistence = self.data_persistence.read().await;
            let existing = data_persistence.get_research_template_by_name(&template.name).await?;
            drop(data_persistence);

            if existing.is_none() {
                self.create_template(template).await?;
            }
        }

        info!("Predefined templates initialized successfully");
        Ok(())
    }

    /// Load template metrics from database
    async fn load_template_metrics(&self) -> AppResult<()> {
        debug!("Loading template metrics from database...");

        // TODO: Implement loading metrics from database
        // For now, we'll initialize empty metrics for all templates
        let templates = self.get_all_templates().await?;
        let mut metrics = self.template_metrics.write().await;
        
        for template in templates {
            if !metrics.contains_key(&template.id) {
                metrics.insert(template.id, TemplateMetrics::new(template.id));
            }
        }

        debug!("Loaded metrics for {} templates", metrics.len());
        Ok(())
    }

    /// Save template metrics to database
    async fn save_template_metrics(&self) -> AppResult<()> {
        debug!("Saving template metrics to database...");

        // TODO: Implement saving metrics to database
        // For now, metrics are kept in memory

        debug!("Template metrics saved successfully");
        Ok(())
    }

    /// Get template statistics
    pub async fn get_template_statistics(&self) -> AppResult<TemplateStatistics> {
        let templates = self.get_all_templates().await?;
        let metrics = self.get_all_template_metrics().await;

        let total = templates.len();
        let public = templates.iter().filter(|t| t.is_public).count();
        let featured = templates.iter().filter(|t| t.is_featured).count();
        let categories = templates.iter()
            .map(|t| t.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .len();

        let total_usage = templates.iter().map(|t| t.usage_count).sum();
        let average_rating = if templates.is_empty() {
            0.0
        } else {
            let rated_templates: Vec<_> = templates.iter().filter(|t| t.rating_count > 0).collect();
            if rated_templates.is_empty() {
                0.0
            } else {
                rated_templates.iter().map(|t| t.rating).sum::<f64>() / rated_templates.len() as f64
            }
        };

        let total_executions = metrics.values().map(|m| m.total_executions).sum();
        let average_success_rate = if metrics.is_empty() {
            100.0
        } else {
            metrics.values().map(|m| m.success_rate).sum::<f64>() / metrics.len() as f64
        };

        Ok(TemplateStatistics {
            total,
            public,
            featured,
            categories,
            total_usage,
            average_rating,
            total_executions,
            average_success_rate,
        })
    }

    /// Start background monitoring
    pub async fn start_background_monitoring(&self) -> AppResult<()> {
        info!("Starting template manager background monitoring...");

        // TODO: Implement background monitoring tasks
        // - Clean up unused templates
        // - Update template performance metrics
        // - Generate template recommendations

        info!("Template manager background monitoring started successfully");
        Ok(())
    }
}
