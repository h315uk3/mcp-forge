//! MCP Forge Prompts
//!
//! Provides reusable prompt templates for each tool in MCP Forge.
//! These prompts help Claude understand how to use mcp-forge tools
//! and can be referenced via @mcp-forge syntax.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a reusable prompt template for MCP Forge
///
/// A prompt is a multi-message template that guides Claude in using specific MCP Forge tools.
/// It can include placeholders for dynamic arguments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    /// Prompt name
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Prompt template with placeholders
    pub template: String,
    /// Arguments that can be provided to the template
    pub arguments: Vec<PromptArgument>,
}

/// Represents a single argument for a prompt template
///
/// Defines an argument that can be substituted into a prompt template,
/// specifying its name, description, and whether it is required.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    /// Argument name
    pub name: String,
    /// Argument description
    pub description: String,
    /// Whether argument is required
    pub required: bool,
}

impl Prompt {
    /// Create a new prompt
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        template: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            template: template.into(),
            arguments: Vec::new(),
        }
    }

    /// Add an argument to the prompt
    pub fn with_argument(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        self.arguments.push(PromptArgument {
            name: name.into(),
            description: description.into(),
            required,
        });
        self
    }
}

/// Get all available prompts
pub fn get_available_prompts() -> HashMap<String, Prompt> {
    let mut prompts = HashMap::new();

    // Generate Project prompt
    prompts.insert(
        "generate-project".to_string(),
        Prompt::new(
            "generate-project",
            "Generate a new MCP server project",
            "Use the generate_project tool to create a new MCP server project.\n\n\
             Parameters:\n\
             - project_name: Name of the new project (required)\n\
             - description: Project description (optional)\n\n\
             Example usage:\n\
             Generate an MCP server project named '{project_name}' with description '{description}'."
        )
        .with_argument("project_name", "The name of the project to create", true)
        .with_argument("description", "A brief description of what the server does", false),
    );

    // Generate Tool prompt
    prompts.insert(
        "generate-tool".to_string(),
        Prompt::new(
            "generate-tool",
            "Generate code template for a new MCP tool",
            "Use the generate_tool tool to create Rust code template for a new MCP tool.\n\n\
             Parameters:\n\
             - tool_name: Name of the tool (required)\n\
             - description: What the tool does (required)\n\n\
             Example usage:\n\
             Generate a tool named '{tool_name}' that {description}.",
        )
        .with_argument("tool_name", "The name of the tool to generate", true)
        .with_argument("description", "Description of what the tool does", true),
    );

    // Generate Resource prompt
    prompts.insert(
        "generate-resource".to_string(),
        Prompt::new(
            "generate-resource",
            "Generate code template for a new MCP resource",
            "Use the generate_resource tool to create Rust code template for a new MCP resource.\n\n\
             Parameters:\n\
             - resource_name: Name of the resource (snake_case, required)\n\
             - resource_type: Type of resource - text, binary, or json (required)\n\
             - description: What the resource contains (optional)\n\n\
             Example usage:\n\
             Generate a {resource_type} resource named '{resource_name}' for {description}."
        )
        .with_argument("resource_name", "Name of the resource in snake_case", true)
        .with_argument("resource_type", "Type: text, binary, or json", true)
        .with_argument("description", "Description of the resource", false),
    );

    // Generate README prompt
    prompts.insert(
        "generate-readme".to_string(),
        Prompt::new(
            "generate-readme",
            "Generate README.md with setup instructions",
            "Use the generate_readme tool to create a comprehensive README for an MCP server.\n\n\
             Parameters:\n\
             - project_name: Name of the MCP server project (required)\n\
             - description: Project description (optional)\n\
             - output_path: Where to save the README (optional, defaults to README.md)\n\n\
             Example usage:\n\
             Generate a README.md for the '{project_name}' project that {description}.",
        )
        .with_argument("project_name", "The name of the MCP server project", true)
        .with_argument("description", "Description of the project", false)
        .with_argument("output_path", "Path where to save the README", false),
    );

    // Validate Manifest prompt
    prompts.insert(
        "validate-manifest".to_string(),
        Prompt::new(
            "validate-manifest",
            "Validate an MCP server manifest file",
            "Use the validate_manifest tool to check if a manifest JSON is valid.\n\n\
             Required fields in manifest:\n\
             - name: Server name\n\
             - version: Version number\n\
             - description: Server description\n\n\
             Parameters:\n\
             - manifest_content: The JSON manifest content as a string (required)\n\n\
             Example usage:\n\
             Validate this manifest JSON: {manifest_content}",
        )
        .with_argument(
            "manifest_content",
            "The manifest JSON content to validate",
            true,
        ),
    );

    // Advanced Tool Implementation prompt
    prompts.insert(
        "advanced-tool-implementation".to_string(),
        Prompt::new(
            "advanced-tool-implementation",
            "Guide for implementing advanced MCP tools with error handling and async operations",
            "When implementing complex MCP tools, follow these best practices:\n\n\
             1. Use Result types for error handling\n\
             2. Implement async operations with tokio\n\
             3. Define structured input/output types with serde and schemars\n\
             4. Add comprehensive validation of input parameters\n\
             5. Include unit tests for success and error cases\n\
             6. Use JSON Schema attributes for client documentation\n\
             7. Return meaningful error messages\n\n\
             The advanced-tool-rs template demonstrates all these patterns.",
        )
        .with_argument("tool_purpose", "What the tool is designed to do", true),
    );

    // Prompts and Resources Integration prompt
    prompts.insert(
        "prompts-resources-guide".to_string(),
        Prompt::new(
            "prompts-resources-guide",
            "Guide for integrating Prompts and Resources in MCP servers",
            "Prompts and Resources work together to provide richer context to Claude:\n\n\
             PROMPTS:\n\
             - Use for multi-message conversations\n\
             - Support parameters for dynamic content\n\
             - Enable step-by-step problem solving\n\
             - Can reference server state\n\n\
             RESOURCES:\n\
             - Provide static or dynamic content access\n\
             - Support multiple MIME types (text, json, binary)\n\
             - Can use templates for parameterized URIs\n\
             - Useful for config, documentation, data\n\n\
             Best practice: Use Resources for data access, Prompts for guidance.",
        ),
    );

    // Error Handling Best Practices prompt
    prompts.insert(
        "error-handling-patterns".to_string(),
        Prompt::new(
            "error-handling-patterns",
            "Best practices for error handling in MCP servers",
            "Proper error handling improves MCP server reliability:\n\n\
             INPUT VALIDATION:\n\
             - Check for required fields\n\
             - Validate parameter ranges and formats\n\
             - Return early with descriptive errors\n\n\
             ERROR RESPONSES:\n\
             - Use McpError for MCP protocol errors\n\
             - Include error codes and context\n\
             - Provide actionable error messages\n\n\
             RECOVERY STRATEGIES:\n\
             - Distinguish between transient and permanent errors\n\
             - Suggest corrective actions\n\
             - Log errors appropriately\n\n\
             TESTING:\n\
             - Test both success and error paths\n\
             - Verify error messages are clear\n\
             - Test edge cases and boundaries",
        ),
    );

    // Async Patterns prompt
    prompts.insert(
        "async-patterns".to_string(),
        Prompt::new(
            "async-patterns",
            "Async/await patterns for MCP servers",
            "MCP Forge uses Tokio for async operations:\n\n\
             BASIC PATTERN:\n\
             async fn tool_name(...) -> Result<CallToolResult, McpError> { ... }\n\n\
             CONCURRENCY:\n\
             - Use Arc<Mutex<T>> for shared state\n\
             - Use Arc for cloning handles\n\
             - Lock only when necessary\n\n\
             ASYNC HELPERS:\n\
             - tokio::time::sleep for delays\n\
             - tokio::spawn for background tasks\n\
             - tokio::select! for multiple operations\n\n\
             BEST PRACTICES:\n\
             - Avoid blocking operations in async code\n\
             - Use .await at call sites\n\
             - Keep critical sections short",
        ),
    );

    // Testing Strategies prompt
    prompts.insert(
        "testing-strategies".to_string(),
        Prompt::new(
            "testing-strategies",
            "Testing strategies for MCP server implementations",
            "Comprehensive testing ensures reliable MCP servers:\n\n\
             UNIT TESTS:\n\
             - Test tool logic in isolation\n\
             - Use #[tokio::test] for async tests\n\
             - Cover success and error cases\n\n\
             PARAMETER VALIDATION:\n\
             - Test with valid inputs\n\
             - Test boundary conditions\n\
             - Test with invalid/missing parameters\n\n\
             MOCK TESTING:\n\
             - Mock external services\n\
             - Mock async operations for speed\n\
             - Test error conditions\n\n\
             INTEGRATION:\n\
             - Test tool chaining\n\
             - Test resource/prompt integration\n\
             - Test with MCP Inspector\n\n\
             COVERAGE:\n\
             - Aim for >80% code coverage\n\
             - Prioritize error paths\n\
             - Test public APIs thoroughly",
        ),
    );

    prompts
}

/// Get a specific prompt by name
pub fn get_prompt(name: &str) -> Option<Prompt> {
    get_available_prompts().get(name).cloned()
}

/// List all available prompt names
pub fn list_prompt_names() -> Vec<String> {
    get_available_prompts().keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_creation() {
        let prompt = Prompt::new("test", "Test prompt", "Test template");
        assert_eq!(prompt.name, "test");
        assert_eq!(prompt.description, "Test prompt");
    }

    #[test]
    fn test_prompt_with_arguments() {
        let prompt = Prompt::new("test", "Test", "Template")
            .with_argument("arg1", "First argument", true)
            .with_argument("arg2", "Second argument", false);

        assert_eq!(prompt.arguments.len(), 2);
        assert!(prompt.arguments[0].required);
        assert!(!prompt.arguments[1].required);
    }

    #[test]
    fn test_get_available_prompts() {
        let prompts = get_available_prompts();
        assert!(prompts.len() >= 10);
        assert!(prompts.contains_key("generate-project"));
        assert!(prompts.contains_key("generate-tool"));
        assert!(prompts.contains_key("generate-resource"));
        assert!(prompts.contains_key("generate-readme"));
        assert!(prompts.contains_key("validate-manifest"));
        assert!(prompts.contains_key("advanced-tool-implementation"));
        assert!(prompts.contains_key("error-handling-patterns"));
        assert!(prompts.contains_key("async-patterns"));
        assert!(prompts.contains_key("testing-strategies"));
    }

    #[test]
    fn test_get_prompt() {
        let prompt = get_prompt("generate-project");
        assert!(prompt.is_some());
        assert_eq!(prompt.unwrap().name, "generate-project");
    }

    #[test]
    fn test_list_prompt_names() {
        let names = list_prompt_names();
        assert!(names.len() >= 10);
        assert!(names.contains(&"generate-project".to_string()));
        assert!(names.contains(&"advanced-tool-implementation".to_string()));
    }
}
