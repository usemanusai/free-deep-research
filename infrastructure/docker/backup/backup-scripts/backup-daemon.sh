#!/bin/bash

# Free Deep Research System - Backup Daemon
# This script runs as a daemon to perform scheduled backups

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[BACKUP]${NC} $(date '+%Y-%m-%d %H:%M:%S') $1"
}

print_success() {
    echo -e "${GREEN}[BACKUP]${NC} $(date '+%Y-%m-%d %H:%M:%S') $1"
}

print_warning() {
    echo -e "${YELLOW}[BACKUP]${NC} $(date '+%Y-%m-%d %H:%M:%S') $1"
}

print_error() {
    echo -e "${RED}[BACKUP]${NC} $(date '+%Y-%m-%d %H:%M:%S') $1"
}

# Configuration
BACKUP_DIR="/backups"
LOG_FILE="/logs/backup.log"
SCRIPTS_DIR="/scripts"

# Default values
BACKUP_SCHEDULE="${BACKUP_SCHEDULE:-0 2 * * *}"  # Daily at 2 AM
BACKUP_RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-30}"

# Function to log messages
log_message() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" >> "$LOG_FILE"
    echo "$1"
}

# Function to setup cron job
setup_cron() {
    print_status "Setting up backup schedule: $BACKUP_SCHEDULE"
    
    # Create crontab entry
    echo "$BACKUP_SCHEDULE /scripts/run-backup.sh >> $LOG_FILE 2>&1" > /tmp/crontab
    
    # Install crontab
    crontab /tmp/crontab
    
    print_success "Backup schedule configured"
}

# Function to cleanup old backups
cleanup_old_backups() {
    print_status "Cleaning up backups older than $BACKUP_RETENTION_DAYS days"
    
    find "$BACKUP_DIR" -name "*.tar.gz" -type f -mtime +$BACKUP_RETENTION_DAYS -delete
    find "$BACKUP_DIR" -name "*.sql.gz" -type f -mtime +$BACKUP_RETENTION_DAYS -delete
    
    print_success "Old backup cleanup completed"
}

# Function to check backup health
check_backup_health() {
    local health_status="healthy"
    
    # Check if backup directory is writable
    if [[ ! -w "$BACKUP_DIR" ]]; then
        print_error "Backup directory is not writable"
        health_status="unhealthy"
    fi
    
    # Check if recent backups exist
    local recent_backup=$(find "$BACKUP_DIR" -name "*.tar.gz" -type f -mtime -1 | head -1)
    if [[ -z "$recent_backup" ]] && [[ $(date +%H) -gt 3 ]]; then
        print_warning "No recent backups found"
    fi
    
    # Check disk space
    local disk_usage=$(df "$BACKUP_DIR" | awk 'NR==2 {print $5}' | sed 's/%//')
    if [[ $disk_usage -gt 90 ]]; then
        print_error "Backup disk usage is at ${disk_usage}%"
        health_status="unhealthy"
    fi
    
    echo "$health_status"
}

# Function to send notifications
send_notification() {
    local message="$1"
    local level="${2:-info}"
    
    # Log the message
    log_message "$message"
    
    # Send to external notification systems if configured
    if [[ -n "$SLACK_WEBHOOK_URL" ]]; then
        curl -X POST -H 'Content-type: application/json' \
            --data "{\"text\":\"[FDR Backup] $message\"}" \
            "$SLACK_WEBHOOK_URL" >/dev/null 2>&1 || true
    fi
    
    if [[ -n "$DISCORD_WEBHOOK_URL" ]]; then
        curl -X POST -H 'Content-type: application/json' \
            --data "{\"content\":\"[FDR Backup] $message\"}" \
            "$DISCORD_WEBHOOK_URL" >/dev/null 2>&1 || true
    fi
}

# Function to handle signals
handle_signal() {
    print_status "Received signal, shutting down backup daemon..."
    
    # Kill cron daemon
    pkill crond || true
    
    # Send shutdown notification
    send_notification "Backup daemon shutting down"
    
    exit 0
}

# Function to start daemon
start_daemon() {
    print_status "Starting Free Deep Research Backup Daemon v3.0.0"
    
    # Create necessary directories
    mkdir -p "$BACKUP_DIR" /logs
    
    # Setup signal handlers
    trap handle_signal SIGTERM SIGINT
    
    # Setup cron job
    setup_cron
    
    # Start cron daemon
    print_status "Starting cron daemon..."
    crond -f -l 2 &
    
    # Send startup notification
    send_notification "Backup daemon started successfully"
    
    # Main daemon loop
    while true; do
        # Perform health check
        health_status=$(check_backup_health)
        
        if [[ "$health_status" != "healthy" ]]; then
            print_warning "Backup system health check failed: $health_status"
        fi
        
        # Cleanup old backups daily
        if [[ $(date +%H:%M) == "01:00" ]]; then
            cleanup_old_backups
        fi
        
        # Sleep for 1 hour
        sleep 3600
    done
}

# Function to run immediate backup
run_immediate_backup() {
    print_status "Running immediate backup..."
    
    if "$SCRIPTS_DIR/run-backup.sh"; then
        print_success "Immediate backup completed successfully"
        send_notification "Immediate backup completed successfully"
    else
        print_error "Immediate backup failed"
        send_notification "Immediate backup failed" "error"
        exit 1
    fi
}

# Main execution
main() {
    case "${1:-daemon}" in
        daemon)
            start_daemon
            ;;
        backup)
            run_immediate_backup
            ;;
        health)
            health_status=$(check_backup_health)
            echo "$health_status"
            [[ "$health_status" == "healthy" ]] && exit 0 || exit 1
            ;;
        cleanup)
            cleanup_old_backups
            ;;
        *)
            echo "Usage: $0 {daemon|backup|health|cleanup}"
            exit 1
            ;;
    esac
}

# Execute main function
main "$@"
