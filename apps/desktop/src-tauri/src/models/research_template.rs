use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::research_workflow::{ResearchMethodology, WorkflowParameters};

/// Template category for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateCategory {
    Academic,
    Business,
    Technical,
    Market,
    Competitive,
    Scientific,
    Legal,
    Medical,
    Financial,
    Custom,
}

/// Template parameter type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Select,
    MultiSelect,
    Date,
    Url,
    Email,
    Text,
}

/// Template parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parameter_type: ParameterType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation_rules: HashMap<String, serde_json::Value>,
    pub options: Option<Vec<String>>, // For Select/MultiSelect types
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub order: u32,
}

impl TemplateParameter {
    /// Create a new template parameter
    pub fn new(
        id: String,
        name: String,
        description: String,
        parameter_type: ParameterType,
        required: bool,
    ) -> Self {
        Self {
            id,
            name,
            description,
            parameter_type,
            required,
            default_value: None,
            validation_rules: HashMap::new(),
            options: None,
            placeholder: None,
            help_text: None,
            order: 0,
        }
    }

    /// Validate a parameter value
    pub fn validate(&self, value: &serde_json::Value) -> Result<(), String> {
        // Check required
        if self.required && value.is_null() {
            return Err(format!("Parameter '{}' is required", self.name));
        }

        if value.is_null() {
            return Ok(());
        }

        // Type validation
        match self.parameter_type {
            ParameterType::String | ParameterType::Url | ParameterType::Email | ParameterType::Text => {
                if !value.is_string() {
                    return Err(format!("Parameter '{}' must be a string", self.name));
                }
            }
            ParameterType::Number => {
                if !value.is_number() {
                    return Err(format!("Parameter '{}' must be a number", self.name));
                }
            }
            ParameterType::Boolean => {
                if !value.is_boolean() {
                    return Err(format!("Parameter '{}' must be a boolean", self.name));
                }
            }
            ParameterType::Select => {
                if let Some(options) = &self.options {
                    if let Some(str_value) = value.as_str() {
                        if !options.contains(&str_value.to_string()) {
                            return Err(format!("Parameter '{}' must be one of: {:?}", self.name, options));
                        }
                    } else {
                        return Err(format!("Parameter '{}' must be a string", self.name));
                    }
                }
            }
            ParameterType::MultiSelect => {
                if let Some(options) = &self.options {
                    if let Some(array_value) = value.as_array() {
                        for item in array_value {
                            if let Some(str_item) = item.as_str() {
                                if !options.contains(&str_item.to_string()) {
                                    return Err(format!("Parameter '{}' contains invalid option: {}", self.name, str_item));
                                }
                            } else {
                                return Err(format!("Parameter '{}' must be an array of strings", self.name));
                            }
                        }
                    } else {
                        return Err(format!("Parameter '{}' must be an array", self.name));
                    }
                }
            }
            ParameterType::Date => {
                if !value.is_string() {
                    return Err(format!("Parameter '{}' must be a date string", self.name));
                }
                // TODO: Add date format validation
            }
        }

        // Custom validation rules
        for (rule, rule_value) in &self.validation_rules {
            match rule.as_str() {
                "min_length" => {
                    if let (Some(str_val), Some(min_len)) = (value.as_str(), rule_value.as_u64()) {
                        if str_val.len() < min_len as usize {
                            return Err(format!("Parameter '{}' must be at least {} characters", self.name, min_len));
                        }
                    }
                }
                "max_length" => {
                    if let (Some(str_val), Some(max_len)) = (value.as_str(), rule_value.as_u64()) {
                        if str_val.len() > max_len as usize {
                            return Err(format!("Parameter '{}' must be at most {} characters", self.name, max_len));
                        }
                    }
                }
                "min_value" => {
                    if let (Some(num_val), Some(min_val)) = (value.as_f64(), rule_value.as_f64()) {
                        if num_val < min_val {
                            return Err(format!("Parameter '{}' must be at least {}", self.name, min_val));
                        }
                    }
                }
                "max_value" => {
                    if let (Some(num_val), Some(max_val)) = (value.as_f64(), rule_value.as_f64()) {
                        if num_val > max_val {
                            return Err(format!("Parameter '{}' must be at most {}", self.name, max_val));
                        }
                    }
                }
                "pattern" => {
                    if let (Some(str_val), Some(pattern)) = (value.as_str(), rule_value.as_str()) {
                        if let Ok(regex) = regex::Regex::new(pattern) {
                            if !regex.is_match(str_val) {
                                return Err(format!("Parameter '{}' does not match required pattern", self.name));
                            }
                        }
                    }
                }
                _ => {} // Unknown validation rule, ignore
            }
        }

        Ok(())
    }
}

/// Template step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStep {
    pub id: String,
    pub name: String,
    pub description: String,
    pub service_provider: Option<String>,
    pub endpoint: Option<String>,
    pub depends_on: Vec<String>,
    pub input_template: HashMap<String, String>, // Template strings with parameter substitution
    pub output_mapping: HashMap<String, String>, // Map output to shared variables
    pub conditional: Option<String>, // Conditional execution based on parameters
    pub retry_count: u32,
    pub timeout_ms: u32,
    pub order: u32,
}

impl TemplateStep {
    /// Create a new template step
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            service_provider: None,
            endpoint: None,
            depends_on: Vec::new(),
            input_template: HashMap::new(),
            output_mapping: HashMap::new(),
            conditional: None,
            retry_count: 3,
            timeout_ms: 30000,
            order: 0,
        }
    }

    /// Substitute parameters in input template
    pub fn substitute_parameters(&self, parameters: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        
        for (key, template) in &self.input_template {
            let substituted = self.substitute_string(template, parameters);
            // Try to parse as JSON, fallback to string
            let value = serde_json::from_str(&substituted)
                .unwrap_or_else(|_| serde_json::Value::String(substituted));
            result.insert(key.clone(), value);
        }
        
        result
    }

    /// Substitute parameters in a template string
    fn substitute_string(&self, template: &str, parameters: &HashMap<String, serde_json::Value>) -> String {
        let mut result = template.to_string();
        
        for (param_name, param_value) in parameters {
            let placeholder = format!("{{{{{}}}}}", param_name);
            let value_str = match param_value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => serde_json::to_string(param_value).unwrap_or_default(),
            };
            result = result.replace(&placeholder, &value_str);
        }
        
        result
    }

    /// Check if step should be executed based on conditional
    pub fn should_execute(&self, parameters: &HashMap<String, serde_json::Value>) -> bool {
        if let Some(conditional) = &self.conditional {
            // Simple conditional evaluation (can be enhanced)
            // Format: "parameter_name == value" or "parameter_name != value"
            if let Some((param, condition)) = conditional.split_once("==") {
                let param = param.trim();
                let expected_value = condition.trim().trim_matches('"');
                if let Some(actual_value) = parameters.get(param) {
                    return actual_value.as_str() == Some(expected_value);
                }
                return false;
            } else if let Some((param, condition)) = conditional.split_once("!=") {
                let param = param.trim();
                let expected_value = condition.trim().trim_matches('"');
                if let Some(actual_value) = parameters.get(param) {
                    return actual_value.as_str() != Some(expected_value);
                }
                return true;
            }
        }
        true
    }
}

/// Research template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub version: String,
    pub methodology: ResearchMethodology,
    pub parameters: Vec<TemplateParameter>,
    pub steps: Vec<TemplateStep>,
    pub workflow_config: WorkflowParameters,
    pub tags: Vec<String>,
    pub author: String,
    pub organization: Option<String>,
    pub is_public: bool,
    pub is_featured: bool,
    pub usage_count: u32,
    pub rating: f64,
    pub rating_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ResearchTemplate {
    /// Create a new research template
    pub fn new(
        name: String,
        description: String,
        category: TemplateCategory,
        methodology: ResearchMethodology,
        author: String,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            category,
            version: "1.0.0".to_string(),
            methodology,
            parameters: Vec::new(),
            steps: Vec::new(),
            workflow_config: WorkflowParameters::default(),
            tags: Vec::new(),
            author,
            organization: None,
            is_public: false,
            is_featured: false,
            usage_count: 0,
            rating: 0.0,
            rating_count: 0,
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    /// Add a parameter to the template
    pub fn add_parameter(&mut self, parameter: TemplateParameter) {
        self.parameters.push(parameter);
        self.updated_at = Utc::now();
    }

    /// Add a step to the template
    pub fn add_step(&mut self, step: TemplateStep) {
        self.steps.push(step);
        self.updated_at = Utc::now();
    }

    /// Validate template parameters
    pub fn validate_parameters(&self, parameters: &HashMap<String, serde_json::Value>) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        for template_param in &self.parameters {
            let value = parameters.get(&template_param.id)
                .unwrap_or(&serde_json::Value::Null);
            
            if let Err(error) = template_param.validate(value) {
                errors.push(error);
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get parameters with default values applied
    pub fn get_parameters_with_defaults(&self, provided: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        
        for template_param in &self.parameters {
            if let Some(value) = provided.get(&template_param.id) {
                result.insert(template_param.id.clone(), value.clone());
            } else if let Some(default_value) = &template_param.default_value {
                result.insert(template_param.id.clone(), default_value.clone());
            }
        }
        
        result
    }

    /// Update rating
    pub fn update_rating(&mut self, new_rating: f64) {
        let total_rating = self.rating * self.rating_count as f64 + new_rating;
        self.rating_count += 1;
        self.rating = total_rating / self.rating_count as f64;
        self.updated_at = Utc::now();
    }

    /// Increment usage count
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
        self.updated_at = Utc::now();
    }

    /// Check if template is compatible with methodology
    pub fn is_compatible_with_methodology(&self, methodology: &ResearchMethodology) -> bool {
        self.methodology == *methodology || self.methodology == ResearchMethodology::Hybrid
    }
}

/// Template execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateExecutionContext {
    pub template_id: Uuid,
    pub parameters: HashMap<String, serde_json::Value>,
    pub workflow_name: String,
    pub created_by: String,
    pub execution_metadata: HashMap<String, serde_json::Value>,
}

/// Template performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetrics {
    pub template_id: Uuid,
    pub total_executions: u32,
    pub successful_executions: u32,
    pub failed_executions: u32,
    pub average_execution_time_ms: f64,
    pub success_rate: f64,
    pub last_executed: Option<DateTime<Utc>>,
    pub performance_score: f64,
}

impl TemplateMetrics {
    /// Create new metrics for a template
    pub fn new(template_id: Uuid) -> Self {
        Self {
            template_id,
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            average_execution_time_ms: 0.0,
            success_rate: 100.0,
            last_executed: None,
            performance_score: 0.0,
        }
    }

    /// Update metrics after execution
    pub fn update_after_execution(&mut self, success: bool, execution_time_ms: u32) {
        self.total_executions += 1;
        self.last_executed = Some(Utc::now());

        if success {
            self.successful_executions += 1;
        } else {
            self.failed_executions += 1;
        }

        // Update average execution time
        if self.total_executions == 1 {
            self.average_execution_time_ms = execution_time_ms as f64;
        } else {
            self.average_execution_time_ms = 
                (self.average_execution_time_ms * (self.total_executions - 1) as f64 + execution_time_ms as f64) 
                / self.total_executions as f64;
        }

        // Update success rate
        self.success_rate = (self.successful_executions as f64 / self.total_executions as f64) * 100.0;

        // Calculate performance score (0-100)
        let time_score = if self.average_execution_time_ms > 0.0 {
            (300000.0 / self.average_execution_time_ms).min(100.0) // 5 minutes = 100 points
        } else {
            100.0
        };
        
        self.performance_score = (self.success_rate * 0.7) + (time_score * 0.3);
    }
}
