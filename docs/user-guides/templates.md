# 📋 Template Management Guide

## Overview

Templates are the foundation of efficient research in the Free Deep Research System. They provide structured frameworks for different types of research, ensuring consistency, quality, and reproducibility across your research workflows.

## 🎯 What Are Research Templates?

Research templates are pre-configured frameworks that define:
- **Research methodology** and approach
- **Query structures** and search strategies  
- **Output formatting** and organization
- **Quality criteria** and validation rules
- **Workflow steps** and automation

## 🚀 Getting Started with Templates

### Accessing Templates

1. **Desktop App**: Navigate to **Research** → **Templates**
2. **Web Interface**: Click the **Templates** tab in the main navigation
3. **Quick Access**: Use `Ctrl+T` (Windows/Linux) or `Cmd+T` (macOS)

### Template Library Overview

The system includes several built-in template categories:

#### 📚 Academic Templates
- **Literature Review** - Systematic literature analysis
- **Systematic Review** - Evidence-based research synthesis
- **Meta-Analysis** - Statistical analysis of multiple studies
- **Citation Analysis** - Reference and impact tracking

#### 🏢 Business Templates
- **Market Research** - Industry and competitor analysis
- **Competitive Intelligence** - Strategic competitor monitoring
- **Trend Analysis** - Market trend identification and forecasting
- **SWOT Analysis** - Strengths, weaknesses, opportunities, threats

#### 🔬 Technical Templates
- **Technology Assessment** - Technical solution evaluation
- **Patent Research** - Intellectual property analysis
- **Standards Research** - Industry standards and compliance
- **Technical Documentation** - Product and process documentation

#### 🎯 General Purpose Templates
- **Quick Research** - Fast, focused information gathering
- **Deep Dive** - Comprehensive topic exploration
- **Fact Checking** - Information verification and validation
- **News Monitoring** - Current events and updates tracking

## 📖 Using Existing Templates

### Selecting a Template

1. **Browse Categories**: Filter templates by research type
2. **Search Templates**: Use keywords to find specific templates
3. **Preview Template**: View template structure before selection
4. **Template Details**: Review methodology and expected outputs

### Starting a Research Session

```typescript
// Example: Using Academic Literature Review Template
1. Select "Literature Review" template
2. Configure parameters:
   - Research topic: "AI in Healthcare"
   - Time range: "2020-2024"
   - Source types: "Peer-reviewed journals"
   - Language: "English"
3. Click "Start Research"
```

### Template Parameters

Each template includes configurable parameters:

#### **Research Scope**
- **Topic focus**: Primary research question
- **Keywords**: Search terms and synonyms
- **Time range**: Publication or relevance dates
- **Geographic scope**: Regional or global focus

#### **Source Configuration**
- **Source types**: Academic, news, web, patents
- **Quality filters**: Peer-reviewed, authority sources
- **Language preferences**: Multi-language support
- **Exclusion criteria**: Sources to avoid

#### **Output Preferences**
- **Format**: Report, summary, data export
- **Detail level**: Brief, standard, comprehensive
- **Citation style**: APA, MLA, Chicago, IEEE
- **Visualization**: Charts, graphs, mind maps

## 🛠️ Creating Custom Templates

### Template Builder Interface

1. **Access Builder**: Click **"Create New Template"**
2. **Template Wizard**: Follow guided setup process
3. **Advanced Editor**: Use JSON editor for complex templates

### Basic Template Structure

```json
{
  "name": "My Custom Template",
  "description": "Template for specific research needs",
  "category": "custom",
  "methodology": "hybrid",
  "parameters": {
    "scope": {
      "topic": "{{research_topic}}",
      "keywords": ["{{keyword1}}", "{{keyword2}}"],
      "timeRange": "{{time_range}}"
    },
    "sources": {
      "types": ["academic", "web"],
      "languages": ["en"],
      "qualityFilter": "high"
    },
    "output": {
      "format": "report",
      "detailLevel": "standard",
      "citationStyle": "apa"
    }
  },
  "workflow": [
    {
      "step": "initial_search",
      "description": "Broad topic exploration",
      "methodology": "web_search",
      "parameters": {
        "depth": "shallow",
        "sources": 10
      }
    },
    {
      "step": "academic_validation",
      "description": "Scholarly source verification",
      "methodology": "academic_search",
      "parameters": {
        "depth": "deep",
        "sources": 20
      }
    }
  ]
}
```

### Template Variables

Use variables to make templates reusable:

#### **Standard Variables**
- `{{research_topic}}` - Main research subject
- `{{keywords}}` - Search terms array
- `{{time_range}}` - Date range specification
- `{{language}}` - Language preference
- `{{output_format}}` - Desired output type

#### **Advanced Variables**
- `{{methodology}}` - Research approach
- `{{quality_threshold}}` - Minimum quality score
- `{{source_limit}}` - Maximum sources to analyze
- `{{budget_limit}}` - API cost constraints

### Conditional Logic

Add dynamic behavior to templates:

```json
{
  "conditionalSteps": [
    {
      "condition": "{{source_count}} < 10",
      "action": "expand_search",
      "parameters": {
        "additional_keywords": true,
        "broader_scope": true
      }
    },
    {
      "condition": "{{confidence_score}} < 0.8",
      "action": "validation_step",
      "parameters": {
        "cross_reference": true,
        "expert_sources": true
      }
    }
  ]
}
```

## 🔧 Advanced Template Features

### Template Inheritance

Create template hierarchies for consistency:

```json
{
  "extends": "base_academic_template",
  "overrides": {
    "methodology": "systematic_review",
    "output.citationStyle": "vancouver"
  }
}
```

### Multi-Stage Workflows

Design complex research processes:

1. **Discovery Phase** - Initial topic exploration
2. **Validation Phase** - Source verification and quality check
3. **Analysis Phase** - Deep content analysis
4. **Synthesis Phase** - Information integration and summary
5. **Review Phase** - Quality assurance and final validation

### Quality Gates

Implement automatic quality controls:

```json
{
  "qualityGates": [
    {
      "stage": "after_discovery",
      "criteria": {
        "minimum_sources": 5,
        "confidence_threshold": 0.7
      },
      "action_if_failed": "expand_search"
    },
    {
      "stage": "before_synthesis",
      "criteria": {
        "source_diversity": 0.6,
        "recency_score": 0.8
      },
      "action_if_failed": "additional_validation"
    }
  ]
}
```

## 📊 Template Performance Analytics

### Usage Metrics

Track template effectiveness:
- **Success rate** - Percentage of successful research sessions
- **Average completion time** - Time from start to final output
- **User satisfaction** - Ratings and feedback scores
- **Cost efficiency** - API costs per research session

### Optimization Recommendations

The system provides automatic suggestions:
- **Parameter tuning** - Optimize search parameters
- **Workflow improvements** - Streamline research steps
- **Source optimization** - Better source selection
- **Quality enhancements** - Improve output quality

## 🔄 Template Sharing and Collaboration

### Community Templates

Access templates shared by other users:
1. **Browse Community** - Explore public template library
2. **Rate and Review** - Provide feedback on templates
3. **Fork Templates** - Create your own versions
4. **Contribute Back** - Share your improvements

### Team Templates

For organizational use:
- **Team Library** - Shared organizational templates
- **Access Controls** - Manage who can view/edit templates
- **Version Control** - Track template changes over time
- **Approval Workflows** - Quality control for shared templates

## 🛡️ Best Practices

### Template Design Principles

1. **Clear Purpose** - Define specific research objectives
2. **Flexible Parameters** - Allow customization without complexity
3. **Quality Focus** - Prioritize accuracy over speed
4. **User-Friendly** - Intuitive parameter names and descriptions
5. **Well-Documented** - Include usage examples and tips

### Common Pitfalls to Avoid

- **Over-Complexity** - Keep templates simple and focused
- **Rigid Parameters** - Allow sufficient customization
- **Poor Documentation** - Always include clear descriptions
- **No Quality Gates** - Implement validation checkpoints
- **Ignoring Feedback** - Regularly update based on user experience

## 🆘 Troubleshooting Templates

### Common Issues

**Template Won't Load**
```bash
# Check template syntax
Validate JSON structure in template editor
Review error messages in console
```

**Poor Research Results**
- Review keyword selection and scope
- Adjust quality thresholds
- Check source type configuration
- Validate methodology selection

**Slow Performance**
- Reduce source limits
- Optimize search parameters
- Use caching where appropriate
- Consider simpler methodologies

### Getting Help

- **Template Documentation** - Built-in help for each template
- **Community Forum** - Ask questions and share experiences
- **Support Tickets** - Direct assistance for complex issues
- **Video Tutorials** - Step-by-step template creation guides

## 🎓 Template Examples and Use Cases

### Academic Research Templates

#### Literature Review Template Example
```json
{
  "name": "Systematic Literature Review",
  "description": "Comprehensive academic literature analysis",
  "category": "academic",
  "methodology": "systematic_review",
  "parameters": {
    "scope": {
      "topic": "{{research_topic}}",
      "keywords": ["{{primary_keyword}}", "{{secondary_keywords}}"],
      "timeRange": "{{publication_years}}",
      "databases": ["pubmed", "ieee", "acm", "scopus"]
    },
    "inclusion_criteria": {
      "publication_types": ["journal_article", "conference_paper"],
      "languages": ["english"],
      "peer_reviewed": true,
      "minimum_citations": 5
    },
    "exclusion_criteria": {
      "publication_types": ["editorial", "letter", "comment"],
      "predatory_journals": true,
      "duplicate_studies": true
    }
  },
  "workflow": [
    {
      "step": "database_search",
      "description": "Search academic databases",
      "methodology": "academic_search",
      "parameters": {
        "databases": ["pubmed", "ieee", "scopus"],
        "search_strategy": "comprehensive"
      }
    },
    {
      "step": "screening",
      "description": "Title and abstract screening",
      "methodology": "ai_screening",
      "parameters": {
        "screening_criteria": "inclusion_exclusion",
        "confidence_threshold": 0.8
      }
    },
    {
      "step": "full_text_review",
      "description": "Full text analysis",
      "methodology": "deep_analysis",
      "parameters": {
        "analysis_depth": "comprehensive",
        "extract_data": true
      }
    },
    {
      "step": "quality_assessment",
      "description": "Study quality evaluation",
      "methodology": "quality_scoring",
      "parameters": {
        "assessment_tool": "prisma",
        "bias_assessment": true
      }
    }
  ]
}
```

#### Business Intelligence Template Example
```json
{
  "name": "Market Analysis Report",
  "description": "Comprehensive market research and analysis",
  "category": "business",
  "methodology": "market_research",
  "parameters": {
    "scope": {
      "market": "{{target_market}}",
      "geography": "{{geographic_scope}}",
      "timeframe": "{{analysis_period}}",
      "competitors": ["{{competitor_list}}"]
    },
    "analysis_areas": {
      "market_size": true,
      "growth_trends": true,
      "competitive_landscape": true,
      "customer_segments": true,
      "regulatory_environment": true
    }
  },
  "workflow": [
    {
      "step": "market_overview",
      "description": "General market landscape",
      "methodology": "web_research",
      "parameters": {
        "sources": ["industry_reports", "news", "analyst_reports"],
        "depth": "comprehensive"
      }
    },
    {
      "step": "competitor_analysis",
      "description": "Competitive intelligence gathering",
      "methodology": "competitive_research",
      "parameters": {
        "analysis_type": "swot",
        "data_points": ["pricing", "features", "market_share"]
      }
    },
    {
      "step": "trend_analysis",
      "description": "Market trend identification",
      "methodology": "trend_detection",
      "parameters": {
        "time_horizon": "5_years",
        "trend_types": ["technology", "consumer", "regulatory"]
      }
    }
  ]
}
```

### Template Customization Scenarios

#### Scenario 1: Academic Researcher
**Need**: Systematic review of AI in healthcare literature
**Template Customization**:
- Set databases to medical and AI-focused sources
- Configure PRISMA-compliant workflow
- Enable citation network analysis
- Set quality thresholds for peer-reviewed sources

#### Scenario 2: Business Analyst
**Need**: Competitive analysis for SaaS market entry
**Template Customization**:
- Focus on SaaS-specific metrics and KPIs
- Include pricing and feature comparison
- Enable real-time market monitoring
- Configure executive summary format

#### Scenario 3: Policy Researcher
**Need**: Regulatory impact analysis
**Template Customization**:
- Include government and legal databases
- Enable policy document analysis
- Configure stakeholder impact assessment
- Set compliance checking parameters

## 🔍 Template Debugging and Optimization

### Common Template Issues

#### **Syntax Errors**
```json
// ❌ Incorrect JSON syntax
{
  "name": "My Template",
  "parameters": {
    "scope": {
      "topic": "{{research_topic}}"  // Missing comma
      "keywords": ["{{keywords}}"]
    }
  }
}

// ✅ Correct JSON syntax
{
  "name": "My Template",
  "parameters": {
    "scope": {
      "topic": "{{research_topic}}",
      "keywords": ["{{keywords}}"]
    }
  }
}
```

#### **Variable Resolution Issues**
```json
// ❌ Undefined variable reference
{
  "workflow": [
    {
      "step": "search",
      "parameters": {
        "query": "{{undefined_variable}}"  // Variable not defined
      }
    }
  ]
}

// ✅ Properly defined variables
{
  "variables": {
    "research_query": {
      "type": "string",
      "required": true,
      "description": "Main research question"
    }
  },
  "workflow": [
    {
      "step": "search",
      "parameters": {
        "query": "{{research_query}}"
      }
    }
  ]
}
```

### Performance Optimization

#### **Template Efficiency Tips**
1. **Minimize API Calls**: Combine similar search operations
2. **Use Caching**: Enable result caching for repeated queries
3. **Optimize Workflows**: Remove redundant steps
4. **Set Reasonable Limits**: Avoid excessive source counts
5. **Progressive Enhancement**: Start simple, add complexity gradually

#### **Resource Management**
```json
{
  "resource_limits": {
    "max_api_calls": 100,
    "timeout_seconds": 300,
    "max_sources": 50,
    "cache_duration": 3600
  },
  "optimization": {
    "enable_caching": true,
    "parallel_processing": true,
    "early_termination": true,
    "quality_threshold": 0.7
  }
}
```

## 📚 Template Library Management

### Organizing Your Templates

#### **Folder Structure**
```
My Templates/
├── Academic/
│   ├── Literature Reviews/
│   ├── Systematic Reviews/
│   └── Citation Analysis/
├── Business/
│   ├── Market Research/
│   ├── Competitive Analysis/
│   └── Industry Reports/
├── Personal/
│   ├── Quick Research/
│   ├── Fact Checking/
│   └── News Monitoring/
└── Experimental/
    ├── AI Testing/
    ├── New Methodologies/
    └── Beta Features/
```

#### **Template Metadata**
```json
{
  "metadata": {
    "version": "2.1.0",
    "created_date": "2024-01-15",
    "last_modified": "2024-07-19",
    "author": "Dr. Jane Smith",
    "organization": "University Research Lab",
    "tags": ["academic", "systematic-review", "healthcare"],
    "usage_count": 47,
    "success_rate": 0.94,
    "average_duration": "45 minutes"
  }
}
```

### Version Control and Collaboration

#### **Template Versioning**
- **Semantic Versioning**: Major.Minor.Patch (e.g., 2.1.3)
- **Change Tracking**: Automatic logging of modifications
- **Rollback Capability**: Revert to previous versions
- **Branching**: Create experimental variations

#### **Team Collaboration**
- **Shared Libraries**: Organization-wide template collections
- **Permission Management**: Control edit and usage rights
- **Review Process**: Peer review before publication
- **Usage Analytics**: Track template adoption and success

## 🎯 Advanced Template Techniques

### Dynamic Template Generation

#### **AI-Assisted Template Creation**
```javascript
// Generate template based on research description
const templateGenerator = {
  input: "I need to research the impact of remote work on productivity",
  analysis: {
    domain: "business_research",
    methodology: "mixed_methods",
    sources: ["academic", "industry_reports", "surveys"],
    timeline: "recent_5_years"
  },
  generated_template: {
    // AI-generated template structure
    name: "Remote Work Productivity Analysis",
    methodology: "hybrid_research",
    // ... complete template configuration
  }
}
```

#### **Adaptive Templates**
```json
{
  "adaptive_parameters": {
    "source_expansion": {
      "trigger": "insufficient_results",
      "action": "expand_search_scope",
      "threshold": 5
    },
    "quality_adjustment": {
      "trigger": "low_confidence",
      "action": "increase_validation",
      "threshold": 0.6
    },
    "methodology_switching": {
      "trigger": "timeout",
      "action": "fallback_methodology",
      "fallback": "quick_search"
    }
  }
}
```

### Integration with External Systems

#### **API Integration Templates**
```json
{
  "external_integrations": {
    "zotero": {
      "auto_export": true,
      "collection": "{{project_name}}",
      "tags": ["{{research_topic}}", "auto-imported"]
    },
    "slack": {
      "notifications": true,
      "channel": "#research-updates",
      "milestone_alerts": true
    },
    "google_sheets": {
      "data_export": true,
      "sheet_id": "{{tracking_sheet}}",
      "update_frequency": "real_time"
    }
  }
}
```

---

**Next Steps**: Once you're comfortable with templates, explore [First Research Session](./first-research.md) to put your templates into action, or dive into [Desktop Application](./desktop-app.md) to master the interface.

**Need Help?** Check our [FAQ](./faq.md) or visit the [Community Forum](https://community.freedeepresearch.org) for template-specific questions.
