# 💻 Installation Guide

## Overview

This guide provides detailed instructions for installing the Free Deep Research System on various platforms and configurations.

## 🎯 Installation Options

### 1. Web Application (No Installation Required)
**Best for**: Quick start, trying the system, cloud-based usage
- **Access**: https://research.freedeepresearch.org
- **Requirements**: Modern web browser, internet connection
- **Setup Time**: Immediate

### 2. Desktop Application (Recommended)
**Best for**: Regular use, offline capabilities, enhanced performance
- **Platforms**: Windows, macOS, Linux
- **Requirements**: 4GB RAM, 2GB storage
- **Setup Time**: 5-10 minutes

### 3. Docker Deployment
**Best for**: Self-hosting, enterprise deployment, development
- **Requirements**: Docker, Docker Compose
- **Setup Time**: 15-30 minutes

### 4. Source Code Installation
**Best for**: Developers, customization, contributing
- **Requirements**: Node.js, Rust, Git
- **Setup Time**: 30-60 minutes

## 🖥️ Desktop Application Installation

### Windows Installation

#### System Requirements
- **OS**: Windows 10 (version 1903) or later
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 2GB available space
- **Internet**: Required for research operations

#### Installation Steps
1. **Download**: Visit [releases page](https://github.com/huggingfacer04/free-deep-research/releases)
2. **Select Version**: Download `Free-Deep-Research-Setup-x.x.x.exe`
3. **Run Installer**: Double-click the downloaded file
4. **Follow Wizard**: 
   - Accept license agreement
   - Choose installation directory
   - Select additional components
5. **Launch**: Start from Start Menu or desktop shortcut

#### Windows-Specific Configuration
```powershell
# Optional: Add to PATH for command line access
$env:PATH += ";C:\Program Files\Free Deep Research"

# Set up Windows Defender exclusion (optional, for performance)
Add-MpPreference -ExclusionPath "C:\Program Files\Free Deep Research"
```

### macOS Installation

#### System Requirements
- **OS**: macOS 10.15 (Catalina) or later
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 2GB available space
- **Architecture**: Intel x64 or Apple Silicon (M1/M2)

#### Installation Steps
1. **Download**: Get `Free-Deep-Research-x.x.x.dmg`
2. **Mount DMG**: Double-click the downloaded file
3. **Install**: Drag application to Applications folder
4. **Security**: First launch may require security approval:
   - Go to System Preferences → Security & Privacy
   - Click "Open Anyway" for Free Deep Research

#### macOS-Specific Configuration
```bash
# Optional: Add to PATH for command line access
echo 'export PATH="/Applications/Free Deep Research.app/Contents/MacOS:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Grant necessary permissions
sudo xattr -rd com.apple.quarantine "/Applications/Free Deep Research.app"
```

### Linux Installation

#### System Requirements
- **OS**: Ubuntu 18.04+, Debian 10+, CentOS 8+, or equivalent
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 2GB available space
- **Dependencies**: GTK3, WebKit2GTK

#### Ubuntu/Debian Installation
```bash
# Download and install
wget https://github.com/huggingfacer04/free-deep-research/releases/download/v3.0.0/free-deep-research_3.0.0_amd64.deb
sudo dpkg -i free-deep-research_3.0.0_amd64.deb

# Install dependencies if needed
sudo apt-get install -f

# Launch
free-deep-research
```

#### CentOS/RHEL Installation
```bash
# Download and install
wget https://github.com/huggingfacer04/free-deep-research/releases/download/v3.0.0/free-deep-research-3.0.0.x86_64.rpm
sudo rpm -i free-deep-research-3.0.0.x86_64.rpm

# Launch
free-deep-research
```

#### AppImage (Universal Linux)
```bash
# Download AppImage
wget https://github.com/huggingfacer04/free-deep-research/releases/download/v3.0.0/Free-Deep-Research-3.0.0.AppImage

# Make executable
chmod +x Free-Deep-Research-3.0.0.AppImage

# Run
./Free-Deep-Research-3.0.0.AppImage
```

## 🐳 Docker Installation

### Prerequisites
```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.20.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### Quick Docker Setup
```bash
# Clone repository
git clone https://github.com/huggingfacer04/free-deep-research.git
cd free-deep-research

# Copy environment template
cp .env.template .env

# Edit configuration
nano .env

# Start services
docker-compose up -d

# Verify installation
docker-compose ps
```

### Docker Environment Configuration
```bash
# .env file for Docker
POSTGRES_DB=free_deep_research
POSTGRES_USER=research_user
POSTGRES_PASSWORD=your_secure_password
DATABASE_URL=postgresql://research_user:your_secure_password@postgres:5432/free_deep_research

REDIS_URL=redis://redis:6379

# API Keys
OPENROUTER_API_KEY=your_openrouter_key
SERPAPI_KEY=your_serpapi_key
TAVILY_API_KEY=your_tavily_key

# Application settings
NODE_ENV=production
APP_PORT=3000
API_PORT=8080
```

## 🔧 Source Code Installation

### Prerequisites
```bash
# Node.js (18.x or later)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Git
sudo apt-get install git
```

### Installation Steps
```bash
# Clone repository
git clone https://github.com/huggingfacer04/free-deep-research.git
cd free-deep-research

# Install dependencies
npm install

# Install Rust dependencies
cd src-tauri
cargo build --release
cd ..

# Install frontend dependencies
cd bmad-agent/deep_research_frontend
npm install
cd ../..

# Build application
npm run build

# Start development server
npm run tauri dev
```

## 🔑 API Configuration

### Required API Keys

#### OpenRouter (AI Models)
1. Visit [OpenRouter](https://openrouter.ai)
2. Create account and get API key
3. Add to configuration:
   ```
   OPENROUTER_API_KEY=sk-or-v1-your-key-here
   ```

#### SerpAPI (Web Search)
1. Visit [SerpAPI](https://serpapi.com)
2. Sign up for free tier
3. Get API key from dashboard
4. Add to configuration:
   ```
   SERPAPI_KEY=your-serpapi-key-here
   ```

#### Tavily (Advanced Search)
1. Visit [Tavily](https://tavily.com)
2. Create developer account
3. Generate API key
4. Add to configuration:
   ```
   TAVILY_API_KEY=tvly-your-key-here
   ```

### Optional API Keys

#### Firecrawl (Web Scraping)
```
FIRECRAWL_API_KEY=fc-your-key-here
```

#### Jina AI (Embeddings)
```
JINA_API_KEY=jina-your-key-here
```

## ⚙️ Configuration

### Application Settings

#### Desktop Application
1. **Open Settings**: Click gear icon or File → Preferences
2. **API Keys**: Add your API keys in the API Configuration section
3. **Research Settings**: Configure default research parameters
4. **Storage**: Set data storage location
5. **Updates**: Configure automatic update preferences

#### Environment Variables
```bash
# Core settings
NODE_ENV=production
RUST_LOG=info
LOG_LEVEL=info

# Database
DATABASE_URL=your_database_url

# Security
JWT_SECRET=your_jwt_secret
ENCRYPTION_KEY=your_32_character_key

# Features
ENABLE_ANALYTICS=true
ENABLE_BMAD_AGENTS=true
ENABLE_MOBILE_API=true
```

### Performance Optimization

#### System Optimization
```bash
# Increase file descriptor limits (Linux/macOS)
echo "* soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Optimize memory settings
export NODE_OPTIONS="--max-old-space-size=4096"
```

#### Application Optimization
```json
{
  "research": {
    "defaultConcurrency": 5,
    "maxCacheSize": "1GB",
    "enablePrefetching": true
  },
  "ui": {
    "enableAnimations": true,
    "lazyLoading": true,
    "virtualScrolling": true
  }
}
```

## 🔍 Verification

### Installation Verification
```bash
# Check application version
free-deep-research --version

# Test API connectivity
curl http://localhost:3000/api/health

# Verify database connection
curl http://localhost:3000/api/status
```

### Health Checks
1. **Application Status**: Green indicator in system tray/dock
2. **API Connectivity**: Settings → API Status shows all green
3. **Database**: Settings → System Status shows database connected
4. **Research Test**: Run a simple research query

## 🚨 Troubleshooting

### Common Issues

#### Installation Fails
```bash
# Clear npm cache
npm cache clean --force

# Clear Rust cache
cargo clean

# Reinstall dependencies
rm -rf node_modules
npm install
```

#### Application Won't Start
```bash
# Check logs
tail -f ~/.local/share/free-deep-research/logs/app.log

# Reset configuration
rm -rf ~/.config/free-deep-research
```

#### API Connection Issues
1. **Check API Keys**: Verify keys are correct and active
2. **Network**: Test internet connectivity
3. **Firewall**: Ensure ports 3000, 8080 are not blocked
4. **Proxy**: Configure proxy settings if needed

#### Performance Issues
```bash
# Monitor resource usage
top -p $(pgrep free-deep-research)

# Check disk space
df -h

# Monitor memory usage
free -h
```

### Getting Help

#### Log Files
- **Windows**: `%APPDATA%\free-deep-research\logs\`
- **macOS**: `~/Library/Logs/free-deep-research/`
- **Linux**: `~/.local/share/free-deep-research/logs/`

#### Support Channels
- **GitHub Issues**: Report bugs and feature requests
- **Community Forum**: Get help from other users
- **Email Support**: support@freedeepresearch.org
- **Discord**: Real-time community support

## 🔄 Updates

### Automatic Updates
- **Desktop App**: Automatic update notifications
- **Docker**: Use `docker-compose pull && docker-compose up -d`
- **Source**: `git pull && npm install && npm run build`

### Manual Updates
1. **Backup Data**: Export important research and settings
2. **Download**: Get latest version from releases page
3. **Install**: Follow installation steps for your platform
4. **Restore**: Import backed up data if needed

---

**Installation complete!** 🎉 

Ready to start researching? Check out our [Quick Start Guide](./quick-start.md) or dive into the [Complete User Guide](./COMPLETE_USER_GUIDE_2025.md).
