import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { performanceMonitor, usePerformanceMonitoring } from '../performance'
import { renderHook, act } from '@testing-library/react'

// Mock Tauri invoke for performance logging
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue(undefined)
}))

describe('Performance Utilities - Stress Tests', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    performanceMonitor.clearMetrics()
  })

  afterEach(() => {
    performanceMonitor.clearMetrics()
  })

  describe('High Volume Measurements', () => {
    it('should handle 1000 concurrent measurements', () => {
      const measurementIds: string[] = []
      const startTime = performance.now()
      
      // Start 1000 measurements
      for (let i = 0; i < 1000; i++) {
        const id = performanceMonitor.startMeasurement(
          `test-measurement-${i}`,
          'component',
          { index: i }
        )
        measurementIds.push(id)
      }
      
      const measurementStartTime = performance.now() - startTime
      expect(measurementStartTime).toBeLessThan(100) // Should complete in under 100ms
      
      // End all measurements
      const endStartTime = performance.now()
      const metrics = measurementIds.map(id => 
        performanceMonitor.endMeasurement(id)
      ).filter(Boolean)
      
      const measurementEndTime = performance.now() - endStartTime
      expect(measurementEndTime).toBeLessThan(200) // Should complete in under 200ms
      expect(metrics).toHaveLength(1000)
    })

    it('should handle rapid successive measurements', () => {
      const measurements = []
      const startTime = performance.now()
      
      // Rapid fire measurements
      for (let i = 0; i < 100; i++) {
        const id = performanceMonitor.startMeasurement(`rapid-${i}`, 'api')
        // Immediately end measurement
        const metric = performanceMonitor.endMeasurement(id)
        measurements.push(metric)
      }
      
      const totalTime = performance.now() - startTime
      expect(totalTime).toBeLessThan(50) // Should be very fast
      expect(measurements).toHaveLength(100)
      expect(measurements.every(m => m !== null)).toBe(true)
    })

    it('should maintain performance with large metric history', () => {
      // Generate a large number of completed metrics
      for (let i = 0; i < 5000; i++) {
        const id = performanceMonitor.startMeasurement(`history-${i}`, 'component')
        performanceMonitor.endMeasurement(id)
      }
      
      // Test that new measurements are still fast
      const startTime = performance.now()
      const id = performanceMonitor.startMeasurement('new-measurement', 'api')
      const metric = performanceMonitor.endMeasurement(id)
      const measurementTime = performance.now() - startTime
      
      expect(measurementTime).toBeLessThan(10) // Should still be fast
      expect(metric).not.toBeNull()
    })
  })

  describe('Memory Usage Tests', () => {
    it('should not leak memory with many measurements', () => {
      const initialMemory = process.memoryUsage().heapUsed
      
      // Create and complete many measurements
      for (let batch = 0; batch < 10; batch++) {
        const ids = []
        
        // Start batch of measurements
        for (let i = 0; i < 1000; i++) {
          ids.push(performanceMonitor.startMeasurement(`batch-${batch}-${i}`, 'component'))
        }
        
        // End batch of measurements
        ids.forEach(id => performanceMonitor.endMeasurement(id))
        
        // Force garbage collection if available
        if (global.gc) {
          global.gc()
        }
      }
      
      const finalMemory = process.memoryUsage().heapUsed
      const memoryIncrease = finalMemory - initialMemory
      
      // Memory increase should be reasonable (less than 50MB)
      expect(memoryIncrease).toBeLessThan(50 * 1024 * 1024)
    })

    it('should handle metric cleanup efficiently', () => {
      // Fill up with metrics
      for (let i = 0; i < 1000; i++) {
        const id = performanceMonitor.startMeasurement(`cleanup-${i}`, 'component')
        performanceMonitor.endMeasurement(id)
      }
      
      const startTime = performance.now()
      performanceMonitor.clearMetrics()
      const clearTime = performance.now() - startTime
      
      expect(clearTime).toBeLessThan(10) // Cleanup should be fast
      
      // Verify metrics are actually cleared
      const report = performanceMonitor.getReport()
      expect(report.metrics).toHaveLength(0)
    })
  })

  describe('Concurrent Access Tests', () => {
    it('should handle concurrent measurements safely', async () => {
      const promises = []
      const results = []
      
      // Create 50 concurrent measurement operations
      for (let i = 0; i < 50; i++) {
        promises.push(
          new Promise<void>((resolve) => {
            setTimeout(() => {
              const id = performanceMonitor.startMeasurement(`concurrent-${i}`, 'api')
              setTimeout(() => {
                const metric = performanceMonitor.endMeasurement(id)
                results.push(metric)
                resolve()
              }, Math.random() * 10) // Random delay 0-10ms
            }, Math.random() * 5) // Random start delay 0-5ms
          })
        )
      }
      
      await Promise.all(promises)
      
      expect(results).toHaveLength(50)
      expect(results.every(r => r !== null)).toBe(true)
      
      // Verify all measurements have unique IDs
      const ids = results.map(r => r!.id)
      const uniqueIds = new Set(ids)
      expect(uniqueIds.size).toBe(50)
    })

    it('should handle concurrent report generation', async () => {
      // Start some measurements
      for (let i = 0; i < 100; i++) {
        const id = performanceMonitor.startMeasurement(`report-test-${i}`, 'component')
        performanceMonitor.endMeasurement(id)
      }
      
      // Generate multiple reports concurrently
      const reportPromises = Array.from({ length: 10 }, () => 
        Promise.resolve(performanceMonitor.getReport())
      )
      
      const reports = await Promise.all(reportPromises)
      
      // All reports should be identical
      const firstReport = reports[0]
      reports.forEach(report => {
        expect(report.metrics).toHaveLength(firstReport.metrics.length)
        expect(report.overall_score).toBe(firstReport.overall_score)
      })
    })
  })

  describe('Performance Budget Stress Tests', () => {
    it('should handle budget violations efficiently', () => {
      // Set strict budget
      performanceMonitor.updateBudget({
        component_render: 1, // Very strict 1ms budget
        api_calls: 10
      })
      
      const startTime = performance.now()
      
      // Create measurements that will violate budget
      for (let i = 0; i < 500; i++) {
        const id = performanceMonitor.startMeasurement(`budget-violation-${i}`, 'component')
        // Simulate slow operation
        const endTime = performance.now() + 5 // 5ms delay
        while (performance.now() < endTime) {
          // Busy wait
        }
        performanceMonitor.endMeasurement(id)
      }
      
      const totalTime = performance.now() - startTime
      const report = performanceMonitor.getReport()
      
      // Should still complete in reasonable time despite violations
      expect(totalTime).toBeLessThan(5000) // 5 seconds max
      expect(report.budget_violations.length).toBeGreaterThan(0)
      expect(report.overall_score).toBeLessThan(50) // Should reflect poor performance
    })

    it('should generate recommendations under load', () => {
      // Create various types of slow measurements
      const measurementTypes = ['api', 'component', 'render', 'navigation', 'user-interaction']
      
      measurementTypes.forEach(type => {
        for (let i = 0; i < 20; i++) {
          const id = performanceMonitor.startMeasurement(`slow-${type}-${i}`, type as any)
          // Simulate slow operation
          const endTime = performance.now() + 100 // 100ms delay
          while (performance.now() < endTime) {
            // Busy wait
          }
          performanceMonitor.endMeasurement(id)
        }
      })
      
      const startTime = performance.now()
      const report = performanceMonitor.getReport()
      const reportTime = performance.now() - startTime
      
      expect(reportTime).toBeLessThan(100) // Report generation should be fast
      expect(report.recommendations.length).toBeGreaterThan(0)
      expect(report.budget_violations.length).toBeGreaterThan(0)
    })
  })

  describe('Hook Performance Tests', () => {
    it('should handle rapid hook usage', () => {
      const { result } = renderHook(() => usePerformanceMonitoring())
      const measurements = []
      
      act(() => {
        // Start many measurements rapidly
        for (let i = 0; i < 100; i++) {
          const id = result.current.startMeasurement(`hook-test-${i}`, 'component')
          measurements.push(id)
        }
        
        // End them all
        measurements.forEach(id => {
          result.current.endMeasurement(id)
        })
      })
      
      const report = result.current.getReport()
      expect(report.metrics).toHaveLength(100)
    })

    it('should maintain performance with frequent updates', () => {
      const { result } = renderHook(() => usePerformanceMonitoring())
      
      const startTime = performance.now()
      
      act(() => {
        // Simulate frequent component updates
        for (let i = 0; i < 50; i++) {
          const id = result.current.startMeasurement(`update-${i}`, 'component')
          result.current.endMeasurement(id)
          
          // Update budget frequently
          result.current.updateBudget({
            component_render: 16 + (i % 10)
          })
        }
      })
      
      const totalTime = performance.now() - startTime
      expect(totalTime).toBeLessThan(100) // Should complete quickly
    })
  })

  describe('Edge Cases and Error Handling', () => {
    it('should handle invalid measurement IDs gracefully', () => {
      const result = performanceMonitor.endMeasurement('non-existent-id')
      expect(result).toBeNull()
      
      // Should not throw or cause issues
      expect(() => {
        for (let i = 0; i < 100; i++) {
          performanceMonitor.endMeasurement(`invalid-${i}`)
        }
      }).not.toThrow()
    })

    it('should handle extremely long measurement names', () => {
      const longName = 'a'.repeat(10000) // 10KB string
      
      const startTime = performance.now()
      const id = performanceMonitor.startMeasurement(longName, 'component')
      const metric = performanceMonitor.endMeasurement(id)
      const totalTime = performance.now() - startTime
      
      expect(totalTime).toBeLessThan(50) // Should still be fast
      expect(metric).not.toBeNull()
      expect(metric!.name).toBe(longName)
    })

    it('should handle rapid budget updates', () => {
      const startTime = performance.now()
      
      // Update budget rapidly
      for (let i = 0; i < 1000; i++) {
        performanceMonitor.updateBudget({
          api_calls: 1000 + i,
          component_render: 16 + (i % 100)
        })
      }
      
      const updateTime = performance.now() - startTime
      expect(updateTime).toBeLessThan(100) // Should be fast
      
      // Verify final budget is applied
      const id = performanceMonitor.startMeasurement('budget-test', 'api')
      performanceMonitor.endMeasurement(id)
      const report = performanceMonitor.getReport()
      
      expect(report).toBeDefined()
    })

    it('should handle measurements with extreme durations', () => {
      // Test with very short duration
      const shortId = performanceMonitor.startMeasurement('short-test', 'component')
      const shortMetric = performanceMonitor.endMeasurement(shortId)
      
      expect(shortMetric).not.toBeNull()
      expect(shortMetric!.duration).toBeGreaterThanOrEqual(0)
      
      // Test with simulated long duration
      const longId = performanceMonitor.startMeasurement('long-test', 'component')
      // Simulate very long operation
      const endTime = performance.now() + 1000 // 1 second
      while (performance.now() < endTime) {
        // Busy wait
      }
      const longMetric = performanceMonitor.endMeasurement(longId)
      
      expect(longMetric).not.toBeNull()
      expect(longMetric!.duration).toBeGreaterThan(900) // Should be close to 1000ms
    })
  })

  describe('Performance Regression Tests', () => {
    it('should maintain consistent performance over time', () => {
      const timings = []
      
      // Run the same operation multiple times
      for (let iteration = 0; iteration < 10; iteration++) {
        const startTime = performance.now()
        
        // Standard operation: 100 measurements
        for (let i = 0; i < 100; i++) {
          const id = performanceMonitor.startMeasurement(`regression-${iteration}-${i}`, 'component')
          performanceMonitor.endMeasurement(id)
        }
        
        const iterationTime = performance.now() - startTime
        timings.push(iterationTime)
      }
      
      // Calculate statistics
      const avgTime = timings.reduce((a, b) => a + b, 0) / timings.length
      const maxTime = Math.max(...timings)
      const minTime = Math.min(...timings)
      
      // Performance should be consistent (max shouldn't be more than 3x min)
      expect(maxTime / minTime).toBeLessThan(3)
      expect(avgTime).toBeLessThan(100) // Average should be under 100ms
    })
  })
})
