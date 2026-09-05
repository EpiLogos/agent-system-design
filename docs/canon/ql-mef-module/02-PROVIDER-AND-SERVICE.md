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

# Part V — Stable initial service surface

## 17. Core operations

The first stable service boundary should remain deliberately small:

```text
capabilities()
locate(target, frame?)
refract(target, lens, frame?)
relate(a, b, frame?, lenses?)
synthesise(readings, frame?)
```

These are semantic operation names, not commitments to a specific transport.

A library, CLI, local service, tool surface, or embedded provider may expose them differently while preserving the contract.

---

## 18. `locate`

Purpose:

> identify a target's QL locus or relevant formal frame without renaming the target.

Possible outputs include:

- one or more candidate QL addresses;
- a frame relation;
- ambiguity;
- insufficient information;
- unsupported mapping.

A semantic locator should make uncertainty visible rather than forcing every object into one address.

---

## 19. `refract`

Purpose:

> read a target through one specified MEF lens.

The output should explain what becomes visible through that lens while retaining subject identity and source/evidence references.

`refract` does not mutate the target.

A caller may request several lenses independently and later synthesise them.

---

## 20. `relate`

Purpose:

> read the relation between two or more existing subjects under an explicit QL frame and/or lens set.

Examples include:

- causal relation;
- logical tension;
- process/historical relation;
- conjugate/complement relation when supported;
- relation between Agent/Agency/Context objects.

The operation returns a relation reading, not a new canonical edge unless a client explicitly promotes it through its own Claims/Decision machinery.

---

## 21. `synthesise`

Purpose:

> integrate several explicit readings into a whole-facing account while retaining disagreement, incompleteness, and provenance.

Synthesis is not concatenation.

It should surface:

```text
common structure
complementary disclosures
tensions
unresolved questions
retained differences
possible next inquiry
```

when supported by the provider.

---
