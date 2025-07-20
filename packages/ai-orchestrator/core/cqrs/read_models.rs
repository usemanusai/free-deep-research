// Read Models for CQRS Query Side
// Phase 4.2: CQRS Pattern Implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::error::{CQRSError, CQRSResult};

/// Research workflow read model optimized for queries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResearchWorkflowReadModel {
    pub id: Uuid,
    pub name: String,
    pub query: String,
    pub methodology: Option<serde_json::Value>,
    pub status: WorkflowStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub results: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub tasks: Vec<TaskReadModel>,
    pub metrics: WorkflowMetrics,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Created,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowMetrics {
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub failed_tasks: u32,
    pub progress_percentage: f64,
    pub estimated_completion_time: Option<DateTime<Utc>>,
    pub actual_duration_minutes: Option<u32>,
}

/// Task read model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskReadModel {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub task_type: String,
    pub agent_type: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub results: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub duration_seconds: Option<u32>,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Created,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Workflow list read model for paginated queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListReadModel {
    pub workflows: Vec<WorkflowSummary>,
    pub total_count: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSummary {
    pub id: Uuid,
    pub name: String,
    pub query: String,
    pub status: WorkflowStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress_percentage: f64,
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub tags: Vec<String>,
}

/// Workflow statistics read model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStatsReadModel {
    pub total_workflows: u64,
    pub workflows_by_status: HashMap<String, u64>,
    pub workflows_by_date: Vec<DateCount>,
    pub average_completion_time_minutes: f64,
    pub success_rate_percentage: f64,
    pub most_common_methodologies: Vec<MethodologyCount>,
    pub task_statistics: TaskStatistics,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateCount {
    pub date: DateTime<Utc>,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyCount {
    pub methodology: String,
    pub count: u64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatistics {
    pub total_tasks: u64,
    pub tasks_by_type: HashMap<String, u64>,
    pub tasks_by_agent: HashMap<String, u64>,
    pub average_task_duration_seconds: f64,
    pub task_success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub workflows_per_hour: f64,
    pub peak_concurrent_workflows: u32,
    pub system_utilization_percentage: f64,
    pub error_rate_percentage: f64,
}

/// Read model store trait for persistence
#[async_trait]
pub trait ReadModelStore: Send + Sync {
    /// Get workflow by ID
    async fn get_workflow(&self, id: Uuid) -> CQRSResult<Option<ResearchWorkflowReadModel>>;
    
    /// Get workflow list with pagination and filtering
    async fn get_workflow_list(
        &self,
        page: u32,
        page_size: u32,
        status_filter: Option<String>,
        search_query: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> CQRSResult<WorkflowListReadModel>;
    
    /// Get workflow statistics
    async fn get_workflow_stats(
        &self,
        date_range_start: Option<DateTime<Utc>>,
        date_range_end: Option<DateTime<Utc>>,
        group_by: Option<String>,
    ) -> CQRSResult<WorkflowStatsReadModel>;
    
    /// Get tasks by workflow ID
    async fn get_tasks_by_workflow(
        &self,
        workflow_id: Uuid,
        status_filter: Option<String>,
    ) -> CQRSResult<Vec<TaskReadModel>>;
    
    /// Search workflows
    async fn search_workflows(
        &self,
        search_term: String,
        page: u32,
        page_size: u32,
        filters: HashMap<String, String>,
    ) -> CQRSResult<WorkflowListReadModel>;
    
    /// Update workflow read model
    async fn update_workflow(&self, workflow: ResearchWorkflowReadModel) -> CQRSResult<()>;
    
    /// Update task read model
    async fn update_task(&self, task: TaskReadModel) -> CQRSResult<()>;
    
    /// Delete workflow read model
    async fn delete_workflow(&self, id: Uuid) -> CQRSResult<()>;
    
    /// Health check
    async fn health_check(&self) -> CQRSResult<()>;
    
    /// Get read model statistics
    async fn get_read_model_stats(&self) -> CQRSResult<ReadModelStats>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadModelStats {
    pub total_workflows: u64,
    pub total_tasks: u64,
    pub last_updated: DateTime<Utc>,
    pub storage_size_bytes: u64,
    pub index_count: u32,
}

/// PostgreSQL implementation of read model store
pub struct PostgresReadModelStore {
    pool: sqlx::PgPool,
}

impl PostgresReadModelStore {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReadModelStore for PostgresReadModelStore {
    async fn get_workflow(&self, id: Uuid) -> CQRSResult<Option<ResearchWorkflowReadModel>> {
        let row = sqlx::query!(
            r#"
            SELECT 
                id, name, query, methodology, status, created_at, started_at, 
                completed_at, updated_at, results, error_message, tags
            FROM workflow_read_models 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        if let Some(row) = row {
            // Get tasks for this workflow
            let tasks = self.get_tasks_by_workflow(id, None).await?;
            
            // Calculate metrics
            let total_tasks = tasks.len() as u32;
            let completed_tasks = tasks.iter().filter(|t| t.status == TaskStatus::Completed).count() as u32;
            let failed_tasks = tasks.iter().filter(|t| t.status == TaskStatus::Failed).count() as u32;
            let progress_percentage = if total_tasks > 0 {
                (completed_tasks as f64 / total_tasks as f64) * 100.0
            } else {
                0.0
            };
            
            let workflow = ResearchWorkflowReadModel {
                id: row.id,
                name: row.name,
                query: row.query,
                methodology: row.methodology,
                status: serde_json::from_str(&row.status).unwrap_or(WorkflowStatus::Created),
                created_at: row.created_at,
                started_at: row.started_at,
                completed_at: row.completed_at,
                updated_at: row.updated_at,
                results: row.results,
                error_message: row.error_message,
                tasks,
                metrics: WorkflowMetrics {
                    total_tasks,
                    completed_tasks,
                    failed_tasks,
                    progress_percentage,
                    estimated_completion_time: None, // Calculate based on current progress
                    actual_duration_minutes: None, // Calculate from start/end times
                },
                tags: row.tags.unwrap_or_default(),
            };
            
            Ok(Some(workflow))
        } else {
            Ok(None)
        }
    }
    
    async fn get_workflow_list(
        &self,
        page: u32,
        page_size: u32,
        status_filter: Option<String>,
        search_query: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> CQRSResult<WorkflowListReadModel> {
        let offset = (page.saturating_sub(1)) * page_size;
        let sort_column = sort_by.as_deref().unwrap_or("created_at");
        let order = sort_order.as_deref().unwrap_or("desc");
        
        // Build dynamic query based on filters
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, name, query, status, created_at, completed_at, tags FROM workflow_read_models WHERE 1=1"
        );
        
        if let Some(status) = &status_filter {
            query_builder.push(" AND status = ");
            query_builder.push_bind(status);
        }
        
        if let Some(search) = &search_query {
            query_builder.push(" AND (name ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(" OR query ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(")");
        }
        
        query_builder.push(format!(" ORDER BY {} {}", sort_column, order));
        query_builder.push(" LIMIT ");
        query_builder.push_bind(page_size as i64);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);
        
        let rows = query_builder
            .build_query_as::<(Uuid, String, String, String, DateTime<Utc>, Option<DateTime<Utc>>, Option<Vec<String>>)>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        // Get total count
        let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM workflow_read_models")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        let workflows: Vec<WorkflowSummary> = rows
            .into_iter()
            .map(|(id, name, query, status, created_at, completed_at, tags)| {
                WorkflowSummary {
                    id,
                    name,
                    query,
                    status: serde_json::from_str(&status).unwrap_or(WorkflowStatus::Created),
                    created_at,
                    completed_at,
                    progress_percentage: 0.0, // Would calculate from tasks
                    total_tasks: 0, // Would get from tasks table
                    completed_tasks: 0, // Would get from tasks table
                    tags: tags.unwrap_or_default(),
                }
            })
            .collect();
        
        let total_pages = ((total_count as u32).saturating_add(page_size - 1)) / page_size;
        
        Ok(WorkflowListReadModel {
            workflows,
            total_count: total_count as u64,
            page,
            page_size,
            total_pages,
            has_next_page: page < total_pages,
            has_previous_page: page > 1,
        })
    }
    
    async fn get_workflow_stats(
        &self,
        _date_range_start: Option<DateTime<Utc>>,
        _date_range_end: Option<DateTime<Utc>>,
        _group_by: Option<String>,
    ) -> CQRSResult<WorkflowStatsReadModel> {
        // Simplified implementation - in production you'd have complex aggregation queries
        let total_workflows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM workflow_read_models")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        Ok(WorkflowStatsReadModel {
            total_workflows: total_workflows as u64,
            workflows_by_status: HashMap::new(),
            workflows_by_date: Vec::new(),
            average_completion_time_minutes: 0.0,
            success_rate_percentage: 0.0,
            most_common_methodologies: Vec::new(),
            task_statistics: TaskStatistics {
                total_tasks: 0,
                tasks_by_type: HashMap::new(),
                tasks_by_agent: HashMap::new(),
                average_task_duration_seconds: 0.0,
                task_success_rate: 0.0,
            },
            performance_metrics: PerformanceMetrics {
                workflows_per_hour: 0.0,
                peak_concurrent_workflows: 0,
                system_utilization_percentage: 0.0,
                error_rate_percentage: 0.0,
            },
        })
    }
    
    async fn get_tasks_by_workflow(
        &self,
        workflow_id: Uuid,
        status_filter: Option<String>,
    ) -> CQRSResult<Vec<TaskReadModel>> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, workflow_id, task_type, agent_type, status, created_at, started_at, completed_at, results, error_message, duration_seconds, retry_count FROM task_read_models WHERE workflow_id = "
        );
        query_builder.push_bind(workflow_id);
        
        if let Some(status) = status_filter {
            query_builder.push(" AND status = ");
            query_builder.push_bind(status);
        }
        
        let rows = query_builder
            .build_query_as::<(Uuid, Uuid, String, Option<String>, String, DateTime<Utc>, Option<DateTime<Utc>>, Option<DateTime<Utc>>, Option<serde_json::Value>, Option<String>, Option<i32>, i32)>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        let tasks = rows
            .into_iter()
            .map(|(id, workflow_id, task_type, agent_type, status, created_at, started_at, completed_at, results, error_message, duration_seconds, retry_count)| {
                TaskReadModel {
                    id,
                    workflow_id,
                    task_type,
                    agent_type,
                    status: serde_json::from_str(&status).unwrap_or(TaskStatus::Created),
                    created_at,
                    started_at,
                    completed_at,
                    results,
                    error_message,
                    duration_seconds: duration_seconds.map(|d| d as u32),
                    retry_count: retry_count as u32,
                }
            })
            .collect();
        
        Ok(tasks)
    }
    
    async fn search_workflows(
        &self,
        search_term: String,
        page: u32,
        page_size: u32,
        _filters: HashMap<String, String>,
    ) -> CQRSResult<WorkflowListReadModel> {
        // Delegate to get_workflow_list with search query
        self.get_workflow_list(page, page_size, None, Some(search_term), None, None).await
    }
    
    async fn update_workflow(&self, workflow: ResearchWorkflowReadModel) -> CQRSResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO workflow_read_models 
            (id, name, query, methodology, status, created_at, started_at, completed_at, updated_at, results, error_message, tags)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                query = EXCLUDED.query,
                methodology = EXCLUDED.methodology,
                status = EXCLUDED.status,
                started_at = EXCLUDED.started_at,
                completed_at = EXCLUDED.completed_at,
                updated_at = EXCLUDED.updated_at,
                results = EXCLUDED.results,
                error_message = EXCLUDED.error_message,
                tags = EXCLUDED.tags
            "#,
            workflow.id,
            workflow.name,
            workflow.query,
            workflow.methodology,
            serde_json::to_string(&workflow.status).unwrap(),
            workflow.created_at,
            workflow.started_at,
            workflow.completed_at,
            workflow.updated_at,
            workflow.results,
            workflow.error_message,
            &workflow.tags
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn update_task(&self, task: TaskReadModel) -> CQRSResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO task_read_models 
            (id, workflow_id, task_type, agent_type, status, created_at, started_at, completed_at, results, error_message, duration_seconds, retry_count)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                task_type = EXCLUDED.task_type,
                agent_type = EXCLUDED.agent_type,
                status = EXCLUDED.status,
                started_at = EXCLUDED.started_at,
                completed_at = EXCLUDED.completed_at,
                results = EXCLUDED.results,
                error_message = EXCLUDED.error_message,
                duration_seconds = EXCLUDED.duration_seconds,
                retry_count = EXCLUDED.retry_count
            "#,
            task.id,
            task.workflow_id,
            task.task_type,
            task.agent_type,
            serde_json::to_string(&task.status).unwrap(),
            task.created_at,
            task.started_at,
            task.completed_at,
            task.results,
            task.error_message,
            task.duration_seconds.map(|d| d as i32),
            task.retry_count as i32
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn delete_workflow(&self, id: Uuid) -> CQRSResult<()> {
        sqlx::query!("DELETE FROM workflow_read_models WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn health_check(&self) -> CQRSResult<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_read_model_stats(&self) -> CQRSResult<ReadModelStats> {
        let total_workflows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM workflow_read_models")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        let total_tasks: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM task_read_models")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| CQRSError::DatabaseError(e.to_string()))?;
        
        Ok(ReadModelStats {
            total_workflows: total_workflows as u64,
            total_tasks: total_tasks as u64,
            last_updated: Utc::now(),
            storage_size_bytes: 0, // Would calculate actual storage size
            index_count: 0, // Would count actual indexes
        })
    }
}

/// Mock read model store for testing
#[cfg(test)]
pub struct MockReadModelStore {
    workflows: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, ResearchWorkflowReadModel>>>,
    tasks: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, TaskReadModel>>>,
}

#[cfg(test)]
impl MockReadModelStore {
    pub fn new() -> Self {
        Self {
            workflows: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            tasks: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[cfg(test)]
#[async_trait]
impl ReadModelStore for MockReadModelStore {
    async fn get_workflow(&self, id: Uuid) -> CQRSResult<Option<ResearchWorkflowReadModel>> {
        let workflows = self.workflows.read().await;
        Ok(workflows.get(&id).cloned())
    }
    
    async fn get_workflow_list(
        &self,
        _page: u32,
        _page_size: u32,
        _status_filter: Option<String>,
        _search_query: Option<String>,
        _sort_by: Option<String>,
        _sort_order: Option<String>,
    ) -> CQRSResult<WorkflowListReadModel> {
        Ok(WorkflowListReadModel {
            workflows: Vec::new(),
            total_count: 0,
            page: 1,
            page_size: 10,
            total_pages: 0,
            has_next_page: false,
            has_previous_page: false,
        })
    }
    
    async fn get_workflow_stats(
        &self,
        _date_range_start: Option<DateTime<Utc>>,
        _date_range_end: Option<DateTime<Utc>>,
        _group_by: Option<String>,
    ) -> CQRSResult<WorkflowStatsReadModel> {
        Ok(WorkflowStatsReadModel {
            total_workflows: 0,
            workflows_by_status: HashMap::new(),
            workflows_by_date: Vec::new(),
            average_completion_time_minutes: 0.0,
            success_rate_percentage: 0.0,
            most_common_methodologies: Vec::new(),
            task_statistics: TaskStatistics {
                total_tasks: 0,
                tasks_by_type: HashMap::new(),
                tasks_by_agent: HashMap::new(),
                average_task_duration_seconds: 0.0,
                task_success_rate: 0.0,
            },
            performance_metrics: PerformanceMetrics {
                workflows_per_hour: 0.0,
                peak_concurrent_workflows: 0,
                system_utilization_percentage: 0.0,
                error_rate_percentage: 0.0,
            },
        })
    }
    
    async fn get_tasks_by_workflow(
        &self,
        workflow_id: Uuid,
        _status_filter: Option<String>,
    ) -> CQRSResult<Vec<TaskReadModel>> {
        let tasks = self.tasks.read().await;
        let workflow_tasks: Vec<TaskReadModel> = tasks
            .values()
            .filter(|task| task.workflow_id == workflow_id)
            .cloned()
            .collect();
        Ok(workflow_tasks)
    }
    
    async fn search_workflows(
        &self,
        _search_term: String,
        _page: u32,
        _page_size: u32,
        _filters: HashMap<String, String>,
    ) -> CQRSResult<WorkflowListReadModel> {
        self.get_workflow_list(1, 10, None, None, None, None).await
    }
    
    async fn update_workflow(&self, workflow: ResearchWorkflowReadModel) -> CQRSResult<()> {
        let mut workflows = self.workflows.write().await;
        workflows.insert(workflow.id, workflow);
        Ok(())
    }
    
    async fn update_task(&self, task: TaskReadModel) -> CQRSResult<()> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id, task);
        Ok(())
    }
    
    async fn delete_workflow(&self, id: Uuid) -> CQRSResult<()> {
        let mut workflows = self.workflows.write().await;
        workflows.remove(&id);
        Ok(())
    }
    
    async fn health_check(&self) -> CQRSResult<()> {
        Ok(())
    }
    
    async fn get_read_model_stats(&self) -> CQRSResult<ReadModelStats> {
        let workflows = self.workflows.read().await;
        let tasks = self.tasks.read().await;
        
        Ok(ReadModelStats {
            total_workflows: workflows.len() as u64,
            total_tasks: tasks.len() as u64,
            last_updated: Utc::now(),
            storage_size_bytes: 0,
            index_count: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_read_model_creation() {
        let workflow = ResearchWorkflowReadModel {
            id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            query: "Test Query".to_string(),
            methodology: None,
            status: WorkflowStatus::Created,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            updated_at: Utc::now(),
            results: None,
            error_message: None,
            tasks: Vec::new(),
            metrics: WorkflowMetrics {
                total_tasks: 0,
                completed_tasks: 0,
                failed_tasks: 0,
                progress_percentage: 0.0,
                estimated_completion_time: None,
                actual_duration_minutes: None,
            },
            tags: Vec::new(),
        };
        
        assert_eq!(workflow.status, WorkflowStatus::Created);
        assert_eq!(workflow.metrics.progress_percentage, 0.0);
    }

    #[tokio::test]
    async fn test_mock_read_model_store() {
        let store = MockReadModelStore::new();
        
        // Test health check
        assert!(store.health_check().await.is_ok());
        
        // Test stats
        let stats = store.get_read_model_stats().await.unwrap();
        assert_eq!(stats.total_workflows, 0);
        assert_eq!(stats.total_tasks, 0);
    }
}
