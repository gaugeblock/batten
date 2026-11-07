# Product Roadmap & Strategic Horizons — `batten`

> **Active Suite Lifecycle:** `0.1.0-beta.X`  
> **Target:** Dynamic Scope-to-CFG Grammar Compiler & Token Mask Engine

---

## 1. Active Milestone: `v0.1.0-beta.3` (Current Focus)

- [x] **Two-Tier Scope Filtering:** Categorization of lexical symbols into Direct Terminals and Producer Chains.
- [x] **Tri-Format Grammar Emission:** GBNF (llama.cpp), Lark EBNF (llguidance/vLLM), and bounded Regex.
- [x] **Strict Depth Bounding:** Hard-cap rule derivation nesting at depth $\le 3$ to prevent infinite loops.
- [x] **Microsecond Pre-Flight Validator:** `<50μs` token membership and delimiter balancing check.
- [x] **Model Context Protocol (MCP):** JSON-RPC 2.0 stdio server exposing `batten_compile` and `batten_validate`.
- [x] **9-Document Architecture Compliance:** Certified clean under `chalkline docs check`.

---

## 2. Horizon 1: Tokenizer Integration & Direct Mask Generation (`v0.2.0`)

- [ ] **Direct Tokenizer Integration:** Compile GBNF rules directly into bitmask arrays matching target model vocabularies (Llama-3, Qwen-2.5, DeepSeek).
- [ ] **llguidance Direct Bridge:** High-performance FFI bridge to `llguidance` for zero-overhead token mask step evaluation.
- [ ] **Python Scope Support:** Grammar production rules specialized for Python lexical scopes and type annotations.
- [ ] **Grammar Inspection TUI:** Terminal visualization of generated grammar production trees and terminal transitions.

---

## 3. Horizon 2: Telemetry-Weighted Grammars & Live Caching (`v0.3.0`)

- [ ] **Probabilistic Grammar Weights:** Weight candidate terminal derivations based on verified success telemetry from `rabbet`.
- [ ] **Logit Mask LRU Cache:** Shared-memory cache of pre-computed token masks for common architectural patterns.
- [ ] **Dynamic Hole Expression Evaluator:** Infer multi-argument method call patterns from `miter` call graphs.

---

## 4. Horizon 3: Zero-Copy Synthesis Fabric (`v1.0.0`)

- [ ] **Zero-Copy In-Memory IPC:** Shared-memory ring buffer connecting `batten` token masks directly to `sawhorse` and `joiner`.
- [ ] **Enterprise Grammar Catalog:** Centralized management of proprietary DSL grammar rules and governance constraints.
- [ ] **Long-Term Wire Stability:** Freeze grammar AST intermediate schema for multi-year backward compatibility.
