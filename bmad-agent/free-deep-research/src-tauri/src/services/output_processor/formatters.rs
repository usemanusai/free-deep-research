use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, debug, error};
use chrono::Utc;
use serde_json;

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, ResearchResults, StepStatus};
use super::{OutputOptions, OutputTemplate};

/// Trait for output formatters
#[async_trait]
pub trait OutputFormatter: Send + Sync {
    /// Format research workflow results
    async fn format(
        &self,
        workflow: &ResearchWorkflow,
        template: Option<&OutputTemplate>,
        options: &OutputOptions,
    ) -> AppResult<String>;

    /// Get supported file extension
    fn file_extension(&self) -> &'static str;

    /// Get MIME type
    fn mime_type(&self) -> &'static str;
}

/// Markdown formatter
pub struct MarkdownFormatter;

impl MarkdownFormatter {
    pub fn new() -> Self {
        Self
    }

    fn format_workflow_header(&self, workflow: &ResearchWorkflow) -> String {
        format!(
            "# Research Report: {}\n\n**Query:** {}\n\n**Status:** {:?}\n\n**Created:** {}\n\n",
            workflow.name,
            workflow.query,
            workflow.status,
            workflow.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }

    fn format_workflow_steps(&self, workflow: &ResearchWorkflow) -> String {
        let mut content = String::from("## Research Steps\n\n");
        
        for (index, step) in workflow.steps.iter().enumerate() {
            let status_icon = match step.status {
                StepStatus::Completed => "✅",
                StepStatus::Running => "🔄",
                StepStatus::Failed => "❌",
                StepStatus::Pending => "⏳",
            };

            content.push_str(&format!(
                "### {} Step {}: {}\n\n**Status:** {} {:?}\n\n",
                status_icon,
                index + 1,
                step.step_type,
                status_icon,
                step.status
            ));

            if let Some(result) = &step.result {
                content.push_str("**Result:**\n```json\n");
                content.push_str(&serde_json::to_string_pretty(result).unwrap_or_else(|_| "Invalid JSON".to_string()));
                content.push_str("\n```\n\n");
            }

            if let Some(error) = &step.error {
                content.push_str(&format!("**Error:** {}\n\n", error));
            }
        }

        content
    }

    fn format_workflow_results(&self, workflow: &ResearchWorkflow) -> String {
        let mut content = String::from("## Results\n\n");

        if let Some(results) = &workflow.results {
            content.push_str("### Summary\n\n");
            content.push_str(&results.summary);
            content.push_str("\n\n");

            if !results.key_findings.is_empty() {
                content.push_str("### Key Findings\n\n");
                for (index, finding) in results.key_findings.iter().enumerate() {
                    content.push_str(&format!("{}. {}\n", index + 1, finding));
                }
                content.push_str("\n");
            }

            if !results.sources.is_empty() {
                content.push_str("### Sources\n\n");
                for (index, source) in results.sources.iter().enumerate() {
                    content.push_str(&format!("{}. [{}]({})\n", index + 1, source.title, source.url));
                }
                content.push_str("\n");
            }

            if let Some(raw_data) = &results.raw_data {
                content.push_str("### Raw Data\n\n```json\n");
                content.push_str(&serde_json::to_string_pretty(raw_data).unwrap_or_else(|_| "Invalid JSON".to_string()));
                content.push_str("\n```\n\n");
            }
        } else {
            content.push_str("No results available.\n\n");
        }

        content
    }
}

#[async_trait]
impl OutputFormatter for MarkdownFormatter {
    async fn format(
        &self,
        workflow: &ResearchWorkflow,
        template: Option<&OutputTemplate>,
        options: &OutputOptions,
    ) -> AppResult<String> {
        debug!("Formatting workflow {} as Markdown", workflow.id);

        if let Some(template) = template {
            // Use template-based formatting
            let mut content = template.content.clone();
            
            // Replace template variables
            content = content.replace("{{workflow_name}}", &workflow.name);
            content = content.replace("{{workflow_query}}", &workflow.query);
            content = content.replace("{{workflow_status}}", &format!("{:?}", workflow.status));
            content = content.replace("{{created_at}}", &workflow.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string());
            
            if let Some(results) = &workflow.results {
                content = content.replace("{{summary}}", &results.summary);
                content = content.replace("{{key_findings}}", &results.key_findings.join("\n- "));
            }

            Ok(content)
        } else {
            // Use default formatting
            let mut content = String::new();

            if options.include_metadata {
                content.push_str(&self.format_workflow_header(workflow));
            }

            content.push_str(&self.format_workflow_steps(workflow));
            content.push_str(&self.format_workflow_results(workflow));

            if options.include_raw_data {
                content.push_str("## Raw Workflow Data\n\n```json\n");
                content.push_str(&serde_json::to_string_pretty(workflow).unwrap_or_else(|_| "Invalid JSON".to_string()));
                content.push_str("\n```\n\n");
            }

            content.push_str(&format!("---\n\n*Generated on {} by Research Engine*\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

            Ok(content)
        }
    }

    fn file_extension(&self) -> &'static str {
        "md"
    }

    fn mime_type(&self) -> &'static str {
        "text/markdown"
    }
}

/// HTML formatter
pub struct HTMLFormatter;

impl HTMLFormatter {
    pub fn new() -> Self {
        Self
    }

    fn get_html_template(&self, options: &OutputOptions) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <style>
        body {{
            font-family: {};
            font-size: {}px;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: {};
            color: {};
        }}
        h1, h2, h3 {{ color: #333; }}
        .metadata {{ background: #f5f5f5; padding: 15px; border-radius: 5px; margin-bottom: 20px; }}
        .step {{ margin-bottom: 20px; padding: 15px; border-left: 4px solid #007acc; background: #f9f9f9; }}
        .step.completed {{ border-left-color: #28a745; }}
        .step.failed {{ border-left-color: #dc3545; }}
        .step.running {{ border-left-color: #ffc107; }}
        .results {{ background: #e8f5e8; padding: 20px; border-radius: 5px; }}
        pre {{ background: #f8f8f8; padding: 10px; border-radius: 3px; overflow-x: auto; }}
        .footer {{ margin-top: 40px; text-align: center; color: #666; font-size: 12px; }}
        {}
    </style>
</head>
<body>
    {{content}}
    <div class="footer">Generated on {{timestamp}} by Research Engine</div>
</body>
</html>"#,
            options.styling.font_family,
            options.styling.font_size,
            if options.styling.color_scheme == "dark" { "#1a1a1a" } else { "#ffffff" },
            if options.styling.color_scheme == "dark" { "#ffffff" } else { "#333333" },
            options.styling.custom_css.as_deref().unwrap_or("")
        )
    }
}

#[async_trait]
impl OutputFormatter for HTMLFormatter {
    async fn format(
        &self,
        workflow: &ResearchWorkflow,
        template: Option<&OutputTemplate>,
        options: &OutputOptions,
    ) -> AppResult<String> {
        debug!("Formatting workflow {} as HTML", workflow.id);

        let mut content = String::new();

        if options.include_metadata {
            content.push_str(&format!(
                r#"<div class="metadata">
                    <h1>{}</h1>
                    <p><strong>Query:</strong> {}</p>
                    <p><strong>Status:</strong> {:?}</p>
                    <p><strong>Created:</strong> {}</p>
                </div>"#,
                workflow.name,
                workflow.query,
                workflow.status,
                workflow.created_at.format("%Y-%m-%d %H:%M:%S UTC")
            ));
        }

        content.push_str("<h2>Research Steps</h2>");
        for (index, step) in workflow.steps.iter().enumerate() {
            let step_class = match step.status {
                StepStatus::Completed => "step completed",
                StepStatus::Running => "step running",
                StepStatus::Failed => "step failed",
                StepStatus::Pending => "step",
            };

            content.push_str(&format!(
                r#"<div class="{}">
                    <h3>Step {}: {}</h3>
                    <p><strong>Status:</strong> {:?}</p>"#,
                step_class,
                index + 1,
                step.step_type,
                step.status
            ));

            if let Some(result) = &step.result {
                content.push_str("<p><strong>Result:</strong></p><pre>");
                content.push_str(&serde_json::to_string_pretty(result).unwrap_or_else(|_| "Invalid JSON".to_string()));
                content.push_str("</pre>");
            }

            if let Some(error) = &step.error {
                content.push_str(&format!("<p><strong>Error:</strong> {}</p>", error));
            }

            content.push_str("</div>");
        }

        if let Some(results) = &workflow.results {
            content.push_str(r#"<div class="results"><h2>Results</h2>"#);
            content.push_str(&format!("<h3>Summary</h3><p>{}</p>", results.summary));

            if !results.key_findings.is_empty() {
                content.push_str("<h3>Key Findings</h3><ul>");
                for finding in &results.key_findings {
                    content.push_str(&format!("<li>{}</li>", finding));
                }
                content.push_str("</ul>");
            }

            if !results.sources.is_empty() {
                content.push_str("<h3>Sources</h3><ol>");
                for source in &results.sources {
                    content.push_str(&format!(r#"<li><a href="{}" target="_blank">{}</a></li>"#, source.url, source.title));
                }
                content.push_str("</ol>");
            }

            content.push_str("</div>");
        }

        let html_template = self.get_html_template(options);
        let final_html = html_template
            .replace("{{title}}", &workflow.name)
            .replace("{{content}}", &content)
            .replace("{{timestamp}}", &Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string());

        Ok(final_html)
    }

    fn file_extension(&self) -> &'static str {
        "html"
    }

    fn mime_type(&self) -> &'static str {
        "text/html"
    }
}

/// JSON formatter
pub struct JSONFormatter;

impl JSONFormatter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OutputFormatter for JSONFormatter {
    async fn format(
        &self,
        workflow: &ResearchWorkflow,
        _template: Option<&OutputTemplate>,
        options: &OutputOptions,
    ) -> AppResult<String> {
        debug!("Formatting workflow {} as JSON", workflow.id);

        let mut output = serde_json::Map::new();

        if options.include_metadata {
            output.insert("metadata".to_string(), serde_json::json!({
                "workflow_id": workflow.id,
                "name": workflow.name,
                "query": workflow.query,
                "status": workflow.status,
                "created_at": workflow.created_at,
                "updated_at": workflow.updated_at,
                "started_at": workflow.started_at,
                "completed_at": workflow.completed_at
            }));
        }

        output.insert("steps".to_string(), serde_json::to_value(&workflow.steps)?);

        if let Some(results) = &workflow.results {
            output.insert("results".to_string(), serde_json::to_value(results)?);
        }

        if options.include_raw_data {
            output.insert("raw_workflow".to_string(), serde_json::to_value(workflow)?);
        }

        output.insert("generated_at".to_string(), serde_json::Value::String(Utc::now().to_rfc3339()));
        output.insert("generator".to_string(), serde_json::Value::String("Research Engine".to_string()));

        Ok(serde_json::to_string_pretty(&output)?)
    }

    fn file_extension(&self) -> &'static str {
        "json"
    }

    fn mime_type(&self) -> &'static str {
        "application/json"
    }
}
