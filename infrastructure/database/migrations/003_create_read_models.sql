-- Read Model Tables for CQRS Query Side
-- Phase 4.2: CQRS Pattern Implementation

-- Workflow read models table
CREATE TABLE IF NOT EXISTS workflow_read_models (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    query TEXT NOT NULL,
    methodology JSONB,
    status VARCHAR(50) NOT NULL DEFAULT 'created',
    created_at TIMESTAMPTZ NOT NULL,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    results JSONB,
    error_message TEXT,
    tags TEXT[] DEFAULT '{}',
    
    -- Computed metrics columns for performance
    total_tasks INTEGER DEFAULT 0,
    completed_tasks INTEGER DEFAULT 0,
    failed_tasks INTEGER DEFAULT 0,
    progress_percentage DECIMAL(5,2) DEFAULT 0.0,
    duration_minutes INTEGER,
    
    -- Constraints
    CONSTRAINT valid_status CHECK (status IN ('created', 'running', 'completed', 'failed', 'cancelled')),
    CONSTRAINT valid_progress CHECK (progress_percentage >= 0 AND progress_percentage <= 100),
    CONSTRAINT valid_duration CHECK (duration_minutes >= 0)
);

-- Task read models table
CREATE TABLE IF NOT EXISTS task_read_models (
    id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL,
    task_type VARCHAR(100) NOT NULL,
    agent_type VARCHAR(100),
    status VARCHAR(50) NOT NULL DEFAULT 'created',
    created_at TIMESTAMPTZ NOT NULL,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    results JSONB,
    error_message TEXT,
    duration_seconds INTEGER,
    retry_count INTEGER DEFAULT 0,
    
    -- Foreign key
    FOREIGN KEY (workflow_id) REFERENCES workflow_read_models(id) ON DELETE CASCADE,
    
    -- Constraints
    CONSTRAINT valid_task_status CHECK (status IN ('created', 'running', 'completed', 'failed', 'cancelled')),
    CONSTRAINT valid_retry_count CHECK (retry_count >= 0),
    CONSTRAINT valid_task_duration CHECK (duration_seconds >= 0)
);

-- Projection checkpoints table
CREATE TABLE IF NOT EXISTS projection_checkpoints_read_models (
    projection_name VARCHAR(255) PRIMARY KEY,
    last_processed_event_id UUID,
    last_processed_sequence BIGINT NOT NULL DEFAULT 0,
    last_processed_timestamp TIMESTAMPTZ,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    error_count INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_checkpoint_status CHECK (status IN ('active', 'paused', 'error', 'rebuilding', 'stopped'))
);

-- Workflow statistics materialized view for fast analytics
CREATE MATERIALIZED VIEW IF NOT EXISTS workflow_stats_view AS
SELECT 
    COUNT(*) as total_workflows,
    COUNT(*) FILTER (WHERE status = 'created') as created_workflows,
    COUNT(*) FILTER (WHERE status = 'running') as running_workflows,
    COUNT(*) FILTER (WHERE status = 'completed') as completed_workflows,
    COUNT(*) FILTER (WHERE status = 'failed') as failed_workflows,
    COUNT(*) FILTER (WHERE status = 'cancelled') as cancelled_workflows,
    
    -- Success rate
    ROUND(
        (COUNT(*) FILTER (WHERE status = 'completed')::DECIMAL / 
         NULLIF(COUNT(*) FILTER (WHERE status IN ('completed', 'failed')), 0)) * 100, 2
    ) as success_rate_percentage,
    
    -- Average completion time
    ROUND(AVG(duration_minutes) FILTER (WHERE status = 'completed'), 2) as avg_completion_time_minutes,
    
    -- Performance metrics
    ROUND(AVG(progress_percentage), 2) as avg_progress_percentage,
    MAX(total_tasks) as max_tasks_per_workflow,
    ROUND(AVG(total_tasks), 2) as avg_tasks_per_workflow,
    
    -- Time-based metrics
    DATE_TRUNC('day', NOW()) as stats_date,
    NOW() as last_updated
FROM workflow_read_models;

-- Task statistics materialized view
CREATE MATERIALIZED VIEW IF NOT EXISTS task_stats_view AS
SELECT 
    COUNT(*) as total_tasks,
    COUNT(*) FILTER (WHERE status = 'created') as created_tasks,
    COUNT(*) FILTER (WHERE status = 'running') as running_tasks,
    COUNT(*) FILTER (WHERE status = 'completed') as completed_tasks,
    COUNT(*) FILTER (WHERE status = 'failed') as failed_tasks,
    
    -- Task type distribution
    task_type,
    agent_type,
    COUNT(*) as task_count,
    ROUND(AVG(duration_seconds), 2) as avg_duration_seconds,
    ROUND(
        (COUNT(*) FILTER (WHERE status = 'completed')::DECIMAL / 
         NULLIF(COUNT(*) FILTER (WHERE status IN ('completed', 'failed')), 0)) * 100, 2
    ) as task_success_rate_percentage,
    
    DATE_TRUNC('day', NOW()) as stats_date,
    NOW() as last_updated
FROM task_read_models
GROUP BY task_type, agent_type;

-- Indexes for performance optimization

-- Workflow read models indexes
CREATE INDEX IF NOT EXISTS idx_workflow_read_models_status ON workflow_read_models (status);
CREATE INDEX IF NOT EXISTS idx_workflow_read_models_created_at ON workflow_read_models (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_workflow_read_models_updated_at ON workflow_read_models (updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_workflow_read_models_name_search ON workflow_read_models USING gin(to_tsvector('english', name));
CREATE INDEX IF NOT EXISTS idx_workflow_read_models_query_search ON workflow_read_models USING gin(to_tsvector('english', query));
CREATE INDEX IF NOT EXISTS idx_workflow_read_models_tags ON workflow_read_models USING gin(tags);
CREATE INDEX IF NOT EXISTS idx_workflow_read_models_progress ON workflow_read_models (progress_percentage DESC);
CREATE INDEX IF NOT EXISTS idx_workflow_read_models_duration ON workflow_read_models (duration_minutes) WHERE duration_minutes IS NOT NULL;

-- Task read models indexes
CREATE INDEX IF NOT EXISTS idx_task_read_models_workflow_id ON task_read_models (workflow_id);
CREATE INDEX IF NOT EXISTS idx_task_read_models_status ON task_read_models (status);
CREATE INDEX IF NOT EXISTS idx_task_read_models_task_type ON task_read_models (task_type);
CREATE INDEX IF NOT EXISTS idx_task_read_models_agent_type ON task_read_models (agent_type);
CREATE INDEX IF NOT EXISTS idx_task_read_models_created_at ON task_read_models (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_task_read_models_duration ON task_read_models (duration_seconds) WHERE duration_seconds IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_task_read_models_workflow_status ON task_read_models (workflow_id, status);

-- Projection checkpoints indexes
CREATE INDEX IF NOT EXISTS idx_projection_checkpoints_status ON projection_checkpoints_read_models (status);
CREATE INDEX IF NOT EXISTS idx_projection_checkpoints_updated_at ON projection_checkpoints_read_models (updated_at DESC);

-- Functions for maintaining read model consistency

-- Function to update workflow metrics when tasks change
CREATE OR REPLACE FUNCTION update_workflow_metrics()
RETURNS TRIGGER AS $$
BEGIN
    -- Update workflow metrics based on tasks
    UPDATE workflow_read_models 
    SET 
        total_tasks = (
            SELECT COUNT(*) 
            FROM task_read_models 
            WHERE workflow_id = COALESCE(NEW.workflow_id, OLD.workflow_id)
        ),
        completed_tasks = (
            SELECT COUNT(*) 
            FROM task_read_models 
            WHERE workflow_id = COALESCE(NEW.workflow_id, OLD.workflow_id) 
            AND status = 'completed'
        ),
        failed_tasks = (
            SELECT COUNT(*) 
            FROM task_read_models 
            WHERE workflow_id = COALESCE(NEW.workflow_id, OLD.workflow_id) 
            AND status = 'failed'
        ),
        updated_at = NOW()
    WHERE id = COALESCE(NEW.workflow_id, OLD.workflow_id);
    
    -- Update progress percentage
    UPDATE workflow_read_models 
    SET progress_percentage = CASE 
        WHEN total_tasks > 0 THEN (completed_tasks::DECIMAL / total_tasks::DECIMAL) * 100
        ELSE 0
    END
    WHERE id = COALESCE(NEW.workflow_id, OLD.workflow_id);
    
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Triggers to maintain workflow metrics
CREATE TRIGGER trigger_update_workflow_metrics_on_task_insert
    AFTER INSERT ON task_read_models
    FOR EACH ROW
    EXECUTE FUNCTION update_workflow_metrics();

CREATE TRIGGER trigger_update_workflow_metrics_on_task_update
    AFTER UPDATE ON task_read_models
    FOR EACH ROW
    EXECUTE FUNCTION update_workflow_metrics();

CREATE TRIGGER trigger_update_workflow_metrics_on_task_delete
    AFTER DELETE ON task_read_models
    FOR EACH ROW
    EXECUTE FUNCTION update_workflow_metrics();

-- Function to calculate workflow duration
CREATE OR REPLACE FUNCTION calculate_workflow_duration()
RETURNS TRIGGER AS $$
BEGIN
    -- Calculate duration when workflow is completed or failed
    IF NEW.status IN ('completed', 'failed') AND NEW.started_at IS NOT NULL THEN
        NEW.duration_minutes = EXTRACT(EPOCH FROM (COALESCE(NEW.completed_at, NOW()) - NEW.started_at)) / 60;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to calculate workflow duration
CREATE TRIGGER trigger_calculate_workflow_duration
    BEFORE UPDATE ON workflow_read_models
    FOR EACH ROW
    EXECUTE FUNCTION calculate_workflow_duration();

-- Function to refresh materialized views
CREATE OR REPLACE FUNCTION refresh_read_model_stats()
RETURNS VOID AS $$
BEGIN
    REFRESH MATERIALIZED VIEW workflow_stats_view;
    REFRESH MATERIALIZED VIEW task_stats_view;
END;
$$ LANGUAGE plpgsql;

-- Function to clean up old read model data
CREATE OR REPLACE FUNCTION cleanup_old_read_models(retention_days INTEGER DEFAULT 90)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    -- Delete old completed/failed workflows and their tasks
    WITH deleted_workflows AS (
        DELETE FROM workflow_read_models 
        WHERE status IN ('completed', 'failed', 'cancelled')
        AND completed_at < NOW() - INTERVAL '1 day' * retention_days
        RETURNING id
    )
    SELECT COUNT(*) INTO deleted_count FROM deleted_workflows;
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Grant permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON workflow_read_models TO fdr_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON task_read_models TO fdr_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON projection_checkpoints_read_models TO fdr_app;
GRANT SELECT ON workflow_stats_view TO fdr_app;
GRANT SELECT ON task_stats_view TO fdr_app;

-- Create initial projection checkpoints
INSERT INTO projection_checkpoints_read_models (projection_name, status) VALUES
('research_workflow_projection', 'active'),
('task_projection', 'active'),
('workflow_stats_projection', 'active')
ON CONFLICT (projection_name) DO NOTHING;

-- Create scheduled job to refresh materialized views (if pg_cron is available)
-- SELECT cron.schedule('refresh-read-model-stats', '*/5 * * * *', 'SELECT refresh_read_model_stats();');

-- Create scheduled job to cleanup old data (if pg_cron is available)
-- SELECT cron.schedule('cleanup-old-read-models', '0 2 * * *', 'SELECT cleanup_old_read_models(90);');

-- Sample queries for testing

-- Get workflow list with pagination
/*
SELECT 
    id, name, query, status, created_at, completed_at, 
    progress_percentage, total_tasks, completed_tasks
FROM workflow_read_models 
ORDER BY created_at DESC 
LIMIT 10 OFFSET 0;
*/

-- Search workflows by name or query
/*
SELECT 
    id, name, query, status, created_at,
    ts_rank(to_tsvector('english', name || ' ' || query), plainto_tsquery('english', 'AI research')) as rank
FROM workflow_read_models 
WHERE to_tsvector('english', name || ' ' || query) @@ plainto_tsquery('english', 'AI research')
ORDER BY rank DESC, created_at DESC;
*/

-- Get workflow statistics
/*
SELECT * FROM workflow_stats_view;
*/

-- Get task statistics by type
/*
SELECT 
    task_type, 
    COUNT(*) as total,
    COUNT(*) FILTER (WHERE status = 'completed') as completed,
    ROUND(AVG(duration_seconds), 2) as avg_duration
FROM task_read_models 
GROUP BY task_type 
ORDER BY total DESC;
*/
