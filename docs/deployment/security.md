# 🔒 Deployment Security Guide

## Overview

This guide provides comprehensive security configurations and best practices for deploying the Free Deep Research System in production environments. Follow these guidelines to ensure a secure deployment.

## 🛡️ Security Checklist

### Pre-Deployment Security Checklist

- [ ] **SSL/TLS Configuration**: Valid certificates installed and configured
- [ ] **Firewall Rules**: Proper network access controls implemented
- [ ] **Secret Management**: All secrets properly managed and rotated
- [ ] **Database Security**: Database hardened and access controlled
- [ ] **Container Security**: Containers scanned and hardened
- [ ] **Monitoring**: Security monitoring and alerting configured
- [ ] **Backup Security**: Backups encrypted and access controlled
- [ ] **Access Controls**: Proper authentication and authorization
- [ ] **Network Segmentation**: Proper network isolation implemented
- [ ] **Security Headers**: All security headers configured

## 🔐 SSL/TLS Configuration

### Certificate Management

#### Let's Encrypt with Certbot
```bash
# Install Certbot
sudo apt-get update
sudo apt-get install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot --nginx -d your-domain.com -d www.your-domain.com

# Auto-renewal setup
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

#### Nginx SSL Configuration
```nginx
# /etc/nginx/sites-available/free-deep-research
server {
    listen 443 ssl http2;
    server_name your-domain.com www.your-domain.com;

    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/your-domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/your-domain.com/privkey.pem;
    
    # SSL Security Settings
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-SHA384;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    
    # HSTS
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    
    # Security Headers
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'; connect-src 'self' https:; frame-ancestors 'none';" always;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Security headers for proxied content
        proxy_hide_header X-Powered-By;
        proxy_hide_header Server;
    }
}

# Redirect HTTP to HTTPS
server {
    listen 80;
    server_name your-domain.com www.your-domain.com;
    return 301 https://$server_name$request_uri;
}
```

### Docker SSL Configuration
```yaml
# docker-compose.yml with SSL
version: '3.8'

services:
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf
      - ./nginx/ssl:/etc/nginx/ssl
      - /etc/letsencrypt:/etc/letsencrypt:ro
    depends_on:
      - app
    restart: unless-stopped

  app:
    build: .
    environment:
      - NODE_ENV=production
      - FORCE_HTTPS=true
      - TRUST_PROXY=true
    expose:
      - "3000"
    restart: unless-stopped
```

## 🔥 Firewall Configuration

### UFW (Ubuntu Firewall)
```bash
# Reset firewall rules
sudo ufw --force reset

# Default policies
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH (change port if using non-standard)
sudo ufw allow 22/tcp

# Allow HTTP and HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Allow specific application ports (if needed)
sudo ufw allow from 10.0.0.0/8 to any port 5432  # PostgreSQL from internal network
sudo ufw allow from 10.0.0.0/8 to any port 6379  # Redis from internal network

# Enable firewall
sudo ufw enable

# Check status
sudo ufw status verbose
```

### iptables Configuration
```bash
#!/bin/bash
# iptables-rules.sh

# Flush existing rules
iptables -F
iptables -X
iptables -t nat -F
iptables -t nat -X

# Default policies
iptables -P INPUT DROP
iptables -P FORWARD DROP
iptables -P OUTPUT ACCEPT

# Allow loopback
iptables -A INPUT -i lo -j ACCEPT
iptables -A OUTPUT -o lo -j ACCEPT

# Allow established connections
iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT

# Allow SSH
iptables -A INPUT -p tcp --dport 22 -j ACCEPT

# Allow HTTP and HTTPS
iptables -A INPUT -p tcp --dport 80 -j ACCEPT
iptables -A INPUT -p tcp --dport 443 -j ACCEPT

# Rate limiting for SSH
iptables -A INPUT -p tcp --dport 22 -m state --state NEW -m recent --set
iptables -A INPUT -p tcp --dport 22 -m state --state NEW -m recent --update --seconds 60 --hitcount 4 -j DROP

# Save rules
iptables-save > /etc/iptables/rules.v4
```

## 🔑 Secret Management

### Environment Variables Security
```bash
# .env.production (secure configuration)
# Database
DATABASE_URL=postgresql://user:$(cat /run/secrets/db_password)@postgres:5432/freedeepresearch

# API Keys (use secrets management)
OPENROUTER_API_KEY_FILE=/run/secrets/openrouter_api_key
SERPAPI_KEY_FILE=/run/secrets/serpapi_key
TAVILY_API_KEY_FILE=/run/secrets/tavily_api_key

# Security
JWT_SECRET_FILE=/run/secrets/jwt_secret
ENCRYPTION_KEY_FILE=/run/secrets/encryption_key
SESSION_SECRET_FILE=/run/secrets/session_secret

# Security settings
SECURE_COOKIES=true
CSRF_PROTECTION=true
RATE_LIMITING=true
SECURITY_HEADERS=true
```

### Docker Secrets
```yaml
# docker-compose.yml with secrets
version: '3.8'

services:
  app:
    image: free-deep-research:latest
    secrets:
      - db_password
      - jwt_secret
      - encryption_key
      - openrouter_api_key
    environment:
      - DATABASE_PASSWORD_FILE=/run/secrets/db_password
      - JWT_SECRET_FILE=/run/secrets/jwt_secret
      - ENCRYPTION_KEY_FILE=/run/secrets/encryption_key
      - OPENROUTER_API_KEY_FILE=/run/secrets/openrouter_api_key

secrets:
  db_password:
    file: ./secrets/db_password.txt
  jwt_secret:
    file: ./secrets/jwt_secret.txt
  encryption_key:
    file: ./secrets/encryption_key.txt
  openrouter_api_key:
    file: ./secrets/openrouter_api_key.txt
```

### Kubernetes Secrets
```yaml
# secrets.yaml
apiVersion: v1
kind: Secret
metadata:
  name: app-secrets
type: Opaque
data:
  database-password: <base64-encoded-password>
  jwt-secret: <base64-encoded-secret>
  encryption-key: <base64-encoded-key>
  openrouter-api-key: <base64-encoded-key>

---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: free-deep-research
spec:
  template:
    spec:
      containers:
      - name: app
        image: free-deep-research:latest
        env:
        - name: DATABASE_PASSWORD
          valueFrom:
            secretKeyRef:
              name: app-secrets
              key: database-password
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: app-secrets
              key: jwt-secret
```

## 🗄️ Database Security

### PostgreSQL Security Configuration
```sql
-- postgresql.conf security settings
ssl = on
ssl_cert_file = '/etc/ssl/certs/server.crt'
ssl_key_file = '/etc/ssl/private/server.key'
ssl_ciphers = 'ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-AES128-GCM-SHA256'
ssl_prefer_server_ciphers = on

-- Connection security
listen_addresses = 'localhost'  # Only local connections
port = 5432
max_connections = 100

-- Authentication
password_encryption = scram-sha-256
```

```sql
-- pg_hba.conf security settings
# TYPE  DATABASE        USER            ADDRESS                 METHOD
local   all             postgres                                peer
local   all             all                                     scram-sha-256
host    freedeepresearch app_user       127.0.0.1/32           scram-sha-256
host    freedeepresearch app_user       ::1/128                 scram-sha-256
hostssl freedeepresearch app_user       10.0.0.0/8             scram-sha-256
```

### Database User Security
```sql
-- Create application user with limited privileges
CREATE USER app_user WITH PASSWORD 'secure_random_password';

-- Create database
CREATE DATABASE freedeepresearch OWNER app_user;

-- Grant only necessary privileges
GRANT CONNECT ON DATABASE freedeepresearch TO app_user;
GRANT USAGE ON SCHEMA public TO app_user;
GRANT CREATE ON SCHEMA public TO app_user;

-- Revoke unnecessary privileges
REVOKE ALL ON DATABASE postgres FROM app_user;
REVOKE ALL ON SCHEMA information_schema FROM app_user;
REVOKE ALL ON SCHEMA pg_catalog FROM app_user;
```

## 🐳 Container Security

### Dockerfile Security Best Practices
```dockerfile
# Use specific version tags
FROM node:18.19.0-alpine3.19 AS base

# Create non-root user
RUN addgroup -g 1001 -S appgroup && \
    adduser -S appuser -u 1001 -G appgroup

# Install security updates
RUN apk update && apk upgrade && \
    apk add --no-cache dumb-init && \
    rm -rf /var/cache/apk/*

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production && \
    npm cache clean --force

# Copy application code
COPY --chown=appuser:appgroup . .

# Remove unnecessary files
RUN rm -rf .git .gitignore README.md docs/ tests/

# Set security headers
ENV NODE_ENV=production
ENV SECURE_HEADERS=true

# Use non-root user
USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Use dumb-init
ENTRYPOINT ["dumb-init", "--"]
CMD ["node", "server.js"]
```

### Container Scanning
```bash
# Scan Docker images for vulnerabilities
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  -v $HOME/Library/Caches:/root/.cache/ \
  aquasec/trivy image free-deep-research:latest

# Scan with Snyk
snyk container test free-deep-research:latest

# Scan with Clair
clairctl analyze free-deep-research:latest
```

## 📊 Security Monitoring

### Log Configuration
```yaml
# docker-compose.yml with logging
version: '3.8'

services:
  app:
    image: free-deep-research:latest
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
    environment:
      - LOG_LEVEL=info
      - AUDIT_LOGGING=true
      - SECURITY_LOGGING=true

  nginx:
    image: nginx:alpine
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf
```

### Security Monitoring with Fail2Ban
```ini
# /etc/fail2ban/jail.local
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 3

[nginx-http-auth]
enabled = true
filter = nginx-http-auth
logpath = /var/log/nginx/error.log
maxretry = 3

[nginx-limit-req]
enabled = true
filter = nginx-limit-req
logpath = /var/log/nginx/error.log
maxretry = 10

[sshd]
enabled = true
port = ssh
logpath = /var/log/auth.log
maxretry = 3
```

### Intrusion Detection with OSSEC
```xml
<!-- /var/ossec/etc/ossec.conf -->
<ossec_config>
  <global>
    <email_notification>yes</email_notification>
    <smtp_server>localhost</smtp_server>
    <email_from>ossec@your-domain.com</email_from>
    <email_to>admin@your-domain.com</email_to>
  </global>

  <rules>
    <include>rules_config.xml</include>
    <include>pam_rules.xml</include>
    <include>sshd_rules.xml</include>
    <include>telnetd_rules.xml</include>
    <include>syslog_rules.xml</include>
    <include>arpwatch_rules.xml</include>
    <include>symantec-av_rules.xml</include>
    <include>symantec-ws_rules.xml</include>
    <include>pix_rules.xml</include>
    <include>named_rules.xml</include>
    <include>smbd_rules.xml</include>
    <include>vsftpd_rules.xml</include>
    <include>pure-ftpd_rules.xml</include>
    <include>proftpd_rules.xml</include>
    <include>ms_ftpd_rules.xml</include>
    <include>ftpd_rules.xml</include>
    <include>hordeimp_rules.xml</include>
    <include>roundcube_rules.xml</include>
    <include>wordpress_rules.xml</include>
    <include>cimserver_rules.xml</include>
    <include>vpopmail_rules.xml</include>
    <include>vmpop3d_rules.xml</include>
    <include>courier_rules.xml</include>
    <include>web_rules.xml</include>
    <include>web_appsec_rules.xml</include>
    <include>apache_rules.xml</include>
    <include>nginx_rules.xml</include>
    <include>php_rules.xml</include>
    <include>mysql_rules.xml</include>
    <include>postgresql_rules.xml</include>
    <include>ids_rules.xml</include>
    <include>squid_rules.xml</include>
    <include>firewall_rules.xml</include>
    <include>cisco-ios_rules.xml</include>
    <include>netscreenfw_rules.xml</include>
    <include>sonicwall_rules.xml</include>
    <include>postfix_rules.xml</include>
    <include>sendmail_rules.xml</include>
    <include>imapd_rules.xml</include>
    <include>mailscanner_rules.xml</include>
    <include>dovecot_rules.xml</include>
    <include>ms-exchange_rules.xml</include>
    <include>racoon_rules.xml</include>
    <include>vpn_concentrator_rules.xml</include>
    <include>spamd_rules.xml</include>
    <include>msauth_rules.xml</include>
    <include>mcafee_av_rules.xml</include>
    <include>trend-osce_rules.xml</include>
    <include>ms-se_rules.xml</include>
    <include>zeus_rules.xml</include>
    <include>solaris_bsm_rules.xml</include>
    <include>vmware_rules.xml</include>
    <include>ms_dhcp_rules.xml</include>
    <include>asterisk_rules.xml</include>
    <include>ossec_rules.xml</include>
    <include>attack_rules.xml</include>
    <include>local_rules.xml</include>
  </rules>

  <syscheck>
    <directories check_all="yes">/etc,/usr/bin,/usr/sbin</directories>
    <directories check_all="yes">/bin,/sbin</directories>
    <directories check_all="yes">/app</directories>
  </syscheck>

  <rootcheck>
    <rootkit_files>/var/ossec/etc/shared/rootkit_files.txt</rootkit_files>
    <rootkit_trojans>/var/ossec/etc/shared/rootkit_trojans.txt</rootkit_trojans>
  </rootcheck>

  <localfile>
    <log_format>syslog</log_format>
    <location>/var/log/auth.log</location>
  </localfile>

  <localfile>
    <log_format>syslog</log_format>
    <location>/var/log/syslog</location>
  </localfile>

  <localfile>
    <log_format>apache</log_format>
    <location>/var/log/nginx/access.log</location>
  </localfile>

  <localfile>
    <log_format>apache</log_format>
    <location>/var/log/nginx/error.log</location>
  </localfile>
</ossec_config>
```

## 🔄 Security Maintenance

### Regular Security Tasks

#### Weekly Tasks
```bash
#!/bin/bash
# weekly-security-maintenance.sh

# Update system packages
sudo apt update && sudo apt upgrade -y

# Update Docker images
docker-compose pull
docker-compose up -d

# Rotate logs
sudo logrotate -f /etc/logrotate.conf

# Check for failed login attempts
sudo grep "Failed password" /var/log/auth.log | tail -20

# Check SSL certificate expiry
openssl x509 -in /etc/letsencrypt/live/your-domain.com/cert.pem -noout -dates
```

#### Monthly Tasks
```bash
#!/bin/bash
# monthly-security-maintenance.sh

# Security audit
sudo lynis audit system

# Check for rootkits
sudo rkhunter --check

# Vulnerability scan
nmap -sS -O localhost

# Review user accounts
sudo cat /etc/passwd | grep -v nologin

# Check for unauthorized SUID files
sudo find / -perm -4000 -type f 2>/dev/null
```

### Backup Security
```bash
#!/bin/bash
# secure-backup.sh

# Encrypt database backup
pg_dump freedeepresearch | gpg --cipher-algo AES256 --compress-algo 1 --symmetric --output backup-$(date +%Y%m%d).sql.gpg

# Secure file permissions
chmod 600 backup-*.sql.gpg

# Upload to secure storage
aws s3 cp backup-$(date +%Y%m%d).sql.gpg s3://secure-backups/ --sse AES256

# Clean up local backup
rm backup-$(date +%Y%m%d).sql.gpg
```

---

**Security is an ongoing process!** Regular monitoring, updates, and security assessments are essential for maintaining a secure deployment. For security incidents, contact [security@freedeepresearch.org](mailto:security@freedeepresearch.org).
