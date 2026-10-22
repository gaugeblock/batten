//! Tool definitions and JSON schemas for `batten-mcp`.

use serde_json::{Value, json};

/// Returns the list of MCP tool definitions provided by `batten-mcp`.
#[must_use]
pub fn list_tools() -> Vec<Value> {
    vec![
        json!({
            "name": "batten_compile",
            "description": "Compiles in-scope symbols and target TypedHole into a bounded Context-Free Grammar constraint (GBNF, Lark, or Regex).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "scope": {
                        "type": "object",
                        "description": "LexicalScope object containing bindings and target language"
                    },
                    "hole": {
                        "type": "object",
                        "description": "TypedHole definition specifying hole_id, expected_type, ast_node_kind, and allow_literals"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["gbnf", "lark", "regex"],
                        "default": "gbnf",
                        "description": "Target grammar constraint format"
                    },
                    "max_depth": {
                        "type": "integer",
                        "default": 3,
                        "description": "Maximum recursive expression nesting depth"
                    }
                },
                "required": ["scope", "hole"]
            }
        }),
        json!({
            "name": "batten_validate",
            "description": "Microsecond pre-flight validation verifying a candidate fill string against a CompiledGrammarConstraint.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "candidate": {
                        "type": "string",
                        "description": "Candidate code snippet to validate"
                    },
                    "constraint": {
                        "type": "object",
                        "description": "CompiledGrammarConstraint with allowed_identifiers"
                    },
                    "allow_literals": {
                        "type": "boolean",
                        "default": true,
                        "description": "Whether literals are permitted in candidate"
                    },
                    "max_tokens": {
                        "type": "integer",
                        "description": "Optional maximum permitted token count"
                    }
                },
                "required": ["candidate", "constraint"]
            }
        }),
    ]
}
