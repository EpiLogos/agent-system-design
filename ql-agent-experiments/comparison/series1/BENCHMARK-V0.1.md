# Series 1 Benchmark v0.1 — matched human-review experiment

## Purpose

Series 1 v0.1 is a small controlled benchmark for **experiencing and comparing agent behaviour**, not an automated leaderboard.

The primary question is:

> When the same model is given the same prompt, files, capabilities, verification contract and execution budget, how does Classic recurrence differ from Direct QL and Deep QL in understanding, action, recovery, restraint and fulfilment of intent?

The first determination is human product judgement over the complete run record. Automated/scalar evals are a later stage and must not replace this initial inspection.

## Experimental variable

Only the loop condition changes inside a matched set:

- `classic`
- `ql-direct`
- `ql-deep`

The benchmark is repeated through three host paths (`pi`, `pydantic-ai`, `native`) to expose host/runtime interaction, but the primary comparison is **within one host for one task**.

## Held-constant contract

For a matched Classic / Direct / Deep set, all of the following MUST be identical:

1. candidate provider and model (`deepseek` / `deepseek-v4-flash` for this series);
2. model parameters;
3. exact user task prompt;
4. exact success/constraint text supplied to the agent;
5. complete starting workspace bytes;
6. capability names, descriptions and argument contracts;
7. task-specific objective verification protocol;
8. maximum recurrence/step budget;
9. host profile and pinned host revision;
10. network policy: the task workspace has no internet/retrieval capability; only the model-provider boundary has network access.

Condition order rotates across repetitions. No condition receives extra hints, a different skill, a different verifier or a privileged tool.

The manifest records digests for prompt/task, starting workspace, capabilities, verifier and execution budget. A mismatch makes the set ineligible for comparison.

## Universal observation protocol

Every run preserves:

- exact benchmark prompt and constraints;
- full starting workspace snapshot;
- every model input (system + request payload) and returned model envelope/control decision;
- every capability call, arguments and complete capability result/error;
- runtime/QL semantic events;
- final answer/outcome;
- full final workspace snapshot;
- task-specific objective verification output;
- model calls, tokens, provider cost where exposed, capability calls and elapsed time;
- exact host/runtime/model/spec revisions.

No fixture provider may enter this evidence path.

## Human determination

Series 1 v0.1 records `determination: pending-human-review`. It does **not** generate a winner from tests or a judge model.

The reviewer inspects at least:

1. **Prompt apprehension** — what did the agent understand, miss, invent or preserve?
2. **Epistemic conduct** — did it distinguish evidence, inference, uncertainty and contradiction appropriately?
3. **Action selection** — were tool calls relevant, ordered and sufficient? Did it inspect before acting when warranted?
4. **Agency / skill use** — did it use procedures as aids to judgement rather than ceremony? Did it know when not to act?
5. **Difference and recovery** — how did it react when evidence contradicted its working view or an action failed?
6. **Closure** — did it stop at the right point, prematurely, or continue pointlessly?
7. **Output / artifact** — did the final answer or workspace actually fulfil the intent?
8. **Friction** — unnecessary model calls, tool calls, verbosity, edits, latency or complexity.
9. **QL-specific behaviour** — for Direct/Deep, did semantic recurrence, conjugation, depth or square modulation visibly help, harm or merely decorate the run?

Objective verification is supporting evidence for that judgement, not the judgement itself.

## Benchmark tasks

### S1-CODE-001 — code repair and verification

A compact multi-file behavioural repair with a public-API constraint and executable tests.

Tests: inspection before change, correct causal repair, constraint preservation, verification, restraint from unrelated edits.

### S1-RESEARCH-001 — local-file research

A small source corpus containing current evidence, superseded material and a genuine unresolved point. The agent must answer from local files only and cite its basis.

Tests: source discovery, relevance selection, temporal/source discrimination, synthesis, citation and resistance to importing outside facts.

### S1-EPISTEMIC-001 — understanding under incomplete evidence

A local evidence packet supports a best current explanation but not certainty. The agent must separate observation, inference and open question, and identify what would resolve the uncertainty.

Tests: epistemic calibration, contradiction handling, explanatory synthesis and non-premature closure.

### S1-SKILL-001 — procedural skill with judgement

The workspace contains a `SKILL.md`, an incoming request and several notes. Some skill steps are conditional. The agent must use the skill to produce the requested deliverable without mechanically performing irrelevant optional work.

Tests: skill discovery, procedural compliance, contextual judgement, selective evidence use and artifact completion.

### S1-AGENCY-001 — truthful state and justified action

The workspace contains code/tests plus a stale status document. The request is to make the workspace truthful and ready, not to assume code must change.

Tests: inspect/verify before editing, distinguish stale description from broken implementation, choose the narrowest justified action, and leave evidence of current state.

### S1-RESTRAINT-001 — trivial bounded request

A one-file factual request that should require one read and a short answer, with no edits.

Tests: whether extra recurrence becomes ceremony, tool minimality, directness and correct stopping.

## Verification philosophy

Each task has an **objective verifier** that is identical across conditions. It may run tests or check invariant facts such as source preservation or required artifact existence. It returns observations/checks, not a scalar capability score.

For research/understanding tasks, a human-only reference sheet is stored in task metadata and included in the review bundle but is never sent to the candidate model. It exists to make source-grounded review easier, not to auto-grade prose.

## Execution phases

**Phase A — exploratory human review (this benchmark):** one repetition is sufficient to begin. Review the complete runs and develop a feel for the behavioural differences and any obvious implementation defects.

**Phase B — repeated evidence:** after the benchmark itself is trusted, run repeated matched trials to account for model variance.

**Phase C — formal evals/ablations:** only after the behaviour is understood should scalar evals, blinded judges, deeper operator ablations or statistical aggregation be introduced.

Do not skip Phase A merely because automated scoring is convenient.