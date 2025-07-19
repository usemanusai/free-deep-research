#!/usr/bin/env node

/**
 * Free Deep Research System - Health Check Script
 * Comprehensive system health monitoring and reporting
 */

import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { readFileSync } from 'fs';

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Colors for console output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
};

class HealthChecker {
  constructor() {
    this.results = {
      overall: 'unknown',
      timestamp: new Date().toISOString(),
      checks: {},
      metrics: {},
      errors: [],
      warnings: [],
    };
  }

  log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
  }

  logSuccess(message) {
    this.log(`✓ ${message}`, 'green');
  }

  logWarning(message) {
    this.log(`⚠ ${message}`, 'yellow');
    this.results.warnings.push(message);
  }

  logError(message) {
    this.log(`✗ ${message}`, 'red');
    this.results.errors.push(message);
  }

  logInfo(message) {
    this.log(`ℹ ${message}`, 'blue');
  }

  async checkSystemHealth() {
    this.log('\n🔍 Free Deep Research System Health Check', 'cyan');
    this.log('=' .repeat(50), 'cyan');

    await this.checkNodeEnvironment();
    await this.checkDependencies();
    await this.checkFileSystem();
    await this.checkNetworkConnectivity();
    await this.checkDatabaseConnection();
    await this.checkRedisConnection();
    await this.checkAPIEndpoints();
    await this.checkSystemResources();
    await this.checkSecurityConfiguration();

    this.generateReport();
  }

  async checkNodeEnvironment() {
    this.log('\n📦 Node.js Environment', 'bright');
    
    try {
      const nodeVersion = process.version;
      const npmVersion = require('child_process')
        .execSync('npm --version', { encoding: 'utf8' })
        .trim();

      this.results.checks.nodeEnvironment = {
        nodeVersion,
        npmVersion,
        platform: process.platform,
        arch: process.arch,
        status: 'healthy',
      };

      this.logSuccess(`Node.js ${nodeVersion}`);
      this.logSuccess(`npm ${npmVersion}`);
      this.logSuccess(`Platform: ${process.platform} ${process.arch}`);
    } catch (error) {
      this.results.checks.nodeEnvironment = { status: 'error', error: error.message };
      this.logError(`Node.js environment check failed: ${error.message}`);
    }
  }

  async checkDependencies() {
    this.log('\n📚 Dependencies', 'bright');
    
    try {
      const packageJsonPath = join(__dirname, '../../package.json');
      const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
      
      const dependencies = Object.keys(packageJson.dependencies || {});
      const devDependencies = Object.keys(packageJson.devDependencies || {});
      
      this.results.checks.dependencies = {
        totalDependencies: dependencies.length,
        totalDevDependencies: devDependencies.length,
        status: 'healthy',
      };

      this.logSuccess(`${dependencies.length} production dependencies`);
      this.logSuccess(`${devDependencies.length} development dependencies`);

      // Check for security vulnerabilities
      try {
        require('child_process').execSync('npm audit --audit-level=high', { 
          stdio: 'pipe',
          encoding: 'utf8' 
        });
        this.logSuccess('No high-severity security vulnerabilities found');
      } catch (auditError) {
        this.logWarning('Security vulnerabilities detected - run npm audit for details');
      }
    } catch (error) {
      this.results.checks.dependencies = { status: 'error', error: error.message };
      this.logError(`Dependencies check failed: ${error.message}`);
    }
  }

  async checkFileSystem() {
    this.log('\n📁 File System', 'bright');
    
    try {
      const fs = require('fs');
      const path = require('path');
      
      const requiredDirectories = [
        'src',
        'docs',
        'bmad-agent',
        'docker',
        'scripts',
      ];

      const requiredFiles = [
        'package.json',
        'tsconfig.json',
        '.env.template',
        'README.md',
      ];

      let missingDirectories = 0;
      let missingFiles = 0;

      // Check directories
      for (const dir of requiredDirectories) {
        const dirPath = join(__dirname, '../../', dir);
        if (fs.existsSync(dirPath)) {
          this.logSuccess(`Directory exists: ${dir}`);
        } else {
          this.logError(`Missing directory: ${dir}`);
          missingDirectories++;
        }
      }

      // Check files
      for (const file of requiredFiles) {
        const filePath = join(__dirname, '../../', file);
        if (fs.existsSync(filePath)) {
          this.logSuccess(`File exists: ${file}`);
        } else {
          this.logError(`Missing file: ${file}`);
          missingFiles++;
        }
      }

      this.results.checks.fileSystem = {
        missingDirectories,
        missingFiles,
        status: missingDirectories === 0 && missingFiles === 0 ? 'healthy' : 'error',
      };
    } catch (error) {
      this.results.checks.fileSystem = { status: 'error', error: error.message };
      this.logError(`File system check failed: ${error.message}`);
    }
  }

  async checkNetworkConnectivity() {
    this.log('\n🌐 Network Connectivity', 'bright');
    
    const testUrls = [
      'https://api.openrouter.ai',
      'https://serpapi.com',
      'https://api.tavily.com',
      'https://api.firecrawl.dev',
      'https://api.jina.ai',
    ];

    let successfulConnections = 0;

    for (const url of testUrls) {
      try {
        const response = await fetch(url, { 
          method: 'HEAD',
          timeout: 5000,
        });
        
        if (response.ok || response.status === 401 || response.status === 403) {
          this.logSuccess(`${url} - reachable`);
          successfulConnections++;
        } else {
          this.logWarning(`${url} - unexpected status: ${response.status}`);
        }
      } catch (error) {
        this.logError(`${url} - unreachable: ${error.message}`);
      }
    }

    this.results.checks.networkConnectivity = {
      totalUrls: testUrls.length,
      successfulConnections,
      status: successfulConnections >= testUrls.length * 0.8 ? 'healthy' : 'warning',
    };
  }

  async checkDatabaseConnection() {
    this.log('\n🗄️ Database Connection', 'bright');
    
    try {
      // This would normally connect to the actual database
      // For now, we'll check if the database configuration exists
      const dbUrl = process.env.DATABASE_URL;
      
      if (dbUrl) {
        this.logSuccess('Database URL configured');
        
        // Try to parse the URL to validate format
        const url = new URL(dbUrl);
        this.logSuccess(`Database host: ${url.hostname}`);
        this.logSuccess(`Database port: ${url.port || 5432}`);
        this.logSuccess(`Database name: ${url.pathname.slice(1)}`);
        
        this.results.checks.database = {
          configured: true,
          host: url.hostname,
          port: url.port || 5432,
          database: url.pathname.slice(1),
          status: 'healthy',
        };
      } else {
        this.logWarning('Database URL not configured');
        this.results.checks.database = {
          configured: false,
          status: 'warning',
        };
      }
    } catch (error) {
      this.results.checks.database = { status: 'error', error: error.message };
      this.logError(`Database check failed: ${error.message}`);
    }
  }

  async checkRedisConnection() {
    this.log('\n🔴 Redis Connection', 'bright');
    
    try {
      const redisUrl = process.env.REDIS_URL;
      
      if (redisUrl) {
        this.logSuccess('Redis URL configured');
        
        const url = new URL(redisUrl);
        this.logSuccess(`Redis host: ${url.hostname}`);
        this.logSuccess(`Redis port: ${url.port || 6379}`);
        
        this.results.checks.redis = {
          configured: true,
          host: url.hostname,
          port: url.port || 6379,
          status: 'healthy',
        };
      } else {
        this.logWarning('Redis URL not configured');
        this.results.checks.redis = {
          configured: false,
          status: 'warning',
        };
      }
    } catch (error) {
      this.results.checks.redis = { status: 'error', error: error.message };
      this.logError(`Redis check failed: ${error.message}`);
    }
  }

  async checkAPIEndpoints() {
    this.log('\n🔌 API Endpoints', 'bright');
    
    const endpoints = [
      { name: 'Health Check', url: 'http://localhost:3000/health' },
      { name: 'API Status', url: 'http://localhost:8080/api/status' },
      { name: 'Metrics', url: 'http://localhost:9090/metrics' },
    ];

    let healthyEndpoints = 0;

    for (const endpoint of endpoints) {
      try {
        const response = await fetch(endpoint.url, { timeout: 3000 });
        
        if (response.ok) {
          this.logSuccess(`${endpoint.name} - healthy`);
          healthyEndpoints++;
        } else {
          this.logWarning(`${endpoint.name} - status: ${response.status}`);
        }
      } catch (error) {
        this.logWarning(`${endpoint.name} - not available (${error.message})`);
      }
    }

    this.results.checks.apiEndpoints = {
      totalEndpoints: endpoints.length,
      healthyEndpoints,
      status: healthyEndpoints > 0 ? 'healthy' : 'warning',
    };
  }

  async checkSystemResources() {
    this.log('\n💻 System Resources', 'bright');
    
    try {
      const os = require('os');
      
      const totalMemory = os.totalmem();
      const freeMemory = os.freemem();
      const usedMemory = totalMemory - freeMemory;
      const memoryUsagePercent = (usedMemory / totalMemory) * 100;
      
      const cpuCount = os.cpus().length;
      const loadAverage = os.loadavg();
      
      this.results.metrics = {
        memory: {
          total: Math.round(totalMemory / 1024 / 1024 / 1024 * 100) / 100,
          used: Math.round(usedMemory / 1024 / 1024 / 1024 * 100) / 100,
          free: Math.round(freeMemory / 1024 / 1024 / 1024 * 100) / 100,
          usagePercent: Math.round(memoryUsagePercent * 100) / 100,
        },
        cpu: {
          count: cpuCount,
          loadAverage: loadAverage.map(load => Math.round(load * 100) / 100),
        },
        uptime: Math.round(os.uptime()),
      };

      this.logSuccess(`Memory: ${this.results.metrics.memory.used}GB / ${this.results.metrics.memory.total}GB (${this.results.metrics.memory.usagePercent}%)`);
      this.logSuccess(`CPU cores: ${cpuCount}`);
      this.logSuccess(`Load average: ${loadAverage.map(l => l.toFixed(2)).join(', ')}`);
      this.logSuccess(`System uptime: ${Math.round(os.uptime() / 3600)}h`);

      // Check for resource warnings
      if (memoryUsagePercent > 90) {
        this.logWarning('High memory usage detected');
      }
      
      if (loadAverage[0] > cpuCount * 2) {
        this.logWarning('High CPU load detected');
      }

      this.results.checks.systemResources = {
        status: memoryUsagePercent < 90 && loadAverage[0] < cpuCount * 2 ? 'healthy' : 'warning',
      };
    } catch (error) {
      this.results.checks.systemResources = { status: 'error', error: error.message };
      this.logError(`System resources check failed: ${error.message}`);
    }
  }

  async checkSecurityConfiguration() {
    this.log('\n🔒 Security Configuration', 'bright');
    
    try {
      const securityChecks = {
        envFile: process.env.NODE_ENV !== 'production' || require('fs').existsSync('.env'),
        httpsEnabled: process.env.FORCE_HTTPS === 'true',
        secureHeaders: process.env.SECURITY_HEADERS === 'true',
        csrfProtection: process.env.CSRF_PROTECTION === 'true',
        rateLimiting: process.env.RATE_LIMITING === 'true',
      };

      let securityScore = 0;
      const totalChecks = Object.keys(securityChecks).length;

      for (const [check, passed] of Object.entries(securityChecks)) {
        if (passed) {
          this.logSuccess(`${check} - configured`);
          securityScore++;
        } else {
          this.logWarning(`${check} - not configured`);
        }
      }

      this.results.checks.security = {
        score: securityScore,
        totalChecks,
        percentage: Math.round((securityScore / totalChecks) * 100),
        status: securityScore >= totalChecks * 0.8 ? 'healthy' : 'warning',
      };

      this.logInfo(`Security score: ${securityScore}/${totalChecks} (${this.results.checks.security.percentage}%)`);
    } catch (error) {
      this.results.checks.security = { status: 'error', error: error.message };
      this.logError(`Security check failed: ${error.message}`);
    }
  }

  generateReport() {
    this.log('\n📊 Health Check Summary', 'cyan');
    this.log('=' .repeat(50), 'cyan');

    // Calculate overall health
    const checks = Object.values(this.results.checks);
    const healthyChecks = checks.filter(check => check.status === 'healthy').length;
    const warningChecks = checks.filter(check => check.status === 'warning').length;
    const errorChecks = checks.filter(check => check.status === 'error').length;

    if (errorChecks > 0) {
      this.results.overall = 'unhealthy';
      this.log('Overall Status: UNHEALTHY', 'red');
    } else if (warningChecks > 0) {
      this.results.overall = 'degraded';
      this.log('Overall Status: DEGRADED', 'yellow');
    } else {
      this.results.overall = 'healthy';
      this.log('Overall Status: HEALTHY', 'green');
    }

    this.log(`\nChecks: ${healthyChecks} healthy, ${warningChecks} warnings, ${errorChecks} errors`);

    if (this.results.warnings.length > 0) {
      this.log('\n⚠️ Warnings:', 'yellow');
      this.results.warnings.forEach(warning => this.log(`  • ${warning}`, 'yellow'));
    }

    if (this.results.errors.length > 0) {
      this.log('\n❌ Errors:', 'red');
      this.results.errors.forEach(error => this.log(`  • ${error}`, 'red'));
    }

    // Save report to file
    const reportPath = join(__dirname, '../../logs/health-check.json');
    require('fs').writeFileSync(reportPath, JSON.stringify(this.results, null, 2));
    this.log(`\n📄 Detailed report saved to: ${reportPath}`, 'blue');

    this.log('\n🔍 Health check completed!', 'cyan');
  }
}

// Run health check if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  const checker = new HealthChecker();
  checker.checkSystemHealth().catch(error => {
    console.error('Health check failed:', error);
    process.exit(1);
  });
}

export default HealthChecker;
