# Applications

This directory contains the main applications of the Free Deep Research System.

## 🖥️ Desktop Application (`desktop/`)

**Tauri-based cross-platform desktop application**

- **Frontend**: React + TypeScript
- **Backend**: Rust (Tauri)
- **Features**: 
  - Native performance
  - Cross-platform support (Windows, macOS, Linux)
  - Offline functionality
  - Advanced research capabilities
  - AI agent orchestration
  - Local data storage

**Quick Start:**
```bash
cd desktop
npm install
npm run dev
```

**Build:**
```bash
# Development build
npm run build:frontend

# Production builds
npm run build:windows
npm run build:macos
npm run build:linux
```

## 🌐 Web Application (`web/`)

**React-based web application**

- **Frontend**: React + TypeScript
- **Features**:
  - Modern responsive design
  - Real-time collaboration
  - Progressive Web App (PWA) capabilities
  - Browser-based research tools
  - Cloud synchronization

**Quick Start:**
```bash
cd web
npm install
npm run dev
```

**Build:**
```bash
npm run build
```

## 🔄 Shared Dependencies

Both applications share:
- **BMAD Core**: Agent personas, tasks, and templates
- **AI Orchestrator**: Core orchestration logic
- **Common UI Components**: Shared React components
- **API Interfaces**: Consistent API contracts

## 🚀 Deployment

### Desktop Application
- Distributable executables for each platform
- Auto-updater support
- Installer packages (MSI, DMG, AppImage)

### Web Application
- Static site deployment
- Docker containerization
- CDN distribution
- Progressive Web App installation

## 📚 Documentation

- [Desktop App Documentation](desktop/README.md)
- [Web App Documentation](web/README.md)
- [Deployment Guide](../docs/deployment/)
- [Development Guide](../docs/development/)

## 🔧 Development

### Prerequisites
- Node.js 20+
- Rust 1.75+ (for desktop app)
- Docker (optional)

### Development Workflow
1. Install dependencies in both apps
2. Start development servers
3. Use shared packages for common functionality
4. Test cross-platform compatibility

### Testing
```bash
# Desktop app
cd desktop
npm run test
npm run test:e2e

# Web app
cd web
npm run test
```

## 🤝 Contributing

When contributing to applications:
1. Follow the established code style
2. Write tests for new features
3. Update documentation
4. Test on multiple platforms (desktop app)
5. Ensure responsive design (web app)

For detailed guidelines, see [Contributing Guide](desktop/CONTRIBUTING.md).
