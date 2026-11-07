# Product Vision & Specifications — `batten`

> **Domain:** Dynamic Scope-to-CFG Grammar Compiler & Token Mask Engine  
> **Ecosystem:** `gaugeblock` Neurosymbolic Meta-Harness  
> **License:** Apache-2.0

---

## 1. Problem Statement & Market Context

When AI coding models fill logic holes in codebases, they suffer from three chronic failure modes:

1. **Out-of-Scope Identifier Hallucination:** Models guess variable names from other files or training data, referencing non-existent identifiers or calling private functions. Prompting alone fails to stop this.
2. **Infinite Token Recursion:** Open-weight models (7B–14B) frequently fall into repetitive decoding loops, generating endless method chaining expressions that blow past token limits.
3. **Severe Compiler Feedback Latency:** Verifying whether generated code is syntactically sound requires invoking language compilers (`cargo check`, `tsc`), taking 2–5 seconds per candidate and severely bottlenecking automated synthesis loops.

`batten` eliminates these issues by compiling the compiler's own symbol table into physical token-level sampling masks.

---

## 2. Target Personas & User Journeys

### Persona A: Neurosymbolic Code Synthesizer (`joiner`)
- **Needs:** Dynamic token masks that constrain model inference to available variables and valid return types at specific AST hole locations.
- **Workflow:** Passes `miter` scope and `dado` hole to `batten_compile`, applies the resulting GBNF/Lark mask to `sawhorse`, and fills the hole with 100% syntactically valid tokens.

### Persona B: Autonomous AI Coding Agent
- **Needs:** Instant validation of generated code candidates before submitting patches for full verification.
- **Workflow:** Calls `batten_validate` to verify token membership and bracket balancing in `<50μs`, immediately rejecting bad completions without waiting for `cargo check`.

### Persona C: Local Inference Runtime Maintainer
- **Needs:** Compact, valid grammar constraints compatible with `llama.cpp`, `mistral.rs`, `vLLM`, and `outlines`.
- **Workflow:** Integrates `batten` to produce dialect-compliant grammar files without maintaining custom grammar converters.

---

## 3. Jobs to Be Done (JTBD)

| When I... | I want to... | So that I can... |
|---|---|---|
| Ask an LLM to generate an expression for a typed hole | Restrict token logits to in-scope variables and functions | Guarantee that the model cannot hallucinate out-of-scope identifiers or invalid syntax. |
| Run an automated code synthesis repair loop | Reject candidate fills with mismatched tokens in `<50μs` | Avoid wasting 2–5 seconds on compiler subprocesses for obviously invalid code. |
| Use different inference engines (llama.cpp vs vLLM) | Compile scope constraints to either GBNF or Lark format | Use the exact same grammar logic across local and cloud inference runtimes. |
| Sample code from smaller open-weight models | Limit grammar recursion depth strictly to $\le 3$ hops | Prevent repetitive looping traps and ensure prompt completions terminate predictably. |

---

## 4. Key Metrics & Success Criteria

* **In-Scope Precision:** `100%` of identifiers generated under a `batten` grammar are members of the lexical scope.
* **Pre-Flight Validation Latency:** `<50μs` per candidate evaluation.
* **Grammar Compilation Time:** `<5ms` to compile a 100-symbol lexical scope into a multi-rule GBNF grammar.
* **Zero Runaway Recursion:** `0%` infinite token loops under depth-bounded grammar constraints.
