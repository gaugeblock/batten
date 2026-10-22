use batten_core::{
    CandidateClassification, ScopeSymbolKind, SymbolBinding, TypeMatch,
    evaluate_type_compatibility, filter_bindings, normalize_type_str,
};
use dado_core::TypedHole;

#[test]
fn test_normalize_types() {
    assert_eq!(normalize_type_str("&str"), "str");
    assert_eq!(normalize_type_str("&mut String"), "String");
    assert_eq!(normalize_type_str("Box<User>"), "User");
    assert_eq!(normalize_type_str("Arc<AppConfig>"), "AppConfig");
}

#[test]
fn test_evaluate_type_compatibility() {
    assert_eq!(
        evaluate_type_compatibility("User", "User"),
        TypeMatch::Exact
    );
    assert_eq!(
        evaluate_type_compatibility("&str", "String"),
        TypeMatch::StringConversion
    );
    assert_eq!(
        evaluate_type_compatibility("Result<User, AppError>", "User"),
        TypeMatch::UnwrappedResult
    );
    assert_eq!(
        evaluate_type_compatibility("Option<User>", "User"),
        TypeMatch::UnwrappedOption
    );
    assert_eq!(
        evaluate_type_compatibility("fn(&str) -> User", "User"),
        TypeMatch::FunctionReturn
    );
    assert_eq!(
        evaluate_type_compatibility("i32", "bool"),
        TypeMatch::Incompatible
    );
}

#[test]
fn test_two_tier_scope_filter() {
    let bindings = vec![
        SymbolBinding {
            name: "current_user".to_string(),
            ty: "User".to_string(),
            is_mutable: false,
            kind: ScopeSymbolKind::LocalVar,
        },
        SymbolBinding {
            name: "counter".to_string(),
            ty: "i32".to_string(),
            is_mutable: false,
            kind: ScopeSymbolKind::LocalVar,
        },
        SymbolBinding {
            name: "find_user".to_string(),
            ty: "fn(&str) -> Result<User, AppError>".to_string(),
            is_mutable: false,
            kind: ScopeSymbolKind::Function,
        },
    ];

    let hole = TypedHole {
        hole_id: "user_hole".to_string(),
        expected_type: "User".to_string(),
        ast_node_kind: "expression".to_string(),
        max_tokens: 64,
        allow_literals: false,
    };

    let filtered = filter_bindings(&bindings, &hole);
    assert!(!filtered.fallback_widened);
    assert_eq!(filtered.direct_terminals.len(), 1);
    assert_eq!(filtered.direct_terminals[0].name, "current_user");
    assert_eq!(
        filtered.direct_terminals[0].classification,
        CandidateClassification::DirectTerminal
    );

    assert_eq!(filtered.producer_chains.len(), 1);
    assert_eq!(filtered.producer_chains[0].name, "find_user");
    assert_eq!(
        filtered.producer_chains[0].classification,
        CandidateClassification::ProducerChain
    );

    assert!(
        filtered
            .allowed_identifiers
            .contains(&"current_user".to_string())
    );
    assert!(
        filtered
            .allowed_identifiers
            .contains(&"find_user".to_string())
    );
}

#[test]
fn test_fallback_widening_when_no_direct_matches() {
    let bindings = vec![SymbolBinding {
        name: "magic_number".to_string(),
        ty: "u64".to_string(),
        is_mutable: false,
        kind: ScopeSymbolKind::Constant,
    }];

    let hole = TypedHole {
        hole_id: "opaque_hole".to_string(),
        expected_type: "MyCustomProjection<T>".to_string(),
        ast_node_kind: "expression".to_string(),
        max_tokens: 32,
        allow_literals: false,
    };

    let filtered = filter_bindings(&bindings, &hole);
    assert!(filtered.fallback_widened);
    assert_eq!(filtered.direct_terminals.len(), 1);
    assert_eq!(filtered.direct_terminals[0].name, "magic_number");
}
