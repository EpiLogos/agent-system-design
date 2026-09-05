# Part VIII — Client integration

## 41. AIKit adapter

AIKit is a principal client of the module but not its owner.

AIKit should support passive interoperability by carrying optional `QLFormRef`, `QLAddress`, `LensRef`, `QLTarget`, and `QLReadingRef` where useful.

For active provider use, AIKit can expose a QL/MEF provider as an actor-available Capability and call:

```text
capabilities
locate
refract
relate
synthesise
```

against existing AIKit/Factory refs.

QL readings must not silently override trust, availability, hard policy, Project identity, Agent identity, Action ownership, or authored preference.

If a Profile explicitly requests a lens-aware capability or QL-derived input, that requirement is visible and explainable like any other capability dependency.

---

## 42. Factory adapter

Factory may use QL/MEF over Project, Context, Run/WorkNode, Closure, Claim, Evidence, Decision, Candidate, Agent/Agency, Capability, Action, ExecutionDisposition, Assessment, Gate, and Project Evolution history.

The adapter passes Refs and structured context to the QL/MEF module and receives derived readings.

Promotion into canonical Factory Claim/Decision/Closure state is performed through normal Factory epistemic/developmental mechanisms, not implicitly by the QL provider.

A verification-oriented Run remains the same Run when refracted through QL/MEF. Its checks, Evidence, Assessments, Closure conditions, and Gates are client-owned semantics rather than MEF replacements.

---

## 43. Workcell boundary

The module does not own material execution.

A QL-aware Action or ExecutionDisposition may eventually derive material requirements, but Workcell remains responsible for provider choice, workspace, runtime/services, network/bindings, lifecycle, and the material Binding Graph.

QL semantics remain above and beside materialisation.

---

# Part IX — Agent-runtime relation

## 44. QL/MEF module versus QL Loop Runtime

The experimental `ql-agent-spec` defines a **QL-native recurrence runtime**.

The module defined here is lower-level and more general.

```text
QL / MEF MODULE
    formal refs · lenses · relation/refraction · operators
              │
              ▼
QL LOOP RUNTIME
    recurrence semantics · residue · determination · closure
              │
              ▼
AGENT HOST / HARNESS
    model I/O · tools · transport · session
```

The Loop Runtime can embed or call the kernel/provider.

The QL/MEF module can also be used by ordinary classic agents, Factory analysis, AIKit, or applications with no QL-native loop.

---

## 45. Common runtime seam

The QL agent development programme correctly standardises only:

```text
LoopRuntime
RuntimeHost
RuntimeObserver
```

while leaving QL-only state inside the QL runtime.

This module should therefore expose operations/refs which the QL runtime can consume without adding QL recurrence internals to generic host APIs.

At the wider architectural level, the shared carrier should remain reusable while Loop Logic is selectable. QL should change recurrence, determination, closure, return, and re-entry semantics rather than force duplication of the whole host/execution loop.

---

## 46. Direct Core runtime relation

The QL-native Direct Core runtime requires real semantic recurrence including Frame/P0 ground, QL acts, projection into exterior encounter, interpreted return/difference, semantic transition relation, residue, determination, positive closure, reopening, and retained re-entry.

`P5` is a candidate determination, not Closure itself. A P5 determination may disclose insufficiency and leave the same becoming open through return relations. Positive QL Closure means that the operative recurrence has become sufficiently determinate relative to what opened it; retained difference may then condition re-entry / `P0+`.

This aligns with the wider Factory clarification:

```text
Run
    actual chronological execution / observation

Closure
    sufficient semantic/processual determinacy of the opened span

Gate
    transition rule that may depend on Closure or narrower conditions
```

The kernel may provide formal validation and shared operators for these structures.

The runtime remains responsible for the loop's state and recurrence semantics.

---

## 47. Conjugation and depth

Conjugation and recursive child circuits are required by the full experimental QL profile but should enter the module through explicit operator capability negotiation.

Potential stable extension families:

```text
extension:conjugation
extension:recursive-depth
extension:runtime-trace
```

The base service must not falsely advertise them before the semantics and conformance tests are implemented.

---

# Part X — Extension architecture

## 48. Extension namespace

Research/deep-QL operators should remain namespaced until explicitly promoted.

Candidate extension families include:

```text
conjugation
recursive-depth
mask / relation-field
harmonic operators
64-state field
context-frame deepening
epogdoon / retained-difference quantification
higher topological operators
runtime typing / witnesses
```

Unknown extension identifiers return a typed unsupported result.

---

## 49. Promotion rule

An extension becomes part of the stable module only when:

1. its relation to canon is explicit;
2. its software semantics are precise enough to implement;
3. operational parity can be demonstrated;
4. fixtures and negative fixtures exist;
5. provenance/schema contracts are stable enough for clients;
6. at least one client can use it without contaminating unrelated module semantics.

Research excitement alone does not promote an operator.

---

# Part XI — No-QL modularity

## 50. Dependency firewall

AIKit, Factory, Workcell, and ordinary agent execution must remain correct with `NoQLProvider` and testable with `FixtureQLProvider`.

No live QL/MEF service may be a hidden prerequisite for Project identity, Run correctness, Agent identity, Agency existence, Capability resolution, Action invocation, Claim/Evidence storage, Recognition, Workcell materialisation, or ordinary harness operation.

---

## 51. Explicit required-QL conditions

A particular Profile, Skill, experiment, or Action MAY explicitly require a QL/MEF capability.

In that case provider absence is an ordinary explicit unsatisfied requirement.

This is different from making the whole platform depend on QL implicitly.

---
