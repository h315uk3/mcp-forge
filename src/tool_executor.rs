//! Tool Executor
//!
//! Executes MCP Forge tools with argument validation and error handling.
//!
//! This module handles the execution of all available MCP Forge tools:
//! - `generate_project`: Generates a new MCP server project structure
//! - `generate_tool`: Generates code for a new tool
//! - `generate_resource`: Generates code for a new resource
//! - `generate_readme`: Generates README.md with setup instructions
//! - `validate_manifest`: Validates an MCP server manifest file
//!
//! All functions accept JSON arguments and return results as strings.

use crate::resources;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tracing::debug;
use tracing::info;

/// Execute a tool by name with the given arguments.
///
/// Routes the tool name to the appropriate executor function and handles
/// argument validation and error reporting.
///
/// # Arguments
///
/// * `tool_name` - The name of the tool to execute
/// * `arguments` - JSON object containing tool arguments
///
/// # Returns
///
/// Returns a `Result` with the tool execution output as a string, or an
/// error message describing what went wrong.
///
/// # Supported Tools
///
/// - `generate_project` - Generate new MCP server project
/// - `generate_tool` - Generate tool code template
/// - `generate_resource` - Generate resource code template
/// - `generate_readme` - Generate README.md with setup instructions
/// - `validate_manifest` - Validate MCP manifest JSON
pub async fn execute_tool(tool_name: &str, arguments: &Value) -> Result<String, String> {
    debug!("Executing tool: {}", tool_name);

    match tool_name {
        "generate_project" => execute_generate_project(arguments).await,
        "generate_tool" => execute_generate_tool(arguments).await,
        "generate_resource" => execute_generate_resource(arguments).await,
        "generate_readme" => execute_generate_readme(arguments).await,
        "validate_manifest" => execute_validate_manifest(arguments).await,
        _ => Err(format!("Unknown tool: {}", tool_name)),
    }
}

/// Generate a new MCP server project structure.
///
/// Creates a complete project directory with:
/// - Standard Rust project structure
/// - Cargo.toml with MCP dependencies
/// - main.rs and lib.rs templates
/// - .gitignore file
///
/// # Arguments
///
/// * `project_name` - (required) Name of the new project
/// * `description` - (optional) Project description
///
/// # Returns
///
/// Returns a success message or an error if directory creation fails.
///
/// # Errors
///
/// Returns an error if:
/// - `project_name` argument is missing
/// - Project directory cannot be created
/// - Template files cannot be written
async fn execute_generate_project(arguments: &Value) -> Result<String, String> {
    info!("Generating new MCP project");

    let project_name = arguments
        .get("project_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing required argument: project_name".to_string())?;

    let description = arguments
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("A new MCP server project");

    debug!(
        "Project name: {}, Description: {}",
        project_name, description
    );

    // Create project directory structure
    create_project_structure(project_name, description).await?;

    Ok(format!(
        "Project '{}' generated successfully in directory '{}'",
        project_name, project_name
    ))
}

/// Generate code template for a new MCP tool.
///
/// Produces a Rust code template with:
/// - Async function skeleton
/// - Input parameter parsing pattern
/// - Unit test template
/// - Proper error handling structure
///
/// # Arguments
///
/// * `tool_name` - (required) Name of the tool
/// * `description` - (required) Tool description
///
/// # Returns
///
/// Returns the generated Rust code as a string.
///
/// # Errors
///
/// Returns an error if required arguments are missing.
///
/// # Example
///
/// ```ignore
/// let args = json!({
///     "tool_name": "my_processor",
///     "description": "Process data"
/// });
/// let code = execute_generate_tool(&args).await?;
/// ```
async fn execute_generate_tool(arguments: &Value) -> Result<String, String> {
    info!("Generating tool code");

    let tool_name = arguments
        .get("tool_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing required argument: tool_name".to_string())?;

    let description = arguments
        .get("description")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing required argument: description".to_string())?;

    debug!("Tool name: {}", tool_name);

    // Generate tool code
    let tool_code = generate_tool_code(tool_name, description);

    Ok(tool_code)
}

/// Generate code template for a new MCP resource.
///
/// Produces a Rust code template for resource management including:
/// - Struct definition for the resource
/// - Methods for accessing resource content
/// - Support for different resource types (text, binary, json)
///
/// # Arguments
///
/// * `resource_name` - (required) Name of the resource (snake_case)
/// * `resource_type` - (required) Type of resource (text, binary, or json)
/// * `description` - (optional) Resource description
///
/// # Returns
///
/// Returns the generated Rust code as a string.
///
/// # Errors
///
/// Returns an error if required arguments are missing.
///
/// # Example
///
/// ```ignore
/// let args = json!({
///     "resource_name": "user_data",
///     "resource_type": "json",
///     "description": "User database resource"
/// });
/// let code = execute_generate_resource(&args).await?;
/// ```
async fn execute_generate_resource(arguments: &Value) -> Result<String, String> {
    info!("Generating resource code");

    let resource_name = arguments
        .get("resource_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing required argument: resource_name".to_string())?;

    let resource_type = arguments
        .get("resource_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing required argument: resource_type".to_string())?;

    let description = arguments
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    debug!("Resource: {} (type: {})", resource_name, resource_type);

    // Generate resource code
    let resource_code = generate_resource_code(resource_name, resource_type, description);

    Ok(resource_code)
}

/// Generate README.md file with MCP server setup instructions.
///
/// Creates a comprehensive README including:
/// - Project overview and description
/// - Prerequisites and installation
/// - Quick start guide
/// - Development instructions
/// - Testing procedures
/// - Deployment guide
/// - Troubleshooting section
///
/// # Arguments
///
/// * `project_name` - (required) Name of the MCP server project
/// * `description` - (optional) Project description
/// * `output_path` - (optional) Path to write README.md (defaults to "README.md")
///
/// # Returns
///
/// Returns a success message with the output file path.
///
/// # Errors
///
/// Returns an error if:
/// - `project_name` argument is missing
/// - README.md cannot be written to the specified path
///
/// # Example
///
/// ```ignore
/// let args = json!({
///     "project_name": "my_server",
///     "description": "My MCP server",
///     "output_path": "./README.md"
/// });
/// let result = execute_generate_readme(&args).await?;
/// ```
async fn execute_generate_readme(arguments: &Value) -> Result<String, String> {
    info!("Generating README.md");

    let project_name = arguments
        .get("project_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing required argument: project_name".to_string())?;

    let description = arguments
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("A new MCP server project");

    let output_path = arguments
        .get("output_path")
        .and_then(|v| v.as_str())
        .unwrap_or("README.md");

    debug!("Generating README for: {}", project_name);

    // Generate README content
    let readme_content = generate_readme_content(project_name, description);

    // Write to file
    fs::write(output_path, readme_content)
        .map_err(|e| format!("Failed to write README.md: {}", e))?;

    Ok(format!(
        "README.md generated successfully at '{}'",
        output_path
    ))
}

/// Validate an MCP server manifest file.
///
/// Performs validation checks on manifest JSON:
/// - Validates JSON syntax
/// - Checks for required fields (name, version, description)
/// - Ensures manifest structure is correct
///
/// # Arguments
///
/// * `manifest_content` - (required) JSON string containing the manifest
///
/// # Returns
///
/// Returns a validation result message or a detailed error description.
///
/// # Errors
///
/// Returns an error if:
/// - `manifest_content` argument is missing
/// - JSON is invalid or malformed
/// - Required fields are missing (name, version, description)
///
/// # Example
///
/// ```ignore
/// let args = json!({
///     "manifest_content": r#"{"name": "my_server", "version": "0.1.0", "description": "test"}"#
/// });
/// let result = execute_validate_manifest(&args).await?;
/// ```
async fn execute_validate_manifest(arguments: &Value) -> Result<String, String> {
    info!("Validating manifest");

    let manifest_content = arguments
        .get("manifest_content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing required argument: manifest_content".to_string())?;

    // Parse and validate manifest JSON
    match serde_json::from_str::<Value>(manifest_content) {
        Ok(manifest) => {
            debug!("Manifest parsed successfully");

            // Validate required fields
            let required_fields = ["name", "version", "description"];
            let missing_fields: Vec<&str> = required_fields
                .iter()
                .filter(|field| manifest.get(*field).is_none())
                .copied()
                .collect();

            if missing_fields.is_empty() {
                Ok("Manifest is valid.".to_string())
            } else {
                Err(format!(
                    "Manifest is invalid. Missing fields: {}",
                    missing_fields.join(", ")
                ))
            }
        }
        Err(e) => Err(format!("Invalid JSON in manifest: {}", e)),
    }
}

/// Validate project name to prevent path traversal attacks
///
/// Checks that the project name:
/// - Does not contain path traversal sequences (../, .., ./, etc.)
/// - Does not start with / (absolute paths)
/// - Does not contain null bytes
/// - Is a valid UTF-8 string
///
/// # Arguments
///
/// * `project_name` - The project name to validate
///
/// # Returns
///
/// Returns Ok(()) if the name is safe, or an error message if validation fails
fn validate_project_name(project_name: &str) -> Result<(), String> {
    // Check for empty name
    if project_name.is_empty() {
        return Err("Project name cannot be empty".to_string());
    }

    // Check for path traversal patterns
    if project_name.contains("..") {
        return Err("Project name cannot contain '..' (path traversal)".to_string());
    }

    // Check for absolute paths
    if project_name.starts_with('/') {
        return Err("Project name cannot be an absolute path".to_string());
    }

    // Check for null bytes
    if project_name.contains('\0') {
        return Err("Project name cannot contain null bytes".to_string());
    }

    // Check for suspicious patterns
    if project_name.contains("./") || project_name.contains("/./") || project_name.ends_with("/.") {
        return Err("Project name cannot contain path traversal patterns".to_string());
    }

    // Check for Windows drive letters (e.g., "C:", "D:")
    if project_name.len() > 1 && project_name.chars().nth(1) == Some(':') {
        return Err("Project name cannot contain Windows drive letter".to_string());
    }

    debug!("Project name '{}' passed validation", project_name);
    Ok(())
}

/// Create project directory structure
///
/// Uses the 2-stage calling pattern: retrieves templates from resources
/// instead of using include_str directly, enabling better separation of
/// concerns and error visibility when templates are missing.
async fn create_project_structure(project_name: &str, description: &str) -> Result<(), String> {
    // Validate project name for security
    validate_project_name(project_name)?;

    let base_dir = Path::new(project_name);

    // Create base directory
    fs::create_dir_all(base_dir)
        .map_err(|e| format!("Failed to create project directory: {}", e))?;

    // Create src directory
    fs::create_dir_all(base_dir.join("src"))
        .map_err(|e| format!("Failed to create src directory: {}", e))?;

    // Create Cargo.toml
    let cargo_toml = generate_cargo_toml(project_name, description);
    fs::write(base_dir.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;

    // Get main.rs from resources
    let main_rs = resources::get_resource("template/main-rs")
        .ok_or_else(|| "Template 'main.rs' not found in resources".to_string())?
        .content;
    fs::write(base_dir.join("src/main.rs"), main_rs)
        .map_err(|e| format!("Failed to write main.rs: {}", e))?;

    // Get lib.rs from resources
    let lib_rs = resources::get_resource("template/lib-rs")
        .ok_or_else(|| "Template 'lib.rs' not found in resources".to_string())?
        .content;
    fs::write(base_dir.join("src/lib.rs"), lib_rs)
        .map_err(|e| format!("Failed to write lib.rs: {}", e))?;

    // Get error.rs from resources
    let error_rs = resources::get_resource("template/error-rs")
        .ok_or_else(|| "Template 'error.rs' not found in resources".to_string())?
        .content;
    fs::write(base_dir.join("src/error.rs"), error_rs)
        .map_err(|e| format!("Failed to write error.rs: {}", e))?;

    // Get server.rs from resources
    let server_rs = resources::get_resource("template/server-rs")
        .ok_or_else(|| "Template 'server.rs' not found in resources".to_string())?
        .content;
    fs::write(base_dir.join("src/server.rs"), server_rs)
        .map_err(|e| format!("Failed to write server.rs: {}", e))?;

    // Get tools.rs from resources
    let tools_rs = resources::get_resource("template/tools-rs")
        .ok_or_else(|| "Template 'tools.rs' not found in resources".to_string())?
        .content;
    fs::write(base_dir.join("src/tools.rs"), tools_rs)
        .map_err(|e| format!("Failed to write tools.rs: {}", e))?;

    // Get resources.rs from resources
    let resources_rs = resources::get_resource("template/resources-rs")
        .ok_or_else(|| "Template 'resources.rs' not found in resources".to_string())?
        .content;
    fs::write(base_dir.join("src/resources.rs"), resources_rs)
        .map_err(|e| format!("Failed to write resources.rs: {}", e))?;

    // Create .gitignore
    let gitignore = "/target\n/Cargo.lock\n.env\n*.swp\n*.swo\n";
    fs::write(base_dir.join(".gitignore"), gitignore)
        .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

    debug!("Project structure created successfully");
    Ok(())
}

/// Generate Cargo.toml content
fn generate_cargo_toml(project_name: &str, description: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
description = "{}"
license = "MIT"

[dependencies]
rmcp = {{ version = "0.8", features = ["server"] }}
tokio = {{ version = "1.40", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = {{ version = "0.3", features = ["env-filter"] }}

[dev-dependencies]
tokio-test = "0.4"

[[bin]]
name = "{}"
path = "src/main.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
"#,
        project_name, description, project_name
    )
}

/// Generate tool Rust code template
fn generate_tool_code(tool_name: &str, description: &str) -> String {
    let tool_name_snake = tool_name.to_lowercase();
    format!(
        r#"/// {tool_name} Tool
///
/// {description}
pub async fn execute_{tool_name_snake}() -> Result<String, String> {{
    // Implementation goes here
    Ok("Tool executed successfully".to_string())
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[tokio::test]
    async fn test_{tool_name_snake}() {{
        let result = execute_{tool_name_snake}().await;
        assert!(result.is_ok());
    }}
}}
"#,
        tool_name = tool_name,
        tool_name_snake = tool_name_snake,
        description = description
    )
}

/// Generate resource Rust code template
fn generate_resource_code(resource_name: &str, resource_type: &str, description: &str) -> String {
    format!(
        r#"/// {resource_name} Resource
///
/// Type: {resource_type}
/// {description}
pub struct {resource_name_pascal} {{
    // Resource fields
}}

impl {resource_name_pascal} {{
    /// Get resource content
    pub fn get_content(&self) -> String {{
        // Return resource content
        String::new()
    }}
}}
"#,
        resource_name = resource_name,
        resource_type = resource_type,
        description = description,
        resource_name_pascal = to_pascal_case(resource_name)
    )
}

/// Convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Generate README.md content with setup instructions
fn generate_readme_content(project_name: &str, description: &str) -> String {
    format!(
        r#"# {project_name}

{description}

## Overview

This is a Model Context Protocol (MCP) server project built with the Rust SDK.

## Prerequisites

- Rust 1.75+ ([Install Rust](https://www.rust-lang.org/tools/install))
- Cargo (comes with Rust)

## Quick Start

### 1. Clone or Create the Project

```bash
git clone <repository-url>
cd {project_name_snake}
```

### 2. Build the Project

```bash
cargo build
```

### 3. Run the Server

```bash
cargo run
```

This starts the MCP server, which communicates via stdio with MCP clients.

## Development

### Running Tests

```bash
cargo test
```

### Running Tests with Output

```bash
RUST_LOG=debug cargo test -- --nocapture
```

### Development Mode with Auto-Reload

Install `cargo-watch` if not already installed:

```bash
cargo install cargo-watch
```

Then run:

```bash
cargo watch -x run
```

## Project Structure

```
{project_name_snake}/
├── src/
│   ├── main.rs           # Entry point
│   └── lib.rs            # Library root
├── Cargo.toml           # Dependencies and metadata
├── Cargo.lock           # Dependency lock file
└── README.md            # This file
```

## Tools

The following tools are available in this MCP server:

- `generate_project` - Generate a new MCP server project
- `generate_tool` - Generate code for a new tool
- `generate_resource` - Generate code for a new resource
- `generate_readme` - Generate README.md with setup instructions
- `validate_manifest` - Validate an MCP manifest file

## Configuration

Edit `src/main.rs` to configure:

- Server name and version
- Available tools
- Available resources
- Custom handlers

## Deployment

### Building for Release

```bash
cargo build --release
```

The optimized binary will be at `target/release/{project_name_snake}`.

### Environment Variables

To control logging during runtime:

```bash
RUST_LOG=info {project_name_snake}
RUST_LOG=debug {project_name_snake}
```

## Resources

- [MCP Specification](https://modelcontextprotocol.io)
- [Rust SDK Documentation](https://github.com/modelcontextprotocol/rust-sdk)
- [Tokio Documentation](https://tokio.rs)

## Contributing

1. Create a feature branch
2. Make your changes
3. Run tests: `cargo test`
4. Format code: `cargo fmt`
5. Lint: `cargo clippy`
6. Commit and push

## License

MIT

## Support

For issues or questions:

1. Check the [MCP Documentation](https://modelcontextprotocol.io)
2. Review [Rust SDK Examples](https://github.com/modelcontextprotocol/rust-sdk)
3. Open an issue on this repository
"#,
        project_name = project_name,
        project_name_snake = project_name.to_lowercase().replace("-", "_"),
        description = description
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_generate_project_args() {
        let args = json!({
            "project_name": "test_project_example",
            "description": "A test project"
        });
        let result = execute_generate_project(&args).await;
        assert!(result.is_ok());
        // Clean up
        let _ = std::fs::remove_dir_all("test_project_example");
    }

    #[tokio::test]
    async fn test_generate_project_missing_args() {
        let args = json!({});
        let result = execute_generate_project(&args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_tool_args() {
        let args = json!({
            "tool_name": "my_tool",
            "description": "A test tool"
        });
        let result = execute_generate_tool(&args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_manifest_valid() {
        let args = json!({
            "manifest_content": r#"{"name": "test", "version": "0.1.0", "description": "test"}"#
        });
        let result = execute_validate_manifest(&args).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("valid"));
    }

    #[tokio::test]
    async fn test_validate_manifest_invalid_json() {
        let args = json!({
            "manifest_content": "not valid json"
        });
        let result = execute_validate_manifest(&args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_manifest_missing_fields() {
        let args = json!({
            "manifest_content": r#"{"name": "test"}"#
        });
        let result = execute_validate_manifest(&args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_readme() {
        let args = json!({
            "project_name": "test_project_example",
            "description": "A test project",
            "output_path": "/tmp/test_readme.md"
        });
        let result = execute_generate_readme(&args).await;
        assert!(result.is_ok());
        // Clean up
        let _ = std::fs::remove_file("/tmp/test_readme.md");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("my_tool"), "MyTool");
        assert_eq!(to_pascal_case("a"), "A");
    }

    #[test]
    fn test_validate_project_name_valid() {
        assert!(validate_project_name("my_project").is_ok());
        assert!(validate_project_name("my-project").is_ok());
        assert!(validate_project_name("project123").is_ok());
        assert!(validate_project_name("a").is_ok());
    }

    #[test]
    fn test_validate_project_name_path_traversal() {
        assert!(validate_project_name("../evil").is_err());
        assert!(validate_project_name("my/../project").is_err());
        assert!(validate_project_name("./project").is_err());
        assert!(validate_project_name("project/.").is_err());
    }

    #[test]
    fn test_validate_project_name_absolute_path() {
        assert!(validate_project_name("/etc/passwd").is_err());
    }

    #[test]
    fn test_validate_project_name_empty() {
        assert!(validate_project_name("").is_err());
    }

    #[test]
    fn test_validate_project_name_null_byte() {
        assert!(validate_project_name("project\0name").is_err());
    }

    #[test]
    fn test_validate_project_name_drive_letter() {
        assert!(validate_project_name("C:/project").is_err());
        assert!(validate_project_name("D:").is_err());
    }
}
