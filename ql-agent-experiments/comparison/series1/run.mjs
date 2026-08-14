import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { ClassicRuntime } from '../../foundation/classic-runtime/index.js';
import { QLDirectCoreRuntime } from '../../foundation/ql-core-runtime/index.js';
import { createRunManifest, executeRun, runIdForManifest } from '../../foundation/optics/index.js';
import { createDeepQLRuntimeClass } from '../../deep-ql/index.js';
import { createModelDrivenQLPolicy, bindSeries1Host } from './policy.mjs';
import {
  NativeOpenAICompatibleProvider,
  providerForHost,
  SERIES1_PROVIDER,
  series1JudgeModelId,
  series1ModelId
} from './providers.mjs';
import { Series1Workspace, createLiveHost } from './host.mjs';
import { CONDITIONS, SERIES1_SCHEMA, assertLiveManifest, classifyEffect, compareHeldConstant, stableDigest } from './contract.mjs';
import { evaluateTask, getTask, setupTask } from './tasks.mjs';

const SPEC_REVISION = 'f9d056c54caf094eb672f005ce3c8cbde4de0a5b+QL-PAIRING-SQUARES-CLARIFICATION-08-14-2026';
const DEEP_CLASS = createDeepQLRuntimeClass(QLDirectCoreRuntime);
const CAPABILITIES = Object.freeze([
  { id: 'list_files', args: { path: 'optional relative directory' } },
  { id: 'read_file', args: { path: 'required relative file path' } },
  { id: 'write_file', args: { path: 'required relative file path', content: 'complete UTF-8 replacement' } },
  { id: 'run_tests', args: { files: 'optional relative test path array' } }
]);

function args() {
  const values = process.argv.slice(2);
  const read = (name, fallback = null) => {
    const index = values.indexOf(`--${name}`);
    return index >= 0 ? values[index + 1] : fallback;
  };
  return {
    host: read('host', process.env.QL_SERIES1_HOST ?? 'native'),
    task: read('task', process.env.QL_SERIES1_TASK ?? 'S1-CODE-001'),
    repetitions: Number(read('repetitions', process.env.QL_SERIES1_REPETITIONS ?? '3')),
    maxSteps: Number(read('max-steps', process.env.QL_SERIES1_MAX_STEPS ?? '16'))
  };
}

async function snapshot(root) {
  async function walk(relative = '.') {
    const absolute = path.join(root, relative);
    const entries = await fs.readdir(absolute, { withFileTypes: true });
    const out = {};
    for (const entry of entries.sort((a, b) => a.name.localeCompare(b.name))) {
      const child = relative === '.' ? entry.name : path.join(relative, entry.name);
      if (entry.isDirectory()) out[child] = await walk(child);
      else out[child] = await fs.readFile(path.join(root, child), 'utf8');
    }
    return out;
  }
  return walk();
}

function canonicalModel() {
  const provider = process.env.QL_SERIES1_PROVIDER ?? SERIES1_PROVIDER;
  if (provider !== SERIES1_PROVIDER) {
    throw new Error(`Series 1 is currently stipulated to provider '${SERIES1_PROVIDER}', not '${provider}'.`);
  }
  const id = series1ModelId();
  return { provider, id, parameters: { temperature: 0 } };
}

function runtimeDescriptor(condition) {
  if (condition === 'classic') return { id: 'classic', version: '0.1.0-foundation' };
  if (condition === 'ql-direct') return { id: 'ql-core', version: '0.1.0-foundation' };
  if (condition === 'ql-deep') return { id: 'ql-deep', version: '0.1.0-deep-candidate' };
  throw new Error(`Unknown condition '${condition}'.`);
}

function createRuntime(condition, runId) {
  if (condition === 'classic') return { runtime: new ClassicRuntime(), policy: null };
  if (condition === 'ql-direct') {
    const policy = createModelDrivenQLPolicy({ mode: 'direct', operatorRunId: `${runId}:operators` });
    return { runtime: new QLDirectCoreRuntime({ policy }), policy };
  }
  const policy = createModelDrivenQLPolicy({ mode: 'deep', operatorRunId: `${runId}:operators` });
  return { runtime: new DEEP_CLASS({ policy }), policy };
}

async function judgeSemantic({ task, output, candidateModel }) {
  const judgeModel = series1JudgeModelId();
  if (!judgeModel) throw new Error(`Task ${task.id} is semantic and requires a distinct judge model.`);
  if (judgeModel === candidateModel.id) throw new Error('Blinded semantic judge model must differ from the candidate model.');
  const judge = new NativeOpenAICompatibleProvider({ model: judgeModel });
  judge.assertReady();
  const result = await judge.complete({
    mode: 'control',
    system: 'You are an independent blinded evaluator. Score only the supplied answer against the task, success conditions and rubric. Return JSON {"score":0.0,"rationale":"..."}. Score must be between 0 and 1.',
    prompt: JSON.stringify({ task: task.prompt, success_conditions: task.successConditions, rubric: task.quality.rubric, answer: output }, null, 2)
  });
  const score = Number(result.control?.score);
  if (!Number.isFinite(score) || score < 0 || score > 1) throw new Error(`Judge returned invalid score '${result.control?.score}'.`);
  return { quality_score: score, judge_model: judgeModel, judge_provider: SERIES1_PROVIDER, rationale: result.control?.rationale ?? null, usage: result.usage };
}

async function runCondition({ hostId, task, condition, repetition, model, maxSteps }) {
  const temp = await fs.mkdtemp(path.join(os.tmpdir(), `ql-series1-${task.id}-${condition}-`));
  try {
    await setupTask(task, temp);
    const before = await snapshot(temp);
    const startStateDigest = stableDigest(before);
    const workspace = new Series1Workspace(temp);
    const provider = providerForHost(hostId);
    await provider.assertReady();
    const host = createLiveHost({ hostId, provider, workspace });
    const descriptor = runtimeDescriptor(condition);
    const logicalEnvironment = { kind: 'isolated-series1-workspace', task_id: task.id };
    const baseRequest = {
      taskId: task.id,
      input: task.prompt,
      successConditions: task.successConditions,
      capabilities: CAPABILITIES,
      maxSteps,
      provenance: { series: 1, repetition, condition }
    };
    const manifest = createRunManifest({
      taskId: task.id,
      fixtureId: `series1:${task.id}:r${repetition}`,
      host,
      runtime: descriptor,
      specRevision: SPEC_REVISION,
      model,
      capabilities: CAPABILITIES,
      environment: logicalEnvironment,
      successConditions: task.successConditions,
      startState: { digest: startStateDigest }
    });
    const expectedRunId = runIdForManifest(manifest);
    const { runtime, policy } = createRuntime(condition, expectedRunId);
    const request = bindSeries1Host(baseRequest, host);
    const started = performance.now();
    const record = await executeRun({ runtime, host, request, manifest });
    const elapsedMs = performance.now() - started;

    if (policy?.getOperatorEvents) {
      for (const event of policy.getOperatorEvents()) {
        record.events.push({ ...event, record_index: record.events.length, channel: 'runtime-semantic' });
      }
    }

    const after = await snapshot(temp);
    const output = typeof record.result?.outcome === 'string' ? record.result.outcome : JSON.stringify(record.result?.outcome ?? '');
    const quality = task.quality.kind === 'semantic'
      ? await judgeSemantic({ task, output, candidateModel: model })
      : await evaluateTask(task, temp);
    const usage = host.snapshotUsage();

    return {
      schema: SERIES1_SCHEMA,
      provider_mode: 'live',
      fixture_provider: false,
      host: { id: host.id, revision: host.revision, real_framework_path: host.realFrameworkPath },
      condition,
      repetition,
      runtime: { id: runtime.id, version: runtime.version },
      model,
      task_id: task.id,
      task_digest: stableDigest({ prompt: task.prompt, success_conditions: task.successConditions }),
      start_state_digest: startStateDigest,
      end_state_digest: stableDigest(after),
      capability_digest: stableDigest(CAPABILITIES),
      outcome: output,
      execution_status: record.status.execution,
      semantic_status: record.status.semantic,
      quality_score: quality.quality_score,
      quality,
      elapsed_ms: elapsedMs,
      model_calls: usage.model_calls,
      total_tokens: usage.total_tokens,
      input_tokens: usage.input_tokens,
      output_tokens: usage.output_tokens,
      model_cost: usage.model_cost,
      capability_calls: record.events.filter((event) => event.channel === 'host' && event.event_type === 'capability_requested').length,
      ql_semantic_events: record.events.filter((event) => event.channel === 'runtime-semantic').length,
      operator_events: record.events.filter((event) => ['conjugate_started','conjugate_completed','child_started','child_completed','child_reintegrated'].includes(event.event_type)).length,
      record
    };
  } finally {
    await fs.rm(temp, { recursive: true, force: true });
  }
}

function rotation(repetition) {
  const n = repetition % CONDITIONS.length;
  return [...CONDITIONS.slice(n), ...CONDITIONS.slice(0, n)];
}

function summarizeEffects(records) {
  const byRepetition = new Map();
  for (const record of records) {
    if (!byRepetition.has(record.repetition)) byRepetition.set(record.repetition, {});
    byRepetition.get(record.repetition)[record.condition] = record;
  }
  return [...byRepetition.entries()].map(([repetition, group]) => ({
    repetition,
    direct: classifyEffect({ classic: group.classic, candidate: group['ql-direct'] }),
    deep: classifyEffect({ classic: group.classic, candidate: group['ql-deep'] }),
    quality: Object.fromEntries(CONDITIONS.map((condition) => [condition, group[condition]?.quality_score ?? null])),
    tokens: Object.fromEntries(CONDITIONS.map((condition) => [condition, group[condition]?.total_tokens ?? null]))
  }));
}

async function main() {
  const config = args();
  if (!Number.isInteger(config.repetitions) || config.repetitions < 1) throw new Error('--repetitions must be a positive integer.');
  if (!Number.isInteger(config.maxSteps) || config.maxSteps < 1) throw new Error('--max-steps must be a positive integer.');
  if (!process.env.DEEPSEEK_API_KEY) throw new Error('DEEPSEEK_API_KEY is required for live Series 1 runs.');
  const task = getTask(config.task);
  const model = canonicalModel();
  const records = [];

  for (let repetition = 0; repetition < config.repetitions; repetition += 1) {
    for (const condition of rotation(repetition)) {
      records.push(await runCondition({ hostId: config.host, task, condition, repetition, model, maxSteps: config.maxSteps }));
    }
  }

  const held = compareHeldConstant(records);
  const manifest = {
    schema: SERIES1_SCHEMA,
    provider_mode: 'live',
    fixture_provider: false,
    credential_contract: 'DEEPSEEK_API_KEY',
    host: records[0].host,
    model,
    conditions: CONDITIONS,
    held_constant: held,
    quality: {
      kind: task.quality.kind,
      judge_provider: task.quality.kind === 'semantic' ? SERIES1_PROVIDER : null,
      judge_model: task.quality.kind === 'semantic' ? series1JudgeModelId() : null
    },
    task: { id: task.id, category: task.category, anti_overengineering: Boolean(task.anti_overengineering) },
    repetitions: config.repetitions,
    records,
    effects: summarizeEffects(records)
  };
  assertLiveManifest(manifest);
  process.stdout.write(`${JSON.stringify(manifest, null, 2)}\n`);
}

main().catch((error) => {
  console.error(error.stack ?? error.message ?? String(error));
  process.exitCode = 1;
});
