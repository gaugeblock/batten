//! Normalized type matching and wrapper unwrap resolution.

/// Semantic relationship between an in-scope symbol type and the expected type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeMatch {
    /// Exact normalized match (e.g. `User` vs `User`).
    Exact,
    /// Satisfies via reference or dereference (e.g. `&T` or `Box<T>` vs `T`).
    BorrowOrDeref,
    /// Unwraps via `?` from `Result<T, E>`.
    UnwrappedResult,
    /// Unwraps via `?` from `Option<T>`.
    UnwrappedOption,
    /// Resolves via `.await` from `Future<Output = T>`.
    UnwrappedFuture,
    /// String conversion (e.g. `String` vs `&str` or vice-versa).
    StringConversion,
    /// Direct producer function returning the expected type.
    FunctionReturn,
    /// Types are semantically incompatible.
    Incompatible,
}

impl TypeMatch {
    /// True if the match satisfies a direct terminal hole expression.
    pub fn is_direct_terminal(self) -> bool {
        matches!(
            self,
            Self::Exact
                | Self::BorrowOrDeref
                | Self::UnwrappedResult
                | Self::UnwrappedOption
                | Self::UnwrappedFuture
                | Self::StringConversion
        )
    }
}

/// Normalizes a type string by stripping lifetimes, outer references, and whitespace.
pub fn normalize_type_str(raw: &str) -> String {
    let mut s = raw.trim().to_string();
    s = s.replace("mut ", "");
    while s.starts_with('&') || s.starts_with('*') {
        s = s[1..].trim().to_string();
    }
    // Strip simple lifetime like 'a
    if s.starts_with('\'')
        && let Some(pos) = s.find(' ')
    {
        s = s[pos + 1..].trim().to_string();
    }
    strip_smart_pointers(&s)
}

fn strip_smart_pointers(s: &str) -> String {
    let prefixes = ["Box<", "Arc<", "Rc<", "std::sync::Arc<", "std::rc::Rc<"];
    for p in prefixes {
        if s.starts_with(p) && s.ends_with('>') {
            let inner = &s[p.len()..s.len() - 1];
            return normalize_type_str(inner);
        }
    }
    s.to_string()
}

/// Extracts inner type from standard wrapper envelopes (`Result`, `Option`, `Future`).
pub fn extract_unwrapped_type(raw: &str) -> Option<(TypeMatch, String)> {
    let norm = normalize_type_str(raw);
    if norm.starts_with("Result<") && norm.ends_with('>') {
        let inner = &norm[7..norm.len() - 1];
        let t = inner.split(',').next().unwrap_or(inner).trim();
        return Some((TypeMatch::UnwrappedResult, normalize_type_str(t)));
    }
    if norm.starts_with("Option<") && norm.ends_with('>') {
        let inner = &norm[7..norm.len() - 1].trim();
        return Some((TypeMatch::UnwrappedOption, normalize_type_str(inner)));
    }
    if norm.contains("Future<Output")
        && norm.ends_with('>')
        && let Some(idx) = norm.find("Output =")
    {
        let rest = &norm[idx + 8..norm.len() - 1];
        let t = rest.split('>').next().unwrap_or(rest).trim();
        return Some((TypeMatch::UnwrappedFuture, normalize_type_str(t)));
    }
    None
}

/// Extracts return type from a function signature if present.
pub fn extract_return_type(raw: &str) -> Option<String> {
    if let Some(idx) = raw.find("->") {
        let ret = raw[idx + 2..].trim();
        Some(normalize_type_str(ret))
    } else {
        None
    }
}

/// Checks semantic compatibility between a candidate symbol type and target expected type.
pub fn evaluate_type_compatibility(candidate_ty: &str, expected_ty: &str) -> TypeMatch {
    let cand_norm = normalize_type_str(candidate_ty);
    let exp_norm = normalize_type_str(expected_ty);

    if cand_norm == exp_norm {
        return TypeMatch::Exact;
    }

    // String / str cross-compatibility
    let is_cand_str = cand_norm == "String" || cand_norm == "str";
    let is_exp_str = exp_norm == "String" || exp_norm == "str";
    if is_cand_str && is_exp_str {
        return TypeMatch::StringConversion;
    }

    // Check unwraps (Result, Option, Future)
    if let Some((match_kind, unwrapped)) = extract_unwrapped_type(candidate_ty)
        && unwrapped == exp_norm
    {
        return match_kind;
    }

    // Check function return type
    if let Some(ret_ty) = extract_return_type(candidate_ty) {
        if ret_ty == exp_norm {
            return TypeMatch::FunctionReturn;
        }
        if let Some((_, unwrapped_ret)) = extract_unwrapped_type(&ret_ty)
            && unwrapped_ret == exp_norm
        {
            return TypeMatch::FunctionReturn;
        }
    }

    TypeMatch::Incompatible
}
