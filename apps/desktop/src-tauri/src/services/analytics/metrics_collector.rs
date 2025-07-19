use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc, Duration};

use crate::error::{AppError, AppResult};
use super::{AnalyticsEvent, EventType};

/// Metrics collector for gathering and storing analytics events
#[derive(Clone)]
pub struct MetricsCollector {
    data_persistence: Arc<RwLock<crate::services::data_persistence::DataPersistenceService>>,
    event_buffer: Arc<RwLock<VecDeque<AnalyticsEvent>>>,
    config: MetricsCollectorConfig,
    collection_stats: Arc<RwLock<CollectionStats>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub async fn new(
        data_persistence: Arc<RwLock<crate::services::data_persistence::DataPersistenceService>>,
    ) -> AppResult<Self> {
        info!("Initializing metrics collector...");

        let config = MetricsCollectorConfig::default();
        let event_buffer = Arc::new(RwLock::new(VecDeque::new()));
        let collection_stats = Arc::new(RwLock::new(CollectionStats::default()));

        let collector = Self {
            data_persistence,
            event_buffer,
            config,
            collection_stats,
        };

        info!("Metrics collector initialized successfully");
        Ok(collector)
    }

    /// Start metrics collection
    pub async fn start_collection(&self) -> AppResult<()> {
        info!("Starting metrics collection...");

        // Start background processing tasks
        let collector_clone = self.clone();
        tokio::spawn(async move {
            collector_clone.process_event_buffer().await;
        });

        let collector_clone = self.clone();
        tokio::spawn(async move {
            collector_clone.cleanup_old_events().await;
        });

        let collector_clone = self.clone();
        tokio::spawn(async move {
            collector_clone.generate_collection_reports().await;
        });

        info!("Metrics collection started successfully");
        Ok(())
    }

    /// Record an analytics event
    pub async fn record_event(&self, event: AnalyticsEvent) -> AppResult<()> {
        // Add to buffer for processing
        {
            let mut buffer = self.event_buffer.write().await;
            buffer.push_back(event.clone());

            // Prevent buffer overflow
            if buffer.len() > self.config.max_buffer_size {
                buffer.pop_front();
                warn!("Event buffer overflow, dropping oldest event");
            }
        }

        // Update collection stats
        {
            let mut stats = self.collection_stats.write().await;
            stats.total_events_collected += 1;
            stats.events_by_type.entry(event.event_type.clone()).and_modify(|e| *e += 1).or_insert(1);
            stats.last_event_time = Some(event.timestamp);
        }

        Ok(())
    }

    /// Get events by type and time range
    pub async fn get_events_by_type_and_time(
        &self,
        event_type: EventType,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> AppResult<Vec<AnalyticsEvent>> {
        // First check buffer for recent events
        let mut events = Vec::new();
        {
            let buffer = self.event_buffer.read().await;
            for event in buffer.iter() {
                if std::mem::discriminant(&event.event_type) == std::mem::discriminant(&event_type)
                    && event.timestamp >= start_time
                    && event.timestamp <= end_time
                {
                    events.push(event.clone());
                }
            }
        }

        // Then get events from persistent storage
        let stored_events = self.get_stored_events_by_type_and_time(event_type, start_time, end_time).await?;
        events.extend(stored_events);

        // Remove duplicates and sort by timestamp
        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        events.dedup_by(|a, b| a.timestamp == b.timestamp && a.session_id == b.session_id);

        Ok(events)
    }

    /// Get events by time range
    pub async fn get_events_by_time(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> AppResult<Vec<AnalyticsEvent>> {
        // Check buffer for recent events
        let mut events = Vec::new();
        {
            let buffer = self.event_buffer.read().await;
            for event in buffer.iter() {
                if event.timestamp >= start_time && event.timestamp <= end_time {
                    events.push(event.clone());
                }
            }
        }

        // Get events from persistent storage
        let stored_events = self.get_stored_events_by_time(start_time, end_time).await?;
        events.extend(stored_events);

        // Remove duplicates and sort by timestamp
        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        events.dedup_by(|a, b| a.timestamp == b.timestamp && a.session_id == b.session_id);

        Ok(events)
    }

    /// Get collection statistics
    pub async fn get_collection_stats(&self) -> AppResult<CollectionStats> {
        let stats = self.collection_stats.read().await;
        Ok(stats.clone())
    }

    /// Get real-time metrics
    pub async fn get_realtime_metrics(&self) -> AppResult<RealtimeMetrics> {
        let buffer = self.event_buffer.read().await;
        let stats = self.collection_stats.read().await;

        let current_time = Utc::now();
        let last_minute = current_time - Duration::minutes(1);
        let last_hour = current_time - Duration::hours(1);

        let events_last_minute = buffer.iter()
            .filter(|e| e.timestamp >= last_minute)
            .count();

        let events_last_hour = buffer.iter()
            .filter(|e| e.timestamp >= last_hour)
            .count();

        Ok(RealtimeMetrics {
            buffer_size: buffer.len(),
            events_last_minute: events_last_minute as u64,
            events_last_hour: events_last_hour as u64,
            total_events_collected: stats.total_events_collected,
            collection_rate: self.calculate_collection_rate(&stats).await,
            last_event_time: stats.last_event_time,
        })
    }

    /// Calculate collection rate (events per minute)
    async fn calculate_collection_rate(&self, stats: &CollectionStats) -> f64 {
        if let Some(last_event_time) = stats.last_event_time {
            let duration_minutes = (Utc::now() - last_event_time).num_minutes() as f64;
            if duration_minutes > 0.0 {
                return stats.total_events_collected as f64 / duration_minutes;
            }
        }
        0.0
    }

    /// Get stored events by type and time from persistent storage
    async fn get_stored_events_by_type_and_time(
        &self,
        event_type: EventType,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> AppResult<Vec<AnalyticsEvent>> {
        let data_persistence = self.data_persistence.read().await;
        
        // Query the database for events
        let query = format!(
            "SELECT * FROM analytics_events WHERE event_type = ? AND timestamp BETWEEN ? AND ? ORDER BY timestamp",
        );
        
        // This would be implemented with actual database queries
        // For now, return empty vector as placeholder
        Ok(Vec::new())
    }

    /// Get stored events by time from persistent storage
    async fn get_stored_events_by_time(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> AppResult<Vec<AnalyticsEvent>> {
        let data_persistence = self.data_persistence.read().await;
        
        // Query the database for events
        let query = format!(
            "SELECT * FROM analytics_events WHERE timestamp BETWEEN ? AND ? ORDER BY timestamp",
        );
        
        // This would be implemented with actual database queries
        // For now, return empty vector as placeholder
        Ok(Vec::new())
    }

    /// Background task for processing event buffer
    async fn process_event_buffer(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(self.config.buffer_flush_interval_seconds));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.flush_event_buffer().await {
                error!("Failed to flush event buffer: {}", e);
            }
        }
    }

    /// Flush event buffer to persistent storage
    async fn flush_event_buffer(&self) -> AppResult<()> {
        let mut events_to_store = Vec::new();
        
        {
            let mut buffer = self.event_buffer.write().await;
            if buffer.is_empty() {
                return Ok(());
            }

            // Take events from buffer
            while let Some(event) = buffer.pop_front() {
                events_to_store.push(event);
                
                // Process in batches
                if events_to_store.len() >= self.config.batch_size {
                    break;
                }
            }
        }

        if !events_to_store.is_empty() {
            self.store_events(events_to_store).await?;
        }

        Ok(())
    }

    /// Store events in persistent storage
    async fn store_events(&self, events: Vec<AnalyticsEvent>) -> AppResult<()> {
        let data_persistence = self.data_persistence.read().await;
        
        // Store events in database
        for event in events {
            // This would be implemented with actual database operations
            // For now, just log the event storage
            info!("Storing event: {:?}", event.event_type);
        }

        Ok(())
    }

    /// Background task for cleaning up old events
    async fn cleanup_old_events(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // 1 hour
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.perform_cleanup().await {
                error!("Failed to cleanup old events: {}", e);
            }
        }
    }

    /// Perform cleanup of old events
    async fn perform_cleanup(&self) -> AppResult<()> {
        let cutoff_time = Utc::now() - Duration::days(self.config.retention_days as i64);
        
        let data_persistence = self.data_persistence.read().await;
        
        // Delete old events from database
        let query = "DELETE FROM analytics_events WHERE timestamp < ?";
        
        // This would be implemented with actual database operations
        info!("Cleaning up events older than {}", cutoff_time);

        Ok(())
    }

    /// Background task for generating collection reports
    async fn generate_collection_reports(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1800)); // 30 minutes
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.generate_report().await {
                error!("Failed to generate collection report: {}", e);
            }
        }
    }

    /// Generate collection report
    async fn generate_report(&self) -> AppResult<()> {
        let stats = self.collection_stats.read().await;
        let realtime_metrics = self.get_realtime_metrics().await?;

        info!("Collection Report - Total Events: {}, Buffer Size: {}, Collection Rate: {:.2} events/min",
            stats.total_events_collected,
            realtime_metrics.buffer_size,
            realtime_metrics.collection_rate
        );

        Ok(())
    }

    /// Health check
    pub async fn health_check(&self) -> AppResult<()> {
        let buffer = self.event_buffer.read().await;
        let stats = self.collection_stats.read().await;

        // Check if buffer is not overflowing
        if buffer.len() >= self.config.max_buffer_size {
            return Err(AppError::Analytics("Event buffer overflow".to_string()));
        }

        // Check if we're receiving events
        if let Some(last_event_time) = stats.last_event_time {
            let time_since_last_event = Utc::now() - last_event_time;
            if time_since_last_event > Duration::minutes(10) {
                warn!("No events received in the last 10 minutes");
            }
        }

        Ok(())
    }

    /// Shutdown
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down metrics collector...");

        // Flush remaining events
        self.flush_event_buffer().await?;

        info!("Metrics collector shutdown complete");
        Ok(())
    }
}

/// Metrics collector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsCollectorConfig {
    pub max_buffer_size: usize,
    pub buffer_flush_interval_seconds: u64,
    pub batch_size: usize,
    pub retention_days: u32,
    pub enable_realtime_metrics: bool,
}

impl Default for MetricsCollectorConfig {
    fn default() -> Self {
        Self {
            max_buffer_size: 10000,
            buffer_flush_interval_seconds: 30,
            batch_size: 100,
            retention_days: 365,
            enable_realtime_metrics: true,
        }
    }
}

/// Collection statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectionStats {
    pub total_events_collected: u64,
    pub events_by_type: HashMap<EventType, u64>,
    pub last_event_time: Option<DateTime<Utc>>,
    pub collection_started_at: Option<DateTime<Utc>>,
}

/// Real-time metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeMetrics {
    pub buffer_size: usize,
    pub events_last_minute: u64,
    pub events_last_hour: u64,
    pub total_events_collected: u64,
    pub collection_rate: f64,
    pub last_event_time: Option<DateTime<Utc>>,
}
