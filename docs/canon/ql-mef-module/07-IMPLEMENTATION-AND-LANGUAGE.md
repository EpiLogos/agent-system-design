# Part XII — Determinism and semantic inference

## 52. Deterministic core

Operations which are mathematically/formally determined should be reproducible for identical inputs and kernel versions.

The kernel must make clear which operations satisfy this condition.

Examples likely include address validation, supported form/lens lookup, relation-code validation, formally specified direct/conjugate mapping, and deterministic fixture operators.

---

## 53. Semantic operations

Semantic refraction may use model inference or context-sensitive interpretation.

Such results must include provider/model/version, input refs/revisions, relevant lens/form refs, confidence/status where meaningful, evidence references, and explicit non-deterministic provenance.

A semantic reading is not silently cached as eternal truth.

---

# Part XIII — Package/service shape

## 54. Recommended implementation decomposition

The design can be implemented in one repository/package family while preserving internal modules such as:

```text
ql-core
    canonical software types + deterministic kernel operations

ql-mef
    lens registry + sublens metadata + refraction contracts

ql-semantic
    semantic provider / synthesis machinery

ql-service
    CLI / JSON / local RPC or service host

ql-adapters
    AIKit / Factory / experimental runtime adapters

ql-fixtures
    deterministic providers, parity fixtures, conformance corpora
```

These are architectural modules, not a requirement for six separate deployable packages.

---

## 55. Transport independence

The semantic service must not depend on one transport.

Supported presentations may include in-process library, CLI with JSON, local RPC/service host, HTTP where needed, tool façade, or embedded provider inside an experimental runtime.

One transport must not become the QL ontology.

---

## 56. CLI direction

A future CLI may expose operations such as:

```text
ql capabilities
ql locate <ref>
ql refract <ref> --lens L4
ql relate <ref-a> <ref-b> --lens L1
ql synthesise <reading...>
```

with structured JSON equivalents.

CLI syntax is not yet canonical. The operation family is.

---

# Part XIV — Claims, evidence, and promotion

## 57. Readings as epistemic returns

Where integrated with Factory, QL readings should normally enter as derived evidence/Claims with provenance rather than mutate canonical state directly.

A semantic provider can say:

```text
"Under L1, these factors appear to constitute the execution."
```

Factory can then retain the reading, compare it with other evidence, assess it, and promote/reject it through normal Claim/Decision mechanisms.

The module does not bypass epistemic discipline because its output is labelled QL.

---

## 58. Human review and typing corpus

For semantic mappings whose quality matters, the module should support a human-reviewed corpus of subject/frame, expected candidate address/lens reading, acceptable alternatives, forbidden collapse, and rationale/evidence.

This can support agreement metrics and provider regression without pretending all interpretive mappings are mechanically decidable.

---

# Part XV — Agentic self-understanding and language

## 59. Language as an operative surface

QL/MEF gives a deep basis for treating language exposed to an agent as part of its operating architecture.

Square A makes relevant:

- how questions establish a field;
- how definitions create distinctions;
- how descriptions disclose powers;
- how over-articulation can constrict latitude;
- how missing language can be open, latent, unknown, bound, or simply defective;
- how work/persistent marks condition subsequent ground.

The module can support analysis and Skills which improve internal communication without making every sentence a kernel operation.

---

## 60. Pre-skill grounding

QL/MEF stands logically prior to a Skill catalogue.

A Skill asks:

> How do I undertake this meaningful kind of activity?

QL/MEF can ask:

> What kind of whole, distinction, cause, contradiction, process, history, encounter, or articulation is present here?

Therefore the module can inform Skill authoring, Skill discovery, Agent orientation, Context retrieval, Claim/Evidence analysis, model/Agency attunement, and reflection/growth without becoming twelve Skills or twelve workflow stages.

---
