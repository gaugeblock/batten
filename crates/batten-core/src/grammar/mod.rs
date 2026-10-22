//! Context-Free Grammar (CFG) emitters and compiler orchestrator.

pub mod gbnf;
pub mod lark;
pub mod regex;

use crate::error::{BattenError, Result};
use crate::model::{CompiledGrammarConstraint, FilteredScope, GrammarFormat};
use dado_core::TypedHole;

/// Configuration options for the grammar compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarConfig {
    /// Maximum recursive expression nesting depth (defaults to 3).
    pub max_depth: usize,
}

impl Default for GrammarConfig {
    fn default() -> Self {
        Self { max_depth: 3 }
    }
}

/// Compiles a `FilteredScope` and `TypedHole` into a `CompiledGrammarConstraint`.
pub fn compile_grammar(
    scope: &FilteredScope,
    hole: &TypedHole,
    format: GrammarFormat,
) -> Result<CompiledGrammarConstraint> {
    compile_grammar_with_config(scope, hole, format, &GrammarConfig::default())
}

/// Compiles a grammar constraint using an explicit `GrammarConfig`.
pub fn compile_grammar_with_config(
    scope: &FilteredScope,
    hole: &TypedHole,
    format: GrammarFormat,
    config: &GrammarConfig,
) -> Result<CompiledGrammarConstraint> {
    if scope.allowed_identifiers.is_empty() {
        return Err(BattenError::GrammarEmission(
            "Cannot emit grammar constraint with zero allowed identifiers".to_string(),
        ));
    }

    let grammar_definition = match format {
        GrammarFormat::Gbnf => gbnf::emit_gbnf(scope, hole, config.max_depth)?,
        GrammarFormat::Lark => lark::emit_lark(scope, hole, config.max_depth)?,
        GrammarFormat::Regex => regex::emit_regex(scope, hole, config.max_depth)?,
    };

    Ok(CompiledGrammarConstraint {
        hole_id: hole.hole_id.clone(),
        format,
        grammar_definition,
        allowed_identifiers: scope.allowed_identifiers.clone(),
    })
}

