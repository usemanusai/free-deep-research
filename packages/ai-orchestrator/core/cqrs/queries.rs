// Query Side Implementation for CQRS
// Phase 4.2: CQRS Pattern Implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

use super::error::{CQRSError, CQRSResult};
use super::read_models::{ResearchWorkflowReadModel, WorkflowListReadModel, WorkflowStatsReadModel, TaskReadModel};

/// Base trait for all queries
pub trait Query: Send + Sync + std::fmt::Debug {
    type Result: Send + Sync;
    
    /// Validate the query
    fn validate(&self) -> CQRSResult<()> {
        Ok(())
    }
    
    /// Get query name for logging/metrics
    fn query_name(&self) -> &'static str;
    
    /// Get query ID for tracking
    fn query_id(&self) -> Uuid;
    
    /// Get correlation ID for tracing
    fn correlation_id(&self) -> Option<Uuid> {
        None
    }
    
    /// Check if query result can be cached
    fn is_cacheable(&self) -> bool {
        true
    }
    
    /// Get cache key for this query
    fn cache_key(&self) -> String {
        format!("{}:{}", self.query_name(), self.query_id())
    }
    
    /// Get cache TTL in seconds
    fn cache_ttl_seconds(&self) -> u64 {
        300 // 5 minutes default
    }
}

/// Query handler trait
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// Handle the query
    async fn handle(&self, query: Q) -> CQRSResult<Q::Result>;
    
    /// Get handler name for logging
    fn handler_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Query execution result wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult<T> {
    pub query_id: Uuid,
    pub result: T,
    pub executed_at: DateTime<Utc>,
    pub execution_time_ms: u64,
    pub from_cache: bool,
}

impl<T> QueryResult<T> {
    pub fn new(query_id: Uuid, result: T, execution_time_ms: u64, from_cache: bool) -> Self {
        Self {
            query_id,
            result,
            executed_at: Utc::now(),
            execution_time_ms,
            from_cache,
        }
    }
}

/// Research workflow queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResearchWorkflowQuery {
    pub query_id: Uuid,
    pub workflow_id: Uuid,
    pub include_tasks: bool,
    pub correlation_id: Option<Uuid>,
}

impl Query for GetResearchWorkflowQuery {
    type Result = Option<ResearchWorkflowReadModel>;
    
    fn query_name(&self) -> &'static str {
        "GetResearchWorkflow"
    }
    
    fn query_id(&self) -> Uuid {
        self.query_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
    
    fn cache_key(&self) -> String {
        format!("workflow:{}:tasks:{}", self.workflow_id, self.include_tasks)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorkflowListQuery {
    pub query_id: Uuid,
    pub page: u32,
    pub page_size: u32,
    pub status_filter: Option<String>,
    pub search_query: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub correlation_id: Option<Uuid>,
}

impl Query for GetWorkflowListQuery {
    type Result = WorkflowListReadModel;
    
    fn validate(&self) -> CQRSResult<()> {
        if self.page_size == 0 || self.page_size > 1000 {
            return Err(CQRSError::ValidationError("Page size must be between 1 and 1000".to_string()));
        }
        Ok(())
    }
    
    fn query_name(&self) -> &'static str {
        "GetWorkflowList"
    }
    
    fn query_id(&self) -> Uuid {
        self.query_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
    
    fn cache_key(&self) -> String {
        format!(
            "workflow_list:{}:{}:{}:{}:{}:{}",
            self.page,
            self.page_size,
            self.status_filter.as_deref().unwrap_or("all"),
            self.search_query.as_deref().unwrap_or(""),
            self.sort_by.as_deref().unwrap_or("created_at"),
            self.sort_order.as_deref().unwrap_or("desc")
        )
    }
    
    fn cache_ttl_seconds(&self) -> u64 {
        60 // 1 minute for list queries
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorkflowStatsQuery {
    pub query_id: Uuid,
    pub date_range_start: Option<DateTime<Utc>>,
    pub date_range_end: Option<DateTime<Utc>>,
    pub group_by: Option<String>, // "day", "week", "month"
    pub correlation_id: Option<Uuid>,
}

impl Query for GetWorkflowStatsQuery {
    type Result = WorkflowStatsReadModel;
    
    fn query_name(&self) -> &'static str {
        "GetWorkflowStats"
    }
    
    fn query_id(&self) -> Uuid {
        self.query_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
    
    fn cache_key(&self) -> String {
        format!(
            "workflow_stats:{}:{}:{}",
            self.date_range_start.map(|d| d.timestamp()).unwrap_or(0),
            self.date_range_end.map(|d| d.timestamp()).unwrap_or(0),
            self.group_by.as_deref().unwrap_or("day")
        )
    }
    
    fn cache_ttl_seconds(&self) -> u64 {
        600 // 10 minutes for stats
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTasksByWorkflowQuery {
    pub query_id: Uuid,
    pub workflow_id: Uuid,
    pub status_filter: Option<String>,
    pub correlation_id: Option<Uuid>,
}

impl Query for GetTasksByWorkflowQuery {
    type Result = Vec<TaskReadModel>;
    
    fn query_name(&self) -> &'static str {
        "GetTasksByWorkflow"
    }
    
    fn query_id(&self) -> Uuid {
        self.query_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
    
    fn cache_key(&self) -> String {
        format!(
            "tasks:workflow:{}:status:{}",
            self.workflow_id,
            self.status_filter.as_deref().unwrap_or("all")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchWorkflowsQuery {
    pub query_id: Uuid,
    pub search_term: String,
    pub page: u32,
    pub page_size: u32,
    pub filters: HashMap<String, String>,
    pub correlation_id: Option<Uuid>,
}

impl Query for SearchWorkflowsQuery {
    type Result = WorkflowListReadModel;
    
    fn validate(&self) -> CQRSResult<()> {
        if self.search_term.trim().is_empty() {
            return Err(CQRSError::ValidationError("Search term cannot be empty".to_string()));
        }
        if self.page_size == 0 || self.page_size > 1000 {
            return Err(CQRSError::ValidationError("Page size must be between 1 and 1000".to_string()));
        }
        Ok(())
    }
    
    fn query_name(&self) -> &'static str {
        "SearchWorkflows"
    }
    
    fn query_id(&self) -> Uuid {
        self.query_id
    }
    
    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
    
    fn cache_key(&self) -> String {
        let filters_str = self.filters
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join(",");
        
        format!(
            "search:{}:{}:{}:{}",
            self.search_term,
            self.page,
            self.page_size,
            filters_str
        )
    }
    
    fn cache_ttl_seconds(&self) -> u64 {
        120 // 2 minutes for search results
    }
}

/// Query bus for routing queries to handlers
pub struct QueryBus {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    cache: Arc<tokio::sync::RwLock<QueryCache>>,
    metrics: QueryBusMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct QueryBusMetrics {
    pub queries_executed: u64,
    pub queries_failed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub average_duration_ms: f64,
    pub total_duration_ms: u64,
}

/// Simple in-memory query cache
pub struct QueryCache {
    cache: HashMap<String, CacheEntry>,
    max_size: usize,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    data: serde_json::Value,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl QueryCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<serde_json::Value> {
        if let Some(entry) = self.cache.get(key) {
            if entry.expires_at > Utc::now() {
                return Some(entry.data.clone());
            }
        }
        None
    }
    
    pub fn set(&mut self, key: String, data: serde_json::Value, ttl_seconds: u64) {
        // Evict expired entries
        self.evict_expired();
        
        // Evict oldest entries if cache is full
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }
        
        let entry = CacheEntry {
            data,
            expires_at: Utc::now() + chrono::Duration::seconds(ttl_seconds as i64),
            created_at: Utc::now(),
        };
        
        self.cache.insert(key, entry);
    }
    
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    
    fn evict_expired(&mut self) {
        let now = Utc::now();
        self.cache.retain(|_, entry| entry.expires_at > now);
    }
    
    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self.cache
            .iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
        {
            self.cache.remove(&oldest_key);
        }
    }
    
    pub fn stats(&self) -> (usize, usize) {
        (self.cache.len(), self.max_size)
    }
}

impl QueryBus {
    pub fn new() -> Self {
        Self::with_cache_size(10000)
    }
    
    pub fn with_cache_size(cache_size: usize) -> Self {
        Self {
            handlers: HashMap::new(),
            cache: Arc::new(tokio::sync::RwLock::new(QueryCache::new(cache_size))),
            metrics: QueryBusMetrics::default(),
        }
    }
    
    /// Register a query handler
    pub async fn register_handler<Q, H>(&mut self, handler: H)
    where
        Q: Query + 'static,
        H: QueryHandler<Q> + 'static,
    {
        let type_id = TypeId::of::<Q>();
        self.handlers.insert(type_id, Box::new(Arc::new(handler)));
    }
    
    /// Execute a query
    pub async fn execute<Q>(&self, query: Q) -> CQRSResult<Q::Result>
    where
        Q: Query + 'static,
        Q::Result: for<'de> Deserialize<'de> + Serialize,
    {
        let start_time = Instant::now();
        let query_id = query.query_id();
        let query_name = query.query_name();
        
        // Check cache first if query is cacheable
        if query.is_cacheable() {
            let cache_key = query.cache_key();
            let cache = self.cache.read().await;
            
            if let Some(cached_data) = cache.get(&cache_key) {
                if let Ok(result) = serde_json::from_value::<Q::Result>(cached_data) {
                    // Update cache hit metrics
                    self.update_metrics(true, start_time.elapsed().as_millis() as u64, true).await;
                    return Ok(result);
                }
            }
        }
        
        // Find handler
        let type_id = TypeId::of::<Q>();
        let handler = self.handlers.get(&type_id)
            .ok_or_else(|| CQRSError::HandlerNotFound(query_name.to_string()))?;
        
        // Cast handler to correct type
        let handler = handler
            .downcast_ref::<Arc<dyn QueryHandler<Q>>>()
            .ok_or_else(|| CQRSError::HandlerCastError(query_name.to_string()))?;
        
        // Execute query
        let result = handler.handle(query).await;
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        match result {
            Ok(result) => {
                // Cache result if query is cacheable
                if query.is_cacheable() {
                    if let Ok(serialized) = serde_json::to_value(&result) {
                        let mut cache = self.cache.write().await;
                        cache.set(query.cache_key(), serialized, query.cache_ttl_seconds());
                    }
                }
                
                // Update metrics
                self.update_metrics(true, execution_time, false).await;
                
                Ok(result)
            }
            Err(error) => {
                // Update metrics
                self.update_metrics(false, execution_time, false).await;
                
                Err(error)
            }
        }
    }
    
    /// Update query bus metrics
    async fn update_metrics(&self, success: bool, execution_time_ms: u64, cache_hit: bool) {
        // In a real implementation, you'd use atomic operations or a proper metrics system
        // For now, this is a placeholder
    }
    
    /// Get query bus metrics
    pub fn get_metrics(&self) -> &QueryBusMetrics {
        &self.metrics
    }
    
    /// Check if query bus is healthy
    pub async fn is_healthy(&self) -> bool {
        !self.handlers.is_empty()
    }
    
    /// Clear query cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
    
    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.read().await;
        cache.stats()
    }
}

impl Default for QueryBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Query factory for creating queries with proper IDs
pub struct QueryFactory {
    correlation_id: Option<Uuid>,
}

impl QueryFactory {
    pub fn new() -> Self {
        Self {
            correlation_id: None,
        }
    }
    
    pub fn with_correlation_id(correlation_id: Uuid) -> Self {
        Self {
            correlation_id: Some(correlation_id),
        }
    }
    
    pub fn get_research_workflow(
        &self,
        workflow_id: Uuid,
        include_tasks: bool,
    ) -> GetResearchWorkflowQuery {
        GetResearchWorkflowQuery {
            query_id: Uuid::new_v4(),
            workflow_id,
            include_tasks,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn get_workflow_list(
        &self,
        page: u32,
        page_size: u32,
        status_filter: Option<String>,
        search_query: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> GetWorkflowListQuery {
        GetWorkflowListQuery {
            query_id: Uuid::new_v4(),
            page,
            page_size,
            status_filter,
            search_query,
            sort_by,
            sort_order,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn get_workflow_stats(
        &self,
        date_range_start: Option<DateTime<Utc>>,
        date_range_end: Option<DateTime<Utc>>,
        group_by: Option<String>,
    ) -> GetWorkflowStatsQuery {
        GetWorkflowStatsQuery {
            query_id: Uuid::new_v4(),
            date_range_start,
            date_range_end,
            group_by,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn get_tasks_by_workflow(
        &self,
        workflow_id: Uuid,
        status_filter: Option<String>,
    ) -> GetTasksByWorkflowQuery {
        GetTasksByWorkflowQuery {
            query_id: Uuid::new_v4(),
            workflow_id,
            status_filter,
            correlation_id: self.correlation_id,
        }
    }
    
    pub fn search_workflows(
        &self,
        search_term: String,
        page: u32,
        page_size: u32,
        filters: HashMap<String, String>,
    ) -> SearchWorkflowsQuery {
        SearchWorkflowsQuery {
            query_id: Uuid::new_v4(),
            search_term,
            page,
            page_size,
            filters,
            correlation_id: self.correlation_id,
        }
    }
}

impl Default for QueryFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_creation() {
        let factory = QueryFactory::new();
        let query = factory.get_research_workflow(Uuid::new_v4(), true);
        
        assert_eq!(query.query_name(), "GetResearchWorkflow");
        assert!(query.validate().is_ok());
        assert!(query.is_cacheable());
    }

    #[test]
    fn test_query_validation() {
        let factory = QueryFactory::new();
        
        // Valid query
        let valid_query = factory.get_workflow_list(1, 50, None, None, None, None);
        assert!(valid_query.validate().is_ok());
        
        // Invalid query - page size too large
        let invalid_query = factory.get_workflow_list(1, 2000, None, None, None, None);
        assert!(invalid_query.validate().is_err());
    }

    #[tokio::test]
    async fn test_query_cache() {
        let mut cache = QueryCache::new(100);
        let data = serde_json::json!({"test": "value"});
        
        // Set and get
        cache.set("test_key".to_string(), data.clone(), 60);
        let retrieved = cache.get("test_key");
        assert_eq!(retrieved, Some(data));
        
        // Test expiration (would need to mock time in real implementation)
        let stats = cache.stats();
        assert_eq!(stats.0, 1); // 1 item in cache
        assert_eq!(stats.1, 100); // max size 100
    }

    #[tokio::test]
    async fn test_query_bus_creation() {
        let query_bus = QueryBus::new();
        assert!(query_bus.is_healthy().await);
        
        let cache_stats = query_bus.get_cache_stats().await;
        assert_eq!(cache_stats.0, 0); // Empty cache
    }
}
