# Root Build Programme Amendment — Execution Intelligence

**Status:** `CURRENT DESIGN — executable programme amendment`  
**Date:** 2026-08-12  
**Applies to:** `QL-SOFTWARE-FACTORY-ROOT-BUILD-PROGRAM.md`  
**Purpose:** extend the pre-ticket programme from 86 to 92 ticket-source units by making the Run Map → situated agent execution seam explicit.

This amendment is intentionally small. It does not create a new subsystem beside the existing architecture. It completes the already-defined Execution Intelligence family and gives the eventual issue/run projection enough information for agents to act without reconstructing orchestration from prose.

## A. New near-primitive / root interface

### `ExecutionDisposition`

Inspectably joins a semantic act to its situated enactment:

```text
subject task
+ Context / ContextResolution
+ Agent / Agency
+ CapabilitySet / ActionSet (including runtime-resolved skills)
+ model / Harness / SessionSpace / ExecutionDemand
+ execution shape
+ independence / convergence / gate constraints
+ resolution rationale
```

It is resolution state, not a replacement for WorkNode, Context, Agent, Action, ExecutionDemand or Execution.

### `IF-33 ExecutionDisposition`

- **Producer:** Execution Intelligence / Factory runtime, using AIKit resource resolution.
- **Consumers:** Agent runtime, Harness, Workcell, Run Map read/write surface, observability, P5 learning.
- **Lifetime:** one situated act / retry epoch; may be re-resolved without changing semantic task identity.
- **Invariant:** the agent receives task + world + competence + suitable body, not a scripted chain of cognition.

## B. Six added ticket-source units

### RC-014 — ExecutionDisposition and Execution Intelligence resolution contract

**Kind:** ROOT-CONTRACT · **Slice:** S0

Fix the inspectable semantic-act → Execution Intelligence → ExecutionDisposition → Execution seam, including task/context/capability/model/body/shape/rationale, without scripting agent cognition.

**Depends on:** RC-007, RC-008, RC-009, RC-010.  
**Produces:** IF-33.

**Acceptance:** the same semantic task can be re-resolved to different suitable runtime resources without changing WorkNode/Agent identity; rationale and independence/convergence remain inspectable.

### S1-005 — Encode execution-shape and gate semantics in Run Map

**Kind:** IMPLEMENTATION · **Slice:** S1

Make real dependency, convergence, evidence barriers, synthesis and returns expressible on the canonical Run Map while leaving runtime decomposition flexible.

**Depends on:** RC-003, RC-005, RC-014, S1-001, S1-002.

**Acceptance:** independent, sequential, barrier, synthesis and return fixtures are representable; no orphan runtime leg or partial barrier can silently count as complete.

### S3-007 — Implement Execution Intelligence selector and ExecutionDisposition

**Kind:** IMPLEMENTATION · **Slice:** S3

Resolve task + Context + Agent/Agency + skills/Capabilities/Actions + model/harness/material demand + execution shape into an explainable situated enactment.

**Depends on:** S2-002, S3-001, S3-002, S3-004, RC-014.  
**Produces:** IF-33.

**Acceptance:** resource/shape selection is provenance-bearing, source-/policy-aware and may use learned fitness without hiding a semantic Decision or confusing runtime resources with Agent identity.

### S3-008 — Build Run-Map Builder capability and agent-native execution fixture

**Kind:** IMPLEMENTATION · **Slice:** S3

Make Run-Map construction/revision a native AIKit-resolved capability composition and prove one frontier is walked from semantic task through resolved skills/context/model/body to evidence-bearing return.

**Depends on:** S1-005, S3-007, S3-003, S3-004.

**Acceptance:** the builder chooses task-appropriate decomposition/inspection/assessment depth; skill content remains runtime-resolved rather than duplicated into canonical map state; the fixture proves task → disposition → Execution → Evidence → Run Map provenance.

### S6-005 — Learn execution and map-building fitness in P5

**Kind:** IMPLEMENTATION · **Slice:** S6

Derive distinct observations about decomposition, context sufficiency, Capability usefulness, model/Agency fitness, execution shape and verification signal quality for future resolution.

**Depends on:** S6-001, S6-003, S3-008.

**Acceptance:** a later equivalent demand can resolve differently because of explicit observations while prior Run history, authored preference and Intent remain unchanged.

### AG-006 — Enforce agent-native Execution Intelligence integrity

**Kind:** ARCHITECTURE-GATE · **Slice:** S3

Detect brittle workflow scripting, copied skill bodies, invalid self-review independence, unexplained resource choice, false seriality, orphan legs, incomplete barriers and non-attributable synthesis.

**Depends on:** S1-005, S3-007, S3-008.

**Acceptance:** architecture fixtures fail each prohibited degradation and pass a task/context/skills/model/body execution fixture with attributable Evidence and P5 observation hooks.

## C. Two additional root journeys

### Journey 28 — information becomes situated agentic action

```text
Run Map frontier
→ AIKit resource/context resolution
→ ExecutionDisposition
→ agent receives task + operative Context + skills/Capabilities + suitable model/body
→ Execution
→ evidence-bearing return
→ same canonical Run Map advances or returns
```

Owned by S3-008.

### Journey 29 — the Factory learns how to execute and map better

```text
Run trajectory
→ P5 observes decomposition/context/capability/model/shape outcomes
→ distinct fitness observations
→ later Execution Intelligence / Run-Map Builder resolution changes
```

Prior Run history and authored Intent remain unchanged. Owned by S6-005 and exercised again by S10-002.

## D. Execution Intelligence field applied to every GitHub issue

Every projected issue should give an entering agent a consistent execution envelope. This is **guidance for runtime resolution**, not a hand-written workflow script.

### 1. Agent task

The issue's developmental purpose in semantic terms.

### 2. Operative context

Load:

- canonical authority/abstractions relevant to the ticket;
- dependency completion evidence;
- relevant Project/Run/frontier state;
- source-inspection evidence where the ticket names an upstream;
- the smallest sufficient information horizon for the act.

Preserve `available ≠ retrieved ≠ loaded`.

### 3. Skills / Capabilities

Resolve task-fit AIKit skills/Capabilities at runtime. Examples include source inspection, coding, testing, architecture review, prototype work, provider operation, Run-map building and QL refraction where applicable.

A skill contributes method/context/standards/output expectations. Its body is not copied into the issue and it does not replace the semantic task.

### 4. Agent / Agency / model / body

Execution Intelligence chooses an enduring Agent, situated Agency, model, Harness and material execution resources according to:

- task/use fitness;
- modalities and tool requirements;
- independence requirements;
- current availability;
- source/policy constraints;
- learned observations;
- user constraints.

Cost/latency may constrain selection but may not silently undercut a stipulated judgement or quality requirement.

### 5. Execution shape

Choose from actual dependency structure: single, sequential, parallel-independent, candidate fan-out, multi-angle assessment, barrier, synthesis, return/repair, nested Run.

A ticket may fix a dependency or gate when that is semantic. Otherwise reversible decomposition remains runtime latitude.

### 6. Independence

Where independent assessment is required, the producing Execution/AgentSession lineage may not satisfy the gate. Independence is not inferred merely from a different model name.

### 7. Return contract

Return typed Artifacts, Claims, Evidence, Assessments and Events against the acceptance contract. Mutate canonical state only through authorised interfaces. Create a HumanRequest only where a genuinely consequential/authorial Decision remains.

### 8. P5 learning hook

P5 may observe context sufficiency, decomposition, Capability/model/Agency fitness, verification signal, returns, cost/latency and execution shape. These are kept as distinct signals and may influence future resolution without rewriting authored Intent or immutable history.

## E. Updated closure counts

After this amendment:

- ticket-source units: **92**;
- root journeys: **29**;
- root interfaces: **33**;
- original Wayfinder node mapping remains **110/110**;
- original source interface mapping remains **80/80**;
- original M/Z coverage remains **16/16**;
- unmapped governing material remains **0** within the previous closure accounting; the six new units are explicit cross-map refinements introduced by this amendment.

The GitHub issue projection should use the original machine manifest plus these six units and the universal Execution Intelligence issue field. A later self-hosted programme synthesis may fold this amendment directly into the monolithic programme file; until then this file records the added determination without falsifying source provenance.
