use std::sync::Arc;
use tabby_index_cli;
use rmcp::{
    ErrorData as McpError,
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
    model::*
};
use tabby_common::config::{config_index_to_id, CodeRepository, HttpModelConfigBuilder, RateLimitBuilder};
use serde_json::json;
// #[allow(dead_code)]
// pub trait DataService: Send + Sync + 'static {
//     fn get_data(&self) -> String;
//     fn set_data(&mut self, data: String);
// }

// #[derive(Debug, Clone)]
// pub struct MemoryDataService {
//     data: String,
// }

// impl MemoryDataService {
//     #[allow(dead_code)]
//     pub fn new(initial_data: impl Into<String>) -> Self {
//         Self {
//             data: initial_data.into(),
//         }
//     }
// }

// impl DataService for MemoryDataService {
//     fn get_data(&self) -> String {
//         self.data.clone()
//     }
//
//     fn set_data(&mut self, data: String) {
//         self.data = data;
//     }
// }

#[derive(Debug, Clone)]
pub struct CodeSearchService {
    #[allow(dead_code)]
    // data_service: Arc<DS>,
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct SetCodeRequest {
    pub query: String,
    pub repository: String
}

#[tool_router]
impl CodeSearchService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            // data_service: Arc::new(data_service),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "get related code from service")]
    pub async fn get_code(&self,
                          Parameters(SetCodeRequest { query, repository }): Parameters<SetCodeRequest>, ) -> Result<CallToolResult, McpError> {
        tracing::debug!("log: {:?}", query);
        let res = tabby_index_cli::code_search(query, repository).await;
        tracing::debug!("log: {:?}", res);
        if let Ok(r) = res {
            Ok(CallToolResult::success(vec![Content::text(
                r,
            )]))
        } else {
            Err(McpError::new(ErrorCode::INTERNAL_ERROR, res.unwrap_err(), None))
        }
        // self.data_service.get_data()
    }

    #[tool(description = "add code to the service")]
    pub async fn add_code(&self,
                          Parameters(SetCodeRequest { query, repository }): Parameters<SetCodeRequest>, ) -> Result<CallToolResult, McpError> {
        tracing::debug!("log: {:?}", query);
        tracing::debug!("log: {:?}", repository);
        let res = tabby_index_cli::code_ingest(repository.clone()).await;
        tracing::debug!("log: {:?}", res);
        if let Ok(res) = res {
            Ok(CallToolResult::success(vec![Content::text(
                res,
            )]))
        } else {
            Err(McpError::internal_error(
                format!("Failed to add code: {}", res.clone().unwrap_err()),
                Some(json!({
                            "code": repository,
                            "error": res.unwrap_err()
                        })),
            ))
            // Err(McpError::new(ErrorCode::INTERNAL_ERROR, res.unwrap_err(), None))
        }
        // self.data_service.get_data()
    }


    #[tool(description = "add code to the service")]
    pub async fn get_available_repos(&self) -> Result<CallToolResult, McpError> {
            let res = tabby_index_cli::get_repos().await;
            if let Ok(res) = res {
                Ok(CallToolResult::success(vec![Content::text(
                    res,
                )]))
            } else {
                Err(McpError::new(ErrorCode::INTERNAL_ERROR, res.unwrap_err(), None))
            }
        }
}
    //
    // #[tool(description = "set index to service")]
    // pub async fn set_index(
    //     &self,
    //     Parameters(SetCodeRequest { query, repository }): Parameters<SetCodeRequest>,
    // ) -> String {
    //     let new_data = data.clone();
    //     format!("Current memory: {}", new_data)
    // }


#[tool_handler]
impl ServerHandler for CodeSearchService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("generic data service".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
