use anyhow::Result;
use mcp_forge::MCPForgeServer;
use rmcp::ServiceExt;
use std::env;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging (write to stderr so stdout is clean for MCP messages)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    // Check for debug mode via environment variable
    let debug_mode = env::var("MCP_FORGE_DEBUG").is_ok();

    if debug_mode {
        // Debug mode: display server information
        print_server_info();
        Ok(())
    } else {
        // Server mode: run MCP server with proper MCP SDK
        run_mcp_server().await
    }
}

/// Print server information (debug mode)
fn print_server_info() {
    let _server = MCPForgeServer::new();

    println!("=== MCP Forge Server ===");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!();

    println!("This server provides tools and resources for MCP development:");
    println!("- Project generation tools");
    println!("- Code generation utilities");
    println!("- Documentation and examples");
    println!();

    println!("To test with MCP Inspector:");
    println!("  npx @modelcontextprotocol/inspector cargo run");
    println!();

    tracing::info!("MCP Forge server info displayed");
}

/// Run MCP server using official MCP SDK
async fn run_mcp_server() -> Result<()> {
    tracing::info!("Starting MCP Forge server with official MCP SDK");

    // Create MCP Forge server instance
    let server = MCPForgeServer::new();

    // Start server with stdio transport
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = server.serve(transport).await.inspect_err(|e| {
        tracing::error!("Error starting MCP server: {:?}", e);
    })?;

    tracing::info!("MCP Forge server started successfully");

    // Wait for the server to finish
    service.waiting().await?;

    tracing::info!("MCP Forge server stopped");
    Ok(())
}
