use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use super::{ExportOptions, CompressionType, PackageType, FileNamingConfig};

/// Export template for reusable export configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ExportCategory,
    pub options: ExportOptions,
    pub variables: HashMap<String, TemplateVariable>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub is_public: bool,
    pub usage_count: u64,
}

/// Export template categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportCategory {
    Report,
    Archive,
    Presentation,
    DataExchange,
    Backup,
    Custom,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub variable_type: VariableType,
    pub default_value: Option<String>,
    pub required: bool,
    pub validation_pattern: Option<String>,
}

/// Variable types for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Date,
    List,
    Choice(Vec<String>),
}

/// Export template manager
pub struct ExportTemplateManager {
    templates: Arc<RwLock<HashMap<String, ExportTemplate>>>,
    default_templates: HashMap<ExportCategory, ExportTemplate>,
}

impl ExportTemplateManager {
    /// Create a new export template manager
    pub async fn new() -> AppResult<Self> {
        info!("Initializing export template manager...");

        let templates = Arc::new(RwLock::new(HashMap::new()));
        let default_templates = Self::create_default_templates();

        let manager = Self {
            templates,
            default_templates,
        };

        // Load default templates
        manager.load_default_templates().await?;

        info!("Export template manager initialized successfully");
        Ok(manager)
    }

    /// Create default export templates
    fn create_default_templates() -> HashMap<ExportCategory, ExportTemplate> {
        let mut templates = HashMap::new();

        // Research Report Template
        let report_template = ExportTemplate {
            id: "default_research_report".to_string(),
            name: "Research Report".to_string(),
            description: "Complete research report with charts and analysis".to_string(),
            category: ExportCategory::Report,
            options: ExportOptions {
                formats: vec![
                    super::super::OutputFormat::Markdown,
                    super::super::OutputFormat::HTML,
                    super::super::OutputFormat::PDF,
                ],
                include_charts: true,
                chart_formats: vec![
                    super::super::ChartOutputFormat::SVG,
                    super::super::ChartOutputFormat::PNG,
                ],
                compression: CompressionType::None,
                encryption: None,
                file_naming: FileNamingConfig {
                    pattern: "research_report_{workflow_name}_{timestamp}".to_string(),
                    include_timestamp: true,
                    include_workflow_id: false,
                    custom_prefix: Some("report".to_string()),
                    custom_suffix: None,
                },
                package_type: PackageType::Folder,
                include_metadata: true,
                include_raw_data: false,
            },
            variables: HashMap::from([
                ("report_title".to_string(), TemplateVariable {
                    name: "report_title".to_string(),
                    description: "Title for the research report".to_string(),
                    variable_type: VariableType::String,
                    default_value: Some("Research Analysis Report".to_string()),
                    required: false,
                    validation_pattern: None,
                }),
                ("include_executive_summary".to_string(), TemplateVariable {
                    name: "include_executive_summary".to_string(),
                    description: "Include executive summary section".to_string(),
                    variable_type: VariableType::Boolean,
                    default_value: Some("true".to_string()),
                    required: false,
                    validation_pattern: None,
                }),
            ]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: "system".to_string(),
            is_public: true,
            usage_count: 0,
        };

        // Data Archive Template
        let archive_template = ExportTemplate {
            id: "default_data_archive".to_string(),
            name: "Data Archive".to_string(),
            description: "Complete data archive with all formats and raw data".to_string(),
            category: ExportCategory::Archive,
            options: ExportOptions {
                formats: vec![
                    super::super::OutputFormat::JSON,
                    super::super::OutputFormat::CSV,
                    super::super::OutputFormat::XML,
                ],
                include_charts: true,
                chart_formats: vec![
                    super::super::ChartOutputFormat::SVG,
                    super::super::ChartOutputFormat::HTML,
                ],
                compression: CompressionType::TarGz,
                encryption: None,
                file_naming: FileNamingConfig {
                    pattern: "archive_{workflow_id}_{timestamp}".to_string(),
                    include_timestamp: true,
                    include_workflow_id: true,
                    custom_prefix: Some("data".to_string()),
                    custom_suffix: Some("backup".to_string()),
                },
                package_type: PackageType::Archive,
                include_metadata: true,
                include_raw_data: true,
            },
            variables: HashMap::from([
                ("retention_period".to_string(), TemplateVariable {
                    name: "retention_period".to_string(),
                    description: "Data retention period in days".to_string(),
                    variable_type: VariableType::Number,
                    default_value: Some("365".to_string()),
                    required: false,
                    validation_pattern: Some(r"^\d+$".to_string()),
                }),
                ("compression_level".to_string(), TemplateVariable {
                    name: "compression_level".to_string(),
                    description: "Compression level (1-9)".to_string(),
                    variable_type: VariableType::Choice(vec!["1".to_string(), "5".to_string(), "9".to_string()]),
                    default_value: Some("5".to_string()),
                    required: false,
                    validation_pattern: None,
                }),
            ]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: "system".to_string(),
            is_public: true,
            usage_count: 0,
        };

        // Presentation Template
        let presentation_template = ExportTemplate {
            id: "default_presentation".to_string(),
            name: "Presentation Package".to_string(),
            description: "Presentation-ready export with charts and summaries".to_string(),
            category: ExportCategory::Presentation,
            options: ExportOptions {
                formats: vec![
                    super::super::OutputFormat::HTML,
                    super::super::OutputFormat::Markdown,
                ],
                include_charts: true,
                chart_formats: vec![
                    super::super::ChartOutputFormat::SVG,
                    super::super::ChartOutputFormat::PNG,
                ],
                compression: CompressionType::Zip,
                encryption: None,
                file_naming: FileNamingConfig {
                    pattern: "presentation_{workflow_name}_{timestamp}".to_string(),
                    include_timestamp: true,
                    include_workflow_id: false,
                    custom_prefix: Some("pres".to_string()),
                    custom_suffix: None,
                },
                package_type: PackageType::Archive,
                include_metadata: false,
                include_raw_data: false,
            },
            variables: HashMap::from([
                ("presentation_theme".to_string(), TemplateVariable {
                    name: "presentation_theme".to_string(),
                    description: "Visual theme for presentation".to_string(),
                    variable_type: VariableType::Choice(vec![
                        "professional".to_string(),
                        "modern".to_string(),
                        "minimal".to_string(),
                    ]),
                    default_value: Some("professional".to_string()),
                    required: false,
                    validation_pattern: None,
                }),
                ("slide_count".to_string(), TemplateVariable {
                    name: "slide_count".to_string(),
                    description: "Maximum number of slides to generate".to_string(),
                    variable_type: VariableType::Number,
                    default_value: Some("20".to_string()),
                    required: false,
                    validation_pattern: Some(r"^\d+$".to_string()),
                }),
            ]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: "system".to_string(),
            is_public: true,
            usage_count: 0,
        };

        // Data Exchange Template
        let exchange_template = ExportTemplate {
            id: "default_data_exchange".to_string(),
            name: "Data Exchange".to_string(),
            description: "Structured data export for API integration".to_string(),
            category: ExportCategory::DataExchange,
            options: ExportOptions {
                formats: vec![
                    super::super::OutputFormat::JSON,
                    super::super::OutputFormat::XML,
                ],
                include_charts: false,
                chart_formats: vec![],
                compression: CompressionType::None,
                encryption: None,
                file_naming: FileNamingConfig {
                    pattern: "data_{workflow_id}_{timestamp}".to_string(),
                    include_timestamp: true,
                    include_workflow_id: true,
                    custom_prefix: Some("api".to_string()),
                    custom_suffix: None,
                },
                package_type: PackageType::SingleFile,
                include_metadata: true,
                include_raw_data: true,
            },
            variables: HashMap::from([
                ("api_version".to_string(), TemplateVariable {
                    name: "api_version".to_string(),
                    description: "API version for data format".to_string(),
                    variable_type: VariableType::Choice(vec!["v1".to_string(), "v2".to_string()]),
                    default_value: Some("v2".to_string()),
                    required: true,
                    validation_pattern: None,
                }),
                ("include_schema".to_string(), TemplateVariable {
                    name: "include_schema".to_string(),
                    description: "Include data schema definition".to_string(),
                    variable_type: VariableType::Boolean,
                    default_value: Some("true".to_string()),
                    required: false,
                    validation_pattern: None,
                }),
            ]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: "system".to_string(),
            is_public: true,
            usage_count: 0,
        };

        templates.insert(ExportCategory::Report, report_template);
        templates.insert(ExportCategory::Archive, archive_template);
        templates.insert(ExportCategory::Presentation, presentation_template);
        templates.insert(ExportCategory::DataExchange, exchange_template);

        templates
    }

    /// Load default templates into the manager
    async fn load_default_templates(&self) -> AppResult<()> {
        let mut templates = self.templates.write().await;
        
        for (_, template) in &self.default_templates {
            templates.insert(template.id.clone(), template.clone());
        }

        info!("Loaded {} default export templates", self.default_templates.len());
        Ok(())
    }

    /// Get all templates
    pub async fn get_all_templates(&self) -> AppResult<Vec<ExportTemplate>> {
        let templates = self.templates.read().await;
        Ok(templates.values().cloned().collect())
    }

    /// Get templates by category
    pub async fn get_templates_by_category(&self, category: ExportCategory) -> AppResult<Vec<ExportTemplate>> {
        let templates = self.templates.read().await;
        Ok(templates.values()
            .filter(|t| std::mem::discriminant(&t.category) == std::mem::discriminant(&category))
            .cloned()
            .collect())
    }

    /// Get template by ID
    pub async fn get_template(&self, template_id: &str) -> AppResult<ExportTemplate> {
        let templates = self.templates.read().await;
        templates.get(template_id)
            .cloned()
            .ok_or_else(|| ResearchError::not_found(format!("Export template not found: {}", template_id)).into())
    }

    /// Create new template
    pub async fn create_template(&self, template: ExportTemplate) -> AppResult<()> {
        let mut templates = self.templates.write().await;
        templates.insert(template.id.clone(), template);
        Ok(())
    }

    /// Update existing template
    pub async fn update_template(&self, template_id: &str, updated_template: ExportTemplate) -> AppResult<()> {
        let mut templates = self.templates.write().await;
        
        if templates.contains_key(template_id) {
            templates.insert(template_id.to_string(), updated_template);
            Ok(())
        } else {
            Err(ResearchError::not_found(format!("Export template not found: {}", template_id)).into())
        }
    }

    /// Delete template
    pub async fn delete_template(&self, template_id: &str) -> AppResult<()> {
        let mut templates = self.templates.write().await;
        
        if templates.remove(template_id).is_some() {
            Ok(())
        } else {
            Err(ResearchError::not_found(format!("Export template not found: {}", template_id)).into())
        }
    }

    /// Increment template usage count
    pub async fn increment_usage(&self, template_id: &str) -> AppResult<()> {
        let mut templates = self.templates.write().await;
        
        if let Some(template) = templates.get_mut(template_id) {
            template.usage_count += 1;
            template.updated_at = Utc::now();
            Ok(())
        } else {
            Err(ResearchError::not_found(format!("Export template not found: {}", template_id)).into())
        }
    }

    /// Get default template for category
    pub fn get_default_template(&self, category: ExportCategory) -> Option<&ExportTemplate> {
        self.default_templates.get(&category)
    }

    /// Validate template variables
    pub fn validate_template_variables(&self, template: &ExportTemplate, values: &HashMap<String, String>) -> AppResult<()> {
        for (var_name, var_def) in &template.variables {
            if var_def.required && !values.contains_key(var_name) {
                return Err(ResearchError::invalid_request(
                    format!("Required template variable missing: {}", var_name)
                ).into());
            }

            if let Some(value) = values.get(var_name) {
                // Validate based on variable type
                match &var_def.variable_type {
                    VariableType::Number => {
                        if value.parse::<f64>().is_err() {
                            return Err(ResearchError::invalid_request(
                                format!("Invalid number value for variable {}: {}", var_name, value)
                            ).into());
                        }
                    }
                    VariableType::Boolean => {
                        if !["true", "false"].contains(&value.to_lowercase().as_str()) {
                            return Err(ResearchError::invalid_request(
                                format!("Invalid boolean value for variable {}: {}", var_name, value)
                            ).into());
                        }
                    }
                    VariableType::Choice(choices) => {
                        if !choices.contains(value) {
                            return Err(ResearchError::invalid_request(
                                format!("Invalid choice for variable {}: {}. Valid choices: {:?}", var_name, value, choices)
                            ).into());
                        }
                    }
                    _ => {} // Other types don't need validation for now
                }

                // Validate against pattern if provided (simplified validation for now)
                if let Some(_pattern) = &var_def.validation_pattern {
                    // TODO: Implement regex validation when regex crate is available
                    debug!("Pattern validation not yet implemented for variable: {}", var_name);
                }
            }
        }

        Ok(())
    }
}
