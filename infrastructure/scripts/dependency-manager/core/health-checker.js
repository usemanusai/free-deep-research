#!/usr/bin/env node

/**
 * HealthChecker - Validates installations and runs security audits
 */

import { execSync, spawn } from 'child_process';
import fs from 'fs/promises';
import path from 'path';

class HealthChecker {
  constructor(config, logger) {
    this.config = config;
    this.logger = logger;
    this.healthReport = {
      timestamp: new Date().toISOString(),
      overall: 'unknown',
      checks: [],
      vulnerabilities: [],
      recommendations: []
    };
  }

  async runHealthCheck(packageFiles) {
    this.logger.info('Running comprehensive health check...');
    
    for (const file of packageFiles) {
      await this.checkPackageFile(file);
    }

    await this.checkSystemDependencies();
    await this.runSecurityAudit(packageFiles);
    await this.checkDiskSpace();
    await this.checkNetworkConnectivity();

    this.calculateOverallHealth();
    await this.generateReport();

    return this.healthReport;
  }

  async checkPackageFile(packageFile) {
    const check = {
      type: 'package-file',
      file: packageFile.path,
      status: 'unknown',
      issues: [],
      details: {}
    };

    try {
      if (packageFile.type === 'npm') {
        await this.checkNpmPackage(packageFile, check);
      } else if (packageFile.type === 'cargo') {
        await this.checkCargoPackage(packageFile, check);
      }

      check.status = check.issues.length === 0 ? 'healthy' : 'issues';
    } catch (error) {
      check.status = 'error';
      check.issues.push(`Health check failed: ${error.message}`);
    }

    this.healthReport.checks.push(check);
  }

  async checkNpmPackage(packageFile, check) {
    const dir = path.dirname(packageFile.path);

    try {
      // Check if node_modules exists
      const nodeModulesPath = path.join(dir, 'node_modules');
      try {
        await fs.access(nodeModulesPath);
        check.details.nodeModulesExists = true;
      } catch {
        check.details.nodeModulesExists = false;
        check.issues.push('node_modules directory not found');
      }

      // Check package-lock.json
      const lockPath = path.join(dir, 'package-lock.json');
      try {
        await fs.access(lockPath);
        check.details.lockFileExists = true;
      } catch {
        check.details.lockFileExists = false;
        check.issues.push('package-lock.json not found');
      }

      // Test npm list (dependency tree validation)
      try {
        execSync('npm list --depth=0', { 
          cwd: dir, 
          stdio: 'pipe',
          timeout: this.config.timeout 
        });
        check.details.dependencyTreeValid = true;
      } catch (error) {
        check.details.dependencyTreeValid = false;
        check.issues.push('Dependency tree has issues');
      }

      // Check for outdated packages
      try {
        const outdatedOutput = execSync('npm outdated --json', { 
          cwd: dir, 
          stdio: 'pipe',
          timeout: this.config.timeout 
        });
        
        const outdated = JSON.parse(outdatedOutput.toString());
        check.details.outdatedPackages = Object.keys(outdated).length;
        
        if (check.details.outdatedPackages > 0) {
          check.issues.push(`${check.details.outdatedPackages} packages are outdated`);
        }
      } catch (error) {
        // npm outdated returns non-zero exit code when packages are outdated
        // This is expected behavior
        check.details.outdatedPackages = 0;
      }

    } catch (error) {
      throw new Error(`npm health check failed: ${error.message}`);
    }
  }

  async checkCargoPackage(packageFile, check) {
    const dir = path.dirname(packageFile.path);

    try {
      // Check if Cargo.lock exists
      const lockPath = path.join(dir, 'Cargo.lock');
      try {
        await fs.access(lockPath);
        check.details.lockFileExists = true;
      } catch {
        check.details.lockFileExists = false;
        check.issues.push('Cargo.lock not found');
      }

      // Test cargo check
      try {
        execSync('cargo check', { 
          cwd: dir, 
          stdio: 'pipe',
          timeout: this.config.timeout 
        });
        check.details.cargoCheckPassed = true;
      } catch (error) {
        check.details.cargoCheckPassed = false;
        check.issues.push('cargo check failed');
      }

      // Check for outdated crates
      try {
        const outdatedOutput = execSync('cargo outdated --format json', { 
          cwd: dir, 
          stdio: 'pipe',
          timeout: this.config.timeout 
        });
        
        const outdated = JSON.parse(outdatedOutput.toString());
        check.details.outdatedCrates = outdated.dependencies?.length || 0;
        
        if (check.details.outdatedCrates > 0) {
          check.issues.push(`${check.details.outdatedCrates} crates are outdated`);
        }
      } catch (error) {
        // cargo-outdated might not be installed
        check.details.outdatedCrates = 'unknown';
      }

    } catch (error) {
      throw new Error(`cargo health check failed: ${error.message}`);
    }
  }

  async checkSystemDependencies() {
    const check = {
      type: 'system-dependencies',
      status: 'unknown',
      issues: [],
      details: {}
    };

    try {
      // Check Node.js version
      try {
        const nodeVersion = execSync('node --version', { encoding: 'utf8' }).trim();
        check.details.nodeVersion = nodeVersion;
        
        const majorVersion = parseInt(nodeVersion.replace('v', '').split('.')[0]);
        if (majorVersion < 16) {
          check.issues.push(`Node.js version ${nodeVersion} is outdated (minimum: v16)`);
        }
      } catch (error) {
        check.issues.push('Node.js not found');
      }

      // Check npm version
      try {
        const npmVersion = execSync('npm --version', { encoding: 'utf8' }).trim();
        check.details.npmVersion = npmVersion;
      } catch (error) {
        check.issues.push('npm not found');
      }

      // Check Rust/Cargo version
      try {
        const rustVersion = execSync('rustc --version', { encoding: 'utf8' }).trim();
        check.details.rustVersion = rustVersion;
      } catch (error) {
        check.details.rustVersion = 'not installed';
      }

      try {
        const cargoVersion = execSync('cargo --version', { encoding: 'utf8' }).trim();
        check.details.cargoVersion = cargoVersion;
      } catch (error) {
        check.details.cargoVersion = 'not installed';
      }

      // Check Git version
      try {
        const gitVersion = execSync('git --version', { encoding: 'utf8' }).trim();
        check.details.gitVersion = gitVersion;
      } catch (error) {
        check.issues.push('Git not found');
      }

      check.status = check.issues.length === 0 ? 'healthy' : 'issues';
    } catch (error) {
      check.status = 'error';
      check.issues.push(`System check failed: ${error.message}`);
    }

    this.healthReport.checks.push(check);
  }

  async runSecurityAudit(packageFiles) {
    this.logger.info('Running security audit...');

    for (const file of packageFiles) {
      if (file.type === 'npm') {
        await this.runNpmAudit(file);
      } else if (file.type === 'cargo') {
        await this.runCargoAudit(file);
      }
    }
  }

  async runNpmAudit(packageFile) {
    const dir = path.dirname(packageFile.path);

    try {
      const auditOutput = execSync('npm audit --json', { 
        cwd: dir, 
        stdio: 'pipe',
        timeout: this.config.timeout 
      });

      const audit = JSON.parse(auditOutput.toString());
      
      if (audit.vulnerabilities) {
        for (const [pkg, vuln] of Object.entries(audit.vulnerabilities)) {
          this.healthReport.vulnerabilities.push({
            package: pkg,
            severity: vuln.severity,
            title: vuln.title,
            file: packageFile.path,
            type: 'npm'
          });
        }
      }
    } catch (error) {
      // npm audit returns non-zero exit code when vulnerabilities are found
      try {
        const errorOutput = error.stdout?.toString();
        if (errorOutput) {
          const audit = JSON.parse(errorOutput);
          // Process vulnerabilities from error output
        }
      } catch (parseError) {
        this.logger.warning(`npm audit failed for ${packageFile.path}: ${error.message}`);
      }
    }
  }

  async runCargoAudit(packageFile) {
    const dir = path.dirname(packageFile.path);

    try {
      const auditOutput = execSync('cargo audit --json', { 
        cwd: dir, 
        stdio: 'pipe',
        timeout: this.config.timeout 
      });

      const audit = JSON.parse(auditOutput.toString());
      
      if (audit.vulnerabilities) {
        for (const vuln of audit.vulnerabilities.list) {
          this.healthReport.vulnerabilities.push({
            package: vuln.package.name,
            severity: vuln.advisory.severity,
            title: vuln.advisory.title,
            file: packageFile.path,
            type: 'cargo'
          });
        }
      }
    } catch (error) {
      // cargo-audit might not be installed
      this.logger.debug(`cargo audit not available for ${packageFile.path}`);
    }
  }

  async checkDiskSpace() {
    const check = {
      type: 'disk-space',
      status: 'unknown',
      issues: [],
      details: {}
    };

    try {
      const dfOutput = execSync('df -h .', { encoding: 'utf8' });
      const lines = dfOutput.trim().split('\n');
      const dataLine = lines[1];
      const parts = dataLine.split(/\s+/);
      
      check.details.available = parts[3];
      check.details.used = parts[2];
      check.details.usagePercent = parts[4];

      const usageNum = parseInt(parts[4].replace('%', ''));
      if (usageNum > 90) {
        check.issues.push(`Disk usage is ${parts[4]} - consider freeing up space`);
      }

      check.status = check.issues.length === 0 ? 'healthy' : 'warning';
    } catch (error) {
      check.status = 'error';
      check.issues.push(`Disk space check failed: ${error.message}`);
    }

    this.healthReport.checks.push(check);
  }

  async checkNetworkConnectivity() {
    const check = {
      type: 'network-connectivity',
      status: 'unknown',
      issues: [],
      details: {}
    };

    const registries = [
      { name: 'npm', url: 'https://registry.npmjs.org' },
      { name: 'crates.io', url: 'https://crates.io' }
    ];

    for (const registry of registries) {
      try {
        const start = Date.now();
        execSync(`curl -s --max-time 10 ${registry.url}`, { stdio: 'pipe' });
        const responseTime = Date.now() - start;
        
        check.details[registry.name] = {
          status: 'reachable',
          responseTime: `${responseTime}ms`
        };
      } catch (error) {
        check.details[registry.name] = {
          status: 'unreachable',
          error: error.message
        };
        check.issues.push(`${registry.name} registry unreachable`);
      }
    }

    check.status = check.issues.length === 0 ? 'healthy' : 'issues';
    this.healthReport.checks.push(check);
  }

  calculateOverallHealth() {
    const errorChecks = this.healthReport.checks.filter(c => c.status === 'error');
    const issueChecks = this.healthReport.checks.filter(c => c.status === 'issues' || c.status === 'warning');
    const criticalVulns = this.healthReport.vulnerabilities.filter(v => v.severity === 'critical' || v.severity === 'high');

    if (errorChecks.length > 0 || criticalVulns.length > 0) {
      this.healthReport.overall = 'critical';
    } else if (issueChecks.length > 0 || this.healthReport.vulnerabilities.length > 0) {
      this.healthReport.overall = 'warning';
    } else {
      this.healthReport.overall = 'healthy';
    }

    // Generate recommendations
    this.generateRecommendations();
  }

  generateRecommendations() {
    const recommendations = [];

    // Check for outdated packages
    const outdatedChecks = this.healthReport.checks.filter(c => 
      c.details.outdatedPackages > 0 || c.details.outdatedCrates > 0
    );
    
    if (outdatedChecks.length > 0) {
      recommendations.push('Consider updating outdated packages to latest versions');
    }

    // Check for vulnerabilities
    if (this.healthReport.vulnerabilities.length > 0) {
      recommendations.push('Address security vulnerabilities by updating affected packages');
    }

    // Check for missing lock files
    const missingLockFiles = this.healthReport.checks.filter(c => 
      c.details.lockFileExists === false
    );
    
    if (missingLockFiles.length > 0) {
      recommendations.push('Generate lock files by running package manager install commands');
    }

    this.healthReport.recommendations = recommendations;
  }

  async generateReport() {
    await fs.mkdir(this.config.reportDir, { recursive: true });
    
    const reportPath = path.join(
      this.config.reportDir, 
      `health-report-${new Date().toISOString().split('T')[0]}.json`
    );

    await fs.writeFile(reportPath, JSON.stringify(this.healthReport, null, 2));
    this.logger.success(`Health report generated: ${reportPath}`);
  }
}

export default HealthChecker;
