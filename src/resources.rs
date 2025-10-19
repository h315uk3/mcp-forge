//! Resource templates and documentation for MCP Forge
//!
//! Provides templates and documentation resources to help developers
//! create MCP servers with the Rust SDK.
//!
//! This module implements the resource layer for the 2-stage calling pattern:
//! tool_executor calls these resources to obtain templates, enabling better
//! separation of concerns and error visibility when templates are missing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a resource available through MCP
///
/// A resource provides content (templates, documentation, etc.) that can be
/// accessed via URIs like `resource://mcp-forge/template/tool.rs`.
/// Resources support different MIME types (text, JSON, binary).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Resource URI
    pub uri: String,
    /// Resource name
    pub name: String,
    /// Resource MIME type
    pub mime_type: String,
    /// Resource content
    pub content: String,
}

impl Resource {
    /// Create a new resource
    pub fn new(
        uri: impl Into<String>,
        name: impl Into<String>,
        mime_type: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            uri: uri.into(),
            name: name.into(),
            mime_type: mime_type.into(),
            content: content.into(),
        }
    }
}

/// Get all available resources
pub fn get_available_resources() -> HashMap<String, Resource> {
    let mut resources = HashMap::new();

    // Cargo.toml template
    resources.insert(
        "template/cargo-toml".to_string(),
        Resource::new(
            "forge://templates/Cargo.toml",
            "Cargo.toml Template",
            "text/plain",
            include_str!("../templates/Cargo.toml.template"),
        ),
    );

    // main.rs template
    resources.insert(
        "template/main-rs".to_string(),
        Resource::new(
            "forge://templates/main.rs",
            "main.rs Template",
            "text/plain",
            include_str!("../templates/main.rs.template"),
        ),
    );

    // lib.rs template
    resources.insert(
        "template/lib-rs".to_string(),
        Resource::new(
            "forge://templates/lib.rs",
            "lib.rs Template",
            "text/plain",
            include_str!("../templates/lib.rs.template"),
        ),
    );

    // error.rs template
    resources.insert(
        "template/error-rs".to_string(),
        Resource::new(
            "forge://templates/error.rs",
            "error.rs Template",
            "text/plain",
            include_str!("../templates/error.rs.template"),
        ),
    );

    // server.rs template
    resources.insert(
        "template/server-rs".to_string(),
        Resource::new(
            "forge://templates/server.rs",
            "server.rs Template",
            "text/plain",
            include_str!("../templates/server.rs.template"),
        ),
    );

    // tools.rs template
    resources.insert(
        "template/tools-rs".to_string(),
        Resource::new(
            "forge://templates/tools.rs",
            "tools.rs Template",
            "text/plain",
            include_str!("../templates/tools.rs.template"),
        ),
    );

    // resources.rs template
    resources.insert(
        "template/resources-rs".to_string(),
        Resource::new(
            "forge://templates/resources.rs",
            "resources.rs Template",
            "text/plain",
            include_str!("../templates/resources.rs.template"),
        ),
    );

    // advanced-tool.rs template
    resources.insert(
        "template/advanced-tool-rs".to_string(),
        Resource::new(
            "forge://templates/advanced-tool.rs",
            "Advanced Tool Template",
            "text/plain",
            include_str!("../templates/advanced-tool.rs.template"),
        ),
    );

    // prompts-advanced.rs template
    resources.insert(
        "template/prompts-advanced-rs".to_string(),
        Resource::new(
            "forge://templates/prompts-advanced.rs",
            "Advanced Prompts Template",
            "text/plain",
            include_str!("../templates/prompts-advanced.rs.template"),
        ),
    );

    // resources-advanced.rs template
    resources.insert(
        "template/resources-advanced-rs".to_string(),
        Resource::new(
            "forge://templates/resources-advanced.rs",
            "Advanced Resources Template",
            "text/plain",
            include_str!("../templates/resources-advanced.rs.template"),
        ),
    );

    resources
}

/// Get a specific resource by key
pub fn get_resource(key: &str) -> Option<Resource> {
    get_available_resources().get(key).cloned()
}

/// List all available resource keys
pub fn list_resource_keys() -> Vec<String> {
    get_available_resources().keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_creation() {
        let resource = Resource::new("test://uri", "Test Resource", "text/plain", "test content");
        assert_eq!(resource.uri, "test://uri");
        assert_eq!(resource.name, "Test Resource");
    }

    #[test]
    fn test_get_available_resources() {
        let resources = get_available_resources();
        assert!(!resources.is_empty());
        assert!(resources.contains_key("template/cargo-toml"));
        assert!(resources.contains_key("template/main-rs"));
        assert!(resources.contains_key("template/lib-rs"));
    }

    #[test]
    fn test_list_resource_keys() {
        let keys = list_resource_keys();
        assert!(!keys.is_empty());
        assert!(keys.iter().any(|k| k.contains("template")));
    }
}
