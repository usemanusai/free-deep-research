use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};
use uuid::Uuid;

use crate::error::AppResult;
use crate::services::{Service, DataPersistenceService};
use crate::models::knowledge_graph::*;

pub mod node_manager;
pub mod relationship_manager;
pub mod data_source_manager;
pub mod visualization_engine;
pub mod graph_traversal;
pub mod knowledge_extractor;

use node_manager::NodeManager;
use relationship_manager::RelationshipManager;
use data_source_manager::DataSourceManager;
use visualization_engine::VisualizationEngine;
use graph_traversal::GraphTraversalEngine;
use knowledge_extractor::KnowledgeExtractor;

/// Global Knowledge Graph Service for interconnected knowledge representation
pub struct KnowledgeGraphService {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    node_manager: Arc<RwLock<NodeManager>>,
    relationship_manager: Arc<RwLock<RelationshipManager>>,
    data_source_manager: Arc<RwLock<DataSourceManager>>,
    visualization_engine: Arc<RwLock<VisualizationEngine>>,
    graph_traversal: Arc<RwLock<GraphTraversalEngine>>,
    knowledge_extractor: Arc<RwLock<KnowledgeExtractor>>,
}

impl KnowledgeGraphService {
    pub async fn new(data_persistence: Arc<RwLock<DataPersistenceService>>) -> AppResult<Self> {
        info!("Initializing Knowledge Graph Service");

        let node_manager = Arc::new(RwLock::new(
            NodeManager::new(data_persistence.clone()).await?
        ));

        let relationship_manager = Arc::new(RwLock::new(
            RelationshipManager::new(data_persistence.clone(), node_manager.clone()).await?
        ));

        let data_source_manager = Arc::new(RwLock::new(
            DataSourceManager::new(data_persistence.clone()).await?
        ));

        let visualization_engine = Arc::new(RwLock::new(
            VisualizationEngine::new(node_manager.clone(), relationship_manager.clone()).await?
        ));

        let graph_traversal = Arc::new(RwLock::new(
            GraphTraversalEngine::new(node_manager.clone(), relationship_manager.clone()).await?
        ));

        let knowledge_extractor = Arc::new(RwLock::new(
            KnowledgeExtractor::new(
                node_manager.clone(),
                relationship_manager.clone(),
                data_source_manager.clone(),
            ).await?
        ));

        Ok(Self {
            data_persistence,
            node_manager,
            relationship_manager,
            data_source_manager,
            visualization_engine,
            graph_traversal,
            knowledge_extractor,
        })
    }

    pub async fn create_knowledge_node(&self, node: KnowledgeNode) -> AppResult<KnowledgeNode> {
        info!("Creating knowledge node: {}", node.name);
        let node_manager = self.node_manager.write().await;
        node_manager.create_node(node).await
    }

    pub async fn create_relationship(&self, relationship: KnowledgeRelationship) -> AppResult<KnowledgeRelationship> {
        debug!("Creating relationship: {:?}", relationship.relationship_type);
        let relationship_manager = self.relationship_manager.write().await;
        relationship_manager.create_relationship(relationship).await
    }

    pub async fn register_data_source(&self, source: DataSource) -> AppResult<DataSource> {
        info!("Registering data source: {}", source.source_name);
        let data_source_manager = self.data_source_manager.write().await;
        data_source_manager.register_source(source).await
    }

    pub async fn traverse_graph(&self, request: GraphTraversalRequest) -> AppResult<GraphTraversalResult> {
        debug!("Traversing graph from node: {}", request.start_node_id);
        let graph_traversal = self.graph_traversal.read().await;
        graph_traversal.traverse(request).await
    }

    pub async fn create_visualization(&self, visualization: GraphVisualization) -> AppResult<GraphVisualization> {
        info!("Creating graph visualization: {}", visualization.name);
        let visualization_engine = self.visualization_engine.write().await;
        visualization_engine.create_visualization(visualization).await
    }

    pub async fn extract_knowledge_from_source(&self, source_id: Uuid) -> AppResult<Vec<KnowledgeNode>> {
        info!("Extracting knowledge from source: {}", source_id);
        let knowledge_extractor = self.knowledge_extractor.write().await;
        knowledge_extractor.extract_from_source(source_id).await
    }

    pub async fn get_graph_statistics(&self) -> AppResult<KnowledgeGraphStatistics> {
        debug!("Getting knowledge graph statistics");
        let node_manager = self.node_manager.read().await;
        node_manager.get_graph_statistics().await
    }

    pub async fn search_nodes(&self, query: String, node_types: Option<Vec<NodeType>>) -> AppResult<Vec<KnowledgeNode>> {
        debug!("Searching nodes with query: {}", query);
        let node_manager = self.node_manager.read().await;
        node_manager.search_nodes(query, node_types).await
    }

    pub async fn get_node_neighbors(&self, node_id: Uuid, max_depth: u32) -> AppResult<Vec<KnowledgeNode>> {
        debug!("Getting neighbors for node: {} with depth: {}", node_id, max_depth);
        let graph_traversal = self.graph_traversal.read().await;
        graph_traversal.get_neighbors(node_id, max_depth).await
    }

    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting knowledge graph background tasks...");
        let knowledge_extractor = self.knowledge_extractor.read().await;
        knowledge_extractor.start_continuous_extraction().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for KnowledgeGraphService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing knowledge graph service health check");
        let node_manager = self.node_manager.read().await;
        node_manager.health_check().await
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down knowledge graph service...");
        let knowledge_extractor = self.knowledge_extractor.read().await;
        knowledge_extractor.shutdown().await?;
        Ok(())
    }
}
