//! Strongly typed errors for `batten-core`.

use thiserror::Error;

/// Result type alias for `batten-core` operations.
pub type Result<T> = std::result::Result<T, BattenError>;

/// Strongly typed error enum for grammar compilation and validation.
#[derive(Debug, Error)]
pub enum BattenError {
    /// Failure during grammar emission.
    #[error("Grammar emission error: {0}")]
    GrammarEmission(String),

    /// Validation error when candidate fill violates grammar constraint.
    #[error("Validation failed: {0}")]
    Validation(String),

    /// Scope parsing or resolution error.
    #[error("Scope resolution error: {0}")]
    ScopeResolution(String),

    /// Serialization or deserialization error.
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// I/O error during scope or grammar file access.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

