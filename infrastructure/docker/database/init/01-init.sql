-- Free Deep Research System Database Initialization
-- This script sets up the core database schema for the FDR system

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "btree_gin";

-- Create database if it doesn't exist (handled by Docker environment)
-- The database name is set via POSTGRES_DB environment variable

-- Set timezone
SET timezone = 'UTC';

-- Create core tables for the Free Deep Research System

-- API Keys Management
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    service_name VARCHAR(100) NOT NULL,
    api_key TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    usage_count BIGINT DEFAULT 0,
    last_used_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT unique_service_name UNIQUE (service_name)
);

-- Research Workflows
CREATE TABLE IF NOT EXISTS research_workflows (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    methodology VARCHAR(50) NOT NULL DEFAULT 'hybrid',
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    priority INTEGER DEFAULT 5,
    query TEXT NOT NULL,
    parameters JSONB DEFAULT '{}',
    results JSONB,
    error_message TEXT,
    progress_percentage INTEGER DEFAULT 0,
    estimated_completion_time TIMESTAMP WITH TIME ZONE,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT valid_status CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled')),
    CONSTRAINT valid_methodology CHECK (methodology IN ('don_lim', 'nick_scamara', 'hybrid')),
    CONSTRAINT valid_priority CHECK (priority BETWEEN 1 AND 10),
    CONSTRAINT valid_progress CHECK (progress_percentage BETWEEN 0 AND 100)
);

-- Research Tasks (sub-tasks of workflows)
CREATE TABLE IF NOT EXISTS research_tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES research_workflows(id) ON DELETE CASCADE,
    task_type VARCHAR(100) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    input_data JSONB,
    output_data JSONB,
    error_message TEXT,
    agent_assigned VARCHAR(100),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT valid_task_status CHECK (status IN ('pending', 'running', 'completed', 'failed', 'skipped'))
);

-- System Metrics
CREATE TABLE IF NOT EXISTS system_metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    metric_type VARCHAR(100) NOT NULL,
    metric_name VARCHAR(100) NOT NULL,
    metric_value NUMERIC NOT NULL,
    unit VARCHAR(20),
    tags JSONB DEFAULT '{}',
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT unique_metric_time UNIQUE (metric_type, metric_name, recorded_at)
);

-- API Usage Logs
CREATE TABLE IF NOT EXISTS api_usage_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    api_key_id UUID REFERENCES api_keys(id) ON DELETE SET NULL,
    service_name VARCHAR(100) NOT NULL,
    endpoint VARCHAR(255),
    method VARCHAR(10),
    status_code INTEGER,
    response_time_ms INTEGER,
    tokens_used INTEGER,
    cost_usd NUMERIC(10, 6),
    request_size_bytes INTEGER,
    response_size_bytes INTEGER,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- User Sessions (for web interface)
CREATE TABLE IF NOT EXISTS user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_token VARCHAR(255) NOT NULL UNIQUE,
    user_agent TEXT,
    ip_address INET,
    is_active BOOLEAN DEFAULT true,
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT (CURRENT_TIMESTAMP + INTERVAL '24 hours')
);

-- Configuration Settings
CREATE TABLE IF NOT EXISTS configuration_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    category VARCHAR(100) NOT NULL,
    key VARCHAR(100) NOT NULL,
    value TEXT NOT NULL,
    description TEXT,
    is_encrypted BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT unique_config_key UNIQUE (category, key)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_research_workflows_status ON research_workflows(status);
CREATE INDEX IF NOT EXISTS idx_research_workflows_created_at ON research_workflows(created_at);
CREATE INDEX IF NOT EXISTS idx_research_workflows_methodology ON research_workflows(methodology);

CREATE INDEX IF NOT EXISTS idx_research_tasks_workflow_id ON research_tasks(workflow_id);
CREATE INDEX IF NOT EXISTS idx_research_tasks_status ON research_tasks(status);
CREATE INDEX IF NOT EXISTS idx_research_tasks_agent ON research_tasks(agent_assigned);

CREATE INDEX IF NOT EXISTS idx_system_metrics_type_name ON system_metrics(metric_type, metric_name);
CREATE INDEX IF NOT EXISTS idx_system_metrics_recorded_at ON system_metrics(recorded_at);

CREATE INDEX IF NOT EXISTS idx_api_usage_logs_service ON api_usage_logs(service_name);
CREATE INDEX IF NOT EXISTS idx_api_usage_logs_created_at ON api_usage_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_api_usage_logs_api_key_id ON api_usage_logs(api_key_id);

CREATE INDEX IF NOT EXISTS idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_user_sessions_active ON user_sessions(is_active, expires_at);

-- Create triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_api_keys_updated_at BEFORE UPDATE ON api_keys
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_research_workflows_updated_at BEFORE UPDATE ON research_workflows
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_research_tasks_updated_at BEFORE UPDATE ON research_tasks
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_configuration_settings_updated_at BEFORE UPDATE ON configuration_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default configuration settings
INSERT INTO configuration_settings (category, key, value, description) VALUES
    ('system', 'max_concurrent_workflows', '5', 'Maximum number of concurrent research workflows'),
    ('system', 'default_methodology', 'hybrid', 'Default research methodology to use'),
    ('system', 'session_timeout_hours', '24', 'User session timeout in hours'),
    ('api', 'rate_limit_per_minute', '100', 'API rate limit per minute per key'),
    ('api', 'max_request_size_mb', '10', 'Maximum API request size in MB'),
    ('research', 'max_search_results', '50', 'Maximum search results per query'),
    ('research', 'timeout_minutes', '30', 'Research task timeout in minutes'),
    ('monitoring', 'metrics_retention_days', '90', 'System metrics retention period'),
    ('monitoring', 'log_retention_days', '30', 'API usage log retention period')
ON CONFLICT (category, key) DO NOTHING;

-- Create a view for active workflows with task counts
CREATE OR REPLACE VIEW active_workflows_summary AS
SELECT 
    w.id,
    w.name,
    w.methodology,
    w.status,
    w.progress_percentage,
    w.created_at,
    w.started_at,
    COUNT(t.id) as total_tasks,
    COUNT(CASE WHEN t.status = 'completed' THEN 1 END) as completed_tasks,
    COUNT(CASE WHEN t.status = 'failed' THEN 1 END) as failed_tasks
FROM research_workflows w
LEFT JOIN research_tasks t ON w.id = t.workflow_id
WHERE w.status IN ('pending', 'running')
GROUP BY w.id, w.name, w.methodology, w.status, w.progress_percentage, w.created_at, w.started_at
ORDER BY w.created_at DESC;

-- Create a view for API usage statistics
CREATE OR REPLACE VIEW api_usage_summary AS
SELECT 
    service_name,
    COUNT(*) as total_requests,
    COUNT(CASE WHEN status_code < 400 THEN 1 END) as successful_requests,
    COUNT(CASE WHEN status_code >= 400 THEN 1 END) as failed_requests,
    AVG(response_time_ms) as avg_response_time_ms,
    SUM(tokens_used) as total_tokens_used,
    SUM(cost_usd) as total_cost_usd,
    MAX(created_at) as last_used_at
FROM api_usage_logs
WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'
GROUP BY service_name
ORDER BY total_requests DESC;

-- Grant permissions (adjust as needed for your security requirements)
-- Note: In production, create specific users with limited permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO postgres;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO postgres;

-- Log successful initialization
INSERT INTO configuration_settings (category, key, value, description) VALUES
    ('system', 'database_initialized_at', EXTRACT(EPOCH FROM CURRENT_TIMESTAMP)::TEXT, 'Database initialization timestamp')
ON CONFLICT (category, key) DO UPDATE SET 
    value = EXTRACT(EPOCH FROM CURRENT_TIMESTAMP)::TEXT,
    updated_at = CURRENT_TIMESTAMP;

-- Success message
DO $$
BEGIN
    RAISE NOTICE 'Free Deep Research System database initialized successfully!';
    RAISE NOTICE 'Database: %', current_database();
    RAISE NOTICE 'Timestamp: %', CURRENT_TIMESTAMP;
END $$;
