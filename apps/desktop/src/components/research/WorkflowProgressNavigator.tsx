import React from 'react'
import { CheckCircleIcon, ClockIcon, ExclamationTriangleIcon } from '@heroicons/react/24/outline'
import { clsx } from 'clsx'

interface WorkflowStep {
  id: string
  name: string
  description: string
  status: 'completed' | 'current' | 'upcoming' | 'error'
  progress?: number
}

interface WorkflowProgressNavigatorProps {
  steps: WorkflowStep[]
  currentStep: number
  onStepClick?: (stepIndex: number) => void
}

export default function WorkflowProgressNavigator({ 
  steps, 
  currentStep, 
  onStepClick 
}: WorkflowProgressNavigatorProps) {
  return (
    <nav aria-label="Progress" className="mb-8">
      <ol role="list" className="space-y-4 md:flex md:space-y-0 md:space-x-8">
        {steps.map((step, stepIdx) => (
          <li key={step.id} className="md:flex-1">
            <div
              className={clsx(
                'group flex flex-col border-l-4 py-2 pl-4 md:border-l-0 md:border-t-4 md:pb-0 md:pl-0 md:pt-4',
                step.status === 'completed'
                  ? 'border-green-600'
                  : step.status === 'current'
                  ? 'border-blue-600'
                  : step.status === 'error'
                  ? 'border-red-600'
                  : 'border-gray-200',
                onStepClick && 'cursor-pointer hover:border-gray-300'
              )}
              onClick={() => onStepClick?.(stepIdx)}
            >
              <span className="text-sm font-medium">
                <div className="flex items-center">
                  {step.status === 'completed' && (
                    <CheckCircleIcon className="w-5 h-5 text-green-600 mr-2" />
                  )}
                  {step.status === 'current' && (
                    <ClockIcon className="w-5 h-5 text-blue-600 mr-2" />
                  )}
                  {step.status === 'error' && (
                    <ExclamationTriangleIcon className="w-5 h-5 text-red-600 mr-2" />
                  )}
                  {step.status === 'upcoming' && (
                    <div className="w-5 h-5 rounded-full border-2 border-gray-300 mr-2" />
                  )}
                  <span
                    className={clsx(
                      step.status === 'completed'
                        ? 'text-green-600'
                        : step.status === 'current'
                        ? 'text-blue-600'
                        : step.status === 'error'
                        ? 'text-red-600'
                        : 'text-gray-500'
                    )}
                  >
                    Step {stepIdx + 1}: {step.name}
                  </span>
                </div>
              </span>
              <span className="text-sm text-gray-500 mt-1">{step.description}</span>
              
              {/* Progress bar for current step */}
              {step.status === 'current' && step.progress !== undefined && (
                <div className="mt-2">
                  <div className="flex justify-between text-xs text-gray-600 mb-1">
                    <span>Progress</span>
                    <span>{Math.round(step.progress)}%</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div 
                      className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                      style={{ width: `${step.progress}%` }}
                    />
                  </div>
                </div>
              )}
            </div>
          </li>
        ))}
      </ol>
    </nav>
  )
}
