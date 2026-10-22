//! Tool dispatch implementation for `batten-mcp`.

use batten_core::{
    CompiledGrammarConstraint, GrammarConfig, GrammarFormat, LexicalScope,
    compile_grammar_with_config, filter_scope_for_hole, validate_candidate,
};
use dado_core::TypedHole;
use serde_json::{Value, json};

/// Dispatches an incoming MCP tool invocation.
pub fn dispatch_tool(name: &str, args: &Value) -> Value {
    match name {
        "batten_compile" => handle_compile(args),
        "batten_validate" => handle_validate(args),
        unknown => json!({
            "isError": true,
            "content": [{
                "type": "text",
                "text": format!("Unknown tool: {unknown}")
            }]
        }),
    }
}

fn handle_compile(args: &Value) -> Value {
    let scope: LexicalScope = match serde_json::from_value(args.get("scope").cloned().unwrap_or(Value::Null)) {
        Ok(s) => s,
        Err(e) => return error_response(&format!("Invalid scope argument: {e}")),
    };

    let hole: TypedHole = match serde_json::from_value(args.get("hole").cloned().unwrap_or(Value::Null)) {
        Ok(h) => h,
        Err(e) => return error_response(&format!("Invalid hole argument: {e}")),
    };

    let format = match args.get("format").and_then(|v| v.as_str()).unwrap_or("gbnf") {
        "gbnf" => GrammarFormat::Gbnf,
        "lark" => GrammarFormat::Lark,
        "regex" => GrammarFormat::Regex,
        other => return error_response(&format!("Unsupported grammar format: {other}")),
    };

    let max_depth = args.get("max_depth").and_then(|v| v.as_u64()).unwrap_or(3) as usize;
    let config = GrammarConfig { max_depth };

    let filtered = filter_scope_for_hole(&scope, &hole);
    match compile_grammar_with_config(&filtered, &hole, format, &config) {
        Ok(constraint) => match serde_json::to_string_pretty(&constraint) {
            Ok(json_str) => json!({
                "content": [{
                    "type": "text",
                    "text": json_str
                }]
            }),
            Err(e) => error_response(&format!("Serialization error: {e}")),
        },
        Err(e) => error_response(&format!("Compilation error: {e}")),
    }
}

fn handle_validate(args: &Value) -> Value {
    let candidate = match args.get("candidate").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return error_response("Missing required 'candidate' string"),
    };

    let constraint: CompiledGrammarConstraint = match serde_json::from_value(
        args.get("constraint").cloned().unwrap_or(Value::Null),
    ) {
        Ok(c) => c,
        Err(e) => return error_response(&format!("Invalid constraint argument: {e}")),
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

    match serde_json::to_string_pretty(&report) {
        Ok(json_str) => json!({
            "content": [{
                "type": "text",
                "text": json_str
            }]
        }),
        Err(e) => error_response(&format!("Serialization error: {e}")),
    }
}

fn error_response(msg: &str) -> Value {
    json!({
        "isError": true,
        "content": [{
            "type": "text",
            "text": msg
        }]
    })
}

