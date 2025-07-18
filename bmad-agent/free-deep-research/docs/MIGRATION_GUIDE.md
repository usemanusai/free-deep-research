# Free Deep Research System - Migration Guide

## Overview

This guide provides step-by-step instructions for migrating between versions of the Free Deep Research System, including data migration, configuration updates, and breaking changes.

## Migration Paths

### Supported Migration Paths
- **1.0.x → 1.1.x**: Minor version upgrade (automatic)
- **1.1.x → 1.2.x**: Feature upgrade (semi-automatic)
- **1.2.x → 2.0.x**: Major version upgrade (manual steps required)
- **Direct 1.0.x → 2.0.x**: Full migration (comprehensive process)

### Unsupported Migrations
- Downgrading from higher to lower versions
- Skipping intermediate versions (except 1.0.x → 2.0.x)

---

## Pre-Migration Checklist

### 1. System Backup
```bash
# Create full system backup
./scripts/backup.sh --full --output /backup/pre-migration-$(date +%Y%m%d)

# Verify backup integrity
./scripts/verify-backup.sh /backup/pre-migration-$(date +%Y%m%d)

# Export current configuration
./scripts/export-config.sh --output /backup/config-$(date +%Y%m%d).json
```

### 2. Environment Assessment
```bash
# Check current version
free-deep-research --version

# Verify system requirements
./scripts/check-requirements.sh --target-version 2.0.0

# Check disk space (minimum 10GB free)
df -h

# Verify network connectivity
./scripts/check-connectivity.sh
```

### 3. Dependency Verification
```bash
# Check Rust version
rustc --version  # Should be 1.75+

# Check Node.js version
node --version   # Should be 20+

# Check database status
sqlite3 ~/.local/share/free-deep-research/research.db ".schema"
```

---

## Migration from 1.0.x to 1.1.x

### Automatic Migration Process

The 1.0.x to 1.1.x migration is fully automated and occurs during the first startup of version 1.1.x.

```bash
# Download and install 1.1.x
wget https://releases.freeresearch.ai/v1.1.0/free-deep-research-1.1.0.tar.gz
tar -xzf free-deep-research-1.1.0.tar.gz
sudo ./install.sh

# First startup triggers automatic migration
free-deep-research
```

### Migration Steps Performed
1. **Database Schema Update**: Adds new tables for enhanced analytics
2. **Configuration Migration**: Updates config format for new features
3. **Data Transformation**: Converts existing research data to new format
4. **Index Creation**: Creates new database indexes for performance

### Verification
```bash
# Check migration status
free-deep-research --migration-status

# Verify new features
free-deep-research --check-features analytics,export,collaboration
```

---

## Migration from 1.1.x to 1.2.x

### Semi-Automatic Migration

The 1.1.x to 1.2.x migration requires some manual configuration for new features.

### Step 1: Install Version 1.2.x
```bash
# Stop current instance
sudo systemctl stop free-deep-research

# Backup current installation
cp -r /opt/free-deep-research /opt/free-deep-research-1.1.x-backup

# Install new version
wget https://releases.freeresearch.ai/v1.2.0/free-deep-research-1.2.0.tar.gz
tar -xzf free-deep-research-1.2.0.tar.gz
sudo ./install.sh --upgrade
```

### Step 2: Configure New Features

#### Plugin System Configuration
```bash
# Create plugin configuration
cat > ~/.config/free-deep-research/plugins.toml << EOF
[plugin_system]
enabled = true
marketplace_enabled = true
sandbox_enabled = true
max_concurrent_plugins = 10

[security]
allowed_permissions = [
    "network_access",
    "file_read",
    "api_access"
]
security_level = "balanced"
EOF
```

#### Workflow Engine Configuration
```bash
# Create workflow configuration
cat > ~/.config/free-deep-research/workflows.toml << EOF
[workflow_engine]
enabled = true
max_concurrent_workflows = 50
auto_save_enabled = true
validation_enabled = true

[execution]
default_timeout_minutes = 60
max_execution_time_minutes = 120
debug_mode = false
EOF
```

#### ML Engine Configuration
```bash
# Create ML configuration
cat > ~/.config/free-deep-research/ml.toml << EOF
[ml_engine]
enabled = true
max_concurrent_training_jobs = 5
auto_retrain_enabled = true
model_explainability_enabled = true

[training]
default_algorithm = "random_forest"
hyperparameter_tuning = true
cross_validation_folds = 5
EOF
```

### Step 3: Data Migration
```bash
# Run data migration script
./scripts/migrate-1.1-to-1.2.sh

# Verify migration
./scripts/verify-migration.sh --from 1.1 --to 1.2
```

### Step 4: Start Services
```bash
# Start with migration mode
free-deep-research --migrate --from 1.1.x

# Verify all services
free-deep-research --health-check
```

---

## Migration from 1.2.x to 2.0.x

### Major Version Migration

This is a significant upgrade requiring manual intervention and careful planning.

### Step 1: Pre-Migration Assessment
```bash
# Assess current system
./scripts/assess-for-2.0.sh

# Check compatibility
./scripts/check-2.0-compatibility.sh

# Estimate migration time
./scripts/estimate-migration-time.sh
```

### Step 2: Export Current Data
```bash
# Export all workflows
./scripts/export-workflows.sh --output /migration/workflows.json

# Export user data
./scripts/export-users.sh --output /migration/users.json

# Export configuration
./scripts/export-config.sh --output /migration/config.json

# Export analytics data
./scripts/export-analytics.sh --output /migration/analytics.json
```

### Step 3: Install Version 2.0.x

#### Option A: In-Place Upgrade
```bash
# Stop all services
sudo systemctl stop free-deep-research

# Create backup
sudo cp -r /opt/free-deep-research /opt/free-deep-research-1.2.x-backup

# Install 2.0.x
wget https://releases.freeresearch.ai/v2.0.0/free-deep-research-2.0.0.tar.gz
tar -xzf free-deep-research-2.0.0.tar.gz
sudo ./install.sh --major-upgrade --from 1.2.x
```

#### Option B: Side-by-Side Installation
```bash
# Install to new directory
sudo mkdir /opt/free-deep-research-2.0
sudo tar -xzf free-deep-research-2.0.0.tar.gz -C /opt/free-deep-research-2.0
cd /opt/free-deep-research-2.0
sudo ./install.sh --new-installation --port 8081
```

### Step 4: Database Migration

#### Schema Migration
```sql
-- Connect to database
sqlite3 ~/.local/share/free-deep-research/research.db

-- Check current schema version
SELECT version FROM schema_version;

-- Run migration scripts
.read migrations/1.2.x-to-2.0.x/001-add-distributed-tables.sql
.read migrations/1.2.x-to-2.0.x/002-add-ai-orchestration-tables.sql
.read migrations/1.2.x-to-2.0.x/003-add-realtime-collaboration-tables.sql
.read migrations/1.2.x-to-2.0.x/004-update-indexes.sql

-- Verify migration
SELECT version FROM schema_version;
```

#### Data Transformation
```bash
# Transform workflow data for new engine
./scripts/transform-workflows-2.0.sh /migration/workflows.json

# Transform user data for enterprise features
./scripts/transform-users-2.0.sh /migration/users.json

# Transform analytics for new dashboard
./scripts/transform-analytics-2.0.sh /migration/analytics.json
```

### Step 5: Configuration Migration

#### Convert Configuration Format
```bash
# Convert TOML to YAML
./scripts/convert-config.sh \
  --input ~/.config/free-deep-research/config.toml \
  --output ~/.config/free-deep-research/config.yaml \
  --format yaml

# Update for 2.0.x features
./scripts/update-config-2.0.sh ~/.config/free-deep-research/config.yaml
```

#### New Configuration Sections
```yaml
# ~/.config/free-deep-research/config.yaml
distributed:
  enabled: true
  cluster_name: "research-cluster"
  node_id: "node-001"
  enable_service_mesh: true
  enable_auto_scaling: true

ai_orchestration:
  enabled: true
  max_concurrent_agents: 100
  communication_protocol: "grpc"
  coordination_strategy: "consensus"

realtime_collaboration:
  enabled: true
  websocket_port: 8080
  max_concurrent_sessions: 100
  enable_operational_transform: true

security:
  zero_knowledge_enabled: true
  end_to_end_encryption: true
  quantum_resistant: true
```

### Step 6: Service Migration

#### Distributed Services Setup
```bash
# Initialize cluster
./scripts/init-cluster.sh --cluster-name research-cluster

# Deploy core services
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/core-services.yaml

# Deploy application services
kubectl apply -f k8s/research-engine.yaml
kubectl apply -f k8s/ai-orchestration.yaml
kubectl apply -f k8s/realtime-collaboration.yaml
```

#### Data Import
```bash
# Import workflows
./scripts/import-workflows-2.0.sh /migration/workflows-transformed.json

# Import users
./scripts/import-users-2.0.sh /migration/users-transformed.json

# Import analytics
./scripts/import-analytics-2.0.sh /migration/analytics-transformed.json
```

### Step 7: Verification and Testing

#### Functional Testing
```bash
# Test core functionality
./scripts/test-core-features.sh

# Test new 2.0.x features
./scripts/test-distributed-features.sh
./scripts/test-ai-orchestration.sh
./scripts/test-realtime-collaboration.sh

# Performance testing
./scripts/performance-test.sh --concurrent-users 100
```

#### Data Integrity Verification
```bash
# Verify workflow data
./scripts/verify-workflows.sh

# Verify user data
./scripts/verify-users.sh

# Verify analytics data
./scripts/verify-analytics.sh

# Check for data loss
./scripts/check-data-integrity.sh
```

---

## Breaking Changes by Version

### Version 1.1.x Breaking Changes
- **API Endpoints**: New analytics endpoints added
- **Database Schema**: New tables for enhanced features
- **Configuration**: New sections for collaboration and mobile

### Version 1.2.x Breaking Changes
- **Plugin API**: New plugin interface for security
- **Workflow Format**: Enhanced workflow definition schema
- **ML Models**: New model format for training data
- **Cloud Sync**: Updated sync protocol for encryption

### Version 2.0.x Breaking Changes
- **Architecture**: Distributed architecture replaces single-node
- **API Format**: REST API replaces some Tauri commands
- **Database**: PostgreSQL replaces SQLite for distributed deployments
- **Configuration**: YAML format replaces TOML
- **Authentication**: Enterprise SSO replaces simple auth
- **Real-time**: WebSocket protocol for collaboration

---

## Rollback Procedures

### Rollback from 1.1.x to 1.0.x
```bash
# Stop current version
sudo systemctl stop free-deep-research

# Restore backup
sudo rm -rf /opt/free-deep-research
sudo cp -r /opt/free-deep-research-1.0.x-backup /opt/free-deep-research

# Restore database
cp ~/.local/share/free-deep-research/research.db.backup \
   ~/.local/share/free-deep-research/research.db

# Start service
sudo systemctl start free-deep-research
```

### Rollback from 1.2.x to 1.1.x
```bash
# Export current data (if needed)
./scripts/export-for-rollback.sh

# Stop services
sudo systemctl stop free-deep-research

# Restore previous version
sudo rm -rf /opt/free-deep-research
sudo cp -r /opt/free-deep-research-1.1.x-backup /opt/free-deep-research

# Restore database
./scripts/restore-database.sh --from-backup /backup/pre-migration-*

# Restart services
sudo systemctl start free-deep-research
```

### Rollback from 2.0.x to 1.2.x
```bash
# This is a complex rollback requiring data transformation
./scripts/rollback-2.0-to-1.2.sh --backup-path /backup/pre-migration-*

# Manual verification required
./scripts/verify-rollback.sh --target-version 1.2.x
```

---

## Troubleshooting Migration Issues

### Common Migration Problems

#### Database Migration Failures
```bash
# Check database integrity
sqlite3 ~/.local/share/free-deep-research/research.db "PRAGMA integrity_check;"

# Repair database if needed
sqlite3 ~/.local/share/free-deep-research/research.db ".recover" | \
  sqlite3 ~/.local/share/free-deep-research/research-recovered.db

# Retry migration
./scripts/retry-migration.sh --from-backup
```

#### Configuration Migration Issues
```bash
# Validate configuration
./scripts/validate-config.sh ~/.config/free-deep-research/config.yaml

# Reset to defaults if needed
./scripts/reset-config.sh --backup-current

# Manually merge settings
./scripts/merge-config.sh \
  --old ~/.config/free-deep-research/config.toml.backup \
  --new ~/.config/free-deep-research/config.yaml
```

#### Service Startup Issues
```bash
# Check service status
systemctl status free-deep-research

# Check logs
journalctl -u free-deep-research -f

# Debug mode startup
free-deep-research --debug --verbose
```

### Recovery Procedures

#### Partial Migration Recovery
```bash
# Identify failed components
./scripts/check-migration-status.sh

# Retry specific components
./scripts/retry-component-migration.sh --component workflows
./scripts/retry-component-migration.sh --component users
./scripts/retry-component-migration.sh --component analytics
```

#### Complete Migration Recovery
```bash
# Full rollback and retry
./scripts/full-rollback.sh --to-version 1.2.x
./scripts/clean-migration-state.sh
./scripts/retry-full-migration.sh --to-version 2.0.x
```

---

## Post-Migration Tasks

### Performance Optimization
```bash
# Rebuild database indexes
./scripts/rebuild-indexes.sh

# Optimize database
./scripts/optimize-database.sh

# Clear caches
./scripts/clear-caches.sh
```

### Security Updates
```bash
# Regenerate encryption keys
./scripts/regenerate-keys.sh

# Update certificates
./scripts/update-certificates.sh

# Audit security settings
./scripts/security-audit.sh
```

### User Communication
```bash
# Generate migration report
./scripts/generate-migration-report.sh --output /reports/migration-$(date +%Y%m%d).html

# Notify users of changes
./scripts/notify-users.sh --template migration-complete

# Update documentation
./scripts/update-user-docs.sh --version 2.0.x
```

---

## Migration Support

### Getting Help
- **Documentation**: https://docs.freeresearch.ai/migration
- **Community Forum**: https://community.freeresearch.ai
- **GitHub Issues**: https://github.com/usemanusai/free-deep-research/issues
- **Enterprise Support**: support@freeresearch.ai

### Professional Migration Services
For complex migrations or enterprise deployments, professional migration services are available:
- **Migration Assessment**: Comprehensive pre-migration analysis
- **Assisted Migration**: Expert-guided migration process
- **Custom Migration**: Tailored migration for unique requirements
- **Post-Migration Support**: Ongoing support after migration

Contact: migration-services@freeresearch.ai
