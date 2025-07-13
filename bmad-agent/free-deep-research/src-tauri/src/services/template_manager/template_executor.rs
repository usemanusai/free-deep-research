use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use uuid::Uuid;

use crate::error::{AppResult, ApiError};
use crate::models::research_template::{ResearchTemplate, TemplateExecutionContext};
use crate::models::research_workflow::{ResearchWorkflow, WorkflowStep, WorkflowParameters};
use crate::services::{DataPersistenceService, ResearchEngineService};

/// Template executor that converts templates into executable workflows
pub struct TemplateExecutor {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    research_engine: Arc<RwLock<ResearchEngineService>>,
}

impl TemplateExecutor {
    /// Create a new template executor
    pub async fn new(
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        research_engine: Arc<RwLock<ResearchEngineService>>,
    ) -> AppResult<Self> {
        info!("Initializing template executor...");

        let executor = Self {
            data_persistence,
            research_engine,
        };

        info!("Template executor initialized successfully");
        Ok(executor)
    }

    /// Execute a template to create a workflow
    pub async fn execute_template(
        &self,
        context: TemplateExecutionContext,
    ) -> AppResult<ResearchWorkflow> {
        info!("Executing template: {}", context.template_id);

        // Get template from database
        let data_persistence = self.data_persistence.read().await;
        let template = data_persistence.get_research_template(context.template_id).await?
            .ok_or_else(|| ApiError::not_found("Template".to_string(), context.template_id.to_string()))?;
        drop(data_persistence);

        // Validate parameters
        template.validate_parameters(&context.parameters)
            .map_err(|errors| ApiError::invalid_input(errors.join(", ")))?;

        // Get parameters with defaults applied
        let final_parameters = template.get_parameters_with_defaults(&context.parameters);

        // Create workflow from template
        let workflow = self.create_workflow_from_template(&template, &context, &final_parameters).await?;

        // Save workflow to database
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_workflow(&workflow).await?;
        drop(data_persistence);

        info!("Template executed successfully, created workflow: {}", workflow.id);
        Ok(workflow)
    }

    /// Create a workflow from a template
    async fn create_workflow_from_template(
        &self,
        template: &ResearchTemplate,
        context: &TemplateExecutionContext,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> AppResult<ResearchWorkflow> {
        debug!("Creating workflow from template: {}", template.name);

        // Create workflow parameters from template
        let mut workflow_params = template.workflow_config.clone();
        workflow_params.methodology = template.methodology.clone();

        // Apply any parameter overrides from execution context
        if let Some(max_iterations) = context.execution_metadata.get("max_iterations").and_then(|v| v.as_u64()) {
            workflow_params.max_iterations = max_iterations as u32;
        }
        if let Some(timeout_minutes) = context.execution_metadata.get("timeout_minutes").and_then(|v| v.as_u64()) {
            workflow_params.timeout_minutes = timeout_minutes as u32;
        }

        // Create workflow
        let mut workflow = ResearchWorkflow::new(
            context.workflow_name.clone(),
            self.extract_query_from_parameters(parameters)?,
            workflow_params,
            context.created_by.clone(),
        );

        // Set template ID
        workflow.template_id = Some(template.id);

        // Add template tags to workflow
        workflow.tags = template.tags.clone();

        // Add execution metadata
        for (key, value) in &context.execution_metadata {
            workflow.metadata.insert(key.clone(), value.to_string());
        }

        // Create workflow steps from template steps
        let workflow_steps = self.create_workflow_steps_from_template(
            &template.steps,
            parameters,
            workflow.id,
        )?;

        // Add steps to workflow
        for step in workflow_steps {
            workflow.add_step(step);
        }

        debug!("Created workflow with {} steps from template", workflow.steps.len());
        Ok(workflow)
    }

    /// Create workflow steps from template steps
    fn create_workflow_steps_from_template(
        &self,
        template_steps: &[crate::models::research_template::TemplateStep],
        parameters: &HashMap<String, serde_json::Value>,
        workflow_id: Uuid,
    ) -> AppResult<Vec<WorkflowStep>> {
        let mut workflow_steps = Vec::new();
        let mut step_id_mapping = HashMap::new();

        // Sort template steps by order
        let mut sorted_steps = template_steps.to_vec();
        sorted_steps.sort_by_key(|s| s.order);

        for (index, template_step) in sorted_steps.iter().enumerate() {
            // Check if step should be executed based on conditional
            if !template_step.should_execute(parameters) {
                debug!("Skipping step '{}' due to conditional", template_step.name);
                continue;
            }

            // Create workflow step
            let step_id = Uuid::new_v4();
            step_id_mapping.insert(template_step.id.clone(), step_id);

            let mut workflow_step = WorkflowStep::new(
                workflow_id,
                index as u32 + 1,
                template_step.name.clone(),
                template_step.description.clone(),
            );

            // Set step properties
            workflow_step.id = step_id;
            workflow_step.service_provider = template_step.service_provider.clone();
            workflow_step.endpoint = template_step.endpoint.clone();
            workflow_step.max_retries = template_step.retry_count;

            // Substitute parameters in input template
            workflow_step.input_data = template_step.substitute_parameters(parameters);

            // Map dependencies
            for dep_id in &template_step.depends_on {
                if let Some(&mapped_id) = step_id_mapping.get(dep_id) {
                    workflow_step.depends_on.push(mapped_id);
                }
            }

            // Add output mapping to metadata
            for (output_key, variable_name) in &template_step.output_mapping {
                workflow_step.metadata.insert(
                    format!("output_mapping_{}", output_key),
                    variable_name.clone(),
                );
            }

            // Add template step ID to metadata
            workflow_step.metadata.insert("template_step_id".to_string(), template_step.id.clone());

            workflow_steps.push(workflow_step);
        }

        debug!("Created {} workflow steps from {} template steps", 
               workflow_steps.len(), template_steps.len());
        Ok(workflow_steps)
    }

    /// Extract the main research query from parameters
    fn extract_query_from_parameters(&self, parameters: &HashMap<String, serde_json::Value>) -> AppResult<String> {
        // Look for common query parameter names
        let query_param_names = ["query", "research_query", "question", "topic", "subject"];
        
        for param_name in &query_param_names {
            if let Some(query_value) = parameters.get(*param_name) {
                if let Some(query_str) = query_value.as_str() {
                    if !query_str.trim().is_empty() {
                        return Ok(query_str.to_string());
                    }
                }
            }
        }

        // If no specific query parameter found, try to construct from available parameters
        let mut query_parts = Vec::new();
        
        for (key, value) in parameters {
            if let Some(str_value) = value.as_str() {
                if !str_value.trim().is_empty() && str_value.len() > 10 {
                    query_parts.push(format!("{}: {}", key, str_value));
                }
            }
        }

        if !query_parts.is_empty() {
            Ok(query_parts.join(" | "))
        } else {
            Err(ApiError::invalid_input(
                "No valid research query found in template parameters".to_string()
            ))
        }
    }

    /// Validate template execution context
    pub fn validate_execution_context(&self, context: &TemplateExecutionContext) -> AppResult<()> {
        if context.workflow_name.trim().is_empty() {
            return Err(ApiError::invalid_input(
                "Workflow name cannot be empty".to_string()
            ));
        }

        if context.created_by.trim().is_empty() {
            return Err(ApiError::invalid_input(
                "Created by field cannot be empty".to_string()
            ));
        }

        Ok(())
    }

    /// Preview workflow that would be created from template
    pub async fn preview_workflow_from_template(
        &self,
        context: &TemplateExecutionContext,
    ) -> AppResult<WorkflowPreview> {
        info!("Previewing workflow from template: {}", context.template_id);

        // Get template from database
        let data_persistence = self.data_persistence.read().await;
        let template = data_persistence.get_research_template(context.template_id).await?
            .ok_or_else(|| ApiError::not_found("Template".to_string(), context.template_id.to_string()))?;
        drop(data_persistence);

        // Validate parameters
        template.validate_parameters(&context.parameters)
            .map_err(|errors| ApiError::invalid_input(errors.join(", ")))?;

        // Get parameters with defaults applied
        let final_parameters = template.get_parameters_with_defaults(&context.parameters);

        // Create preview
        let query = self.extract_query_from_parameters(&final_parameters)?;
        
        // Count steps that would be executed
        let executable_steps = template.steps.iter()
            .filter(|step| step.should_execute(&final_parameters))
            .count();

        // Estimate execution time based on template complexity
        let estimated_time_minutes = self.estimate_execution_time(&template, executable_steps);

        let preview = WorkflowPreview {
            template_name: template.name.clone(),
            workflow_name: context.workflow_name.clone(),
            methodology: template.methodology.clone(),
            query,
            total_steps: executable_steps,
            estimated_time_minutes,
            parameters_used: final_parameters.len(),
            services_involved: self.get_services_from_template(&template),
        };

        info!("Generated workflow preview for template: {}", template.name);
        Ok(preview)
    }

    /// Estimate execution time for a template
    fn estimate_execution_time(&self, template: &ResearchTemplate, executable_steps: usize) -> u32 {
        // Base time per step (in minutes)
        let base_time_per_step = match template.methodology {
            crate::models::research_workflow::ResearchMethodology::DonLim => 2, // Faster, cost-optimized
            crate::models::research_workflow::ResearchMethodology::NickScamara => 3, // More thorough
            crate::models::research_workflow::ResearchMethodology::Hybrid => 4, // Most comprehensive
            crate::models::research_workflow::ResearchMethodology::Custom => 3,
        };

        // Additional time based on complexity
        let complexity_multiplier = if executable_steps > 5 { 1.5 } else { 1.0 };
        
        ((executable_steps as f64 * base_time_per_step as f64 * complexity_multiplier) as u32).max(1)
    }

    /// Get list of services involved in template execution
    fn get_services_from_template(&self, template: &ResearchTemplate) -> Vec<String> {
        let mut services = std::collections::HashSet::new();
        
        for step in &template.steps {
            if let Some(service) = &step.service_provider {
                services.insert(service.clone());
            }
        }

        // Add default services based on methodology
        match template.methodology {
            crate::models::research_workflow::ResearchMethodology::DonLim => {
                services.insert("serpapi".to_string());
                services.insert("jina".to_string());
                services.insert("openrouter".to_string());
            }
            crate::models::research_workflow::ResearchMethodology::NickScamara => {
                services.insert("firecrawl".to_string());
                services.insert("openrouter".to_string());
            }
            crate::models::research_workflow::ResearchMethodology::Hybrid => {
                services.insert("serpapi".to_string());
                services.insert("jina".to_string());
                services.insert("firecrawl".to_string());
                services.insert("openrouter".to_string());
            }
            crate::models::research_workflow::ResearchMethodology::Custom => {}
        }

        services.into_iter().collect()
    }
}

/// Workflow preview information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowPreview {
    pub template_name: String,
    pub workflow_name: String,
    pub methodology: crate::models::research_workflow::ResearchMethodology,
    pub query: String,
    pub total_steps: usize,
    pub estimated_time_minutes: u32,
    pub parameters_used: usize,
    pub services_involved: Vec<String>,
}
