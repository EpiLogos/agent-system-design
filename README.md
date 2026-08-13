# Software Factory

This repository is the **Software Factory product repository**.

The Factory develops software through durable, inspectable relations among Project, resolved Context, Run, canonical Run Map, Agent/Agency, AgentSession/Execution, Actions and Capabilities, Claims and Evidence, Candidates, material execution, encounter, Decision/Recognition, retained effects, and renewed Context/Ground.

The canonical architecture and development programme are already established. Implementation proceeds from them; this repository is not a design-only reference collection.

## Current implementation surfaces

- `docs/canon/` — governing architectural corpus and current semantic determinations.
- `docs/program/` — executable build programmes and dependency order.
- `contracts/factory/` — language-neutral Factory contracts, schemas, provenance and fixtures.
- `tests/factory/` — public-semantic and contract tests.
- `scripts/factory_verify.py` — canonical repository-owned deterministic verification operation.
- `factory/` — root Factory reference implementation as it is materialised by the build programme.

Run the current deterministic Factory checks with:

```bash
python3 scripts/factory_verify.py
```

A passing Check is evidence. It is not by itself an Assessment, human Review/Recognition, Gate decision, or Closure determination.

## Product boundaries

The Factory owns the canonical developmental ontology and Run governance. Neighbor products remain separate:

- **AIKit** supplies ContextResolution, resources, provider/harness projection and related operational resolution through public contracts.
- **Workcell** supplies material execution-world realisation through public contracts.
- **QL/MEF** supplies optional formal-semantic/refraction capabilities through a pluggable provider seam.
- **QL Loop Runtime** is a separate recurrence-runtime programme under `ql-agent-experiments/`.
- **Central** is an independent human-owned source product.

GitHub Issues, pull requests and Actions are projections/providers, never the canonical Factory ontology.

## Historical and source-integration material

This repository began by gathering substantial research and executable prior art. That material remains valuable and inspectable, but it is not silently promoted into the current Factory core:

- `super-simple-software-factory/` — SSSF skill/source island.
- `inkwell-agent-sandboxes-and-software-factory/` — Factory-In-A-Box/source island.
- `transcripts/`, `seed-docs/`, `ideas/` — research and design-history material.

Reuse from those sources is explicit: preserve provenance, name the adopted seam, and verify the actual integration. Historical vocabulary such as phase gates or local Run types does not override the current constitutional meanings of Run, Gate, Closure, Claim, Evidence, Decision, Candidate, Agent, Agency, Action or Capability.

## Development entry point

The root build programme is:

`docs/program/QL-SOFTWARE-FACTORY-ROOT-BUILD-PROGRAM.md`

The GitHub issue graph is the executable development programme. Tickets close only against their actual acceptance and Closure conditions with evidence for the exact claimed state.
