use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use serde_json;
use chrono::Utc;

use crate::error::{AppResult, ResearchError};
use crate::services::{ApiManagerService, DataPersistenceService};
use crate::models::research_workflow::{ResearchResult, Source, SourceType};

/// Result processor for compiling and analyzing research results
pub struct ResultProcessor {
    api_manager: Arc<RwLock<ApiManagerService>>,
    data_persistence: Arc<RwLock<DataPersistenceService>>,
}

impl ResultProcessor {
    /// Create a new result processor
    pub async fn new(
        api_manager: Arc<RwLock<ApiManagerService>>,
        data_persistence: Arc<RwLock<DataPersistenceService>>,
    ) -> AppResult<Self> {
        info!("Initializing result processor");
        
        let processor = Self {
            api_manager,
            data_persistence,
        };
        
        info!("Result processor initialized successfully");
        Ok(processor)
    }
    
    /// Process and compile research results
    pub async fn process_results(
        &self,
        step_results: &[serde_json::Value],
        query: &str,
        methodology: &str,
    ) -> AppResult<ResearchResult> {
        info!("Processing research results for query: '{}'", query);
        debug!("Processing {} step results using methodology: {}", step_results.len(), methodology);
        
        // Extract sources from all steps
        let sources = self.extract_sources(step_results).await?;
        
        // Compile findings
        let detailed_findings = self.compile_findings(step_results, methodology).await?;
        
        // Generate summary
        let summary = self.generate_summary(&detailed_findings, query).await?;
        
        // Calculate confidence score
        let confidence = self.calculate_confidence_score(step_results, &sources).await?;
        
        // Create metadata
        let metadata = self.create_metadata(step_results, methodology).await?;
        
        let result = ResearchResult {
            summary,
            detailed_findings,
            sources,
            confidence,
            metadata,
        };
        
        info!("Research results processed successfully (confidence: {:.2})", confidence);
        Ok(result)
    }
    
    /// Extract sources from step results
    async fn extract_sources(&self, step_results: &[serde_json::Value]) -> AppResult<Vec<Source>> {
        debug!("Extracting sources from step results");
        
        let mut sources = Vec::new();
        
        for (step_index, result) in step_results.iter().enumerate() {
            if let Some(step_sources) = result.get("results").and_then(|r| r.as_array()) {
                for source_data in step_sources {
                    if let Some(source) = self.parse_source(source_data, step_index).await? {
                        sources.push(source);
                    }
                }
            }
            
            // Also check for direct source information
            if let Some(url) = result.get("url").and_then(|u| u.as_str()) {
                sources.push(Source {
                    id: uuid::Uuid::new_v4(),
                    title: result.get("title")
                        .and_then(|t| t.as_str())
                        .unwrap_or("Unknown Title")
                        .to_string(),
                    url: url.to_string(),
                    source_type: self.determine_source_type(url),
                    content_snippet: result.get("snippet")
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string()),
                    relevance_score: result.get("relevance")
                        .and_then(|r| r.as_f64())
                        .unwrap_or(0.5),
                    accessed_at: Utc::now(),
                    metadata: result.clone(),
                });
            }
        }
        
        // Remove duplicates based on URL
        sources.sort_by(|a, b| a.url.cmp(&b.url));
        sources.dedup_by(|a, b| a.url == b.url);
        
        debug!("Extracted {} unique sources", sources.len());
        Ok(sources)
    }
    
    /// Parse a source from JSON data
    async fn parse_source(&self, source_data: &serde_json::Value, step_index: usize) -> AppResult<Option<Source>> {
        if let Some(url) = source_data.get("url").and_then(|u| u.as_str()) {
            Ok(Some(Source {
                id: uuid::Uuid::new_v4(),
                title: source_data.get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown Title")
                    .to_string(),
                url: url.to_string(),
                source_type: self.determine_source_type(url),
                content_snippet: source_data.get("snippet")
                    .or_else(|| source_data.get("description"))
                    .and_then(|s| s.as_str())
                    .map(|s| s.to_string()),
                relevance_score: source_data.get("relevance")
                    .or_else(|| source_data.get("score"))
                    .and_then(|r| r.as_f64())
                    .unwrap_or(0.5),
                accessed_at: Utc::now(),
                metadata: source_data.clone(),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Determine source type based on URL
    fn determine_source_type(&self, url: &str) -> SourceType {
        let url_lower = url.to_lowercase();
        
        if url_lower.contains("arxiv.org") || url_lower.contains("scholar.google") || url_lower.contains("pubmed") {
            SourceType::Academic
        } else if url_lower.contains("news") || url_lower.contains("reuters") || url_lower.contains("bloomberg") {
            SourceType::News
        } else if url_lower.contains("blog") || url_lower.contains("medium.com") {
            SourceType::Blog
        } else if url_lower.contains("gov") || url_lower.contains("official") {
            SourceType::Government
        } else {
            SourceType::Web
        }
    }
    
    /// Compile findings from step results
    async fn compile_findings(&self, step_results: &[serde_json::Value], methodology: &str) -> AppResult<String> {
        debug!("Compiling findings using methodology: {}", methodology);
        
        let mut findings = Vec::new();
        
        for (step_index, result) in step_results.iter().enumerate() {
            let step_type = result.get("step_type")
                .and_then(|t| t.as_str())
                .unwrap_or("unknown");
            
            let finding = match step_type {
                "web_search" | "academic_search" => {
                    self.compile_search_findings(result, step_index).await?
                }
                "content_extraction" => {
                    self.compile_extraction_findings(result, step_index).await?
                }
                "ai_analysis" | "ai_summary" | "academic_analysis" => {
                    self.compile_analysis_findings(result, step_index).await?
                }
                _ => {
                    format!("Step {}: {} - {}", 
                        step_index + 1, 
                        step_type, 
                        result.get("message").and_then(|m| m.as_str()).unwrap_or("No details available")
                    )
                }
            };
            
            findings.push(finding);
        }
        
        let compiled = findings.join("\n\n");
        debug!("Compiled findings: {} characters", compiled.len());
        Ok(compiled)
    }
    
    /// Compile search findings
    async fn compile_search_findings(&self, result: &serde_json::Value, step_index: usize) -> AppResult<String> {
        let provider = result.get("provider").and_then(|p| p.as_str()).unwrap_or("unknown");
        let results_count = result.get("results")
            .and_then(|r| r.as_array())
            .map(|a| a.len())
            .unwrap_or(0);
        
        Ok(format!("Step {}: Search Results ({})\n- Found {} results\n- Provider: {}", 
            step_index + 1, provider, results_count, provider))
    }
    
    /// Compile extraction findings
    async fn compile_extraction_findings(&self, result: &serde_json::Value, step_index: usize) -> AppResult<String> {
        let provider = result.get("provider").and_then(|p| p.as_str()).unwrap_or("unknown");
        let content_count = result.get("extracted_content")
            .and_then(|c| c.as_array())
            .map(|a| a.len())
            .unwrap_or(0);
        
        Ok(format!("Step {}: Content Extraction ({})\n- Extracted {} content pieces\n- Provider: {}", 
            step_index + 1, provider, content_count, provider))
    }
    
    /// Compile analysis findings
    async fn compile_analysis_findings(&self, result: &serde_json::Value, step_index: usize) -> AppResult<String> {
        let provider = result.get("provider").and_then(|p| p.as_str()).unwrap_or("unknown");
        let analysis = result.get("analysis")
            .or_else(|| result.get("summary"))
            .and_then(|a| a.as_str())
            .unwrap_or("No analysis available");
        
        Ok(format!("Step {}: AI Analysis ({})\n{}", 
            step_index + 1, provider, analysis))
    }
    
    /// Generate summary from detailed findings
    async fn generate_summary(&self, detailed_findings: &str, query: &str) -> AppResult<String> {
        debug!("Generating summary for query: '{}'", query);
        
        // TODO: Use AI to generate intelligent summary
        // For now, create a basic summary
        let word_count = detailed_findings.split_whitespace().count();
        let summary = format!(
            "Research completed for query: '{}'. Analysis includes {} words of detailed findings across multiple research steps.",
            query, word_count
        );
        
        debug!("Generated summary: {} characters", summary.len());
        Ok(summary)
    }
    
    /// Calculate confidence score based on results quality
    async fn calculate_confidence_score(&self, step_results: &[serde_json::Value], sources: &[Source]) -> AppResult<f64> {
        debug!("Calculating confidence score");
        
        let mut confidence_factors = Vec::new();
        
        // Factor 1: Number of successful steps
        let successful_steps = step_results.iter()
            .filter(|r| r.get("error").is_none())
            .count();
        let step_success_rate = successful_steps as f64 / step_results.len() as f64;
        confidence_factors.push(step_success_rate * 0.3);
        
        // Factor 2: Number and quality of sources
        let source_quality = if sources.is_empty() {
            0.0
        } else {
            let avg_relevance = sources.iter()
                .map(|s| s.relevance_score)
                .sum::<f64>() / sources.len() as f64;
            let source_count_factor = (sources.len() as f64 / 10.0).min(1.0);
            avg_relevance * source_count_factor * 0.4
        };
        confidence_factors.push(source_quality);
        
        // Factor 3: Diversity of source types
        let unique_source_types = sources.iter()
            .map(|s| &s.source_type)
            .collect::<std::collections::HashSet<_>>()
            .len();
        let diversity_factor = (unique_source_types as f64 / 5.0).min(1.0) * 0.2;
        confidence_factors.push(diversity_factor);
        
        // Factor 4: Presence of analysis steps
        let has_analysis = step_results.iter()
            .any(|r| r.get("analysis").is_some() || r.get("summary").is_some());
        if has_analysis {
            confidence_factors.push(0.1);
        }
        
        let total_confidence = confidence_factors.iter().sum::<f64>();
        let final_confidence = total_confidence.min(1.0).max(0.0);
        
        debug!("Calculated confidence score: {:.3}", final_confidence);
        Ok(final_confidence)
    }
    
    /// Create metadata for the research result
    async fn create_metadata(&self, step_results: &[serde_json::Value], methodology: &str) -> AppResult<serde_json::Value> {
        debug!("Creating result metadata");
        
        let metadata = serde_json::json!({
            "methodology": methodology,
            "steps_executed": step_results.len(),
            "processing_timestamp": Utc::now().to_rfc3339(),
            "processor_version": "1.0.0",
            "step_summary": step_results.iter().enumerate().map(|(i, r)| {
                serde_json::json!({
                    "step_index": i,
                    "step_type": r.get("step_type").and_then(|t| t.as_str()).unwrap_or("unknown"),
                    "provider": r.get("provider").and_then(|p| p.as_str()).unwrap_or("unknown"),
                    "success": r.get("error").is_none()
                })
            }).collect::<Vec<_>>()
        });
        
        debug!("Created metadata with {} fields", metadata.as_object().unwrap().len());
        Ok(metadata)
    }
}
