//! MCP Forge: A development framework for building MCP servers with Rust SDK
//!
//! This library provides tools and resources to help developers create MCP servers
//! using the Rust SDK, including code generation, templates, and documentation.
//!
//! # Modules
//!
//! - [`prompts`] - Reusable prompt templates for Claude integration
//! - [`resources`] - Documentation and code templates as resources
//! - [`server`] - Main MCP server implementation
//! - [`tool_executor`] - Tool execution logic and handlers
//! - [`tools`] - Tool definitions and metadata

pub mod prompts;
pub mod resources;
pub mod server;
pub mod tool_executor;
pub mod tools;

pub use server::MCPForgeServer;

/// MCP Forge version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// MCP Forge description
pub const DESCRIPTION: &str = "A development framework for building MCP servers with Rust SDK";
