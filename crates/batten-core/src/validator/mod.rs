//! Microsecond-fast zero-LLM deterministic grammar validator.

pub mod delimiters;
pub mod tokens;

use crate::model::CompiledGrammarConstraint;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Specific violation detected by the pre-flight validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GrammarViolation {
    /// An identifier was referenced that is not present in `allowed_identifiers`.
    UnrecognizedIdentifier {
        /// Identifier name.
        identifier: String,
    },
    /// A literal constant was found when `allow_literals` is false.
    DisallowedLiteral {
        /// The literal snippet.
        snippet: String,
    },
    /// Unbalanced delimiter detected in the candidate expression.
    UnbalancedDelimiter {
        /// Expected delimiter character.
        expected: char,
        /// Encountered mismatched character if any.
        found: Option<char>,
        /// Byte offset in candidate string.
        position: usize,
    },
    /// Candidate string exceeds the permitted token/character budget.
    LengthLimitExceeded {
        /// Approximate token count.
        length: usize,
        /// Maximum allowed tokens.
        max_allowed: usize,
    },
}

/// Comprehensive validation diagnostic report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Whether the candidate satisfies all grammar and safety constraints.
    pub is_valid: bool,
    /// Detailed list of violations detected.
    pub violations: Vec<GrammarViolation>,
    /// Identifiers inspected during validation.
    pub checked_identifiers: Vec<String>,
}

/// Validates a candidate fill string against a `CompiledGrammarConstraint`.
pub fn validate_candidate(
    candidate: &str,
    constraint: &CompiledGrammarConstraint,
    allow_literals: bool,
    max_tokens: Option<u16>,
) -> ValidationReport {
    let mut violations = Vec::new();

    // 1. Length budget check (heuristic: 1 token ~ 3.5 chars)
    if let Some(max) = max_tokens {
        let approx_tokens = candidate.len().div_ceil(4);
        if approx_tokens > max as usize {
            violations.push(GrammarViolation::LengthLimitExceeded {
                length: approx_tokens,
                max_allowed: max as usize,
            });
        }
    }

    // 2. Delimiter balancing check
    violations.extend(delimiters::check_balanced_delimiters(candidate));

    // 3. Token extraction and identifier verification
    let extracted = tokens::extract_tokens_and_literals(candidate);
    let allowed_set: HashSet<&str> = constraint
        .allowed_identifiers
        .iter()
        .map(|s| s.as_str())
        .collect();

    for ident in &extracted.referenced_identifiers {
        if !allowed_set.contains(ident.as_str()) {
            violations.push(GrammarViolation::UnrecognizedIdentifier {
                identifier: ident.clone(),
            });
        }
    }

    // 4. Literal policy check
    if !allow_literals && !extracted.literals.is_empty() {
        for lit in &extracted.literals {
            // "true" and "false" boolean values are permitted in predicate contexts
            if lit != "true" && lit != "false" {
                violations.push(GrammarViolation::DisallowedLiteral {
                    snippet: lit.clone(),
                });
            }
        }
    }

    ValidationReport {
        is_valid: violations.is_empty(),
        violations,
        checked_identifiers: extracted.referenced_identifiers,
    }
}
