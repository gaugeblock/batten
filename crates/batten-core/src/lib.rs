//! # `batten-core`
//!
//! Dynamic Scope-to-CFG Grammar Compiler and Token Mask Generator.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod model;
pub mod scope_filter;
pub mod type_norm;

pub use error::{BattenError, Result};
pub use model::{
    CandidateClassification, CompiledGrammarConstraint, FilteredScope, GrammarFormat, LexicalScope,
    ScopeCandidate, ScopeSymbolKind, SymbolBinding, SymbolKind,
};
pub use scope_filter::{filter_bindings, filter_scope_for_hole};
pub use type_norm::{
    TypeMatch, evaluate_type_compatibility, extract_return_type, extract_unwrapped_type,
    normalize_type_str,
};

/// Version of the `batten-core` crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
