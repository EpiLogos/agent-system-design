# QL Agent Series 1 — live capability experiment

This directory is the **performance experiment**, not another semantic-conformance suite.

The question is deliberately ordinary:

> Holding model, task, start state, capabilities and host condition as constant as practical, does changing the recurrence from Classic to Direct QL or Deep QL degrade, do nothing for, or improve the agent's ability to respond, enact work and fulfil intent?

## Evidence conditions

A Series 1 record is evidence-eligible only when all of the following are true:

- `provider_mode = live`; no fixture/test/faux model provider;
- a concrete model identity is recorded;
- the host adapter records its real framework/provider path and revision;
- Classic and QL conditions receive the same task, start-state digest, model, parameters and capability surface;
- raw model/capability usage is recorded, including failures;
- outcome evaluation is independent of QL trace conformance;
- coding tasks use deterministic artifact tests where available;
- chat/semantic tasks require a configured blinded judge model or remain `unscored`, never guessed;
- latency, model calls, capability calls and token usage are retained alongside outcome quality;
- Deep operator use is optional and observable; forced-use diagnostic runs are labelled separately.

Fixture hosts remain valuable for conformance but are **ineligible** for capability-effect claims.

## Conditions

Each host exposes three primary conditions:

```text
classic
ql-direct
ql-deep
```

The baseline comparison is within-host first. Cross-host analysis is secondary.

## Host paths

- `pi`: live model turns go through pinned `@earendil-works/pi-ai` (`0.84.1`, corresponding to the inspected Pi revision recorded by #101).
- `pydantic-ai`: live model turns go through a Python bridge using Pydantic AI from the pinned upstream checkout.
- `native`: live model turns use the minimal repo-owned OpenAI-compatible transport directly.

Loop recurrence remains owned by the experiment `LoopRuntime`; framework/provider code supplies the exterior model boundary.

## Model protocol

To keep the recurrence comparison controlled, every host normalises a model turn to:

```json
{
  "content": "assistant content",
  "capabilityCalls": [
    {"id":"...","name":"read_file","args":{"path":"..."}}
  ],
  "usage": {"input_tokens":0,"output_tokens":0}
}
```

Classic lets the model request capabilities through this normalised turn. QL uses model-driven control calls to choose the next QL act and then crosses the same host/capability boundary.

## Running

`node preflight.mjs` performs fail-closed readiness checks.

`node run.mjs` requires live credentials and writes a machine-readable record. The GitHub workflow `QL Series 1 Live` is manual (`workflow_dispatch`) and must not silently fall back to fixture providers.

No live run has occurred merely because this harness exists.