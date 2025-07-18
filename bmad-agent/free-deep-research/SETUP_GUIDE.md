# Free Deep Research System - Setup Guide

## Overview

The Free Deep Research System is a comprehensive desktop application that combines multiple research methodologies with enterprise-grade API management. This guide will walk you through the complete setup process.

## Prerequisites

### System Requirements

- **Operating System**: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+)
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 2GB free space
- **Network**: Internet connection for API services

### Development Requirements

- **Node.js**: Version 18.0.0 or higher
- **Rust**: Latest stable version (for Tauri backend)
- **Git**: For version control

## Installation Steps

### 1. Clone the Repository

```bash
git clone https://github.com/huggingfacer04/free-deep-research.git
cd free-deep-research/bmad-agent/free-deep-research
```

### 2. Install Dependencies

#### Frontend Dependencies
```bash
npm install
```

#### Rust Dependencies (Tauri Backend)
```bash
cd src-tauri
cargo build
cd ..
```

### 3. Environment Configuration

#### Copy Environment Template
```bash
cp .env.template .env
```

#### Configure API Keys

Edit the `.env` file and add your API keys:

```env
# Required API Keys
OPENROUTER_API_KEY=your_openrouter_api_key_here
SERPAPI_KEY=your_serpapi_key_here
JINA_API_KEY=your_jina_api_key_here
FIRECRAWL_API_KEY=your_firecrawl_api_key_here

# Security
ENCRYPTION_KEY=your_32_character_encryption_key_here
```

#### Generate Encryption Key

For security, generate a strong encryption key:

```bash
# Using Node.js
node -e "console.log(require('crypto').randomBytes(32).toString('hex'))"

# Using OpenSSL
openssl rand -hex 32
```

### 4. API Key Setup

#### OpenRouter.ai
1. Visit [OpenRouter.ai](https://openrouter.ai/keys)
2. Create an account and generate an API key
3. Add credits to your account (free tier available)

#### SerpApi
1. Visit [SerpApi](https://serpapi.com/manage-api-key)
2. Create an account and get your API key
3. Free tier includes 100 searches/month

#### Jina AI
1. Visit [Jina.ai](https://jina.ai/)
2. Create an account and generate an API key
3. Free tier available

#### Firecrawl
1. Visit [Firecrawl.dev](https://firecrawl.dev/)
2. Create an account and get your API key
3. Free tier includes 500 pages/month

### 5. Database Setup

The application uses SQLite for local storage. The database will be created automatically on first run.

#### Database Location
- **Windows**: `%APPDATA%/free-deep-research/data/`
- **macOS**: `~/Library/Application Support/free-deep-research/data/`
- **Linux**: `~/.local/share/free-deep-research/data/`

### 6. Build and Run

#### Development Mode
```bash
npm run dev
```

#### Production Build
```bash
# Build frontend
npm run build:frontend

# Build Tauri application
npm run build
```

#### Platform-Specific Builds
```bash
# Windows
npm run build:windows

# macOS
npm run build:macos

# Linux
npm run build:linux
```

## Configuration

### Application Settings

The application provides a comprehensive settings panel accessible through the UI:

1. **API Management**: Configure and test API keys
2. **Research Settings**: Set default methodologies and parameters
3. **Security**: Configure encryption and backup settings
4. **Performance**: Adjust rate limits and concurrency
5. **Monitoring**: Configure logging and metrics

### Advanced Configuration

For advanced users, additional configuration options are available in the `.env` file:

#### Rate Limiting
```env
OPENROUTER_RATE_LIMIT=60
SERPAPI_RATE_LIMIT=100
RATE_LIMIT_BUFFER=10
```

#### Research Engine
```env
DEFAULT_METHODOLOGY=hybrid
MAX_CONCURRENT_WORKFLOWS=3
RESEARCH_TIMEOUT=30
```

#### Monitoring
```env
LOG_LEVEL=info
ENABLE_PERFORMANCE_MONITORING=true
METRICS_INTERVAL=30
```

## Verification

### 1. Health Check

After starting the application, verify system health:

1. Open the application
2. Navigate to the Dashboard
3. Check that all services show "Healthy" status
4. Verify API keys are properly configured

### 2. Test Research Workflow

1. Navigate to Research → Workflows
2. Create a new research workflow
3. Start the workflow and monitor progress
4. Verify results are generated successfully

### 3. API Key Testing

1. Navigate to API Keys
2. Use the "Test Connection" feature for each service
3. Verify all APIs respond successfully

## Troubleshooting

### Common Issues

#### Application Won't Start
- Check Node.js and Rust versions
- Verify all dependencies are installed
- Check console for error messages

#### API Key Errors
- Verify API keys are correctly formatted
- Check API key quotas and limits
- Ensure network connectivity

#### Database Issues
- Check file permissions in data directory
- Verify disk space availability
- Check database encryption settings

#### Performance Issues
- Reduce concurrent workflows
- Adjust rate limits
- Check system resources

### Log Files

Application logs are stored in:
- **Windows**: `%APPDATA%/free-deep-research/logs/`
- **macOS**: `~/Library/Logs/free-deep-research/`
- **Linux**: `~/.local/share/free-deep-research/logs/`

### Support

For additional support:

1. Check the [GitHub Issues](https://github.com/huggingfacer04/free-deep-research/issues)
2. Review the [Documentation](./docs/)
3. Join the community discussions

## Security Considerations

### API Key Security
- Never commit API keys to version control
- Use environment variables for sensitive data
- Enable encryption for local storage
- Regularly rotate API keys

### Data Protection
- Enable automatic backups
- Use strong encryption keys
- Secure backup storage
- Regular security updates

### Network Security
- Use HTTPS for all API communications
- Consider using a VPN for sensitive research
- Monitor network traffic
- Keep application updated

## Next Steps

After successful setup:

1. **Explore Templates**: Check out pre-built research templates
2. **Customize Workflows**: Create custom research methodologies
3. **Monitor Performance**: Use analytics to optimize usage
4. **Backup Configuration**: Export settings for backup
5. **Scale Usage**: Add more API keys for higher throughput

## Updates

To update the application:

```bash
git pull origin main
npm install
npm run build
```

For major updates, check the [CHANGELOG.md](./CHANGELOG.md) for breaking changes and migration instructions.
