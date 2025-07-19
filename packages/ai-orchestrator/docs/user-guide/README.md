# AI Agent Orchestrator - User Guide

## Welcome

Welcome to the AI Agent Orchestrator, your professional AI assistant capable of embodying different specialist roles to help you accomplish complex software development tasks. This guide will help you get started and make the most of the system's capabilities.

## Quick Start

### 1. Getting Started
When you first interact with the orchestrator, you'll be greeted with a professional introduction and a mandatory mode selection menu:

```
🎯 **BMAD AI Agent System - Mode Selection Required**

Please choose your workflow mode:

**1. Documentation Mode (Default & Recommended)**
   📋 Generate exactly 3 complete, final documents ready for developer handoff:
   • `prd.md` - Product Requirements Document
   • `architecture.md` - Technical architecture document  
   • `checklist.md` - Development checklist

**2. Full Development Mode**
   🚀 Build the entire project within this chat session
   • Complete application development with AI agents
   • Interactive development workflow
   • Full implementation and testing

**Please type "1" for Documentation Mode or "2" for Full Development Mode to continue.**
```

### 2. Mode Selection
Choose the mode that best fits your needs:

- **Documentation Mode**: Perfect for creating comprehensive project documentation that can be handed off to developers
- **Full Development Mode**: Ideal for interactive development with real-time AI agent collaboration

### 3. Working with Agents
Once you've selected a mode, the orchestrator will help you choose the right specialist agent for your task:

- **John (Product Manager)**: Requirements, PRDs, stakeholder management
- **Fred (Technical Architect)**: System design, architecture, technical decisions
- **Alex (Platform Engineer)**: Infrastructure, DevOps, cloud architecture
- **Jane (Design Architect)**: UI/UX design, frontend architecture
- **Sarah (Product Owner)**: Agile processes, story management
- **Mike (Scrum Master)**: Process facilitation, team coordination

## Available Commands

### Essential Commands
- `/help` - Get help and guidance
- `/agents` - List all available specialist agents
- `/switch <agent>` - Change to a specific agent
- `/tasks` - Show available tasks for current agent
- `/reset` - Return to base orchestrator

### Advanced Commands
- `/yolo` - Toggle rapid execution mode
- `/full_yolo` - Enhanced rapid mode with auto-approval
- `/agent-list` - Detailed agent and task listing
- `/doc-out` - Output complete document without truncation
- `/party-mode` - Multi-agent group collaboration

### Agent-Specific Commands
- `/{agent}` - Quick switch to specific agent
- `/load-{agent}` - Force switch and reset to agent
- `/{agent} {query}` - Ask specific agent a question
- `/bmad {query}` - Ask the orchestrator directly

## Workflow Modes

### Documentation Mode
This mode generates three professional documents ready for developer handoff:

**Process:**
1. Select Documentation Mode (option 1)
2. Describe your project or requirements
3. The orchestrator activates appropriate agents collaboratively
4. Agents work together to create comprehensive documentation
5. Receive three complete documents:
   - **PRD** (`prd.md`) - Complete product requirements
   - **Architecture** (`architecture.md`) - Technical design and approach
   - **Checklist** (`checklist.md`) - Implementation steps and acceptance criteria

**Best for:**
- Handing off specifications to development teams
- Creating comprehensive project documentation
- Ensuring all stakeholders have clear requirements
- Professional documentation that requires no additional clarification

### Full Development Mode
This mode provides interactive development with AI agent collaboration:

**Process:**
1. Select Full Development Mode (option 2)
2. Choose specific agents or let the orchestrator recommend
3. Work interactively with agents on development tasks
4. Iterate and refine through collaborative development
5. Receive working code, tests, and documentation

**Best for:**
- Interactive development sessions
- Real-time problem solving
- Iterative development and refinement
- Learning and exploration

## Working with Specific Agents

### John - Product Manager
**Specializes in:**
- Product Requirements Documents (PRDs)
- Requirements analysis and validation
- Stakeholder management and communication
- User story development
- Market research and competitive analysis

**Common Tasks:**
- Create comprehensive PRDs
- Analyze and refine requirements
- Conduct stakeholder interviews
- Develop user stories with acceptance criteria

**Example Interaction:**
```
User: "I need a PRD for a mobile expense tracking app"
Orchestrator: "This requires our Product Manager. Switching to John..."
John: "I'll help you create a comprehensive PRD. Let me start by understanding your target users and business objectives..."
```

### Fred - Technical Architect
**Specializes in:**
- System architecture and design
- Technology selection and evaluation
- Scalability and performance planning
- Security architecture
- Integration design and API planning

**Common Tasks:**
- Create system architecture documents
- Evaluate technology stacks
- Design scalable system architectures
- Plan security and compliance measures

**Example Interaction:**
```
User: "I need architecture for a high-traffic e-commerce platform"
Orchestrator: "This requires our Technical Architect. Switching to Fred..."
Fred: "I'll design a scalable e-commerce architecture. Let me analyze your traffic requirements and business constraints..."
```

### Alex - Platform Engineer
**Specializes in:**
- Infrastructure design and automation
- DevOps and CI/CD pipelines
- Cloud architecture and deployment
- Monitoring and observability
- Security and compliance

**Common Tasks:**
- Design infrastructure architecture
- Create CI/CD pipeline specifications
- Plan deployment and scaling strategies
- Design monitoring and alerting systems

## Best Practices

### Getting the Best Results
1. **Be Specific**: Provide clear, detailed requirements and context
2. **Use the Right Agent**: Choose the agent that best matches your task
3. **Iterate**: Don't hesitate to refine and improve through multiple interactions
4. **Ask Questions**: Use commands to get clarification and guidance
5. **Review Outputs**: Validate deliverables against your requirements

### Communication Tips
- **Provide Context**: Share relevant background information
- **Set Expectations**: Clarify what you need and when you need it
- **Ask for Clarification**: Use `/help` or ask questions if anything is unclear
- **Use Commands**: Leverage commands for efficient navigation and control
- **Give Feedback**: Let agents know if outputs meet your needs

### Quality Assurance
- **Review Deliverables**: Carefully review all outputs for completeness and accuracy
- **Validate Requirements**: Ensure requirements are clear, testable, and complete
- **Check Consistency**: Verify that all documents and outputs are consistent
- **Test Feasibility**: Validate that technical solutions are feasible and appropriate

## Troubleshooting

### Common Issues
- **Agent Not Responding**: Try `/reset` and reselect the agent
- **Unclear Output**: Ask for clarification or use `/doc-out` for complete documents
- **Wrong Agent**: Use `/switch <agent>` to change to the appropriate specialist
- **Mode Issues**: Use `/reset` to return to mode selection

### Getting Help
- **Use `/help`**: Get context-sensitive help and guidance
- **Check Documentation**: Review this user guide and API reference
- **Try Different Approaches**: Experiment with different agents or commands
- **Start Over**: Use `/reset` to start fresh if needed

## Advanced Features

### Multi-Agent Collaboration
- **Party Mode**: Use `/party-mode` for group brainstorming and collaboration
- **Agent Consultation**: Use `/{agent} {query}` to consult specific agents
- **Workflow Orchestration**: Let the system coordinate multiple agents automatically

### Customization
- **YOLO Modes**: Use `/yolo` or `/full_yolo` for rapid execution
- **Pre-Selection**: Use `/pre_select_agents` to choose agents in advance
- **Task Focus**: Use `/tasks` to see available tasks for current agent

---

*This user guide provides comprehensive guidance for effectively using the AI Agent Orchestrator to accomplish your software development and documentation goals.*
