import React from 'react'
import { cn } from '@/utils/codeQuality'

// ============================================================================
// TYPES AND INTERFACES
// ============================================================================

export interface MetricsCardProps {
  title: string
  value: string | number
  subtitle?: string
  icon?: React.ComponentType<{ className?: string }>
  trend?: {
    value: number
    label: string
    isPositive?: boolean
  }
  status?: 'success' | 'warning' | 'error' | 'neutral'
  className?: string
  onClick?: () => void
  loading?: boolean
  error?: string
}

export interface MetricsGridProps {
  children: React.ReactNode
  className?: string
}

export interface MetricsTrendProps {
  value: number
  label: string
  isPositive?: boolean
  className?: string
}

// ============================================================================
// TREND COMPONENT
// ============================================================================

export function MetricsTrend({ 
  value, 
  label, 
  isPositive = true,
  className = '' 
}: MetricsTrendProps) {
  const trendColor = isPositive 
    ? value > 0 ? 'text-green-600' : 'text-red-600'
    : value > 0 ? 'text-red-600' : 'text-green-600'
  
  const trendIcon = value > 0 ? '↗' : value < 0 ? '↘' : '→'
  
  return (
    <div className={cn('flex items-center text-sm', className)}>
      <span className={cn('font-medium', trendColor)}>
        {trendIcon} {Math.abs(value)}%
      </span>
      <span className="text-gray-500 ml-1">{label}</span>
    </div>
  )
}

// ============================================================================
// METRICS CARD COMPONENT
// ============================================================================

export function MetricsCard({
  title,
  value,
  subtitle,
  icon: Icon,
  trend,
  status = 'neutral',
  className = '',
  onClick,
  loading = false,
  error
}: MetricsCardProps) {
  const statusStyles = {
    success: 'border-green-200 bg-green-50',
    warning: 'border-yellow-200 bg-yellow-50',
    error: 'border-red-200 bg-red-50',
    neutral: 'border-gray-200 bg-white'
  }

  const statusIconColors = {
    success: 'text-green-600',
    warning: 'text-yellow-600',
    error: 'text-red-600',
    neutral: 'text-gray-600'
  }

  if (loading) {
    return (
      <div className={cn(
        'p-6 border rounded-lg animate-pulse',
        statusStyles.neutral,
        className
      )}>
        <div className="flex items-center justify-between mb-4">
          <div className="h-4 bg-gray-200 rounded w-1/2"></div>
          <div className="h-6 w-6 bg-gray-200 rounded"></div>
        </div>
        <div className="h-8 bg-gray-200 rounded w-3/4 mb-2"></div>
        <div className="h-4 bg-gray-200 rounded w-1/2"></div>
      </div>
    )
  }

  if (error) {
    return (
      <div className={cn(
        'p-6 border rounded-lg',
        statusStyles.error,
        className
      )}>
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-sm font-medium text-gray-900">{title}</h3>
          {Icon && <Icon className="h-6 w-6 text-red-600" />}
        </div>
        <div className="text-sm text-red-600">
          Error loading data
        </div>
      </div>
    )
  }

  const cardContent = (
    <>
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-sm font-medium text-gray-900">{title}</h3>
        {Icon && (
          <Icon className={cn('h-6 w-6', statusIconColors[status])} />
        )}
      </div>
      
      <div className="mb-2">
        <div className="text-2xl font-bold text-gray-900">
          {typeof value === 'number' ? value.toLocaleString() : value}
        </div>
        {subtitle && (
          <div className="text-sm text-gray-500">{subtitle}</div>
        )}
      </div>
      
      {trend && (
        <MetricsTrend
          value={trend.value}
          label={trend.label}
          isPositive={trend.isPositive}
        />
      )}
    </>
  )

  if (onClick) {
    return (
      <button
        onClick={onClick}
        className={cn(
          'p-6 border rounded-lg text-left transition-all duration-200',
          'hover:shadow-md hover:border-blue-300 focus:outline-none focus:ring-2 focus:ring-blue-500',
          statusStyles[status],
          className
        )}
      >
        {cardContent}
      </button>
    )
  }

  return (
    <div className={cn(
      'p-6 border rounded-lg',
      statusStyles[status],
      className
    )}>
      {cardContent}
    </div>
  )
}

// ============================================================================
// METRICS GRID COMPONENT
// ============================================================================

export function MetricsGrid({ 
  children, 
  className = '' 
}: MetricsGridProps) {
  return (
    <div className={cn(
      'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6',
      className
    )}>
      {children}
    </div>
  )
}

// ============================================================================
// SPECIALIZED METRICS CARDS
// ============================================================================

export interface SystemHealthCardProps {
  health: {
    security: string
    data_persistence: string
    monitoring: string
    api_manager: string
    research_engine: string
  }
  className?: string
}

export function SystemHealthCard({ health, className }: SystemHealthCardProps) {
  const healthyServices = Object.values(health).filter(status => status === 'Healthy').length
  const totalServices = Object.values(health).length
  const healthPercentage = Math.round((healthyServices / totalServices) * 100)
  
  const status = healthPercentage === 100 ? 'success' : 
                 healthPercentage >= 80 ? 'warning' : 'error'

  return (
    <MetricsCard
      title="System Health"
      value={`${healthyServices}/${totalServices}`}
      subtitle="Services Healthy"
      status={status}
      trend={{
        value: healthPercentage,
        label: 'Overall Health',
        isPositive: true
      }}
      className={className}
    />
  )
}

export interface ResourceUsageCardProps {
  resources: {
    cpu_usage_percent: number
    memory_usage_percent: number
    disk_usage_percent: number
  }
  className?: string
}

export function ResourceUsageCard({ resources, className }: ResourceUsageCardProps) {
  const avgUsage = Math.round(
    (resources.cpu_usage_percent + resources.memory_usage_percent + resources.disk_usage_percent) / 3
  )
  
  const status = avgUsage < 70 ? 'success' : 
                 avgUsage < 85 ? 'warning' : 'error'

  return (
    <MetricsCard
      title="Resource Usage"
      value={`${avgUsage}%`}
      subtitle="Average Usage"
      status={status}
      trend={{
        value: avgUsage - 60, // Assuming 60% is baseline
        label: 'vs baseline',
        isPositive: false
      }}
      className={className}
    />
  )
}

export interface QueueStatusCardProps {
  queue: {
    total_workflows: number
    active_workflows: number
    queued_workflows: number
    completed_workflows: number
    failed_workflows: number
  }
  className?: string
}

export function QueueStatusCard({ queue, className }: QueueStatusCardProps) {
  const successRate = queue.total_workflows > 0 
    ? Math.round((queue.completed_workflows / queue.total_workflows) * 100)
    : 100
  
  const status = successRate >= 95 ? 'success' : 
                 successRate >= 85 ? 'warning' : 'error'

  return (
    <MetricsCard
      title="Queue Status"
      value={queue.active_workflows}
      subtitle="Active Workflows"
      status={status}
      trend={{
        value: successRate,
        label: 'Success Rate',
        isPositive: true
      }}
      className={className}
    />
  )
}

// ============================================================================
// EXPORT DEFAULT
// ============================================================================

export default MetricsCard
