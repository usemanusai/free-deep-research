use crate::models::research_template::{ResearchTemplate, TemplateCategory};
use crate::models::research_workflow::{ResearchMethodology, WorkflowParameters, OutputFormat};
use crate::services::template_manager::template_builder::TemplateBuilder;

/// Predefined research templates for common use cases
pub struct PredefinedTemplates;

impl PredefinedTemplates {
    /// Create all predefined templates
    pub fn create_all() -> Vec<ResearchTemplate> {
        vec![
            Self::create_academic_research_template(),
            Self::create_market_analysis_template(),
            Self::create_competitive_intelligence_template(),
            Self::create_technical_documentation_template(),
            Self::create_business_strategy_template(),
            Self::create_scientific_literature_review_template(),
            Self::create_legal_research_template(),
            Self::create_financial_analysis_template(),
            Self::create_product_research_template(),
            Self::create_industry_trends_template(),
        ]
    }

    /// Academic Research Template
    fn create_academic_research_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Academic Research".to_string(),
            "Comprehensive academic research with scholarly sources and citations".to_string(),
            TemplateCategory::Academic,
            ResearchMethodology::Hybrid,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .featured()
        .tags(vec!["academic".to_string(), "research".to_string(), "scholarly".to_string()])
        .workflow_config(WorkflowParameters {
            methodology: ResearchMethodology::Hybrid,
            output_format: OutputFormat::Markdown,
            include_sources: true,
            max_sources: Some(50),
            max_iterations: 15,
            max_concurrent_steps: 3,
            timeout_minutes: 45,
            auto_retry: true,
            save_intermediate_results: true,
            enable_caching: true,
            custom_parameters: std::collections::HashMap::new(),
        })
        .add_text_parameter(
            "research_topic".to_string(),
            "Research Topic".to_string(),
            "The main topic or research question you want to investigate".to_string(),
            true,
            Some("Enter your research topic or question".to_string()),
        )
        .add_select_parameter(
            "academic_level".to_string(),
            "Academic Level".to_string(),
            "The academic level for the research depth and complexity".to_string(),
            true,
            vec!["undergraduate".to_string(), "graduate".to_string(), "doctoral".to_string(), "postdoctoral".to_string()],
            Some("graduate".to_string()),
        )
        .add_multi_select_parameter(
            "disciplines".to_string(),
            "Academic Disciplines".to_string(),
            "Select relevant academic disciplines for cross-disciplinary research".to_string(),
            false,
            vec![
                "computer_science".to_string(), "psychology".to_string(), "biology".to_string(),
                "physics".to_string(), "chemistry".to_string(), "mathematics".to_string(),
                "economics".to_string(), "sociology".to_string(), "philosophy".to_string(),
                "history".to_string(), "literature".to_string(), "engineering".to_string(),
            ],
            None,
        )
        .add_boolean_parameter(
            "include_recent_only".to_string(),
            "Include Recent Research Only".to_string(),
            "Focus on research published in the last 5 years".to_string(),
            false,
        )
        .add_number_parameter(
            "min_citations".to_string(),
            "Minimum Citations".to_string(),
            "Minimum number of citations for sources to be considered authoritative".to_string(),
            false,
            Some(0.0),
            Some(10000.0),
        )
        .add_step(
            "academic_search".to_string(),
            "Academic Literature Search".to_string(),
            "Search for academic papers and scholarly sources".to_string(),
            Some("serpapi".to_string()),
            Some("/search".to_string()),
        )
        .add_step_input("query".to_string(), "{{research_topic}} academic papers scholarly research".to_string())
        .add_step_input("tbm".to_string(), "\"sch\"".to_string())
        .add_step_input("num".to_string(), "30".to_string())
        .add_step_output_mapping("organic_results".to_string(), "academic_sources".to_string())
        .add_step(
            "content_extraction".to_string(),
            "Extract Academic Content".to_string(),
            "Extract and analyze content from academic sources".to_string(),
            Some("firecrawl".to_string()),
            Some("/scrape".to_string()),
        )
        .add_step_input("formats".to_string(), "[\"markdown\"]".to_string())
        .add_step_input("only_main_content".to_string(), "true".to_string())
        .add_step_output_mapping("scraped_content".to_string(), "extracted_content".to_string())
        .add_step_with_dependencies(
            "academic_analysis".to_string(),
            "Academic Analysis and Synthesis".to_string(),
            "Analyze and synthesize academic findings with proper citations".to_string(),
            Some("openrouter".to_string()),
            Some("/chat/completions".to_string()),
            vec!["academic_search".to_string(), "content_extraction".to_string()],
        )
        .add_step_input("model".to_string(), "\"anthropic/claude-3-sonnet\"".to_string())
        .add_step_input("temperature".to_string(), "0.2".to_string())
        .add_step_input("max_tokens".to_string(), "8000".to_string())
        .build()
    }

    /// Market Analysis Template
    fn create_market_analysis_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Market Analysis".to_string(),
            "Comprehensive market research and competitive analysis".to_string(),
            TemplateCategory::Market,
            ResearchMethodology::Hybrid,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .featured()
        .tags(vec!["market".to_string(), "analysis".to_string(), "business".to_string()])
        .add_string_parameter(
            "market_or_industry".to_string(),
            "Market/Industry".to_string(),
            "The market or industry you want to analyze".to_string(),
            true,
        )
        .add_string_parameter(
            "geographic_region".to_string(),
            "Geographic Region".to_string(),
            "Geographic focus for the market analysis".to_string(),
            false,
        )
        .add_select_parameter(
            "analysis_timeframe".to_string(),
            "Analysis Timeframe".to_string(),
            "Time period for the market analysis".to_string(),
            true,
            vec!["current".to_string(), "1_year".to_string(), "3_years".to_string(), "5_years".to_string()],
            Some("current".to_string()),
        )
        .add_multi_select_parameter(
            "analysis_aspects".to_string(),
            "Analysis Aspects".to_string(),
            "Specific aspects of market analysis to focus on".to_string(),
            true,
            vec![
                "market_size".to_string(), "growth_trends".to_string(), "key_players".to_string(),
                "competitive_landscape".to_string(), "customer_segments".to_string(), "pricing_analysis".to_string(),
                "regulatory_environment".to_string(), "technological_trends".to_string(), "barriers_to_entry".to_string(),
            ],
            Some(vec!["market_size".to_string(), "growth_trends".to_string(), "key_players".to_string()]),
        )
        .add_step(
            "market_search".to_string(),
            "Market Data Search".to_string(),
            "Search for market data, reports, and industry information".to_string(),
            Some("serpapi".to_string()),
            Some("/search".to_string()),
        )
        .add_step_input("query".to_string(), "{{market_or_industry}} market analysis industry report {{geographic_region}}".to_string())
        .add_step(
            "competitive_search".to_string(),
            "Competitive Intelligence".to_string(),
            "Search for competitor information and market positioning".to_string(),
            Some("serpapi".to_string()),
            Some("/search".to_string()),
        )
        .add_step_input("query".to_string(), "{{market_or_industry}} competitors market share competitive analysis".to_string())
        .add_step_with_dependencies(
            "market_synthesis".to_string(),
            "Market Analysis Synthesis".to_string(),
            "Synthesize market data into comprehensive analysis report".to_string(),
            Some("openrouter".to_string()),
            Some("/chat/completions".to_string()),
            vec!["market_search".to_string(), "competitive_search".to_string()],
        )
        .add_step_input("model".to_string(), "\"anthropic/claude-3-sonnet\"".to_string())
        .add_step_input("temperature".to_string(), "0.3".to_string())
        .build()
    }

    /// Competitive Intelligence Template
    fn create_competitive_intelligence_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Competitive Intelligence".to_string(),
            "Deep competitive analysis and intelligence gathering".to_string(),
            TemplateCategory::Competitive,
            ResearchMethodology::NickScamara,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .tags(vec!["competitive".to_string(), "intelligence".to_string(), "analysis".to_string()])
        .add_string_parameter(
            "target_company".to_string(),
            "Target Company".to_string(),
            "The company you want to analyze".to_string(),
            true,
        )
        .add_multi_select_parameter(
            "competitors".to_string(),
            "Known Competitors".to_string(),
            "List of known competitors to include in analysis".to_string(),
            false,
            vec![], // Will be populated dynamically
            None,
        )
        .add_multi_select_parameter(
            "analysis_areas".to_string(),
            "Analysis Areas".to_string(),
            "Specific areas to focus the competitive analysis on".to_string(),
            true,
            vec![
                "products_services".to_string(), "pricing_strategy".to_string(), "marketing_approach".to_string(),
                "financial_performance".to_string(), "technology_stack".to_string(), "partnerships".to_string(),
                "market_positioning".to_string(), "customer_reviews".to_string(), "hiring_trends".to_string(),
            ],
            Some(vec!["products_services".to_string(), "pricing_strategy".to_string(), "market_positioning".to_string()]),
        )
        .add_step(
            "company_research".to_string(),
            "Company Research".to_string(),
            "Research target company information and recent developments".to_string(),
            Some("firecrawl".to_string()),
            Some("/scrape".to_string()),
        )
        .add_step(
            "competitor_mapping".to_string(),
            "Competitor Discovery".to_string(),
            "Discover and map competitive landscape".to_string(),
            Some("serpapi".to_string()),
            Some("/search".to_string()),
        )
        .add_step_input("query".to_string(), "{{target_company}} competitors alternative similar companies".to_string())
        .add_step_with_dependencies(
            "competitive_analysis".to_string(),
            "Competitive Analysis".to_string(),
            "Comprehensive competitive intelligence analysis".to_string(),
            Some("openrouter".to_string()),
            Some("/chat/completions".to_string()),
            vec!["company_research".to_string(), "competitor_mapping".to_string()],
        )
        .build()
    }

    /// Technical Documentation Template
    fn create_technical_documentation_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Technical Documentation Research".to_string(),
            "Research and analyze technical documentation, APIs, and developer resources".to_string(),
            TemplateCategory::Technical,
            ResearchMethodology::NickScamara,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .tags(vec!["technical".to_string(), "documentation".to_string(), "api".to_string()])
        .add_string_parameter(
            "technology_or_api".to_string(),
            "Technology/API".to_string(),
            "The technology, framework, or API you want to research".to_string(),
            true,
        )
        .add_select_parameter(
            "documentation_type".to_string(),
            "Documentation Type".to_string(),
            "Type of technical documentation to focus on".to_string(),
            true,
            vec!["api_reference".to_string(), "tutorials".to_string(), "best_practices".to_string(), "troubleshooting".to_string(), "all".to_string()],
            Some("all".to_string()),
        )
        .add_boolean_parameter(
            "include_code_examples".to_string(),
            "Include Code Examples".to_string(),
            "Extract and include code examples in the research".to_string(),
            true,
        )
        .build()
    }

    /// Business Strategy Template
    fn create_business_strategy_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Business Strategy Research".to_string(),
            "Strategic business research for planning and decision making".to_string(),
            TemplateCategory::Business,
            ResearchMethodology::Hybrid,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .tags(vec!["business".to_string(), "strategy".to_string(), "planning".to_string()])
        .add_text_parameter(
            "business_question".to_string(),
            "Business Question".to_string(),
            "The strategic business question or challenge you want to research".to_string(),
            true,
            Some("Enter your strategic business question".to_string()),
        )
        .add_string_parameter(
            "industry_context".to_string(),
            "Industry Context".to_string(),
            "Industry or business context for the strategic research".to_string(),
            true,
        )
        .build()
    }

    /// Scientific Literature Review Template
    fn create_scientific_literature_review_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Scientific Literature Review".to_string(),
            "Systematic review of scientific literature and research papers".to_string(),
            TemplateCategory::Scientific,
            ResearchMethodology::DonLim,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .tags(vec!["scientific".to_string(), "literature".to_string(), "review".to_string()])
        .add_text_parameter(
            "research_question".to_string(),
            "Research Question".to_string(),
            "The scientific research question for the literature review".to_string(),
            true,
            None,
        )
        .build()
    }

    /// Legal Research Template
    fn create_legal_research_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Legal Research".to_string(),
            "Legal research including case law, statutes, and regulations".to_string(),
            TemplateCategory::Legal,
            ResearchMethodology::Hybrid,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .tags(vec!["legal".to_string(), "law".to_string(), "research".to_string()])
        .add_text_parameter(
            "legal_question".to_string(),
            "Legal Question".to_string(),
            "The legal question or issue you want to research".to_string(),
            true,
            None,
        )
        .add_select_parameter(
            "jurisdiction".to_string(),
            "Jurisdiction".to_string(),
            "Legal jurisdiction for the research".to_string(),
            true,
            vec!["federal".to_string(), "state".to_string(), "international".to_string(), "all".to_string()],
            Some("federal".to_string()),
        )
        .build()
    }

    /// Financial Analysis Template
    fn create_financial_analysis_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Financial Analysis".to_string(),
            "Financial research and analysis including market data and company financials".to_string(),
            TemplateCategory::Financial,
            ResearchMethodology::Hybrid,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .tags(vec!["financial".to_string(), "analysis".to_string(), "investment".to_string()])
        .add_string_parameter(
            "financial_topic".to_string(),
            "Financial Topic".to_string(),
            "The financial topic, company, or investment you want to analyze".to_string(),
            true,
        )
        .build()
    }

    /// Product Research Template
    fn create_product_research_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Product Research".to_string(),
            "Comprehensive product research including features, reviews, and market positioning".to_string(),
            TemplateCategory::Business,
            ResearchMethodology::NickScamara,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .tags(vec!["product".to_string(), "research".to_string(), "market".to_string()])
        .add_string_parameter(
            "product_name".to_string(),
            "Product Name".to_string(),
            "The product you want to research".to_string(),
            true,
        )
        .build()
    }

    /// Industry Trends Template
    fn create_industry_trends_template() -> ResearchTemplate {
        TemplateBuilder::new(
            "Industry Trends Analysis".to_string(),
            "Analysis of current and emerging trends in specific industries".to_string(),
            TemplateCategory::Market,
            ResearchMethodology::Hybrid,
            "system".to_string(),
        )
        .version("1.0.0".to_string())
        .public()
        .tags(vec!["industry".to_string(), "trends".to_string(), "analysis".to_string()])
        .add_string_parameter(
            "industry".to_string(),
            "Industry".to_string(),
            "The industry you want to analyze for trends".to_string(),
            true,
        )
        .add_select_parameter(
            "trend_timeframe".to_string(),
            "Trend Timeframe".to_string(),
            "Time period for trend analysis".to_string(),
            true,
            vec!["current".to_string(), "emerging".to_string(), "future_5_years".to_string(), "all".to_string()],
            Some("current".to_string()),
        )
        .build()
    }
}
