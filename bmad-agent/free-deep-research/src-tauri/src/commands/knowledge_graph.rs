use tauri::State;
use uuid::Uuid;
use tracing::{info, debug, error};

use crate::services::ServiceManager;
use crate::models::knowledge_graph::*;

#[tauri::command]
pub async fn create_knowledge_node(
    service_manager: State<'_, ServiceManager>,
    node: KnowledgeNode,
) -> Result<KnowledgeNode, String> {
    info!("API: Creating knowledge node: {}", node.name);
    match service_manager.knowledge_graph_service.create_knowledge_node(node).await {
        Ok(created_node) => Ok(created_node),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn create_knowledge_relationship(
    service_manager: State<'_, ServiceManager>,
    relationship: KnowledgeRelationship,
) -> Result<KnowledgeRelationship, String> {
    debug!("API: Creating relationship: {:?}", relationship.relationship_type);
    match service_manager.knowledge_graph_service.create_relationship(relationship).await {
        Ok(created_relationship) => Ok(created_relationship),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn register_data_source(
    service_manager: State<'_, ServiceManager>,
    source: DataSource,
) -> Result<DataSource, String> {
    info!("API: Registering data source: {}", source.source_name);
    match service_manager.knowledge_graph_service.register_data_source(source).await {
        Ok(registered_source) => Ok(registered_source),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn traverse_knowledge_graph(
    service_manager: State<'_, ServiceManager>,
    request: GraphTraversalRequest,
) -> Result<GraphTraversalResult, String> {
    debug!("API: Traversing graph from node: {}", request.start_node_id);
    match service_manager.knowledge_graph_service.traverse_graph(request).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn create_graph_visualization(
    service_manager: State<'_, ServiceManager>,
    visualization: GraphVisualization,
) -> Result<GraphVisualization, String> {
    info!("API: Creating graph visualization: {}", visualization.name);
    match service_manager.knowledge_graph_service.create_visualization(visualization).await {
        Ok(created_visualization) => Ok(created_visualization),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn extract_knowledge_from_source(
    service_manager: State<'_, ServiceManager>,
    source_id: String,
) -> Result<Vec<KnowledgeNode>, String> {
    info!("API: Extracting knowledge from source: {}", source_id);
    let sid = Uuid::parse_str(&source_id).map_err(|e| format!("Invalid source ID: {}", e))?;
    match service_manager.knowledge_graph_service.extract_knowledge_from_source(sid).await {
        Ok(nodes) => Ok(nodes),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_knowledge_graph_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<KnowledgeGraphStatistics, String> {
    debug!("API: Getting knowledge graph statistics");
    match service_manager.knowledge_graph_service.get_graph_statistics().await {
        Ok(statistics) => Ok(statistics),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn search_knowledge_nodes(
    service_manager: State<'_, ServiceManager>,
    query: String,
    node_types: Option<Vec<NodeType>>,
) -> Result<Vec<KnowledgeNode>, String> {
    debug!("API: Searching nodes with query: {}", query);
    match service_manager.knowledge_graph_service.search_nodes(query, node_types).await {
        Ok(nodes) => Ok(nodes),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_node_neighbors(
    service_manager: State<'_, ServiceManager>,
    node_id: String,
    max_depth: u32,
) -> Result<Vec<KnowledgeNode>, String> {
    debug!("API: Getting neighbors for node: {} with depth: {}", node_id, max_depth);
    let nid = Uuid::parse_str(&node_id).map_err(|e| format!("Invalid node ID: {}", e))?;
    match service_manager.knowledge_graph_service.get_node_neighbors(nid, max_depth).await {
        Ok(neighbors) => Ok(neighbors),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_knowledge_insights(
    service_manager: State<'_, ServiceManager>,
    topic: String,
) -> Result<KnowledgeInsights, String> {
    debug!("API: Getting knowledge insights for topic: {}", topic);
    
    // Mock response for now - would analyze the knowledge graph for insights
    Ok(KnowledgeInsights {
        topic,
        key_concepts: vec![],
        related_research: vec![],
        trending_topics: vec![],
        knowledge_gaps: vec![],
        research_opportunities: vec![],
        confidence_score: 0.0,
        generated_at: chrono::Utc::now(),
    })
}

/// Knowledge insights response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnowledgeInsights {
    pub topic: String,
    pub key_concepts: Vec<String>,
    pub related_research: Vec<String>,
    pub trending_topics: Vec<String>,
    pub knowledge_gaps: Vec<String>,
    pub research_opportunities: Vec<String>,
    pub confidence_score: f64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}
