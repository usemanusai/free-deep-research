use tauri::State;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::research_template::{
    ResearchTemplate, TemplateCategory, TemplateExecutionContext, TemplateMetrics
};
use crate::models::research_workflow::ResearchWorkflow;
use crate::services::{ServiceManager, template_manager::TemplateStatistics};

/// Create a new research template
#[tauri::command]
pub async fn create_research_template(
    template: ResearchTemplate,
    service_manager: State<'_, ServiceManager>,
) -> Result<ResearchTemplate, String> {
    info!("Creating research template: {}", template.name);

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.create_template(template).await {
        Ok(created_template) => {
            info!("Created research template with ID: {}", created_template.id);
            Ok(created_template)
        }
        Err(e) => {
            error!("Failed to create research template: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get a research template by ID
#[tauri::command]
pub async fn get_research_template(
    template_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<ResearchTemplate>, String> {
    info!("Getting research template: {}", template_id);

    let template_uuid = Uuid::parse_str(&template_id)
        .map_err(|e| format!("Invalid template ID: {}", e))?;

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.get_template(template_uuid).await {
        Ok(template) => Ok(template),
        Err(e) => {
            error!("Failed to get research template {}: {}", template_id, e);
            Err(e.to_string())
        }
    }
}

/// Get all research templates
#[tauri::command]
pub async fn get_all_research_templates(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchTemplate>, String> {
    info!("Getting all research templates");

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.get_all_templates().await {
        Ok(templates) => {
            info!("Retrieved {} research templates", templates.len());
            Ok(templates)
        }
        Err(e) => {
            error!("Failed to get research templates: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get research templates by category
#[tauri::command]
pub async fn get_research_templates_by_category(
    category: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchTemplate>, String> {
    info!("Getting research templates by category: {}", category);

    let template_category = match category.to_lowercase().as_str() {
        "academic" => TemplateCategory::Academic,
        "business" => TemplateCategory::Business,
        "technical" => TemplateCategory::Technical,
        "market" => TemplateCategory::Market,
        "competitive" => TemplateCategory::Competitive,
        "scientific" => TemplateCategory::Scientific,
        "legal" => TemplateCategory::Legal,
        "medical" => TemplateCategory::Medical,
        "financial" => TemplateCategory::Financial,
        "custom" => TemplateCategory::Custom,
        _ => return Err(format!("Invalid template category: {}", category)),
    };

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.get_templates_by_category(template_category).await {
        Ok(templates) => {
            info!("Retrieved {} templates for category: {}", templates.len(), category);
            Ok(templates)
        }
        Err(e) => {
            error!("Failed to get templates by category {}: {}", category, e);
            Err(e.to_string())
        }
    }
}

/// Get featured research templates
#[tauri::command]
pub async fn get_featured_research_templates(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchTemplate>, String> {
    info!("Getting featured research templates");

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.get_featured_templates().await {
        Ok(templates) => {
            info!("Retrieved {} featured templates", templates.len());
            Ok(templates)
        }
        Err(e) => {
            error!("Failed to get featured templates: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get public research templates
#[tauri::command]
pub async fn get_public_research_templates(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchTemplate>, String> {
    info!("Getting public research templates");

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.get_public_templates().await {
        Ok(templates) => {
            info!("Retrieved {} public templates", templates.len());
            Ok(templates)
        }
        Err(e) => {
            error!("Failed to get public templates: {}", e);
            Err(e.to_string())
        }
    }
}

/// Search research templates
#[tauri::command]
pub async fn search_research_templates(
    query: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchTemplate>, String> {
    info!("Searching research templates with query: {}", query);

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.search_templates(&query).await {
        Ok(templates) => {
            info!("Found {} templates matching query: {}", templates.len(), query);
            Ok(templates)
        }
        Err(e) => {
            error!("Failed to search templates: {}", e);
            Err(e.to_string())
        }
    }
}

/// Update a research template
#[tauri::command]
pub async fn update_research_template(
    template: ResearchTemplate,
    service_manager: State<'_, ServiceManager>,
) -> Result<ResearchTemplate, String> {
    info!("Updating research template: {}", template.id);

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.update_template(template).await {
        Ok(updated_template) => {
            info!("Updated research template: {}", updated_template.id);
            Ok(updated_template)
        }
        Err(e) => {
            error!("Failed to update research template: {}", e);
            Err(e.to_string())
        }
    }
}

/// Delete a research template
#[tauri::command]
pub async fn delete_research_template(
    template_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Deleting research template: {}", template_id);

    let template_uuid = Uuid::parse_str(&template_id)
        .map_err(|e| format!("Invalid template ID: {}", e))?;

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.delete_template(template_uuid).await {
        Ok(()) => {
            info!("Deleted research template: {}", template_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete research template {}: {}", template_id, e);
            Err(e.to_string())
        }
    }
}

/// Execute a template to create a workflow
#[tauri::command]
pub async fn execute_research_template(
    context: TemplateExecutionContext,
    service_manager: State<'_, ServiceManager>,
) -> Result<ResearchWorkflow, String> {
    info!("Executing research template: {}", context.template_id);

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.execute_template(context).await {
        Ok(workflow) => {
            info!("Template executed successfully, created workflow: {}", workflow.id);
            Ok(workflow)
        }
        Err(e) => {
            error!("Failed to execute template: {}", e);
            Err(e.to_string())
        }
    }
}

/// Preview workflow that would be created from template
#[tauri::command]
pub async fn preview_template_execution(
    context: TemplateExecutionContext,
    service_manager: State<'_, ServiceManager>,
) -> Result<crate::services::template_manager::template_executor::WorkflowPreview, String> {
    info!("Previewing template execution: {}", context.template_id);

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.preview_template_execution(&context).await {
        Ok(preview) => {
            info!("Generated template execution preview");
            Ok(preview)
        }
        Err(e) => {
            error!("Failed to preview template execution: {}", e);
            Err(e.to_string())
        }
    }
}

/// Rate a research template
#[tauri::command]
pub async fn rate_research_template(
    template_id: String,
    rating: f64,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Rating research template {} with score: {}", template_id, rating);

    let template_uuid = Uuid::parse_str(&template_id)
        .map_err(|e| format!("Invalid template ID: {}", e))?;

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.rate_template(template_uuid, rating).await {
        Ok(()) => {
            info!("Rated research template: {}", template_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to rate research template {}: {}", template_id, e);
            Err(e.to_string())
        }
    }
}

/// Get template metrics
#[tauri::command]
pub async fn get_template_metrics(
    template_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<TemplateMetrics>, String> {
    let template_uuid = Uuid::parse_str(&template_id)
        .map_err(|e| format!("Invalid template ID: {}", e))?;

    let template_manager = service_manager.inner().template_manager.read().await;
    let metrics = template_manager.get_template_metrics(template_uuid).await;
    Ok(metrics)
}

/// Get all template metrics
#[tauri::command]
pub async fn get_all_template_metrics(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(String, TemplateMetrics)>, String> {
    info!("Getting all template metrics");

    let template_manager = service_manager.inner().template_manager.read().await;
    let metrics = template_manager.get_all_template_metrics().await;

    let result: Vec<_> = metrics.into_iter()
        .map(|(template_id, metrics)| (template_id.to_string(), metrics))
        .collect();

    info!("Retrieved metrics for {} templates", result.len());
    Ok(result)
}

/// Get template recommendations
#[tauri::command]
pub async fn get_template_recommendations(
    limit: usize,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchTemplate>, String> {
    info!("Getting template recommendations (limit: {})", limit);

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.get_template_recommendations(limit).await {
        Ok(templates) => {
            info!("Retrieved {} template recommendations", templates.len());
            Ok(templates)
        }
        Err(e) => {
            error!("Failed to get template recommendations: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get template statistics
#[tauri::command]
pub async fn get_template_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<TemplateStatistics, String> {
    info!("Getting template statistics");

    let template_manager = service_manager.inner().template_manager.read().await;
    match template_manager.get_template_statistics().await {
        Ok(stats) => {
            info!("Retrieved template statistics");
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to get template statistics: {}", e);
            Err(e.to_string())
        }
    }
}
