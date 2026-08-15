# Series 1 — DeepSeek Harness maximal-reference amendment

**Status:** proposed normative amendment to Series 1 v0.1 host strategy  
**Parent programme:** #94  
**Live experiment:** #110  
**Base benchmark:** `BENCHMARK-V0.1.md`

## 1. Determination

DeepSeek Harness (`dsh`) is added as the **maximal / ideal reference host** for the already-defined QL loop-runtime experiment.

This amendment does not create a new loop experiment, change Direct/Deep QL semantics, or reopen the frozen `LoopRuntime` / `RuntimeHost` / `RuntimeObserver` contract.

The independent variable inside every matched set remains:

```text
classic
ql-direct
ql-deep
```

DeepSeek Harness is another host implementation of that seam.

The reason to privilege it as the first experiential/reference host is architectural rather than promotional: the harness is compositionally constituted, exposes a replaceable agent loop, retains an inspectable session/tool trajectory, and provides a strong web UI into which experiment-specific inspection surfaces can be composed.

This gives Series 1 a richer test body closer to the maximal agency environment the wider O:I architecture is intended to span, while the existing Pi / Pydantic AI / Native hosts continue to prove portability and host-interaction effects across thinner/different bodies.

---

## 2. Source pin and observed upstream

The adapter MUST be implemented against current pinned upstream source rather than remembered architecture.

Source observed for this amendment:

```text
DeepSeek Harness
repo      deepseek-ai/deepseek-harness
branch    master
revision  47f943859bef60e4160492346772ded9b24f765a
release   @deepseek-ai/dsh 0.1.0-rc.5

Cordis spatiotemporal-composability paper
repo      cordiverse/paper
branch    main
revision  948a07b369c62adb3b12e102458be5c18dfb69b9
draft     2026-08-13
```

DeepSeek Harness is a developer preview. Before implementation or a live benchmark dispatch, re-check upstream, record the exact source revision used, and treat any seam drift as an integration change rather than guessing compatibility.

The observed architecture at the source pin includes:

- a Cordis plugin tree composed from profiles/bundles/patch layers;
- services, typed events, injected dependencies and lifecycle-owned effects;
- append-oriented durable `SessionEvent` history;
- a separate `Agent` service and default `agent-loop` implementation;
- replaceable model, tool, filesystem, subprocess, sandbox, persistence and subagent providers;
- scoped registrations;
- composable Web Client nodes/renderers and client plugin graph;
- trajectory/session query surfaces suitable for complete run inspection.

---

## 3. Host relation

The experiment boundary remains:

```text
DEEPSEEK HARNESS HOST
  model + capabilities + session + environment
  + target-native component/surface composition
                    │
                    ▼
               LOOP RUNTIME
       classic | ql-direct | ql-deep
```

Host mechanics stay outside recurrence semantics.

The DSH adapter should use the cleanest current upstream seam rather than fork the harness. At the observed revision, the architecture explicitly separates the public `Agent` service from the default `agent-loop`; the implementation must verify the exact runtime replacement/interception point before coding.

Any host-specific carrier needed to bridge `RuntimeHost` calls into DSH remains adapter-local unless it exposes genuine pressure on the already-frozen common contract.

---

## 4. Maximal-reference lane and existing cross-host matrix

Series 1 now has two evidence layers at the host dimension.

### A. Maximal-reference lane — primary experiential basis

Run the six frozen v0.1 tasks first through DeepSeek Harness:

```text
6 tasks × 1 dsh host × 1 repetition × 3 loop conditions
= 18 condition-runs
```

This is the primary in-person / human-inspection reference lane.

Its purpose is to experience Classic / Direct / Deep in a rich, inspectable environment before broadening interpretation across the thinner/different hosts.

### B. Existing portability / host-interaction matrix

Retain the already-specified Pi / Pydantic AI / Native exploratory matrix:

```text
6 tasks × 3 hosts × 1 repetition × 3 loop conditions
= 54 condition-runs
```

This matrix remains valuable and is not replaced by DSH. It establishes how the same runtime conditions behave across independent host architectures and prevents a DSH-specific implementation detail from being mistaken for a QL effect.

If both layers complete, Series 1 v0.1 contains 72 condition-runs in total, but they are not one flat 4-host leaderboard:

```text
18  maximal-reference / primary experiential lane
54  cross-host portability / interaction lane
```

The primary comparison remains **within one host/task/repetition**. Cross-host conclusions remain secondary.

---

## 5. Held constants in the DSH lane

Within one DSH matched set, the base benchmark held-constant law applies unchanged.

In particular, Classic / Direct / Deep must share:

- the same `deepseek-v4-flash` candidate and parameters;
- exact task prompt and success/constraint bytes;
- exact starting workspace bytes;
- the same portable benchmark capability contract;
- the same objective verifier;
- the same execution budget;
- the same DSH source revision/profile/component baseline;
- the same network boundary;
- the same evidence/review renderer revision.

The composability of DSH MUST NOT become a condition-specific advantage. No QL condition receives extra tools, skills, components, UI-visible hints or context faculties.

The only condition-specific runtime difference is the selected loop runtime and the semantic/operator state necessarily produced by that runtime.

---

## 6. DSH baseline composition

Freeze a minimal-but-rich DSH benchmark profile that preserves the harness's normal inspectability without introducing unrelated powers.

The profile should include only what is required for:

```text
model/provider access
benchmark workspace capability bridge
session/event persistence
agent-loop/runtime selection seam
human web inspection
experiment observer / trajectory projection
```

The base benchmark still exposes only its portable candidate capabilities:

```text
list_files
read_file
write_file
run_tests
```

DSH may internally provide richer infrastructure to implement those operations, but the candidate model MUST NOT gain extra semantic capabilities relative to the other benchmark hosts.

Record the resolved DSH profile/plugin tree or equivalent composition fingerprint as run provenance.

---

## 7. Inspectable trajectory law

DeepSeek Harness is especially useful because the experiment can retain both portable QL/runtime evidence and the target-native trajectory.

For every DSH condition-run preserve:

```text
base Series 1 run manifest
portable RuntimeObserver / QL semantic events
DSH durable SessionEvent stream relevant to the run
model-visible derived history / request record
all tool calls and results
workspace start/final state
runtime-selection evidence
DSH source + profile/component fingerprint
final output
objective verifier output
```

The DSH session log does not replace Factory/portable Event/Trace semantics. It is target-native evidence that can be cross-referenced to the portable run record.

Model-visible information must remain reconstructable from the retained host evidence to the degree DSH guarantees at the pinned revision.

---

## 8. QL inspection surfaces

Add DSH-native UI plugins/components for human inspection of the experiment where they improve visibility.

Candidate views include:

```text
selected runtime: classic | ql-direct | ql-deep
current / historical Circuit position
Rij transitions
π / ρ exchange and returned difference
P5 candidate determination versus positive closure
reopening / re-entry
conjugation / depth / square-modulation events when present
portable trace ↔ DSH SessionEvent alignment
matched-run comparator links
```

These are **observer/read-model surfaces**. They must not feed hidden hints or condition-specific information back into the model.

The QL UI plugin is therefore held outside the causal candidate capability surface:

```text
runtime emits portable state/events
          ↓
observer / projection
          ↓
DSH human inspection Surface
```

The UI may expose the same data to the human for Classic runs where applicable, with QL-specific fields absent rather than fabricated.

---

## 9. Composition and QL remain separate variables

DSH also creates a future research surface for varying runtime-body composition, but Series 1 v0.1 MUST NOT confound that with the current recurrence comparison.

For this benchmark:

```text
DSH HarnessComposition = held constant inside matched sets
LoopRuntime            = varied
```

Later O:I/AIKit experiments may deliberately vary Components, providers, context faculties or Surfaces while holding the loop fixed. Those are distinct experiments and should use distinct run manifests/claims.

---

## 10. Execution order

The preferred Series 1 sequence becomes:

```text
benchmark freeze / non-leakage
        ↓
review/evidence optics
        ↓
DSH host adapter + composition freeze
        ↓
DSH bounded real-provider smoke
        ↓
18-run maximal-reference lane
        ↓
in-person / close human inspection
        ↓
Pi/Pydantic/Native bounded smoke
        ↓
54-run portability/host-interaction matrix
        ↓
combined human determination
        ↓
Series 1 closure / next frontier
```

A defect in DSH integration is an experiment defect, not a QL result. Fix the adapter and rerun affected matched sets without tuning the task/condition.

---

## 11. Acceptance

The maximal-reference amendment is satisfied when:

- a pinned DSH adapter implements the frozen common `RuntimeHost`/`LoopRuntime` relation without changing common recurrence semantics;
- Classic, Direct and Deep run through the same DSH host/model/capability/session surface;
- the DSH baseline profile/component tree is frozen and provenance-bearing;
- the candidate capability surface remains benchmark-equivalent to the other hosts;
- a bounded live smoke proves real DeepSeek provider execution with no fixture fallback;
- all six tasks produce valid matched DSH sets or explicit invalidation/rerun evidence;
- target-native DSH trajectory and portable runtime/QL trace can be aligned;
- QL inspection UI is observational and cannot leak condition-specific information to the candidate;
- human review can inspect the complete DSH run in the Web UI plus the existing masked/unmasked benchmark bundle;
- the existing Pi/Pydantic/Native matrix remains intact as the cross-host portability layer;
- no result is interpreted as a four-host scalar leaderboard.

---

## 12. Relation to AIKit / O:I

DeepSeek Harness is also the first rich reference target for AIKit V2's composable-runtime work.

That relation must not leak into Series 1 as extra candidate capability. The experiment can record DSH Components/Surfaces/activation provenance in its host manifest, while AIKit separately develops the language-neutral `Component / Contract / Requirement / Contribution / Surface / HarnessComposition` abstraction.

Series 1 uses DSH because it is a strong experimental body. AIKit uses DSH because it is a strong composability conformance target. Those programmes can share source inspection and adapter knowledge without collapsing their semantic purposes.
