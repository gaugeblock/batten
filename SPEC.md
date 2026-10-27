# Behavioral Specifications & Invariants

## Formal Type Contracts

- **`error::BattenError`** (`enum`): Strongly typed error enum for grammar compilation and validation.
- **`model::CandidateClassification`** (`enum`): Two-tier classification of a symbol candidate.
- **`args::Cli`** (`struct`): Dynamic variable-scope to Context-Free Grammar (GBNF/Lark) compiler and token mask generator.
- **`args::Command`** (`enum`): Available CLI subcommands.
- **`model::CompiledGrammarConstraint`** (`struct`): Compiled grammar constraint produced by `batten` from `(LexicalScope + TypedHole)`.
- **`tokens::ExtractedTokens`** (`struct`): Extracted tokens from a candidate fill string.
- **`model::FilteredScope`** (`struct`): Result of scope filtering against a typed hole.
- **`args::FormatArg`** (`enum`): Grammar constraint output formats.
- **`mod::GrammarConfig`** (`struct`): Configuration options for the grammar compiler.
- **`model::GrammarFormat`** (`enum`): Supported grammar constraint formats across the `gaugeblock` suite.
- **`mod::GrammarViolation`** (`enum`): Specific violation detected by the pre-flight validator.
- **`model::LexicalScope`** (`struct`): Extracted lexical scope at a target AST location.
- **`model::ScopeCandidate`** (`struct`): A scope symbol candidate evaluated for grammar emission.
- **`model::ScopeSymbolKind`** (`enum`): Kind of symbol binding inside a `LexicalScope`.
- **`model::SymbolBinding`** (`struct`): An individual symbol binding in lexical scope.
- **`type_norm::TypeMatch`** (`enum`): Semantic relationship between an in-scope symbol type and the expected type.
- **`mod::ValidationReport`** (`struct`): Comprehensive validation diagnostic report.
