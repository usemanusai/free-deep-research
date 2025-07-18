import { test, expect, Page } from '@playwright/test'

/**
 * Integration Tests for Complete Workflow Lifecycle
 * 
 * These tests verify the end-to-end functionality of research workflows,
 * from creation through execution to completion, including error scenarios
 * and edge cases.
 */

test.describe('Research Workflow Lifecycle', () => {
  let page: Page

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage
    await page.goto('/')
    
    // Wait for app to load
    await page.waitForSelector('[data-testid="dashboard"]', { timeout: 10000 })
  })

  test.describe('Workflow Creation', () => {
    test('should create a basic research workflow', async () => {
      // Navigate to research page
      await page.click('[data-testid="nav-research"]')
      await page.waitForSelector('[data-testid="research-page"]')

      // Click create workflow button
      await page.click('[data-testid="create-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-form"]')

      // Fill out workflow form
      await page.fill('[data-testid="workflow-name"]', 'Test AI Research Workflow')
      await page.fill('[data-testid="workflow-description"]', 'Testing AI research capabilities')
      await page.selectOption('[data-testid="workflow-methodology"]', 'hybrid')
      await page.fill('[data-testid="workflow-query"]', 'artificial intelligence machine learning 2024')

      // Configure parameters
      await page.fill('[data-testid="max-results"]', '50')
      await page.selectOption('[data-testid="search-depth"]', 'comprehensive')
      await page.check('[data-testid="include-academic"]')
      await page.check('[data-testid="include-news"]')
      await page.selectOption('[data-testid="date-range"]', '1y')

      // Submit form
      await page.click('[data-testid="submit-workflow"]')

      // Verify workflow was created
      await page.waitForSelector('[data-testid="workflow-created-success"]')
      await expect(page.locator('[data-testid="workflow-name-display"]')).toContainText('Test AI Research Workflow')
    })

    test('should validate required fields', async () => {
      await page.click('[data-testid="nav-research"]')
      await page.click('[data-testid="create-workflow-btn"]')

      // Try to submit without required fields
      await page.click('[data-testid="submit-workflow"]')

      // Should show validation errors
      await expect(page.locator('[data-testid="error-workflow-name"]')).toBeVisible()
      await expect(page.locator('[data-testid="error-workflow-query"]')).toBeVisible()
    })

    test('should handle special characters in workflow name', async () => {
      await page.click('[data-testid="nav-research"]')
      await page.click('[data-testid="create-workflow-btn"]')

      const specialName = 'Test Workflow: AI & ML Research (2024) - Phase #1'
      await page.fill('[data-testid="workflow-name"]', specialName)
      await page.fill('[data-testid="workflow-query"]', 'test query')
      await page.click('[data-testid="submit-workflow"]')

      await page.waitForSelector('[data-testid="workflow-created-success"]')
      await expect(page.locator('[data-testid="workflow-name-display"]')).toContainText(specialName)
    })
  })

  test.describe('Workflow Execution', () => {
    test('should start and monitor workflow execution', async () => {
      // Create a workflow first
      await createTestWorkflow(page, 'Execution Test Workflow')

      // Start the workflow
      await page.click('[data-testid="start-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-running"]')

      // Verify status changed to running
      await expect(page.locator('[data-testid="workflow-status"]')).toContainText('Running')

      // Monitor progress
      await page.waitForSelector('[data-testid="progress-indicator"]')
      const progressBar = page.locator('[data-testid="progress-bar"]')
      
      // Progress should be greater than 0
      await expect(progressBar).toHaveAttribute('aria-valuenow', /[1-9]\d*/)

      // Wait for completion or timeout
      await page.waitForSelector('[data-testid="workflow-completed"]', { timeout: 60000 })
      await expect(page.locator('[data-testid="workflow-status"]')).toContainText('Completed')
    })

    test('should pause and resume workflow', async () => {
      await createTestWorkflow(page, 'Pause Resume Test')
      
      // Start workflow
      await page.click('[data-testid="start-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-running"]')

      // Pause workflow
      await page.click('[data-testid="pause-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-paused"]')
      await expect(page.locator('[data-testid="workflow-status"]')).toContainText('Paused')

      // Resume workflow
      await page.click('[data-testid="resume-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-running"]')
      await expect(page.locator('[data-testid="workflow-status"]')).toContainText('Running')
    })

    test('should cancel workflow execution', async () => {
      await createTestWorkflow(page, 'Cancel Test Workflow')
      
      // Start workflow
      await page.click('[data-testid="start-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-running"]')

      // Cancel workflow
      await page.click('[data-testid="cancel-workflow-btn"]')
      
      // Confirm cancellation
      await page.click('[data-testid="confirm-cancel"]')
      
      await page.waitForSelector('[data-testid="workflow-cancelled"]')
      await expect(page.locator('[data-testid="workflow-status"]')).toContainText('Cancelled')
    })
  })

  test.describe('Workflow Results', () => {
    test('should display workflow results after completion', async () => {
      await createTestWorkflow(page, 'Results Test Workflow')
      
      // Start and wait for completion
      await page.click('[data-testid="start-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-completed"]', { timeout: 60000 })

      // View results
      await page.click('[data-testid="view-results-btn"]')
      await page.waitForSelector('[data-testid="workflow-results"]')

      // Verify results components are present
      await expect(page.locator('[data-testid="results-summary"]')).toBeVisible()
      await expect(page.locator('[data-testid="results-sources"]')).toBeVisible()
      await expect(page.locator('[data-testid="results-analysis"]')).toBeVisible()

      // Check for data
      const resultCount = await page.locator('[data-testid="result-count"]').textContent()
      expect(parseInt(resultCount || '0')).toBeGreaterThan(0)
    })

    test('should export workflow results', async () => {
      await createTestWorkflow(page, 'Export Test Workflow')
      
      // Complete workflow
      await page.click('[data-testid="start-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-completed"]', { timeout: 60000 })

      // Export results
      const downloadPromise = page.waitForEvent('download')
      await page.click('[data-testid="export-results-btn"]')
      const download = await downloadPromise

      // Verify download
      expect(download.suggestedFilename()).toMatch(/export.*\.json$/)
    })
  })

  test.describe('Error Handling', () => {
    test('should handle API key errors gracefully', async () => {
      // Create workflow without valid API keys
      await createTestWorkflow(page, 'API Error Test')
      
      // Start workflow (should fail due to missing API keys)
      await page.click('[data-testid="start-workflow-btn"]')
      
      // Should show error state
      await page.waitForSelector('[data-testid="workflow-error"]')
      await expect(page.locator('[data-testid="error-message"]')).toContainText('API key')
    })

    test('should handle network errors', async () => {
      // Simulate network failure
      await page.route('**/api/**', route => route.abort())
      
      await createTestWorkflow(page, 'Network Error Test')
      await page.click('[data-testid="start-workflow-btn"]')
      
      // Should show network error
      await page.waitForSelector('[data-testid="workflow-error"]')
      await expect(page.locator('[data-testid="error-message"]')).toContainText('network')
    })

    test('should retry failed operations', async () => {
      let requestCount = 0
      
      // Fail first 2 requests, succeed on 3rd
      await page.route('**/start_workflow', route => {
        requestCount++
        if (requestCount < 3) {
          route.abort()
        } else {
          route.continue()
        }
      })
      
      await createTestWorkflow(page, 'Retry Test Workflow')
      await page.click('[data-testid="start-workflow-btn"]')
      
      // Should eventually succeed after retries
      await page.waitForSelector('[data-testid="workflow-running"]', { timeout: 15000 })
    })
  })

  test.describe('Concurrent Workflows', () => {
    test('should handle multiple concurrent workflows', async () => {
      const workflowNames = ['Concurrent 1', 'Concurrent 2', 'Concurrent 3']
      
      // Create multiple workflows
      for (const name of workflowNames) {
        await createTestWorkflow(page, name)
        await page.click('[data-testid="back-to-list"]')
      }
      
      // Start all workflows
      const workflowRows = page.locator('[data-testid="workflow-row"]')
      const count = await workflowRows.count()
      
      for (let i = 0; i < count; i++) {
        await workflowRows.nth(i).locator('[data-testid="start-btn"]').click()
      }
      
      // Verify all are running
      for (let i = 0; i < count; i++) {
        await expect(
          workflowRows.nth(i).locator('[data-testid="status"]')
        ).toContainText('Running')
      }
    })

    test('should respect queue limits', async () => {
      // Create more workflows than queue limit
      for (let i = 1; i <= 10; i++) {
        await createTestWorkflow(page, `Queue Test ${i}`)
        await page.click('[data-testid="back-to-list"]')
      }
      
      // Start all workflows rapidly
      const startButtons = page.locator('[data-testid="start-btn"]')
      const count = await startButtons.count()
      
      for (let i = 0; i < count; i++) {
        await startButtons.nth(i).click()
      }
      
      // Some should be queued
      await expect(page.locator('[data-testid="status"]:has-text("Queued")')).toHaveCount(
        { min: 1 }
      )
    })
  })

  test.describe('Performance Tests', () => {
    test('should handle large result sets efficiently', async () => {
      await createTestWorkflow(page, 'Large Results Test', {
        maxResults: 1000,
        searchDepth: 'comprehensive'
      })
      
      const startTime = Date.now()
      
      await page.click('[data-testid="start-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-completed"]', { timeout: 120000 })
      
      const endTime = Date.now()
      const duration = endTime - startTime
      
      // Should complete within reasonable time (2 minutes)
      expect(duration).toBeLessThan(120000)
      
      // Results should load quickly
      await page.click('[data-testid="view-results-btn"]')
      await page.waitForSelector('[data-testid="workflow-results"]', { timeout: 5000 })
    })

    test('should maintain UI responsiveness during execution', async () => {
      await createTestWorkflow(page, 'Responsiveness Test')
      
      await page.click('[data-testid="start-workflow-btn"]')
      await page.waitForSelector('[data-testid="workflow-running"]')
      
      // UI should remain responsive
      const navigationStart = Date.now()
      await page.click('[data-testid="nav-dashboard"]')
      await page.waitForSelector('[data-testid="dashboard"]')
      const navigationTime = Date.now() - navigationStart
      
      // Navigation should be fast even with running workflow
      expect(navigationTime).toBeLessThan(2000)
    })
  })
})

// Helper function to create a test workflow
async function createTestWorkflow(
  page: Page, 
  name: string, 
  options: {
    maxResults?: number
    searchDepth?: string
    query?: string
  } = {}
) {
  const {
    maxResults = 50,
    searchDepth = 'basic',
    query = 'test research query'
  } = options

  await page.click('[data-testid="nav-research"]')
  await page.click('[data-testid="create-workflow-btn"]')
  
  await page.fill('[data-testid="workflow-name"]', name)
  await page.fill('[data-testid="workflow-description"]', `Test workflow: ${name}`)
  await page.selectOption('[data-testid="workflow-methodology"]', 'hybrid')
  await page.fill('[data-testid="workflow-query"]', query)
  
  await page.fill('[data-testid="max-results"]', maxResults.toString())
  await page.selectOption('[data-testid="search-depth"]', searchDepth)
  
  await page.click('[data-testid="submit-workflow"]')
  await page.waitForSelector('[data-testid="workflow-created-success"]')
}
