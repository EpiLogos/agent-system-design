# Execution Intelligence Integration — Canonical Design Amendment

**Status:** `CURRENT DESIGN — cross-map integration amendment`  
**Date:** 2026-08-15  
**Authority:** subordinate to `QL-SOFTWARE-FACTORY-CONSTITUTIONAL-INDEX.md`; integrates and refines the existing Execution Intelligence family in Architecture, Primitive Relations and the three harmonised Wayfinders. It does not supersede constitutional determinations.

## 1. Determination

Execution Intelligence is already a constitutional family crossing the six positions. This amendment makes explicit the seam that turns a meaningful developmental act into situated agentic execution.

```text
Run Map frontier / WorkNode / ActionInvocation / Assessment demand
 + governing Claims / Decisions / completion contract
 + Context = Operative World + Information Horizon + Focus
 + available Agents / Agencies
 + AIKit-resolved skills / Capabilities / Actions / sources
 + model / harness / optional effective HarnessComposition / SessionSpace / Workcell availability
 + learned fitness / policy / user constraints
        ↓
EXECUTION INTELLIGENCE
        ↓
ExecutionDisposition
        ↓
Execution(s)
        ↓
Artifacts + Claims + Evidence + Assessments + Events
        ↓
canonical Run Map advance / gate / return / Candidate / Decision
        ↓
P5 learning → future Ground / Context / execution resolution
```

The agent is not a legacy workflow step. It receives a meaningful task, the relevant operative world, runtime competence, and a suitable intelligence/body. The Factory and AIKit resolve and enforce the boundaries around the act without pre-scripting the agent's internal reasoning.

Some Harnesses are comparatively fixed. Others expose an internally composable runtime body. Where AIKit can resolve such a body, Factory needs only enough opaque `HarnessComposition` provenance to explain which effective embodiment carried an Execution. Component, Contract, Surface, activation-scope and target-native plugin semantics remain outside Factory ownership.

## 2. ExecutionDisposition

`ExecutionDisposition` is the inspectable near-primitive resolution product answering:

> Given this semantic act in this Run state, how is it being constituted for execution now?

It is analogous in role to `ContextResolution`: inspectable, attributable resolution state rather than a replacement for the product primitive it serves.

```ts
type ExecutionDisposition = {
  subject: Ref
  task: string

  context: ContextRef
  context_resolution?: ContextResolutionRef

  agent: AgentRef
  agency: AgencyRef

  capabilities: CapabilityRef[]
  actions: ActionRef[]

  model: ModelRef
  harness: HarnessRef
  harness_composition?: Ref
  session_space?: SessionSpaceRef
  execution_demand?: ExecutionDemandRef

  shape: ExecutionShape
  independence_from?: ExecutionRef[]
  requires: Ref[]
  converges_to?: Ref[]
  gate_contract?: Ref

  rationale: EvidenceRef[]
}

type ExecutionShape =
  | 'single'
  | 'sequential'
  | 'parallel-independent'
  | 'candidate-fanout'
  | 'multi-angle-assessment'
  | 'barrier'
  | 'synthesis'
  | 'return-repair'
  | 'nested-run'
```

The exact serialized schema remains an implementation contract to settle through the root ticket. `harness_composition` is intentionally opaque at the Factory boundary: its source/revision/fingerprint/provenance may be carried through the common Ref floor, while AIKit/target adapters own its internal component graph. A fixed/thin harness may omit it entirely.

The semantic relation is fixed here.

`ExecutionDisposition` is **not**:

- the WorkNode or Run Map node itself;
- Agent identity;
- Agency;
- Context;
- a CapabilitySet or skill body;
- an Action or ActionInvocation;
- a HarnessComposition or Component graph;
- an ExecutionDemand;
- an Execution;
- a provider prescription;
- a chain-of-thought plan.

The same semantic task may be re-resolved after host/session/provider/model/body availability change without changing Run, WorkNode, Agent or Candidate identity. A body-composition change that materially affects an Execution must remain attributable in ExecutionDisposition/Execution provenance; it does not create a new semantic Agent or Run merely by occurring.

## 3. Run Map walking

The Run Map remains the single canonical developmental topology of a Run. It contains more than execution: Decisions, Claims, Candidates, Recognition, returns, source gates, research routes, prototypes and recursion all belong to the developmental graph.

Execution Intelligence **walks executable regions** of that map.

A WorkNode/frontier need not encode a full runtime graph. It should carry enough semantic information for an agent to know what the act means:

```text
purpose / desired change
relevant Intent + Design + governing Claims
real dependencies
required information/source horizon
completion / Evidence / Assessment contract
convergence or return relation where semantically fixed
human-authorship boundary where genuinely consequential
optional capability affinities or explicit required capabilities
```

From this, Execution Intelligence may resolve one act or a local execution topology. Runtime decomposition is recorded back into the Run through Execution/Event/Evidence lineage and may not become an invisible second source of truth.

### Execution-shape grammar

The graph-engineering research adds a useful repertoire to the existing `execution shape` term:

- **single** — one coherent act;
- **sequential** — a real dependency requires an earlier result;
- **parallel-independent** — independent work may proceed concurrently;
- **candidate fan-out** — deliberately produce multiple coherent possible realities;
- **multi-angle assessment** — one subject is independently read under distinct evaluative criteria;
- **barrier** — downstream work requires a stipulated set of evidence-bearing legs;
- **synthesis** — multiple reports/results must become one actionable surface rather than concatenated noise;
- **return/repair** — evidence returns the Run to an earlier developmental concern;
- **nested Run** — the discovered transformation is substantial enough to acquire its own durable Run identity.

Shape follows the actual dependency/developmental relation. Parallelism is not intrinsically better, and seriality is not assumed merely because a workflow was written linearly.

## 4. Skills and AIKit

AIKit remains the operational resolver/index, not the owner of Project or Run meaning.

Skills are resolved `Capability` resources. At runtime they can contribute:

- task-specific method;
- source-inspection procedure;
- standards and references;
- workflow conventions;
- tool usage patterns;
- output/evidence expectations;
- domain competence.

Their content normally belongs in the agent's operative Context, **not copied into the Run Map or every GitHub ticket**. The Run Map tells the agent what this act means in this Project/Run; skills help the agent perform that kind of act well.

Capability composition may include standalone, embedded/gating, or orchestrating usage patterns where useful, but these are workflow participation modes, not an exhaustive ontology of all skills/Capabilities.

For composition-capable harnesses, AIKit may additionally resolve the effective runtime body through which those powers become active. Factory consumes only the resulting body/provenance relation needed for execution attribution; it does not acquire AIKit's Component/Contract/Surface composition model.

## 5. Run-Map Builder capability

Constructing or materially revising a Run Map is itself agentic work.

A Run-Map Builder capability should provide an appropriate agent with the method to:

- recover Project Ground before asking the human;
- read Intent and Design as authored constraints/horizons;
- inspect relevant source reality;
- identify developmental units and real dependencies;
- preserve implementation latitude for reversible engineering judgement;
- expose genuine unresolved authorial Decisions rather than routine uncertainty;
- attach source, evidence and gate requirements;
- identify useful parallelism, convergence, prototypes, experiments and nested Runs;
- preserve one canonical Run Map and projection identity.

The builder can itself use Execution Intelligence. A small Run may require one agent pass. A large or source-heavy Run may resolve parallel inspections, prototype work, synthesis and fresh assessment. These are chosen from the actual Project state rather than stipulated as mandatory ceremony.

Factory Run commands own canonical Run Map mutation. AIKit resolves the capabilities/resources used by the builder.

## 6. Independent assessment

Independent assessment is expressed against the **producing Execution / AgentSession lineage**, not merely by choosing a different model name.

Where a gate requires an independent judge:

- the assessor receives the governing requirement/Claim and relevant artifact/evidence;
- it does not inherit the builder's conversational state merely for convenience;
- the resulting Assessment is attributable to its Agent/Agency/Execution;
- model/Agency selection must satisfy the required judgement use-fitness and independence;
- economy may constrain selection but may not silently down-tier below the stipulated judgement requirement.

A builder's deterministic tests are still Evidence. They simply do not satisfy an explicitly independent Assessment contract by themselves.

## 7. Information → agent → action

This is the most delicate seam and should remain explicit:

```text
Project information horizon
    available ≠ retrieved ≠ loaded
        ↓
ContextResolution / source retrieval
        ↓
Focus for this semantic act
        ↓
skills / Capabilities supply runtime competence
        ↓
Agency situates the enduring Agent
        ↓
model + Harness + optional HarnessComposition + material world give the act a body
        ↓
Execution
        ↓
ActionInvocation(s) / tool use / reasoning / source change
        ↓
attributable artifacts + epistemic returns
```

The selector should not stuff every available document or skill into the prompt. It should resolve the smallest sufficient operative world while preserving addressability to the wider Information Horizon.

The same principle applies to runtime embodiment: a richly composable harness does not imply every available Component is mounted. AIKit may resolve a body appropriate to this Agency/Context; Factory records which effective body carried the act when that distinction is material.

Actions remain project/application-owned domain operations. Capabilities remain broader powers. Skills may help an agent use Actions; they do not acquire Action semantic ownership by being resolved alongside them. A target-native Component or UI Surface exposing an Action likewise does not acquire that Action's canonical meaning.

## 8. P5 / recursion

P5 interprets the trajectory as well as the final artifact.

It may derive **distinct** observations about:

- model use-fitness;
- Agency/Agent fitness for a use type;
- Capability/skill usefulness;
- context sufficiency and retrieval quality;
- harness/body-composition use-fitness where observable;
- decomposition quality;
- false or missing dependencies;
- execution-shape effectiveness;
- independent-review signal quality / false positives;
- return/rework frequency;
- cost, latency and context occupancy;
- Run-Map granularity and map-building quality.

These observations may influence future Execution Intelligence and Run-Map Builder resolution. They do not overwrite immutable Run history, collapse trust/preference/frecency/fitness into one signal, silently rewrite authored Intent, or acquire authority over AIKit/target Component meaning.

## 9. Architecture gates implied by this amendment

The implementation must be able to detect or demonstrate failure for at least:

- a WorkNode turned into a brittle internal-reasoning script;
- skill bodies copied into canonical map state as a substitute for capability resolution;
- an execution leg that cannot be attributed back to its Run/frontier;
- orphan runtime work;
- a materially different effective HarnessComposition used without execution provenance when the adapter supplies that relation;
- a target-native plugin/component ID replacing canonical Agent, Action, Capability, Project or Run identity;
- false seriality where dependency is absent when the selector is expected to exploit parallelism;
- a barrier opening without all stipulated evidence-bearing legs;
- report concatenation presented as synthesis;
- builder self-review satisfying an explicitly independent gate;
- resource/model/body choice with no reconstructable rationale;
- AIKit resolution acquiring ownership of Project, Action or Run semantics;
- P5 learning rewriting historical facts or authored intent.

## 10. QL / MEF relation

The architecture remains QL-rooted without requiring an executable QL service for ordinary operation.

Execution Intelligence is a constitutional family crossing all six positions; it is not assigned to one QL stage. MEF may later refract tasks, Context, Agents, Capabilities, ExecutionDispositions, HarnessComposition refs or Assessments where there is a real operational consequence. Graph-review angles are not automatically MEF lenses, and the existence of twelve lenses does not imply twelve review agents.

The current QL Loop Runtime experiment uses this body distinction directly: `Harness`/host mechanics remain outside the selectable `classic | ql-direct | ql-deep` recurrence seam, and a composition-capable DeepSeek Harness can carry that same seam while its body composition is held constant within matched comparisons.

This preserves the open socket for deeper QL-native/conjugate/nested orchestration while keeping present execution excellent under NoQL/FixtureQL conditions.
