#!/bin/bash

# Free Deep Research System - Backup Service Health Check
# This script checks the health of the backup service

set -e

# Configuration
BACKUP_DIR="/backups"
LOG_FILE="/logs/backup.log"

# Function to check if backup directory is accessible
check_backup_directory() {
    if [[ -d "$BACKUP_DIR" ]] && [[ -w "$BACKUP_DIR" ]]; then
        return 0
    else
        echo "Backup directory not accessible"
        return 1
    fi
}

# Function to check if cron is running
check_cron_service() {
    if pgrep crond >/dev/null; then
        return 0
    else
        echo "Cron service not running"
        return 1
    fi
}

# Function to check disk space
check_disk_space() {
    local disk_usage=$(df "$BACKUP_DIR" | awk 'NR==2 {print $5}' | sed 's/%//')
    
    if [[ $disk_usage -lt 95 ]]; then
        return 0
    else
        echo "Disk usage too high: ${disk_usage}%"
        return 1
    fi
}

# Function to check recent backup
check_recent_backup() {
    local recent_backup=$(find "$BACKUP_DIR" -name "*.tar.gz" -type f -mtime -2 | head -1)
    
    if [[ -n "$recent_backup" ]] || [[ $(date +%H) -lt 4 ]]; then
        return 0
    else
        echo "No recent backups found"
        return 1
    fi
}

# Main health check
main() {
    local health_issues=()
    
    # Check backup directory
    if ! check_backup_directory; then
        health_issues+=("backup_directory")
    fi
    
    # Check cron service
    if ! check_cron_service; then
        health_issues+=("cron_service")
    fi
    
    # Check disk space
    if ! check_disk_space; then
        health_issues+=("disk_space")
    fi
    
    # Check recent backup
    if ! check_recent_backup; then
        health_issues+=("recent_backup")
    fi
    
    # Return health status
    if [[ ${#health_issues[@]} -eq 0 ]]; then
        echo "healthy"
        exit 0
    else
        echo "unhealthy: ${health_issues[*]}"
        exit 1
    fi
}

# Execute main function
main "$@"
