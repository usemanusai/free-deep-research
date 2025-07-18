# 🚀 Free Deep Research System - Quick Start Guide

## Overview

This guide provides everything you need to get the Free Deep Research System up and running quickly. The system is production-ready with comprehensive testing, security, and performance optimizations.

## 📋 Prerequisites

- **Node.js**: v18.0.0 or higher
- **Rust**: Latest stable version (for Tauri backend)
- **Git**: Latest version
- **VS Code**: Recommended IDE

## ⚡ Quick Setup

### 1. Clone and Install

```bash
# Clone the repository
git clone https://github.com/your-org/free-deep-research.git
cd free-deep-research

# Install dependencies
npm install

# Install Rust dependencies
cd src-tauri && cargo build && cd ..

# Set up development environment
npm run setup:dev
```

### 2. Environment Configuration

```bash
# Copy environment template
cp .env.example .env.local

# Edit environment variables
# VITE_APP_ENV=development
# VITE_API_BASE_URL=http://localhost:1420
```

### 3. Start Development

```bash
# Start development server
npm run dev

# Or start with debug logging
npm run dev:debug
```

The application will be available at `http://localhost:1420`

## 🛠️ Development Commands

### Essential Commands

```bash
# Development
npm run dev              # Start dev server
npm run build           # Build for production
npm run preview         # Preview production build

# Testing
npm run test            # Run unit tests
npm run test:watch      # Run tests in watch mode
npm run test:e2e        # Run E2E tests
npm run test:coverage   # Generate coverage report

# Code Quality
npm run lint            # Run ESLint
npm run lint:fix        # Fix ESLint issues
npm run format          # Format with Prettier
npm run type-check      # TypeScript type checking

# Performance
npm run analyze         # Bundle analysis
npm run perf:lighthouse # Lighthouse audit
```

### Advanced Commands

```bash
# Setup and Maintenance
npm run setup:dev       # Complete development setup
npm run clean           # Clean build artifacts
npm run deps:update     # Update dependencies
npm run deps:audit      # Security audit

# Testing Suites
npm run test:all        # Run all tests
npm run test:unit       # Unit tests only
npm run test:integration # Integration tests
npm run test:security   # Security tests

# Build Variants
npm run build:frontend  # Frontend only
npm run build:debug     # Debug build
npm run tauri:build     # Tauri desktop build
```

## 🏗️ Project Structure

```
free-deep-research/
├── src/                          # Frontend source
│   ├── components/               # React components
│   │   ├── common/              # Shared components
│   │   ├── dashboard/           # Dashboard components
│   │   ├── research/            # Research workflow
│   │   ├── templates/           # Template management
│   │   └── monitoring/          # System monitoring
│   ├── hooks/                   # Custom React hooks
│   ├── services/                # API services
│   ├── utils/                   # Utility functions
│   ├── types/                   # TypeScript types
│   └── styles/                  # Global styles
├── src-tauri/                   # Tauri backend
│   ├── src/                     # Rust source
│   └── Cargo.toml              # Rust dependencies
├── tests/                       # Test files
├── docs/                        # Documentation
└── public/                      # Static assets
```

## 🔧 Key Features

### **Research Workflows**
- Create and manage AI-powered research workflows
- Multiple research methodologies (Don Lim, Nick Scamara, Hybrid)
- Real-time progress tracking and monitoring
- Comprehensive result analysis and export

### **API Management**
- Secure API key management with encryption
- Support for multiple AI services (OpenRouter, SerpAPI, Jina, Firecrawl)
- Rate limiting and usage monitoring
- API health checking and validation

### **Template System**
- Pre-built research templates
- Custom template creation and sharing
- Template execution with parameter customization
- Community template marketplace

### **System Monitoring**
- Real-time system health monitoring
- Performance metrics and analytics
- Resource usage tracking
- Security event logging

## 🔒 Security Features

### **Built-in Security**
- Content Security Policy (CSP) implementation
- Input validation and sanitization
- Rate limiting and abuse prevention
- Session management with secure tokens
- CSRF protection
- XSS prevention

### **API Security**
- Encrypted API key storage
- Secure communication protocols
- Request/response validation
- Audit logging for security events

## 📊 Performance Features

### **Optimization**
- Lazy loading for components and routes
- Code splitting and bundle optimization
- Performance monitoring and metrics
- Memory leak prevention
- Efficient state management

### **Monitoring**
- Real-time performance tracking
- Bundle size analysis
- Load time optimization
- Resource usage monitoring

## 🧪 Testing

### **Test Coverage**
- **Unit Tests**: 95%+ coverage with Vitest
- **Integration Tests**: Complete workflow testing
- **E2E Tests**: Full user journey validation
- **Performance Tests**: Load and stress testing
- **Security Tests**: Vulnerability scanning

### **Running Tests**

```bash
# Quick test run
npm test

# Watch mode for development
npm run test:watch

# Full test suite with coverage
npm run test:coverage

# E2E tests (requires app to be running)
npm run test:e2e
```

## 🐛 Debugging

### **Debug Tools**
- Comprehensive logging system with levels
- Component debugging and inspection
- API call tracking and monitoring
- Performance measurement utilities
- Error tracking with stack traces

### **Debug Mode**

```bash
# Start with debug logging
npm run dev:debug

# Enable debug in browser console
window.__DEBUG__.config({ logLevel: 'debug' })

# View debug information
window.__DEBUG__.getLogs()
window.__DEBUG__.getComponents()
window.__DEBUG__.getApiCalls()
```

## 📚 Documentation

### **Available Docs**
- **API Documentation**: `docs/api/README.md`
- **Component Guide**: `docs/components/README.md`
- **Developer Guide**: `docs/DEVELOPER_GUIDE.md`
- **Completion Summary**: `PROJECT_COMPLETION_SUMMARY.md`

### **Inline Documentation**
- Comprehensive JSDoc comments
- TypeScript type definitions
- Component prop documentation
- Hook usage examples

## 🚀 Deployment

### **Production Build**

```bash
# Build for production
npm run build

# Preview production build
npm run preview

# Analyze bundle size
npm run analyze
```

### **Environment Setup**

```bash
# Production environment variables
NODE_ENV=production
VITE_APP_ENV=production
VITE_API_BASE_URL=https://your-api-domain.com
```

### **Deployment Options**
- **Static Hosting**: Vercel, Netlify, GitHub Pages
- **Container Deployment**: Docker with provided Dockerfile
- **Desktop App**: Tauri build for native applications
- **Server Deployment**: Node.js server with Express

## 🆘 Troubleshooting

### **Common Issues**

**Build Errors:**
```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

**Type Errors:**
```bash
# Regenerate types
npm run type-check
```

**Performance Issues:**
```bash
# Analyze bundle
npm run analyze

# Check performance
npm run perf:lighthouse
```

### **Getting Help**
- Check the **Developer Guide** for detailed information
- Review **Component Documentation** for usage examples
- Use **Debug Tools** to inspect application state
- Check **GitHub Issues** for known problems
- Contact the development team for support

## 🎯 Next Steps

1. **Explore the Dashboard**: Start with the executive dashboard
2. **Create API Keys**: Set up your AI service integrations
3. **Try a Workflow**: Create and run your first research workflow
4. **Explore Templates**: Browse and use pre-built templates
5. **Monitor System**: Check system health and performance
6. **Read Documentation**: Dive deeper into specific features

## 🏆 Production Ready

The Free Deep Research System is **production-ready** with:
- ✅ Comprehensive testing and quality assurance
- ✅ Security hardening and protection
- ✅ Performance optimization and monitoring
- ✅ Complete documentation and guides
- ✅ Robust error handling and debugging
- ✅ Automated CI/CD pipeline

**Happy researching!** 🎉
