//! Micro-lexer for identifier token and literal extraction.

use std::collections::HashSet;

/// Extracted tokens from a candidate fill string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedTokens {
    /// In-scope identifier candidates referenced by the candidate.
    pub referenced_identifiers: Vec<String>,
    /// Literal constants detected in the candidate.
    pub literals: Vec<String>,
}

const RUST_KEYWORDS: &[&str] = &[
    "let", "mut", "await", "true", "false", "if", "else", "match", "return", "as", "fn", "async",
    "self", "Self", "crate", "super",
];

const STANDARD_CONSTRUCTORS: &[&str] = &[
    "Ok", "Err", "Some", "None", "Vec", "Box", "String", "Default", "Result", "Option",
];

const COMMON_METHODS: &[&str] = &[
    "clone",
    "into",
    "to_string",
    "to_owned",
    "as_str",
    "unwrap",
    "expect",
    "unwrap_or",
    "unwrap_or_default",
    "is_some",
    "is_ok",
    "is_err",
    "is_empty",
    "len",
    "map",
    "and_then",
    "iter",
    "collect",
];

/// Extracts free identifier tokens and literal fragments from a candidate string.
pub fn extract_tokens_and_literals(candidate: &str) -> ExtractedTokens {
    let mut referenced_identifiers = Vec::new();
    let mut literals = Vec::new();

    let chars: Vec<char> = candidate.chars().collect();
    let n = chars.len();
    let mut i = 0;

    let keywords: HashSet<&str> = RUST_KEYWORDS
        .iter()
        .chain(STANDARD_CONSTRUCTORS)
        .chain(COMMON_METHODS)
        .copied()
        .collect();

    while i < n {
        let ch = chars[i];

        if ch.is_whitespace() {
            i += 1;
            continue;
        }

        // Quoted string literal
        if ch == '"' {
            let start = i;
            i += 1;
            let mut escaped = false;
            while i < n {
                if escaped {
                    escaped = false;
                } else if chars[i] == '\\' {
                    escaped = true;
                } else if chars[i] == '"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            literals.push(s);
            continue;
        }

        // Numeric literal
        if ch.is_ascii_digit() {
            let start = i;
            while i < n && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_')
            {
                // Prevent capturing method call after number like 42.to_string()
                if chars[i] == '.' && i + 1 < n && chars[i + 1].is_alphabetic() {
                    break;
                }
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            literals.push(s);
            continue;
        }

        // Identifier or keyword
        if ch.is_alphabetic() || ch == '_' {
            let is_field_access = i > 0 && chars[i - 1] == '.';
            let start = i;
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();

            if !is_field_access && !keywords.contains(ident.as_str()) {
                referenced_identifiers.push(ident);
            }
            continue;
        }

        i += 1;
    }

    ExtractedTokens {
        referenced_identifiers,
        literals,
    }
}
