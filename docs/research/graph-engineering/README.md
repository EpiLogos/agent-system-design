# Graph Engineering Research

**Status:** `RESEARCH / COMPARATIVE CLAIM-SET`.

This directory contains evidence and extracted claims from the AI LABS graph-engineering video and related practice. It is deliberately separated from the canonical Software Factory corpus.

The original extraction previously lived beside the core specification files under `seed-docs/QL-SOFTWARE-FACTORY-GRAPH-ENGINEERING-PRINCIPLES.md`. That placement overstated its determination status. The source is useful because it provides analogous practice around parallel agent execution, verification, fresh-context review, fan-in/fan-out and skill composition, but it does not define Factory semantics.

Claims from this research become design only where explicitly integrated into:

- the Constitutional/Architecture/Primitive corpus;
- a harmonised Wayfinder determination;
- the Root Build Programme or a clearly marked programme amendment.

The current integrated consequence is captured in `docs/canon/EXECUTION-INTELLIGENCE-INTEGRATION.md`.

## Strongest useful claims

1. **Execution shape should follow real dependency structure.** Independent work may fan out; genuine dependencies remain sequential.
2. **Parallel work needs explicit convergence.** Barriers and synthesis should be structural rather than hoped for.
3. **Verification is a dependency boundary.** Downstream work should not build silently on failed or absent evidence.
4. **Independent assessment benefits from genuinely independent context.** In Factory terms this is expressed against producing Execution/AgentSession lineage.
5. **Judgement quality is graph-wide.** Model/Agency selection for assessment should satisfy a stipulated judgement-fitness requirement rather than be down-tiered merely to save cost.
6. **Skills can compose execution competence.** Runtime-resolved skills may supply methods, standards and workflow detail without being copied into the Run Map.
7. **Fan-in must compress, not concatenate.** Synthesis is an act with its own output contract.
8. **Parallel execution magnifies observability requirements.** Every execution leg and handoff must remain attributable to the canonical Run.
9. **Total graph cost matters.** Fan-out width is a Run/runtime resolution concern, informed by budget and measured use rather than per-node price alone.

## Factory-specific corrections to the raw extraction

- The canonical Run Map is the whole developmental topology, **not simply an agent execution DAG**.
- A runtime graph is an enactment/elaboration of executable Run Map regions and remains attributable to the same Run.
- Not every Run Map node means “one job, one context, one report.” Decisions, Candidates, Recognition, source gates and other semantic nodes retain their own meanings.
- `standalone / embedded / orchestrator` are useful workflow participation patterns for some skills, not the exhaustive ontology of Capability.
- “Always use the strongest model” is retained as the underlying warning **not to economise below required judgement fitness**; actual model/Agency selection uses measured use-fitness and independence.
- “Every node needs independent review” is too broad as a Factory invariant. Independent assessment applies where the gate/quality contract requires it.
- One-angle-per-skill is a useful discipline for evaluative clarity, not an absolute rule against every multi-purpose capability.

The raw transcript and original extraction remain source evidence. They should be cited and re-audited when a later architectural decision depends on them.
