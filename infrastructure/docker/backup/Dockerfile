# Backup Service Dockerfile for Free Deep Research System
FROM alpine:3.18

# Install required packages
RUN apk add --no-cache \
    postgresql-client \
    redis \
    aws-cli \
    curl \
    bash \
    dcron \
    gzip \
    tar \
    && rm -rf /var/cache/apk/*

# Create backup user
RUN addgroup -g 1000 backup && \
    adduser -u 1000 -G backup -s /bin/bash -D backup

# Create directories
RUN mkdir -p /backups /scripts /logs && \
    chown -R backup:backup /backups /scripts /logs

# Copy backup scripts
COPY backup-scripts/ /scripts/
RUN chmod +x /scripts/*.sh && \
    chown -R backup:backup /scripts

# Switch to backup user
USER backup

# Set working directory
WORKDIR /backups

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD /scripts/health-check.sh

# Default command
CMD ["/scripts/backup-daemon.sh"]
