// GraphQL Mutation Resolvers for Free Deep Research System
// Phase 4.4: API Gateway & GraphQL

use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;

use crate::{
    types::*,
    AppContext, GraphQLError,
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // Authentication Mutations
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let app_ctx = ctx.data::<AppContext>()?;
        
        // Validate input
        if input.username.is_empty() || input.password.is_empty() {
            return Err(GraphQLError::Validation("Username and password are required".to_string()).into());
        }

        // Authenticate user
        let auth_result = app_ctx.auth_service.login(&input.username, &input.password).await?;
        
        // Record login event
        let login_event = DomainEvent::UserLoggedIn {
            user_id: auth_result.user.id,
            timestamp: chrono::Utc::now(),
            ip_address: ctx.data_opt::<ClientIP>().map(|ip| ip.0.clone()),
            user_agent: ctx.data_opt::<crate::UserAgent>().map(|ua| ua.0.clone()),
        };
        
        app_ctx.event_store.append_event(login_event).await?;
        
        Ok(auth_result)
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let app_ctx = ctx.data::<AppContext>()?;
        
        if let Some(auth_token) = ctx.data_opt::<crate::AuthToken>() {
            let user = app_ctx.auth_service.authenticate(&auth_token.0).await?;
            
            // Invalidate token
            app_ctx.auth_service.logout(&auth_token.0).await?;
            
            // Record logout event
            let logout_event = DomainEvent::UserLoggedOut {
                user_id: user.id,
                timestamp: chrono::Utc::now(),
            };
            
            app_ctx.event_store.append_event(logout_event).await?;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn refresh_token(&self, ctx: &Context<'_>, token: String) -> Result<AuthPayload> {
        let app_ctx = ctx.data::<AppContext>()?;
        app_ctx.auth_service.refresh_token(&token).await.map_err(Into::into)
    }

    async fn update_profile(&self, ctx: &Context<'_>, input: UpdateProfileInput) -> Result<User> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Create update command
        let command = Command::UpdateUserProfile {
            user_id: current_user.id,
            display_name: input.display_name,
            email: input.email,
            preferences: input.preferences,
        };
        
        // Execute command through CQRS
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::UserUpdated(user) => Ok(user),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    // API Key Management Mutations
    async fn create_api_key(&self, ctx: &Context<'_>, input: CreateApiKeyInput) -> Result<ApiKey> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Validate input
        if input.name.is_empty() {
            return Err(GraphQLError::Validation("API key name is required".to_string()).into());
        }
        
        if input.key.is_empty() {
            return Err(GraphQLError::Validation("API key value is required".to_string()).into());
        }

        // Create command
        let command = Command::CreateApiKey {
            user_id: current_user.id,
            name: input.name,
            service: input.service,
            key: input.key,
            rate_limit: input.rate_limit.unwrap_or(1000),
        };
        
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::ApiKeyCreated(api_key) => Ok(api_key),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    async fn update_api_key(&self, ctx: &Context<'_>, id: ID, input: UpdateApiKeyInput) -> Result<ApiKey> {
        let key_id = Uuid::parse_str(&id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify ownership
        let existing_key = app_ctx.api_manager.get_api_key(key_id).await?
            .ok_or_else(|| GraphQLError::Validation("API key not found".to_string()))?;
        
        if existing_key.user_id != current_user.id {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        let command = Command::UpdateApiKey {
            key_id,
            name: input.name,
            rate_limit: input.rate_limit,
            status: input.status,
        };
        
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::ApiKeyUpdated(api_key) => Ok(api_key),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    async fn delete_api_key(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let key_id = Uuid::parse_str(&id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify ownership
        let existing_key = app_ctx.api_manager.get_api_key(key_id).await?
            .ok_or_else(|| GraphQLError::Validation("API key not found".to_string()))?;
        
        if existing_key.user_id != current_user.id {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        let command = Command::DeleteApiKey { key_id };
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::ApiKeyDeleted => Ok(true),
            _ => Ok(false),
        }
    }

    async fn test_api_key(&self, ctx: &Context<'_>, id: ID) -> Result<ApiKeyTestResult> {
        let key_id = Uuid::parse_str(&id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify ownership
        let api_key = app_ctx.api_manager.get_api_key(key_id).await?
            .ok_or_else(|| GraphQLError::Validation("API key not found".to_string()))?;
        
        if api_key.user_id != current_user.id {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        app_ctx.api_manager.test_api_key(key_id).await.map_err(Into::into)
    }

    async fn rotate_api_key(&self, ctx: &Context<'_>, id: ID) -> Result<ApiKey> {
        let key_id = Uuid::parse_str(&id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify ownership
        let existing_key = app_ctx.api_manager.get_api_key(key_id).await?
            .ok_or_else(|| GraphQLError::Validation("API key not found".to_string()))?;
        
        if existing_key.user_id != current_user.id {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        let command = Command::RotateApiKey { key_id };
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::ApiKeyRotated(api_key) => Ok(api_key),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    // Research Workflow Mutations
    async fn create_research_workflow(&self, ctx: &Context<'_>, input: CreateWorkflowInput) -> Result<ResearchWorkflow> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Validate input
        if input.name.is_empty() {
            return Err(GraphQLError::Validation("Workflow name is required".to_string()).into());
        }

        let command = Command::CreateResearchWorkflow {
            creator_id: current_user.id,
            name: input.name,
            description: input.description,
            methodology: input.methodology,
            template_id: input.template_id,
            configuration: input.configuration,
        };
        
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::WorkflowCreated(workflow) => Ok(workflow),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    async fn execute_research_workflow(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: Option<ExecuteWorkflowInput>,
    ) -> Result<WorkflowExecution> {
        let workflow_id = Uuid::parse_str(&id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify access to workflow
        let workflow = app_ctx.research_engine.get_workflow(workflow_id).await?
            .ok_or_else(|| GraphQLError::Validation("Workflow not found".to_string()))?;
        
        if workflow.creator_id != current_user.id && 
           !workflow.collaborators.contains(&current_user.id) &&
           !current_user.is_admin() {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        let command = Command::ExecuteWorkflow {
            workflow_id,
            executor_id: current_user.id,
            parameters: input.map(|i| i.parameters).unwrap_or_default(),
        };
        
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::WorkflowExecutionStarted(execution) => Ok(execution),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    // V3.0.0 Feature Mutations
    async fn create_federated_research(&self, ctx: &Context<'_>, input: FederatedResearchInput) -> Result<FederatedResearchNode> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        let command = Command::CreateFederatedResearch {
            creator_id: current_user.id,
            name: input.name,
            description: input.description,
            network_id: input.network_id,
            node_type: input.node_type,
            capabilities: input.capabilities,
        };
        
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::FederatedResearchCreated(node) => Ok(node),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    async fn create_knowledge_graph(&self, ctx: &Context<'_>, input: CreateKnowledgeGraphInput) -> Result<KnowledgeGraph> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        let command = Command::CreateKnowledgeGraph {
            creator_id: current_user.id,
            name: input.name,
            description: input.description,
            domain: input.domain,
            configuration: input.configuration,
        };
        
        let result = app_ctx.cqrs_service.execute_command(command).await?;
        
        match result {
            CommandResult::KnowledgeGraphCreated(graph) => Ok(graph),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    // BMAD Integration Mutations
    async fn execute_bmad_workflow(
        &self,
        ctx: &Context<'_>,
        agent_id: String,
        input: BMadExecutionInput,
    ) -> Result<BMadExecution> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;

        let command = Command::ExecuteBMadWorkflow {
            agent_id,
            executor_id: current_user.id,
            workflow_input: input.workflow_input,
            parameters: input.parameters,
        };

        let result = app_ctx.cqrs_service.execute_command(command).await?;

        match result {
            CommandResult::BMadExecutionStarted(execution) => Ok(execution),
            _ => Err(GraphQLError::Validation("Invalid command result".to_string()).into()),
        }
    }

    // Phase 4.5: Serverless Function Integration
    async fn execute_research_processing(
        &self,
        ctx: &Context<'_>,
        input: ServerlessResearchInput,
    ) -> Result<ServerlessJobResponse> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;

        // Call serverless research processor function
        let serverless_request = ServerlessRequest {
            workflow_id: input.workflow_id,
            research_query: input.research_query,
            methodology: input.methodology,
            parameters: input.parameters.unwrap_or_default(),
            priority: input.priority.unwrap_or(ProcessingPriority::Normal),
            callback_url: Some(format!("https://graphql.freedeepresearch.org/webhooks/research-complete")),
        };

        let job_response = app_ctx.serverless_service
            .invoke_function("research-processor", serverless_request)
            .await?;

        Ok(ServerlessJobResponse {
            job_id: job_response.job_id,
            status: job_response.status,
            estimated_completion: job_response.estimated_completion,
            function_url: Some("https://functions.freedeepresearch.org/research".to_string()),
        })
    }

    async fn execute_ml_inference(
        &self,
        ctx: &Context<'_>,
        input: MLInferenceInput,
    ) -> Result<MLInferenceResponse> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;

        // Call serverless ML inference function
        let inference_request = InferenceRequest {
            model_name: input.model_name,
            inputs: input.inputs,
            parameters: input.parameters,
            callback_url: None, // Synchronous inference
        };

        let inference_response = app_ctx.serverless_service
            .invoke_function("ml-inference", inference_request)
            .await?;

        Ok(inference_response)
    }

    async fn process_file_serverless(
        &self,
        ctx: &Context<'_>,
        input: FileProcessingInput,
    ) -> Result<FileProcessingResponse> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;

        // Call serverless file processor function
        let processing_request = FileProcessingRequest {
            file_url: input.file_url,
            processing_type: input.processing_type,
            options: input.options.unwrap_or_default(),
            callback_url: Some(format!("https://graphql.freedeepresearch.org/webhooks/file-complete")),
        };

        let processing_response = app_ctx.serverless_service
            .invoke_function("file-processor", processing_request)
            .await?;

        Ok(processing_response)
    }
}

// Helper methods for MutationRoot
impl MutationRoot {
    async fn require_auth(&self, ctx: &Context<'_>) -> Result<User> {
        if let Some(auth_token) = ctx.data_opt::<crate::AuthToken>() {
            let app_ctx = ctx.data::<AppContext>()?;
            app_ctx.auth_service.authenticate(&auth_token.0).await.map_err(Into::into)
        } else {
            Err(GraphQLError::Auth("Authentication required".to_string()).into())
        }
    }
}

// Additional types for client IP tracking
#[derive(Clone)]
pub struct ClientIP(pub String);
