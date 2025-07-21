// Snapshot Store for Event Sourcing
// Phase 4.1: Event Sourcing Foundation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::error::{EventStoreError, EventStoreResult};

/// Snapshot data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub stream_id: Uuid,
    pub snapshot_version: u64,
    pub snapshot_data: serde_json::Value,
    pub snapshot_metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Snapshot store trait for pluggable storage backends
#[async_trait]
pub trait SnapshotStorage: Send + Sync {
    /// Save a snapshot
    async fn save_snapshot(
        &self,
        stream_id: Uuid,
        data: serde_json::Value,
        version: u64,
    ) -> EventStoreResult<()>;
    
    /// Load the latest snapshot for a stream
    async fn load_latest_snapshot(
        &self,
        stream_id: Uuid,
    ) -> EventStoreResult<Option<(serde_json::Value, u64)>>;
    
    /// Load a specific snapshot version
    async fn load_snapshot_at_version(
        &self,
        stream_id: Uuid,
        version: u64,
    ) -> EventStoreResult<Option<(serde_json::Value, u64)>>;
    
    /// Delete old snapshots (for cleanup)
    async fn delete_snapshots_before_version(
        &self,
        stream_id: Uuid,
        version: u64,
    ) -> EventStoreResult<u64>;
    
    /// Get snapshot statistics
    async fn get_snapshot_stats(&self, stream_id: Uuid) -> EventStoreResult<SnapshotStats>;
}

/// Snapshot statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotStats {
    pub stream_id: Uuid,
    pub total_snapshots: u64,
    pub latest_version: u64,
    pub oldest_version: u64,
    pub total_size_bytes: u64,
    pub last_snapshot_at: Option<DateTime<Utc>>,
}

/// PostgreSQL-based snapshot storage implementation
pub struct PostgresSnapshotStorage {
    pool: PgPool,
}

impl PostgresSnapshotStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SnapshotStorage for PostgresSnapshotStorage {
    async fn save_snapshot(
        &self,
        stream_id: Uuid,
        data: serde_json::Value,
        version: u64,
    ) -> EventStoreResult<()> {
        let metadata = serde_json::json!({
            "created_at": Utc::now(),
            "size_bytes": serde_json::to_vec(&data)?.len(),
            "compression": "none"
        });
        
        sqlx::query!(
            r#"
            INSERT INTO snapshots (stream_id, snapshot_version, snapshot_data, snapshot_metadata)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (stream_id, snapshot_version) 
            DO UPDATE SET 
                snapshot_data = EXCLUDED.snapshot_data,
                snapshot_metadata = EXCLUDED.snapshot_metadata,
                created_at = NOW()
            "#,
            stream_id,
            version as i64,
            data,
            metadata
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn load_latest_snapshot(
        &self,
        stream_id: Uuid,
    ) -> EventStoreResult<Option<(serde_json::Value, u64)>> {
        let row = sqlx::query!(
            r#"
            SELECT snapshot_data, snapshot_version
            FROM snapshots
            WHERE stream_id = $1
            ORDER BY snapshot_version DESC
            LIMIT 1
            "#,
            stream_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(Some((row.snapshot_data, row.snapshot_version as u64)))
        } else {
            Ok(None)
        }
    }
    
    async fn load_snapshot_at_version(
        &self,
        stream_id: Uuid,
        version: u64,
    ) -> EventStoreResult<Option<(serde_json::Value, u64)>> {
        let row = sqlx::query!(
            r#"
            SELECT snapshot_data, snapshot_version
            FROM snapshots
            WHERE stream_id = $1 AND snapshot_version <= $2
            ORDER BY snapshot_version DESC
            LIMIT 1
            "#,
            stream_id,
            version as i64
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(Some((row.snapshot_data, row.snapshot_version as u64)))
        } else {
            Ok(None)
        }
    }
    
    async fn delete_snapshots_before_version(
        &self,
        stream_id: Uuid,
        version: u64,
    ) -> EventStoreResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM snapshots
            WHERE stream_id = $1 AND snapshot_version < $2
            "#,
            stream_id,
            version as i64
        )
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected())
    }
    
    async fn get_snapshot_stats(&self, stream_id: Uuid) -> EventStoreResult<SnapshotStats> {
        let row = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_snapshots,
                COALESCE(MAX(snapshot_version), 0) as latest_version,
                COALESCE(MIN(snapshot_version), 0) as oldest_version,
                COALESCE(SUM(LENGTH(snapshot_data::text)), 0) as total_size_bytes,
                MAX(created_at) as last_snapshot_at
            FROM snapshots
            WHERE stream_id = $1
            "#,
            stream_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(SnapshotStats {
            stream_id,
            total_snapshots: row.total_snapshots.unwrap_or(0) as u64,
            latest_version: row.latest_version.unwrap_or(0) as u64,
            oldest_version: row.oldest_version.unwrap_or(0) as u64,
            total_size_bytes: row.total_size_bytes.unwrap_or(0) as u64,
            last_snapshot_at: row.last_snapshot_at,
        })
    }
}

/// Snapshot store with caching and compression
pub struct SnapshotStore {
    storage: Arc<dyn SnapshotStorage>,
    cache: Arc<RwLock<HashMap<Uuid, CachedSnapshot>>>,
    config: SnapshotConfig,
}

/// Cached snapshot entry
#[derive(Debug, Clone)]
struct CachedSnapshot {
    data: serde_json::Value,
    version: u64,
    cached_at: DateTime<Utc>,
}

/// Snapshot store configuration
#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    pub cache_size: usize,
    pub cache_ttl_seconds: u64,
    pub compression_enabled: bool,
    pub cleanup_frequency_hours: u64,
    pub max_snapshots_per_stream: u64,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            cache_size: 1000,
            cache_ttl_seconds: 3600, // 1 hour
            compression_enabled: true,
            cleanup_frequency_hours: 24,
            max_snapshots_per_stream: 10,
        }
    }
}

impl SnapshotStore {
    pub fn new(storage: Arc<dyn SnapshotStorage>) -> Self {
        Self::with_config(storage, SnapshotConfig::default())
    }
    
    pub fn with_config(storage: Arc<dyn SnapshotStorage>, config: SnapshotConfig) -> Self {
        Self {
            storage,
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    /// Save a snapshot with caching
    pub async fn save_snapshot(
        &self,
        stream_id: Uuid,
        data: serde_json::Value,
        version: u64,
    ) -> EventStoreResult<()> {
        // Save to storage
        self.storage.save_snapshot(stream_id, data.clone(), version).await?;
        
        // Update cache
        let cached_snapshot = CachedSnapshot {
            data,
            version,
            cached_at: Utc::now(),
        };
        
        let mut cache = self.cache.write().await;
        
        // Evict old entries if cache is full
        if cache.len() >= self.config.cache_size {
            self.evict_oldest_cache_entry(&mut cache).await;
        }
        
        cache.insert(stream_id, cached_snapshot);
        
        Ok(())
    }
    
    /// Load latest snapshot with caching
    pub async fn load_latest_snapshot(
        &self,
        stream_id: Uuid,
    ) -> EventStoreResult<Option<(serde_json::Value, u64)>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(&stream_id) {
                if self.is_cache_valid(&cached) {
                    return Ok(Some((cached.data.clone(), cached.version)));
                }
            }
        }
        
        // Load from storage
        let result = self.storage.load_latest_snapshot(stream_id).await?;
        
        // Update cache if found
        if let Some((data, version)) = &result {
            let cached_snapshot = CachedSnapshot {
                data: data.clone(),
                version: *version,
                cached_at: Utc::now(),
            };
            
            let mut cache = self.cache.write().await;
            cache.insert(stream_id, cached_snapshot);
        }
        
        Ok(result)
    }
    
    /// Load snapshot at specific version
    pub async fn load_snapshot_at_version(
        &self,
        stream_id: Uuid,
        version: u64,
    ) -> EventStoreResult<Option<(serde_json::Value, u64)>> {
        // For specific versions, always go to storage (no caching)
        self.storage.load_snapshot_at_version(stream_id, version).await
    }
    
    /// Clean up old snapshots
    pub async fn cleanup_old_snapshots(&self, stream_id: Uuid) -> EventStoreResult<u64> {
        let stats = self.storage.get_snapshot_stats(stream_id).await?;
        
        if stats.total_snapshots > self.config.max_snapshots_per_stream {
            let keep_version = stats.latest_version.saturating_sub(self.config.max_snapshots_per_stream);
            self.storage.delete_snapshots_before_version(stream_id, keep_version).await
        } else {
            Ok(0)
        }
    }
    
    /// Get snapshot statistics
    pub async fn get_stats(&self, stream_id: Uuid) -> EventStoreResult<SnapshotStats> {
        self.storage.get_snapshot_stats(stream_id).await
    }
    
    /// Clear cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
    
    /// Check if cached snapshot is still valid
    fn is_cache_valid(&self, cached: &CachedSnapshot) -> bool {
        let age = Utc::now().signed_duration_since(cached.cached_at);
        age.num_seconds() < self.config.cache_ttl_seconds as i64
    }
    
    /// Evict oldest cache entry
    async fn evict_oldest_cache_entry(&self, cache: &mut HashMap<Uuid, CachedSnapshot>) {
        if let Some(oldest_key) = cache
            .iter()
            .min_by_key(|(_, snapshot)| snapshot.cached_at)
            .map(|(key, _)| *key)
        {
            cache.remove(&oldest_key);
        }
    }
}

/// Snapshot compression utilities
pub struct SnapshotCompression;

impl SnapshotCompression {
    /// Compress snapshot data (placeholder implementation)
    pub fn compress(data: &serde_json::Value) -> EventStoreResult<Vec<u8>> {
        // In production, you might use gzip, lz4, or zstd compression
        let json_bytes = serde_json::to_vec(data)?;
        Ok(json_bytes) // No compression for now
    }
    
    /// Decompress snapshot data (placeholder implementation)
    pub fn decompress(compressed_data: &[u8]) -> EventStoreResult<serde_json::Value> {
        // In production, decompress using the same algorithm
        let data: serde_json::Value = serde_json::from_slice(compressed_data)?;
        Ok(data)
    }
}

/// Snapshot cleanup service for background maintenance
pub struct SnapshotCleanupService {
    snapshot_store: Arc<SnapshotStore>,
    config: SnapshotConfig,
}

impl SnapshotCleanupService {
    pub fn new(snapshot_store: Arc<SnapshotStore>, config: SnapshotConfig) -> Self {
        Self {
            snapshot_store,
            config,
        }
    }
    
    /// Start background cleanup task
    pub async fn start_cleanup_task(&self) -> EventStoreResult<()> {
        let snapshot_store = Arc::clone(&self.snapshot_store);
        let cleanup_interval = tokio::time::Duration::from_secs(
            self.config.cleanup_frequency_hours * 3600
        );
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            
            loop {
                interval.tick().await;
                
                // This is a simplified cleanup - in production you'd want to:
                // 1. Get list of all streams
                // 2. Clean up each stream
                // 3. Handle errors gracefully
                // 4. Add metrics and logging
                
                println!("Running snapshot cleanup task...");
                
                // Clear expired cache entries
                snapshot_store.clear_cache().await;
            }
        });
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    // Mock storage for testing
    struct MockSnapshotStorage {
        snapshots: Arc<RwLock<HashMap<(Uuid, u64), serde_json::Value>>>,
    }

    impl MockSnapshotStorage {
        fn new() -> Self {
            Self {
                snapshots: Arc::new(RwLock::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl SnapshotStorage for MockSnapshotStorage {
        async fn save_snapshot(
            &self,
            stream_id: Uuid,
            data: serde_json::Value,
            version: u64,
        ) -> EventStoreResult<()> {
            let mut snapshots = self.snapshots.write().await;
            snapshots.insert((stream_id, version), data);
            Ok(())
        }

        async fn load_latest_snapshot(
            &self,
            stream_id: Uuid,
        ) -> EventStoreResult<Option<(serde_json::Value, u64)>> {
            let snapshots = self.snapshots.read().await;
            let latest = snapshots
                .iter()
                .filter(|((id, _), _)| *id == stream_id)
                .max_by_key(|((_, version), _)| version)
                .map(|((_, version), data)| (data.clone(), *version));
            
            Ok(latest)
        }

        async fn load_snapshot_at_version(
            &self,
            stream_id: Uuid,
            version: u64,
        ) -> EventStoreResult<Option<(serde_json::Value, u64)>> {
            let snapshots = self.snapshots.read().await;
            let snapshot = snapshots
                .iter()
                .filter(|((id, v), _)| *id == stream_id && *v <= version)
                .max_by_key(|((_, v), _)| v)
                .map(|((_, v), data)| (data.clone(), *v));
            
            Ok(snapshot)
        }

        async fn delete_snapshots_before_version(
            &self,
            stream_id: Uuid,
            version: u64,
        ) -> EventStoreResult<u64> {
            let mut snapshots = self.snapshots.write().await;
            let keys_to_remove: Vec<_> = snapshots
                .keys()
                .filter(|(id, v)| *id == stream_id && *v < version)
                .cloned()
                .collect();
            
            let count = keys_to_remove.len() as u64;
            for key in keys_to_remove {
                snapshots.remove(&key);
            }
            
            Ok(count)
        }

        async fn get_snapshot_stats(&self, stream_id: Uuid) -> EventStoreResult<SnapshotStats> {
            let snapshots = self.snapshots.read().await;
            let stream_snapshots: Vec<_> = snapshots
                .iter()
                .filter(|((id, _), _)| *id == stream_id)
                .collect();
            
            let total_snapshots = stream_snapshots.len() as u64;
            let latest_version = stream_snapshots
                .iter()
                .map(|((_, v), _)| *v)
                .max()
                .unwrap_or(0);
            let oldest_version = stream_snapshots
                .iter()
                .map(|((_, v), _)| *v)
                .min()
                .unwrap_or(0);
            
            Ok(SnapshotStats {
                stream_id,
                total_snapshots,
                latest_version,
                oldest_version,
                total_size_bytes: 0,
                last_snapshot_at: Some(Utc::now()),
            })
        }
    }

    #[tokio::test]
    async fn test_snapshot_store_save_and_load() {
        let storage = Arc::new(MockSnapshotStorage::new());
        let snapshot_store = SnapshotStore::new(storage);
        
        let stream_id = Uuid::new_v4();
        let data = serde_json::json!({"state": "test"});
        
        // Save snapshot
        snapshot_store.save_snapshot(stream_id, data.clone(), 1).await.unwrap();
        
        // Load snapshot
        let loaded = snapshot_store.load_latest_snapshot(stream_id).await.unwrap();
        assert!(loaded.is_some());
        
        let (loaded_data, version) = loaded.unwrap();
        assert_eq!(loaded_data, data);
        assert_eq!(version, 1);
    }

    #[tokio::test]
    async fn test_snapshot_caching() {
        let storage = Arc::new(MockSnapshotStorage::new());
        let snapshot_store = SnapshotStore::new(storage);
        
        let stream_id = Uuid::new_v4();
        let data = serde_json::json!({"state": "cached"});
        
        // Save and load to populate cache
        snapshot_store.save_snapshot(stream_id, data.clone(), 1).await.unwrap();
        let _ = snapshot_store.load_latest_snapshot(stream_id).await.unwrap();
        
        // Load again - should come from cache
        let loaded = snapshot_store.load_latest_snapshot(stream_id).await.unwrap();
        assert!(loaded.is_some());
        
        let (loaded_data, version) = loaded.unwrap();
        assert_eq!(loaded_data, data);
        assert_eq!(version, 1);
    }
}
