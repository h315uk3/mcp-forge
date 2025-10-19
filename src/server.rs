//! MCP Forge Server Implementation
//!
//! Main server implementation that provides tools and resources for MCP development.

use crate::tool_executor;
use rmcp::{
    ServerHandler,
    model::*,
    service::RequestContext,
};
use std::{sync::Arc, borrow::Cow};

/// MCP Forge Server implementation
///
/// The main server struct that implements the MCP `ServerHandler` trait.
/// Provides tools, resources, and prompts for MCP server development.
#[derive(Debug, Clone, Default)]
pub struct MCPForgeServer;

impl MCPForgeServer {
    /// Create a new MCP Forge Server
    pub fn new() -> Self {
        Self
    }
}

impl ServerHandler for MCPForgeServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
            server_info: Implementation {
                name: "MCP Forge".to_string(),
                title: Some("MCP Development Framework".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
                website_url: Some("https://github.com/h315uk3/mcp-forge".to_string()),
                icons: None,
            },
            instructions: Some(
                "A development framework for building MCP servers with Rust SDK. \
                 Provides tools for project generation, code templates, and documentation."
                    .to_string(),
            ),
            ..Default::default()
        }
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_name = &request.name;
        let arguments_map = request.arguments.unwrap_or_default();
        let arguments = serde_json::Value::Object(arguments_map);

        tracing::info!("Calling tool: {}", tool_name);
        tracing::debug!("Tool arguments: {:?}", arguments);

        match tool_executor::execute_tool(tool_name, &arguments).await {
            Ok(content) => Ok(CallToolResult::success(vec![Content::text(content)])),
            Err(e) => {
                tracing::error!("Tool execution error: {}", e);
                Err(ErrorData {
                    code: ErrorCode(-32603),
                    message: Cow::Borrowed("Tool execution failed"),
                    data: Some(serde_json::Value::String(e)),
                })
            }
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        use crate::tools::get_available_tools;

        let tools = get_available_tools()
            .into_iter()
            .map(|tool| {
                let input_schema = match tool.input_schema {
                    serde_json::Value::Object(map) => Arc::new(map),
                    _ => Arc::new(serde_json::Map::new()),
                };

                Tool {
                    name: Cow::Owned(tool.name),
                    title: None,
                    description: Some(Cow::Owned(tool.description)),
                    input_schema,
                    output_schema: None,
                    annotations: None,
                    icons: None,
                }
            })
            .collect();

        Ok(ListToolsResult { 
            tools,
            next_cursor: None,
        })
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        use crate::resources::get_available_resources;

        let resources = get_available_resources()
            .into_values()
            .map(|resource| {
                let raw_resource = RawResource {
                    uri: resource.uri.clone(),
                    name: resource.name.clone(),
                    title: None,
                    description: None,
                    mime_type: Some(resource.mime_type.clone()),
                    size: None,
                    icons: None,
                };
                Resource {
                    raw: raw_resource,
                    annotations: None,
                }
            })
            .collect();

        Ok(ListResourcesResult { 
            resources,
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        use crate::resources::get_available_resources;

        let resources = get_available_resources();
        
        let resource = resources.values()
            .find(|r| r.uri == request.uri)
            .ok_or_else(|| ErrorData {
                code: ErrorCode(-32602),
                message: Cow::Borrowed("Resource not found"),
                data: Some(serde_json::Value::String(request.uri.clone())),
            })?;

        Ok(ReadResourceResult {
            contents: vec![ResourceContents::text(&resource.content, &resource.uri)],
        })
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        use crate::prompts::get_available_prompts;

        let prompts = get_available_prompts().into_values().map(|prompt| Prompt {
                name: prompt.name.clone(),
                title: Some(prompt.name.clone()),
                description: Some(prompt.description),
                arguments: if prompt.arguments.is_empty() {
                    None
                } else {
                    Some(prompt.arguments.into_iter().map(|arg| PromptArgument {
                        name: arg.name,
                        title: None,
                        description: Some(arg.description),
                        required: Some(arg.required),
                    }).collect())
                },
                icons: None,
            })
            .collect();

        Ok(ListPromptsResult { 
            prompts,
            next_cursor: None,
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        use crate::prompts::get_prompt;

        let prompt = get_prompt(&request.name)
            .ok_or_else(|| ErrorData {
                code: ErrorCode(-32602),
                message: Cow::Borrowed("Prompt not found"),
                data: Some(serde_json::Value::String(request.name.clone())),
            })?;

        let messages = vec![
            PromptMessage::new_text(
                PromptMessageRole::User,
                prompt.template,
            ),
        ];

        Ok(GetPromptResult { 
            description: Some(prompt.description),
            messages,
        })
    }
}
