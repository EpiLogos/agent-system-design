# Part IV — Provider architecture

## 15. Provider classes

The module should distinguish at least two internal capability classes even if one process exposes both.

### 15.1 Formal kernel provider

Deterministic or otherwise formally specified operations over supported QL structures.

Examples:

- validate address;
- enumerate supported forms;
- map direct/conjugate address where canonical;
- validate relation identifiers;
- apply typed formal operator;
- manage nested frame references.

### 15.2 Semantic refraction provider

Interprets independently owned subjects through MEF/QL frames.

This may involve structured rules, graph/context lookup, language models, human-reviewed mappings, or hybrid procedures.

Semantic refraction can be stochastic. Therefore it must be provenance-bearing and must not masquerade as deterministic kernel output.

---

## 16. Capability negotiation

Every provider should expose a machine-readable capability description before clients call advanced operators.

Conceptually:

```text
capabilities() -> {
    provider_id
    version
    supported_forms
    supported_lenses
    operations
    extension_namespaces
    deterministic_operations
    input_limits
    output_schema_versions
}
```

A client must not infer support for advanced operators merely because it supports basic refraction. Unknown extensions fail explicitly.

---
