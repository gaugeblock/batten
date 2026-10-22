//! Command-line argument parser definitions for `batten`.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Dynamic variable-scope to Context-Free Grammar (GBNF/Lark) compiler and token mask generator.
#[derive(Debug, Parser)]
#[command(name = "batten", version, about, long_about = None)]
pub struct Cli {
    /// Verbose logging output to stderr.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Grammar constraint output formats.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FormatArg {
    /// llama.cpp / mistral.rs BNF format.
    Gbnf,
    /// llguidance / outlines EBNF format.
    Lark,
    /// Bounded regex pattern.
    Regex,
}

impl From<FormatArg> for batten_core::GrammarFormat {
    fn from(arg: FormatArg) -> Self {
        match arg {
            FormatArg::Gbnf => Self::Gbnf,
            FormatArg::Lark => Self::Lark,
            FormatArg::Regex => Self::Regex,
        }
    }
}

/// Available CLI subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Compiles a LexicalScope and TypedHole into a Context-Free Grammar constraint.
    Compile {
        /// Path to LexicalScope JSON file.
        #[arg(short, long)]
        scope: PathBuf,

        /// Path to TypedHole JSON file.
        #[arg(short, long)]
        hole: PathBuf,

        /// Grammar output format.
        #[arg(short, long, value_enum, default_value_t = FormatArg::Gbnf)]
        format: FormatArg,

        /// Maximum recursive expression nesting depth.
        #[arg(short = 'd', long, default_value_t = 3)]
        max_depth: usize,

        /// Output file path (defaults to stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Emit full CompiledGrammarConstraint as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Validates a candidate code fill against a compiled grammar constraint.
    Validate {
        /// Candidate expression or snippet to validate.
        #[arg(short, long)]
        candidate: String,

        /// Path to CompiledGrammarConstraint JSON file.
        #[arg(short, long)]
        constraint: PathBuf,

        /// Permit literal constants in candidate.
        #[arg(long)]
        allow_literals: bool,

        /// Optional maximum token budget.
        #[arg(long)]
        max_tokens: Option<u16>,

        /// Emit structured validation report as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Inspects filtered scope terminals and candidate chains for a typed hole.
    Inspect {
        /// Path to LexicalScope JSON file.
        #[arg(short, long)]
        scope: PathBuf,

        /// Path to TypedHole JSON file.
        #[arg(short, long)]
        hole: PathBuf,

        /// Format output as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Starts the Model Context Protocol (MCP) JSON-RPC 2.0 stdio server.
    Mcp,
}

