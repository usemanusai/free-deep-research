use tauri::State;
use uuid::Uuid;
use tracing::{info, debug, error};

use crate::services::ServiceManager;
use crate::models::blockchain::*;

#[tauri::command]
pub async fn submit_peer_review(
    service_manager: State<'_, ServiceManager>,
    review: PeerReview,
) -> Result<PeerReview, String> {
    info!("API: Submitting peer review for workflow: {}", review.research_workflow_id);
    match service_manager.blockchain_service.submit_peer_review(review).await {
        Ok(submitted_review) => Ok(submitted_review),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn validate_research(
    service_manager: State<'_, ServiceManager>,
    validation: ResearchValidation,
) -> Result<ResearchValidation, String> {
    info!("API: Validating research workflow: {}", validation.research_workflow_id);
    match service_manager.blockchain_service.validate_research(validation).await {
        Ok(validated_research) => Ok(validated_research),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn distribute_token_rewards(
    service_manager: State<'_, ServiceManager>,
    user_id: String,
    reward: TokenReward,
) -> Result<TokenReward, String> {
    info!("API: Distributing reward to user: {}", user_id);
    let uid = Uuid::parse_str(&user_id).map_err(|e| format!("Invalid user ID: {}", e))?;
    match service_manager.blockchain_service.distribute_rewards(uid, reward).await {
        Ok(distributed_reward) => Ok(distributed_reward),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn create_blockchain_transaction(
    service_manager: State<'_, ServiceManager>,
    transaction: BlockchainTransaction,
) -> Result<BlockchainTransaction, String> {
    debug!("API: Creating blockchain transaction: {:?}", transaction.transaction_type);
    match service_manager.blockchain_service.create_transaction(transaction).await {
        Ok(created_transaction) => Ok(created_transaction),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_audit_trail(
    service_manager: State<'_, ServiceManager>,
    resource: String,
) -> Result<Vec<AuditTrailEntry>, String> {
    debug!("API: Getting audit trail for resource: {}", resource);
    match service_manager.blockchain_service.get_audit_trail(resource).await {
        Ok(audit_trail) => Ok(audit_trail),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_blockchain_network_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<NetworkStatistics, String> {
    debug!("API: Getting blockchain network statistics");
    match service_manager.blockchain_service.get_network_statistics().await {
        Ok(statistics) => Ok(statistics),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn get_user_token_balance(
    service_manager: State<'_, ServiceManager>,
    user_id: String,
) -> Result<UserTokenBalance, String> {
    debug!("API: Getting token balance for user: {}", user_id);
    let uid = Uuid::parse_str(&user_id).map_err(|e| format!("Invalid user ID: {}", e))?;
    
    // Mock response for now
    Ok(UserTokenBalance {
        user_id: uid,
        total_balance: 0.0,
        available_balance: 0.0,
        pending_rewards: 0.0,
        total_earned: 0.0,
        total_spent: 0.0,
        last_updated: chrono::Utc::now(),
    })
}

#[tauri::command]
pub async fn get_research_validation_status(
    service_manager: State<'_, ServiceManager>,
    workflow_id: String,
) -> Result<ValidationStatusInfo, String> {
    debug!("API: Getting validation status for workflow: {}", workflow_id);
    let wid = Uuid::parse_str(&workflow_id).map_err(|e| format!("Invalid workflow ID: {}", e))?;
    
    // Mock response for now
    Ok(ValidationStatusInfo {
        workflow_id: wid,
        validation_status: ValidationStatus::Pending,
        peer_reviews_count: 0,
        required_reviews: 3,
        consensus_score: 0.0,
        validation_deadline: None,
        blockchain_hash: None,
        last_updated: chrono::Utc::now(),
    })
}

/// User token balance information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserTokenBalance {
    pub user_id: Uuid,
    pub total_balance: f64,
    pub available_balance: f64,
    pub pending_rewards: f64,
    pub total_earned: f64,
    pub total_spent: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Validation status information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidationStatusInfo {
    pub workflow_id: Uuid,
    pub validation_status: ValidationStatus,
    pub peer_reviews_count: u32,
    pub required_reviews: u32,
    pub consensus_score: f64,
    pub validation_deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub blockchain_hash: Option<String>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
