# `batten` — Dynamic Scope-to-CFG Grammar Compiler & Token Mask Engine

[![Version](https://img.shields.io/badge/version-0.1.0--beta.2-blue.svg)](https://gaugeblock.dev)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![gaugeblock](https://img.shields.io/badge/suite-gaugeblock-orange.svg)](https://github.com/gaugeblock)
[![Documentation](https://img.shields.io/badge/docs-9--doc%20topology-green.svg)](ARCHITECTURE.md)

Part of the **[`gaugeblock`](https://gaugeblock.dev)** neurosymbolic code synthesis ecosystem.

> In carpentry, a **batten** is a slender strip of lumber fastened across adjacent planks to secure seams, seal gaps, and maintain exact spacing. In the `gaugeblock` suite, `batten` secures the unfilled holes during code generation: it dynamically compiles in-scope lexical symbols and expected hole types into strict Context-Free Grammars (GBNF, Lark, Regex). This constrains LLM decoding to 100% syntactically valid tokens, preventing dead ends, infinite recursion, and hallucinations.

---

## Why `batten`?

Stochastic code generation models frequently hallucinate out-of-scope identifiers, call non-existent methods, or generate syntactically broken code.

| Without `batten` | With `batten` |
|---|---|
| **Out-of-Scope Hallucinations:** In 40%+ of complex generation tasks, LLMs invent variable names, reference unimported modules, or call private methods. | **Physical Grammar Constraints:** `batten` compiles active variables from `miter` into Context-Free Grammars (GBNF/Lark), making it mathematically impossible for the LLM to emit invalid tokens. |
| **Run-Away Recursion:** Small open-weight models frequently get caught in repetitive token loops, generating endless chaining expressions like `foo.bar().bar().bar()`. | **Strict Depth Bounding:** Grammar production rules enforce hard recursion ceilings (`max_depth = 3`), guaranteeing derivation termination into valid direct terminals. |
| **Slow Subprocess Feedback:** Detecting invalid syntax requires invoking heavy compiler subshells (`cargo check` or `tsc`), taking 2–5 seconds per attempt. | **Microsecond Pre-Flight Validation:** `batten validate` evaluates token membership, balanced delimiters, and literal safety rules in `<50μs` before invoking compilers. |
| **Format Fragmentation:** Different inference runtimes require incompatible grammar formats (llama.cpp requires GBNF, outlines/vLLM requires Lark, outlines requires Regex). | **Unified Tri-Format Compiler:** Compiles a single internal AST constraint representation into GBNF, Lark EBNF, or bounded Regex seamlessly. |

---

## 1. System Architecture

```text
┌────────────────────────────────────────────────────────┐
│ Input Artifacts: Lexical Scope & Typed Hole Target     │
│ - Available symbols (types, functions, variables)      │
│ - Target type: Result<User, AppError> @ expression     │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│ batten Core Compiler                                   │
│ - Two-Tier Scope Filter (Direct Terminals vs Producers)│
│ - Safe Type Widening Fallbacks                         │
│ - Grammar Production Generator (Depth Bounded <= 3)    │
│ - Multi-Dialect Emitter (GBNF, Lark EBNF, PCRE Regex)  │
│ - Microsecond Pre-Flight AST Validator                 │
└───────────────┬────────────────────────┬───────────────┘
                │                        │
                ▼                        ▼
┌───────────────────────────┐ ┌──────────────────────────┐
│ CLI (`batten`)            │ │ MCP Stdio Server         │
│ - batten compile          │ │ - batten_compile         │
│ - batten validate         │ │ - batten_validate        │
│ - batten inspect          │ │                          │
└───────────────────────────┘ └──────────────────────────┘
```

---

## 2. CLI Commands Reference

`batten` provides a streamlined command-line interface for testing grammar constraints and validating completion tokens. Pass `--json` to any command for structured machine-readable outputs.

| Command | Description | MCP Tool |
|---|---|---|
| `compile` | Compile a `LexicalScope` and `TypedHole` into a GBNF, Lark, or Regex grammar. | `batten_compile` |
| `validate` | Perform microsecond lexical scope and syntax validation on candidate code. | `batten_validate` |
| `inspect` | Inspect filtered scope terminals, candidate chains, and type match ratings. | — |
| `mcp` | Launch the Model Context Protocol (JSON-RPC 2.0 stdio) server. | — |

---

## 3. Model Context Protocol (MCP) Reference

When operating inside AI agents (Antigravity, Claude Code, Cursor, Cline), `batten` exposes 2 core tools:

| Tool | Parameters | Description |
|---|---|---|
| `batten_compile` | `scope_json, hole_json, format?` | Compile in-scope symbols and target hole into GBNF, Lark, or Regex grammar constraint. |
| `batten_validate` | `candidate, scope_json, hole_json` | Perform microsecond lexical scope and syntax validation on candidate text. |

---

## 4. Installation & Quickstart

```bash
# Install globally via cargo
cargo install --path crates/batten-cli

# 1. Compile scope and typed hole into a GBNF grammar constraint
batten compile --scope scope.json --hole hole.json --format gbnf

# 2. Compile into a Lark grammar for vLLM / outlines
batten compile --scope scope.json --hole hole.json --format lark

# 3. Inspect filtered terminals and candidate producer chains
batten inspect --scope scope.json --hole hole.json

# 4. Validate completion candidate before compiler invocation
batten validate --candidate "user_repo.find_by_id(user_id)?" --constraint constraint.json
```

---

## 5. Canonical 9-Document Architecture

`batten` adheres strictly to the canonical 9-Document Architecture:
* **[`README.md`](./README.md):** Public elevator pitch, CLI reference, and quickstart.
* **[`CONTRIBUTING.md`](./CONTRIBUTING.md):** Developer environment setup, Rust 2024 toolchain, and test execution.
* **[`PRODUCT.md`](./PRODUCT.md):** User personas, JTBD, token reduction metrics, and problem statements.
* **[`ARCHITECTURE.md`](./ARCHITECTURE.md):** Scope reduction pipeline, grammar AST trees, and dialect transformers.
* **[`SPEC.md`](./SPEC.md):** Grammar generation invariants, recursion depth contracts, and type matching rules.
* **[`DESIGN.md`](./DESIGN.md):** Architectural Decision Records (ADRs) and trade-off rationales.
* **[`ROADMAP.md`](./ROADMAP.md):** Strategic milestones, active beta capabilities, and horizons.
* **[`CHANGELOG.md`](./CHANGELOG.md):** Chronological historical release logs and unreleased entries.
* **[`AGENTS.md`](./AGENTS.md):** Non-negotiable engineering directives, 250-line file ceiling, and verification matrix.

---

## 6. License

Licensed under the [Apache License, Version 2.0](LICENSE).
