# Structural Source Fidelity

**Status:** proposed Factory canon addition, derived from returned implementation evidence  
**Scope:** development targets that already possess an authored/canonical structural ontology, coordinate map, relation map, schema graph, state machine, protocol algebra, or equivalent relational specification

## Why this exists

A software-development Agent can implement the requested user-facing feature correctly and still deform the system it is extending.

The failure mode appears when the target already has a meaningful structural map, but development begins by decomposing the requested feature into locally convenient types, routes, components or services. Those local structures then become the de facto ontology of the new implementation. Coordinates that have no current feature disappear. Recursive structure is flattened. Relations become incidental call paths. A later developer can no longer tell where a new capability belongs except by reverse-engineering the code created by the previous tranche.

That is not ordinary incompleteness. It is **structural source loss**.

The Epi-Logos Nara/M4′ work exposed the problem concretely. The first daily journal vertical was useful and correctly protected, but the new bridge initially represented Nara mainly at its M4′ root plus the daily episode. The authored Bimba source was more specific: M4-0…M4-5, recursive #4.4.4.4 Personal Pratibimba, and explicit evidence/review/return relations. The vertical was therefore functionally right but structurally under-rooted.

The correction was to make the source map executable beneath the feature, not to turn the map into visible UI.

This Factory law generalises that lesson.

---

## 1. The law

When a development target already has an accepted relational specification, **Factory must preserve that structural ground before feature decomposition is allowed to become architectural authority**.

```text
human/authored structural source
        ↓
accepted coordinate / entity / relation ground
        ↓
executable or otherwise testable structural representation
        ↓
current implementation mapped onto that ground
        ↓
feature / vertical / candidate implementation
        ↓
acceptance against both behavior and structural fidelity
```

A feature is not permitted to become the new semantic root merely because it is the first thing implemented in a fresh runtime.

---

## 2. Applicability

This gate is **conditional**, not universal bureaucracy.

It applies when the Commission/target world contains a source which is intended to determine relational shape, for example:

- a canonical coordinate or ontology map;
- a protocol/state-machine specification;
- an authored domain tree whose parentage is semantically meaningful;
- a schema/relation graph with stable identity;
- an algebra or formal model which the software claims to reflect;
- a migration target whose native ownership/identity must survive a new host;
- another explicitly authoritative structural source.

It does not require every ordinary CRUD application or unconstrained greenfield feature to invent a formal ontology.

The first question is therefore:

> **Does this target already say what its parts are and how they relate in a way the implementation is meant to preserve?**

If yes, the answer becomes development ground.

---

## 3. Structural ground is different from feature scope

A tranche may implement only one narrow capability while preserving a much wider structural space.

These statements are compatible:

```text
coordinate exists
capability not implemented
provider unavailable
no UI surface yet
```

Factory must not infer:

```text
not implemented
    therefore
not part of the system
```

This distinction lets development remain incremental without making each increment ontologically destructive.

---

## 4. Source truth, implementation truth and experienced truth

Factory should retain three separate questions.

### Source truth

What structural distinctions and relations does the accepted authored/domain source establish?

### Implementation truth

Which of those structures have a real current materialisation, binding, provider, test or operational effect?

### Experienced truth

What does the human or Agent actually encounter in the present instrument/Surface?

A source coordinate may be real without an implementation. An implementation may be real without deserving a prominent UI. A good Surface can remain simple precisely because the deeper structure is carried underneath it.

---

## 5. Required Structural Ground record

When this gate applies, the Commission/Run should be able to name a small `StructuralGround` account before architecture-changing implementation begins.

Conceptually:

```text
StructuralGround
  source_refs
  source_revisions
  structural_manifest_refs
  relation_manifest_refs
  in_scope_identities
  constitutive_relations
  unresolved_source_gaps
  existing_implementation_bindings
  implementation_states
  return_or_mutation_law
  structural_acceptance
```

This is not necessarily a single Factory-owned file format. The target product remains owner of its domain manifests. Factory needs sufficient stable references and acceptance criteria to prevent its own decomposition from replacing them.

---

## 6. Development sequence

When Structural Source Fidelity applies, the correct sequence is:

```text
PERCEIVE requested capability / discrepancy
        ↓
GROUND authored purpose and current target reality
        ↓
LOCATE canonical structural source(s)
        ↓
INVENTORY identities / coordinates / constitutive relations in scope
        ↓
BIND or verify target-owned executable/testable structural manifests
        ↓
MAP current code/providers onto that structure
        ↓
DECOMPOSE the requested feature within those identities
        ↓
BUILD
        ↓
PROVE behavioral acceptance
        +
PROVE structural fidelity acceptance
        ↓
RETURN discrepancies to the source/architecture explicitly
```

The key ordering constraint is `BIND/MAP before DECOMPOSE` when decomposition could otherwise redefine the target ontology.

---

## 7. Anti-flattening invariants

A Candidate should fail structural recognition when it does any of the following without an explicit accepted source revision:

- deletes or makes unreachable an in-scope canonical identity merely because no feature implements it;
- replaces stable source identity with component, route, database-row or provider identity;
- flattens semantically meaningful recursion into sibling features;
- changes parentage because the implementation hierarchy is more convenient;
- turns typed constitutive relations into untyped incidental edges when relation kind affects behavior;
- bypasses an authored review/return/mutation relation with a direct write;
- treats provider reachability as authority over the represented domain;
- claims source parity from naming similarity alone;
- reconstructs missing source structure from current code and silently calls the result canonical.

---

## 8. Parity / fidelity claims

For structurally mapped targets, Recognition should ask which claim is actually supported:

1. **Source fidelity** — the accepted structural source/revision is known.
2. **Identity/coordinate fidelity** — in-scope identities preserve canonical identity, path/parentage and source provenance.
3. **Relation fidelity** — constitutive relations are represented or explicitly gap-recorded.
4. **Operational fidelity** — software actually enacts the structural relation it claims.
5. **Experiential fidelity** — the resulting human/Agent experience has the right consequence without forcing architecture into foreground UI.

A Candidate should not report undifferentiated "parity" when only one or two of these have been proven.

---

## 9. Missing or contradictory structural source

Factory must not solve a missing canonical source by making the implementation authoritative by default.

If an expected structural corpus is missing, stale, contradictory or only partially recoverable:

```text
record the source gap
    ↓
implement only the source-proven structural floor needed by the tranche
    ↓
mark unresolved structure explicitly
    ↓
open source-recovery / supersession work
    ↓
do not claim full structural parity
```

Returned implementation reality may legitimately reveal that the authored map needs revision. That revision should happen explicitly at the source/architecture level, after which implementation can follow the new accepted structure.

---

## 10. Native ownership

Factory does not become owner of the target ontology by enforcing this gate.

The target product/domain owns its structural identities and manifests. Factory owns the developmental obligation to discover, preserve, test and return against them.

This preserves the broader O:I rule that generic development/infrastructure products should strengthen native ownership rather than absorbing it.

---

## 11. Agent prompt / Run requirement

When the gate applies, a development Agent should be able to answer before significant architecture work:

1. What is the accepted structural source?
2. What revision/provenance am I working against?
3. Which identities/coordinates are in scope?
4. Which relations are constitutive rather than incidental?
5. What current implementation already materialises them?
6. Which parts are unimplemented, unavailable, research-only or unresolved?
7. Where does this requested vertical actually root in the source structure?
8. What governs return/mutation across that structure?
9. Which tests/evidence will catch structural flattening?

If the Agent cannot answer because the source is unavailable, that absence is a real development finding, not permission to invent the missing ground.

---

## 12. First proving case: Epi-Logos Nara

The first concrete application is tracked in:

- Epi-Logos-C-Experiments #12 / PR #14;
- QL-MEF #62 / #63 / #64 / PR #65;
- O:I PR #90.

Nara demonstrates why the rule matters:

```text
correct small vertical
!=
complete structural grounding
```

The corrected implementation can keep one quiet daily journal Surface while its artifacts are precisely rooted in the wider M4/M4′ map. That is the desired relationship: **whole structural intelligibility underneath smallest sufficient experience**.

---

## 13. Recognition criterion

For an applicable target, a Candidate is not ready for structural Recognition merely because its feature tests pass.

Recognition must be able to say either:

```text
structural fidelity preserved for the declared source scope
```

or:

```text
structural discrepancy returned explicitly for human/source-level decision
```

It must not silently let the current implementation decide what the source was supposed to mean.
