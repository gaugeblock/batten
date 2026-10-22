use batten_core::{
    CompiledGrammarConstraint, GrammarFormat, GrammarViolation, validate_candidate,
};

#[test]
fn test_validator_accepts_clean_in_scope_candidate() {
    let constraint = CompiledGrammarConstraint {
        hole_id: "test_hole".to_string(),
        format: GrammarFormat::Gbnf,
        grammar_definition: "".to_string(),
        allowed_identifiers: vec!["user".to_string(), "find_by_id".to_string()],
    };

    let report = validate_candidate("find_by_id(user.id)?", &constraint, false, Some(32));
    assert!(report.is_valid);
    assert!(report.violations.is_empty());
}

#[test]
fn test_validator_rejects_unrecognized_identifier() {
    let constraint = CompiledGrammarConstraint {
        hole_id: "test_hole".to_string(),
        format: GrammarFormat::Gbnf,
        grammar_definition: "".to_string(),
        allowed_identifiers: vec!["user".to_string()],
    };

    let report = validate_candidate("admin_token.verify()", &constraint, false, Some(32));
    assert!(!report.is_valid);
    assert_eq!(report.violations.len(), 1);
    match &report.violations[0] {
        GrammarViolation::UnrecognizedIdentifier { identifier } => {
            assert_eq!(identifier, "admin_token");
        }
        other => panic!("Unexpected violation: {other:?}"),
    }
}

#[test]
fn test_validator_rejects_disallowed_literals() {
    let constraint = CompiledGrammarConstraint {
        hole_id: "test_hole".to_string(),
        format: GrammarFormat::Gbnf,
        grammar_definition: "".to_string(),
        allowed_identifiers: vec!["count".to_string()],
    };

    let report = validate_candidate("count + 42", &constraint, false, Some(32));
    assert!(!report.is_valid);
    assert_eq!(report.violations.len(), 1);
    match &report.violations[0] {
        GrammarViolation::DisallowedLiteral { snippet } => {
            assert_eq!(snippet, "42");
        }
        other => panic!("Unexpected violation: {other:?}"),
    }
}

#[test]
fn test_validator_detects_unbalanced_delimiters() {
    let constraint = CompiledGrammarConstraint {
        hole_id: "test_hole".to_string(),
        format: GrammarFormat::Gbnf,
        grammar_definition: "".to_string(),
        allowed_identifiers: vec!["user".to_string()],
    };

    let report = validate_candidate("user.get(foo", &constraint, true, Some(32));
    assert!(!report.is_valid);
    assert!(report.violations.iter().any(|v| matches!(
        v,
        GrammarViolation::UnbalancedDelimiter { expected: ')', .. }
    )));
}
