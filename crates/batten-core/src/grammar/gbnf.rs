//! GBNF (llama.cpp / mistral.rs) grammar emitter.

use crate::error::Result;
use crate::model::FilteredScope;
use dado_core::TypedHole;

/// Emits a bounded GBNF grammar definition for a typed hole.
pub fn emit_gbnf(scope: &FilteredScope, hole: &TypedHole, max_depth: usize) -> Result<String> {
    let mut out = String::new();
    let idents = format_ident_alternatives(&scope.allowed_identifiers);

    match hole.ast_node_kind.as_str() {
        "identifier" => {
            out.push_str("root ::= ident\n");
            out.push_str(&format!("ident ::= {idents}\n"));
        }
        "predicate" => {
            emit_gbnf_predicate(&mut out, &idents, hole.allow_literals);
        }
        "type_annotation" => {
            emit_gbnf_type_annotation(&mut out, scope);
        }
        _ => {
            // Default: expression or match_arm
            emit_gbnf_expression(&mut out, &idents, hole.allow_literals, max_depth);
        }
    }

    out.push_str("ws ::= [ \\t\\n\\r]*\n");
    Ok(out)
}

fn format_ident_alternatives(idents: &[String]) -> String {
    idents
        .iter()
        .map(|id| format!("\"{}\"", escape_gbnf_str(id)))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn escape_gbnf_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn emit_gbnf_predicate(out: &mut String, idents: &str, allow_literals: bool) {
    out.push_str("root ::= pred_expr\n");
    out.push_str("pred_expr ::= ( \"!\" ws )? pred_atom ( ws pred_op ws pred_atom )?\n");
    out.push_str(
        "pred_op ::= \"==\" | \"!=\" | \"<=\" | \">=\" | \"<\" | \">\" | \"&&\" | \"||\"\n",
    );
    out.push_str("pred_atom ::= ident ( \".\" method_check )? | literal\n");
    out.push_str("method_check ::= \"is_some()\" | \"is_ok()\" | \"is_err()\" | \"is_empty()\"\n");
    out.push_str(&format!("ident ::= {idents}\n"));
    if allow_literals {
        out.push_str("literal ::= \"true\" | \"false\" | [0-9]+\n");
    } else {
        out.push_str("literal ::= \"true\" | \"false\"\n");
    }
}

fn emit_gbnf_expression(out: &mut String, idents: &str, allow_literals: bool, max_depth: usize) {
    out.push_str("root ::= expr_0\n");

    for d in 0..max_depth {
        let next_d = d + 1;
        out.push_str(&format!(
            "expr_{d} ::= expr_{next_d} ( ws bin_op ws expr_{next_d} )?\n"
        ));
    }

    let atom_level = max_depth;
    out.push_str(&format!(
        "expr_{atom_level} ::= atom ( \".\" member_call | \"?\" | \".await\" )*\n"
    ));
    out.push_str(
        "atom ::= ident ( \"(\" ( ws ident ( ws \",\" ws ident )* )? ws \")\" )? | \"(\" ws expr_0 ws \")\"",
    );
    if allow_literals {
        out.push_str(" | literal\n");
        out.push_str("literal ::= [0-9]+ | \"\\\"\" [^\"\\n]* \"\\\"\" | \"true\" | \"false\"\n");
    } else {
        out.push('\n');
    }

    out.push_str("bin_op ::= \"+\" | \"-\" | \"*\" | \"/\" | \"==\" | \"!=\" | \"&&\" | \"||\"\n");
    out.push_str("member_call ::= [a-zA-Z_][a-zA-Z0-9_]* ( \"()\" )?\n");
    out.push_str(&format!("ident ::= {idents}\n"));
}

fn emit_gbnf_type_annotation(out: &mut String, scope: &FilteredScope) {
    out.push_str("root ::= type_expr\n");
    out.push_str("type_expr ::= prim_type | custom_type | wrapper_type\n");
    out.push_str("prim_type ::= \"bool\" | \"u8\" | \"u32\" | \"u64\" | \"usize\" | \"i32\" | \"i64\" | \"String\" | \"&str\"\n");

    let custom_types: Vec<_> = scope
        .allowed_identifiers
        .iter()
        .filter(|id| id.chars().next().is_some_and(|c| c.is_uppercase()))
        .map(|id| format!("\"{}\"", escape_gbnf_str(id)))
        .collect();

    if custom_types.is_empty() {
        out.push_str("custom_type ::= prim_type\n");
    } else {
        out.push_str(&format!("custom_type ::= {}\n", custom_types.join(" | ")));
    }

    out.push_str("wrapper_type ::= \"Option<\" ws type_expr ws \">\" | \"Result<\" ws type_expr ws \",\" ws type_expr ws \">\"\n");
}
