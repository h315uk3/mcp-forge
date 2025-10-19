//! MCP Forge Server Implementation (v0.8.1 compatible)
//!
//! Refactored implementation using #[tool_router] and #[tool_handler] macros
//! for rmcp v0.8.1 compatibility.

use crate::tool_executor;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, handler::server::router::tool::ToolRouter,
    model::*, schemars, service::RequestContext, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};

/// Request parameters for project generation
///
/// Generates a complete MCP server project structure with standard Rust configuration,
/// dependencies, and template files. The project name will be validated to ensure
/// it's a valid Rust package name (alphanumeric with hyphens/underscores).
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateProjectRequest {
    /// Name of the MCP server project (must be a valid Rust package name)
    pub project_name: String,
    /// Project description (optional, defaults to "A new MCP server project")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request parameters for tool generation
///
/// Generates code template for a new MCP tool with proper structure, error handling,
/// and documentation placeholders. The generated tool will include parameter validation
/// and logging.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateToolRequest {
    /// Name of the tool (used as the function name)
    pub tool_name: String,
    /// Tool description (displayed in MCP tool list and documentation)
    pub description: String,
}

/// Request parameters for resource generation
///
/// Generates a new MCP resource template with proper URI naming, MIME type support,
/// and content structure. Resources can be used for templates, documentation, or data files.
/// Supported resource types: text, binary, json.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateResourceRequest {
    /// Name of the resource (used in the resource URI)
    pub resource_name: String,
    /// Type of resource content: "text", "json", or "binary"
    #[serde(rename = "type")]
    pub resource_type: String,
    /// Resource description (optional, displayed in resource listings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request parameters for README generation
///
/// Generates a comprehensive README.md file with project setup instructions,
/// MCP configuration details, and development guidelines. Includes examples
/// and troubleshooting information.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateReadmeRequest {
    /// Name of the MCP server project (should match project_name from GenerateProjectRequest)
    pub project_name: String,
    /// Project description (optional, used in README header)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Output path for README.md file (optional, defaults to "README.md")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_path: Option<String>,
}

/// Request parameters for manifest validation
///
/// Validates an MCP server manifest file (typically claude_desktop_config.json).
/// Checks for required fields, proper JSON structure, and schema compliance.
/// Returns detailed validation errors if issues are found.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ValidateManifestRequest {
    /// Contents of the manifest file in JSON format (as a string)
    pub manifest_content: String,
}

/// MCP Forge Server implementation using macro-based routing
#[derive(Debug, Clone)]
pub struct MCPForgeServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl MCPForgeServer {
    /// Create a new MCP Forge Server
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Generate a new MCP server project structure
    ///
    /// Creates a complete project directory with standard Rust configuration.
    /// Includes Cargo.toml with MCP dependencies, main.rs template, lib.rs,
    /// and .gitignore. Validates project name format.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - project_name is empty or contains invalid characters
    /// - Project directory already exists
    /// - File system operations fail (permissions, disk space)
    #[tool(description = "Generate a new MCP server project structure")]
    async fn generate_project(
        &self,
        rmcp::handler::server::wrapper::Parameters(req): rmcp::handler::server::wrapper::Parameters<
            GenerateProjectRequest,
        >,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Generating new MCP project: {}", req.project_name);

        let args = serde_json::json!({
            "project_name": req.project_name,
            "description": req.description.as_deref().unwrap_or("A new MCP server project"),
        });

        match tool_executor::execute_tool("generate_project", &args).await {
            Ok(result) => {
                tracing::info!("Project generation completed successfully");
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::error!("Project generation failed: {}", e);
                Err(McpError::internal_error(
                    format!("Failed to generate project: {}", e),
                    None,
                ))
            }
        }
    }

    /// Generate code for a new MCP tool
    ///
    /// Produces a template implementation for a new tool with proper structure,
    /// error handling, logging, and type safety. Includes parameter validation
    /// and async-safe patterns.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - tool_name is empty or contains invalid characters
    /// - description is empty
    #[tool(description = "Generate code for a new MCP tool")]
    async fn generate_tool(
        &self,
        rmcp::handler::server::wrapper::Parameters(req): rmcp::handler::server::wrapper::Parameters<
            GenerateToolRequest,
        >,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Generating tool: {} - {}", req.tool_name, req.description);

        let args = serde_json::json!({
            "tool_name": req.tool_name,
            "description": req.description,
        });

        match tool_executor::execute_tool("generate_tool", &args).await {
            Ok(result) => {
                tracing::info!("Tool generation completed: {}", req.tool_name);
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::error!("Tool generation failed for {}: {}", req.tool_name, e);
                Err(McpError::internal_error(
                    format!("Failed to generate tool: {}", e),
                    None,
                ))
            }
        }
    }

    /// Generate code for a new MCP resource
    ///
    /// Creates a template for a new resource with proper MIME type handling,
    /// URI conventions, and content structure. Supports text, JSON, and binary resources.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - resource_name is empty or contains invalid characters
    /// - resource_type is not one of: text, json, binary
    #[tool(description = "Generate code for a new MCP resource")]
    async fn generate_resource(
        &self,
        rmcp::handler::server::wrapper::Parameters(req): rmcp::handler::server::wrapper::Parameters<
            GenerateResourceRequest,
        >,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!(
            "Generating resource: {} (type: {})",
            req.resource_name,
            req.resource_type
        );

        let args = serde_json::json!({
            "resource_name": req.resource_name,
            "resource_type": req.resource_type,
            "description": req.description.as_deref().unwrap_or(""),
        });

        match tool_executor::execute_tool("generate_resource", &args).await {
            Ok(result) => {
                tracing::info!("Resource generation completed: {}", req.resource_name);
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::error!(
                    "Resource generation failed for {}: {}",
                    req.resource_name,
                    e
                );
                Err(McpError::internal_error(
                    format!("Failed to generate resource: {}", e),
                    None,
                ))
            }
        }
    }

    /// Generate README.md with MCP server setup instructions
    ///
    /// Produces a comprehensive README.md file with setup instructions, configuration
    /// examples, and troubleshooting tips. Includes sections on MCP protocol,
    /// development workflow, and deployment.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - project_name is empty
    /// - output_path is not writable
    /// - File system operations fail
    #[tool(description = "Generate README.md with MCP server setup instructions")]
    async fn generate_readme(
        &self,
        rmcp::handler::server::wrapper::Parameters(req): rmcp::handler::server::wrapper::Parameters<
            GenerateReadmeRequest,
        >,
    ) -> Result<CallToolResult, McpError> {
        let output_path = req.output_path.as_deref().unwrap_or("README.md");
        tracing::info!(
            "Generating README for project: {} -> {}",
            req.project_name,
            output_path
        );

        let args = serde_json::json!({
            "project_name": req.project_name,
            "description": req.description.as_deref().unwrap_or("A new MCP server project"),
            "output_path": output_path,
        });

        match tool_executor::execute_tool("generate_readme", &args).await {
            Ok(result) => {
                tracing::info!("README generation completed at: {}", output_path);
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::error!("README generation failed: {}", e);
                Err(McpError::internal_error(
                    format!("Failed to generate README: {}", e),
                    None,
                ))
            }
        }
    }

    /// Validate an MCP server manifest file
    ///
    /// Checks manifest.json or claude_desktop_config.json for correctness.
    /// Validates required fields, JSON schema compliance, and URI formatting.
    /// Provides detailed error messages for any issues found.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - manifest_content is not valid JSON
    /// - Required fields are missing
    /// - Schema validation fails
    /// - Manifest structure is incorrect
    #[tool(description = "Validate an MCP server manifest file")]
    async fn validate_manifest(
        &self,
        rmcp::handler::server::wrapper::Parameters(req): rmcp::handler::server::wrapper::Parameters<
            ValidateManifestRequest,
        >,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Validating manifest file");

        let args = serde_json::json!({
            "manifest_content": req.manifest_content,
        });

        match tool_executor::execute_tool("validate_manifest", &args).await {
            Ok(result) => {
                tracing::info!("Manifest validation succeeded");
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::warn!("Manifest validation failed: {}", e);
                Err(McpError::internal_error(
                    format!("Manifest validation error: {}", e),
                    None,
                ))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for MCPForgeServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
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
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        use crate::resources::get_available_resources;

        tracing::debug!("Listing available resources");

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
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        use crate::resources::get_available_resources;

        tracing::debug!("Reading resource: {}", request.uri);

        let resources = get_available_resources();

        let resource = resources
            .values()
            .find(|r| r.uri == request.uri)
            .ok_or_else(|| {
                tracing::warn!("Resource not found: {}", request.uri);
                McpError::resource_not_found(
                    format!(
                        "Resource not found: {}. Available resources: {} items",
                        request.uri,
                        resources.len()
                    ),
                    None,
                )
            })?;

        tracing::debug!(
            "Successfully read resource: {} ({} bytes)",
            request.uri,
            resource.content.len()
        );

        Ok(ReadResourceResult {
            contents: vec![ResourceContents::text(&resource.content, &resource.uri)],
        })
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        use crate::prompts::get_available_prompts;

        tracing::debug!("Listing available prompts");

        let prompts = get_available_prompts()
            .into_values()
            .map(|prompt| Prompt {
                name: prompt.name.clone(),
                title: Some(prompt.name.clone()),
                description: Some(prompt.description),
                arguments: if prompt.arguments.is_empty() {
                    None
                } else {
                    Some(
                        prompt
                            .arguments
                            .into_iter()
                            .map(|arg| PromptArgument {
                                name: arg.name,
                                title: None,
                                description: Some(arg.description),
                                required: Some(arg.required),
                            })
                            .collect(),
                    )
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
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        use crate::prompts::get_prompt;

        tracing::debug!("Retrieving prompt: {}", request.name);

        let prompt = get_prompt(&request.name).ok_or_else(|| {
            tracing::warn!("Prompt not found: {}", request.name);
            McpError::invalid_request(format!("Prompt not found: {}", request.name), None)
        })?;

        tracing::debug!("Successfully retrieved prompt: {}", request.name);

        let messages = vec![PromptMessage::new_text(
            PromptMessageRole::User,
            prompt.template,
        )];

        Ok(GetPromptResult {
            description: Some(prompt.description),
            messages,
        })
    }
}

impl Default for MCPForgeServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = MCPForgeServer::new();
        let info = server.get_info();
        assert_eq!(info.server_info.name, "MCP Forge");
    }

    #[test]
    fn test_default_server() {
        let server = MCPForgeServer::default();
        let info = server.get_info();
        assert!(!info.server_info.version.is_empty());
    }
}
