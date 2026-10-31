//! Model Context Protocol (MCP) stdio server for `batten`.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod dispatch;
pub mod schema;

use gaugeblock_mcp::{McpHandler, RpcResponse, ToolOutcome};
use serde_json::Value;
use std::io::{self};

/// MCP server version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// `batten`'s tool surface.
#[derive(Debug, Default)]
pub struct BattenHandler;

impl McpHandler for BattenHandler {
    fn server_name(&self) -> &str {
        "batten-mcp"
    }

    fn list_tools(&self) -> Value {
        Value::Array(schema::list_tools())
    }

    fn call_tool(&self, name: &str, arguments: &Value) -> ToolOutcome {
        match dispatch::dispatch_tool(name, arguments) {
            // The dispatcher formats its reports; the transport wraps them.
            Ok(text) => ToolOutcome::Text(text),
            Err(message) => ToolOutcome::Failed(message),
        }
    }
}

/// Runs the stdio JSON-RPC 2.0 MCP server loop over stdin and stdout.
///
/// # Errors
///
/// Returns the I/O failure when stdin cannot be read or stdout cannot be written.
pub fn run_stdio_server() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting batten-mcp stdio server loop");
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    gaugeblock_mcp::serve(stdin.lock(), &mut stdout, &BattenHandler)?;
    Ok(())
}

/// Handles a single JSON-RPC 2.0 message and generates a response.
///
/// Kept because the existing tests drive the server this way; the line-shaped entry point lives in
/// `gaugeblock-mcp`, so this is a thin adapter rather than a second implementation.
#[must_use]
pub fn handle_message(msg: &Value) -> Option<Value> {
    gaugeblock_mcp::handle_line(&msg.to_string(), &BattenHandler).map(RpcResponse::into_value)
}
