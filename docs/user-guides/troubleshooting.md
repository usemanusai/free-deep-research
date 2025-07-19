# 🔧 Troubleshooting Guide

## Overview

This guide helps you diagnose and resolve common issues with the Free Deep Research System. Follow the step-by-step solutions to get back up and running quickly.

## 🚨 Quick Diagnostics

### System Health Check
Run these commands to check system status:

```bash
# Check application status
curl http://localhost:3000/api/health

# Check API connectivity
curl http://localhost:3000/api/status

# Check logs
tail -f ~/.local/share/free-deep-research/logs/app.log
```

### Common Symptoms & Quick Fixes

| Symptom | Quick Fix |
|---------|-----------|
| App won't start | Check [Installation Issues](#installation-issues) |
| Research not working | Verify [API Configuration](#api-configuration-issues) |
| Slow performance | See [Performance Issues](#performance-issues) |
| Login problems | Check [Authentication Issues](#authentication-issues) |
| Export failing | Review [Export Issues](#export-issues) |

## 🔧 Installation Issues

### Desktop Application Won't Start

#### Windows Issues
```powershell
# Check if app is running
Get-Process -Name "free-deep-research" -ErrorAction SilentlyContinue

# Clear application data
Remove-Item -Recurse -Force "$env:APPDATA\free-deep-research"

# Reinstall with admin privileges
Start-Process -FilePath "Free-Deep-Research-Setup.exe" -Verb RunAs
```

#### macOS Issues
```bash
# Check security permissions
spctl -a "/Applications/Free Deep Research.app"

# Reset quarantine
sudo xattr -rd com.apple.quarantine "/Applications/Free Deep Research.app"

# Clear application data
rm -rf ~/Library/Application\ Support/free-deep-research
```

#### Linux Issues
```bash
# Check dependencies
ldd /usr/bin/free-deep-research

# Install missing dependencies (Ubuntu/Debian)
sudo apt-get install libgtk-3-0 libwebkit2gtk-4.0-37

# Check permissions
chmod +x /usr/bin/free-deep-research
```

### Docker Installation Issues

#### Container Won't Start
```bash
# Check Docker status
docker --version
docker-compose --version

# Check container logs
docker-compose logs app

# Rebuild containers
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

#### Port Conflicts
```bash
# Check port usage
netstat -tulpn | grep :3000
lsof -i :3000

# Kill processes using ports
sudo kill -9 $(lsof -t -i:3000)

# Use different ports
export APP_PORT=3001
docker-compose up -d
```

## 🔑 API Configuration Issues

### API Keys Not Working

#### Verify API Key Format
```bash
# OpenRouter key format
OPENROUTER_API_KEY=sk-or-v1-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

# SerpAPI key format  
SERPAPI_KEY=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

# Tavily key format
TAVILY_API_KEY=tvly-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

#### Test API Connectivity
```bash
# Test OpenRouter
curl -H "Authorization: Bearer $OPENROUTER_API_KEY" \
     https://openrouter.ai/api/v1/models

# Test SerpAPI
curl "https://serpapi.com/search.json?q=test&api_key=$SERPAPI_KEY"

# Test Tavily
curl -X POST "https://api.tavily.com/search" \
     -H "Content-Type: application/json" \
     -d '{"api_key":"'$TAVILY_API_KEY'","query":"test"}'
```

#### API Key Configuration Locations
- **Desktop App**: Settings → API Configuration
- **Docker**: `.env` file in project root
- **Environment**: System environment variables

### Rate Limiting Issues

#### Symptoms
- "Rate limit exceeded" errors
- Slow or failed research requests
- API timeout messages

#### Solutions
```bash
# Check current usage
curl -H "Authorization: Bearer $OPENROUTER_API_KEY" \
     https://openrouter.ai/api/v1/auth/key

# Reduce concurrent requests
export MAX_CONCURRENT_REQUESTS=3

# Implement request delays
export REQUEST_DELAY_MS=1000
```

## 🐌 Performance Issues

### Slow Research Performance

#### Optimize Research Parameters
```typescript
// Reduce source count for faster results
const optimizedParams = {
  maxSources: 15,        // Instead of 50
  methodology: 'hybrid', // Balanced approach
  timeout: 300000,       // 5 minutes max
  concurrent: 3          // Limit concurrent requests
};
```

#### System Resource Optimization
```bash
# Check system resources
top -p $(pgrep free-deep-research)
free -h
df -h

# Increase Node.js memory limit
export NODE_OPTIONS="--max-old-space-size=4096"

# Clear application cache
rm -rf ~/.cache/free-deep-research
```

### Database Performance Issues

#### PostgreSQL Optimization
```sql
-- Check database performance
SELECT * FROM pg_stat_activity WHERE state = 'active';

-- Optimize queries
ANALYZE;
VACUUM;

-- Check index usage
SELECT schemaname, tablename, attname, n_distinct, correlation 
FROM pg_stats WHERE tablename = 'research_workflows';
```

#### Redis Cache Issues
```bash
# Check Redis status
redis-cli ping

# Clear cache if needed
redis-cli flushall

# Monitor Redis memory
redis-cli info memory
```

## 🔐 Authentication Issues

### Login Problems

#### Password Reset
```bash
# Request password reset
curl -X POST http://localhost:3000/api/auth/reset-password \
     -H "Content-Type: application/json" \
     -d '{"email":"your-email@example.com"}'
```

#### Session Issues
```bash
# Clear browser cookies and localStorage
# In browser console:
localStorage.clear();
sessionStorage.clear();

# Clear application session data
rm -rf ~/.config/free-deep-research/sessions
```

#### OAuth Integration Issues
```bash
# Check OAuth configuration
curl http://localhost:3000/api/auth/oauth/providers

# Test OAuth callback
curl "http://localhost:3000/api/auth/oauth/callback?code=test&state=test"
```

## 📄 Export Issues

### PDF Export Problems

#### Missing Dependencies
```bash
# Install PDF generation dependencies
npm install puppeteer

# For Docker
docker-compose exec app npm install puppeteer
```

#### Font Issues
```bash
# Install system fonts (Linux)
sudo apt-get install fonts-liberation fonts-dejavu

# For Docker, add to Dockerfile:
RUN apt-get update && apt-get install -y fonts-liberation
```

### File Permission Issues
```bash
# Check export directory permissions
ls -la ~/.local/share/free-deep-research/exports

# Fix permissions
chmod 755 ~/.local/share/free-deep-research/exports
chown $USER:$USER ~/.local/share/free-deep-research/exports
```

## 🌐 Network Issues

### Connectivity Problems

#### Proxy Configuration
```bash
# Set proxy for npm
npm config set proxy http://proxy.company.com:8080
npm config set https-proxy http://proxy.company.com:8080

# Set proxy for application
export HTTP_PROXY=http://proxy.company.com:8080
export HTTPS_PROXY=http://proxy.company.com:8080
```

#### Firewall Issues
```bash
# Check if ports are blocked
telnet api.openrouter.ai 443
telnet serpapi.com 443

# Common ports to allow
# 80, 443 (HTTP/HTTPS)
# 3000 (Application)
# 8080 (API)
# 5432 (PostgreSQL)
# 6379 (Redis)
```

### DNS Resolution Issues
```bash
# Test DNS resolution
nslookup api.openrouter.ai
dig api.openrouter.ai

# Use alternative DNS
echo "nameserver 8.8.8.8" | sudo tee /etc/resolv.conf
```

## 💾 Data Issues

### Database Connection Problems

#### PostgreSQL Issues
```bash
# Check PostgreSQL status
sudo systemctl status postgresql

# Test connection
psql -h localhost -U research_user -d free_deep_research

# Reset database
dropdb free_deep_research
createdb free_deep_research
```

#### Data Corruption
```bash
# Backup current data
pg_dump free_deep_research > backup.sql

# Restore from backup
psql free_deep_research < backup.sql

# Check data integrity
psql -c "SELECT COUNT(*) FROM research_workflows;"
```

### Storage Issues
```bash
# Check disk space
df -h

# Clean up old research data
find ~/.local/share/free-deep-research -name "*.tmp" -delete

# Compress old logs
gzip ~/.local/share/free-deep-research/logs/*.log
```

## 🔍 Debugging Tools

### Log Analysis
```bash
# Application logs
tail -f ~/.local/share/free-deep-research/logs/app.log

# Error logs only
grep -i error ~/.local/share/free-deep-research/logs/app.log

# API request logs
grep -i "api request" ~/.local/share/free-deep-research/logs/app.log
```

### Browser Developer Tools
```javascript
// Check for JavaScript errors
console.log("Checking for errors...");

// Monitor network requests
// Open Network tab in DevTools

// Check local storage
console.log(localStorage.getItem('free-deep-research-config'));
```

### System Monitoring
```bash
# Monitor resource usage
htop

# Monitor network connections
netstat -tulpn | grep free-deep-research

# Monitor file system usage
iotop
```

## 🆘 Getting Additional Help

### Collecting Debug Information

#### System Information
```bash
# Create debug report
cat > debug-report.txt << EOF
System: $(uname -a)
Node.js: $(node --version)
NPM: $(npm --version)
Docker: $(docker --version)
Application Version: $(free-deep-research --version)
EOF
```

#### Log Collection
```bash
# Collect all relevant logs
mkdir debug-logs
cp ~/.local/share/free-deep-research/logs/* debug-logs/
cp /var/log/docker.log debug-logs/ 2>/dev/null || true
tar -czf debug-logs.tar.gz debug-logs/
```

### Support Channels

#### Self-Service
1. **Search Documentation**: Check [FAQ](./faq.md) and guides
2. **Community Forum**: Search existing discussions
3. **GitHub Issues**: Check for known issues

#### Direct Support
1. **Email**: support@freedeepresearch.org
   - Include debug information
   - Describe steps to reproduce
   - Attach relevant logs

2. **Discord**: Real-time community support
   - #troubleshooting channel
   - Screen sharing available

3. **GitHub Issues**: Technical problems
   - Use issue templates
   - Provide minimal reproduction case

### Emergency Procedures

#### Complete System Reset
```bash
# Backup important data first
mkdir backup
cp -r ~/.local/share/free-deep-research/research backup/

# Remove all application data
rm -rf ~/.local/share/free-deep-research
rm -rf ~/.config/free-deep-research
rm -rf ~/.cache/free-deep-research

# Reinstall application
# Follow installation guide
```

#### Recovery Mode
```bash
# Start in safe mode (minimal features)
free-deep-research --safe-mode

# Start with clean configuration
free-deep-research --reset-config

# Start with debug logging
DEBUG=* free-deep-research
```

---

**Still having issues?** 

Don't hesitate to reach out:
- 📧 **Email**: [support@freedeepresearch.org](mailto:support@freedeepresearch.org)
- 💬 **Discord**: [Join our community](https://discord.gg/freedeepresearch)
- 🐛 **GitHub**: [Report a bug](https://github.com/huggingfacer04/free-deep-research/issues)

Include your debug information and steps to reproduce the issue for faster resolution!
