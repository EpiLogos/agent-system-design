# QL Agent Series 1 — live capability experiment

This directory is the **performance experiment**, not another semantic-conformance suite.

The question is deliberately ordinary:

> Holding model, task, start state, capabilities and host condition as constant as practical, does changing the recurrence from Classic to Direct QL or Deep QL degrade, do nothing for, or improve the agent's ability to respond, enact work and fulfil intent?

## Canonical Series 1 model contract

Series 1 is currently stipulated to DeepSeek rather than a fictional generic experiment provider:

- provider: `deepseek`
- candidate model: `deepseek-v4-flash`
- blinded semantic judge: `deepseek-v4-pro`
- API base: `https://api.deepseek.com`
- credential: the provider-native `DEEPSEEK_API_KEY`

`QL_SERIES1_API_KEY` does **not** exist and must not be introduced. Pi, Pydantic AI and the Native transport all consume the same `DEEPSEEK_API_KEY` contract. `QL_SERIES1_MODEL` and `QL_SERIES1_JUDGE_MODEL` remain optional experiment overrides; their defaults are V4 Flash and V4 Pro respectively.

The judge is used only for semantic/chat tasks. Artifact/coding tasks are scored by their deterministic evaluators.

## Evidence conditions

A Series 1 record is evidence-eligible only when all of the following are true:

- `provider_mode = live`; no fixture/test/faux model provider;
- a concrete model identity is recorded;
- the host adapter records its real framework/provider path and revision;
- Classic and QL conditions receive the same task, start-state digest, model, parameters and capability surface;
- raw model/capability usage is recorded, including failures;
- outcome evaluation is independent of QL trace conformance;
- coding tasks use deterministic artifact tests where available;
- chat/semantic tasks use the distinct blinded judge model or remain unscored, never guessed;
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

- `pi`: live model turns go through pinned `@earendil-works/pi-ai` (`0.84.1`) using Pi's built-in `deepseek` provider and its native `DEEPSEEK_API_KEY` discovery.
- `pydantic-ai`: live model turns go through the pinned Pydantic AI checkout. The bridge explicitly constructs `OpenAIChatModel('deepseek-v4-flash', provider=DeepSeekProvider())`; `DeepSeekProvider` discovers `DEEPSEEK_API_KEY`.
- `native`: live model turns call DeepSeek's documented OpenAI-compatible `/chat/completions` endpoint directly with `DEEPSEEK_API_KEY`.

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

## Running locally

If `DEEPSEEK_API_KEY` is already exported in your shell, there is no Series-1-specific credential setup.

The real Pi dependency must be present:

```bash
cd ql-agent-experiments/comparison/series1
npm install --no-save --no-package-lock @earendil-works/pi-ai@0.84.1
```

The Pydantic host requires the pinned Pydantic AI revision and its OpenAI-compatible dependency group, matching the GitHub workflow. Once those dependencies are installed, preflight and a coding run are simply:

```bash
node preflight.mjs --live --host pi --task S1-CODE-001
node run.mjs --host pi --task S1-CODE-001 --repetitions 3
```

Swap `pi` for `pydantic-ai` or `native`. With no model overrides these commands use `deepseek-v4-flash`; semantic tasks use `deepseek-v4-pro` as the judge.

Local execution is the lowest-friction way to smoke/debug the live adapters because it can consume the existing shell environment directly.

## Running in GitHub Actions

GitHub Actions can run the real experiment; a local machine is **not** required for final evidence. GitHub cannot inherit environment variables from a developer machine, so the repository must contain one Actions secret named:

```text
DEEPSEEK_API_KEY
```

The default-branch `QL Series 1 Live` workflow then:

1. checks out the requested experiment ref (default `ql/deep-runtime`);
2. installs the pinned Pi and Pydantic AI stacks;
3. defaults the candidate to `deepseek-v4-flash` and the semantic judge to `deepseek-v4-pro`;
4. runs fail-closed live preflight;
5. runs Classic / Direct QL / Deep QL with matched task/model/capabilities;
6. uploads the JSON evidence artifact.

For the formal experiment record, GitHub Actions is preferable because the dependency setup, ref, inputs and evidence artifact are reproducible. Local runs remain equally valid for development and can also be retained as evidence when their environment/provenance is recorded.

No live run has occurred merely because this harness exists.
