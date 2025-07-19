# Comprehensive Automated Dependency Management and Health Check System

## Overview

This document describes the comprehensive automated dependency management and health check system implemented for the free-deep-research project. The system addresses the immediate npm installation failure with `@visx/hierarchy@^3.12.2` and provides a robust, enterprise-grade dependency management solution.

## 🚨 Immediate Fix Applied

### Problem Resolved
- **Issue**: `npm error notarget No matching version found for @visx/hierarchy@^3.12.2`
- **Root Cause**: Package.json requested version 3.12.2, but only version 3.12.0 exists in npm registry
- **Solution**: Updated both `@visx/hierarchy` and `@visx/network` from `^3.12.2` to `^3.12.0`

### Files Modified
- `bmad-agent/free-deep-research/package.json` - Updated @visx package versions

## 🏗️ System Architecture

### Core Components

1. **DependencyScanner** (`scripts/dependency-health-check.js`)
   - Scans all package files (package.json, Cargo.toml, requirements.txt)
   - Supports multiple package managers (npm, yarn, pnpm, cargo, pip)
   - Recursive directory traversal with intelligent filtering

2. **VersionResolver** (`scripts/dependency-health-check.js`)
   - Queries package registries (npmjs.com, crates.io, PyPI)
   - Implements caching for performance
   - Handles registry fallbacks and timeouts
   - Semantic versioning compatibility checks

3. **ConflictDetector** (`scripts/dependency-health-check.js`)
   - Identifies version conflicts and compatibility issues
   - Checks for non-existent package versions
   - Validates peer dependencies
   - Detects deprecated packages

4. **AutoUpdater** (`scripts/dependency-manager/core/auto-updater.js`)
   - Safely updates package files with automatic backups
   - Supports multiple update modes (conservative, aggressive, security-only)
   - Validates updates before applying
   - Automatic lock file regeneration

5. **HealthChecker** (`scripts/dependency-manager/core/health-checker.js`)
   - Comprehensive system health validation
   - Security vulnerability scanning
   - Dependency tree integrity checks
   - System requirements verification

6. **HookInstaller** (`scripts/dependency-manager/hooks/install-hooks.js`)
   - Installs startup integration hooks
   - Git pre-commit hooks
   - Docker health checks
   - CI/CD pipeline integration

## 🚀 Quick Start

### 1. Test the Immediate Fix
```bash
# Navigate to the main project directory
cd bmad-agent/free-deep-research

# Test npm install (should now work)
npm install

# Verify the fix worked
npm list @visx/hierarchy @visx/network
```

### 2. Run Dependency Health Check
```bash
# Basic health check
node scripts/dependency-health-check.js

# Auto-fix detected issues
node scripts/dependency-health-check.js --auto-fix

# Security-only mode
node scripts/dependency-health-check.js --mode=security-only
```

### 3. Install Integration Hooks
```bash
# Install all integration hooks
node scripts/dependency-manager/hooks/install-hooks.js

# Uninstall hooks if needed
node scripts/dependency-manager/hooks/install-hooks.js --uninstall
```

### 4. Run Comprehensive Tests
```bash
# Verify the entire system
node scripts/test-dependency-fix.js
```

## 📋 Available Commands

### Package.json Scripts (Auto-installed)
```bash
npm run deps:check      # Basic dependency check
npm run deps:fix        # Auto-fix dependency issues
npm run deps:health     # Conservative health check
npm run deps:security   # Security-only updates
```

### CLI Options
```bash
# Dependency Health Check
node scripts/dependency-health-check.js [options]

Options:
  --auto-fix, -f           Auto-fix detected conflicts
  --mode=MODE             Set update mode (conservative|aggressive|security-only)
  --no-health-check       Skip health validation
  --help                  Show help information

# Hook Installation
node scripts/dependency-manager/hooks/install-hooks.js [options]

Options:
  --uninstall             Remove installed hooks
  --help                  Show help information
```

## ⚙️ Configuration

### Update Modes

1. **Conservative Mode** (Default)
   - Only patch and minor version updates
   - Safest option for production environments
   - Minimal risk of breaking changes

2. **Aggressive Mode**
   - Allows major version updates
   - Maximum feature updates
   - Higher risk of breaking changes

3. **Security-Only Mode**
   - Only applies security-related updates
   - Minimal changes, maximum security
   - Ideal for stable production systems

### Configuration File
Edit `scripts/dependency-manager/config/dependency-config.json` to customize:
- Update modes and thresholds
- Registry endpoints and fallbacks
- Notification channels
- Health check parameters
- Integration settings

## 🔧 Integration Points

### 1. Startup Integration
- **Pre-install hooks**: Run before `npm install`
- **Post-install hooks**: Validate after installation
- **Application startup**: Middleware integration
- **Development server**: Vite dev server integration

### 2. Git Integration
- **Pre-commit hooks**: Validate dependencies before commits
- **Pre-push hooks**: Optional validation before pushes
- **Commit message templates**: Standardized dependency update messages

### 3. Docker Integration
- **Build-time checks**: Validate dependencies during Docker builds
- **Health checks**: Container health validation
- **Multi-stage optimization**: Dependency caching strategies

### 4. CI/CD Integration
- **GitHub Actions**: Automated dependency checks on PRs
- **Daily security scans**: Scheduled vulnerability checks
- **Artifact uploads**: Health reports and metrics
- **PR comments**: Automated status updates

## 📊 Monitoring and Reporting

### Health Reports
- **JSON format**: Machine-readable for automation
- **HTML format**: Human-readable dashboards
- **Markdown format**: Documentation integration

### Metrics Tracked
- Dependency update success rates
- Security vulnerability counts
- Package registry response times
- Build success/failure rates
- System resource utilization

### Notification Channels
- Console output with color coding
- File-based logging
- Slack/Discord webhooks (configurable)
- Email notifications (configurable)

## 🛡️ Security Features

### Vulnerability Scanning
- **npm audit**: JavaScript package vulnerabilities
- **cargo audit**: Rust crate vulnerabilities
- **pip-audit**: Python package vulnerabilities (planned)

### Security Thresholds
- Critical vulnerabilities: Immediate alerts
- High vulnerabilities: Daily reports
- Medium/Low vulnerabilities: Weekly summaries

### Secure Update Process
- Automatic backups before updates
- Validation after updates
- Rollback capability on failures
- Cryptographic integrity checks

## 🔍 Troubleshooting

### Common Issues

1. **Registry Timeout Errors**
   ```bash
   # Increase timeout in config
   "timeout": 60000  # 60 seconds
   ```

2. **Permission Errors**
   ```bash
   # Run with appropriate permissions
   sudo node scripts/dependency-health-check.js
   ```

3. **Lock File Conflicts**
   ```bash
   # Clean and regenerate
   rm package-lock.json
   npm install
   ```

4. **Build Failures After Updates**
   ```bash
   # Restore from backup
   node scripts/dependency-health-check.js --restore-backup
   ```

### Debug Mode
```bash
# Enable debug logging
DEBUG=1 node scripts/dependency-health-check.js
```

### Log Files
- Health check logs: `reports/dependency-notifications.log`
- Test results: `reports/dependency-fix-test-*.json`
- Health reports: `reports/health-report-*.json`

## 📈 Performance Optimization

### Caching Strategy
- Registry response caching (1 hour TTL)
- Dependency tree caching
- Build artifact caching

### Parallel Processing
- Concurrent package checks
- Parallel registry queries
- Asynchronous health validations

### Resource Management
- Configurable concurrency limits
- Memory usage monitoring
- Timeout management

## 🔄 Maintenance

### Regular Tasks
- Weekly dependency updates
- Monthly security audits
- Quarterly system health reviews
- Annual configuration updates

### Backup Management
- Automatic backup creation
- 30-day retention policy
- Compression for storage efficiency
- Restoration procedures

### System Updates
- Keep dependency management tools updated
- Monitor for new security sources
- Update registry endpoints as needed
- Review and update configuration

## 📚 Additional Resources

- [Package Manager Documentation](./PACKAGE_MANAGERS.md)
- [Security Best Practices](./SECURITY_PRACTICES.md)
- [CI/CD Integration Guide](./CICD_INTEGRATION.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)

## 🤝 Contributing

To contribute to the dependency management system:

1. Follow the existing code structure
2. Add comprehensive tests for new features
3. Update documentation
4. Ensure backward compatibility
5. Submit pull requests with detailed descriptions

## 📄 License

This dependency management system is part of the free-deep-research project and follows the same licensing terms.
