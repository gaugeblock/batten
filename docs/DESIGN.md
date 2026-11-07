# Architectural Design & Decision Records — `batten`

> **Crates:** `batten`, `batten-core`, `batten-cli`, `batten-mcp`  
> **Status:** Active Architectural Decision Records (ADRs)

---

## Architectural Decision Records (ADRs)

### ADR-001: Context-Free Grammar Masking Over Stochastic Prompt Engineering
- **Status:** Accepted
- **Context:** Telling an LLM "only use in-scope variables" in a natural language prompt fails 40%+ of the time. Autoregressive sampling stochastically samples out-of-scope identifiers, invalid keywords, and mismatched syntax.
- **Decision:** Constrain model sampling at the token-logit level using formal Context-Free Grammars (GBNF/Lark) compiled dynamically from lexical scope.
- **Consequences:** The model is physically incapable of emitting invalid identifiers or illegal syntax during hole filling, eliminating entire classes of hallucinations.

### ADR-002: Two-Tier Scope Partitioning (Direct Terminals vs Producers)
- **Status:** Accepted
- **Context:** Including all in-scope functions, methods, and variables into a single recursive grammar rule causes combinatorial grammar explosion, slowing down logit-mask generation in local runtimes.
- **Decision:** Partition scope into Direct Terminals (Tier 1: immediate variable or unwrapped field matches) and Producer Chains (Tier 2: callable functions returning the target type). Give Tier 1 higher sampling priority and limit Tier 2 argument recursions.
- **Consequences:** Compact, efficient grammar rules with fast generation times and high token accuracy.

### ADR-003: Tri-Format Grammar Generation from a Neutral Intermediate AST
- **Status:** Accepted
- **Context:** Different local and cloud inference engines require divergent grammar specifications: `llama.cpp` and `mistral.rs` require GBNF; `llguidance`, `vLLM`, and `outlines` require Lark EBNF; while other tools require PCRE Regex.
- **Decision:** Model grammar productions internally as a language-neutral AST (`GrammarProduction`) and provide separate, dedicated serialization passes for GBNF, Lark, and Regex.
- **Consequences:** Unified logic across all inference backends with zero dialect drift.

### ADR-004: Microsecond Pre-Flight Validation Before Compiler Invocations
- **Status:** Accepted
- **Context:** Running full compiler checks (`cargo check` or `tsc`) takes 2–5 seconds per candidate. If a model generates text with an obvious out-of-scope variable or unbalanced bracket, waiting for the compiler wastes precious inner-loop latency.
- **Decision:** Implement `batten validate`: a microsecond-fast validator that tokenizes candidates and validates symbol membership against `LexicalScope` in `<50μs`.
- **Consequences:** Instant rejection of malformed candidates, saving 95%+ of compiler invocation overhead during synthesis repair loops.

### ADR-005: Strict Depth Bounding (`max_depth = 3`)
- **Status:** Accepted
- **Context:** Unconstrained recursive grammar rules allow models to enter infinite loop traps (e.g. `obj.next().next().next()...`).
- **Decision:** Hard-cap rule derivation nesting at depth 3. When the derivation hits the ceiling, force termination into direct terminal variables or constants.
- **Consequences:** Guaranteed derivation termination with predictable token lengths.
