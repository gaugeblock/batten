//! Stack-based balanced delimiter checking for candidate expressions.

use crate::validator::GrammarViolation;

/// Checks that parentheses, brackets, and braces in the candidate string are balanced.
pub fn check_balanced_delimiters(candidate: &str) -> Vec<GrammarViolation> {
    let mut violations = Vec::new();
    let mut stack: Vec<(char, usize)> = Vec::new();
    let mut in_string = false;
    let mut escaped = false;

    for (pos, ch) in candidate.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
            continue;
        }

        match ch {
            '(' | '[' | '{' => stack.push((ch, pos)),
            ')' => {
                if let Some((top, _)) = stack.pop() {
                    if top != '(' {
                        violations.push(GrammarViolation::UnbalancedDelimiter {
                            expected: ')',
                            found: Some(top),
                            position: pos,
                        });
                    }
                } else {
                    violations.push(GrammarViolation::UnbalancedDelimiter {
                        expected: '(',
                        found: Some(')'),
                        position: pos,
                    });
                }
            }
            ']' => {
                if let Some((top, _)) = stack.pop() {
                    if top != '[' {
                        violations.push(GrammarViolation::UnbalancedDelimiter {
                            expected: ']',
                            found: Some(top),
                            position: pos,
                        });
                    }
                } else {
                    violations.push(GrammarViolation::UnbalancedDelimiter {
                        expected: '[',
                        found: Some(']'),
                        position: pos,
                    });
                }
            }
            '}' => {
                if let Some((top, _)) = stack.pop() {
                    if top != '{' {
                        violations.push(GrammarViolation::UnbalancedDelimiter {
                            expected: '}',
                            found: Some(top),
                            position: pos,
                        });
                    }
                } else {
                    violations.push(GrammarViolation::UnbalancedDelimiter {
                        expected: '{',
                        found: Some('}'),
                        position: pos,
                    });
                }
            }
            _ => {}
        }
    }

    if in_string {
        violations.push(GrammarViolation::UnbalancedDelimiter {
            expected: '"',
            found: None,
            position: candidate.len(),
        });
    }

    while let Some((unclosed, pos)) = stack.pop() {
        let expected = match unclosed {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            other => other,
        };
        violations.push(GrammarViolation::UnbalancedDelimiter {
            expected,
            found: None,
            position: pos,
        });
    }

    violations
}

