import React from 'react'
import { invoke } from '@tauri-apps/api/core'

// Configuration bridge service for connecting frontend and backend
export class ConfigBridge {
  private static instance: ConfigBridge
  private configCache: Map<string, any> = new Map()
  private listeners: Map<string, Set<(value: any) => void>> = new Map()

  static getInstance(): ConfigBridge {
    if (!ConfigBridge.instance) {
      ConfigBridge.instance = new ConfigBridge()
    }
    return ConfigBridge.instance
  }

  // Get configuration value with caching
  async getConfig<T>(key: string, defaultValue?: T): Promise<T> {
    try {
      // Check cache first
      if (this.configCache.has(key)) {
        return this.configCache.get(key)
      }

      // Fetch from backend
      const config = await invoke<any>('get_configuration')
      const value = this.getNestedValue(config, key) ?? defaultValue

      // Cache the value
      this.configCache.set(key, value)
      
      return value
    } catch (error) {
      console.error(`Failed to get config for key: ${key}`, error)
      return defaultValue as T
    }
  }

  // Set configuration value
  async setConfig(key: string, value: any): Promise<void> {
    try {
      // Get current config
      const currentConfig = await invoke<any>('get_configuration')
      
      // Update the nested value
      const updatedConfig = this.setNestedValue(currentConfig, key, value)
      
      // Save to backend
      await invoke('update_configuration', { config: updatedConfig })
      
      // Update cache
      this.configCache.set(key, value)
      
      // Notify listeners
      this.notifyListeners(key, value)
    } catch (error) {
      console.error(`Failed to set config for key: ${key}`, error)
      throw error
    }
  }

  // Subscribe to configuration changes
  subscribe(key: string, callback: (value: any) => void): () => void {
    if (!this.listeners.has(key)) {
      this.listeners.set(key, new Set())
    }
    
    this.listeners.get(key)!.add(callback)
    
    // Return unsubscribe function
    return () => {
      const keyListeners = this.listeners.get(key)
      if (keyListeners) {
        keyListeners.delete(callback)
        if (keyListeners.size === 0) {
          this.listeners.delete(key)
        }
      }
    }
  }

  // Clear cache for a specific key or all keys
  clearCache(key?: string): void {
    if (key) {
      this.configCache.delete(key)
    } else {
      this.configCache.clear()
    }
  }

  // Batch get multiple configuration values
  async getBatchConfig(keys: string[]): Promise<Record<string, any>> {
    try {
      const config = await invoke<any>('get_configuration')
      const result: Record<string, any> = {}
      
      for (const key of keys) {
        result[key] = this.getNestedValue(config, key)
        this.configCache.set(key, result[key])
      }
      
      return result
    } catch (error) {
      console.error('Failed to get batch config', error)
      throw error
    }
  }

  // Batch set multiple configuration values
  async setBatchConfig(updates: Record<string, any>): Promise<void> {
    try {
      const currentConfig = await invoke<any>('get_configuration')
      let updatedConfig = { ...currentConfig }
      
      // Apply all updates
      for (const [key, value] of Object.entries(updates)) {
        updatedConfig = this.setNestedValue(updatedConfig, key, value)
        this.configCache.set(key, value)
      }
      
      // Save to backend
      await invoke('update_configuration', { config: updatedConfig })
      
      // Notify all listeners
      for (const [key, value] of Object.entries(updates)) {
        this.notifyListeners(key, value)
      }
    } catch (error) {
      console.error('Failed to set batch config', error)
      throw error
    }
  }

  // Reset configuration to defaults
  async resetConfig(): Promise<void> {
    try {
      await invoke('reset_configuration')
      this.configCache.clear()
      
      // Notify all listeners that config was reset
      for (const [key, listeners] of this.listeners.entries()) {
        const defaultValue = await this.getConfig(key)
        listeners.forEach(callback => callback(defaultValue))
      }
    } catch (error) {
      console.error('Failed to reset configuration', error)
      throw error
    }
  }

  // Export configuration
  async exportConfig(): Promise<string> {
    try {
      const config = await invoke<any>('get_configuration')
      return JSON.stringify(config, null, 2)
    } catch (error) {
      console.error('Failed to export configuration', error)
      throw error
    }
  }

  // Import configuration
  async importConfig(configJson: string): Promise<void> {
    try {
      const config = JSON.parse(configJson)
      await invoke('update_configuration', { config })
      this.configCache.clear()
      
      // Notify all listeners
      for (const [key, listeners] of this.listeners.entries()) {
        const newValue = this.getNestedValue(config, key)
        this.configCache.set(key, newValue)
        listeners.forEach(callback => callback(newValue))
      }
    } catch (error) {
      console.error('Failed to import configuration', error)
      throw error
    }
  }

  // Private helper methods
  private getNestedValue(obj: any, path: string): any {
    return path.split('.').reduce((current, key) => current?.[key], obj)
  }

  private setNestedValue(obj: any, path: string, value: any): any {
    const keys = path.split('.')
    const lastKey = keys.pop()!
    const target = keys.reduce((current, key) => {
      if (!(key in current)) {
        current[key] = {}
      }
      return current[key]
    }, obj)
    
    target[lastKey] = value
    return obj
  }

  private notifyListeners(key: string, value: any): void {
    const listeners = this.listeners.get(key)
    if (listeners) {
      listeners.forEach(callback => callback(value))
    }
  }
}

// React hook for using configuration
export function useConfig<T>(key: string, defaultValue?: T) {
  const [value, setValue] = React.useState<T | undefined>(defaultValue)
  const [loading, setLoading] = React.useState(true)
  const [error, setError] = React.useState<string | null>(null)

  React.useEffect(() => {
    const configBridge = ConfigBridge.getInstance()
    
    // Load initial value
    configBridge.getConfig(key, defaultValue)
      .then(initialValue => {
        setValue(initialValue)
        setLoading(false)
      })
      .catch(err => {
        setError(err.message)
        setLoading(false)
      })

    // Subscribe to changes
    const unsubscribe = configBridge.subscribe(key, (newValue) => {
      setValue(newValue)
    })

    return unsubscribe
  }, [key, defaultValue])

  const updateValue = React.useCallback(async (newValue: T) => {
    try {
      setError(null)
      await ConfigBridge.getInstance().setConfig(key, newValue)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update config')
    }
  }, [key])

  return {
    value,
    loading,
    error,
    updateValue
  }
}

// Export singleton instance
export const configBridge = ConfigBridge.getInstance()
