import React, { useEffect, useState } from 'react'
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'
import { Toaster } from 'react-hot-toast'

// Components
import Layout from '@components/Layout'
import LoadingScreen from '@components/common/LoadingScreen'
import ErrorBoundary from '@components/common/ErrorBoundary'

// Pages
import Dashboard from '@components/dashboard/ExecutiveDashboard'
import ApiKeyManager from '@components/api-management/ApiKeyManager'
import ResearchWorkflow from '@components/research/ResearchWorkflow'
import MonitoringConsole from '@components/monitoring/RealTimeConsole'
import Analytics from '@components/analytics'
import Settings from '@components/settings/ConfigurationPanel'

// Hooks
import { useSystemHealth } from '@hooks/useSystemHealth'

// Types
interface AppState {
  isLoading: boolean
  isHealthy: boolean
  error: string | null
}

function App() {
  const [appState, setAppState] = useState<AppState>({
    isLoading: true,
    isHealthy: false,
    error: null,
  })

  const { data: healthStatus, isLoading: healthLoading, error: healthError } = useSystemHealth()

  useEffect(() => {
    const initializeApp = async () => {
      try {
        // Perform initial health check
        const healthCheck = await invoke<string>('health_check')
        console.log('Health check result:', healthCheck)
        
        setAppState({
          isLoading: false,
          isHealthy: true,
          error: null,
        })
      } catch (error) {
        console.error('App initialization failed:', error)
        setAppState({
          isLoading: false,
          isHealthy: false,
          error: error instanceof Error ? error.message : 'Unknown error occurred',
        })
      }
    }

    initializeApp()
  }, [])

  // Show loading screen during initialization
  if (appState.isLoading || healthLoading) {
    return <LoadingScreen message="Initializing Free Deep Research System..." />
  }

  // Show error screen if initialization failed
  if (appState.error || healthError) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="max-w-md w-full bg-white shadow-lg rounded-lg p-6">
          <div className="flex items-center justify-center w-12 h-12 mx-auto bg-red-100 rounded-full mb-4">
            <svg className="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
            </svg>
          </div>
          <h3 className="text-lg font-medium text-gray-900 text-center mb-2">
            Application Error
          </h3>
          <p className="text-sm text-gray-500 text-center mb-4">
            {appState.error || healthError?.message || 'Failed to initialize the application'}
          </p>
          <button
            onClick={() => window.location.reload()}
            className="w-full bg-primary-600 text-white py-2 px-4 rounded-md hover:bg-primary-700 transition-colors"
          >
            Retry
          </button>
        </div>
      </div>
    )
  }

  return (
    <ErrorBoundary>
      <Router>
        <div className="min-h-screen bg-gray-50">
          <Layout>
            <Routes>
              <Route path="/" element={<Navigate to="/dashboard" replace />} />
              <Route path="/dashboard" element={<Dashboard />} />
              <Route path="/api-keys" element={<ApiKeyManager />} />
              <Route path="/research" element={<ResearchWorkflow />} />
              <Route path="/monitoring" element={<MonitoringConsole />} />
              <Route path="/analytics" element={<Analytics />} />
              <Route path="/settings" element={<Settings />} />
              <Route path="*" element={<Navigate to="/dashboard" replace />} />
            </Routes>
          </Layout>
          
          {/* Toast notifications */}
          <Toaster
            position="top-right"
            toastOptions={{
              duration: 4000,
              style: {
                background: '#363636',
                color: '#fff',
              },
              success: {
                duration: 3000,
                iconTheme: {
                  primary: '#10B981',
                  secondary: '#fff',
                },
              },
              error: {
                duration: 5000,
                iconTheme: {
                  primary: '#EF4444',
                  secondary: '#fff',
                },
              },
            }}
          />
        </div>
      </Router>
    </ErrorBoundary>
  )
}

export default App
