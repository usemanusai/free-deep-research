use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex, Semaphore};
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use rand;

use crate::error::{AppResult, BackgroundProcessingError};
use crate::services::Service;

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Background task definition
#[derive(Debug, Clone)]
pub struct BackgroundTask {
    pub id: Uuid,
    pub name: String,
    pub priority: TaskPriority,
    pub created_at: DateTime<Utc>,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub max_retries: u32,
    pub retry_count: u32,
    pub timeout_seconds: Option<u64>,
    pub task_data: serde_json::Value,
    pub task_type: String,
}

/// Task execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub success: bool,
    pub result_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
    pub completed_at: DateTime<Utc>,
}

/// Background processing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundProcessingStatistics {
    pub total_tasks_processed: u64,
    pub tasks_in_queue: usize,
    pub active_tasks: usize,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub average_processing_time_ms: f64,
    pub success_rate: f64,
    pub queue_processing_rate: f64,
    pub worker_utilization: f64,
}

/// Background processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundProcessorConfig {
    pub max_concurrent_tasks: usize,
    pub max_queue_size: usize,
    pub worker_count: usize,
    pub task_timeout_seconds: u64,
    pub retry_delay_seconds: u64,
    pub enable_scheduling: bool,
    pub enable_statistics: bool,
}

/// Task execution context
pub struct TaskExecutionContext {
    pub task: BackgroundTask,
    pub processor: Arc<BackgroundProcessor>,
}

/// Background processor for handling long-running tasks
pub struct BackgroundProcessor {
    // Task queue
    task_queue: Arc<Mutex<VecDeque<BackgroundTask>>>,
    
    // Active tasks
    active_tasks: Arc<RwLock<Vec<BackgroundTask>>>,
    
    // Completed tasks history
    completed_tasks: Arc<RwLock<Vec<TaskResult>>>,
    
    // Configuration
    config: Arc<RwLock<BackgroundProcessorConfig>>,
    
    // Statistics
    statistics: Arc<RwLock<BackgroundProcessingStatistics>>,
    
    // Concurrency control
    semaphore: Arc<Semaphore>,
    
    // Worker tasks
    workers: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    
    // Shutdown signal
    shutdown_signal: Arc<RwLock<bool>>,
}

impl BackgroundProcessor {
    /// Create a new background processor
    pub async fn new() -> AppResult<Self> {
        info!("Initializing background processor...");

        let config = BackgroundProcessorConfig {
            max_concurrent_tasks: 10,
            max_queue_size: 1000,
            worker_count: 4,
            task_timeout_seconds: 300, // 5 minutes
            retry_delay_seconds: 30,
            enable_scheduling: true,
            enable_statistics: true,
        };

        let statistics = BackgroundProcessingStatistics {
            total_tasks_processed: 0,
            tasks_in_queue: 0,
            active_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            average_processing_time_ms: 0.0,
            success_rate: 0.0,
            queue_processing_rate: 0.0,
            worker_utilization: 0.0,
        };

        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));

        let processor = Self {
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            active_tasks: Arc::new(RwLock::new(Vec::new())),
            completed_tasks: Arc::new(RwLock::new(Vec::new())),
            config: Arc::new(RwLock::new(config)),
            statistics: Arc::new(RwLock::new(statistics)),
            semaphore,
            workers: Arc::new(RwLock::new(Vec::new())),
            shutdown_signal: Arc::new(RwLock::new(false)),
        };

        // Start worker tasks
        processor.start_workers().await?;

        info!("Background processor initialized successfully");
        Ok(processor)
    }

    /// Submit a task for background processing
    pub async fn submit_task(&self, mut task: BackgroundTask) -> AppResult<Uuid> {
        let config = self.config.read().await;
        
        // Check queue size
        let queue_size = {
            let queue = self.task_queue.lock().await;
            queue.len()
        };
        
        if queue_size >= config.max_queue_size {
            return Err(BackgroundProcessingError::queue_full(
                format!("Queue is full ({}/{})", queue_size, config.max_queue_size)
            ).into());
        }
        drop(config);

        // Generate task ID if not provided
        if task.id == Uuid::nil() {
            task.id = Uuid::new_v4();
        }

        debug!("Submitting background task: {} ({})", task.name, task.id);

        // Add to queue
        let mut queue = self.task_queue.lock().await;
        
        // Insert based on priority (higher priority first)
        let insert_position = queue
            .iter()
            .position(|existing_task| existing_task.priority < task.priority)
            .unwrap_or(queue.len());
        
        queue.insert(insert_position, task.clone());
        drop(queue);

        // Update statistics
        self.update_statistics_on_submit().await;

        info!("Background task submitted: {} ({})", task.name, task.id);
        Ok(task.id)
    }

    /// Get task status
    pub async fn get_task_status(&self, task_id: Uuid) -> AppResult<Option<TaskStatus>> {
        // Check active tasks
        let active_tasks = self.active_tasks.read().await;
        if let Some(task) = active_tasks.iter().find(|t| t.id == task_id) {
            return Ok(Some(TaskStatus::Running(task.clone())));
        }
        drop(active_tasks);

        // Check completed tasks
        let completed_tasks = self.completed_tasks.read().await;
        if let Some(result) = completed_tasks.iter().find(|r| r.task_id == task_id) {
            return Ok(Some(TaskStatus::Completed(result.clone())));
        }
        drop(completed_tasks);

        // Check queued tasks
        let queue = self.task_queue.lock().await;
        if let Some(task) = queue.iter().find(|t| t.id == task_id) {
            return Ok(Some(TaskStatus::Queued(task.clone())));
        }
        drop(queue);

        Ok(None)
    }

    /// Get processing statistics
    pub async fn get_statistics(&self) -> BackgroundProcessingStatistics {
        let mut statistics = self.statistics.read().await;
        
        // Update real-time statistics
        let queue_size = {
            let queue = self.task_queue.lock().await;
            queue.len()
        };
        
        let active_count = {
            let active_tasks = self.active_tasks.read().await;
            active_tasks.len()
        };

        let mut updated_stats = statistics.clone();
        updated_stats.tasks_in_queue = queue_size;
        updated_stats.active_tasks = active_count;
        
        // Calculate worker utilization
        let config = self.config.read().await;
        updated_stats.worker_utilization = active_count as f64 / config.max_concurrent_tasks as f64;
        drop(config);

        updated_stats
    }

    /// Cancel a task
    pub async fn cancel_task(&self, task_id: Uuid) -> AppResult<bool> {
        debug!("Attempting to cancel task: {}", task_id);

        // Try to remove from queue first
        let mut queue = self.task_queue.lock().await;
        if let Some(pos) = queue.iter().position(|t| t.id == task_id) {
            queue.remove(pos);
            drop(queue);
            info!("Task cancelled from queue: {}", task_id);
            return Ok(true);
        }
        drop(queue);

        // If task is active, it cannot be cancelled (would need more complex cancellation logic)
        let active_tasks = self.active_tasks.read().await;
        if active_tasks.iter().any(|t| t.id == task_id) {
            drop(active_tasks);
            warn!("Cannot cancel active task: {}", task_id);
            return Ok(false);
        }
        drop(active_tasks);

        debug!("Task not found for cancellation: {}", task_id);
        Ok(false)
    }

    /// Start worker tasks
    async fn start_workers(&self) -> AppResult<()> {
        let config = self.config.read().await;
        let worker_count = config.worker_count;
        drop(config);

        let mut workers = self.workers.write().await;
        
        for worker_id in 0..worker_count {
            let worker_task = self.create_worker_task(worker_id).await;
            workers.push(worker_task);
        }
        
        drop(workers);
        info!("Started {} background workers", worker_count);
        Ok(())
    }

    /// Create a worker task
    async fn create_worker_task(&self, worker_id: usize) -> tokio::task::JoinHandle<()> {
        let task_queue = self.task_queue.clone();
        let active_tasks = self.active_tasks.clone();
        let completed_tasks = self.completed_tasks.clone();
        let config = self.config.clone();
        let statistics = self.statistics.clone();
        let semaphore = self.semaphore.clone();
        let shutdown_signal = self.shutdown_signal.clone();

        tokio::spawn(async move {
            debug!("Background worker {} started", worker_id);

            loop {
                // Check shutdown signal
                {
                    let shutdown = shutdown_signal.read().await;
                    if *shutdown {
                        break;
                    }
                }

                // Try to get a task from the queue
                let task = {
                    let mut queue = task_queue.lock().await;
                    queue.pop_front()
                };

                if let Some(task) = task {
                    // Acquire semaphore permit
                    let permit = semaphore.acquire().await;
                    if permit.is_err() {
                        error!("Failed to acquire semaphore permit for worker {}", worker_id);
                        continue;
                    }

                    // Move task to active tasks
                    {
                        let mut active = active_tasks.write().await;
                        active.push(task.clone());
                    }

                    debug!("Worker {} processing task: {} ({})", worker_id, task.name, task.id);

                    // Execute the task
                    let start_time = std::time::Instant::now();
                    let result = Self::execute_task(task.clone()).await;
                    let execution_time = start_time.elapsed();

                    // Remove from active tasks
                    {
                        let mut active = active_tasks.write().await;
                        active.retain(|t| t.id != task.id);
                    }

                    // Create task result
                    let task_result = TaskResult {
                        task_id: task.id,
                        success: result.is_ok(),
                        result_data: result.as_ref().ok().cloned(),
                        error_message: result.as_ref().err().map(|e| e.to_string()),
                        execution_time_ms: execution_time.as_millis() as u64,
                        completed_at: Utc::now(),
                    };

                    // Store completed task
                    {
                        let mut completed = completed_tasks.write().await;
                        completed.push(task_result.clone());
                        
                        // Keep only recent completed tasks (last 1000)
                        if completed.len() > 1000 {
                            completed.remove(0);
                        }
                    }

                    // Update statistics
                    {
                        let mut stats = statistics.write().await;
                        stats.total_tasks_processed += 1;
                        
                        if task_result.success {
                            stats.completed_tasks += 1;
                        } else {
                            stats.failed_tasks += 1;
                        }
                        
                        stats.success_rate = stats.completed_tasks as f64 / stats.total_tasks_processed as f64;
                        
                        // Update average processing time
                        let total_time = stats.average_processing_time_ms * (stats.total_tasks_processed - 1) as f64;
                        stats.average_processing_time_ms = (total_time + task_result.execution_time_ms as f64) / stats.total_tasks_processed as f64;
                    }

                    debug!("Worker {} completed task: {} (success: {}, time: {:?})", 
                           worker_id, task.name, task_result.success, execution_time);

                    // Release semaphore permit
                    drop(permit);
                } else {
                    // No tasks available, wait a bit
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }

            debug!("Background worker {} stopped", worker_id);
        })
    }

    /// Execute a task (mock implementation)
    async fn execute_task(task: BackgroundTask) -> AppResult<serde_json::Value> {
        // This is a mock implementation
        // In a real system, this would dispatch to specific task handlers based on task_type
        
        debug!("Executing task: {} (type: {})", task.name, task.task_type);
        
        // Simulate task execution time
        let execution_time = match task.priority {
            TaskPriority::Critical => 100,
            TaskPriority::High => 500,
            TaskPriority::Normal => 1000,
            TaskPriority::Low => 2000,
        };
        
        tokio::time::sleep(tokio::time::Duration::from_millis(execution_time)).await;
        
        // Mock success/failure (90% success rate)
        if rand::random::<f64>() < 0.9 {
            Ok(serde_json::json!({
                "status": "completed",
                "task_id": task.id,
                "result": "Task executed successfully",
                "execution_time_ms": execution_time
            }))
        } else {
            Err(BackgroundProcessingError::task_execution_failed(
                format!("Mock task failure for task: {}", task.name)
            ).into())
        }
    }

    /// Update statistics on task submission
    async fn update_statistics_on_submit(&self) {
        let mut statistics = self.statistics.write().await;
        // Statistics are updated in real-time by get_statistics()
        drop(statistics);
    }
}

/// Task status enumeration
#[derive(Debug, Clone)]
pub enum TaskStatus {
    Queued(BackgroundTask),
    Running(BackgroundTask),
    Completed(TaskResult),
}

#[async_trait::async_trait]
impl Service for BackgroundProcessor {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing background processor health check");
        
        // Check if workers are running
        let workers = self.workers.read().await;
        let mut active_workers = 0;
        
        for worker in workers.iter() {
            if !worker.is_finished() {
                active_workers += 1;
            }
        }
        
        let config = self.config.read().await;
        if active_workers < config.worker_count {
            warn!("Some background workers have stopped ({}/{})", active_workers, config.worker_count);
        }
        drop(config);
        drop(workers);

        // Check queue size
        let queue_size = {
            let queue = self.task_queue.lock().await;
            queue.len()
        };
        
        let config = self.config.read().await;
        if queue_size > config.max_queue_size / 2 {
            warn!("Background task queue is getting full ({}/{})", queue_size, config.max_queue_size);
        }
        drop(config);

        debug!("Background processor health check completed successfully");
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down background processor...");
        
        // Set shutdown signal
        {
            let mut shutdown = self.shutdown_signal.write().await;
            *shutdown = true;
        }

        // Wait for workers to finish
        let mut workers = self.workers.write().await;
        for worker in workers.drain(..) {
            worker.abort();
        }
        drop(workers);

        info!("Background processor shutdown completed");
        Ok(())
    }
}
