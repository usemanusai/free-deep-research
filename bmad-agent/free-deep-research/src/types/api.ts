// API types and interfaces for the Free Deep Research System

// ============================================================================
// DASHBOARD & MONITORING TYPES
// ============================================================================

export interface SystemMetrics {
  cpu_usage: number;
  memory_usage: number;
  disk_usage: number;
  network_activity: NetworkActivity;
  uptime: number;
  active_processes: number;
}

export interface NetworkActivity {
  bytes_sent: number;
  bytes_received: number;
  requests_per_minute: number;
}

export interface ServiceHealthStatus {
  overall_status: 'healthy' | 'degraded' | 'unhealthy';
  services: Record<string, ServiceStatus>;
  last_check: string;
  uptime: number;
}

export interface ServiceStatus {
  name: string;
  status: 'healthy' | 'degraded' | 'unhealthy';
  response_time: number;
  last_check: string;
  error_message?: string;
}

export interface DashboardStats {
  total_api_keys: number;
  active_api_keys: number;
  total_research: number;
  active_research: number;
  completed_research: number;
  failed_research: number;
  system_health: ServiceHealthStatus;
  uptime: string;
  recent_activity: ActivityEvent[];
}

export interface ActivityEvent {
  id: string;
  type: 'research' | 'api' | 'system' | 'security';
  message: string;
  timestamp: string;
  severity: 'info' | 'warning' | 'error' | 'success';
  metadata?: Record<string, any>;
}

// ============================================================================
// API KEY MANAGEMENT TYPES (Enhanced)
// ============================================================================

export interface ApiKey {
  id: string
  service: ServiceProvider
  name: string
  encrypted_key: string
  usage_count: number
  rate_limit: number
  reset_period: ResetPeriod
  last_used?: string
  last_reset: string
  status: ApiKeyStatus
  created_at: string
  updated_at: string
  usage_stats: ApiKeyUsageStats;
  performance_metrics: ApiKeyPerformanceMetrics;
}

export type ServiceProvider = 'openrouter' | 'serpapi' | 'jina' | 'firecrawl'

export type ResetPeriod = 'daily' | 'monthly'

export type ApiKeyStatus = 'active' | 'exhausted' | 'error' | 'disabled' | 'rate_limited'

export interface ApiKeyUsageStats {
  total_requests: number;
  successful_requests: number;
  failed_requests: number;
  current_period_usage: number;
  current_period_limit: number;
  reset_time: string;
  average_response_time: number;
  last_24h_usage: number[];
}

export interface ApiKeyPerformanceMetrics {
  key_id: string;
  success_rate: number;
  average_response_time: number;
  error_rate: number;
  uptime_percentage: number;
  last_failure: string | null;
  performance_score: number;
}

export interface CreateApiKeyRequest {
  service: ServiceProvider
  name: string
  api_key: string
  rate_limit?: number
}

export interface UpdateApiKeyRequest {
  name?: string
  api_key?: string
  rate_limit?: number
  status?: ApiKeyStatus
}

export interface ApiKeyTestResult {
  key_id: string
  success: boolean
  message: string
  response_time_ms?: number
  tested_at: string
}

export interface ApiKeyImport {
  service: ServiceProvider
  name: string
  api_key: string
  rate_limit?: number
}

export interface ApiKeyExport {
  service: ServiceProvider
  name: string
  rate_limit: number
  usage_count: number
  status: ApiKeyStatus
  created_at: string
}

export interface ImportResult {
  successful_count: number
  failed_count: number
  errors: ImportError[]
}

export interface ImportError {
  index: number
  service: string
  name: string
  error: string
}

// ============================================================================
// RESEARCH WORKFLOW TYPES
// ============================================================================

export interface ResearchWorkflow {
  id: string;
  name: string;
  description: string;
  template_id: string | null;
  status: WorkflowStatus;
  methodology: 'don_lim' | 'nick_scamara' | 'hybrid';
  parameters: WorkflowParameters;
  created_at: string;
  started_at: string | null;
  completed_at: string | null;
  progress: WorkflowProgress;
  results: WorkflowResults | null;
  error_message: string | null;
  estimated_completion: string | null;
}

export type WorkflowStatus =
  | 'draft'
  | 'queued'
  | 'running'
  | 'paused'
  | 'completed'
  | 'failed'
  | 'cancelled';

export interface WorkflowParameters {
  query: string;
  max_results: number;
  depth_level: number;
  include_sources: boolean;
  output_format: string[];
  custom_instructions: string | null;
  filters: Record<string, any>;
}

export interface WorkflowProgress {
  current_step: string;
  total_steps: number;
  completed_steps: number;
  percentage: number;
  estimated_time_remaining: number;
  current_operation: string;
  step_details: StepProgress[];
}

export interface StepProgress {
  step_name: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  start_time: string | null;
  end_time: string | null;
  progress_percentage: number;
  details: string | null;
}

export interface WorkflowResults {
  summary: string;
  sources_found: number;
  data_points: number;
  confidence_score: number;
  output_files: OutputFile[];
  metadata: Record<string, any>;
  execution_time: number;
}

export interface OutputFile {
  filename: string;
  format: string;
  size: number;
  path: string;
  created_at: string;
}

// ============================================================================
// RESEARCH TEMPLATE TYPES
// ============================================================================

export interface ResearchTemplate {
  id: string;
  name: string;
  description: string;
  category: string;
  version: string;
  author: string;
  is_public: boolean;
  is_featured: boolean;
  rating: number;
  usage_count: number;
  parameters: TemplateParameter[];
  workflow_steps: TemplateStep[];
  created_at: string;
  updated_at: string;
  tags: string[];
}

export interface TemplateParameter {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'select' | 'multiselect';
  description: string;
  required: boolean;
  default_value: any;
  options?: string[];
  validation?: ParameterValidation;
}

export interface ParameterValidation {
  min_length?: number;
  max_length?: number;
  min_value?: number;
  max_value?: number;
  pattern?: string;
}

export interface TemplateStep {
  step_id: string;
  name: string;
  description: string;
  type: 'search' | 'extract' | 'analyze' | 'format';
  configuration: Record<string, any>;
  dependencies: string[];
}

// ============================================================================
// QUEUE MANAGEMENT TYPES
// ============================================================================

export interface QueueStatistics {
  total_queued: number;
  currently_running: number;
  completed_today: number;
  failed_today: number;
  average_execution_time: number;
  queue_processing_rate: number;
  estimated_queue_time: number;
}

export interface QueueConfiguration {
  max_concurrent_workflows: number;
  max_queue_size: number;
  priority_levels: number;
  auto_retry_failed: boolean;
  retry_attempts: number;
  queue_processing_enabled: boolean;
}

export interface ResourceStatus {
  cpu_usage: number;
  memory_usage: number;
  api_quota_usage: Record<string, number>;
  concurrent_workflows: number;
  max_concurrent_workflows: number;
  available_capacity: number;
}

// ============================================================================
// CONFIGURATION TYPES
// ============================================================================

export interface AppConfiguration {
  general: GeneralConfig;
  security: SecurityConfig;
  api_management: ApiManagementConfig;
  research: ResearchConfig;
  monitoring: MonitoringConfig;
  backup: BackupConfig;
}

export interface GeneralConfig {
  app_name: string;
  version: string;
  environment: 'development' | 'production';
  log_level: 'debug' | 'info' | 'warn' | 'error';
  auto_save_interval: number;
  theme: 'light' | 'dark' | 'auto';
}

export interface SecurityConfig {
  encryption_enabled: boolean;
  master_password_required: boolean;
  session_timeout: number;
  audit_logging_enabled: boolean;
  backup_encryption_enabled: boolean;
}

export interface ApiManagementConfig {
  auto_rotation_enabled: boolean;
  rate_limit_buffer_percentage: number;
  health_check_interval: number;
  performance_monitoring_enabled: boolean;
  emergency_stop_threshold: number;
}

export interface ResearchConfig {
  default_methodology: 'don_lim' | 'nick_scamara' | 'hybrid';
  max_concurrent_workflows: number;
  default_output_formats: string[];
  auto_cleanup_completed: boolean;
  cleanup_after_days: number;
}

export interface MonitoringConfig {
  metrics_collection_enabled: boolean;
  metrics_retention_days: number;
  real_time_monitoring_enabled: boolean;
  alert_thresholds: AlertThresholds;
}

export interface AlertThresholds {
  cpu_usage_warning: number;
  cpu_usage_critical: number;
  memory_usage_warning: number;
  memory_usage_critical: number;
  api_quota_warning: number;
  api_quota_critical: number;
}

export interface BackupConfig {
  auto_backup_enabled: boolean;
  backup_interval_seconds: number;
  max_backup_files: number;
  backup_compression_enabled: boolean;
  backup_encryption_enabled: boolean;
}

// ============================================================================
// AUDIT & SECURITY TYPES
// ============================================================================

export interface AuditEvent {
  id: string;
  event_type: string;
  user_id: string | null;
  resource_type: string;
  resource_id: string | null;
  action: string;
  details: Record<string, any>;
  ip_address: string | null;
  user_agent: string | null;
  timestamp: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
}

// ============================================================================
// UTILITY TYPES
// ============================================================================

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T;
  message: string | null;
  errors: string[] | null;
  timestamp: string;
}

export interface ChartData {
  labels: string[];
  datasets: ChartDataset[];
}

export interface ChartDataset {
  label: string;
  data: number[];
  backgroundColor?: string | string[];
  borderColor?: string | string[];
  borderWidth?: number;
}

export interface ExportRequest {
  workflow_ids: string[];
  format: 'pdf' | 'html' | 'markdown' | 'json' | 'csv';
  include_metadata: boolean;
  include_sources: boolean;
  template_id: string | null;
}
