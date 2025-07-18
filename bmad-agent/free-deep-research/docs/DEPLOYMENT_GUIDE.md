# Free Deep Research System - Deployment Guide

## Overview

This guide covers deployment strategies for all versions of the Free Deep Research System, from single-node desktop applications to distributed enterprise deployments.

## Prerequisites

### System Requirements

#### Minimum Requirements (V1.x)
- **CPU**: 4 cores, 2.0 GHz
- **Memory**: 8 GB RAM
- **Storage**: 50 GB available space
- **OS**: Windows 10+, macOS 10.15+, Ubuntu 20.04+

#### Recommended Requirements (V2.x)
- **CPU**: 16 cores, 3.0 GHz
- **Memory**: 32 GB RAM
- **Storage**: 500 GB SSD
- **Network**: 1 Gbps connection
- **OS**: Latest stable versions

#### Enterprise Requirements (V2.x Distributed)
- **Nodes**: 3+ nodes for high availability
- **CPU**: 32+ cores per node
- **Memory**: 128+ GB RAM per node
- **Storage**: 2+ TB NVMe SSD per node
- **Network**: 10 Gbps connection with redundancy

### Software Dependencies

#### Development Environment
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable

# Node.js and npm
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Tauri CLI
cargo install tauri-cli

# Additional tools
sudo apt-get install -y build-essential libssl-dev pkg-config
```

#### Runtime Dependencies
```bash
# Database
sudo apt-get install -y sqlite3 libsqlite3-dev

# Encryption libraries
sudo apt-get install -y libsodium-dev

# WebRTC (for real-time collaboration)
sudo apt-get install -y libwebrtc-dev

# Container runtime (for V2.x)
sudo apt-get install -y docker.io containerd
```

---

## Version 1.x Deployment (Desktop Application)

### Development Build

```bash
# Clone repository
git clone https://github.com/usemanusai/free-deep-research.git
cd free-deep-research

# Install dependencies
npm install
cd src-tauri && cargo build

# Run development server
npm run tauri dev
```

### Production Build

```bash
# Build for production
npm run tauri build

# Outputs:
# - Windows: target/release/bundle/msi/
# - macOS: target/release/bundle/dmg/
# - Linux: target/release/bundle/deb/ or target/release/bundle/appimage/
```

### Installation

#### Windows
```powershell
# Install MSI package
msiexec /i "Free Deep Research_1.0.0_x64_en-US.msi" /quiet

# Or use Windows Package Manager
winget install FreeDeepResearch
```

#### macOS
```bash
# Install DMG
sudo hdiutil attach "Free Deep Research_1.0.0_x64.dmg"
sudo cp -R "/Volumes/Free Deep Research/Free Deep Research.app" /Applications/
sudo hdiutil detach "/Volumes/Free Deep Research"

# Or use Homebrew
brew install --cask free-deep-research
```

#### Linux
```bash
# Install DEB package
sudo dpkg -i free-deep-research_1.0.0_amd64.deb
sudo apt-get install -f

# Or install AppImage
chmod +x Free_Deep_Research-1.0.0.AppImage
./Free_Deep_Research-1.0.0.AppImage

# Or use Snap
sudo snap install free-deep-research
```

---

## Version 1.2.0 Deployment (Enhanced Features)

### Plugin System Setup

```bash
# Create plugin directory
mkdir -p ~/.config/free-deep-research/plugins

# Set plugin permissions
chmod 755 ~/.config/free-deep-research/plugins

# Configure plugin security
cat > ~/.config/free-deep-research/plugin-config.toml << EOF
[security]
sandbox_enabled = true
max_memory_mb = 512
max_execution_time_seconds = 300
allowed_permissions = ["network_access", "file_read"]
EOF
```

### Cloud Sync Configuration

```yaml
# ~/.config/free-deep-research/cloud-sync.yaml
providers:
  aws:
    region: us-east-1
    bucket: your-research-bucket
    access_key_id: ${AWS_ACCESS_KEY_ID}
    secret_access_key: ${AWS_SECRET_ACCESS_KEY}
  
  google_cloud:
    project_id: your-project-id
    bucket: your-research-bucket
    credentials_path: /path/to/service-account.json
  
  azure:
    account_name: your-storage-account
    container: research-data
    access_key: ${AZURE_STORAGE_KEY}

sync:
  auto_sync_enabled: true
  sync_interval_seconds: 300
  encryption_enabled: true
  compression_enabled: true
```

### Enterprise Setup

```bash
# Create enterprise configuration
sudo mkdir -p /etc/free-deep-research
sudo cat > /etc/free-deep-research/enterprise.yaml << EOF
enterprise:
  multi_tenant_enabled: true
  rbac_enabled: true
  audit_logging_enabled: true
  sso_enabled: true
  
authentication:
  providers:
    - type: saml
      entity_id: your-entity-id
      sso_url: https://your-idp.com/sso
      certificate_path: /etc/ssl/certs/idp-cert.pem
    
    - type: ldap
      server: ldap://your-ldap-server:389
      base_dn: dc=company,dc=com
      bind_dn: cn=admin,dc=company,dc=com
      bind_password: ${LDAP_PASSWORD}

compliance:
  frameworks: [SOC2, GDPR, HIPAA]
  data_retention_days: 2555
  audit_log_retention_days: 3650
EOF
```

---

## Version 2.0.0 Deployment (Distributed Architecture)

### Kubernetes Deployment

#### Prerequisites
```bash
# Install kubectl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl

# Install Helm
curl https://get.helm.sh/helm-v3.12.0-linux-amd64.tar.gz | tar xz
sudo mv linux-amd64/helm /usr/local/bin/

# Install Istio (for service mesh)
curl -L https://istio.io/downloadIstio | sh -
sudo mv istio-*/bin/istioctl /usr/local/bin/
```

#### Cluster Setup
```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: free-deep-research
  labels:
    istio-injection: enabled
---
# k8s/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: research-config
  namespace: free-deep-research
data:
  config.yaml: |
    cluster:
      name: research-cluster
      enable_service_mesh: true
      enable_auto_scaling: true
      replication_factor: 3
    
    services:
      research_engine:
        replicas: 3
        resources:
          requests:
            cpu: 500m
            memory: 1Gi
          limits:
            cpu: 2000m
            memory: 4Gi
      
      ai_orchestration:
        replicas: 5
        resources:
          requests:
            cpu: 1000m
            memory: 2Gi
          limits:
            cpu: 4000m
            memory: 8Gi
```

#### Service Deployment
```yaml
# k8s/research-engine-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: research-engine
  namespace: free-deep-research
spec:
  replicas: 3
  selector:
    matchLabels:
      app: research-engine
  template:
    metadata:
      labels:
        app: research-engine
        version: v2.0.0
    spec:
      containers:
      - name: research-engine
        image: freeresearch/research-engine:2.0.0
        ports:
        - containerPort: 8080
        env:
        - name: RUST_LOG
          value: "info"
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: database-secret
              key: url
        resources:
          requests:
            cpu: 500m
            memory: 1Gi
          limits:
            cpu: 2000m
            memory: 4Gi
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: research-engine-service
  namespace: free-deep-research
spec:
  selector:
    app: research-engine
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

#### Horizontal Pod Autoscaler
```yaml
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: research-engine-hpa
  namespace: free-deep-research
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: research-engine
  minReplicas: 3
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

#### Deploy to Kubernetes
```bash
# Apply configurations
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secrets.yaml
kubectl apply -f k8s/research-engine-deployment.yaml
kubectl apply -f k8s/ai-orchestration-deployment.yaml
kubectl apply -f k8s/realtime-collaboration-deployment.yaml
kubectl apply -f k8s/hpa.yaml

# Install Istio service mesh
istioctl install --set values.defaultRevision=default
kubectl apply -f k8s/istio-gateway.yaml
kubectl apply -f k8s/istio-virtualservice.yaml

# Verify deployment
kubectl get pods -n free-deep-research
kubectl get services -n free-deep-research
kubectl get hpa -n free-deep-research
```

### Helm Chart Deployment

```bash
# Add Helm repository
helm repo add freeresearch https://charts.freeresearch.ai
helm repo update

# Install with custom values
cat > values.yaml << EOF
global:
  imageRegistry: freeresearch
  imageTag: "2.0.0"
  
cluster:
  name: production-cluster
  replicationFactor: 3
  
services:
  researchEngine:
    enabled: true
    replicas: 5
    resources:
      requests:
        cpu: 1000m
        memory: 2Gi
      limits:
        cpu: 4000m
        memory: 8Gi
  
  aiOrchestration:
    enabled: true
    replicas: 3
    
  realtimeCollaboration:
    enabled: true
    replicas: 2
    
ingress:
  enabled: true
  className: nginx
  hosts:
    - host: research.company.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: research-tls
      hosts:
        - research.company.com

monitoring:
  prometheus:
    enabled: true
  grafana:
    enabled: true
  jaeger:
    enabled: true
EOF

# Install
helm install free-deep-research freeresearch/free-deep-research \
  --namespace free-deep-research \
  --create-namespace \
  --values values.yaml

# Upgrade
helm upgrade free-deep-research freeresearch/free-deep-research \
  --namespace free-deep-research \
  --values values.yaml
```

### Docker Compose (Development)

```yaml
# docker-compose.yml
version: '3.8'

services:
  research-engine:
    image: freeresearch/research-engine:2.0.0
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
      - DATABASE_URL=postgresql://user:pass@postgres:5432/research
    depends_on:
      - postgres
      - redis
    volumes:
      - ./config:/app/config
    networks:
      - research-network

  ai-orchestration:
    image: freeresearch/ai-orchestration:2.0.0
    ports:
      - "8081:8080"
    environment:
      - RUST_LOG=info
      - RESEARCH_ENGINE_URL=http://research-engine:8080
    depends_on:
      - research-engine
    networks:
      - research-network

  realtime-collaboration:
    image: freeresearch/realtime-collaboration:2.0.0
    ports:
      - "8082:8080"
    environment:
      - RUST_LOG=info
      - WEBSOCKET_PORT=8080
    networks:
      - research-network

  postgres:
    image: postgres:15
    environment:
      - POSTGRES_DB=research
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - research-network

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    networks:
      - research-network

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/ssl
    depends_on:
      - research-engine
      - ai-orchestration
      - realtime-collaboration
    networks:
      - research-network

volumes:
  postgres_data:
  redis_data:

networks:
  research-network:
    driver: bridge
```

### Monitoring Setup

```yaml
# monitoring/prometheus.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: prometheus-config
  namespace: free-deep-research
data:
  prometheus.yml: |
    global:
      scrape_interval: 15s
    
    scrape_configs:
    - job_name: 'research-engine'
      static_configs:
      - targets: ['research-engine-service:80']
      metrics_path: /metrics
    
    - job_name: 'ai-orchestration'
      static_configs:
      - targets: ['ai-orchestration-service:80']
      metrics_path: /metrics
    
    - job_name: 'kubernetes-pods'
      kubernetes_sd_configs:
      - role: pod
        namespaces:
          names:
          - free-deep-research
---
# monitoring/grafana-dashboard.json
{
  "dashboard": {
    "title": "Free Deep Research System",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])",
            "legendFormat": "{{service}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      }
    ]
  }
}
```

---

## Security Configuration

### TLS/SSL Setup
```bash
# Generate certificates
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout research.key -out research.crt \
  -subj "/CN=research.company.com"

# Create Kubernetes secret
kubectl create secret tls research-tls \
  --cert=research.crt \
  --key=research.key \
  --namespace=free-deep-research
```

### Network Policies
```yaml
# k8s/network-policy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: research-network-policy
  namespace: free-deep-research
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: istio-system
    - podSelector:
        matchLabels:
          app: research-engine
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          name: kube-system
  - to: []
    ports:
    - protocol: TCP
      port: 443
    - protocol: TCP
      port: 80
```

---

## Backup and Recovery

### Database Backup
```bash
#!/bin/bash
# backup.sh
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backups/research"

# Create backup directory
mkdir -p $BACKUP_DIR

# Backup PostgreSQL
kubectl exec -n free-deep-research postgres-0 -- \
  pg_dump -U user research | gzip > $BACKUP_DIR/research_$DATE.sql.gz

# Backup configuration
kubectl get configmaps -n free-deep-research -o yaml > $BACKUP_DIR/configmaps_$DATE.yaml
kubectl get secrets -n free-deep-research -o yaml > $BACKUP_DIR/secrets_$DATE.yaml

# Upload to cloud storage
aws s3 cp $BACKUP_DIR/ s3://research-backups/ --recursive
```

### Disaster Recovery
```bash
#!/bin/bash
# restore.sh
BACKUP_DATE=$1

# Restore database
gunzip -c /backups/research/research_$BACKUP_DATE.sql.gz | \
  kubectl exec -i -n free-deep-research postgres-0 -- psql -U user research

# Restore configuration
kubectl apply -f /backups/research/configmaps_$BACKUP_DATE.yaml
kubectl apply -f /backups/research/secrets_$BACKUP_DATE.yaml

# Restart services
kubectl rollout restart deployment -n free-deep-research
```

---

## Performance Tuning

### Resource Optimization
```yaml
# k8s/resource-quotas.yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: research-quota
  namespace: free-deep-research
spec:
  hard:
    requests.cpu: "50"
    requests.memory: 100Gi
    limits.cpu: "100"
    limits.memory: 200Gi
    persistentvolumeclaims: "10"
```

### JVM Tuning (if using Java components)
```bash
export JAVA_OPTS="-Xms2g -Xmx8g -XX:+UseG1GC -XX:MaxGCPauseMillis=200"
```

### Database Optimization
```sql
-- PostgreSQL optimization
ALTER SYSTEM SET shared_buffers = '4GB';
ALTER SYSTEM SET effective_cache_size = '12GB';
ALTER SYSTEM SET maintenance_work_mem = '1GB';
ALTER SYSTEM SET checkpoint_completion_target = 0.9;
ALTER SYSTEM SET wal_buffers = '16MB';
ALTER SYSTEM SET default_statistics_target = 100;
SELECT pg_reload_conf();
```

---

## Troubleshooting

### Common Issues

#### Pod Startup Issues
```bash
# Check pod status
kubectl get pods -n free-deep-research

# Check pod logs
kubectl logs -n free-deep-research deployment/research-engine

# Describe pod for events
kubectl describe pod -n free-deep-research <pod-name>
```

#### Service Discovery Issues
```bash
# Check service endpoints
kubectl get endpoints -n free-deep-research

# Test service connectivity
kubectl run test-pod --image=busybox --rm -it -- \
  wget -qO- http://research-engine-service/health
```

#### Performance Issues
```bash
# Check resource usage
kubectl top pods -n free-deep-research
kubectl top nodes

# Check HPA status
kubectl get hpa -n free-deep-research
```

### Log Analysis
```bash
# Centralized logging with ELK stack
kubectl apply -f monitoring/elasticsearch.yaml
kubectl apply -f monitoring/logstash.yaml
kubectl apply -f monitoring/kibana.yaml
kubectl apply -f monitoring/filebeat.yaml
```
