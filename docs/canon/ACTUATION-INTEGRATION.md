# Software Factory ↔ Actuation Integration

**Status:** canonical ownership-boundary amendment  
**Actuation repository:** `EpiLogos/Actuation`

## 1. Ownership correction

Software Factory does not own generic first-class agent runtime semantics.

The QL Agent Runtime programme was developed here because Factory was a useful proving ground. That experiment has now exposed a more general seam: an agent/runtime can be varied and studied independently of the developmental Project/Run semantics that happen to employ it.

The canonical developmental/reference home for that seam is now **Actuation**.

Actuation owns the generic subject of:

- world-bound first-class agency;
- metagency — agency acting on the distribution and constitution of agency in its world;
- `AgenticComposition` and its determination/lineage/interrelation/bounds/return grammar;
- harness-neutral Loop Runtime and recurrence experiments;
- QL Agent Runtime experimental bodies and their comparative evidence.

Factory remains a consumer and proving client of those capabilities.

## 2. What Factory continues to own

Factory continues to own developmental semantics and evidence:

```text
Project / ProjectBinding
Run / Run Map
semantic acts and frontier
Claim / Evidence / Decision / Artifact
Candidate / comparison / selection
repair / return / synthesis
Execution Intelligence
ExecutionDisposition
Factory-specific developmental policies
```

`ExecutionDisposition` remains the decision about **how a developmental semantic act should be enacted**. It may request one actor, sequential actors, parallel independent work, candidate fan-out, multiple assessment angles, barriers, nested work, repair, or synthesis.

That does not make the resulting semantic plurality a Factory-owned Agent ontology.

## 3. Actuation seam

When Factory determines that developmental work requires one or more first-class agentic loci, the semantic order is:

```text
Factory Project / Run / semantic need
              │
              ▼
      Execution Intelligence
              │
              │ requires agentic plurality
              ▼
          Actuation
  AgenticComposition(g,W,I,L,D,Γ,B,R)
              │
              ▼
      ExecutionDisposition
       enacts the demand
              │
              ▼
            AIKit
   body/session/surface resolution
              │
              ▼
          Workcell
      material execution
              │
              ▼
    evidence / attributed Return
              │
              └──────────► Factory Run / world
```

The exact API direction may be mediated through the shared interop contract; this document establishes responsibility, not transport.

## 4. Distinctions that MUST survive

### AgenticComposition ≠ ExecutionDisposition

`AgenticComposition` identifies the semantic plurality: its governing locus, participating Agents/Agencies, lineage/determination, relation graph, authority/world bounds, and return obligations.

`ExecutionDisposition` chooses an execution shape for a developmental act. A different execution strategy may enact the same semantic composition if it preserves those declared relations and bounds.

### AgenticComposition ≠ HarnessComposition

Actuation actor plurality is distinct from AIKit operational body composition. One semantic locus can have a complex HarnessComposition; a four-locus AgenticComposition can resolve four separate bodies.

### AgenticComposition ≠ Workcell topology

Material process/service/network placement does not define semantic Agent identity or the composition's meaning.

### AgenticComposition ≠ SharedField

O:I participation/co-internality and internal agentic determination are separate relations. An independently grounded Other can enter a composition by federation without becoming a derived worker identity.

## 5. Existing Agent semantics

Factory's proven `Agent` / `Agency` / `AgentSession` / `Execution` distinctions remain valid shared foundations while ownership is harmonised across the suite:

- `Agent` — enduring semantic identity;
- `Agency` — situated determination of an Agent;
- `AgentSession` — replaceable runtime/session continuity;
- `Execution` — one concrete act.

Actuation MUST consume those meanings compatibly rather than fork them. A later shared-schema amendment may move canonical ownership without semantic drift.

## 6. QL profile

Factory retains the Epi-Logos developmental profile in which the 0/1 reader-composer coordinates six canonical Agent identities. Those are legitimate first-class Agents used by Factory; they are not the generic definition of actuation.

Generic Actuation is number-neutral and N-agent. QL-MEF may refract the relation through bimba/pratibimba, conjugation, 0/1, 1/0, 4+2, MEF lenses, or other formal structures. Factory MAY consume those projections without requiring them for baseline correctness.

## 7. QL Agent Runtime graduation

Historical source programme:

```text
Factory issue      #94
Factory Deep PR    #130
Deep source head   a654c62f68b82236061986d9215b23257fe53b17
Factory main at reconciliation
                   94c72f534cb51410865de0af138feb488b13e999
```

The current Actuation migration preserves the complete corrected Deep experiment tree while reconciling the two later `main`-only Series 1 dispatcher commits. Historical Factory issue/PR discussion remains provenance.

After the Actuation migration PR is accepted:

- new generic QL runtime development occurs in Actuation;
- Factory does not retain a second canonical `ql-agent-experiments/` tree;
- Factory may add only genuinely Factory-specific adapters/fixtures around the Actuation contract;
- QL runtime capability-effect claims remain evidence-gated and are not inferred from structural convergence.

At migration there are **zero claimed live Series 1 capability runs**.

## 8. Repository relocation

The Factory ownership-cleanup branch removes:

```text
ql-agent-experiments/
.github/workflows/ql-runtime-foundation.yml
.github/workflows/ql-experiment-native.yml
.github/workflows/ql-experiment-pi.yml
.github/workflows/ql-experiment-pydantic.yml
.github/workflows/ql-series1-live.yml
```

Their canonical successors live under Actuation's:

```text
experiments/ql-runtime/
.github/workflows/ql-*.yml
```

This is an ownership move, not deletion of the research record: Factory Git history, issue #94, and PR #130 retain the full developmental provenance.
