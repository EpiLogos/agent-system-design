---
name: factory-operation
description: Operate Software Factory Projects, Runs, RunMaps, build state, evidence, Candidates and native Actions through the public Factory read/action contracts.
---

# Factory operation

Use this Skill when an authorised human or Agent needs to orient in Factory developmental state, inspect a Run, gather evidence, request a native Factory Action, or explain why a Run is blocked or not promotable.

## Contract metadata

- Semantic ref: `factory:operator`
- Native owner: `EpiLogos/agent-system-design` / Software Factory
- Executable root: `factory/`
- Build read contract: `factory.build-view/v1`
- Build provider contract: `factory.build-view-provider/v1`
- Public read path: `FactoryBuildViewProvider::snapshot`
- Public action path: `FactoryActionExecutor::execute`
- Verification: Factory Rust quality baseline plus `python3 scripts/validate_factory_skills.py`
- Risk class: developmental/action-sensitive; a Run is not host authority

## Ownership boundaries

Factory owns developmental Project/Run/RunMap/Candidate/Claim/Evidence/HumanRequest and Factory-native Action semantics. AIKit refs in Execution records remain AIKit-owned. Workcell binding refs remain Workcell-owned. Actuation Agency/Return refs remain Actuation-owned. QL/MEF is optional formalism/provider. Preserve opaque external identities; do not reinterpret them into Factory authority.

Keep these distinctions explicit:

```text
observation != Claim acceptance
Claim != Evidence
Candidate != recognised change
RunMutationAuthority != host authority
Capability granted != Action authorised
Factory Action != arbitrary shell execution
Return received != owner Recognition
successful Run != automatic promotion
```

## Inputs

Obtain the resolved Project reference, any ProjectMap/navigation reference available from the Project/AIKit context, the selected Run reference, and an authority-bearing caller context. Do not require a UI layout: the semantic read model is `FactoryBuildSnapshot` / `FactoryBuildView`.

## Procedure

1. **Orient in the authored whole.** Confirm the Project ref and use the resolved ProjectMap/code/knowledge navigation surface when available to understand the wider project. ProjectMap is a navigation source; do not fabricate one from the Build view.
2. **Read Factory state through the native provider.** Build a `FactoryBuildSelection { project_ref, run_ref }` and request `FactoryBuildViewProvider::snapshot`. Verify the snapshot contracts, revision and provenance before treating it as current.
3. **Inspect the Run and RunMap.** Read `view.run`, its `run_map_ref`, status and `view.frontier`. The frontier may be work, decision, recognition or return. Do not turn one projection of the RunMap into the canonical history.
4. **Inspect epistemic state.** Read Claims and their Evidence refs separately. Check Evidence assessment, native provenance and producing Execution refs. Missing evidence remains missing.
5. **Inspect Candidates and human apertures.** Read Candidate revision/status, producing Executions, Claims, Evidence, artifacts, preview and trade-offs. Read `HumanRequestRecord` for actual authorial questions and blocked Executions rather than escalating routine implementation choices.
6. **Inspect Agency/Execution/Return relations without ownership collapse.** Agency records may carry root-scope/metagency/actuation/Return refs; Execution records may carry AIKit harness, HarnessComposition, AgentSession, SessionSpace and Surface refs plus Workcell bindings. These refs are evidence of relation, not Factory grants.
7. **Mutate Run topology only through Run authority.** Use the Run's `RunMutationAuthority` with `apply_run_topology_command`. Never treat file access, a Skill, or a model as Run mutation authority.
8. **Invoke only advertised Factory Actions.** Read `view.actions`. For `FactoryActionExecutor::execute`, supply the exact Action ref/subject/Run plus `FactoryActionAuthority`. The executor must see the Factory native owner, non-empty authority ref, the required capability ref with `capability_granted=true`, and `action_authorised=true`. Any missing dimension fails closed.
9. **Separate evidence request from Recognition.** The current native `Request more evidence` Action creates a HumanRequest; it does not recognise a Candidate or promote a change. Recognition/promotion remains owner/human governed.
10. **Verify provenance.** Record Factory state revision, Run revision, RunMap revision, source and exact repository revision with the evidence used for the decision.

## Outputs

Return a concise current-state explanation: Project/Run, frontier, Claims/Evidence, Candidates, HumanRequests, Agency/Execution/Return references, available native Actions and their required capability refs, provenance revisions, and any withheld/missing authority.

## Verification

Run:

```text
cargo fmt --manifest-path factory/Cargo.toml -- --check
cargo clippy --manifest-path factory/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path factory/Cargo.toml --all-targets
python3 scripts/validate_factory_skills.py
```

Acceptance must include fail-closed Action cases for missing capability grant and missing Action authority, not only a successful request.
