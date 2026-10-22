//! Two-tier type and lexical scope filtering with safe fallback widening.

use crate::model::{
    CandidateClassification, FilteredScope, LexicalScope, ScopeCandidate, ScopeSymbolKind,
    SymbolBinding,
};
use crate::type_norm::{TypeMatch, evaluate_type_compatibility};
use dado_core::TypedHole;
use std::collections::BTreeSet;

/// Filters a `LexicalScope` against a `TypedHole`, sorting candidates into two tiers.
pub fn filter_scope_for_hole(scope: &LexicalScope, hole: &TypedHole) -> FilteredScope {
    let mut direct_terminals = Vec::new();
    let mut producer_chains = Vec::new();
    let mut allowed_ident_set = BTreeSet::new();

    for binding in &scope.bindings {
        let match_kind = evaluate_type_compatibility(&binding.ty, &hole.expected_type);

        if match_kind.is_direct_terminal() && is_value_symbol(binding.kind) {
            direct_terminals.push(ScopeCandidate {
                name: binding.name.clone(),
                ty: binding.ty.clone(),
                kind: binding.kind,
                is_mutable: binding.is_mutable,
                classification: CandidateClassification::DirectTerminal,
            });
            allowed_ident_set.insert(binding.name.clone());
        } else if match_kind == TypeMatch::FunctionReturn
            || binding.kind == ScopeSymbolKind::Function
        {
            producer_chains.push(ScopeCandidate {
                name: binding.name.clone(),
                ty: binding.ty.clone(),
                kind: binding.kind,
                is_mutable: binding.is_mutable,
                classification: CandidateClassification::ProducerChain,
            });
            allowed_ident_set.insert(binding.name.clone());
        }
    }

    let mut fallback_widened = false;
    // Fallback safety: widen if zero direct terminals found
    if direct_terminals.is_empty() {
        fallback_widened = true;
        for binding in &scope.bindings {
            if is_value_symbol(binding.kind) {
                direct_terminals.push(ScopeCandidate {
                    name: binding.name.clone(),
                    ty: binding.ty.clone(),
                    kind: binding.kind,
                    is_mutable: binding.is_mutable,
                    classification: CandidateClassification::DirectTerminal,
                });
                allowed_ident_set.insert(binding.name.clone());
            }
        }
    }

    // Always include producer chains in allowed identifiers
    for candidate in &producer_chains {
        allowed_ident_set.insert(candidate.name.clone());
    }

    FilteredScope {
        direct_terminals,
        producer_chains,
        allowed_identifiers: allowed_ident_set.into_iter().collect(),
        fallback_widened,
    }
}

fn is_value_symbol(kind: ScopeSymbolKind) -> bool {
    matches!(
        kind,
        ScopeSymbolKind::LocalVar | ScopeSymbolKind::Parameter | ScopeSymbolKind::Constant
    )
}

/// Convenience function to filter symbols from raw bindings list.
pub fn filter_bindings(bindings: &[SymbolBinding], hole: &TypedHole) -> FilteredScope {
    let scope = LexicalScope {
        repo_relative_path: "memory".to_string(),
        language: miter_engine::Language::Rust,
        enclosing_symbol: None,
        bindings: bindings.to_vec(),
        available_imports: Vec::new(),
    };
    filter_scope_for_hole(&scope, hole)
}
