use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};
use uuid::Uuid;

use crate::error::AppResult;
use crate::services::{Service, DataPersistenceService};
use crate::models::blockchain::*;

pub mod transaction_manager;
pub mod peer_review_manager;
pub mod validation_engine;
pub mod reward_system;
pub mod consensus_manager;
pub mod audit_trail;

use transaction_manager::TransactionManager;
use peer_review_manager::PeerReviewManager;
use validation_engine::ValidationEngine;
use reward_system::RewardSystem;
use consensus_manager::ConsensusManager;
use audit_trail::AuditTrailManager;

/// Blockchain Integration Service for decentralized research validation
pub struct BlockchainService {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    transaction_manager: Arc<RwLock<TransactionManager>>,
    peer_review_manager: Arc<RwLock<PeerReviewManager>>,
    validation_engine: Arc<RwLock<ValidationEngine>>,
    reward_system: Arc<RwLock<RewardSystem>>,
    consensus_manager: Arc<RwLock<ConsensusManager>>,
    audit_trail: Arc<RwLock<AuditTrailManager>>,
}

impl BlockchainService {
    pub async fn new(data_persistence: Arc<RwLock<DataPersistenceService>>) -> AppResult<Self> {
        info!("Initializing Blockchain Service");

        let transaction_manager = Arc::new(RwLock::new(
            TransactionManager::new(data_persistence.clone()).await?
        ));

        let peer_review_manager = Arc::new(RwLock::new(
            PeerReviewManager::new(data_persistence.clone(), transaction_manager.clone()).await?
        ));

        let validation_engine = Arc::new(RwLock::new(
            ValidationEngine::new(data_persistence.clone(), transaction_manager.clone()).await?
        ));

        let reward_system = Arc::new(RwLock::new(
            RewardSystem::new(data_persistence.clone(), transaction_manager.clone()).await?
        ));

        let consensus_manager = Arc::new(RwLock::new(
            ConsensusManager::new().await?
        ));

        let audit_trail = Arc::new(RwLock::new(
            AuditTrailManager::new(data_persistence.clone(), transaction_manager.clone()).await?
        ));

        Ok(Self {
            data_persistence,
            transaction_manager,
            peer_review_manager,
            validation_engine,
            reward_system,
            consensus_manager,
            audit_trail,
        })
    }

    pub async fn submit_peer_review(&self, review: PeerReview) -> AppResult<PeerReview> {
        info!("Submitting peer review for workflow: {}", review.research_workflow_id);
        let peer_review_manager = self.peer_review_manager.write().await;
        peer_review_manager.submit_review(review).await
    }

    pub async fn validate_research(&self, validation: ResearchValidation) -> AppResult<ResearchValidation> {
        info!("Validating research workflow: {}", validation.research_workflow_id);
        let validation_engine = self.validation_engine.write().await;
        validation_engine.validate_research(validation).await
    }

    pub async fn distribute_rewards(&self, user_id: Uuid, reward: TokenReward) -> AppResult<TokenReward> {
        info!("Distributing reward to user: {}", user_id);
        let reward_system = self.reward_system.write().await;
        reward_system.distribute_reward(user_id, reward).await
    }

    pub async fn create_transaction(&self, transaction: BlockchainTransaction) -> AppResult<BlockchainTransaction> {
        debug!("Creating blockchain transaction: {}", transaction.transaction_type);
        let transaction_manager = self.transaction_manager.write().await;
        transaction_manager.create_transaction(transaction).await
    }

    pub async fn get_audit_trail(&self, resource: String) -> AppResult<Vec<AuditTrailEntry>> {
        debug!("Getting audit trail for resource: {}", resource);
        let audit_trail = self.audit_trail.read().await;
        audit_trail.get_audit_trail(resource).await
    }

    pub async fn get_network_statistics(&self) -> AppResult<NetworkStatistics> {
        debug!("Getting blockchain network statistics");
        let consensus_manager = self.consensus_manager.read().await;
        consensus_manager.get_network_statistics().await
    }

    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting blockchain background tasks...");
        let consensus_manager = self.consensus_manager.read().await;
        consensus_manager.start_consensus_process().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for BlockchainService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing blockchain service health check");
        let transaction_manager = self.transaction_manager.read().await;
        transaction_manager.health_check().await
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down blockchain service...");
        let consensus_manager = self.consensus_manager.read().await;
        consensus_manager.shutdown().await?;
        Ok(())
    }
}
