#!/usr/bin/env node

/**
 * Install Hooks - Sets up automated dependency management integration points
 */

import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

class HookInstaller {
  constructor() {
    this.rootDir = path.resolve(__dirname, '../../..');
    this.packageFiles = [];
  }

  async install() {
    console.log('🔧 Installing dependency management hooks...');
    
    await this.findPackageFiles();
    await this.installPackageJsonHooks();
    await this.installGitHooks();
    await this.installDockerHooks();
    await this.installCIHooks();
    
    console.log('✅ Dependency management hooks installed successfully');
  }

  async findPackageFiles() {
    const packagePaths = [
      'bmad-agent/free-deep-research/package.json',
      'bmad-agent/deep_research_frontend/package.json'
    ];

    for (const packagePath of packagePaths) {
      const fullPath = path.join(this.rootDir, packagePath);
      try {
        await fs.access(fullPath);
        this.packageFiles.push(fullPath);
      } catch (error) {
        console.log(`⚠️  Package file not found: ${packagePath}`);
      }
    }
  }

  async installPackageJsonHooks() {
    for (const packagePath of this.packageFiles) {
      await this.addPackageJsonHooks(packagePath);
    }
  }

  async addPackageJsonHooks(packagePath) {
    try {
      const content = await fs.readFile(packagePath, 'utf8');
      const packageJson = JSON.parse(content);

      // Add dependency health check scripts
      if (!packageJson.scripts) {
        packageJson.scripts = {};
      }

      const dependencyScript = 'node ../../scripts/dependency-health-check.js';
      const dependencyFixScript = 'node ../../scripts/dependency-health-check.js --auto-fix';

      packageJson.scripts = {
        ...packageJson.scripts,
        'deps:check': dependencyScript,
        'deps:fix': dependencyFixScript,
        'deps:health': `${dependencyScript} --mode=conservative`,
        'deps:security': `${dependencyScript} --mode=security-only`,
        'preinstall': packageJson.scripts.preinstall 
          ? `${packageJson.scripts.preinstall} && ${dependencyScript}`
          : dependencyScript,
        'postinstall': packageJson.scripts.postinstall
          ? `${packageJson.scripts.postinstall} && ${dependencyScript} --no-health-check`
          : `${dependencyScript} --no-health-check`
      };

      // Write updated package.json
      const updatedContent = JSON.stringify(packageJson, null, 2) + '\n';
      await fs.writeFile(packagePath, updatedContent);
      
      console.log(`✅ Added hooks to ${path.relative(this.rootDir, packagePath)}`);
    } catch (error) {
      console.error(`❌ Failed to add hooks to ${packagePath}: ${error.message}`);
    }
  }

  async installGitHooks() {
    const gitHooksDir = path.join(this.rootDir, '.git/hooks');
    
    try {
      await fs.access(gitHooksDir);
    } catch (error) {
      console.log('⚠️  Git hooks directory not found, skipping git hooks');
      return;
    }

    // Pre-commit hook
    const preCommitHook = `#!/bin/bash
# Dependency Health Check - Pre-commit Hook

echo "🔍 Running dependency health check..."
node scripts/dependency-health-check.js --mode=conservative

if [ $? -ne 0 ]; then
    echo "❌ Dependency health check failed. Commit aborted."
    echo "💡 Run 'npm run deps:fix' to auto-resolve issues"
    exit 1
fi

echo "✅ Dependency health check passed"
`;

    const preCommitPath = path.join(gitHooksDir, 'pre-commit');
    await fs.writeFile(preCommitPath, preCommitHook);
    await fs.chmod(preCommitPath, 0o755);
    
    console.log('✅ Installed Git pre-commit hook');
  }

  async installDockerHooks() {
    const dockerFiles = [
      'docker/frontend/Dockerfile',
      'docker/frontend/Dockerfile.dev',
      'bmad-agent/free-deep-research/docker/Dockerfile'
    ];

    for (const dockerFile of dockerFiles) {
      const dockerPath = path.join(this.rootDir, dockerFile);
      
      try {
        await fs.access(dockerPath);
        await this.addDockerHealthCheck(dockerPath);
      } catch (error) {
        console.log(`⚠️  Docker file not found: ${dockerFile}`);
      }
    }
  }

  async addDockerHealthCheck(dockerPath) {
    try {
      let content = await fs.readFile(dockerPath, 'utf8');
      
      // Check if health check is already added
      if (content.includes('dependency-health-check')) {
        console.log(`✅ Docker health check already present in ${path.relative(this.rootDir, dockerPath)}`);
        return;
      }

      // Add health check before the final CMD/ENTRYPOINT
      const healthCheckInstruction = `
# Add dependency health check
COPY scripts/dependency-health-check.js /app/scripts/
RUN node /app/scripts/dependency-health-check.js --mode=conservative || echo "Warning: Dependency health check failed"
`;

      // Insert before the last CMD or ENTRYPOINT
      const lines = content.split('\n');
      let insertIndex = lines.length;
      
      for (let i = lines.length - 1; i >= 0; i--) {
        if (lines[i].trim().startsWith('CMD') || lines[i].trim().startsWith('ENTRYPOINT')) {
          insertIndex = i;
          break;
        }
      }

      lines.splice(insertIndex, 0, healthCheckInstruction);
      content = lines.join('\n');

      await fs.writeFile(dockerPath, content);
      console.log(`✅ Added Docker health check to ${path.relative(this.rootDir, dockerPath)}`);
    } catch (error) {
      console.error(`❌ Failed to add Docker health check to ${dockerPath}: ${error.message}`);
    }
  }

  async installCIHooks() {
    const githubWorkflowsDir = path.join(this.rootDir, '.github/workflows');
    
    try {
      await fs.mkdir(githubWorkflowsDir, { recursive: true });
    } catch (error) {
      // Directory might already exist
    }

    const workflowContent = `name: Dependency Health Check

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]
  schedule:
    # Run daily at 2 AM UTC
    - cron: '0 2 * * *'

jobs:
  dependency-health:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '18'
        cache: 'npm'
        cache-dependency-path: |
          bmad-agent/free-deep-research/package-lock.json
          bmad-agent/deep_research_frontend/package-lock.json
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        components: rustfmt, clippy
    
    - name: Install cargo-audit
      run: cargo install cargo-audit
    
    - name: Run Dependency Health Check
      run: |
        node scripts/dependency-health-check.js --mode=conservative
    
    - name: Run Security Audit
      run: |
        node scripts/dependency-health-check.js --mode=security-only
    
    - name: Upload Health Report
      uses: actions/upload-artifact@v4
      if: always()
      with:
        name: dependency-health-report
        path: reports/
        retention-days: 30
    
    - name: Comment PR with Health Status
      if: github.event_name == 'pull_request'
      uses: actions/github-script@v7
      with:
        script: |
          const fs = require('fs');
          const path = require('path');
          
          try {
            const reportFiles = fs.readdirSync('reports/');
            const latestReport = reportFiles
              .filter(f => f.startsWith('health-report-'))
              .sort()
              .pop();
            
            if (latestReport) {
              const report = JSON.parse(fs.readFileSync(path.join('reports', latestReport), 'utf8'));
              
              const statusEmoji = {
                'healthy': '✅',
                'warning': '⚠️',
                'critical': '❌'
              };
              
              const comment = \`## 🏥 Dependency Health Check
              
              **Overall Status:** \${statusEmoji[report.overall]} \${report.overall.toUpperCase()}
              
              **Summary:**
              - 📦 Package files checked: \${report.checks.filter(c => c.type === 'package-file').length}
              - 🔒 Security vulnerabilities: \${report.vulnerabilities.length}
              - 💡 Recommendations: \${report.recommendations.length}
              
              \${report.recommendations.length > 0 ? '**Recommendations:**\\n' + report.recommendations.map(r => \`- \${r}\`).join('\\n') : ''}
              \`;
              
              github.rest.issues.createComment({
                issue_number: context.issue.number,
                owner: context.repo.owner,
                repo: context.repo.repo,
                body: comment
              });
            }
          } catch (error) {
            console.log('Could not post health check comment:', error.message);
          }
`;

    const workflowPath = path.join(githubWorkflowsDir, 'dependency-health.yml');
    await fs.writeFile(workflowPath, workflowContent);
    
    console.log('✅ Installed GitHub Actions workflow');
  }

  async uninstall() {
    console.log('🗑️  Uninstalling dependency management hooks...');
    
    // Remove scripts from package.json files
    for (const packagePath of this.packageFiles) {
      await this.removePackageJsonHooks(packagePath);
    }
    
    // Remove git hooks
    const preCommitPath = path.join(this.rootDir, '.git/hooks/pre-commit');
    try {
      await fs.unlink(preCommitPath);
      console.log('✅ Removed Git pre-commit hook');
    } catch (error) {
      console.log('⚠️  Git pre-commit hook not found');
    }
    
    console.log('✅ Dependency management hooks uninstalled');
  }

  async removePackageJsonHooks(packagePath) {
    try {
      const content = await fs.readFile(packagePath, 'utf8');
      const packageJson = JSON.parse(content);

      if (packageJson.scripts) {
        // Remove dependency-related scripts
        const scriptsToRemove = ['deps:check', 'deps:fix', 'deps:health', 'deps:security'];
        for (const script of scriptsToRemove) {
          delete packageJson.scripts[script];
        }

        // Clean up preinstall/postinstall hooks
        if (packageJson.scripts.preinstall?.includes('dependency-health-check')) {
          delete packageJson.scripts.preinstall;
        }
        if (packageJson.scripts.postinstall?.includes('dependency-health-check')) {
          delete packageJson.scripts.postinstall;
        }
      }

      const updatedContent = JSON.stringify(packageJson, null, 2) + '\n';
      await fs.writeFile(packagePath, updatedContent);
      
      console.log(`✅ Removed hooks from ${path.relative(this.rootDir, packagePath)}`);
    } catch (error) {
      console.error(`❌ Failed to remove hooks from ${packagePath}: ${error.message}`);
    }
  }
}

// CLI Interface
async function main() {
  const args = process.argv.slice(2);
  const installer = new HookInstaller();

  if (args.includes('--uninstall')) {
    await installer.uninstall();
  } else {
    await installer.install();
  }
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(error => {
    console.error('Fatal error:', error);
    process.exit(1);
  });
}

export default HookInstaller;
