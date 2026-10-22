//! Command handlers executing CLI operations.

use crate::args::FormatArg;
use batten_core::{
    CompiledGrammarConstraint, GrammarConfig, LexicalScope, compile_grammar_with_config,
    filter_scope_for_hole, validate_candidate,
};
use dado_core::TypedHole;
use std::fs;
use std::path::Path;

/// Handles the `compile` subcommand.
pub fn handle_compile(
    scope_path: &Path,
    hole_path: &Path,
    format: FormatArg,
    max_depth: usize,
    output: Option<&Path>,
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let scope_content = fs::read_to_string(scope_path)?;
    let scope: LexicalScope = serde_json::from_str(&scope_content)?;

    let hole_content = fs::read_to_string(hole_path)?;
    let hole: TypedHole = serde_json::from_str(&hole_content)?;

    let config = GrammarConfig { max_depth };
    let filtered = filter_scope_for_hole(&scope, &hole);
    let constraint = compile_grammar_with_config(&filtered, &hole, format.into(), &config)?;

    let output_str = if json {
        serde_json::to_string_pretty(&constraint)?
    } else {
        constraint.grammar_definition
    };

    if let Some(out_p) = output {
        fs::write(out_p, output_str)?;
    } else {
        println!("{output_str}");
    }

    Ok(())
}

/// Handles the `validate` subcommand.
pub fn handle_validate(
    candidate: &str,
    constraint_path: &Path,
    allow_literals: bool,
    max_tokens: Option<u16>,
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let constraint_content = fs::read_to_string(constraint_path)?;
    let constraint: CompiledGrammarConstraint = serde_json::from_str(&constraint_content)?;

    let report = validate_candidate(candidate, &constraint, allow_literals, max_tokens);

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if report.is_valid {
        println!("VALID: Candidate satisfies grammar and scope constraints.");
    } else {
        eprintln!("INVALID: Candidate failed validation with violations:");
        for v in &report.violations {
            eprintln!(" - {v:?}");
        }
        std::process::exit(1);
    }

    Ok(())
}

/// Handles the `inspect` subcommand.
pub fn handle_inspect(
    scope_path: &Path,
    hole_path: &Path,
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let scope_content = fs::read_to_string(scope_path)?;
    let scope: LexicalScope = serde_json::from_str(&scope_content)?;

    let hole_content = fs::read_to_string(hole_path)?;
    let hole: TypedHole = serde_json::from_str(&hole_content)?;

    let filtered = filter_scope_for_hole(&scope, &hole);

    if json {
        println!("{}", serde_json::to_string_pretty(&filtered)?);
    } else {
        println!("Filtered Scope Analysis for Hole: {}", hole.hole_id);
        println!("Expected Type: {}", hole.expected_type);
        println!("Fallback Widened: {}", filtered.fallback_widened);
        println!("Direct Terminals ({}):", filtered.direct_terminals.len());
        for t in &filtered.direct_terminals {
            println!("  - {} : {}", t.name, t.ty);
        }
        println!("Producer Chains ({}):", filtered.producer_chains.len());
        for p in &filtered.producer_chains {
            println!("  - {} : {}", p.name, p.ty);
        }
    }

    Ok(())
}

