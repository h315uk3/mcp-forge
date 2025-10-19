# MCP Forge

A development framework for building MCP (Model Context Protocol) servers using the official Rust SDK.

## Features

- **Project Generation**: Scaffold new MCP server projects
- **Tool & Resource Templates**: Generate implementation code
- **10 Reusable Prompts**: For Claude integration
- **README Generation**: Automatic documentation
- **Manifest Validation**: Schema validation
- **Stdio Communication**: Full MCP protocol compliance

## Quick Start

```bash
git clone https://github.com/h315uk3/mcp-forge.git && cd mcp-forge
cargo build --release
cargo run
```

## Usage with Claude CLI

```bash
# Create a new project
/mcp-forge:generate-project my_server "A data processing MCP server"

# Get implementation guide
/mcp-forge:advanced-tool-implementation "file search with parallel processing"

# Learn error handling
/mcp-forge:error-handling-patterns

# Learn async patterns
/mcp-forge:async-patterns

# Learn testing strategies
/mcp-forge:testing-strategies
```

## Available Tools

| Tool | Purpose | Parameters |
|------|---------|-----------|
| `generate-project` | Scaffold new MCP server | `project_name`, `description` (optional) |
| `generate-tool` | Create tool template | `tool_name`, `description` |
| `generate-resource` | Create resource template | `resource_name`, `resource_type` (text/binary/json), `description` (optional) |
| `generate-readme` | Auto-generate README | `project_name`, `description` (optional), `output_path` (optional) |
| `validate-manifest` | Validate manifest JSON | `manifest_content` |

## Available Prompts (10 Total)

**Core Tools:**
- `generate-project`, `generate-tool`, `generate-resource`, `generate-readme`, `validate-manifest`

**Best Practices:**
- `advanced-tool-implementation` - Complex tools with error handling & async
- `prompts-resources-guide` - Prompt/Resource integration patterns
- `error-handling-patterns` - Input validation & recovery strategies
- `async-patterns` - Tokio concurrency patterns
- `testing-strategies` - Unit/integration test approaches

## Project Structure

```
src/
├── main.rs           # Entry point
├── server.rs         # MCP ServerHandler implementation
├── tools.rs          # Tool definitions
├── tool_executor.rs  # Tool execution logic
├── prompts.rs        # 10 prompt templates
├── resources.rs      # 10 resource templates
└── lib.rs            # Library root

templates/            # 10 Rust code templates
Cargo.toml            # Dependencies (Rust 1.89.0+, Tokio 1.48, rmcp 0.8.1)
```

## Development

```bash
cargo build              # Debug build
cargo build --release   # Optimized build
cargo test              # Run tests (25 test cases)
cargo fmt               # Format code
cargo clippy            # Lint check
RUST_LOG=debug cargo run # Debug logging
```

## Architecture

MCP Forge implements the `ServerHandler` trait from the official Rust SDK:

- **Server**: Handles MCP protocol (tools, resources, prompts)
- **Tools**: Code generation for MCP development
- **Resources**: Static/dynamic templates
- **Prompts**: Multi-message guidance for Claude

**Protocol Compliance:**
- ✅ Stdio-based communication
- ✅ JSON-RPC 2.0
- ✅ Official `rmcp` 0.8.1 SDK
- ✅ No custom protocol implementation

## Security

- Path validation (traversal attack prevention)
- Input sanitization
- JSON schema validation
- Safe error messages

## Testing with MCP Inspector

```bash
# Terminal 1: Start server
cargo run

# Terminal 2: Run inspector
npx @modelcontextprotocol/inspector cargo run
```

## Requirements

- Rust 1.89.0+
- Tokio 1.48+
- rmcp 0.8.1 (official SDK)

## Quality Metrics

- ✅ 25/25 tests passing
- ✅ Zero lint warnings
- ✅ 100% SDK compliance
- ✅ Comprehensive documentation

## License

MIT

## References

- [MCP Specification](https://modelcontextprotocol.io/)
- [Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [MCP Inspector](https://github.com/modelcontextprotocol/inspector)
