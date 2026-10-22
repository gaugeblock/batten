//! Bounded Regular Expression grammar emitter.

use crate::error::Result;
use crate::model::FilteredScope;
use dado_core::TypedHole;

/// Emits a bounded regular expression pattern for a typed hole.
pub fn emit_regex(scope: &FilteredScope, hole: &TypedHole, _max_depth: usize) -> Result<String> {
    let idents = format_regex_alternatives(&scope.allowed_identifiers);

    let pattern = match hole.ast_node_kind.as_str() {
        "identifier" => format!("^({idents})$"),
        "predicate" => emit_regex_predicate(&idents, hole.allow_literals),
        "type_annotation" => emit_regex_type_annotation(scope),
        _ => emit_regex_expression(&idents, hole.allow_literals),
    };

    Ok(pattern)
}

fn format_regex_alternatives(idents: &[String]) -> String {
    idents
        .iter()
        .map(|id| regex::escape(id))
        .collect::<Vec<_>>()
        .join("|")
}

fn emit_regex_predicate(idents: &str, allow_literals: bool) -> String {
    let atom = if allow_literals {
        format!("(?:(?:{idents})(?:\\.(?:is_some|is_ok|is_err|is_empty)\\(\\))?|true|false|[0-9]+)")
    } else {
        format!("(?:(?:{idents})(?:\\.(?:is_some|is_ok|is_err|is_empty)\\(\\))?|true|false)")
    };

    let op = "(?:==|!=|<=|>=|<|>|&&|\\|\\|)";
    format!("^\\s*(!\\s*)?{atom}(?:\\s*{op}\\s*{atom})?\\s*$")
}

fn emit_regex_expression(idents: &str, allow_literals: bool) -> String {
    let atom = if allow_literals {
        format!("(?:(?:{idents})(?:\\s*\\((?:\\s*(?:{idents})(?:\\s*,\\s*(?:{idents}))*)?\\s*\\))?|[0-9]+|\"[^\"]*\"|true|false)")
    } else {
        format!("(?:(?:{idents})(?:\\s*\\((?:\\s*(?:{idents})(?:\\s*,\\s*(?:{idents}))*)?\\s*\\))?)")
    };

    let chain = "(?:\\.[a-zA-Z_][a-zA-Z0-9_]*(?:\\(\\))?|\\?|\\.await)*";
    let op = "(?:\\+|-|\\*|/|==|!=|&&|\\|\\|)";

    format!("^\\s*{atom}{chain}(?:\\s*{op}\\s*{atom}{chain})?\\s*$")
}

fn emit_regex_type_annotation(scope: &FilteredScope) -> String {
    let mut types = vec![
        "bool", "u8", "u32", "u64", "usize", "i32", "i64", "String", "&str",
    ];

    let custom_types: Vec<&str> = scope
        .allowed_identifiers
        .iter()
        .filter(|id| id.chars().next().is_some_and(|c| c.is_uppercase()))
        .map(|s| s.as_str())
        .collect();

    types.extend(custom_types);
    let type_alt = types.join("|");

    format!("^(?:{type_alt}|Option<\\s*(?:{type_alt})\\s*>|Result<\\s*(?:{type_alt})\\s*,\\s*(?:{type_alt})\\s*>)$")
}

