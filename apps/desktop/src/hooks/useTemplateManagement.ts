import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/core'
import { ResearchTemplate, TemplateParameter, TemplateStep } from '@/types/api'

// ============================================================================
// TEMPLATE MANAGEMENT HOOKS
// ============================================================================

export interface TemplateStatistics {
  total: number
  public: number
  featured: number
  categories: number
  total_usage: number
  average_rating: number
  total_executions: number
  average_success_rate: number
}

export interface TemplateExecutionContext {
  template_id: string
  parameters: Record<string, any>
  workflow_name: string
  created_by: string
  execution_metadata: Record<string, any>
}

export interface CreateTemplateRequest {
  name: string
  description: string
  category: string
  version: string
  author: string
  is_public: boolean
  is_featured: boolean
  tags: string[]
  parameters: TemplateParameter[]
  workflow_steps: TemplateStep[]
}

export interface UpdateTemplateRequest {
  id: string
  name?: string
  description?: string
  category?: string
  version?: string
  is_public?: boolean
  is_featured?: boolean
  tags?: string[]
  parameters?: TemplateParameter[]
  workflow_steps?: TemplateStep[]
}

// ============================================================================
// TEMPLATE QUERY HOOKS
// ============================================================================

export function useTemplates(category?: string, searchQuery?: string) {
  return useQuery({
    queryKey: ['templates', category, searchQuery],
    queryFn: async () => {
      if (searchQuery?.trim()) {
        return invoke<ResearchTemplate[]>('search_research_templates', { query: searchQuery })
      } else if (category && category !== 'all') {
        if (category === 'featured') {
          return invoke<ResearchTemplate[]>('get_featured_research_templates')
        } else if (category === 'public') {
          return invoke<ResearchTemplate[]>('get_public_research_templates')
        } else {
          return invoke<ResearchTemplate[]>('get_research_templates_by_category', { category })
        }
      } else {
        return invoke<ResearchTemplate[]>('get_all_research_templates')
      }
    },
    refetchInterval: 30000,
    retry: 2,
  })
}

export function useTemplate(templateId: string) {
  return useQuery({
    queryKey: ['template', templateId],
    queryFn: () => invoke<ResearchTemplate>('get_research_template', { templateId }),
    enabled: !!templateId,
    retry: 2,
  })
}

export function useTemplateStatistics() {
  return useQuery({
    queryKey: ['template-statistics'],
    queryFn: () => invoke<TemplateStatistics>('get_template_statistics'),
    refetchInterval: 60000,
    retry: 2,
  })
}

export function useTemplateRecommendations(limit: number = 5) {
  return useQuery({
    queryKey: ['template-recommendations', limit],
    queryFn: () => invoke<ResearchTemplate[]>('get_template_recommendations', { limit }),
    refetchInterval: 300000, // 5 minutes
    retry: 2,
  })
}

export function useFeaturedTemplates() {
  return useQuery({
    queryKey: ['featured-templates'],
    queryFn: () => invoke<ResearchTemplate[]>('get_featured_research_templates'),
    refetchInterval: 60000,
    retry: 2,
  })
}

export function usePublicTemplates() {
  return useQuery({
    queryKey: ['public-templates'],
    queryFn: () => invoke<ResearchTemplate[]>('get_public_research_templates'),
    refetchInterval: 60000,
    retry: 2,
  })
}

export function useTemplatesByCategory(category: string) {
  return useQuery({
    queryKey: ['templates-by-category', category],
    queryFn: () => invoke<ResearchTemplate[]>('get_research_templates_by_category', { category }),
    enabled: !!category,
    refetchInterval: 60000,
    retry: 2,
  })
}

// ============================================================================
// TEMPLATE MUTATION HOOKS
// ============================================================================

export function useCreateTemplate() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (template: CreateTemplateRequest) =>
      invoke<ResearchTemplate>('create_research_template', { template }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['templates'] })
      queryClient.invalidateQueries({ queryKey: ['template-statistics'] })
    },
  })
}

export function useUpdateTemplate() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (updates: UpdateTemplateRequest) =>
      invoke<ResearchTemplate>('update_research_template', { updates }),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: ['templates'] })
      queryClient.invalidateQueries({ queryKey: ['template', data.id] })
      queryClient.invalidateQueries({ queryKey: ['template-statistics'] })
    },
  })
}

export function useDeleteTemplate() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (templateId: string) =>
      invoke('delete_research_template', { templateId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['templates'] })
      queryClient.invalidateQueries({ queryKey: ['template-statistics'] })
    },
  })
}

export function useExecuteTemplate() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (context: TemplateExecutionContext) =>
      invoke('execute_research_template', { context }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['research-workflows'] })
      queryClient.invalidateQueries({ queryKey: ['template-statistics'] })
    },
  })
}

export function useRateTemplate() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ templateId, rating }: { templateId: string; rating: number }) =>
      invoke('rate_research_template', { templateId, rating }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['templates'] })
      queryClient.invalidateQueries({ queryKey: ['template-statistics'] })
    },
  })
}

export function useCloneTemplate() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ templateId, newName }: { templateId: string; newName: string }) =>
      invoke<ResearchTemplate>('clone_research_template', { templateId, newName }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['templates'] })
      queryClient.invalidateQueries({ queryKey: ['template-statistics'] })
    },
  })
}

export function useImportTemplate() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (templateData: string) =>
      invoke<ResearchTemplate>('import_research_template', { templateData }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['templates'] })
      queryClient.invalidateQueries({ queryKey: ['template-statistics'] })
    },
  })
}

export function useExportTemplate() {
  return useMutation({
    mutationFn: (templateId: string) =>
      invoke<string>('export_research_template', { templateId }),
  })
}

// ============================================================================
// TEMPLATE VALIDATION HOOKS
// ============================================================================

export function useValidateTemplate() {
  return useMutation({
    mutationFn: (template: CreateTemplateRequest | UpdateTemplateRequest) =>
      invoke<{ valid: boolean; errors: string[] }>('validate_research_template', { template }),
  })
}

// ============================================================================
// COMBINED TEMPLATE MANAGEMENT HOOK
// ============================================================================

export function useTemplateManagement(category?: string, searchQuery?: string) {
  const templatesQuery = useTemplates(category, searchQuery)
  const statisticsQuery = useTemplateStatistics()
  const recommendationsQuery = useTemplateRecommendations()

  const createMutation = useCreateTemplate()
  const updateMutation = useUpdateTemplate()
  const deleteMutation = useDeleteTemplate()
  const executeMutation = useExecuteTemplate()
  const rateMutation = useRateTemplate()

  return {
    // Data
    templates: templatesQuery.data || [],
    statistics: statisticsQuery.data,
    recommendations: recommendationsQuery.data || [],
    
    // Loading states
    isLoading: templatesQuery.isLoading || statisticsQuery.isLoading,
    isCreating: createMutation.isPending,
    isUpdating: updateMutation.isPending,
    isDeleting: deleteMutation.isPending,
    isExecuting: executeMutation.isPending,
    isRating: rateMutation.isPending,
    
    // Error states
    error: templatesQuery.error || statisticsQuery.error,
    createError: createMutation.error,
    updateError: updateMutation.error,
    deleteError: deleteMutation.error,
    executeError: executeMutation.error,
    rateError: rateMutation.error,
    
    // Actions
    createTemplate: createMutation.mutate,
    updateTemplate: updateMutation.mutate,
    deleteTemplate: deleteMutation.mutate,
    executeTemplate: executeMutation.mutate,
    rateTemplate: rateMutation.mutate,
    
    // Refetch
    refetch: () => {
      templatesQuery.refetch()
      statisticsQuery.refetch()
      recommendationsQuery.refetch()
    }
  }
}
