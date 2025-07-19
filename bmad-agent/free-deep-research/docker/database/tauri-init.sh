#!/bin/sh

# Tauri SQLite Database Initialization Script
# This script sets up the SQLite database for the Tauri desktop application

set -e

echo "🚀 Initializing Tauri SQLite Database..."

# Create data directory if it doesn't exist
mkdir -p /data

# Set permissions
chmod 755 /data

# Create empty database file if it doesn't exist
if [ ! -f "/data/research.db" ]; then
    echo "📁 Creating new SQLite database: /data/research.db"
    touch /data/research.db
    chmod 664 /data/research.db
    echo "✅ Database file created successfully"
else
    echo "📁 Database file already exists: /data/research.db"
fi

# Create test database for testing
if [ ! -f "/data/test_research.db" ]; then
    echo "📁 Creating test SQLite database: /data/test_research.db"
    touch /data/test_research.db
    chmod 664 /data/test_research.db
    echo "✅ Test database file created successfully"
else
    echo "📁 Test database file already exists: /data/test_research.db"
fi

# Create backup directory
mkdir -p /data/backups
chmod 755 /data/backups

# Create logs directory
mkdir -p /data/logs
chmod 755 /data/logs

echo "✅ Tauri SQLite Database initialization completed!"
echo "📊 Database location: /data/research.db"
echo "🧪 Test database location: /data/test_research.db"
echo "💾 Backup directory: /data/backups"
echo "📝 Logs directory: /data/logs"

# Keep container running
exec "$@"
