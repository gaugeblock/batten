//! Tool dispatch implementation for `batten-mcp`.

use batten_core::{
    CompiledGrammarConstraint, GrammarConfig, GrammarFormat, LexicalScope,
    compile_grammar_with_config, filter_scope_for_hole, validate_candidate,
};
use dado_core::TypedHole;
use serde_json::Value;

/// Dispatches an incoming MCP tool invocation.
///
/// Returns the tool's text or a failure message. Building the MCP `content` envelope is not a tool's
/// job: the transport owns what a result looks like on the wire (`GB-234`).
///
/// # Errors
///
/// The message shown to the caller when the arguments are unusable or the tool fails.
pub fn dispatch_tool(name: &str, args: &Value) -> Result<String, String> {
    match name {
        "batten_compile" => handle_compile(args),
        "batten_validate" => handle_validate(args),
        unknown => Err(format!("Unknown tool: {unknown}")),
    }
}

fn handle_compile(args: &Value) -> Result<String, String> {
    let scope: LexicalScope =
        match serde_json::from_value(args.get("scope").cloned().unwrap_or(Value::Null)) {
            Ok(s) => s,
            Err(e) => return Err(format!("Invalid scope argument: {e}")),
        };

    let hole: TypedHole =
        match serde_json::from_value(args.get("hole").cloned().unwrap_or(Value::Null)) {
            Ok(h) => h,
            Err(e) => return Err(format!("Invalid hole argument: {e}")),
        };

    let format = match args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("gbnf")
    {
        "gbnf" => GrammarFormat::Gbnf,
        "lark" => GrammarFormat::Lark,
        "regex" => GrammarFormat::Regex,
        other => return Err(format!("Unsupported grammar format: {other}")),
    };

    let max_depth = args.get("max_depth").and_then(|v| v.as_u64()).unwrap_or(3) as usize;
    let config = GrammarConfig { max_depth };

    let filtered = filter_scope_for_hole(&scope, &hole);
    match compile_grammar_with_config(&filtered, &hole, format, &config) {
        Ok(constraint) => serde_json::to_string_pretty(&constraint)
            .map_err(|e| format!("Serialization error: {e}")),
        Err(e) => Err(format!("Compilation error: {e}")),
    }
}

fn handle_validate(args: &Value) -> Result<String, String> {
    let candidate = match args.get("candidate").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return Err("Missing required 'candidate' string".to_string()),
    };

    let constraint: CompiledGrammarConstraint =
        match serde_json::from_value(args.get("constraint").cloned().unwrap_or(Value::Null)) {
            Ok(c) => c,
            Err(e) => return Err(format!("Invalid constraint argument: {e}")),
        };

    let allow_literals = args
        .get("allow_literals")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let max_tokens = args
        .get("max_tokens")
        .and_then(|v| v.as_u64())
        .map(|v| v as u16);

    let report = validate_candidate(candidate, &constraint, allow_literals, max_tokens);

    serde_json::to_string_pretty(&report).map_err(|e| format!("Serialization error: {e}"))
}
