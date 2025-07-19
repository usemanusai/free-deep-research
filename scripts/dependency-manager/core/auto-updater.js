#!/usr/bin/env node

/**
 * AutoUpdater - Safely updates package files with backup mechanisms
 */

import fs from 'fs/promises';
import path from 'path';
import { execSync } from 'child_process';

class AutoUpdater {
  constructor(config, logger) {
    this.config = config;
    this.logger = logger;
    this.backups = new Map();
  }

  async createBackup(filePath) {
    try {
      const content = await fs.readFile(filePath, 'utf8');
      const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
      const backupPath = path.join(
        this.config.backupDir,
        `${path.basename(filePath)}.${timestamp}.backup`
      );

      await fs.mkdir(this.config.backupDir, { recursive: true });
      await fs.writeFile(backupPath, content);
      
      this.backups.set(filePath, backupPath);
      this.logger.debug(`Created backup: ${backupPath}`);
      
      return backupPath;
    } catch (error) {
      throw new Error(`Failed to create backup for ${filePath}: ${error.message}`);
    }
  }

  async restoreBackup(filePath) {
    const backupPath = this.backups.get(filePath);
    if (!backupPath) {
      throw new Error(`No backup found for ${filePath}`);
    }

    try {
      const content = await fs.readFile(backupPath, 'utf8');
      await fs.writeFile(filePath, content);
      this.logger.success(`Restored backup for ${filePath}`);
    } catch (error) {
      throw new Error(`Failed to restore backup: ${error.message}`);
    }
  }

  async updatePackageJson(filePath, updates) {
    await this.createBackup(filePath);

    try {
      const content = await fs.readFile(filePath, 'utf8');
      const packageJson = JSON.parse(content);

      // Apply updates
      for (const update of updates) {
        const { section, package: pkg, newVersion, reason } = update;
        
        if (packageJson[section] && packageJson[section][pkg]) {
          const oldVersion = packageJson[section][pkg];
          packageJson[section][pkg] = newVersion;
          
          this.logger.info(`Updated ${pkg}: ${oldVersion} → ${newVersion} (${reason})`);
        }
      }

      // Write updated content with proper formatting
      const updatedContent = JSON.stringify(packageJson, null, 2) + '\n';
      await fs.writeFile(filePath, updatedContent);

      this.logger.success(`Updated ${filePath}`);
      return true;
    } catch (error) {
      this.logger.error(`Failed to update ${filePath}: ${error.message}`);
      await this.restoreBackup(filePath);
      return false;
    }
  }

  async updateCargoToml(filePath, updates) {
    await this.createBackup(filePath);

    try {
      let content = await fs.readFile(filePath, 'utf8');

      for (const update of updates) {
        const { package: pkg, newVersion, reason } = update;
        
        // Simple regex replacement for Cargo.toml
        const regex = new RegExp(`(${pkg}\\s*=\\s*)"([^"]+)"`, 'g');
        const oldMatch = content.match(regex);
        
        if (oldMatch) {
          content = content.replace(regex, `$1"${newVersion}"`);
          this.logger.info(`Updated ${pkg}: ${oldMatch[0]} → "${newVersion}" (${reason})`);
        }
      }

      await fs.writeFile(filePath, content);
      this.logger.success(`Updated ${filePath}`);
      return true;
    } catch (error) {
      this.logger.error(`Failed to update ${filePath}: ${error.message}`);
      await this.restoreBackup(filePath);
      return false;
    }
  }

  async updateLockFiles(packagePath) {
    const dir = path.dirname(packagePath);
    const basename = path.basename(packagePath);

    try {
      if (basename === 'package.json') {
        this.logger.info('Updating package-lock.json...');
        
        // Remove existing lock file to force regeneration
        const lockPath = path.join(dir, 'package-lock.json');
        try {
          await fs.unlink(lockPath);
        } catch (error) {
          // Lock file might not exist, that's okay
        }

        // Run npm install to regenerate lock file
        execSync('npm install', { 
          cwd: dir, 
          stdio: 'pipe',
          timeout: this.config.timeout 
        });
        
        this.logger.success('Updated package-lock.json');
      } else if (basename === 'Cargo.toml') {
        this.logger.info('Updating Cargo.lock...');
        
        execSync('cargo update', { 
          cwd: dir, 
          stdio: 'pipe',
          timeout: this.config.timeout 
        });
        
        this.logger.success('Updated Cargo.lock');
      }
    } catch (error) {
      this.logger.warning(`Failed to update lock files: ${error.message}`);
    }
  }

  async validateUpdate(packagePath) {
    const dir = path.dirname(packagePath);
    const basename = path.basename(packagePath);

    try {
      if (basename === 'package.json') {
        // Test npm install
        execSync('npm install --dry-run', { 
          cwd: dir, 
          stdio: 'pipe',
          timeout: this.config.timeout 
        });
        
        this.logger.success('npm install validation passed');
        return true;
      } else if (basename === 'Cargo.toml') {
        // Test cargo check
        execSync('cargo check', { 
          cwd: dir, 
          stdio: 'pipe',
          timeout: this.config.timeout 
        });
        
        this.logger.success('cargo check validation passed');
        return true;
      }
    } catch (error) {
      this.logger.error(`Validation failed: ${error.message}`);
      return false;
    }

    return true;
  }

  async applyUpdates(packageFile, conflicts, resolver) {
    const updates = [];

    for (const conflict of conflicts) {
      if (conflict.file !== packageFile.path) continue;

      const info = await resolver.checkPackageVersion(
        conflict.package, 
        conflict.requested, 
        packageFile.type
      );

      if (!info) continue;

      let newVersion;
      let reason;

      switch (this.config.mode) {
        case 'conservative':
          // Only patch and minor updates
          newVersion = this.findConservativeUpdate(conflict.requested, info.versions);
          reason = 'conservative update';
          break;
        
        case 'aggressive':
          // Allow major updates
          newVersion = info.latest;
          reason = 'latest version';
          break;
        
        case 'security-only':
          // Only security updates (would need vulnerability database)
          newVersion = this.findSecurityUpdate(conflict.requested, info.versions);
          reason = 'security update';
          break;
        
        default:
          newVersion = info.latest;
          reason = 'available version';
      }

      if (newVersion && newVersion !== conflict.requested) {
        updates.push({
          section: conflict.section || 'dependencies',
          package: conflict.package,
          newVersion: packageFile.type === 'npm' ? `^${newVersion}` : newVersion,
          reason
        });
      }
    }

    if (updates.length === 0) {
      this.logger.info(`No updates needed for ${packageFile.path}`);
      return true;
    }

    // Apply updates based on file type
    let success = false;
    if (packageFile.type === 'npm') {
      success = await this.updatePackageJson(packageFile.path, updates);
    } else if (packageFile.type === 'cargo') {
      success = await this.updateCargoToml(packageFile.path, updates);
    }

    if (success) {
      // Update lock files
      await this.updateLockFiles(packageFile.path);
      
      // Validate the update
      const isValid = await this.validateUpdate(packageFile.path);
      if (!isValid) {
        this.logger.warning('Validation failed, restoring backup');
        await this.restoreBackup(packageFile.path);
        return false;
      }
    }

    return success;
  }

  findConservativeUpdate(current, available) {
    // Simple conservative update logic
    const currentMajor = current.replace(/[\^~]/, '').split('.')[0];
    return available.find(v => v.startsWith(currentMajor)) || current;
  }

  findSecurityUpdate(current, available) {
    // Placeholder for security update logic
    // In production, integrate with npm audit or similar
    return this.findConservativeUpdate(current, available);
  }

  async cleanupBackups(olderThanDays = 7) {
    try {
      const files = await fs.readdir(this.config.backupDir);
      const cutoffDate = new Date();
      cutoffDate.setDate(cutoffDate.getDate() - olderThanDays);

      for (const file of files) {
        const filePath = path.join(this.config.backupDir, file);
        const stats = await fs.stat(filePath);
        
        if (stats.mtime < cutoffDate) {
          await fs.unlink(filePath);
          this.logger.debug(`Cleaned up old backup: ${file}`);
        }
      }
    } catch (error) {
      this.logger.warning(`Failed to cleanup backups: ${error.message}`);
    }
  }
}

export default AutoUpdater;
