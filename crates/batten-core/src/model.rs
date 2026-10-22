//! Core data models and schemas for `batten-core`.

use miter_engine::Language;
use serde::{Deserialize, Serialize};

/// Supported grammar constraint formats across the `gaugeblock` suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrammarFormat {
    /// llama.cpp and mistral.rs BNF format.
    Gbnf,
    /// llguidance, vLLM, and outlines EBNF format.
    Lark,
    /// Bounded regular expression pattern.
    Regex,
}

impl std::fmt::Display for GrammarFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gbnf => write!(f, "gbnf"),
            Self::Lark => write!(f, "lark"),
            Self::Regex => write!(f, "regex"),
        }
    }
}

/// Kind of symbol binding inside a `LexicalScope`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeSymbolKind {
    /// Local variable binding (`let x = ...`).
    LocalVar,
    /// Function parameter.
    Parameter,
    /// Function or method.
    Function,
    /// Type, struct, or class definition.
    TypeOrStruct,
    /// Trait or interface definition.
    TraitOrInterface,
    /// Associated or global constant.
    Constant,
}

/// Alias for `ScopeSymbolKind` matching `CONTRACTS.md`.
pub type SymbolKind = ScopeSymbolKind;

/// An individual symbol binding in lexical scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolBinding {
    /// Identifier name.
    pub name: String,
    /// Declared or inferred type.
    pub ty: String,
    /// Whether the binding is mutable.
    pub is_mutable: bool,
    /// Kind of symbol.
    pub kind: ScopeSymbolKind,
}

/// Extracted lexical scope at a target AST location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexicalScope {
    /// Path of the source file relative to repository root.
    pub repo_relative_path: String,
    /// Target language.
    #[serde(with = "dado_core::language_serde")]
    pub language: Language,
    /// Name of enclosing function, method, or block.
    pub enclosing_symbol: Option<String>,
    /// Symbols actively in scope.
    pub bindings: Vec<SymbolBinding>,
    /// Available imports / use declarations.
    pub available_imports: Vec<String>,
}

/// Compiled grammar constraint produced by `batten` from `(LexicalScope + TypedHole)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledGrammarConstraint {
    /// Target hole identifier in the template (e.g. `query_expr`).
    pub hole_id: String,
    /// Format of the emitted grammar definition.
    pub format: GrammarFormat,
    /// Raw grammar definition (GBNF, Lark, or Regex string).
    pub grammar_definition: String,
    /// Allowed in-scope variable/function/constant identifiers.
    pub allowed_identifiers: Vec<String>,
}

/// Two-tier classification of a symbol candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateClassification {
    /// Directly satisfies the expected hole type or unwrap.
    DirectTerminal,
    /// Producer function/method or chaining target.
    ProducerChain,
}

/// A scope symbol candidate evaluated for grammar emission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeCandidate {
    /// Name of the symbol (variable, function, or constant).
    pub name: String,
    /// Declared or inferred type.
    pub ty: String,
    /// Symbol kind from lexical scope.
    pub kind: ScopeSymbolKind,
    /// Whether the binding is mutable.
    pub is_mutable: bool,
    /// Classification tier.
    pub classification: CandidateClassification,
}

/// Result of scope filtering against a typed hole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilteredScope {
    /// Direct terminal symbols (variables/constants satisfying type).
    pub direct_terminals: Vec<ScopeCandidate>,
    /// Producer functions or chaining candidates.
    pub producer_chains: Vec<ScopeCandidate>,
    /// All identifier tokens permitted in the generated grammar.
    pub allowed_identifiers: Vec<String>,
    /// True if zero direct matches triggered fallback widening.
    pub fallback_widened: bool,
}
