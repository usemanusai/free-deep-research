use std::collections::HashMap;
use tracing::{info, debug};

use crate::error::AppResult;
use crate::models::research_template::{
    ResearchTemplate, TemplateCategory, TemplateParameter, TemplateStep, ParameterType
};
use crate::models::research_workflow::{ResearchMethodology, WorkflowParameters};

/// Template builder for creating research templates programmatically
pub struct TemplateBuilder {
    template: ResearchTemplate,
}

impl TemplateBuilder {
    /// Create a new template builder
    pub fn new(
        name: String,
        description: String,
        category: TemplateCategory,
        methodology: ResearchMethodology,
        author: String,
    ) -> Self {
        let template = ResearchTemplate::new(name, description, category, methodology, author);
        
        Self { template }
    }

    /// Set template version
    pub fn version(mut self, version: String) -> Self {
        self.template.version = version;
        self
    }

    /// Set template organization
    pub fn organization(mut self, organization: String) -> Self {
        self.template.organization = Some(organization);
        self
    }

    /// Make template public
    pub fn public(mut self) -> Self {
        self.template.is_public = true;
        self
    }

    /// Make template featured
    pub fn featured(mut self) -> Self {
        self.template.is_featured = true;
        self
    }

    /// Add tags to template
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.template.tags = tags;
        self
    }

    /// Add metadata to template
    pub fn metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.template.metadata.insert(key, value);
        self
    }

    /// Set workflow configuration
    pub fn workflow_config(mut self, config: WorkflowParameters) -> Self {
        self.template.workflow_config = config;
        self
    }

    /// Add a string parameter
    pub fn add_string_parameter(
        mut self,
        id: String,
        name: String,
        description: String,
        required: bool,
    ) -> Self {
        let mut param = TemplateParameter::new(id, name, description, ParameterType::String, required);
        param.order = self.template.parameters.len() as u32;
        self.template.add_parameter(param);
        self
    }

    /// Add a string parameter with validation
    pub fn add_string_parameter_with_validation(
        mut self,
        id: String,
        name: String,
        description: String,
        required: bool,
        min_length: Option<u32>,
        max_length: Option<u32>,
        pattern: Option<String>,
    ) -> Self {
        let mut param = TemplateParameter::new(id, name, description, ParameterType::String, required);
        param.order = self.template.parameters.len() as u32;
        
        if let Some(min) = min_length {
            param.validation_rules.insert("min_length".to_string(), serde_json::Value::Number(serde_json::Number::from(min)));
        }
        if let Some(max) = max_length {
            param.validation_rules.insert("max_length".to_string(), serde_json::Value::Number(serde_json::Number::from(max)));
        }
        if let Some(pat) = pattern {
            param.validation_rules.insert("pattern".to_string(), serde_json::Value::String(pat));
        }
        
        self.template.add_parameter(param);
        self
    }

    /// Add a number parameter
    pub fn add_number_parameter(
        mut self,
        id: String,
        name: String,
        description: String,
        required: bool,
        min_value: Option<f64>,
        max_value: Option<f64>,
    ) -> Self {
        let mut param = TemplateParameter::new(id, name, description, ParameterType::Number, required);
        param.order = self.template.parameters.len() as u32;
        
        if let Some(min) = min_value {
            param.validation_rules.insert("min_value".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(min).unwrap()));
        }
        if let Some(max) = max_value {
            param.validation_rules.insert("max_value".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(max).unwrap()));
        }
        
        self.template.add_parameter(param);
        self
    }

    /// Add a boolean parameter
    pub fn add_boolean_parameter(
        mut self,
        id: String,
        name: String,
        description: String,
        default_value: bool,
    ) -> Self {
        let mut param = TemplateParameter::new(id, name, description, ParameterType::Boolean, false);
        param.order = self.template.parameters.len() as u32;
        param.default_value = Some(serde_json::Value::Bool(default_value));
        self.template.add_parameter(param);
        self
    }

    /// Add a select parameter
    pub fn add_select_parameter(
        mut self,
        id: String,
        name: String,
        description: String,
        required: bool,
        options: Vec<String>,
        default_value: Option<String>,
    ) -> Self {
        let mut param = TemplateParameter::new(id, name, description, ParameterType::Select, required);
        param.order = self.template.parameters.len() as u32;
        param.options = Some(options);
        
        if let Some(default) = default_value {
            param.default_value = Some(serde_json::Value::String(default));
        }
        
        self.template.add_parameter(param);
        self
    }

    /// Add a multi-select parameter
    pub fn add_multi_select_parameter(
        mut self,
        id: String,
        name: String,
        description: String,
        required: bool,
        options: Vec<String>,
        default_values: Option<Vec<String>>,
    ) -> Self {
        let mut param = TemplateParameter::new(id, name, description, ParameterType::MultiSelect, required);
        param.order = self.template.parameters.len() as u32;
        param.options = Some(options);
        
        if let Some(defaults) = default_values {
            param.default_value = Some(serde_json::Value::Array(
                defaults.into_iter().map(serde_json::Value::String).collect()
            ));
        }
        
        self.template.add_parameter(param);
        self
    }

    /// Add a text area parameter
    pub fn add_text_parameter(
        mut self,
        id: String,
        name: String,
        description: String,
        required: bool,
        placeholder: Option<String>,
    ) -> Self {
        let mut param = TemplateParameter::new(id, name, description, ParameterType::Text, required);
        param.order = self.template.parameters.len() as u32;
        param.placeholder = placeholder;
        self.template.add_parameter(param);
        self
    }

    /// Add a URL parameter
    pub fn add_url_parameter(
        mut self,
        id: String,
        name: String,
        description: String,
        required: bool,
    ) -> Self {
        let mut param = TemplateParameter::new(id, name, description, ParameterType::Url, required);
        param.order = self.template.parameters.len() as u32;
        param.validation_rules.insert(
            "pattern".to_string(),
            serde_json::Value::String(r"^https?://.*".to_string())
        );
        self.template.add_parameter(param);
        self
    }

    /// Add a step to the template
    pub fn add_step(
        mut self,
        id: String,
        name: String,
        description: String,
        service_provider: Option<String>,
        endpoint: Option<String>,
    ) -> Self {
        let mut step = TemplateStep::new(id, name, description);
        step.service_provider = service_provider;
        step.endpoint = endpoint;
        step.order = self.template.steps.len() as u32;
        self.template.add_step(step);
        self
    }

    /// Add a step with dependencies
    pub fn add_step_with_dependencies(
        mut self,
        id: String,
        name: String,
        description: String,
        service_provider: Option<String>,
        endpoint: Option<String>,
        depends_on: Vec<String>,
    ) -> Self {
        let mut step = TemplateStep::new(id, name, description);
        step.service_provider = service_provider;
        step.endpoint = endpoint;
        step.depends_on = depends_on;
        step.order = self.template.steps.len() as u32;
        self.template.add_step(step);
        self
    }

    /// Add input template to the last step
    pub fn add_step_input(mut self, key: String, template: String) -> Self {
        if let Some(last_step) = self.template.steps.last_mut() {
            last_step.input_template.insert(key, template);
        }
        self
    }

    /// Add output mapping to the last step
    pub fn add_step_output_mapping(mut self, output_key: String, variable_name: String) -> Self {
        if let Some(last_step) = self.template.steps.last_mut() {
            last_step.output_mapping.insert(output_key, variable_name);
        }
        self
    }

    /// Add conditional execution to the last step
    pub fn add_step_conditional(mut self, conditional: String) -> Self {
        if let Some(last_step) = self.template.steps.last_mut() {
            last_step.conditional = Some(conditional);
        }
        self
    }

    /// Set step timeout
    pub fn set_step_timeout(mut self, timeout_ms: u32) -> Self {
        if let Some(last_step) = self.template.steps.last_mut() {
            last_step.timeout_ms = timeout_ms;
        }
        self
    }

    /// Set step retry count
    pub fn set_step_retry_count(mut self, retry_count: u32) -> Self {
        if let Some(last_step) = self.template.steps.last_mut() {
            last_step.retry_count = retry_count;
        }
        self
    }

    /// Build the template
    pub fn build(self) -> ResearchTemplate {
        debug!("Built template '{}' with {} parameters and {} steps", 
               self.template.name, self.template.parameters.len(), self.template.steps.len());
        self.template
    }

    /// Validate and build the template
    pub fn validate_and_build(self) -> AppResult<ResearchTemplate> {
        // Validate template structure
        if self.template.name.trim().is_empty() {
            return Err(crate::error::ApiError::invalid_input(
                "Template name cannot be empty".to_string()
            ));
        }

        if self.template.description.trim().is_empty() {
            return Err(crate::error::ApiError::invalid_input(
                "Template description cannot be empty".to_string()
            ));
        }

        if self.template.steps.is_empty() {
            return Err(crate::error::ApiError::invalid_input(
                "Template must have at least one step".to_string()
            ));
        }

        // Validate parameter IDs are unique
        let mut param_ids = std::collections::HashSet::new();
        for param in &self.template.parameters {
            if !param_ids.insert(&param.id) {
                return Err(crate::error::ApiError::invalid_input(
                    format!("Duplicate parameter ID: {}", param.id)
                ));
            }
        }

        // Validate step IDs are unique
        let mut step_ids = std::collections::HashSet::new();
        for step in &self.template.steps {
            if !step_ids.insert(&step.id) {
                return Err(crate::error::ApiError::invalid_input(
                    format!("Duplicate step ID: {}", step.id)
                ));
            }
        }

        // Validate step dependencies
        let step_id_set: std::collections::HashSet<_> = self.template.steps.iter().map(|s| &s.id).collect();
        for step in &self.template.steps {
            for dep in &step.depends_on {
                if !step_id_set.contains(dep) {
                    return Err(crate::error::ApiError::invalid_input(
                        format!("Step '{}' depends on non-existent step '{}'", step.id, dep)
                    ));
                }
            }
        }

        info!("Successfully validated template '{}'", self.template.name);
        Ok(self.template)
    }
}

impl Default for TemplateBuilder {
    fn default() -> Self {
        Self::new(
            "New Template".to_string(),
            "Template description".to_string(),
            TemplateCategory::Custom,
            ResearchMethodology::Hybrid,
            "system".to_string(),
        )
    }
}
