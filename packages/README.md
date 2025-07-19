# Packages

This directory contains shared packages and libraries used across the Free Deep Research System.

## 📦 Available Packages

### AI Orchestrator (`ai-orchestrator/`)
Core AI orchestration system that manages agent coordination, workflow execution, and research automation.

**Key Features:**
- Agent lifecycle management
- Workflow coordination
- Research integration
- Service registration
- Error handling and recovery

**Usage:**
```bash
cd ai-orchestrator
# See ai-orchestrator/README.md for detailed usage
```

### BMAD Core (`bmad-core/`)
BMAD (Business Methodology for AI Development) implementation containing agent personas, tasks, templates, and checklists.

**Key Components:**
- **Personas**: AI agent personality definitions and capabilities
- **Tasks**: Structured task definitions and workflows
- **Templates**: Document and output templates
- **Checklists**: Quality assurance and validation checklists
- **Data**: Knowledge base and technical preferences

**Usage:**
```bash
cd bmad-core
# Contains configuration files and templates
# Used by both desktop and web applications
```

## 🔗 Integration

These packages are designed to work together and are integrated into both the desktop and web applications:

- **Desktop App**: Uses both packages for full AI orchestration and BMAD methodology
- **Web App**: Leverages BMAD core for agent interactions and templates
- **Infrastructure**: Docker configurations include package dependencies

## 📚 Documentation

- [AI Orchestrator Documentation](ai-orchestrator/README.md)
- [BMAD Core Documentation](bmad-core/)
- [Integration Guide](../docs/development/)

## 🤝 Contributing

When contributing to packages:
1. Ensure changes are backward compatible
2. Update relevant documentation
3. Test integration with both applications
4. Follow the established coding standards

For detailed contribution guidelines, see the main [Contributing Guide](../apps/desktop/CONTRIBUTING.md).
