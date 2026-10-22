use batten_core::{
    GrammarFormat, LexicalScope, ScopeSymbolKind, SymbolBinding, compile_grammar, filter_bindings,
};
use dado_core::TypedHole;
use miter_engine::Language;

#[test]
fn test_compile_gbnf_identifier() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = vec![
        SymbolBinding {
            name: "user_a".to_string(),
            ty: "User".to_string(),
            is_mutable: false,
            kind: ScopeSymbolKind::LocalVar,
        },
        SymbolBinding {
            name: "user_b".to_string(),
            ty: "User".to_string(),
            is_mutable: false,
            kind: ScopeSymbolKind::LocalVar,
        },
    ];

    let hole = TypedHole {
        hole_id: "id_hole".to_string(),
        expected_type: "User".to_string(),
        ast_node_kind: "identifier".to_string(),
        max_tokens: 16,
        allow_literals: false,
    };

    let filtered = filter_bindings(&bindings, &hole);
    let constraint = compile_grammar(&filtered, &hole, GrammarFormat::Gbnf)?;

    assert_eq!(constraint.format, GrammarFormat::Gbnf);
    assert!(constraint.grammar_definition.contains("root ::= ident"));
    assert!(constraint.grammar_definition.contains("\"user_a\""));
    assert!(constraint.grammar_definition.contains("\"user_b\""));
    Ok(())
}

#[test]
fn test_compile_lark_predicate() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = vec![SymbolBinding {
        name: "is_valid".to_string(),
        ty: "bool".to_string(),
        is_mutable: false,
        kind: ScopeSymbolKind::LocalVar,
    }];

    let hole = TypedHole {
        hole_id: "pred_hole".to_string(),
        expected_type: "bool".to_string(),
        ast_node_kind: "predicate".to_string(),
        max_tokens: 32,
        allow_literals: true,
    };

    let filtered = filter_bindings(&bindings, &hole);
    let constraint = compile_grammar(&filtered, &hole, GrammarFormat::Lark)?;

    assert_eq!(constraint.format, GrammarFormat::Lark);
    assert!(constraint.grammar_definition.contains("start: pred_expr"));
    assert!(constraint.grammar_definition.contains("\"is_valid\""));
    assert!(constraint.grammar_definition.contains("method_check"));
    Ok(())
}

#[test]
fn test_compile_regex_expression() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = vec![
        SymbolBinding {
            name: "user".to_string(),
            ty: "User".to_string(),
            is_mutable: false,
            kind: ScopeSymbolKind::LocalVar,
        },
        SymbolBinding {
            name: "fetch_user".to_string(),
            ty: "fn(&str) -> User".to_string(),
            is_mutable: false,
            kind: ScopeSymbolKind::Function,
        },
    ];

    let hole = TypedHole {
        hole_id: "expr_hole".to_string(),
        expected_type: "User".to_string(),
        ast_node_kind: "expression".to_string(),
        max_tokens: 64,
        allow_literals: true,
    };

    let scope = LexicalScope {
        repo_relative_path: "src/handler.rs".to_string(),
        language: Language::Rust,
        enclosing_symbol: Some("handle_request".to_string()),
        bindings,
        available_imports: vec!["crate::models::User".to_string()],
    };

    let filtered = batten_core::filter_scope_for_hole(&scope, &hole);
    let constraint = compile_grammar(&filtered, &hole, GrammarFormat::Regex)?;

    assert_eq!(constraint.format, GrammarFormat::Regex);
    let re = regex::Regex::new(&constraint.grammar_definition)?;
    assert!(re.is_match("user"));
    assert!(re.is_match("user.id"));
    assert!(re.is_match("fetch_user(user)"));
    Ok(())
}
