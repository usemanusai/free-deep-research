# 💻 Development Guide

## Overview

This guide provides comprehensive information for developers working on the Free Deep Research System. It covers setup, development workflows, coding standards, and contribution guidelines.

## 🚀 Quick Start

### Prerequisites

- **Node.js**: 18.x or higher
- **Rust**: 1.75.0 or higher
- **Docker**: 24.0+ (for containerized development)
- **Git**: Latest version
- **VS Code**: Recommended IDE with extensions

### Development Environment Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/huggingfacer04/free-deep-research.git
   cd free-deep-research
   ```

2. **Install Dependencies**
   ```bash
   # Install Node.js dependencies
   npm install
   
   # Install Rust dependencies
   cd src-tauri
   cargo build
   cd ..
   
   # Install web frontend dependencies
   cd bmad-agent/deep_research_frontend
   npm install
   cd ../..
   ```

3. **Environment Configuration**
   ```bash
   # Copy environment templates
   cp .env.template .env
   cp bmad-agent/free-deep-research/.env.template bmad-agent/free-deep-research/.env
   
   # Configure your API keys and settings
   nano .env
   ```

4. **Start Development Servers**
   ```bash
   # Start the desktop application
   npm run tauri dev
   
   # Or start web development server
   cd bmad-agent/deep_research_frontend
   npm run dev
   ```

## 📁 Project Structure

```
free-deep-research/
├── apps/
│   ├── desktop/                    # Tauri desktop application
│   └── web/                        # React web application
├── packages/
│   ├── ai-orchestrator/            # AI orchestration system
│   └── bmad-core/                  # BMAD agent configurations
├── bmad-agent/
│   ├── free-deep-research/         # Main Tauri application
│   └── deep_research_frontend/     # Web frontend
├── infrastructure/
│   ├── docker/                     # Docker configurations
│   └── scripts/                    # Build and deployment scripts
├── docs/                           # Documentation
└── src-tauri/                      # Rust backend code
```

## 🛠️ Development Workflows

### Feature Development

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Development Process**
   - Write code following our coding standards
   - Add tests for new functionality
   - Update documentation as needed
   - Test thoroughly in development environment

3. **Code Quality Checks**
   ```bash
   # Run linting
   npm run lint
   
   # Run tests
   npm run test
   
   # Check Rust code
   cd src-tauri
   cargo clippy
   cargo test
   ```

4. **Submit Pull Request**
   - Push your branch to GitHub
   - Create a pull request with detailed description
   - Ensure all CI checks pass
   - Request review from team members

### Testing Strategy

#### Frontend Testing
```bash
# Unit tests
npm run test:unit

# Integration tests
npm run test:integration

# E2E tests
npm run test:e2e
```

#### Backend Testing
```bash
cd src-tauri
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Performance tests
cargo test --release --test performance
```

## 🎯 Coding Standards

### TypeScript/JavaScript Standards

- **ESLint Configuration**: Follow the project's ESLint rules
- **Prettier**: Use Prettier for code formatting
- **Naming Conventions**:
  - Variables: `camelCase`
  - Functions: `camelCase`
  - Classes: `PascalCase`
  - Constants: `UPPER_SNAKE_CASE`
  - Files: `kebab-case.ts`

### Rust Standards

- **Clippy**: Use Clippy for linting
- **Rustfmt**: Use rustfmt for formatting
- **Naming Conventions**:
  - Variables: `snake_case`
  - Functions: `snake_case`
  - Structs: `PascalCase`
  - Modules: `snake_case`
  - Constants: `UPPER_SNAKE_CASE`

### Documentation Standards

- **Code Comments**: Document complex logic and public APIs
- **README Files**: Each major component should have a README
- **API Documentation**: Use JSDoc for TypeScript, rustdoc for Rust
- **Architecture Decisions**: Document in ADR format

## 🔧 Development Tools

### Recommended VS Code Extensions

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tauri-apps.tauri-vscode",
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode",
    "dbaeumer.vscode-eslint",
    "ms-vscode.vscode-typescript-next"
  ]
}
```

### Development Scripts

```bash
# Development server
npm run dev

# Build for production
npm run build

# Run tests
npm run test

# Lint code
npm run lint

# Format code
npm run format

# Type checking
npm run type-check

# Dependency health check
npm run deps:check

# Security audit
npm run security:audit
```

## 🐛 Debugging

### Frontend Debugging

1. **Browser DevTools**: Use Chrome/Firefox DevTools
2. **React DevTools**: Install React Developer Tools extension
3. **Console Logging**: Use structured logging with levels
4. **Source Maps**: Enabled in development builds

### Backend Debugging

1. **Rust Debugging**: Use `rust-analyzer` with VS Code
2. **Logging**: Use `tracing` crate for structured logging
3. **Performance Profiling**: Use `cargo flamegraph`
4. **Memory Debugging**: Use `valgrind` or `heaptrack`

### Tauri-Specific Debugging

```bash
# Enable Tauri development mode
RUST_LOG=debug npm run tauri dev

# Debug WebView
WEBKIT_DISABLE_COMPOSITING_MODE=1 npm run tauri dev

# Debug IPC communication
TAURI_DEBUG=1 npm run tauri dev
```

## 🔒 Security Guidelines

### Code Security

- **Input Validation**: Validate all user inputs
- **SQL Injection**: Use parameterized queries
- **XSS Prevention**: Sanitize output, use CSP headers
- **Authentication**: Implement proper session management
- **API Security**: Use rate limiting and authentication

### Dependency Security

```bash
# Audit npm dependencies
npm audit

# Audit Rust dependencies
cargo audit

# Update dependencies
npm update
cargo update
```

## 📊 Performance Guidelines

### Frontend Performance

- **Bundle Size**: Monitor and optimize bundle size
- **Code Splitting**: Use dynamic imports for large components
- **Caching**: Implement proper caching strategies
- **Image Optimization**: Optimize images and use modern formats

### Backend Performance

- **Database Queries**: Optimize database queries
- **Caching**: Implement Redis caching where appropriate
- **Async Operations**: Use async/await for I/O operations
- **Memory Management**: Monitor memory usage and prevent leaks

## 🚨 Troubleshooting

### Common Issues

1. **Build Failures**
   ```bash
   # Clear caches
   npm run clean
   rm -rf node_modules
   npm install
   ```

2. **Tauri Build Issues**
   ```bash
   # Update Tauri CLI
   cargo install tauri-cli --version "^1.0"
   
   # Clear Rust cache
   cargo clean
   ```

3. **Development Server Issues**
   ```bash
   # Check port availability
   lsof -i :3000
   
   # Kill processes using port
   kill -9 $(lsof -t -i:3000)
   ```

## 📚 Additional Resources

- **[Technical Issues Analysis](./TECHNICAL_ISSUES_ANALYSIS.md)** - Known issues and solutions
- **[Dependency Management](./DEPENDENCY_MANAGEMENT_SYSTEM.md)** - Dependency management system
- **[Immediate Action Plan](./IMMEDIATE_ACTION_PLAN_2025.md)** - Current development priorities
- **[API Documentation](../api/README.md)** - Complete API reference
- **[Architecture Documentation](../architecture/README.md)** - System architecture

## 🤝 Contributing

### Getting Help

- **GitHub Issues**: Report bugs and request features
- **Discussions**: Ask questions in GitHub Discussions
- **Discord**: Join our development Discord server
- **Documentation**: Check existing documentation first

### Code Review Process

1. **Self Review**: Review your own code before submitting
2. **Automated Checks**: Ensure all CI checks pass
3. **Peer Review**: At least one team member must review
4. **Testing**: Verify functionality works as expected
5. **Documentation**: Update relevant documentation

---

**Next**: Check out [Technical Issues Analysis](./TECHNICAL_ISSUES_ANALYSIS.md) for current development priorities.
