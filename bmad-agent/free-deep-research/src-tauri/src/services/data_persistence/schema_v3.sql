-- Version 3.0.0 Database Schema Extensions
-- Global Intelligence Network Features

-- Federated Research System Tables
CREATE TABLE IF NOT EXISTS federated_organizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL UNIQUE,
    public_key TEXT NOT NULL,
    trust_level INTEGER DEFAULT 0, -- 0-100 trust score
    api_endpoint TEXT NOT NULL,
    contact_email TEXT,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, active, suspended, revoked
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS research_partnerships (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    partner_organization_id TEXT NOT NULL,
    partnership_type TEXT NOT NULL, -- bilateral, multilateral, observer
    data_sharing_level TEXT NOT NULL, -- public, restricted, private
    permissions TEXT, -- JSON array of permissions
    status TEXT NOT NULL DEFAULT 'pending',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    FOREIGN KEY (organization_id) REFERENCES federated_organizations (id),
    FOREIGN KEY (partner_organization_id) REFERENCES federated_organizations (id)
);

CREATE TABLE IF NOT EXISTS shared_research_sessions (
    id TEXT PRIMARY KEY,
    local_workflow_id TEXT NOT NULL,
    sharing_organization_id TEXT NOT NULL,
    shared_data TEXT, -- Encrypted JSON data
    access_permissions TEXT, -- JSON permissions
    sharing_level TEXT NOT NULL, -- metadata_only, partial, full
    expiration_date DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (local_workflow_id) REFERENCES research_workflows (id),
    FOREIGN KEY (sharing_organization_id) REFERENCES federated_organizations (id)
);

-- AI Research Marketplace Tables
CREATE TABLE IF NOT EXISTS marketplace_users (
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
);

CREATE TABLE IF NOT EXISTS ai_agents_marketplace (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL, -- research, analysis, automation, etc.
    creator_id TEXT NOT NULL,
    version TEXT NOT NULL,
    agent_config TEXT NOT NULL, -- JSON configuration
    pricing_model TEXT NOT NULL, -- free, credits, subscription
    price_per_use DECIMAL(10, 4) DEFAULT 0,
    downloads INTEGER DEFAULT 0,
    rating DECIMAL(3, 2) DEFAULT 0,
    rating_count INTEGER DEFAULT 0,
    tags TEXT, -- JSON array of tags
    requirements TEXT, -- JSON system requirements
    status TEXT NOT NULL DEFAULT 'draft', -- draft, published, deprecated
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creator_id) REFERENCES marketplace_users (id)
);

CREATE TABLE IF NOT EXISTS research_methodologies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    creator_id TEXT NOT NULL,
    methodology_config TEXT NOT NULL, -- JSON configuration
    category TEXT NOT NULL,
    complexity_level INTEGER NOT NULL, -- 1-5 scale
    estimated_time_minutes INTEGER,
    success_rate DECIMAL(5, 2),
    usage_count INTEGER DEFAULT 0,
    rating DECIMAL(3, 2) DEFAULT 0,
    rating_count INTEGER DEFAULT 0,
    is_public BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creator_id) REFERENCES marketplace_users (id)
);

CREATE TABLE IF NOT EXISTS community_ratings (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    target_type TEXT NOT NULL, -- agent, methodology, user
    target_id TEXT NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review_text TEXT,
    helpful_votes INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES marketplace_users (id),
    UNIQUE(user_id, target_type, target_id)
);

-- Quantum-Ready Architecture Tables
CREATE TABLE IF NOT EXISTS quantum_algorithms (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    algorithm_type TEXT NOT NULL, -- encryption, signature, key_exchange
    classical_equivalent TEXT,
    quantum_safe BOOLEAN DEFAULT TRUE,
    implementation_status TEXT NOT NULL, -- experimental, stable, deprecated
    performance_metrics TEXT, -- JSON performance data
    security_level INTEGER NOT NULL, -- NIST security levels 1-5
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS compute_resources (
    id TEXT PRIMARY KEY,
    resource_type TEXT NOT NULL, -- classical, quantum, hybrid
    provider TEXT NOT NULL,
    endpoint_url TEXT,
    capabilities TEXT, -- JSON capabilities description
    availability_status TEXT NOT NULL DEFAULT 'available',
    cost_per_operation DECIMAL(10, 8),
    max_qubits INTEGER, -- For quantum resources
    coherence_time_ms DECIMAL(10, 3), -- For quantum resources
    error_rate DECIMAL(10, 8),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_health_check DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS security_protocols (
    id TEXT PRIMARY KEY,
    protocol_name TEXT NOT NULL,
    protocol_version TEXT NOT NULL,
    quantum_safe BOOLEAN DEFAULT FALSE,
    classical_fallback BOOLEAN DEFAULT TRUE,
    implementation_config TEXT, -- JSON configuration
    migration_path TEXT, -- JSON migration instructions
    status TEXT NOT NULL DEFAULT 'active', -- active, deprecated, experimental
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Advanced NLP Engine Tables
CREATE TABLE IF NOT EXISTS nlp_models (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    model_type TEXT NOT NULL, -- transformer, embedding, classification
    provider TEXT NOT NULL, -- local, openrouter, huggingface
    model_path TEXT,
    api_endpoint TEXT,
    capabilities TEXT, -- JSON array of capabilities
    language_support TEXT, -- JSON array of supported languages
    max_tokens INTEGER,
    context_window INTEGER,
    performance_metrics TEXT, -- JSON performance data
    cost_per_token DECIMAL(10, 8),
    status TEXT NOT NULL DEFAULT 'available',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS literature_reviews (
    id TEXT PRIMARY KEY,
    research_query TEXT NOT NULL,
    nlp_model_id TEXT NOT NULL,
    search_parameters TEXT, -- JSON search configuration
    sources_found INTEGER DEFAULT 0,
    sources_analyzed INTEGER DEFAULT 0,
    key_findings TEXT, -- JSON structured findings
    sentiment_analysis TEXT, -- JSON sentiment data
    topic_clusters TEXT, -- JSON topic clustering results
    confidence_score DECIMAL(5, 2),
    processing_time_ms INTEGER,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, processing, completed, failed
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    FOREIGN KEY (nlp_model_id) REFERENCES nlp_models (id)
);

CREATE TABLE IF NOT EXISTS semantic_queries (
    id TEXT PRIMARY KEY,
    original_query TEXT NOT NULL,
    processed_query TEXT NOT NULL,
    intent_classification TEXT, -- JSON intent analysis
    entity_extraction TEXT, -- JSON extracted entities
    query_expansion TEXT, -- JSON expanded query terms
    semantic_embedding BLOB, -- Vector embedding
    nlp_model_id TEXT NOT NULL,
    confidence_score DECIMAL(5, 2),
    processing_time_ms INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (nlp_model_id) REFERENCES nlp_models (id)
);

-- Blockchain Integration Tables
CREATE TABLE IF NOT EXISTS blockchain_transactions (
    id TEXT PRIMARY KEY,
    transaction_hash TEXT NOT NULL UNIQUE,
    block_number INTEGER,
    transaction_type TEXT NOT NULL, -- validation, reward, governance
    from_address TEXT,
    to_address TEXT,
    data_payload TEXT, -- JSON transaction data
    gas_used INTEGER,
    transaction_fee DECIMAL(18, 8),
    status TEXT NOT NULL DEFAULT 'pending', -- pending, confirmed, failed
    confirmations INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    confirmed_at DATETIME
);

CREATE TABLE IF NOT EXISTS peer_reviews (
    id TEXT PRIMARY KEY,
    research_workflow_id TEXT NOT NULL,
    reviewer_id TEXT NOT NULL,
    review_type TEXT NOT NULL, -- methodology, results, quality
    review_score INTEGER CHECK (review_score >= 1 AND review_score <= 10),
    review_comments TEXT,
    review_criteria TEXT, -- JSON structured criteria
    blockchain_transaction_id TEXT,
    review_status TEXT NOT NULL DEFAULT 'draft', -- draft, submitted, validated
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    submitted_at DATETIME,
    FOREIGN KEY (research_workflow_id) REFERENCES research_workflows (id),
    FOREIGN KEY (reviewer_id) REFERENCES marketplace_users (id),
    FOREIGN KEY (blockchain_transaction_id) REFERENCES blockchain_transactions (id)
);

CREATE TABLE IF NOT EXISTS research_validations (
    id TEXT PRIMARY KEY,
    research_workflow_id TEXT NOT NULL,
    validation_type TEXT NOT NULL, -- peer_review, automated, consensus
    validation_score DECIMAL(5, 2),
    validation_criteria TEXT, -- JSON validation criteria
    validator_nodes TEXT, -- JSON array of validator information
    consensus_reached BOOLEAN DEFAULT FALSE,
    blockchain_record_hash TEXT,
    validation_status TEXT NOT NULL DEFAULT 'pending',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    validated_at DATETIME,
    FOREIGN KEY (research_workflow_id) REFERENCES research_workflows (id)
);

CREATE TABLE IF NOT EXISTS token_rewards (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    reward_type TEXT NOT NULL, -- research_contribution, peer_review, validation
    reward_amount DECIMAL(18, 8) NOT NULL,
    research_workflow_id TEXT,
    blockchain_transaction_id TEXT,
    reward_criteria TEXT, -- JSON criteria met
    status TEXT NOT NULL DEFAULT 'pending', -- pending, distributed, failed
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    distributed_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES marketplace_users (id),
    FOREIGN KEY (research_workflow_id) REFERENCES research_workflows (id),
    FOREIGN KEY (blockchain_transaction_id) REFERENCES blockchain_transactions (id)
);

-- Global Knowledge Graph Tables
CREATE TABLE IF NOT EXISTS knowledge_nodes (
    id TEXT PRIMARY KEY,
    node_type TEXT NOT NULL, -- concept, entity, research, methodology
    name TEXT NOT NULL,
    description TEXT,
    properties TEXT, -- JSON node properties
    embedding_vector BLOB, -- Vector representation
    confidence_score DECIMAL(5, 2),
    source_type TEXT, -- research_workflow, external_source, user_input
    source_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS knowledge_relationships (
    id TEXT PRIMARY KEY,
    source_node_id TEXT NOT NULL,
    target_node_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL, -- related_to, part_of, causes, enables
    relationship_strength DECIMAL(5, 2), -- 0.0 to 1.0
    relationship_properties TEXT, -- JSON additional properties
    evidence_sources TEXT, -- JSON array of evidence
    confidence_score DECIMAL(5, 2),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_node_id) REFERENCES knowledge_nodes (id),
    FOREIGN KEY (target_node_id) REFERENCES knowledge_nodes (id)
);

CREATE TABLE IF NOT EXISTS data_sources (
    id TEXT PRIMARY KEY,
    source_name TEXT NOT NULL,
    source_type TEXT NOT NULL, -- academic, web, database, api
    source_url TEXT,
    access_method TEXT, -- api, scraping, manual
    credibility_score DECIMAL(5, 2),
    update_frequency TEXT, -- real_time, daily, weekly, monthly
    last_accessed DATETIME,
    data_format TEXT, -- json, xml, csv, pdf
    access_credentials_id TEXT, -- Reference to encrypted credentials
    status TEXT NOT NULL DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS graph_visualizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    graph_query TEXT NOT NULL, -- Query to generate the graph
    layout_config TEXT, -- JSON layout configuration
    visual_config TEXT, -- JSON visual styling
    node_filters TEXT, -- JSON node filtering criteria
    relationship_filters TEXT, -- JSON relationship filtering
    created_by TEXT,
    is_public BOOLEAN DEFAULT FALSE,
    usage_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_federated_orgs_domain ON federated_organizations(domain);
CREATE INDEX IF NOT EXISTS idx_federated_orgs_status ON federated_organizations(status);
CREATE INDEX IF NOT EXISTS idx_partnerships_org ON research_partnerships(organization_id);
CREATE INDEX IF NOT EXISTS idx_partnerships_partner ON research_partnerships(partner_organization_id);
CREATE INDEX IF NOT EXISTS idx_shared_sessions_workflow ON shared_research_sessions(local_workflow_id);
CREATE INDEX IF NOT EXISTS idx_marketplace_users_username ON marketplace_users(username);
CREATE INDEX IF NOT EXISTS idx_marketplace_users_email ON marketplace_users(email);
CREATE INDEX IF NOT EXISTS idx_ai_agents_creator ON ai_agents_marketplace(creator_id);
CREATE INDEX IF NOT EXISTS idx_ai_agents_category ON ai_agents_marketplace(category);
CREATE INDEX IF NOT EXISTS idx_ai_agents_status ON ai_agents_marketplace(status);
CREATE INDEX IF NOT EXISTS idx_methodologies_creator ON research_methodologies(creator_id);
CREATE INDEX IF NOT EXISTS idx_methodologies_category ON research_methodologies(category);
CREATE INDEX IF NOT EXISTS idx_ratings_user ON community_ratings(user_id);
CREATE INDEX IF NOT EXISTS idx_ratings_target ON community_ratings(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_quantum_algorithms_type ON quantum_algorithms(algorithm_type);
CREATE INDEX IF NOT EXISTS idx_compute_resources_type ON compute_resources(resource_type);
CREATE INDEX IF NOT EXISTS idx_security_protocols_status ON security_protocols(status);
CREATE INDEX IF NOT EXISTS idx_nlp_models_type ON nlp_models(model_type);
CREATE INDEX IF NOT EXISTS idx_nlp_models_provider ON nlp_models(provider);
CREATE INDEX IF NOT EXISTS idx_literature_reviews_model ON literature_reviews(nlp_model_id);
CREATE INDEX IF NOT EXISTS idx_semantic_queries_model ON semantic_queries(nlp_model_id);
CREATE INDEX IF NOT EXISTS idx_blockchain_tx_hash ON blockchain_transactions(transaction_hash);
CREATE INDEX IF NOT EXISTS idx_blockchain_tx_type ON blockchain_transactions(transaction_type);
CREATE INDEX IF NOT EXISTS idx_peer_reviews_workflow ON peer_reviews(research_workflow_id);
CREATE INDEX IF NOT EXISTS idx_peer_reviews_reviewer ON peer_reviews(reviewer_id);
CREATE INDEX IF NOT EXISTS idx_research_validations_workflow ON research_validations(research_workflow_id);
CREATE INDEX IF NOT EXISTS idx_token_rewards_user ON token_rewards(user_id);
CREATE INDEX IF NOT EXISTS idx_knowledge_nodes_type ON knowledge_nodes(node_type);
CREATE INDEX IF NOT EXISTS idx_knowledge_relationships_source ON knowledge_relationships(source_node_id);
CREATE INDEX IF NOT EXISTS idx_knowledge_relationships_target ON knowledge_relationships(target_node_id);
CREATE INDEX IF NOT EXISTS idx_data_sources_type ON data_sources(source_type);
CREATE INDEX IF NOT EXISTS idx_graph_visualizations_public ON graph_visualizations(is_public);
