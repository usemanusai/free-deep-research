import React from 'react'
import { BeakerIcon } from '@heroicons/react/24/outline'

interface LoadingScreenProps {
  message?: string
}

export default function LoadingScreen({ message = 'Loading...' }: LoadingScreenProps) {
  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center">
      <div className="text-center">
        {/* Logo */}
        <div className="flex justify-center mb-6">
          <div className="h-16 w-16 bg-primary-600 rounded-xl flex items-center justify-center animate-pulse">
            <BeakerIcon className="h-8 w-8 text-white" />
          </div>
        </div>
        
        {/* Title */}
        <h1 className="text-2xl font-bold text-gray-900 mb-2">
          Free Deep Research System
        </h1>
        
        {/* Subtitle */}
        <p className="text-gray-600 mb-8">
          Enterprise-grade research powered by AI
        </p>
        
        {/* Loading indicator */}
        <div className="flex items-center justify-center space-x-2 mb-4">
          <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-primary-600"></div>
          <span className="text-gray-700 font-medium">{message}</span>
        </div>
        
        {/* Progress dots */}
        <div className="flex justify-center space-x-1">
          <div className="h-2 w-2 bg-primary-600 rounded-full animate-pulse"></div>
          <div className="h-2 w-2 bg-primary-600 rounded-full animate-pulse" style={{ animationDelay: '0.1s' }}></div>
          <div className="h-2 w-2 bg-primary-600 rounded-full animate-pulse" style={{ animationDelay: '0.2s' }}></div>
        </div>
        
        {/* Version info */}
        <div className="mt-8 text-xs text-gray-400">
          <p>Version 1.0.0 • BMAD AI Agent Team</p>
        </div>
      </div>
    </div>
  )
}
