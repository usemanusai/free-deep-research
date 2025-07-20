-- Event Store Schema for Free Deep Research System
-- Phase 4.1: Event Sourcing Foundation

-- Event Store Table
CREATE TABLE IF NOT EXISTS event_store (
    id BIGSERIAL PRIMARY KEY,
    stream_id UUID NOT NULL,
    event_id UUID NOT NULL UNIQUE,
    event_type VARCHAR(255) NOT NULL,
    event_version INTEGER NOT NULL DEFAULT 1,
    sequence_number BIGINT NOT NULL,
    event_data JSONB NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    correlation_id UUID,
    causation_id UUID,
    
    -- Constraints
    CONSTRAINT unique_stream_sequence UNIQUE (stream_id, sequence_number),
    CONSTRAINT positive_sequence CHECK (sequence_number > 0),
    CONSTRAINT positive_version CHECK (event_version > 0)
);

-- Indexes for performance
CREATE INDEX idx_event_store_stream_id ON event_store (stream_id);
CREATE INDEX idx_event_store_event_type ON event_store (event_type);
CREATE INDEX idx_event_store_timestamp ON event_store (timestamp);
CREATE INDEX idx_event_store_correlation_id ON event_store (correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_event_store_sequence ON event_store (stream_id, sequence_number);

-- Stream Metadata Table
CREATE TABLE IF NOT EXISTS stream_metadata (
    stream_id UUID PRIMARY KEY,
    stream_type VARCHAR(255) NOT NULL,
    current_version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}',
    
    CONSTRAINT positive_version CHECK (current_version >= 0)
);

-- Snapshot Store Table
CREATE TABLE IF NOT EXISTS snapshots (
    id BIGSERIAL PRIMARY KEY,
    stream_id UUID NOT NULL,
    snapshot_version BIGINT NOT NULL,
    snapshot_data JSONB NOT NULL,
    snapshot_metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT unique_stream_snapshot_version UNIQUE (stream_id, snapshot_version),
    CONSTRAINT positive_snapshot_version CHECK (snapshot_version > 0),
    
    -- Foreign key
    FOREIGN KEY (stream_id) REFERENCES stream_metadata(stream_id) ON DELETE CASCADE
);

-- Indexes for snapshots
CREATE INDEX idx_snapshots_stream_id ON snapshots (stream_id);
CREATE INDEX idx_snapshots_version ON snapshots (stream_id, snapshot_version DESC);

-- Projection Checkpoints Table
CREATE TABLE IF NOT EXISTS projection_checkpoints (
    projection_name VARCHAR(255) PRIMARY KEY,
    last_processed_event_id UUID,
    last_processed_sequence BIGINT NOT NULL DEFAULT 0,
    last_processed_timestamp TIMESTAMPTZ,
    projection_status VARCHAR(50) NOT NULL DEFAULT 'active',
    error_count INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_status CHECK (projection_status IN ('active', 'paused', 'error', 'rebuilding'))
);

-- Event Type Registry
CREATE TABLE IF NOT EXISTS event_type_registry (
    event_type VARCHAR(255) PRIMARY KEY,
    schema_version INTEGER NOT NULL DEFAULT 1,
    schema_definition JSONB NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Functions for event store operations
CREATE OR REPLACE FUNCTION update_stream_version()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO stream_metadata (stream_id, stream_type, current_version, updated_at)
    VALUES (NEW.stream_id, 'research_workflow', NEW.sequence_number, NOW())
    ON CONFLICT (stream_id) 
    DO UPDATE SET 
        current_version = NEW.sequence_number,
        updated_at = NOW();
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically update stream version
CREATE TRIGGER trigger_update_stream_version
    AFTER INSERT ON event_store
    FOR EACH ROW
    EXECUTE FUNCTION update_stream_version();

-- Function to get stream version
CREATE OR REPLACE FUNCTION get_stream_version(p_stream_id UUID)
RETURNS BIGINT AS $$
DECLARE
    v_version BIGINT;
BEGIN
    SELECT current_version INTO v_version
    FROM stream_metadata
    WHERE stream_id = p_stream_id;
    
    RETURN COALESCE(v_version, 0);
END;
$$ LANGUAGE plpgsql;

-- Function to append events with optimistic concurrency control
CREATE OR REPLACE FUNCTION append_events(
    p_stream_id UUID,
    p_events JSONB[],
    p_expected_version BIGINT DEFAULT NULL
)
RETURNS BIGINT AS $$
DECLARE
    v_current_version BIGINT;
    v_new_version BIGINT;
    v_event JSONB;
    v_sequence BIGINT;
BEGIN
    -- Get current version
    v_current_version := get_stream_version(p_stream_id);
    
    -- Check expected version if provided
    IF p_expected_version IS NOT NULL AND v_current_version != p_expected_version THEN
        RAISE EXCEPTION 'Concurrency conflict: expected version %, actual version %', 
            p_expected_version, v_current_version;
    END IF;
    
    -- Insert events
    v_sequence := v_current_version;
    FOREACH v_event IN ARRAY p_events
    LOOP
        v_sequence := v_sequence + 1;
        
        INSERT INTO event_store (
            stream_id,
            event_id,
            event_type,
            event_version,
            sequence_number,
            event_data,
            metadata,
            correlation_id,
            causation_id
        ) VALUES (
            p_stream_id,
            (v_event->>'event_id')::UUID,
            v_event->>'event_type',
            COALESCE((v_event->>'event_version')::INTEGER, 1),
            v_sequence,
            v_event->'data',
            COALESCE(v_event->'metadata', '{}'::JSONB),
            NULLIF(v_event->>'correlation_id', '')::UUID,
            NULLIF(v_event->>'causation_id', '')::UUID
        );
    END LOOP;
    
    v_new_version := v_sequence;
    RETURN v_new_version;
END;
$$ LANGUAGE plpgsql;

-- Grant permissions
GRANT SELECT, INSERT, UPDATE ON event_store TO fdr_app;
GRANT SELECT, INSERT, UPDATE ON stream_metadata TO fdr_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON snapshots TO fdr_app;
GRANT SELECT, INSERT, UPDATE ON projection_checkpoints TO fdr_app;
GRANT SELECT, INSERT, UPDATE ON event_type_registry TO fdr_app;
GRANT USAGE ON SEQUENCE event_store_id_seq TO fdr_app;
GRANT USAGE ON SEQUENCE snapshots_id_seq TO fdr_app;

-- Insert initial event types
INSERT INTO event_type_registry (event_type, schema_definition) VALUES
('research.workflow.created', '{"type": "object", "properties": {"workflow_id": {"type": "string"}, "name": {"type": "string"}, "query": {"type": "string"}}}'),
('research.workflow.started', '{"type": "object", "properties": {"workflow_id": {"type": "string"}, "started_at": {"type": "string"}}}'),
('research.workflow.completed', '{"type": "object", "properties": {"workflow_id": {"type": "string"}, "results": {"type": "object"}, "completed_at": {"type": "string"}}}'),
('research.task.created', '{"type": "object", "properties": {"task_id": {"type": "string"}, "workflow_id": {"type": "string"}, "task_type": {"type": "string"}}}'),
('research.task.completed', '{"type": "object", "properties": {"task_id": {"type": "string"}, "results": {"type": "object"}, "completed_at": {"type": "string"}}}'),
('ai.agent.task_assigned', '{"type": "object", "properties": {"agent_id": {"type": "string"}, "task_id": {"type": "string"}, "agent_type": {"type": "string"}}}'),
('ai.agent.response_generated', '{"type": "object", "properties": {"agent_id": {"type": "string"}, "response": {"type": "object"}, "generated_at": {"type": "string"}}}')
ON CONFLICT (event_type) DO NOTHING;
