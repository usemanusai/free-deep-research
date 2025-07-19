#!/usr/bin/env node

/**
 * Free Deep Research System - Dependency Health Check Script
 * Comprehensive dependency analysis, security scanning, and health monitoring
 */

import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { readFileSync, writeFileSync, existsSync } from 'fs';
import { execSync } from 'child_process';

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

class DependencyHealthChecker {
  constructor(options = {}) {
    this.options = {
      mode: options.mode || 'comprehensive', // comprehensive, conservative, security-only
      autoFix: options.autoFix || false,
      outputFile: options.outputFile || 'dependency-health-report.json',
      ...options,
    };

    this.results = {
      timestamp: new Date().toISOString(),
      mode: this.options.mode,
      summary: {
        totalDependencies: 0,
        outdatedDependencies: 0,
        vulnerabilities: 0,
        healthScore: 0,
      },
      dependencies: {},
      vulnerabilities: [],
      recommendations: [],
      errors: [],
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
  }

  logError(message) {
    this.log(`✗ ${message}`, 'red');
    this.results.errors.push(message);
  }

  logInfo(message) {
    this.log(`ℹ ${message}`, 'blue');
  }

  async runHealthCheck() {
    this.log('\n🔍 Dependency Health Check', 'cyan');
    this.log(`Mode: ${this.options.mode}`, 'cyan');
    this.log('=' .repeat(50), 'cyan');

    try {
      await this.analyzeDependencies();
      await this.checkSecurity();
      await this.checkOutdated();
      await this.analyzePackageJson();
      await this.checkLockFiles();
      await this.generateRecommendations();
      
      if (this.options.autoFix) {
        await this.applyAutoFixes();
      }

      this.calculateHealthScore();
      this.generateReport();
    } catch (error) {
      this.logError(`Health check failed: ${error.message}`);
      throw error;
    }
  }

  async analyzeDependencies() {
    this.log('\n📦 Analyzing Dependencies', 'bright');

    const packageJsonPath = join(__dirname, '../package.json');
    if (!existsSync(packageJsonPath)) {
      throw new Error('package.json not found');
    }

    const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
    const dependencies = packageJson.dependencies || {};
    const devDependencies = packageJson.devDependencies || {};
    const allDependencies = { ...dependencies, ...devDependencies };

    this.results.summary.totalDependencies = Object.keys(allDependencies).length;

    this.logSuccess(`Found ${Object.keys(dependencies).length} production dependencies`);
    this.logSuccess(`Found ${Object.keys(devDependencies).length} development dependencies`);

    // Analyze each dependency
    for (const [name, version] of Object.entries(allDependencies)) {
      try {
        const packageInfo = await this.getPackageInfo(name);
        this.results.dependencies[name] = {
          currentVersion: version,
          latestVersion: packageInfo.latest,
          type: dependencies[name] ? 'production' : 'development',
          ...packageInfo,
        };
      } catch (error) {
        this.logWarning(`Could not analyze ${name}: ${error.message}`);
        this.results.dependencies[name] = {
          currentVersion: version,
          error: error.message,
        };
      }
    }
  }

  async getPackageInfo(packageName) {
    try {
      const output = execSync(`npm view ${packageName} --json`, { 
        encoding: 'utf8',
        timeout: 10000,
      });
      
      const info = JSON.parse(output);
      return {
        latest: info.version,
        description: info.description,
        homepage: info.homepage,
        repository: info.repository?.url,
        license: info.license,
        maintainers: info.maintainers?.length || 0,
        lastPublished: info.time?.[info.version],
        weeklyDownloads: await this.getWeeklyDownloads(packageName),
      };
    } catch (error) {
      throw new Error(`Failed to get package info: ${error.message}`);
    }
  }

  async getWeeklyDownloads(packageName) {
    try {
      const output = execSync(`npm view ${packageName} --json`, { 
        encoding: 'utf8',
        timeout: 5000,
      });
      // This is a simplified version - in reality you'd call npm registry API
      return 'N/A';
    } catch {
      return 'N/A';
    }
  }

  async checkSecurity() {
    this.log('\n🔒 Security Audit', 'bright');

    try {
      // Run npm audit
      const auditOutput = execSync('npm audit --json', { 
        encoding: 'utf8',
        timeout: 30000,
      });
      
      const auditResult = JSON.parse(auditOutput);
      
      if (auditResult.vulnerabilities) {
        const vulnCount = Object.keys(auditResult.vulnerabilities).length;
        this.results.summary.vulnerabilities = vulnCount;
        
        if (vulnCount === 0) {
          this.logSuccess('No security vulnerabilities found');
        } else {
          this.logWarning(`Found ${vulnCount} security vulnerabilities`);
          
          // Categorize vulnerabilities
          const severityCount = { low: 0, moderate: 0, high: 0, critical: 0 };
          
          for (const [name, vuln] of Object.entries(auditResult.vulnerabilities)) {
            const severity = vuln.severity;
            severityCount[severity]++;
            
            this.results.vulnerabilities.push({
              package: name,
              severity,
              title: vuln.title,
              url: vuln.url,
              range: vuln.range,
            });
          }
          
          this.logInfo(`Severity breakdown: Critical: ${severityCount.critical}, High: ${severityCount.high}, Moderate: ${severityCount.moderate}, Low: ${severityCount.low}`);
        }
      }
    } catch (error) {
      if (error.status === 1) {
        // npm audit returns exit code 1 when vulnerabilities are found
        try {
          const auditResult = JSON.parse(error.stdout);
          this.results.summary.vulnerabilities = Object.keys(auditResult.vulnerabilities || {}).length;
          this.logWarning(`Security audit completed with ${this.results.summary.vulnerabilities} vulnerabilities`);
        } catch {
          this.logError('Failed to parse security audit results');
        }
      } else {
        this.logError(`Security audit failed: ${error.message}`);
      }
    }
  }

  async checkOutdated() {
    this.log('\n📅 Checking Outdated Packages', 'bright');

    try {
      const outdatedOutput = execSync('npm outdated --json', { 
        encoding: 'utf8',
        timeout: 30000,
      });
      
      const outdatedPackages = JSON.parse(outdatedOutput);
      const outdatedCount = Object.keys(outdatedPackages).length;
      
      this.results.summary.outdatedDependencies = outdatedCount;
      
      if (outdatedCount === 0) {
        this.logSuccess('All packages are up to date');
      } else {
        this.logWarning(`${outdatedCount} packages are outdated`);
        
        for (const [name, info] of Object.entries(outdatedPackages)) {
          const current = info.current;
          const wanted = info.wanted;
          const latest = info.latest;
          
          if (this.results.dependencies[name]) {
            this.results.dependencies[name].outdated = {
              current,
              wanted,
              latest,
              type: info.type,
            };
          }
          
          this.logInfo(`${name}: ${current} → ${wanted} (latest: ${latest})`);
        }
      }
    } catch (error) {
      if (error.status === 1) {
        // npm outdated returns exit code 1 when outdated packages are found
        this.logInfo('Outdated check completed');
      } else {
        this.logError(`Outdated check failed: ${error.message}`);
      }
    }
  }

  async analyzePackageJson() {
    this.log('\n📋 Package.json Analysis', 'bright');

    const packageJsonPath = join(__dirname, '../package.json');
    const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));

    // Check for required fields
    const requiredFields = ['name', 'version', 'description', 'main', 'scripts', 'author', 'license'];
    const missingFields = requiredFields.filter(field => !packageJson[field]);
    
    if (missingFields.length === 0) {
      this.logSuccess('All required package.json fields are present');
    } else {
      this.logWarning(`Missing package.json fields: ${missingFields.join(', ')}`);
    }

    // Check scripts
    const recommendedScripts = ['test', 'build', 'start', 'lint'];
    const missingScripts = recommendedScripts.filter(script => !packageJson.scripts?.[script]);
    
    if (missingScripts.length === 0) {
      this.logSuccess('All recommended scripts are present');
    } else {
      this.logInfo(`Consider adding scripts: ${missingScripts.join(', ')}`);
    }

    // Check for security-related configurations
    if (packageJson.engines) {
      this.logSuccess('Node.js engine version specified');
    } else {
      this.logWarning('Consider specifying Node.js engine version');
    }
  }

  async checkLockFiles() {
    this.log('\n🔒 Lock File Analysis', 'bright');

    const lockFiles = [
      { name: 'package-lock.json', manager: 'npm' },
      { name: 'yarn.lock', manager: 'yarn' },
      { name: 'pnpm-lock.yaml', manager: 'pnpm' },
    ];

    const presentLockFiles = lockFiles.filter(lock => 
      existsSync(join(__dirname, '..', lock.name))
    );

    if (presentLockFiles.length === 1) {
      this.logSuccess(`Lock file present: ${presentLockFiles[0].name}`);
    } else if (presentLockFiles.length === 0) {
      this.logWarning('No lock file found - consider using npm, yarn, or pnpm');
    } else {
      this.logWarning(`Multiple lock files found: ${presentLockFiles.map(l => l.name).join(', ')}`);
      this.results.recommendations.push('Remove unused lock files to avoid conflicts');
    }
  }

  generateRecommendations() {
    this.log('\n💡 Generating Recommendations', 'bright');

    // Security recommendations
    if (this.results.summary.vulnerabilities > 0) {
      this.results.recommendations.push('Run "npm audit fix" to automatically fix vulnerabilities');
      
      if (this.results.vulnerabilities.some(v => v.severity === 'critical' || v.severity === 'high')) {
        this.results.recommendations.push('Address critical and high severity vulnerabilities immediately');
      }
    }

    // Outdated package recommendations
    if (this.results.summary.outdatedDependencies > 0) {
      if (this.options.mode === 'conservative') {
        this.results.recommendations.push('Update packages conservatively using "npm update"');
      } else {
        this.results.recommendations.push('Consider updating outdated packages to latest versions');
      }
    }

    // General recommendations
    const totalDeps = this.results.summary.totalDependencies;
    if (totalDeps > 100) {
      this.results.recommendations.push('Consider reviewing dependencies - large dependency trees can impact security and performance');
    }

    // Display recommendations
    if (this.results.recommendations.length > 0) {
      this.results.recommendations.forEach((rec, index) => {
        this.logInfo(`${index + 1}. ${rec}`);
      });
    } else {
      this.logSuccess('No specific recommendations - dependencies look healthy!');
    }
  }

  async applyAutoFixes() {
    this.log('\n🔧 Applying Auto-fixes', 'bright');

    if (this.results.summary.vulnerabilities > 0) {
      try {
        this.logInfo('Running npm audit fix...');
        execSync('npm audit fix', { stdio: 'inherit', timeout: 60000 });
        this.logSuccess('Auto-fix completed');
      } catch (error) {
        this.logWarning('Auto-fix encountered issues - manual review may be needed');
      }
    }

    if (this.options.mode !== 'security-only' && this.results.summary.outdatedDependencies > 0) {
      try {
        this.logInfo('Updating packages...');
        if (this.options.mode === 'conservative') {
          execSync('npm update', { stdio: 'inherit', timeout: 120000 });
        } else {
          // More aggressive updates would go here
          this.logInfo('Aggressive updates not implemented in auto-fix mode');
        }
        this.logSuccess('Package updates completed');
      } catch (error) {
        this.logWarning('Package updates encountered issues');
      }
    }
  }

  calculateHealthScore() {
    let score = 100;

    // Deduct points for vulnerabilities
    const vulnPenalty = {
      critical: 25,
      high: 15,
      moderate: 5,
      low: 1,
    };

    this.results.vulnerabilities.forEach(vuln => {
      score -= vulnPenalty[vuln.severity] || 0;
    });

    // Deduct points for outdated packages
    const outdatedPenalty = Math.min(this.results.summary.outdatedDependencies * 2, 20);
    score -= outdatedPenalty;

    // Deduct points for errors
    score -= this.results.errors.length * 5;

    this.results.summary.healthScore = Math.max(0, Math.round(score));
  }

  generateReport() {
    this.log('\n📊 Dependency Health Summary', 'cyan');
    this.log('=' .repeat(50), 'cyan');

    const { summary } = this.results;
    
    this.log(`Health Score: ${summary.healthScore}/100`, 
      summary.healthScore >= 80 ? 'green' : summary.healthScore >= 60 ? 'yellow' : 'red');
    
    this.log(`Total Dependencies: ${summary.totalDependencies}`);
    this.log(`Outdated: ${summary.outdatedDependencies}`);
    this.log(`Vulnerabilities: ${summary.vulnerabilities}`);
    this.log(`Recommendations: ${this.results.recommendations.length}`);

    // Save detailed report
    const reportPath = join(__dirname, '..', this.options.outputFile);
    writeFileSync(reportPath, JSON.stringify(this.results, null, 2));
    this.log(`\n📄 Detailed report saved to: ${reportPath}`, 'blue');

    // Exit with appropriate code
    if (summary.healthScore < 60 || this.results.errors.length > 0) {
      this.log('\n❌ Dependency health check failed', 'red');
      process.exit(1);
    } else if (summary.vulnerabilities > 0 || summary.outdatedDependencies > 10) {
      this.log('\n⚠️ Dependency health check completed with warnings', 'yellow');
      process.exit(0);
    } else {
      this.log('\n✅ Dependency health check passed', 'green');
      process.exit(0);
    }
  }
}

// Parse command line arguments
function parseArgs() {
  const args = process.argv.slice(2);
  const options = {};

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    
    if (arg === '--auto-fix') {
      options.autoFix = true;
    } else if (arg === '--mode') {
      options.mode = args[++i];
    } else if (arg === '--output') {
      options.outputFile = args[++i];
    } else if (arg === '--help') {
      console.log(`
Usage: node dependency-health-check.js [options]

Options:
  --mode <mode>        Set check mode: comprehensive, conservative, security-only
  --auto-fix          Automatically fix issues where possible
  --output <file>     Output report file name
  --help              Show this help message

Examples:
  node dependency-health-check.js --mode conservative
  node dependency-health-check.js --auto-fix --mode security-only
      `);
      process.exit(0);
    }
  }

  return options;
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  const options = parseArgs();
  const checker = new DependencyHealthChecker(options);
  
  checker.runHealthCheck().catch(error => {
    console.error('Dependency health check failed:', error);
    process.exit(1);
  });
}

export default DependencyHealthChecker;
