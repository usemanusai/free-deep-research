import { chromium, FullConfig } from '@playwright/test'
import path from 'path'
import fs from 'fs'

async function globalSetup(config: FullConfig) {
  console.log('🚀 Starting global setup for E2E tests...')

  // Create test data directory
  const testDataDir = path.join(__dirname, '../test-data')
  if (!fs.existsSync(testDataDir)) {
    fs.mkdirSync(testDataDir, { recursive: true })
  }

  // Create test database
  const testDbPath = path.join(testDataDir, 'test.db')
  if (fs.existsSync(testDbPath)) {
    fs.unlinkSync(testDbPath)
  }

  // Set environment variables for testing
  process.env.NODE_ENV = 'test'
  process.env.DATABASE_PATH = testDbPath
  process.env.MOCK_APIS = 'true'
  process.env.LOG_LEVEL = 'error'

  // Create a browser instance for authentication if needed
  const browser = await chromium.launch()
  const context = await browser.newContext()
  const page = await context.newPage()

  // Wait for the application to be ready
  try {
    await page.goto('http://localhost:1420', { waitUntil: 'networkidle' })
    await page.waitForSelector('[data-testid="app-ready"]', { timeout: 30000 })
    console.log('✅ Application is ready for testing')
  } catch (error) {
    console.warn('⚠️ Application readiness check failed, continuing anyway')
  }

  // Setup test data
  await setupTestData(page)

  await browser.close()
  console.log('✅ Global setup completed')
}

async function setupTestData(page: any) {
  console.log('📝 Setting up test data...')

  // Create test API keys
  const testApiKeys = [
    {
      name: 'Test OpenRouter Key',
      service: 'openrouter',
      key: 'test-openrouter-key-123',
      description: 'Test key for OpenRouter service'
    },
    {
      name: 'Test SerpApi Key',
      service: 'serpapi',
      key: 'test-serpapi-key-456',
      description: 'Test key for SerpApi service'
    }
  ]

  // Create test workflows
  const testWorkflows = [
    {
      name: 'Test Research Workflow',
      description: 'A test workflow for E2E testing',
      methodology: 'hybrid',
      query: 'Test research query'
    }
  ]

  // Store test data in localStorage for the application to use
  await page.evaluate((data) => {
    localStorage.setItem('test-api-keys', JSON.stringify(data.apiKeys))
    localStorage.setItem('test-workflows', JSON.stringify(data.workflows))
    localStorage.setItem('test-mode', 'true')
  }, {
    apiKeys: testApiKeys,
    workflows: testWorkflows
  })

  console.log('✅ Test data setup completed')
}

export default globalSetup
