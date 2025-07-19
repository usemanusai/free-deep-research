#!/usr/bin/env node

/**
 * Test script to verify the @visx/hierarchy dependency fix and overall npm install process
 */

import { execSync, spawn } from 'child_process';
import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

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

class TestLogger {
  static info(message) {
    console.log(`${colors.blue}[TEST-INFO]${colors.reset} ${message}`);
  }

  static success(message) {
    console.log(`${colors.green}[TEST-PASS]${colors.reset} ${message}`);
  }

  static warning(message) {
    console.log(`${colors.yellow}[TEST-WARN]${colors.reset} ${message}`);
  }

  static error(message) {
    console.log(`${colors.red}[TEST-FAIL]${colors.reset} ${message}`);
  }

  static step(message) {
    console.log(`${colors.cyan}[TEST-STEP]${colors.reset} ${message}`);
  }
}

class DependencyFixTester {
  constructor() {
    this.rootDir = path.resolve(__dirname, '..');
    this.testResults = {
      timestamp: new Date().toISOString(),
      tests: [],
      overall: 'unknown',
      summary: {}
    };
  }

  async runAllTests() {
    TestLogger.info('🧪 Starting Dependency Fix Verification Tests');
    TestLogger.info('=' .repeat(60));

    try {
      await this.testPackageJsonFix();
      await this.testNpmInstallProcess();
      await this.testVisxPackageAvailability();
      await this.testDependencyTreeIntegrity();
      await this.testBuildProcess();
      await this.testDependencyManagerSystem();
      
      this.calculateOverallResult();
      await this.generateTestReport();
      
      TestLogger.info('=' .repeat(60));
      TestLogger.info(`🏁 Test Suite Completed - Overall: ${this.testResults.overall.toUpperCase()}`);
      
      if (this.testResults.overall === 'failed') {
        process.exit(1);
      }
      
    } catch (error) {
      TestLogger.error(`Fatal test error: ${error.message}`);
      process.exit(1);
    }
  }

  async testPackageJsonFix() {
    const test = {
      name: 'Package.json @visx/hierarchy Fix',
      status: 'unknown',
      details: {},
      issues: []
    };

    TestLogger.step('Testing package.json @visx/hierarchy version fix...');

    try {
      const packagePath = path.join(this.rootDir, 'bmad-agent/free-deep-research/package.json');
      const content = await fs.readFile(packagePath, 'utf8');
      const packageJson = JSON.parse(content);

      // Check @visx/hierarchy version
      const hierarchyVersion = packageJson.dependencies?.['@visx/hierarchy'];
      const networkVersion = packageJson.dependencies?.['@visx/network'];

      test.details.hierarchyVersion = hierarchyVersion;
      test.details.networkVersion = networkVersion;

      if (hierarchyVersion === '^3.12.0') {
        TestLogger.success('@visx/hierarchy version correctly set to ^3.12.0');
      } else {
        test.issues.push(`@visx/hierarchy version is ${hierarchyVersion}, expected ^3.12.0`);
      }

      if (networkVersion === '^3.12.0') {
        TestLogger.success('@visx/network version correctly set to ^3.12.0');
      } else {
        test.issues.push(`@visx/network version is ${networkVersion}, expected ^3.12.0`);
      }

      test.status = test.issues.length === 0 ? 'passed' : 'failed';
    } catch (error) {
      test.status = 'error';
      test.issues.push(`Failed to read package.json: ${error.message}`);
    }

    this.testResults.tests.push(test);
    this.logTestResult(test);
  }

  async testVisxPackageAvailability() {
    const test = {
      name: 'Visx Package Registry Availability',
      status: 'unknown',
      details: {},
      issues: []
    };

    TestLogger.step('Testing @visx package availability in npm registry...');

    try {
      const packages = ['@visx/hierarchy', '@visx/network'];
      
      for (const pkg of packages) {
        try {
          const output = execSync(`npm view ${pkg} version`, { 
            encoding: 'utf8',
            timeout: 10000 
          });
          
          const availableVersion = output.trim();
          test.details[pkg] = availableVersion;
          
          if (availableVersion === '3.12.0') {
            TestLogger.success(`${pkg} version 3.12.0 is available in registry`);
          } else {
            test.issues.push(`${pkg} latest version is ${availableVersion}, not 3.12.0`);
          }
        } catch (error) {
          test.issues.push(`Failed to check ${pkg}: ${error.message}`);
        }
      }

      test.status = test.issues.length === 0 ? 'passed' : 'failed';
    } catch (error) {
      test.status = 'error';
      test.issues.push(`Registry check failed: ${error.message}`);
    }

    this.testResults.tests.push(test);
    this.logTestResult(test);
  }

  async testNpmInstallProcess() {
    const test = {
      name: 'NPM Install Process',
      status: 'unknown',
      details: {},
      issues: []
    };

    TestLogger.step('Testing npm install process...');

    const projectDirs = [
      'bmad-agent/free-deep-research',
      'bmad-agent/deep_research_frontend'
    ];

    for (const dir of projectDirs) {
      const projectPath = path.join(this.rootDir, dir);
      
      try {
        await fs.access(path.join(projectPath, 'package.json'));
        
        TestLogger.info(`Testing npm install in ${dir}...`);
        
        // Clean install test
        const startTime = Date.now();
        execSync('npm ci --silent', { 
          cwd: projectPath,
          stdio: 'pipe',
          timeout: 120000 // 2 minutes timeout
        });
        const installTime = Date.now() - startTime;
        
        test.details[dir] = {
          installTime: `${installTime}ms`,
          status: 'success'
        };
        
        TestLogger.success(`npm install completed successfully in ${dir} (${installTime}ms)`);
        
        // Verify node_modules exists
        await fs.access(path.join(projectPath, 'node_modules'));
        TestLogger.success(`node_modules directory created in ${dir}`);
        
      } catch (error) {
        test.issues.push(`npm install failed in ${dir}: ${error.message}`);
        test.details[dir] = {
          status: 'failed',
          error: error.message
        };
      }
    }

    test.status = test.issues.length === 0 ? 'passed' : 'failed';
    this.testResults.tests.push(test);
    this.logTestResult(test);
  }

  async testDependencyTreeIntegrity() {
    const test = {
      name: 'Dependency Tree Integrity',
      status: 'unknown',
      details: {},
      issues: []
    };

    TestLogger.step('Testing dependency tree integrity...');

    const projectDirs = [
      'bmad-agent/free-deep-research',
      'bmad-agent/deep_research_frontend'
    ];

    for (const dir of projectDirs) {
      const projectPath = path.join(this.rootDir, dir);
      
      try {
        await fs.access(path.join(projectPath, 'package.json'));
        
        // Test npm list
        const output = execSync('npm list --depth=0', { 
          cwd: projectPath,
          encoding: 'utf8',
          timeout: 30000
        });
        
        test.details[dir] = {
          status: 'valid',
          packages: output.split('\n').length - 1
        };
        
        TestLogger.success(`Dependency tree is valid in ${dir}`);
        
      } catch (error) {
        // npm list returns non-zero exit code for missing dependencies
        const errorOutput = error.stdout?.toString() || error.message;
        
        if (errorOutput.includes('missing') || errorOutput.includes('invalid')) {
          test.issues.push(`Dependency tree issues in ${dir}: ${errorOutput}`);
          test.details[dir] = {
            status: 'issues',
            error: errorOutput
          };
        } else {
          test.details[dir] = {
            status: 'valid',
            note: 'npm list completed with warnings'
          };
          TestLogger.success(`Dependency tree is valid in ${dir} (with warnings)`);
        }
      }
    }

    test.status = test.issues.length === 0 ? 'passed' : 'failed';
    this.testResults.tests.push(test);
    this.logTestResult(test);
  }

  async testBuildProcess() {
    const test = {
      name: 'Build Process Verification',
      status: 'unknown',
      details: {},
      issues: []
    };

    TestLogger.step('Testing build process...');

    const buildTests = [
      {
        dir: 'bmad-agent/free-deep-research',
        command: 'npm run build:frontend',
        description: 'Frontend build'
      },
      {
        dir: 'bmad-agent/deep_research_frontend',
        command: 'npm run build',
        description: 'React frontend build'
      }
    ];

    for (const buildTest of buildTests) {
      const projectPath = path.join(this.rootDir, buildTest.dir);
      
      try {
        await fs.access(path.join(projectPath, 'package.json'));
        
        TestLogger.info(`Testing ${buildTest.description}...`);
        
        const startTime = Date.now();
        execSync(buildTest.command, { 
          cwd: projectPath,
          stdio: 'pipe',
          timeout: 180000 // 3 minutes timeout
        });
        const buildTime = Date.now() - startTime;
        
        test.details[buildTest.dir] = {
          command: buildTest.command,
          buildTime: `${buildTime}ms`,
          status: 'success'
        };
        
        TestLogger.success(`${buildTest.description} completed successfully (${buildTime}ms)`);
        
      } catch (error) {
        test.issues.push(`${buildTest.description} failed: ${error.message}`);
        test.details[buildTest.dir] = {
          command: buildTest.command,
          status: 'failed',
          error: error.message
        };
      }
    }

    test.status = test.issues.length === 0 ? 'passed' : 'failed';
    this.testResults.tests.push(test);
    this.logTestResult(test);
  }

  async testDependencyManagerSystem() {
    const test = {
      name: 'Dependency Manager System',
      status: 'unknown',
      details: {},
      issues: []
    };

    TestLogger.step('Testing dependency management system...');

    try {
      // Test if the dependency health check script exists and runs
      const scriptPath = path.join(__dirname, 'dependency-health-check.js');
      
      try {
        await fs.access(scriptPath);
        test.details.scriptExists = true;
        TestLogger.success('Dependency health check script exists');
      } catch (error) {
        test.issues.push('Dependency health check script not found');
        test.details.scriptExists = false;
      }

      // Test running the dependency health check
      try {
        const output = execSync('node scripts/dependency-health-check.js --no-health-check', { 
          cwd: this.rootDir,
          encoding: 'utf8',
          timeout: 60000
        });
        
        test.details.healthCheckRun = true;
        test.details.output = output.substring(0, 200) + '...'; // Truncate for report
        TestLogger.success('Dependency health check executed successfully');
      } catch (error) {
        test.issues.push(`Dependency health check failed: ${error.message}`);
        test.details.healthCheckRun = false;
      }

      test.status = test.issues.length === 0 ? 'passed' : 'failed';
    } catch (error) {
      test.status = 'error';
      test.issues.push(`System test failed: ${error.message}`);
    }

    this.testResults.tests.push(test);
    this.logTestResult(test);
  }

  logTestResult(test) {
    if (test.status === 'passed') {
      TestLogger.success(`✅ ${test.name}: PASSED`);
    } else if (test.status === 'failed') {
      TestLogger.error(`❌ ${test.name}: FAILED`);
      test.issues.forEach(issue => TestLogger.error(`   - ${issue}`));
    } else if (test.status === 'error') {
      TestLogger.error(`💥 ${test.name}: ERROR`);
      test.issues.forEach(issue => TestLogger.error(`   - ${issue}`));
    }
  }

  calculateOverallResult() {
    const passed = this.testResults.tests.filter(t => t.status === 'passed').length;
    const failed = this.testResults.tests.filter(t => t.status === 'failed').length;
    const errors = this.testResults.tests.filter(t => t.status === 'error').length;

    this.testResults.summary = {
      total: this.testResults.tests.length,
      passed,
      failed,
      errors,
      passRate: `${((passed / this.testResults.tests.length) * 100).toFixed(1)}%`
    };

    if (errors > 0 || failed > 0) {
      this.testResults.overall = 'failed';
    } else {
      this.testResults.overall = 'passed';
    }
  }

  async generateTestReport() {
    const reportDir = path.join(this.rootDir, 'reports');
    await fs.mkdir(reportDir, { recursive: true });

    const reportPath = path.join(reportDir, `dependency-fix-test-${new Date().toISOString().split('T')[0]}.json`);
    await fs.writeFile(reportPath, JSON.stringify(this.testResults, null, 2));

    TestLogger.info(`📊 Test report generated: ${reportPath}`);
  }
}

// CLI Interface
async function main() {
  const tester = new DependencyFixTester();
  await tester.runAllTests();
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(error => {
    console.error('Fatal test error:', error);
    process.exit(1);
  });
}

export default DependencyFixTester;
