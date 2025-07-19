use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use rusqlite::{Connection, Result as SqliteResult};

use crate::error::{AppResult, ResearchError};
use crate::services::data_persistence::DataPersistenceService;

/// Migration for Version 3.0.0 - Global Intelligence Network
pub struct V3Migration {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
}

impl V3Migration {
    pub fn new(data_persistence: Arc<RwLock<DataPersistenceService>>) -> Self {
        Self {
            data_persistence,
        }
    }

    /// Execute the V3.0.0 migration
    pub async fn execute(&self) -> AppResult<()> {
        info!("Starting V3.0.0 database migration...");

        let data_persistence = self.data_persistence.write().await;
        let connection = data_persistence.get_connection().await?;

        // Execute migration in a transaction
        connection.execute("BEGIN TRANSACTION", [])?;

        match self.run_migration(&connection).await {
            Ok(_) => {
                connection.execute("COMMIT", [])?;
                info!("V3.0.0 migration completed successfully");
                Ok(())
            }
            Err(e) => {
                error!("V3.0.0 migration failed: {}", e);
                connection.execute("ROLLBACK", [])?;
                Err(e)
            }
        }
    }

    /// Run the actual migration steps
    async fn run_migration(&self, connection: &Connection) -> AppResult<()> {
        // Check if migration has already been applied
        if self.is_migration_applied(connection)? {
            info!("V3.0.0 migration already applied, skipping");
            return Ok(());
        }

        // Apply schema changes
        self.create_federated_research_tables(connection)?;
        self.create_ai_marketplace_tables(connection)?;
        self.create_quantum_ready_tables(connection)?;
        self.create_nlp_engine_tables(connection)?;
        self.create_blockchain_tables(connection)?;
        self.create_knowledge_graph_tables(connection)?;
        self.create_indexes(connection)?;

        // Record migration as applied
        self.record_migration_applied(connection)?;

        Ok(())
    }

    /// Check if the migration has already been applied
    fn is_migration_applied(&self, connection: &Connection) -> SqliteResult<bool> {
        // Check if migration tracking table exists
        let mut stmt = connection.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='schema_migrations'"
        )?;
        
        let table_exists = stmt.exists([])?;
        
        if !table_exists {
            // Create migration tracking table
            connection.execute(
                "CREATE TABLE schema_migrations (
                    version TEXT PRIMARY KEY,
                    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            )?;
            return Ok(false);
        }

        // Check if V3.0.0 migration is recorded
        let mut stmt = connection.prepare(
            "SELECT COUNT(*) FROM schema_migrations WHERE version = '3.0.0'"
        )?;
        
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count > 0)
    }

    /// Record that the migration has been applied
    fn record_migration_applied(&self, connection: &Connection) -> SqliteResult<()> {
        connection.execute(
            "INSERT INTO schema_migrations (version) VALUES ('3.0.0')",
            [],
        )?;
        Ok(())
    }

    /// Create federated research tables
    fn create_federated_research_tables(&self, connection: &Connection) -> SqliteResult<()> {
        debug!("Creating federated research tables...");

        connection.execute(
            "CREATE TABLE IF NOT EXISTS federated_organizations (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                domain TEXT NOT NULL UNIQUE,
                public_key TEXT NOT NULL,
                trust_level INTEGER DEFAULT 0,
                api_endpoint TEXT NOT NULL,
                contact_email TEXT,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS research_partnerships (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                partner_organization_id TEXT NOT NULL,
                partnership_type TEXT NOT NULL,
                data_sharing_level TEXT NOT NULL,
                permissions TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                expires_at DATETIME,
                FOREIGN KEY (organization_id) REFERENCES federated_organizations (id),
                FOREIGN KEY (partner_organization_id) REFERENCES federated_organizations (id)
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS shared_research_sessions (
                id TEXT PRIMARY KEY,
                local_workflow_id TEXT NOT NULL,
                sharing_organization_id TEXT NOT NULL,
                shared_data TEXT,
                access_permissions TEXT,
                sharing_level TEXT NOT NULL,
                expiration_date DATETIME,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (sharing_organization_id) REFERENCES federated_organizations (id)
            )",
            [],
        )?;

        Ok(())
    }

    /// Create AI marketplace tables
    fn create_ai_marketplace_tables(&self, connection: &Connection) -> SqliteResult<()> {
        debug!("Creating AI marketplace tables...");

        connection.execute(
            "CREATE TABLE IF NOT EXISTS marketplace_users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                display_name TEXT NOT NULL,
                bio TEXT,
                avatar_url TEXT,
                reputation_score INTEGER DEFAULT 0,
                total_contributions INTEGER DEFAULT 0,
                verified BOOLEAN DEFAULT FALSE,
                organization_id TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_active DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (organization_id) REFERENCES federated_organizations (id)
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS ai_agents_marketplace (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                category TEXT NOT NULL,
                creator_id TEXT NOT NULL,
                version TEXT NOT NULL,
                agent_config TEXT NOT NULL,
                pricing_model TEXT NOT NULL,
                price_per_use DECIMAL(10, 4) DEFAULT 0,
                downloads INTEGER DEFAULT 0,
                rating DECIMAL(3, 2) DEFAULT 0,
                rating_count INTEGER DEFAULT 0,
                tags TEXT,
                requirements TEXT,
                status TEXT NOT NULL DEFAULT 'draft',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (creator_id) REFERENCES marketplace_users (id)
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS research_methodologies (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                creator_id TEXT NOT NULL,
                methodology_config TEXT NOT NULL,
                category TEXT NOT NULL,
                complexity_level INTEGER NOT NULL,
                estimated_time_minutes INTEGER,
                success_rate DECIMAL(5, 2),
                usage_count INTEGER DEFAULT 0,
                rating DECIMAL(3, 2) DEFAULT 0,
                rating_count INTEGER DEFAULT 0,
                is_public BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (creator_id) REFERENCES marketplace_users (id)
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS community_ratings (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                target_type TEXT NOT NULL,
                target_id TEXT NOT NULL,
                rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
                review_text TEXT,
                helpful_votes INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES marketplace_users (id),
                UNIQUE(user_id, target_type, target_id)
            )",
            [],
        )?;

        Ok(())
    }

    /// Create quantum-ready architecture tables
    fn create_quantum_ready_tables(&self, connection: &Connection) -> SqliteResult<()> {
        debug!("Creating quantum-ready architecture tables...");

        connection.execute(
            "CREATE TABLE IF NOT EXISTS quantum_algorithms (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                algorithm_type TEXT NOT NULL,
                classical_equivalent TEXT,
                quantum_safe BOOLEAN DEFAULT TRUE,
                implementation_status TEXT NOT NULL,
                performance_metrics TEXT,
                security_level INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS compute_resources (
                id TEXT PRIMARY KEY,
                resource_type TEXT NOT NULL,
                provider TEXT NOT NULL,
                endpoint_url TEXT,
                capabilities TEXT,
                availability_status TEXT NOT NULL DEFAULT 'available',
                cost_per_operation DECIMAL(10, 8),
                max_qubits INTEGER,
                coherence_time_ms DECIMAL(10, 3),
                error_rate DECIMAL(10, 8),
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_health_check DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS security_protocols (
                id TEXT PRIMARY KEY,
                protocol_name TEXT NOT NULL,
                protocol_version TEXT NOT NULL,
                quantum_safe BOOLEAN DEFAULT FALSE,
                classical_fallback BOOLEAN DEFAULT TRUE,
                implementation_config TEXT,
                migration_path TEXT,
                status TEXT NOT NULL DEFAULT 'active',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(())
    }

    /// Create remaining tables (NLP, Blockchain, Knowledge Graph)
    fn create_nlp_engine_tables(&self, connection: &Connection) -> SqliteResult<()> {
        debug!("Creating NLP engine tables...");
        // Load and execute the remaining schema from schema_v3.sql
        let schema_content = include_str!("../schema_v3.sql");
        let nlp_section = self.extract_schema_section(schema_content, "-- Advanced NLP Engine Tables", "-- Blockchain Integration Tables");
        connection.execute_batch(&nlp_section)?;
        Ok(())
    }

    fn create_blockchain_tables(&self, connection: &Connection) -> SqliteResult<()> {
        debug!("Creating blockchain tables...");
        let schema_content = include_str!("../schema_v3.sql");
        let blockchain_section = self.extract_schema_section(schema_content, "-- Blockchain Integration Tables", "-- Global Knowledge Graph Tables");
        connection.execute_batch(&blockchain_section)?;
        Ok(())
    }

    fn create_knowledge_graph_tables(&self, connection: &Connection) -> SqliteResult<()> {
        debug!("Creating knowledge graph tables...");
        let schema_content = include_str!("../schema_v3.sql");
        let kg_section = self.extract_schema_section(schema_content, "-- Global Knowledge Graph Tables", "-- Create indexes for performance");
        connection.execute_batch(&kg_section)?;
        Ok(())
    }

    fn create_indexes(&self, connection: &Connection) -> SqliteResult<()> {
        debug!("Creating performance indexes...");
        let schema_content = include_str!("../schema_v3.sql");
        let indexes_section = self.extract_schema_section(schema_content, "-- Create indexes for performance", "");
        connection.execute_batch(&indexes_section)?;
        Ok(())
    }

    /// Extract a section from the schema file
    fn extract_schema_section(&self, content: &str, start_marker: &str, end_marker: &str) -> String {
        let start_pos = content.find(start_marker).unwrap_or(0);
        let end_pos = if end_marker.is_empty() {
            content.len()
        } else {
            content.find(end_marker).unwrap_or(content.len())
        };
        
        content[start_pos..end_pos].to_string()
    }
}
