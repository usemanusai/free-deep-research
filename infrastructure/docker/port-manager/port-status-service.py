#!/usr/bin/env python3

"""
Free Deep Research System - Port Status Service
Provides HTTP endpoints for port status and service discovery
"""

import json
import os
import subprocess
import sys
from datetime import datetime
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse, parse_qs
import socket
import threading
import time

class PortStatusHandler(BaseHTTPRequestHandler):
    """HTTP request handler for port status service"""
    
    def __init__(self, *args, **kwargs):
        self.project_root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
        self.port_registry_file = os.path.join(self.project_root, '.env.ports')
        super().__init__(*args, **kwargs)
    
    def do_GET(self):
        """Handle GET requests"""
        parsed_path = urlparse(self.path)
        path = parsed_path.path
        query_params = parse_qs(parsed_path.query)
        
        if path == '/health':
            self.handle_health_check()
        elif path == '/ports':
            self.handle_port_status()
        elif path == '/services':
            self.handle_service_discovery()
        elif path == '/containers':
            self.handle_container_status()
        elif path == '/':
            self.handle_index()
        else:
            self.send_error(404, "Not Found")
    
    def handle_health_check(self):
        """Handle health check endpoint"""
        response = {
            "status": "healthy",
            "service": "port-status-service",
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "version": "1.0.0"
        }
        
        self.send_json_response(response)
    
    def handle_port_status(self):
        """Handle port status endpoint"""
        try:
            ports = self.get_port_assignments()
            port_status = {}
            
            for service, port in ports.items():
                if port and port.isdigit():
                    port_int = int(port)
                    is_available = self.is_port_available(port_int)
                    port_status[service] = {
                        "port": port_int,
                        "available": is_available,
                        "status": "free" if is_available else "in_use",
                        "url": f"http://localhost:{port_int}" if not is_available else None
                    }
            
            response = {
                "status": "success",
                "timestamp": datetime.utcnow().isoformat() + "Z",
                "ports": port_status,
                "total_ports": len(port_status),
                "ports_in_use": sum(1 for p in port_status.values() if not p["available"])
            }
            
            self.send_json_response(response)
            
        except Exception as e:
            self.send_error_response(f"Failed to get port status: {str(e)}")
    
    def handle_service_discovery(self):
        """Handle service discovery endpoint"""
        try:
            ports = self.get_port_assignments()
            services = {}
            
            # Map service names to user-friendly names and paths
            service_mapping = {
                "FRONTEND_PORT": {"name": "Frontend", "path": "/", "icon": "🌐"},
                "BACKEND_PORT": {"name": "Backend API", "path": "/health", "icon": "🔧"},
                "GRAFANA_PORT": {"name": "Grafana", "path": "/", "icon": "📈"},
                "PROMETHEUS_PORT": {"name": "Prometheus", "path": "/", "icon": "📊"},
                "ADMINER_PORT": {"name": "Database Admin", "path": "/", "icon": "🗄️"},
                "REDIS_COMMANDER_PORT": {"name": "Redis Commander", "path": "/", "icon": "🔴"},
                "MAILHOG_WEB_PORT": {"name": "Mailhog", "path": "/", "icon": "📧"},
                "DEV_DASHBOARD_PORT": {"name": "Dev Dashboard", "path": "/", "icon": "🛠️"}
            }
            
            for service_key, port in ports.items():
                if port and port.isdigit() and service_key in service_mapping:
                    port_int = int(port)
                    is_available = not self.is_port_available(port_int)  # Service should be using the port
                    
                    if is_available:  # Only include services that are actually running
                        mapping = service_mapping[service_key]
                        services[service_key.lower().replace('_port', '')] = {
                            "name": mapping["name"],
                            "icon": mapping["icon"],
                            "port": port_int,
                            "url": f"http://localhost:{port_int}{mapping['path']}",
                            "status": "running" if is_available else "stopped",
                            "health_check": f"http://localhost:{port_int}/health"
                        }
            
            response = {
                "status": "success",
                "timestamp": datetime.utcnow().isoformat() + "Z",
                "services": services,
                "total_services": len(services),
                "running_services": sum(1 for s in services.values() if s["status"] == "running")
            }
            
            self.send_json_response(response)
            
        except Exception as e:
            self.send_error_response(f"Failed to get service discovery: {str(e)}")
    
    def handle_container_status(self):
        """Handle container status endpoint"""
        try:
            containers = self.get_container_status()
            
            response = {
                "status": "success",
                "timestamp": datetime.utcnow().isoformat() + "Z",
                "containers": containers,
                "total_containers": len(containers),
                "running_containers": sum(1 for c in containers if c.get("status") == "running")
            }
            
            self.send_json_response(response)
            
        except Exception as e:
            self.send_error_response(f"Failed to get container status: {str(e)}")
    
    def handle_index(self):
        """Handle index page with HTML dashboard"""
        html_content = self.generate_dashboard_html()
        
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(html_content.encode('utf-8'))
    
    def get_port_assignments(self):
        """Read port assignments from registry file"""
        ports = {}
        
        if not os.path.exists(self.port_registry_file):
            return ports
        
        try:
            with open(self.port_registry_file, 'r') as f:
                for line in f:
                    line = line.strip()
                    if line and not line.startswith('#') and '=' in line:
                        key, value = line.split('=', 1)
                        if key.endswith('_PORT'):
                            ports[key] = value
        except Exception as e:
            print(f"Error reading port registry: {e}")
        
        return ports
    
    def is_port_available(self, port):
        """Check if a port is available (not in use)"""
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
                sock.settimeout(1)
                result = sock.connect_ex(('localhost', port))
                return result != 0  # Port is available if connection fails
        except Exception:
            return True  # Assume available if check fails
    
    def get_container_status(self):
        """Get Docker container status"""
        containers = []
        
        try:
            # Get container information using docker ps
            result = subprocess.run([
                'docker', 'ps', '-a', 
                '--filter', 'name=free-deep-research',
                '--format', 'json'
            ], capture_output=True, text=True, timeout=10)
            
            if result.returncode == 0:
                for line in result.stdout.strip().split('\n'):
                    if line:
                        try:
                            container_info = json.loads(line)
                            containers.append({
                                "name": container_info.get("Names", ""),
                                "status": container_info.get("State", ""),
                                "image": container_info.get("Image", ""),
                                "ports": container_info.get("Ports", ""),
                                "created": container_info.get("CreatedAt", "")
                            })
                        except json.JSONDecodeError:
                            continue
        except Exception as e:
            print(f"Error getting container status: {e}")
        
        return containers
    
    def generate_dashboard_html(self):
        """Generate HTML dashboard"""
        return """
<!DOCTYPE html>
<html>
<head>
    <title>Free Deep Research - Port Status Dashboard</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background-color: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; }
        .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; border-radius: 10px; margin-bottom: 20px; }
        .card { background: white; padding: 20px; border-radius: 10px; margin-bottom: 20px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .service-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .service-card { background: white; padding: 15px; border-radius: 8px; border-left: 4px solid #667eea; }
        .service-card.running { border-left-color: #28a745; }
        .service-card.stopped { border-left-color: #dc3545; }
        .status-badge { padding: 4px 8px; border-radius: 4px; font-size: 12px; font-weight: bold; }
        .status-running { background: #d4edda; color: #155724; }
        .status-stopped { background: #f8d7da; color: #721c24; }
        .refresh-btn { background: #667eea; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; }
        .refresh-btn:hover { background: #5a6fd8; }
        a { color: #667eea; text-decoration: none; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🐳 Free Deep Research System</h1>
            <p>Port Status Dashboard - Version 3.0.0 "Global Intelligence Network"</p>
            <button class="refresh-btn" onclick="location.reload()">🔄 Refresh</button>
        </div>
        
        <div class="card">
            <h2>📊 System Status</h2>
            <div id="system-status">Loading...</div>
        </div>
        
        <div class="card">
            <h2>🌐 Services</h2>
            <div id="services-grid" class="service-grid">Loading...</div>
        </div>
        
        <div class="card">
            <h2>🔌 Port Assignments</h2>
            <div id="port-status">Loading...</div>
        </div>
        
        <div class="card">
            <h2>📦 Containers</h2>
            <div id="container-status">Loading...</div>
        </div>
    </div>
    
    <script>
        async function loadData() {
            try {
                // Load services
                const servicesResponse = await fetch('/services');
                const servicesData = await servicesResponse.json();
                displayServices(servicesData);
                
                // Load port status
                const portsResponse = await fetch('/ports');
                const portsData = await portsResponse.json();
                displayPorts(portsData);
                
                // Load container status
                const containersResponse = await fetch('/containers');
                const containersData = await containersResponse.json();
                displayContainers(containersData);
                
                // Update system status
                displaySystemStatus(servicesData, portsData, containersData);
                
            } catch (error) {
                console.error('Error loading data:', error);
            }
        }
        
        function displayServices(data) {
            const grid = document.getElementById('services-grid');
            if (!data.services || Object.keys(data.services).length === 0) {
                grid.innerHTML = '<p>No services found. Make sure containers are running.</p>';
                return;
            }
            
            grid.innerHTML = Object.entries(data.services).map(([key, service]) => `
                <div class="service-card ${service.status}">
                    <h3>${service.icon} ${service.name}</h3>
                    <p><strong>Port:</strong> ${service.port}</p>
                    <p><strong>Status:</strong> <span class="status-badge status-${service.status}">${service.status}</span></p>
                    <p><strong>URL:</strong> <a href="${service.url}" target="_blank">${service.url}</a></p>
                </div>
            `).join('');
        }
        
        function displayPorts(data) {
            const container = document.getElementById('port-status');
            if (!data.ports || Object.keys(data.ports).length === 0) {
                container.innerHTML = '<p>No port assignments found.</p>';
                return;
            }
            
            container.innerHTML = `
                <table style="width: 100%; border-collapse: collapse;">
                    <thead>
                        <tr style="background: #f8f9fa;">
                            <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Service</th>
                            <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Port</th>
                            <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Status</th>
                            <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">URL</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${Object.entries(data.ports).map(([service, info]) => `
                            <tr>
                                <td style="padding: 10px; border: 1px solid #dee2e6;">${service.replace('_PORT', '').toLowerCase()}</td>
                                <td style="padding: 10px; border: 1px solid #dee2e6;">${info.port}</td>
                                <td style="padding: 10px; border: 1px solid #dee2e6;">
                                    <span class="status-badge ${info.available ? 'status-stopped' : 'status-running'}">
                                        ${info.available ? 'Available' : 'In Use'}
                                    </span>
                                </td>
                                <td style="padding: 10px; border: 1px solid #dee2e6;">
                                    ${info.url ? `<a href="${info.url}" target="_blank">${info.url}</a>` : 'N/A'}
                                </td>
                            </tr>
                        `).join('')}
                    </tbody>
                </table>
            `;
        }
        
        function displayContainers(data) {
            const container = document.getElementById('container-status');
            if (!data.containers || data.containers.length === 0) {
                container.innerHTML = '<p>No containers found.</p>';
                return;
            }
            
            container.innerHTML = `
                <table style="width: 100%; border-collapse: collapse;">
                    <thead>
                        <tr style="background: #f8f9fa;">
                            <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Name</th>
                            <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Status</th>
                            <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Image</th>
                            <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Ports</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${data.containers.map(container => `
                            <tr>
                                <td style="padding: 10px; border: 1px solid #dee2e6;">${container.name}</td>
                                <td style="padding: 10px; border: 1px solid #dee2e6;">
                                    <span class="status-badge ${container.status === 'running' ? 'status-running' : 'status-stopped'}">
                                        ${container.status}
                                    </span>
                                </td>
                                <td style="padding: 10px; border: 1px solid #dee2e6;">${container.image}</td>
                                <td style="padding: 10px; border: 1px solid #dee2e6;">${container.ports || 'None'}</td>
                            </tr>
                        `).join('')}
                    </tbody>
                </table>
            `;
        }
        
        function displaySystemStatus(services, ports, containers) {
            const container = document.getElementById('system-status');
            const runningServices = services.running_services || 0;
            const totalServices = services.total_services || 0;
            const portsInUse = ports.ports_in_use || 0;
            const totalPorts = ports.total_ports || 0;
            const runningContainers = containers.running_containers || 0;
            const totalContainers = containers.total_containers || 0;
            
            container.innerHTML = `
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px;">
                    <div style="text-align: center; padding: 20px; background: #e3f2fd; border-radius: 8px;">
                        <h3 style="margin: 0; color: #1976d2;">Services</h3>
                        <p style="font-size: 24px; margin: 10px 0; font-weight: bold;">${runningServices}/${totalServices}</p>
                        <p style="margin: 0; color: #666;">Running</p>
                    </div>
                    <div style="text-align: center; padding: 20px; background: #f3e5f5; border-radius: 8px;">
                        <h3 style="margin: 0; color: #7b1fa2;">Ports</h3>
                        <p style="font-size: 24px; margin: 10px 0; font-weight: bold;">${portsInUse}/${totalPorts}</p>
                        <p style="margin: 0; color: #666;">In Use</p>
                    </div>
                    <div style="text-align: center; padding: 20px; background: #e8f5e8; border-radius: 8px;">
                        <h3 style="margin: 0; color: #388e3c;">Containers</h3>
                        <p style="font-size: 24px; margin: 10px 0; font-weight: bold;">${runningContainers}/${totalContainers}</p>
                        <p style="margin: 0; color: #666;">Running</p>
                    </div>
                </div>
            `;
        }
        
        // Load data on page load
        loadData();
        
        // Auto-refresh every 30 seconds
        setInterval(loadData, 30000);
    </script>
</body>
</html>
        """
    
    def send_json_response(self, data, status_code=200):
        """Send JSON response"""
        self.send_response(status_code)
        self.send_header('Content-type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(data, indent=2).encode('utf-8'))
    
    def send_error_response(self, message, status_code=500):
        """Send error response"""
        response = {
            "status": "error",
            "message": message,
            "timestamp": datetime.utcnow().isoformat() + "Z"
        }
        self.send_json_response(response, status_code)
    
    def log_message(self, format, *args):
        """Override to customize logging"""
        timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        print(f"[{timestamp}] {format % args}")

def main():
    """Main function to start the port status service"""
    port = int(os.environ.get('PORT_STATUS_SERVICE_PORT', 8084))
    
    try:
        server = HTTPServer(('localhost', port), PortStatusHandler)
        print(f"🚀 Port Status Service started on http://localhost:{port}")
        print(f"📊 Dashboard: http://localhost:{port}/")
        print(f"🔌 Port Status API: http://localhost:{port}/ports")
        print(f"🌐 Service Discovery API: http://localhost:{port}/services")
        print(f"📦 Container Status API: http://localhost:{port}/containers")
        print(f"❤️  Health Check: http://localhost:{port}/health")
        print("Press Ctrl+C to stop the service")
        
        server.serve_forever()
        
    except KeyboardInterrupt:
        print("\n🛑 Port Status Service stopped")
    except Exception as e:
        print(f"❌ Error starting Port Status Service: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main()
