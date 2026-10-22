# batten

> Dynamic scope-to-Context-Free Grammar (GBNF/Lark/Regex) compiler and token mask generator for neurosymbolic code synthesis.

Part of the **`gaugeblock`** ecosystem ([gaugeblock.dev](https://gaugeblock.dev)) alongside `miter`, `dado`, `sawhorse`, and `storypole`.

---

## Capabilities

* **Two-Tier Type & Scope Filtering:** Classifies in-scope lexical symbols into *Direct Terminals* (variables/constants directly satisfying hole types or unwrapping via `?` / `.await`) and *Producer Chains* (functions/methods producing the target type), with safe fallback widening to eliminate dead-end grammars.
* **AST-Driven Grammar Productions:** Emits grammar rules tailored to `ast_node_kind` (`expression`, `predicate`, `identifier`, `type_annotation`).
* **Bounded Recursion Depth:** Explicit configurable depth rules (`max_depth = 3`) to prevent infinite token loops in small local models.
* **Tri-Format Grammar Emission:** Compiles into llama.cpp/mistral.rs **GBNF**, llguidance/vLLM/outlines **Lark**, and bounded **Regex**.
* **Zero-LLM Pre-Flight Validator:** Microsecond-fast validator checking 100% identifier membership, delimiter balancing, and literal safety rules.
* **Model Context Protocol (MCP) Server:** Native JSON-RPC 2.0 stdio server exposing `batten_compile` and `batten_validate`.

---

## CLI Usage

```bash
# Compile scope and hole into a GBNF grammar constraint
batten compile --scope scope.json --hole hole.json --format gbnf

# Compile into Lark EBNF format for llguidance
batten compile --scope scope.json --hole hole.json --format lark

# Pre-flight validate candidate tokens against constraint
batten validate --constraint constraint.json --candidate "find_user(req)?"

# Start Model Context Protocol stdio server
batten mcp
```

---

## License

Apache-2.0
