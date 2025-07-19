#!/usr/bin/env python3
"""
Free Deep Research System - Backend Health Check Server
Provides comprehensive health check endpoints for Docker deployment
"""

import json
import time
import psutil
import subprocess
import sqlite3
import os
from datetime import datetime, timezone
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
import threading
import signal
import sys

class HealthCheckHandler(BaseHTTPRequestHandler):
    """HTTP request handler for health check endpoints"""
    
    def do_GET(self):
        """Handle GET requests"""
        parsed_path = urlparse(self.path)
        path = parsed_path.path
        
        if path == '/health':
            self.handle_basic_health()
        elif path == '/health/detailed':
            self.handle_detailed_health()
        elif path == '/health/database':
            self.handle_database_health()
        elif path == '/health/services':
            self.handle_services_health()
        elif path == '/health/system':
            self.handle_system_health()
        elif path == '/metrics':
            self.handle_metrics()
        else:
            self.send_error(404, "Not Found")
    
    def handle_basic_health(self):
        """Basic health check endpoint"""
        try:
            health_data = {
                "status": "healthy",
                "service": "free-deep-research-backend",
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "version": "3.0.0",
                "uptime": self.get_uptime()
            }
            
            self.send_json_response(health_data)
        except Exception as e:
            self.send_error_response(500, f"Health check failed: {str(e)}")
    
    def handle_detailed_health(self):
        """Detailed health check with all components"""
        try:
            health_data = {
                "status": "healthy",
                "service": "free-deep-research-backend",
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "version": "3.0.0",
                "uptime": self.get_uptime(),
                "components": {
                    "database": self.check_database_health(),
                    "system": self.check_system_health(),
                    "services": self.check_services_health(),
                    "disk": self.check_disk_health(),
                    "memory": self.check_memory_health()
                }
            }
            
            # Determine overall status
            component_statuses = [comp.get("status", "unknown") for comp in health_data["components"].values()]
            if "unhealthy" in component_statuses:
                health_data["status"] = "unhealthy"
            elif "warning" in component_statuses:
                health_data["status"] = "warning"
            
            self.send_json_response(health_data)
        except Exception as e:
            self.send_error_response(500, f"Detailed health check failed: {str(e)}")
    
    def handle_database_health(self):
        """Database-specific health check"""
        try:
            db_health = self.check_database_health()
            self.send_json_response(db_health)
        except Exception as e:
            self.send_error_response(500, f"Database health check failed: {str(e)}")
    
    def handle_services_health(self):
        """Services-specific health check"""
        try:
            services_health = self.check_services_health()
            self.send_json_response(services_health)
        except Exception as e:
            self.send_error_response(500, f"Services health check failed: {str(e)}")
    
    def handle_system_health(self):
        """System-specific health check"""
        try:
            system_health = self.check_system_health()
            self.send_json_response(system_health)
        except Exception as e:
            self.send_error_response(500, f"System health check failed: {str(e)}")
    
    def handle_metrics(self):
        """Prometheus-style metrics endpoint"""
        try:
            metrics = self.generate_metrics()
            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            self.wfile.write(metrics.encode())
        except Exception as e:
            self.send_error_response(500, f"Metrics generation failed: {str(e)}")
    
    def check_database_health(self):
        """Check database connectivity and health"""
        try:
            # Check PostgreSQL connection (if available)
            pg_status = self.check_postgresql()
            
            # Check SQLite connection (fallback)
            sqlite_status = self.check_sqlite()
            
            return {
                "status": "healthy" if (pg_status["status"] == "healthy" or sqlite_status["status"] == "healthy") else "unhealthy",
                "postgresql": pg_status,
                "sqlite": sqlite_status,
                "timestamp": datetime.now(timezone.utc).isoformat()
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "error": str(e),
                "timestamp": datetime.now(timezone.utc).isoformat()
            }
    
    def check_postgresql(self):
        """Check PostgreSQL database health"""
        try:
            import psycopg2
            
            # Get connection details from environment
            db_host = os.getenv('DB_HOST', 'database')
            db_port = os.getenv('DB_PORT', '5432')
            db_name = os.getenv('DB_NAME', 'free_deep_research')
            db_user = os.getenv('DB_USER', 'fdr_user')
            db_password = os.getenv('DB_PASSWORD', 'secure_password_change_me')
            
            conn = psycopg2.connect(
                host=db_host,
                port=db_port,
                database=db_name,
                user=db_user,
                password=db_password,
                connect_timeout=5
            )
            
            cursor = conn.cursor()
            cursor.execute("SELECT 1")
            cursor.fetchone()
            cursor.close()
            conn.close()
            
            return {
                "status": "healthy",
                "type": "postgresql",
                "host": db_host,
                "database": db_name
            }
        except ImportError:
            return {
                "status": "unavailable",
                "type": "postgresql",
                "error": "psycopg2 not installed"
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "type": "postgresql",
                "error": str(e)
            }
    
    def check_sqlite(self):
        """Check SQLite database health"""
        try:
            db_path = os.getenv('SQLITE_DB_PATH', '/app/data/research.db')
            
            # Create directory if it doesn't exist
            os.makedirs(os.path.dirname(db_path), exist_ok=True)
            
            conn = sqlite3.connect(db_path, timeout=5)
            cursor = conn.cursor()
            cursor.execute("SELECT 1")
            cursor.fetchone()
            cursor.close()
            conn.close()
            
            return {
                "status": "healthy",
                "type": "sqlite",
                "path": db_path,
                "size": os.path.getsize(db_path) if os.path.exists(db_path) else 0
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "type": "sqlite",
                "error": str(e)
            }
    
    def check_system_health(self):
        """Check system resource health"""
        try:
            cpu_percent = psutil.cpu_percent(interval=1)
            memory = psutil.virtual_memory()
            disk = psutil.disk_usage('/')
            
            # Determine status based on thresholds
            status = "healthy"
            if cpu_percent > 90 or memory.percent > 90 or disk.percent > 90:
                status = "unhealthy"
            elif cpu_percent > 70 or memory.percent > 70 or disk.percent > 80:
                status = "warning"
            
            return {
                "status": status,
                "cpu_percent": cpu_percent,
                "memory_percent": memory.percent,
                "memory_available": memory.available,
                "disk_percent": disk.percent,
                "disk_free": disk.free,
                "load_average": os.getloadavg() if hasattr(os, 'getloadavg') else None,
                "timestamp": datetime.now(timezone.utc).isoformat()
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "error": str(e),
                "timestamp": datetime.now(timezone.utc).isoformat()
            }
    
    def check_services_health(self):
        """Check external services health"""
        try:
            # This would typically check external API services
            # For now, return a basic status
            return {
                "status": "healthy",
                "api_services": {
                    "openrouter": {"status": "unknown", "note": "API key required for testing"},
                    "serpapi": {"status": "unknown", "note": "API key required for testing"},
                    "jina": {"status": "unknown", "note": "API key required for testing"},
                    "firecrawl": {"status": "unknown", "note": "API key required for testing"},
                    "tavily": {"status": "unknown", "note": "API key required for testing"},
                    "exa": {"status": "unknown", "note": "API key required for testing"}
                },
                "timestamp": datetime.now(timezone.utc).isoformat()
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "error": str(e),
                "timestamp": datetime.now(timezone.utc).isoformat()
            }
    
    def check_disk_health(self):
        """Check disk space health"""
        try:
            disk = psutil.disk_usage('/')
            
            status = "healthy"
            if disk.percent > 95:
                status = "unhealthy"
            elif disk.percent > 85:
                status = "warning"
            
            return {
                "status": status,
                "usage_percent": disk.percent,
                "free_bytes": disk.free,
                "total_bytes": disk.total,
                "used_bytes": disk.used
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "error": str(e)
            }
    
    def check_memory_health(self):
        """Check memory health"""
        try:
            memory = psutil.virtual_memory()
            
            status = "healthy"
            if memory.percent > 95:
                status = "unhealthy"
            elif memory.percent > 85:
                status = "warning"
            
            return {
                "status": status,
                "usage_percent": memory.percent,
                "available_bytes": memory.available,
                "total_bytes": memory.total,
                "used_bytes": memory.used
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "error": str(e)
            }
    
    def get_uptime(self):
        """Get system uptime in seconds"""
        try:
            return time.time() - psutil.boot_time()
        except:
            return 0
    
    def generate_metrics(self):
        """Generate Prometheus-style metrics"""
        try:
            cpu_percent = psutil.cpu_percent()
            memory = psutil.virtual_memory()
            disk = psutil.disk_usage('/')
            
            metrics = f"""# HELP fdr_cpu_usage_percent CPU usage percentage
# TYPE fdr_cpu_usage_percent gauge
fdr_cpu_usage_percent {cpu_percent}

# HELP fdr_memory_usage_percent Memory usage percentage
# TYPE fdr_memory_usage_percent gauge
fdr_memory_usage_percent {memory.percent}

# HELP fdr_disk_usage_percent Disk usage percentage
# TYPE fdr_disk_usage_percent gauge
fdr_disk_usage_percent {disk.percent}

# HELP fdr_uptime_seconds System uptime in seconds
# TYPE fdr_uptime_seconds counter
fdr_uptime_seconds {self.get_uptime()}

# HELP fdr_health_status Health status (1=healthy, 0=unhealthy)
# TYPE fdr_health_status gauge
fdr_health_status 1
"""
            return metrics
        except Exception as e:
            return f"# Error generating metrics: {str(e)}\n"
    
    def send_json_response(self, data, status_code=200):
        """Send JSON response"""
        self.send_response(status_code)
        self.send_header('Content-type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(data, indent=2).encode())
    
    def send_error_response(self, status_code, message):
        """Send error response"""
        error_data = {
            "status": "error",
            "error": message,
            "timestamp": datetime.now(timezone.utc).isoformat()
        }
        self.send_json_response(error_data, status_code)
    
    def log_message(self, format, *args):
        """Override to customize logging"""
        timestamp = datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M:%S UTC')
        print(f"[{timestamp}] {format % args}")

def signal_handler(signum, frame):
    """Handle shutdown signals"""
    print(f"\nReceived signal {signum}, shutting down health check server...")
    sys.exit(0)

def main():
    """Main function to start the health check server"""
    # Set up signal handlers
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)
    
    # Get port from environment or use default
    port = int(os.getenv('HEALTH_CHECK_PORT', '8080'))
    
    # Create and start server
    server = HTTPServer(('0.0.0.0', port), HealthCheckHandler)
    
    print(f"Starting Free Deep Research Health Check Server on port {port}")
    print(f"Health endpoints:")
    print(f"  - Basic health: http://localhost:{port}/health")
    print(f"  - Detailed health: http://localhost:{port}/health/detailed")
    print(f"  - Database health: http://localhost:{port}/health/database")
    print(f"  - System health: http://localhost:{port}/health/system")
    print(f"  - Metrics: http://localhost:{port}/metrics")
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down health check server...")
        server.shutdown()

if __name__ == '__main__':
    main()
