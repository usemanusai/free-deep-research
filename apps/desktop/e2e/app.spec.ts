import { test, expect } from '@playwright/test'

test.describe('Free Deep Research System', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
    // Wait for the app to load
    await page.waitForSelector('[data-testid="app-ready"]', { timeout: 10000 })
  })

  test('should load the application successfully', async ({ page }) => {
    // Check that the main title is visible
    await expect(page.locator('h1')).toContainText('Free Deep Research System')
    
    // Check that navigation is present
    await expect(page.locator('nav')).toBeVisible()
    
    // Check that the dashboard is the default view
    await expect(page.locator('[data-testid="dashboard"]')).toBeVisible()
  })

  test('should navigate between main sections', async ({ page }) => {
    // Test navigation to API Keys
    await page.click('text=API Keys')
    await expect(page.locator('[data-testid="api-keys-page"]')).toBeVisible()
    
    // Test navigation to Research
    await page.click('text=Research')
    await expect(page.locator('[data-testid="research-page"]')).toBeVisible()
    
    // Test navigation to Workflows
    await page.click('text=Workflows')
    await expect(page.locator('[data-testid="workflows-page"]')).toBeVisible()
    
    // Test navigation to Settings
    await page.click('text=Settings')
    await expect(page.locator('[data-testid="settings-page"]')).toBeVisible()
    
    // Return to Dashboard
    await page.click('text=Dashboard')
    await expect(page.locator('[data-testid="dashboard"]')).toBeVisible()
  })

  test('should display system health status', async ({ page }) => {
    // Check that health indicators are present
    await expect(page.locator('[data-testid="health-status"]')).toBeVisible()
    
    // Check for service status indicators
    await expect(page.locator('[data-testid="service-health"]')).toBeVisible()
    
    // Check for system metrics
    await expect(page.locator('[data-testid="system-metrics"]')).toBeVisible()
  })

  test('should handle API key management', async ({ page }) => {
    // Navigate to API Keys page
    await page.click('text=API Keys')
    
    // Check that the API keys table is visible
    await expect(page.locator('[data-testid="api-keys-table"]')).toBeVisible()
    
    // Test adding a new API key
    await page.click('[data-testid="add-api-key-button"]')
    await expect(page.locator('[data-testid="add-api-key-modal"]')).toBeVisible()
    
    // Fill in the form
    await page.fill('[data-testid="api-key-name"]', 'Test API Key')
    await page.selectOption('[data-testid="api-key-service"]', 'openrouter')
    await page.fill('[data-testid="api-key-value"]', 'test-key-123')
    
    // Submit the form
    await page.click('[data-testid="save-api-key"]')
    
    // Check that the key was added
    await expect(page.locator('text=Test API Key')).toBeVisible()
  })

  test('should handle research workflow creation', async ({ page }) => {
    // Navigate to Research page
    await page.click('text=Research')
    
    // Start a new research workflow
    await page.click('[data-testid="new-workflow-button"]')
    
    // Fill in workflow details
    await page.fill('[data-testid="workflow-name"]', 'Test Research Workflow')
    await page.fill('[data-testid="workflow-query"]', 'AI in healthcare research')
    await page.selectOption('[data-testid="workflow-methodology"]', 'hybrid')
    
    // Start the workflow
    await page.click('[data-testid="start-workflow"]')
    
    // Check that the workflow appears in the list
    await expect(page.locator('text=Test Research Workflow')).toBeVisible()
    
    // Check that the workflow status is displayed
    await expect(page.locator('[data-testid="workflow-status"]')).toBeVisible()
  })

  test('should display real-time monitoring data', async ({ page }) => {
    // Navigate to Monitoring page
    await page.click('text=Monitoring')
    
    // Check that monitoring console is visible
    await expect(page.locator('[data-testid="monitoring-console"]')).toBeVisible()
    
    // Check for system metrics
    await expect(page.locator('[data-testid="cpu-usage"]')).toBeVisible()
    await expect(page.locator('[data-testid="memory-usage"]')).toBeVisible()
    
    // Check for service health indicators
    await expect(page.locator('[data-testid="service-health-indicators"]')).toBeVisible()
    
    // Test auto-refresh toggle
    const autoRefreshToggle = page.locator('[data-testid="auto-refresh-toggle"]')
    await expect(autoRefreshToggle).toBeVisible()
    await autoRefreshToggle.click()
  })

  test('should handle error states gracefully', async ({ page }) => {
    // Test network error handling
    await page.route('**/api/**', route => route.abort())
    
    // Navigate to a page that requires API calls
    await page.click('text=API Keys')
    
    // Check that error message is displayed
    await expect(page.locator('[data-testid="error-message"]')).toBeVisible()
    
    // Check that retry button is available
    await expect(page.locator('[data-testid="retry-button"]')).toBeVisible()
  })

  test('should be responsive on different screen sizes', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 })
    await page.reload()
    
    // Check that mobile navigation is working
    await expect(page.locator('[data-testid="mobile-menu-button"]')).toBeVisible()
    
    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 })
    await page.reload()
    
    // Check that layout adapts
    await expect(page.locator('[data-testid="dashboard"]')).toBeVisible()
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 })
    await page.reload()
    
    // Check that full navigation is visible
    await expect(page.locator('nav')).toBeVisible()
  })

  test('should persist user preferences', async ({ page }) => {
    // Navigate to Settings
    await page.click('text=Settings')
    
    // Change a setting
    await page.check('[data-testid="dark-mode-toggle"]')
    
    // Reload the page
    await page.reload()
    
    // Check that the setting persisted
    await page.click('text=Settings')
    await expect(page.locator('[data-testid="dark-mode-toggle"]')).toBeChecked()
  })
})
