-- Free Deep Research System - Database Health Check Functions
-- This script creates functions and procedures for database health monitoring

-- Create health check function
CREATE OR REPLACE FUNCTION database_health_check()
RETURNS TABLE (
    check_name TEXT,
    status TEXT,
    details TEXT,
    timestamp TIMESTAMP WITH TIME ZONE
) AS $$
BEGIN
    -- Check database connectivity
    RETURN QUERY SELECT 
        'database_connectivity'::TEXT,
        'healthy'::TEXT,
        'Database connection successful'::TEXT,
        CURRENT_TIMESTAMP;
    
    -- Check table existence
    RETURN QUERY SELECT 
        'core_tables'::TEXT,
        CASE 
            WHEN EXISTS (
                SELECT 1 FROM information_schema.tables 
                WHERE table_name IN ('api_keys', 'research_workflows', 'research_tasks', 'system_metrics')
            ) THEN 'healthy'::TEXT
            ELSE 'unhealthy'::TEXT
        END,
        'Core tables: ' || (
            SELECT string_agg(table_name, ', ')
            FROM information_schema.tables 
            WHERE table_name IN ('api_keys', 'research_workflows', 'research_tasks', 'system_metrics')
        )::TEXT,
        CURRENT_TIMESTAMP;
    
    -- Check database size
    RETURN QUERY SELECT 
        'database_size'::TEXT,
        'healthy'::TEXT,
        'Database size: ' || pg_size_pretty(pg_database_size(current_database()))::TEXT,
        CURRENT_TIMESTAMP;
    
    -- Check active connections
    RETURN QUERY SELECT 
        'active_connections'::TEXT,
        CASE 
            WHEN (SELECT count(*) FROM pg_stat_activity WHERE state = 'active') < 100 
            THEN 'healthy'::TEXT
            ELSE 'warning'::TEXT
        END,
        'Active connections: ' || (SELECT count(*) FROM pg_stat_activity WHERE state = 'active')::TEXT,
        CURRENT_TIMESTAMP;
    
    -- Check recent activity
    RETURN QUERY SELECT 
        'recent_activity'::TEXT,
        CASE 
            WHEN EXISTS (
                SELECT 1 FROM research_workflows 
                WHERE created_at > CURRENT_TIMESTAMP - INTERVAL '1 hour'
            ) THEN 'active'::TEXT
            ELSE 'idle'::TEXT
        END,
        'Recent workflows: ' || (
            SELECT count(*) FROM research_workflows 
            WHERE created_at > CURRENT_TIMESTAMP - INTERVAL '1 hour'
        )::TEXT,
        CURRENT_TIMESTAMP;
END;
$$ LANGUAGE plpgsql;

-- Create system metrics collection function
CREATE OR REPLACE FUNCTION collect_system_metrics()
RETURNS VOID AS $$
BEGIN
    -- Insert database metrics
    INSERT INTO system_metrics (metric_type, metric_name, metric_value, unit)
    VALUES 
        ('database', 'size_bytes', pg_database_size(current_database()), 'bytes'),
        ('database', 'active_connections', (SELECT count(*) FROM pg_stat_activity WHERE state = 'active'), 'count'),
        ('database', 'total_workflows', (SELECT count(*) FROM research_workflows), 'count'),
        ('database', 'active_workflows', (SELECT count(*) FROM research_workflows WHERE status IN ('pending', 'running')), 'count'),
        ('database', 'completed_workflows_today', (SELECT count(*) FROM research_workflows WHERE status = 'completed' AND created_at >= CURRENT_DATE), 'count');
    
    -- Clean old metrics (keep last 90 days)
    DELETE FROM system_metrics 
    WHERE recorded_at < CURRENT_TIMESTAMP - INTERVAL '90 days';
    
    -- Clean old API usage logs (keep last 30 days)
    DELETE FROM api_usage_logs 
    WHERE created_at < CURRENT_TIMESTAMP - INTERVAL '30 days';
    
    -- Clean expired sessions
    DELETE FROM user_sessions 
    WHERE expires_at < CURRENT_TIMESTAMP OR (is_active = false AND last_activity < CURRENT_TIMESTAMP - INTERVAL '7 days');
END;
$$ LANGUAGE plpgsql;

-- Create workflow cleanup function
CREATE OR REPLACE FUNCTION cleanup_old_workflows()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    -- Delete workflows older than 6 months that are completed or failed
    DELETE FROM research_workflows 
    WHERE status IN ('completed', 'failed', 'cancelled') 
    AND completed_at < CURRENT_TIMESTAMP - INTERVAL '6 months';
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    -- Log the cleanup
    INSERT INTO system_metrics (metric_type, metric_name, metric_value, unit)
    VALUES ('maintenance', 'workflows_cleaned', deleted_count, 'count');
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Create API usage statistics function
CREATE OR REPLACE FUNCTION get_api_usage_stats(days_back INTEGER DEFAULT 7)
RETURNS TABLE (
    service_name TEXT,
    total_requests BIGINT,
    successful_requests BIGINT,
    failed_requests BIGINT,
    avg_response_time NUMERIC,
    total_tokens BIGINT,
    total_cost NUMERIC,
    success_rate NUMERIC
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        aul.service_name::TEXT,
        COUNT(*)::BIGINT as total_requests,
        COUNT(CASE WHEN aul.status_code < 400 THEN 1 END)::BIGINT as successful_requests,
        COUNT(CASE WHEN aul.status_code >= 400 THEN 1 END)::BIGINT as failed_requests,
        ROUND(AVG(aul.response_time_ms), 2) as avg_response_time,
        COALESCE(SUM(aul.tokens_used), 0)::BIGINT as total_tokens,
        COALESCE(SUM(aul.cost_usd), 0) as total_cost,
        ROUND(
            (COUNT(CASE WHEN aul.status_code < 400 THEN 1 END)::NUMERIC / COUNT(*)::NUMERIC) * 100, 
            2
        ) as success_rate
    FROM api_usage_logs aul
    WHERE aul.created_at >= CURRENT_TIMESTAMP - (days_back || ' days')::INTERVAL
    GROUP BY aul.service_name
    ORDER BY total_requests DESC;
END;
$$ LANGUAGE plpgsql;

-- Create workflow performance analysis function
CREATE OR REPLACE FUNCTION analyze_workflow_performance()
RETURNS TABLE (
    methodology TEXT,
    avg_duration_minutes NUMERIC,
    success_rate NUMERIC,
    total_workflows BIGINT,
    avg_tasks_per_workflow NUMERIC
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        w.methodology::TEXT,
        ROUND(
            AVG(EXTRACT(EPOCH FROM (w.completed_at - w.started_at)) / 60), 
            2
        ) as avg_duration_minutes,
        ROUND(
            (COUNT(CASE WHEN w.status = 'completed' THEN 1 END)::NUMERIC / COUNT(*)::NUMERIC) * 100, 
            2
        ) as success_rate,
        COUNT(*)::BIGINT as total_workflows,
        ROUND(AVG(task_counts.task_count), 2) as avg_tasks_per_workflow
    FROM research_workflows w
    LEFT JOIN (
        SELECT workflow_id, COUNT(*) as task_count
        FROM research_tasks
        GROUP BY workflow_id
    ) task_counts ON w.id = task_counts.workflow_id
    WHERE w.created_at >= CURRENT_TIMESTAMP - INTERVAL '30 days'
    GROUP BY w.methodology
    ORDER BY total_workflows DESC;
END;
$$ LANGUAGE plpgsql;

-- Create maintenance schedule
CREATE OR REPLACE FUNCTION schedule_maintenance()
RETURNS TEXT AS $$
DECLARE
    result TEXT;
BEGIN
    -- Collect current metrics
    PERFORM collect_system_metrics();
    
    -- Cleanup old workflows
    result := 'Cleaned ' || cleanup_old_workflows() || ' old workflows. ';
    
    -- Analyze and vacuum tables
    VACUUM ANALYZE;
    result := result || 'Database maintenance completed.';
    
    RETURN result;
END;
$$ LANGUAGE plpgsql;

-- Grant execute permissions
GRANT EXECUTE ON FUNCTION database_health_check() TO postgres;
GRANT EXECUTE ON FUNCTION collect_system_metrics() TO postgres;
GRANT EXECUTE ON FUNCTION cleanup_old_workflows() TO postgres;
GRANT EXECUTE ON FUNCTION get_api_usage_stats(INTEGER) TO postgres;
GRANT EXECUTE ON FUNCTION analyze_workflow_performance() TO postgres;
GRANT EXECUTE ON FUNCTION schedule_maintenance() TO postgres;

-- Create initial health check entry
INSERT INTO system_metrics (metric_type, metric_name, metric_value, unit)
VALUES ('system', 'health_check_functions_created', 1, 'boolean');

-- Success message
DO $$
BEGIN
    RAISE NOTICE 'Database health check functions created successfully!';
END $$;
