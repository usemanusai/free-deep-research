import { FullConfig } from '@playwright/test'
import path from 'path'
import fs from 'fs'

async function globalTeardown(config: FullConfig) {
  console.log('🧹 Starting global teardown for E2E tests...')

  // Clean up test data directory
  const testDataDir = path.join(__dirname, '../test-data')
  if (fs.existsSync(testDataDir)) {
    try {
      fs.rmSync(testDataDir, { recursive: true, force: true })
      console.log('✅ Test data directory cleaned up')
    } catch (error) {
      console.warn('⚠️ Failed to clean up test data directory:', error)
    }
  }

  // Clean up test artifacts
  const testArtifacts = [
    path.join(__dirname, '../test-results'),
    path.join(__dirname, '../playwright-report'),
    path.join(__dirname, '../coverage')
  ]

  for (const artifactPath of testArtifacts) {
    if (fs.existsSync(artifactPath)) {
      try {
        // Keep the directories but clean old files
        const files = fs.readdirSync(artifactPath)
        for (const file of files) {
          const filePath = path.join(artifactPath, file)
          const stats = fs.statSync(filePath)
          
          // Remove files older than 7 days
          const sevenDaysAgo = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000)
          if (stats.mtime < sevenDaysAgo) {
            fs.rmSync(filePath, { recursive: true, force: true })
          }
        }
      } catch (error) {
        console.warn(`⚠️ Failed to clean up ${artifactPath}:`, error)
      }
    }
  }

  // Reset environment variables
  delete process.env.DATABASE_PATH
  delete process.env.MOCK_APIS
  delete process.env.LOG_LEVEL

  console.log('✅ Global teardown completed')
}

export default globalTeardown
