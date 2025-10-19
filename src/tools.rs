//! Tool definitions for MCP Forge
//!
//! Defines the tools available for MCP server development, such as:
//! - Project generation
//! - Tool code generation
//! - Resource creation
//! - Manifest validation

use serde::{Deserialize, Serialize};

/// Tool definition structure for MCP
///
/// Defines a tool with its name, description, and JSON Schema for input validation.
/// Tools are the primary mechanism for invoking MCP Forge functionality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Tool input schema (JSON Schema)
    pub input_schema: serde_json::Value,
}

impl ToolDefinition {
    /// Create a new tool definition
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema: serde_json::json!({}),
        }
    }

    /// Set the input schema
    pub fn with_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = schema;
        self
    }
}

/// Get all available tools for MCP Forge
pub fn get_available_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition::new(
            "generate_project",
            "Generate a new MCP server project structure",
        )
        .with_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "project_name": {
                    "type": "string",
                    "description": "Name of the MCP server project"
                },
                "description": {
                    "type": "string",
                    "description": "Project description"
                }
            },
            "required": ["project_name"]
        })),
        ToolDefinition::new("generate_tool", "Generate code for a new MCP tool").with_schema(
            serde_json::json!({
                "type": "object",
                "properties": {
                    "tool_name": {
                        "type": "string",
                        "description": "Name of the tool"
                    },
                    "description": {
                        "type": "string",
                        "description": "Tool description"
                    }
                },
                "required": ["tool_name", "description"]
            }),
        ),
        ToolDefinition::new("generate_resource", "Generate code for a new MCP resource")
            .with_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "resource_name": {
                        "type": "string",
                        "description": "Name of the resource"
                    },
                    "resource_type": {
                        "type": "string",
                        "enum": ["text", "binary", "json"],
                        "description": "Type of resource content"
                    },
                    "description": {
                        "type": "string",
                        "description": "Resource description"
                    }
                },
                "required": ["resource_name", "resource_type"]
            })),
        ToolDefinition::new(
            "generate_readme",
            "Generate README.md with MCP server setup instructions",
        )
        .with_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "project_name": {
                    "type": "string",
                    "description": "Name of the MCP server project"
                },
                "description": {
                    "type": "string",
                    "description": "Project description"
                },
                "output_path": {
                    "type": "string",
                    "description": "Output path for README.md (defaults to README.md)"
                }
            },
            "required": ["project_name"]
        })),
        ToolDefinition::new("validate_manifest", "Validate an MCP server manifest file")
            .with_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "manifest_content": {
                        "type": "string",
                        "description": "Contents of the manifest file (JSON format)"
                    }
                },
                "required": ["manifest_content"]
            })),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_available_tools() {
        let tools = get_available_tools();
        assert_eq!(tools.len(), 5);
        assert_eq!(tools[0].name, "generate_project");
    }

    #[test]
    fn test_tool_definition() {
        let tool = ToolDefinition::new("test_tool", "A test tool");
        assert_eq!(tool.name, "test_tool");
        assert_eq!(tool.description, "A test tool");
    }

    #[test]
    fn test_tool_names() {
        let tools = get_available_tools();
        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(tool_names.contains(&"generate_project"));
        assert!(tool_names.contains(&"generate_tool"));
        assert!(tool_names.contains(&"generate_resource"));
        assert!(tool_names.contains(&"generate_readme"));
        assert!(tool_names.contains(&"validate_manifest"));
    }
}
