# `batten` — Dynamic Scope-to-CFG Grammar Compiler & Token Mask Engine

[![Version](https://img.shields.io/badge/version-0.1.0--beta.2-blue.svg)](https://gaugeblock.dev)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![gaugeblock](https://img.shields.io/badge/suite-gaugeblock-orange.svg)](https://github.com/gaugeblock)
[![Documentation](https://img.shields.io/badge/docs-9--doc%20topology-green.svg)](ARCHITECTURE.md)

Part of the **[`gaugeblock`](https://gaugeblock.dev)** neurosymbolic code synthesis ecosystem.

> In carpentry, a **batten** is a slender strip of lumber fastened across adjacent planks to secure seams, seal gaps, and maintain exact spacing. In the `gaugeblock` suite, `batten` secures the unfilled holes during code generation: it dynamically compiles in-scope lexical symbols and expected hole types into strict Context-Free Grammars (GBNF, Lark, Regex). This constrains LLM decoding to 100% syntactically valid tokens, preventing dead ends, infinite recursion, and hallucinations.

---

## 1. Architecture Overview

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

## 2. Core Capabilities

* **Two-Tier Type & Scope Filtering:** Categorizes accessible lexical symbols into *Direct Terminals* (variables/constants satisfying the type immediately or via `?` / `.await`) and *Producer Chains* (functions/methods yielding the type), with automatic fallback widening to eliminate empty grammars.
* **AST-Driven Grammar Productions:** Emits grammar rules specialized for target `ast_node_kind` (`expression`, `predicate`, `identifier`, `type_annotation`).
* **Bounded Recursion Depth:** Enforces strict recursion depth limits (`max_depth = 3`) to prevent repetitive loop traps during local model decoding.
* **Tri-Format Grammar Emission:** Compiles constraints directly into llama.cpp/mistral.rs **GBNF**, llguidance/vLLM/outlines **Lark**, and bounded **Regex**.
* **Zero-LLM Pre-Flight Validator:** Microsecond-fast validator verifying 100% identifier membership, delimiter balancing, and literal safety rules before escalating to compiler verification.

---

## 3. Workspace Crate Layout

* **`batten`**: Facade crate re-exporting `batten-core`.
* **`crates/batten-core`**: Core type analyzer, scope filters, grammar production engines (GBNF, Lark, Regex), and pre-flight validator.
* **`crates/batten-cli`**: Clap binary entrypoint (`batten`).
* **`crates/batten-mcp`**: JSON-RPC 2.0 stdio Model Context Protocol (MCP) server.

---

## 4. Installation & CLI Quickstart

```bash
# Install globally via cargo
cargo install --path crates/batten-cli

# 1. Compile scope and typed hole into a GBNF grammar constraint
batten compile --scope scope.json --hole hole.json --format gbnf

# 2. Compile into a Lark grammar for vLLM / outlines
batten compile --scope scope.json --hole hole.json --format lark

# 3. Compile bounded regex pattern
batten compile --scope scope.json --hole hole.json --format regex

# 4. Validate completion candidate before compiler invocation
batten validate --candidate "user_repo.find_by_id(user_id)?" --scope scope.json --hole hole.json
```

---

## 5. Model Context Protocol (MCP) Reference

| Tool | Parameters | Description |
|---|---|---|
| `batten_compile` | `scope_json, hole_json, format` | Compile in-scope symbols and target hole into GBNF, Lark, or Regex grammar. |
| `batten_validate` | `candidate, scope_json, hole_json` | Perform microsecond lexical scope and syntax validation on candidate text. |

---

## 6. Repository Documentation

`batten` adheres strictly to the canonical **9-Document Architecture**:
* **[`CONTRIBUTING.md`](./CONTRIBUTING.md):** Local toolchain prerequisites, grammar testing, and benchmark execution.
* **[`ARCHITECTURE.md`](./ARCHITECTURE.md):** Scope reduction pipeline, grammar AST trees, and dialect transformers.
* **[`SPEC.md`](./SPEC.md):** Grammar generation invariants, recursion depth contracts, and type matching rules.
* **[`AGENTS.md`](./AGENTS.md):** Autonomous systems directives, <=250 LOC limits, and zero-unwrap rules.
* **[`CHANGELOG.md`](./CHANGELOG.md):** Chronological historical release entries and unreleased changes.

---

## 7. Verification & Development Directives

* **File Size Ceiling:** All `.rs` files must strictly remain under 250 LOC.
* **Tiered Verification Matrix:**
  * Tier 1: `cargo check -p batten-core` (<2s).
  * Tier 2: `cargo fmt --check -p batten-core && cargo clippy -p batten-core && cargo test -p batten-core`.
  * Tier 3: `chalkline docs check -d .` before task verification.

---

## 8. License

Licensed under the [Apache License, Version 2.0](LICENSE).
Part of the [`gaugeblock`](https://gaugeblock.dev) suite.
