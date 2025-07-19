import React from 'react'
import { formatDistanceToNow } from 'date-fns'
import {
  KeyIcon,
  BeakerIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  XCircleIcon,
  ClockIcon,
  UserIcon,
  CogIcon
} from '@heroicons/react/24/outline'
import { cn, truncateText } from '@/utils/codeQuality'

// ============================================================================
// TYPES AND INTERFACES
// ============================================================================

export interface ActivityItem {
  id: string
  type: 'api_key' | 'workflow' | 'template' | 'system' | 'user' | 'security'
  action: string
  description: string
  timestamp: string
  status: 'success' | 'warning' | 'error' | 'info'
  user?: string
  metadata?: Record<string, any>
}

export interface ActivityFeedProps {
  activities: ActivityItem[]
  maxItems?: number
  showTimestamps?: boolean
  showUsers?: boolean
  className?: string
  onItemClick?: (item: ActivityItem) => void
  loading?: boolean
  error?: string
}

export interface ActivityItemProps {
  item: ActivityItem
  showTimestamp?: boolean
  showUser?: boolean
  onClick?: (item: ActivityItem) => void
  className?: string
}

// ============================================================================
// ACTIVITY ITEM COMPONENT
// ============================================================================

export function ActivityItemComponent({
  item,
  showTimestamp = true,
  showUser = true,
  onClick,
  className = ''
}: ActivityItemProps) {
  const getIcon = (type: ActivityItem['type']) => {
    switch (type) {
      case 'api_key':
        return KeyIcon
      case 'workflow':
        return BeakerIcon
      case 'template':
        return CogIcon
      case 'system':
        return CheckCircleIcon
      case 'user':
        return UserIcon
      case 'security':
        return ExclamationTriangleIcon
      default:
        return ClockIcon
    }
  }

  const getStatusColor = (status: ActivityItem['status']) => {
    switch (status) {
      case 'success':
        return 'text-green-600 bg-green-100'
      case 'warning':
        return 'text-yellow-600 bg-yellow-100'
      case 'error':
        return 'text-red-600 bg-red-100'
      case 'info':
      default:
        return 'text-blue-600 bg-blue-100'
    }
  }

  const getStatusIcon = (status: ActivityItem['status']) => {
    switch (status) {
      case 'success':
        return CheckCircleIcon
      case 'warning':
        return ExclamationTriangleIcon
      case 'error':
        return XCircleIcon
      case 'info':
      default:
        return ClockIcon
    }
  }

  const Icon = getIcon(item.type)
  const StatusIcon = getStatusIcon(item.status)
  const statusColor = getStatusColor(item.status)

  const content = (
    <div className={cn(
      'flex items-start space-x-3 p-4 rounded-lg transition-colors duration-200',
      onClick && 'hover:bg-gray-50 cursor-pointer',
      className
    )}>
      {/* Icon */}
      <div className={cn(
        'flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center',
        statusColor
      )}>
        <Icon className="w-4 h-4" />
      </div>

      {/* Content */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center justify-between">
          <p className="text-sm font-medium text-gray-900">
            {item.action}
          </p>
          <div className="flex items-center space-x-2">
            <StatusIcon className={cn('w-4 h-4', statusColor.split(' ')[0])} />
            {showTimestamp && (
              <span className="text-xs text-gray-500">
                {formatDistanceToNow(new Date(item.timestamp), { addSuffix: true })}
              </span>
            )}
          </div>
        </div>
        
        <p className="text-sm text-gray-600 mt-1">
          {truncateText(item.description, 100)}
        </p>
        
        {showUser && item.user && (
          <p className="text-xs text-gray-500 mt-1">
            by {item.user}
          </p>
        )}
        
        {item.metadata && Object.keys(item.metadata).length > 0 && (
          <div className="mt-2 flex flex-wrap gap-1">
            {Object.entries(item.metadata).slice(0, 3).map(([key, value]) => (
              <span
                key={key}
                className="inline-flex items-center px-2 py-1 rounded-full text-xs bg-gray-100 text-gray-600"
              >
                {key}: {String(value)}
              </span>
            ))}
          </div>
        )}
      </div>
    </div>
  )

  if (onClick) {
    return (
      <button
        onClick={() => onClick(item)}
        className="w-full text-left focus:outline-none focus:ring-2 focus:ring-blue-500 rounded-lg"
      >
        {content}
      </button>
    )
  }

  return content
}

// ============================================================================
// ACTIVITY FEED COMPONENT
// ============================================================================

export function ActivityFeed({
  activities,
  maxItems = 10,
  showTimestamps = true,
  showUsers = true,
  className = '',
  onItemClick,
  loading = false,
  error
}: ActivityFeedProps) {
  if (loading) {
    return (
      <div className={cn('space-y-4', className)}>
        {Array.from({ length: 5 }).map((_, index) => (
          <div key={index} className="animate-pulse">
            <div className="flex items-start space-x-3 p-4">
              <div className="w-8 h-8 bg-gray-200 rounded-full"></div>
              <div className="flex-1 space-y-2">
                <div className="h-4 bg-gray-200 rounded w-3/4"></div>
                <div className="h-3 bg-gray-200 rounded w-1/2"></div>
              </div>
            </div>
          </div>
        ))}
      </div>
    )
  }

  if (error) {
    return (
      <div className={cn(
        'flex items-center justify-center p-8 text-center',
        className
      )}>
        <div>
          <ExclamationTriangleIcon className="w-8 h-8 text-red-500 mx-auto mb-2" />
          <p className="text-sm text-gray-600">Failed to load activity feed</p>
          <p className="text-xs text-gray-500 mt-1">{error}</p>
        </div>
      </div>
    )
  }

  if (!activities || activities.length === 0) {
    return (
      <div className={cn(
        'flex items-center justify-center p-8 text-center',
        className
      )}>
        <div>
          <ClockIcon className="w-8 h-8 text-gray-400 mx-auto mb-2" />
          <p className="text-sm text-gray-600">No recent activity</p>
          <p className="text-xs text-gray-500 mt-1">
            Activity will appear here as you use the system
          </p>
        </div>
      </div>
    )
  }

  const displayedActivities = activities.slice(0, maxItems)

  return (
    <div className={cn('space-y-2', className)}>
      {displayedActivities.map((item) => (
        <ActivityItemComponent
          key={item.id}
          item={item}
          showTimestamp={showTimestamps}
          showUser={showUsers}
          onClick={onItemClick}
          className="border border-gray-200 hover:border-gray-300"
        />
      ))}
      
      {activities.length > maxItems && (
        <div className="text-center pt-4">
          <p className="text-sm text-gray-500">
            Showing {maxItems} of {activities.length} activities
          </p>
        </div>
      )}
    </div>
  )
}

// ============================================================================
// ACTIVITY FEED HEADER COMPONENT
// ============================================================================

export interface ActivityFeedHeaderProps {
  title?: string
  subtitle?: string
  actionButton?: React.ReactNode
  className?: string
}

export function ActivityFeedHeader({
  title = 'Recent Activity',
  subtitle = 'Latest system events and user actions',
  actionButton,
  className = ''
}: ActivityFeedHeaderProps) {
  return (
    <div className={cn('flex items-center justify-between mb-6', className)}>
      <div>
        <h2 className="text-lg font-semibold text-gray-900">{title}</h2>
        {subtitle && (
          <p className="text-sm text-gray-600 mt-1">{subtitle}</p>
        )}
      </div>
      {actionButton && (
        <div className="flex-shrink-0">
          {actionButton}
        </div>
      )}
    </div>
  )
}

// ============================================================================
// ACTIVITY FILTER COMPONENT
// ============================================================================

export interface ActivityFilterProps {
  selectedTypes: ActivityItem['type'][]
  selectedStatuses: ActivityItem['status'][]
  onTypesChange: (types: ActivityItem['type'][]) => void
  onStatusesChange: (statuses: ActivityItem['status'][]) => void
  className?: string
}

export function ActivityFilter({
  selectedTypes,
  selectedStatuses,
  onTypesChange,
  onStatusesChange,
  className = ''
}: ActivityFilterProps) {
  const activityTypes: { value: ActivityItem['type']; label: string }[] = [
    { value: 'api_key', label: 'API Keys' },
    { value: 'workflow', label: 'Workflows' },
    { value: 'template', label: 'Templates' },
    { value: 'system', label: 'System' },
    { value: 'user', label: 'User' },
    { value: 'security', label: 'Security' }
  ]

  const activityStatuses: { value: ActivityItem['status']; label: string }[] = [
    { value: 'success', label: 'Success' },
    { value: 'warning', label: 'Warning' },
    { value: 'error', label: 'Error' },
    { value: 'info', label: 'Info' }
  ]

  return (
    <div className={cn('space-y-4', className)}>
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Activity Types
        </label>
        <div className="flex flex-wrap gap-2">
          {activityTypes.map((type) => (
            <button
              key={type.value}
              onClick={() => {
                if (selectedTypes.includes(type.value)) {
                  onTypesChange(selectedTypes.filter(t => t !== type.value))
                } else {
                  onTypesChange([...selectedTypes, type.value])
                }
              }}
              className={cn(
                'px-3 py-1 rounded-full text-xs font-medium transition-colors',
                selectedTypes.includes(type.value)
                  ? 'bg-blue-100 text-blue-800'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
              )}
            >
              {type.label}
            </button>
          ))}
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Status
        </label>
        <div className="flex flex-wrap gap-2">
          {activityStatuses.map((status) => (
            <button
              key={status.value}
              onClick={() => {
                if (selectedStatuses.includes(status.value)) {
                  onStatusesChange(selectedStatuses.filter(s => s !== status.value))
                } else {
                  onStatusesChange([...selectedStatuses, status.value])
                }
              }}
              className={cn(
                'px-3 py-1 rounded-full text-xs font-medium transition-colors',
                selectedStatuses.includes(status.value)
                  ? 'bg-blue-100 text-blue-800'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
              )}
            >
              {status.label}
            </button>
          ))}
        </div>
      </div>
    </div>
  )
}

// ============================================================================
// EXPORT DEFAULT
// ============================================================================

export default ActivityFeed
