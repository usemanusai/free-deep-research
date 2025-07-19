# 🔬 Free Deep Research System

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Node.js](https://img.shields.io/badge/node.js-20+-green.svg)](https://nodejs.org)
[![Tauri](https://img.shields.io/badge/tauri-1.5+-blue.svg)](https://tauri.app)
[![TypeScript](https://img.shields.io/badge/typescript-5.3+-blue.svg)](https://www.typescriptlang.org)
[![React](https://img.shields.io/badge/react-18.2+-61DAFB.svg)](https://reactjs.org)
[![Docker](https://img.shields.io/badge/docker-supported-blue.svg)](https://www.docker.com)
[![Version](https://img.shields.io/badge/version-3.0.0-blue.svg)](CHANGELOG.md)

> **Next-Generation AI-Powered Research Platform with BMAD Agent Integration and Enterprise Intelligence**

A revolutionary, multi-component research automation platform that combines advanced AI agent orchestration, cross-platform desktop applications, containerized deployments, and enterprise-grade intelligence capabilities. Features complete BMAD AI Agent integration, distributed computing, real-time collaboration, and autonomous research capabilities—all while maintaining zero operational costs through intelligent free-tier optimization.

## 🚀 Quick Start

### Prerequisites
- Node.js 20+ 
- Rust 1.75+
- Docker & Docker Compose

### Installation
```bash
git clone https://github.com/usemanusai/free-deep-research.git
cd free-deep-research
./infrastructure/scripts/setup.sh
```

### Development
```bash
# Desktop application
cd apps/desktop
npm run dev

# Web application  
cd apps/web
npm run dev

# Full system with Docker
docker-compose -f infrastructure/docker-compose.dev.yml up
```

## 📁 Repository Structure

```
free-deep-research/
├── apps/                           # Applications
│   ├── desktop/                    # Tauri desktop application
│   └── web/                        # React web application
├── packages/                       # Shared packages
│   ├── ai-orchestrator/            # AI orchestration system
│   └── bmad-core/                  # BMAD agent configurations
├── docs/                           # Documentation
│   ├── api/                        # API documentation
│   ├── architecture/               # Architecture documentation
│   ├── deployment/                 # Deployment guides
│   ├── development/                # Development guides
│   ├── reports/                    # Analysis reports
│   └── user-guides/                # User documentation
├── infrastructure/                 # Infrastructure & deployment
│   ├── docker/                     # Docker configurations
│   ├── scripts/                    # Build and deployment scripts
│   ├── docker-compose.dev.yml      # Development environment
│   ├── docker-compose.prod.yml     # Production environment
│   └── docker-compose.yml          # Default configuration
└── tools/                          # Development tools
```

## 🔧 Features

- **AI-Powered Research**: Advanced research capabilities with multiple AI providers
- **Desktop Application**: Cross-platform desktop app built with Tauri (React + Rust)
- **Web Interface**: Modern React-based web interface
- **AI Agent Orchestration**: BMAD methodology for AI agent coordination
- **API Management**: Comprehensive API key and service management
- **Real-time Analytics**: Performance monitoring and analytics
- **Enterprise Security**: Advanced security features and compliance

## 🏗️ Applications

### Desktop Application (`apps/desktop/`)
Tauri-based desktop application with React frontend and Rust backend.
- Cross-platform support (Windows, macOS, Linux)
- Native performance with web technologies
- Advanced research capabilities
- Offline functionality

### Web Application (`apps/web/`)
React-based web application for browser access.
- Modern responsive design
- Real-time collaboration features
- Progressive Web App (PWA) capabilities

## 📦 Packages

### AI Orchestrator (`packages/ai-orchestrator/`)
Core AI orchestration system with agent coordination capabilities.

### BMAD Core (`packages/bmad-core/`)
BMAD methodology implementation with agent personas, tasks, and templates.

## 📚 Documentation

### 📖 User Documentation
- **[Complete User Guide](docs/user-guides/COMPLETE_USER_GUIDE_2025.md)** - Comprehensive user documentation
- **[Desktop App Setup](apps/desktop/SETUP_GUIDE.md)** - Desktop application setup guide

### 🔌 API Documentation
- **[API Overview](docs/api/README.md)** - Complete API reference and examples
- **[Authentication API](docs/api/authentication.md)** - API key management and security
- **[Research Workflow API](docs/api/research-workflow.md)** - Research execution and management
- **[BMAD Integration API](docs/api/bmad-integration.md)** - AI agent orchestration
- **[Analytics API](docs/api/analytics.md)** - Business intelligence and insights
- **[Monitoring API](docs/api/monitoring.md)** - System health and performance
- **[Configuration API](docs/api/configuration.md)** - System and user settings

### 🏗️ Architecture Documentation
- **[Architecture Overview](docs/architecture/README.md)** - High-level architecture and design
- **[System Overview](docs/architecture/system-overview.md)** - Detailed system components

### 🛠️ Development & Deployment
- **[Development Guide](docs/development/)** - Development setup and guidelines
- **[Docker Deployment](docs/deployment/DOCKER-IMPLEMENTATION-SUMMARY.md)** - Docker deployment guide
- **[Reports & Analysis](docs/reports/)** - Technical reports and system analysis

## 🚀 Deployment

### Docker Deployment
```bash
# Development
docker-compose -f infrastructure/docker-compose.dev.yml up

# Production
docker-compose -f infrastructure/docker-compose.prod.yml up -d
```

### Manual Deployment
```bash
# Build desktop app
cd apps/desktop
npm run build

# Build web app
cd apps/web
npm run build
```

## 🤝 Contributing

Please read our [Contributing Guidelines](apps/desktop/CONTRIBUTING.md) before submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🌟 Latest Updates - Version 3.0.0 "Global Intelligence Network"

**🌐 NEW: Global Intelligence Network Features**
- ✅ **Federated Research System**: Secure cross-organization research collaboration
- ✅ **AI Research Marketplace**: Community platform for sharing AI agents and methodologies
- ✅ **Quantum-Ready Architecture**: Post-quantum cryptography and quantum computing integration
- ✅ **Advanced NLP Engine**: Natural language processing for research automation
- ✅ **Blockchain Integration**: Decentralized research validation and peer review
- ✅ **Global Knowledge Graph**: Interconnected knowledge representation and discovery

**🤖 BMAD AI Agent Integration Complete (v2.1.0)**
- ✅ **Research-Powered AI Agents**: Product Manager, Technical Architect, Platform Engineer
- ✅ **Multi-Agent Workflow Coordination**: Collaborative research with validation
- ✅ **Evidence-Based Documentation**: PRD, Architecture, and Implementation documents
- ✅ **Professional Quality Standards**: Enterprise-grade communication
- ✅ **Cost-Optimized Research**: $12-25 per session with 5:1 ROI
- ✅ **Real-Time Research Monitoring**: Live progress tracking and optimization
