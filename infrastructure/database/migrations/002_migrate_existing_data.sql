-- Data Migration Script for Event Sourcing
-- Phase 4.1: Event Sourcing Foundation
-- Migrates existing data to event store format

-- Create temporary tables for migration tracking
CREATE TABLE IF NOT EXISTS migration_log (
    id BIGSERIAL PRIMARY KEY,
    migration_name VARCHAR(255) NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    status VARCHAR(50) NOT NULL DEFAULT 'running',
    records_processed BIGINT DEFAULT 0,
    records_failed BIGINT DEFAULT 0,
    error_message TEXT,
    
    CONSTRAINT valid_migration_status CHECK (status IN ('running', 'completed', 'failed', 'rolled_back'))
);

-- Create backup tables for rollback capability
CREATE TABLE IF NOT EXISTS research_workflows_backup AS 
SELECT * FROM research_workflows WHERE 1=0; -- Create structure only

CREATE TABLE IF NOT EXISTS research_tasks_backup AS 
SELECT * FROM research_tasks WHERE 1=0; -- Create structure only

-- Function to log migration progress
CREATE OR REPLACE FUNCTION log_migration_progress(
    p_migration_name VARCHAR(255),
    p_status VARCHAR(50),
    p_records_processed BIGINT DEFAULT 0,
    p_records_failed BIGINT DEFAULT 0,
    p_error_message TEXT DEFAULT NULL
)
RETURNS VOID AS $$
BEGIN
    INSERT INTO migration_log (
        migration_name, 
        status, 
        records_processed, 
        records_failed, 
        error_message,
        completed_at
    ) VALUES (
        p_migration_name,
        p_status,
        p_records_processed,
        p_records_failed,
        p_error_message,
        CASE WHEN p_status IN ('completed', 'failed', 'rolled_back') THEN NOW() ELSE NULL END
    );
END;
$$ LANGUAGE plpgsql;

-- Function to migrate research workflows to events
CREATE OR REPLACE FUNCTION migrate_research_workflows_to_events()
RETURNS VOID AS $$
DECLARE
    workflow_record RECORD;
    task_record RECORD;
    workflow_events JSONB[];
    task_events JSONB[];
    event_data JSONB;
    sequence_num BIGINT;
    total_workflows BIGINT;
    processed_workflows BIGINT := 0;
    failed_workflows BIGINT := 0;
BEGIN
    -- Log migration start
    PERFORM log_migration_progress('migrate_research_workflows', 'running');
    
    -- Get total count for progress tracking
    SELECT COUNT(*) INTO total_workflows FROM research_workflows;
    
    -- Create backup
    INSERT INTO research_workflows_backup SELECT * FROM research_workflows;
    
    -- Process each workflow
    FOR workflow_record IN 
        SELECT * FROM research_workflows ORDER BY created_at ASC
    LOOP
        BEGIN
            workflow_events := ARRAY[]::JSONB[];
            sequence_num := 0;
            
            -- Create WorkflowCreated event
            sequence_num := sequence_num + 1;
            event_data := jsonb_build_object(
                'event_id', gen_random_uuid(),
                'event_type', 'research.workflow.created',
                'event_version', 1,
                'data', jsonb_build_object(
                    'workflow_id', workflow_record.id,
                    'name', workflow_record.name,
                    'query', workflow_record.query,
                    'methodology', COALESCE(workflow_record.methodology, '{}'::jsonb),
                    'created_at', workflow_record.created_at,
                    'correlation_id', NULL
                ),
                'metadata', jsonb_build_object(
                    'migrated_from', 'research_workflows',
                    'migration_timestamp', NOW()
                )
            );
            workflow_events := array_append(workflow_events, event_data);
            
            -- Create ExecutionStarted event if workflow was started
            IF workflow_record.started_at IS NOT NULL THEN
                sequence_num := sequence_num + 1;
                event_data := jsonb_build_object(
                    'event_id', gen_random_uuid(),
                    'event_type', 'research.workflow.started',
                    'event_version', 1,
                    'data', jsonb_build_object(
                        'workflow_id', workflow_record.id,
                        'started_at', workflow_record.started_at,
                        'correlation_id', NULL
                    ),
                    'metadata', jsonb_build_object(
                        'migrated_from', 'research_workflows',
                        'migration_timestamp', NOW()
                    )
                );
                workflow_events := array_append(workflow_events, event_data);
            END IF;
            
            -- Create ExecutionCompleted event if workflow was completed
            IF workflow_record.completed_at IS NOT NULL AND workflow_record.status = 'completed' THEN
                sequence_num := sequence_num + 1;
                event_data := jsonb_build_object(
                    'event_id', gen_random_uuid(),
                    'event_type', 'research.workflow.completed',
                    'event_version', 1,
                    'data', jsonb_build_object(
                        'workflow_id', workflow_record.id,
                        'results', COALESCE(workflow_record.results, '{}'::jsonb),
                        'completed_at', workflow_record.completed_at,
                        'correlation_id', NULL
                    ),
                    'metadata', jsonb_build_object(
                        'migrated_from', 'research_workflows',
                        'migration_timestamp', NOW()
                    )
                );
                workflow_events := array_append(workflow_events, event_data);
            END IF;
            
            -- Create ExecutionFailed event if workflow failed
            IF workflow_record.status = 'failed' THEN
                sequence_num := sequence_num + 1;
                event_data := jsonb_build_object(
                    'event_id', gen_random_uuid(),
                    'event_type', 'research.workflow.failed',
                    'event_version', 1,
                    'data', jsonb_build_object(
                        'workflow_id', workflow_record.id,
                        'error', COALESCE(workflow_record.error_message, 'Unknown error'),
                        'failed_at', COALESCE(workflow_record.completed_at, workflow_record.updated_at),
                        'correlation_id', NULL
                    ),
                    'metadata', jsonb_build_object(
                        'migrated_from', 'research_workflows',
                        'migration_timestamp', NOW()
                    )
                );
                workflow_events := array_append(workflow_events, event_data);
            END IF;
            
            -- Migrate associated tasks
            FOR task_record IN 
                SELECT * FROM research_tasks 
                WHERE workflow_id = workflow_record.id 
                ORDER BY created_at ASC
            LOOP
                -- Create TaskCreated event
                sequence_num := sequence_num + 1;
                event_data := jsonb_build_object(
                    'event_id', gen_random_uuid(),
                    'event_type', 'research.task.created',
                    'event_version', 1,
                    'data', jsonb_build_object(
                        'workflow_id', workflow_record.id,
                        'task_id', task_record.id,
                        'task_type', task_record.task_type,
                        'agent_type', task_record.agent_type,
                        'created_at', task_record.created_at,
                        'correlation_id', NULL
                    ),
                    'metadata', jsonb_build_object(
                        'migrated_from', 'research_tasks',
                        'migration_timestamp', NOW()
                    )
                );
                workflow_events := array_append(workflow_events, event_data);
                
                -- Create TaskCompleted event if task was completed
                IF task_record.completed_at IS NOT NULL AND task_record.status = 'completed' THEN
                    sequence_num := sequence_num + 1;
                    event_data := jsonb_build_object(
                        'event_id', gen_random_uuid(),
                        'event_type', 'research.task.completed',
                        'event_version', 1,
                        'data', jsonb_build_object(
                            'workflow_id', workflow_record.id,
                            'task_id', task_record.id,
                            'results', COALESCE(task_record.results, '{}'::jsonb),
                            'completed_at', task_record.completed_at,
                            'correlation_id', NULL
                        ),
                        'metadata', jsonb_build_object(
                            'migrated_from', 'research_tasks',
                            'migration_timestamp', NOW()
                        )
                    );
                    workflow_events := array_append(workflow_events, event_data);
                END IF;
            END LOOP;
            
            -- Insert all events for this workflow using the append_events function
            IF array_length(workflow_events, 1) > 0 THEN
                PERFORM append_events(workflow_record.id::UUID, workflow_events, NULL);
            END IF;
            
            processed_workflows := processed_workflows + 1;
            
            -- Log progress every 100 workflows
            IF processed_workflows % 100 = 0 THEN
                PERFORM log_migration_progress(
                    'migrate_research_workflows', 
                    'running', 
                    processed_workflows, 
                    failed_workflows
                );
            END IF;
            
        EXCEPTION WHEN OTHERS THEN
            failed_workflows := failed_workflows + 1;
            
            -- Log the error but continue with next workflow
            INSERT INTO migration_log (
                migration_name,
                status,
                records_processed,
                records_failed,
                error_message
            ) VALUES (
                'migrate_research_workflows_error',
                'failed',
                processed_workflows,
                failed_workflows,
                'Workflow ID: ' || workflow_record.id || ' - Error: ' || SQLERRM
            );
        END;
    END LOOP;
    
    -- Log final results
    PERFORM log_migration_progress(
        'migrate_research_workflows', 
        CASE WHEN failed_workflows = 0 THEN 'completed' ELSE 'failed' END,
        processed_workflows, 
        failed_workflows,
        CASE WHEN failed_workflows > 0 THEN 'Some workflows failed to migrate' ELSE NULL END
    );
    
    RAISE NOTICE 'Migration completed. Processed: %, Failed: %', processed_workflows, failed_workflows;
END;
$$ LANGUAGE plpgsql;

-- Function to migrate AI agent data to events
CREATE OR REPLACE FUNCTION migrate_ai_agents_to_events()
RETURNS VOID AS $$
DECLARE
    agent_record RECORD;
    event_data JSONB;
    total_agents BIGINT;
    processed_agents BIGINT := 0;
    failed_agents BIGINT := 0;
BEGIN
    -- Log migration start
    PERFORM log_migration_progress('migrate_ai_agents', 'running');
    
    -- Get total count
    SELECT COUNT(*) INTO total_agents FROM ai_agents;
    
    -- Process each agent
    FOR agent_record IN 
        SELECT * FROM ai_agents ORDER BY created_at ASC
    LOOP
        BEGIN
            -- Create AgentCreated event
            event_data := jsonb_build_object(
                'event_id', gen_random_uuid(),
                'event_type', 'ai.agent.created',
                'event_version', 1,
                'data', jsonb_build_object(
                    'agent_id', agent_record.id,
                    'agent_type', agent_record.agent_type,
                    'configuration', COALESCE(agent_record.configuration, '{}'::jsonb),
                    'created_at', agent_record.created_at,
                    'correlation_id', NULL
                ),
                'metadata', jsonb_build_object(
                    'migrated_from', 'ai_agents',
                    'migration_timestamp', NOW()
                )
            );
            
            -- Insert event
            PERFORM append_events(agent_record.id::UUID, ARRAY[event_data], NULL);
            
            processed_agents := processed_agents + 1;
            
        EXCEPTION WHEN OTHERS THEN
            failed_agents := failed_agents + 1;
            
            INSERT INTO migration_log (
                migration_name,
                status,
                records_processed,
                records_failed,
                error_message
            ) VALUES (
                'migrate_ai_agents_error',
                'failed',
                processed_agents,
                failed_agents,
                'Agent ID: ' || agent_record.id || ' - Error: ' || SQLERRM
            );
        END;
    END LOOP;
    
    -- Log final results
    PERFORM log_migration_progress(
        'migrate_ai_agents', 
        CASE WHEN failed_agents = 0 THEN 'completed' ELSE 'failed' END,
        processed_agents, 
        failed_agents,
        CASE WHEN failed_agents > 0 THEN 'Some agents failed to migrate' ELSE NULL END
    );
    
    RAISE NOTICE 'AI Agent migration completed. Processed: %, Failed: %', processed_agents, failed_agents;
END;
$$ LANGUAGE plpgsql;

-- Function to rollback migration
CREATE OR REPLACE FUNCTION rollback_event_migration()
RETURNS VOID AS $$
BEGIN
    -- Log rollback start
    PERFORM log_migration_progress('rollback_migration', 'running');
    
    -- Clear event store data
    DELETE FROM event_store;
    DELETE FROM stream_metadata;
    DELETE FROM snapshots;
    DELETE FROM projection_checkpoints;
    
    -- Restore original data from backups
    DELETE FROM research_workflows;
    INSERT INTO research_workflows SELECT * FROM research_workflows_backup;
    
    DELETE FROM research_tasks;
    INSERT INTO research_tasks SELECT * FROM research_tasks_backup;
    
    -- Log rollback completion
    PERFORM log_migration_progress('rollback_migration', 'completed');
    
    RAISE NOTICE 'Migration rollback completed successfully';
END;
$$ LANGUAGE plpgsql;

-- Function to validate migration results
CREATE OR REPLACE FUNCTION validate_migration()
RETURNS TABLE(
    validation_check VARCHAR(255),
    original_count BIGINT,
    migrated_count BIGINT,
    status VARCHAR(50)
) AS $$
BEGIN
    -- Validate workflow migration
    RETURN QUERY
    SELECT 
        'research_workflows'::VARCHAR(255),
        (SELECT COUNT(*) FROM research_workflows_backup)::BIGINT,
        (SELECT COUNT(DISTINCT stream_id) FROM event_store WHERE stream_id IN (SELECT id::UUID FROM research_workflows_backup))::BIGINT,
        CASE 
            WHEN (SELECT COUNT(*) FROM research_workflows_backup) = 
                 (SELECT COUNT(DISTINCT stream_id) FROM event_store WHERE stream_id IN (SELECT id::UUID FROM research_workflows_backup))
            THEN 'PASS'::VARCHAR(50)
            ELSE 'FAIL'::VARCHAR(50)
        END;
    
    -- Validate event counts
    RETURN QUERY
    SELECT 
        'total_events'::VARCHAR(255),
        0::BIGINT, -- No direct comparison available
        (SELECT COUNT(*) FROM event_store)::BIGINT,
        CASE 
            WHEN (SELECT COUNT(*) FROM event_store) > 0 
            THEN 'PASS'::VARCHAR(50)
            ELSE 'FAIL'::VARCHAR(50)
        END;
    
    -- Validate stream metadata
    RETURN QUERY
    SELECT 
        'stream_metadata'::VARCHAR(255),
        (SELECT COUNT(*) FROM research_workflows_backup)::BIGINT,
        (SELECT COUNT(*) FROM stream_metadata)::BIGINT,
        CASE 
            WHEN (SELECT COUNT(*) FROM research_workflows_backup) <= (SELECT COUNT(*) FROM stream_metadata)
            THEN 'PASS'::VARCHAR(50)
            ELSE 'FAIL'::VARCHAR(50)
        END;
END;
$$ LANGUAGE plpgsql;

-- Grant permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON migration_log TO fdr_app;
GRANT SELECT ON research_workflows_backup TO fdr_app;
GRANT SELECT ON research_tasks_backup TO fdr_app;
GRANT USAGE ON SEQUENCE migration_log_id_seq TO fdr_app;

-- Create indexes for migration performance
CREATE INDEX IF NOT EXISTS idx_migration_log_name_status ON migration_log (migration_name, status);
CREATE INDEX IF NOT EXISTS idx_migration_log_started_at ON migration_log (started_at);

-- Instructions for running the migration
/*
To run the migration:

1. Backup your database first!
   pg_dump -h localhost -U postgres -d free_deep_research > backup_before_migration.sql

2. Run the migration:
   SELECT migrate_research_workflows_to_events();
   SELECT migrate_ai_agents_to_events();

3. Validate the migration:
   SELECT * FROM validate_migration();

4. Check migration logs:
   SELECT * FROM migration_log ORDER BY started_at DESC;

5. If rollback is needed:
   SELECT rollback_event_migration();

Note: This migration assumes the existence of tables:
- research_workflows (id, name, query, methodology, status, created_at, started_at, completed_at, results, error_message, updated_at)
- research_tasks (id, workflow_id, task_type, agent_type, status, created_at, completed_at, results)
- ai_agents (id, agent_type, configuration, created_at)

Adjust the migration functions based on your actual table schemas.
*/
