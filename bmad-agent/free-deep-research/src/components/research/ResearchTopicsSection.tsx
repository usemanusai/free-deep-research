import React, { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { 
  PlusIcon, 
  TrashIcon, 
  PencilIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  DocumentTextIcon
} from '@heroicons/react/24/outline'
import { clsx } from 'clsx'

interface ResearchTopic {
  id: string
  title: string
  description: string
  keywords: string[]
  priority: 'high' | 'medium' | 'low'
  status: 'draft' | 'active' | 'completed'
}

interface ResearchTopicsSectionProps {
  topics: ResearchTopic[]
  onTopicsChange: (topics: ResearchTopic[]) => void
  isExpanded: boolean
  onToggleExpanded: () => void
  isEditable?: boolean
}

export default function ResearchTopicsSection({
  topics,
  onTopicsChange,
  isExpanded,
  onToggleExpanded,
  isEditable = true
}: ResearchTopicsSectionProps) {
  const [newTopic, setNewTopic] = useState<Partial<ResearchTopic>>({
    title: '',
    description: '',
    keywords: [],
    priority: 'medium',
    status: 'draft'
  })
  const [isAddingTopic, setIsAddingTopic] = useState(false)
  const [editingTopicId, setEditingTopicId] = useState<string | null>(null)

  const handleAddTopic = async () => {
    if (!newTopic.title || !newTopic.description) {
      return
    }

    const topic: ResearchTopic = {
      id: `topic_${Date.now()}`,
      title: newTopic.title,
      description: newTopic.description,
      keywords: newTopic.keywords || [],
      priority: newTopic.priority || 'medium',
      status: 'draft'
    }

    const updatedTopics = [...topics, topic]
    onTopicsChange(updatedTopics)
    
    setNewTopic({
      title: '',
      description: '',
      keywords: [],
      priority: 'medium',
      status: 'draft'
    })
    setIsAddingTopic(false)
  }

  const handleDeleteTopic = (topicId: string) => {
    const updatedTopics = topics.filter(topic => topic.id !== topicId)
    onTopicsChange(updatedTopics)
  }

  const handleUpdateTopic = (topicId: string, updates: Partial<ResearchTopic>) => {
    const updatedTopics = topics.map(topic =>
      topic.id === topicId ? { ...topic, ...updates } : topic
    )
    onTopicsChange(updatedTopics)
    setEditingTopicId(null)
  }

  const getPriorityColor = (priority: ResearchTopic['priority']) => {
    switch (priority) {
      case 'high': return 'bg-red-100 text-red-800'
      case 'medium': return 'bg-yellow-100 text-yellow-800'
      case 'low': return 'bg-green-100 text-green-800'
      default: return 'bg-gray-100 text-gray-800'
    }
  }

  const getStatusColor = (status: ResearchTopic['status']) => {
    switch (status) {
      case 'draft': return 'bg-gray-100 text-gray-800'
      case 'active': return 'bg-blue-100 text-blue-800'
      case 'completed': return 'bg-green-100 text-green-800'
      default: return 'bg-gray-100 text-gray-800'
    }
  }

  return (
    <div className="bg-white rounded-lg shadow border border-gray-200">
      {/* Section Header */}
      <div 
        className="flex items-center justify-between p-6 cursor-pointer hover:bg-gray-50"
        onClick={onToggleExpanded}
      >
        <div className="flex items-center">
          {isExpanded ? (
            <ChevronDownIcon className="w-5 h-5 text-gray-400 mr-3" />
          ) : (
            <ChevronRightIcon className="w-5 h-5 text-gray-400 mr-3" />
          )}
          <DocumentTextIcon className="w-6 h-6 text-blue-600 mr-3" />
          <div>
            <h3 className="text-lg font-semibold text-gray-900">Research Topics</h3>
            <p className="text-sm text-gray-600">Define and manage your research areas</p>
          </div>
        </div>
        <div className="flex items-center space-x-2">
          <span className="text-sm text-gray-500">{topics.length} topics</span>
          {isEditable && (
            <button
              onClick={(e) => {
                e.stopPropagation()
                setIsAddingTopic(true)
              }}
              className="inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded-md text-blue-700 bg-blue-100 hover:bg-blue-200"
            >
              <PlusIcon className="w-4 h-4 mr-1" />
              Add Topic
            </button>
          )}
        </div>
      </div>

      {/* Section Content */}
      {isExpanded && (
        <div className="px-6 pb-6">
          {/* Topics List */}
          <div className="space-y-4">
            {topics.map((topic) => (
              <div key={topic.id} className="border border-gray-200 rounded-lg p-4">
                {editingTopicId === topic.id ? (
                  <EditTopicForm
                    topic={topic}
                    onSave={(updates) => handleUpdateTopic(topic.id, updates)}
                    onCancel={() => setEditingTopicId(null)}
                  />
                ) : (
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <div className="flex items-center space-x-2 mb-2">
                        <h4 className="text-md font-medium text-gray-900">{topic.title}</h4>
                        <span className={clsx('px-2 py-1 text-xs font-medium rounded-full', getPriorityColor(topic.priority))}>
                          {topic.priority}
                        </span>
                        <span className={clsx('px-2 py-1 text-xs font-medium rounded-full', getStatusColor(topic.status))}>
                          {topic.status}
                        </span>
                      </div>
                      <p className="text-sm text-gray-600 mb-2">{topic.description}</p>
                      {topic.keywords.length > 0 && (
                        <div className="flex flex-wrap gap-1">
                          {topic.keywords.map((keyword, index) => (
                            <span key={index} className="px-2 py-1 text-xs bg-gray-100 text-gray-700 rounded">
                              {keyword}
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                    {isEditable && (
                      <div className="flex items-center space-x-2 ml-4">
                        <button
                          onClick={() => setEditingTopicId(topic.id)}
                          className="text-gray-400 hover:text-gray-600"
                        >
                          <PencilIcon className="w-4 h-4" />
                        </button>
                        <button
                          onClick={() => handleDeleteTopic(topic.id)}
                          className="text-gray-400 hover:text-red-600"
                        >
                          <TrashIcon className="w-4 h-4" />
                        </button>
                      </div>
                    )}
                  </div>
                )}
              </div>
            ))}

            {/* Add New Topic Form */}
            {isAddingTopic && (
              <div className="border border-gray-200 rounded-lg p-4 bg-gray-50">
                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Topic Title
                    </label>
                    <input
                      type="text"
                      value={newTopic.title || ''}
                      onChange={(e) => setNewTopic({ ...newTopic, title: e.target.value })}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      placeholder="Enter topic title..."
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Description
                    </label>
                    <textarea
                      value={newTopic.description || ''}
                      onChange={(e) => setNewTopic({ ...newTopic, description: e.target.value })}
                      rows={3}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      placeholder="Describe what you want to research..."
                    />
                  </div>
                  <div className="flex items-center space-x-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-1">
                        Priority
                      </label>
                      <select
                        value={newTopic.priority || 'medium'}
                        onChange={(e) => setNewTopic({ ...newTopic, priority: e.target.value as ResearchTopic['priority'] })}
                        className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      >
                        <option value="low">Low</option>
                        <option value="medium">Medium</option>
                        <option value="high">High</option>
                      </select>
                    </div>
                  </div>
                  <div className="flex items-center space-x-3">
                    <button
                      onClick={handleAddTopic}
                      className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
                    >
                      Add Topic
                    </button>
                    <button
                      onClick={() => setIsAddingTopic(false)}
                      className="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
                    >
                      Cancel
                    </button>
                  </div>
                </div>
              </div>
            )}
          </div>

          {topics.length === 0 && !isAddingTopic && (
            <div className="text-center py-8">
              <DocumentTextIcon className="w-12 h-12 text-gray-400 mx-auto mb-4" />
              <p className="text-gray-500 mb-4">No research topics defined yet</p>
              {isEditable && (
                <button
                  onClick={() => setIsAddingTopic(true)}
                  className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                >
                  <PlusIcon className="w-4 h-4 mr-2" />
                  Add Your First Topic
                </button>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  )
}

interface EditTopicFormProps {
  topic: ResearchTopic
  onSave: (updates: Partial<ResearchTopic>) => void
  onCancel: () => void
}

function EditTopicForm({ topic, onSave, onCancel }: EditTopicFormProps) {
  const [editData, setEditData] = useState<Partial<ResearchTopic>>(topic)

  return (
    <div className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          Topic Title
        </label>
        <input
          type="text"
          value={editData.title || ''}
          onChange={(e) => setEditData({ ...editData, title: e.target.value })}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          Description
        </label>
        <textarea
          value={editData.description || ''}
          onChange={(e) => setEditData({ ...editData, description: e.target.value })}
          rows={3}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div className="flex items-center space-x-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Priority
          </label>
          <select
            value={editData.priority || 'medium'}
            onChange={(e) => setEditData({ ...editData, priority: e.target.value as ResearchTopic['priority'] })}
            className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
          </select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Status
          </label>
          <select
            value={editData.status || 'draft'}
            onChange={(e) => setEditData({ ...editData, status: e.target.value as ResearchTopic['status'] })}
            className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="draft">Draft</option>
            <option value="active">Active</option>
            <option value="completed">Completed</option>
          </select>
        </div>
      </div>
      <div className="flex items-center space-x-3">
        <button
          onClick={() => onSave(editData)}
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
        >
          Save Changes
        </button>
        <button
          onClick={onCancel}
          className="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
        >
          Cancel
        </button>
      </div>
    </div>
  )
}
