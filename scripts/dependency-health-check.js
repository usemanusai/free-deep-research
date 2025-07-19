#!/usr/bin/env node

/**
 * Comprehensive Automated Dependency Management and Health Check System
 * for the free-deep-research project
 * 
 * Features:
 * - Multi-layer dependency analysis
 * - Intelligent automated resolution
 * - Robust error handling
 * - Startup integration hooks
 * - Security vulnerability scanning
 * - Cross-platform support
 */

import fs from 'fs/promises';
import path from 'path';
import { execSync, spawn } from 'child_process';
import https from 'https';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Configuration
const CONFIG = {
  mode: process.env.DEPENDENCY_MODE || 'conservative', // conservative, aggressive, security-only
  registries: {
    npm: 'https://registry.npmjs.org',
    cargo: 'https://crates.io/api/v1'
  },
  backupDir: path.join(__dirname, '../.dependency-backups'),
  reportDir: path.join(__dirname, '../reports'),
  maxRetries: 3,
  retryDelay: 1000,
  timeout: 30000
};

// Color codes for console output
const colors = {
  reset: '\x1b[0m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m'
};

class Logger {
  static info(message) {
    console.log(`${colors.blue}[INFO]${colors.reset} ${message}`);
  }

  static success(message) {
    console.log(`${colors.green}[SUCCESS]${colors.reset} ${message}`);
  }

  static warning(message) {
    console.log(`${colors.yellow}[WARNING]${colors.reset} ${message}`);
  }

  static error(message) {
    console.log(`${colors.red}[ERROR]${colors.reset} ${message}`);
  }

  static debug(message) {
    if (process.env.DEBUG) {
      console.log(`${colors.magenta}[DEBUG]${colors.reset} ${message}`);
    }
  }
}

class DependencyScanner {
  constructor() {
    this.packageFiles = [];
    this.rootDir = path.resolve(__dirname, '..');
  }

  async scan() {
    Logger.info('Scanning for package files...');
    
    const packagePatterns = [
      '**/package.json',
      '**/Cargo.toml',
      '**/requirements.txt',
      '**/Pipfile',
      '**/go.mod'
    ];

    for (const pattern of packagePatterns) {
      await this.findFiles(pattern);
    }

    Logger.success(`Found ${this.packageFiles.length} package files`);
    return this.packageFiles;
  }

  async findFiles(pattern) {
    try {
      const command = `find ${this.rootDir} -name "${pattern.replace('**/', '')}" -type f`;
      const output = execSync(command, { encoding: 'utf8' });
      const files = output.trim().split('\n').filter(f => f && !f.includes('node_modules'));
      
      for (const file of files) {
        const type = this.getFileType(file);
        this.packageFiles.push({ path: file, type });
      }
    } catch (error) {
      Logger.debug(`Pattern ${pattern} not found or error: ${error.message}`);
    }
  }

  getFileType(filePath) {
    const basename = path.basename(filePath);
    const typeMap = {
      'package.json': 'npm',
      'Cargo.toml': 'cargo',
      'requirements.txt': 'pip',
      'Pipfile': 'pipenv',
      'go.mod': 'go'
    };
    return typeMap[basename] || 'unknown';
  }
}

class VersionResolver {
  constructor() {
    this.cache = new Map();
  }

  async checkPackageVersion(packageName, currentVersion, type = 'npm') {
    const cacheKey = `${type}:${packageName}`;
    
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey);
    }

    try {
      const result = await this.queryRegistry(packageName, type);
      this.cache.set(cacheKey, result);
      return result;
    } catch (error) {
      Logger.error(`Failed to check version for ${packageName}: ${error.message}`);
      return null;
    }
  }

  async queryRegistry(packageName, type) {
    return new Promise((resolve, reject) => {
      const url = type === 'npm' 
        ? `${CONFIG.registries.npm}/${packageName}`
        : `${CONFIG.registries.cargo}/crates/${packageName}`;

      const request = https.get(url, { timeout: CONFIG.timeout }, (response) => {
        let data = '';
        
        response.on('data', chunk => data += chunk);
        response.on('end', () => {
          try {
            const parsed = JSON.parse(data);
            const latestVersion = type === 'npm' 
              ? parsed['dist-tags']?.latest 
              : parsed.crate?.max_version;
            
            resolve({
              name: packageName,
              latest: latestVersion,
              versions: type === 'npm' ? Object.keys(parsed.versions || {}) : [],
              deprecated: parsed.deprecated || false
            });
          } catch (parseError) {
            reject(new Error(`Failed to parse registry response: ${parseError.message}`));
          }
        });
      });

      request.on('error', reject);
      request.on('timeout', () => {
        request.destroy();
        reject(new Error('Registry request timeout'));
      });
    });
  }

  findCompatibleVersion(availableVersions, requestedRange) {
    // Simplified semver compatibility check
    // In production, use a proper semver library
    const cleanRange = requestedRange.replace(/[\^~]/, '');
    
    return availableVersions.find(version => {
      return version.startsWith(cleanRange.split('.')[0]);
    }) || availableVersions[availableVersions.length - 1];
  }
}

class ConflictDetector {
  constructor() {
    this.conflicts = [];
  }

  async detectConflicts(packageFiles, resolver) {
    Logger.info('Detecting dependency conflicts...');
    
    for (const file of packageFiles) {
      if (file.type === 'npm') {
        await this.checkNpmConflicts(file, resolver);
      } else if (file.type === 'cargo') {
        await this.checkCargoConflicts(file, resolver);
      }
    }

    if (this.conflicts.length > 0) {
      Logger.warning(`Found ${this.conflicts.length} potential conflicts`);
      this.conflicts.forEach(conflict => {
        Logger.warning(`  ${conflict.package}: ${conflict.issue}`);
      });
    } else {
      Logger.success('No dependency conflicts detected');
    }

    return this.conflicts;
  }

  async checkNpmConflicts(file, resolver) {
    try {
      const content = await fs.readFile(file.path, 'utf8');
      const packageJson = JSON.parse(content);
      
      const dependencies = {
        ...packageJson.dependencies,
        ...packageJson.devDependencies
      };

      for (const [name, version] of Object.entries(dependencies)) {
        const info = await resolver.checkPackageVersion(name, version, 'npm');
        
        if (!info) {
          this.conflicts.push({
            file: file.path,
            package: name,
            requested: version,
            issue: 'Package not found in registry'
          });
          continue;
        }

        if (!info.versions.length) {
          this.conflicts.push({
            file: file.path,
            package: name,
            requested: version,
            available: info.latest,
            issue: 'No versions available'
          });
          continue;
        }

        const cleanVersion = version.replace(/[\^~]/, '');
        if (!info.versions.includes(cleanVersion) && !info.versions.includes(info.latest)) {
          this.conflicts.push({
            file: file.path,
            package: name,
            requested: version,
            available: info.latest,
            issue: 'Requested version not available'
          });
        }
      }
    } catch (error) {
      Logger.error(`Failed to check conflicts in ${file.path}: ${error.message}`);
    }
  }

  async checkCargoConflicts(file, resolver) {
    // Simplified Cargo.toml parsing
    // In production, use a proper TOML parser
    try {
      const content = await fs.readFile(file.path, 'utf8');
      const dependencySection = content.match(/\[dependencies\]([\s\S]*?)(?=\[|$)/);
      
      if (!dependencySection) return;

      const lines = dependencySection[1].split('\n');
      for (const line of lines) {
        const match = line.match(/^(\w+)\s*=\s*"([^"]+)"/);
        if (match) {
          const [, name, version] = match;
          const info = await resolver.checkPackageVersion(name, version, 'cargo');
          
          if (!info) {
            this.conflicts.push({
              file: file.path,
              package: name,
              requested: version,
              issue: 'Crate not found in registry'
            });
          }
        }
      }
    } catch (error) {
      Logger.error(`Failed to check Cargo conflicts in ${file.path}: ${error.message}`);
    }
  }
}

class DependencyManager {
  constructor() {
    this.scanner = new DependencyScanner();
    this.resolver = new VersionResolver();
    this.conflictDetector = new ConflictDetector();
    this.logger = Logger;
  }

  async run(options = {}) {
    const startTime = Date.now();

    try {
      this.logger.info('🚀 Starting Comprehensive Dependency Health Check');
      this.logger.info(`Mode: ${CONFIG.mode}`);

      // Phase 1: Scan for package files
      const packageFiles = await this.scanner.scan();

      if (packageFiles.length === 0) {
        this.logger.warning('No package files found');
        return;
      }

      // Phase 2: Detect conflicts
      const conflicts = await this.conflictDetector.detectConflicts(packageFiles, this.resolver);

      // Phase 3: Auto-resolve if requested
      if (options.autoFix && conflicts.length > 0) {
        const { default: AutoUpdater } = await import('./dependency-manager/core/auto-updater.js');
        const updater = new AutoUpdater(CONFIG, this.logger);

        this.logger.info('🔧 Auto-fixing detected conflicts...');

        for (const packageFile of packageFiles) {
          const fileConflicts = conflicts.filter(c => c.file === packageFile.path);
          if (fileConflicts.length > 0) {
            await updater.applyUpdates(packageFile, fileConflicts, this.resolver);
          }
        }
      }

      // Phase 4: Health check
      if (options.healthCheck !== false) {
        const { default: HealthChecker } = await import('./dependency-manager/core/health-checker.js');
        const healthChecker = new HealthChecker(CONFIG, this.logger);
        const healthReport = await healthChecker.runHealthCheck(packageFiles);

        this.logger.info(`🏥 Overall Health: ${healthReport.overall.toUpperCase()}`);

        if (healthReport.vulnerabilities.length > 0) {
          this.logger.warning(`⚠️  Found ${healthReport.vulnerabilities.length} security vulnerabilities`);
        }
      }

      const duration = ((Date.now() - startTime) / 1000).toFixed(2);
      this.logger.success(`✅ Dependency health check completed in ${duration}s`);

    } catch (error) {
      this.logger.error(`❌ Dependency health check failed: ${error.message}`);
      process.exit(1);
    }
  }
}

// CLI Interface
async function main() {
  const args = process.argv.slice(2);
  const options = {
    autoFix: args.includes('--auto-fix') || args.includes('-f'),
    healthCheck: !args.includes('--no-health-check'),
    mode: args.find(arg => arg.startsWith('--mode='))?.split('=')[1] || CONFIG.mode
  };

  // Override config mode if specified
  if (options.mode) {
    CONFIG.mode = options.mode;
  }

  const manager = new DependencyManager();
  await manager.run(options);
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(error => {
    console.error('Fatal error:', error);
    process.exit(1);
  });
}

export { DependencyScanner, VersionResolver, ConflictDetector, DependencyManager, Logger, CONFIG };
