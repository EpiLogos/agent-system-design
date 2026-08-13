import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import {
  RuntimeRegistry,
  RUN_STATUS
} from '../runtime-contract/index.js';
import { ClassicRuntime } from '../classic-runtime/index.js';
import {
  QLDirectCoreRuntime,
  qlPosition,
  qlRelation,
  qlFace,
  UnsupportedQLOperatorError,
  RESIDUE_KIND_BY_POSITION
} from '../ql-core-runtime/index.js';
import {
  ScriptedHost,
  ScriptedQLPolicy,
  FOUNDATION_FIXTURES,
  NEGATIVE_FIXTURES,
  runQLFixture,
  evaluateFixtureRun,
  runFoundationGate,
  runABDemo
} from '../fixtures/index.js';
import { replayRun, formatReplay } from '../optics/index.js';

function observer() {
  const events = [];
  return { events, emit: (event) => events.push(structuredClone(event)) };
}

test('QL 4+2 structure is explicit and all R00-R55 relations are representable', () => {
  assert.equal(qlPosition('P0').structuralKind, 'implicate');
  assert.equal(qlPosition('P5').structuralKind, 'implicate');
  for (const id of ['P1', 'P2', 'P3', 'P4']) {
    assert.equal(qlPosition(id).structuralKind, 'explicate');
  }
  assert.equal(qlFace('direct'), 'direct');
  assert.equal(qlFace('conjugate'), 'conjugate');

  const relations = [];
  for (let from = 0; from < 6; from += 1) {
    for (let to = 0; to < 6; to += 1) {
      relations.push(qlRelation(`P${from}`, `P${to}`).id);
    }
  }
  assert.equal(new Set(relations).size, 36);
  assert.ok(relations.includes('R41'));
  assert.ok(relations.includes('R50'));
  assert.ok(relations.includes('R00'));
  assert.deepEqual(Object.values(RESIDUE_KIND_BY_POSITION), [
    'frame', 'material', 'effect', 'form', 'evaluation', 'determination'
  ]);
});

test('reference host module is QL-semantic-type free', async () => {
  const source = await readFile(new URL('../fixtures/scripted-host.js', import.meta.url), 'utf8');
  assert.equal(source.includes('ql-core-runtime'), false);
  assert.equal(/\bP[0-5]\b/.test(source), false);
  assert.equal(/\bR[0-5][0-5]\b/.test(source), false);
});

test('reference host is runtime-agnostic and both runtimes use the same host contract', async () => {
  const hostMethods = ['callModel', 'executeCapability', 'receiveExternalInput', 'readContext'];
  const host = new ScriptedHost();
  for (const method of hostMethods) assert.equal(typeof host[method], 'function');

  const classic = new ClassicRuntime();
  const ql = new QLDirectCoreRuntime({
    policy: new ScriptedQLPolicy([
      { source: 'P0', carrier: { kind: 'model' }, difference: 'form', destination: 'P3' },
      { source: 'P3', carrier: { kind: 'model' }, difference: 'adequate', destination: 'P4' },
      {
        source: 'P4', carrier: { kind: 'model' }, difference: 'determined', destination: 'P5',
        determination: {
          synthesis: 'done', intent_ref: 'i', evidence_refs: [], evaluation_refs: [], unresolved_refs: [], requested_outcome: 'close'
        },
        verdict: { status: 'close', task_success: 'true' }
      }
    ])
  });
  const registry = new RuntimeRegistry().register(classic).register(ql);
  assert.deepEqual(registry.list().map((item) => item.id), ['classic', 'ql-core']);
});

test('ClassicRuntime covers zero-tool completion', async () => {
  const o = observer();
  const host = new ScriptedHost({ modelResponses: [{ content: 'done', capabilityCalls: [] }] });
  host.attachObserver(o, 'classic-zero');
  const result = await new ClassicRuntime().run({ taskId: 'classic-zero', input: 'x', runId: 'classic-zero' }, host, o);
  assert.equal(result.status, RUN_STATUS.COMPLETED);
  assert.equal(result.capabilityCalls, 0);
  assert.equal(result.modelCalls, 1);
});

test('ClassicRuntime covers one-tool and multi-tool recurrence', async () => {
  const o = observer();
  const host = new ScriptedHost({
    modelResponses: [
      { content: 'tools', capabilityCalls: [{ name: 'a' }, { name: 'b' }] },
      { content: 'done', capabilityCalls: [] }
    ],
    capabilities: {
      a: async () => ({ ok: true }),
      b: async () => ({ ok: true })
    }
  });
  host.attachObserver(o, 'classic-tools');
  const result = await new ClassicRuntime().run({ taskId: 'classic-tools', input: 'x', runId: 'classic-tools' }, host, o);
  assert.equal(result.status, RUN_STATUS.COMPLETED);
  assert.equal(result.capabilityCalls, 2);
  assert.equal(result.modelCalls, 2);
});

test('ClassicRuntime keeps failed capability result in the ordinary loop', async () => {
  const o = observer();
  const host = new ScriptedHost({
    modelResponses: [
      { content: 'tool', capabilityCalls: [{ name: 'fail' }] },
      { content: 'recover', capabilityCalls: [] }
    ],
    capabilities: {
      fail: async () => ({ ok: false, error: 'scripted failure' })
    }
  });
  host.attachObserver(o, 'classic-failed-tool');
  const result = await new ClassicRuntime().run({ taskId: 'classic-failed-tool', input: 'x', runId: 'classic-failed-tool' }, host, o);
  assert.equal(result.status, RUN_STATUS.COMPLETED);
  assert.equal(result.capabilityCalls, 1);
  assert.equal(result.history.some((entry) => entry.role === 'capability' && entry.result.ok === false), true);
});

test('ClassicRuntime incorporates follow-up input and calls the model again', async () => {
  const o = observer();
  const host = new ScriptedHost({
    modelResponses: [
      { content: 'first', capabilityCalls: [] },
      { content: 'second', capabilityCalls: [] }
    ],
    externalInputs: ['steer']
  });
  host.attachObserver(o, 'classic-followup');
  const result = await new ClassicRuntime().run({ taskId: 'classic-followup', input: 'x', runId: 'classic-followup' }, host, o);
  assert.equal(result.status, RUN_STATUS.COMPLETED);
  assert.equal(result.modelCalls, 2);
  assert.equal(result.history.some((entry) => entry.role === 'user' && entry.content === 'steer'), true);
});

test('ClassicRuntime distinguishes cancellation and exhaustion from completion', async () => {
  const cancelledObserver = observer();
  const cancelledHost = new ScriptedHost();
  cancelledHost.attachObserver(cancelledObserver, 'classic-cancel');
  const controller = new AbortController();
  controller.abort();
  const cancelled = await new ClassicRuntime().run({ taskId: 'classic-cancel', runId: 'classic-cancel' }, cancelledHost, cancelledObserver, controller.signal);
  assert.equal(cancelled.status, RUN_STATUS.CANCELLED);

  const exhaustedObserver = observer();
  const exhaustedHost = new ScriptedHost({
    modelResponses: [{ content: 'loop', capabilityCalls: [{ name: 'a' }] }],
    capabilities: { a: async () => ({ ok: true }) }
  });
  exhaustedHost.attachObserver(exhaustedObserver, 'classic-exhaust');
  const exhausted = await new ClassicRuntime().run({ taskId: 'classic-exhaust', runId: 'classic-exhaust', maxSteps: 1 }, exhaustedHost, exhaustedObserver);
  assert.equal(exhausted.status, RUN_STATUS.EXHAUSTED);
});

test('Direct Core emits projection 0/1, return 1/0, interpretation, then transition in order', async () => {
  const fixture = FOUNDATION_FIXTURES.find((item) => item.id === 'QLF-005');
  const run = await runQLFixture(fixture);
  const semantic = run.events.filter((event) => event.channel === 'runtime-semantic');
  const projectionIndex = semantic.findIndex((event) => event.event_type === 'projection' && event.ql?.projection === '0/1');
  const returnIndex = semantic.findIndex((event) => event.event_type === 'return_received' && event.ql?.return === '1/0');
  const interpretedIndex = semantic.findIndex((event) => event.event_type === 'return_interpreted' && event.ql?.relation === 'R43');
  const transitionIndex = semantic.findIndex((event) => event.event_type === 'transition' && event.ql?.relation === 'R43');
  assert.ok(projectionIndex >= 0);
  assert.ok(returnIndex > projectionIndex);
  assert.ok(interpretedIndex > returnIndex);
  assert.ok(transitionIndex > interpretedIndex);
});

test('same read carrier produces different semantic relations and destinations', async () => {
  const fixture = FOUNDATION_FIXTURES.find((item) => item.id === 'QLF-012');
  const run = await runQLFixture(fixture);
  const readActs = run.events.filter((event) => event.event_type === 'act_created' && event.payload?.act?.carrier?.name === 'read');
  assert.equal(readActs.length, 3);
  const relations = run.events.filter((event) => event.event_type === 'transition').map((event) => event.ql?.relation);
  assert.ok(relations.includes('R11'));
  assert.ok(relations.includes('R13'));
  assert.ok(relations.includes('R34'));
});

test('tool-free QL run can positively close on non-linear P0→P3→P4→P5 route', async () => {
  const fixture = FOUNDATION_FIXTURES.find((item) => item.id === 'QLF-013');
  const run = await runQLFixture(fixture);
  const evidence = evaluateFixtureRun(run);
  assert.equal(evidence.passed, true, evidence.failures.join('\n'));
  assert.equal(run.result.status, RUN_STATUS.COMPLETED);
  assert.deepEqual(evidence.relations, ['R03', 'R34', 'R45']);
  assert.equal(evidence.capability_calls, 0);
  assert.equal(evidence.positive_closure, true);
});

test('successful tools do not imply QL closure', async () => {
  const fixture = FOUNDATION_FIXTURES.find((item) => item.id === 'QLF-014');
  const run = await runQLFixture(fixture);
  const evidence = evaluateFixtureRun(run);
  assert.equal(evidence.passed, true, evidence.failures.join('\n'));
  assert.ok(evidence.capability_calls >= 3);
  assert.equal(evidence.positive_closure, false);
  assert.equal(run.result.status, RUN_STATUS.EXHAUSTED);
});

test('P5 reopening remains typed across R51/R52/R53/R54/R50', async () => {
  for (const id of ['QLF-006', 'QLF-007', 'QLF-008', 'QLF-009', 'QLF-010']) {
    const fixture = FOUNDATION_FIXTURES.find((item) => item.id === id);
    const run = await runQLFixture(fixture);
    const evidence = evaluateFixtureRun(run);
    assert.equal(evidence.passed, true, `${id}: ${evidence.failures.join('; ')}`);
    assert.equal(evidence.positive_closure, false);
  }
});

test('positive closure is explicit and re-entry retains typed difference', async () => {
  const fixture = FOUNDATION_FIXTURES.find((item) => item.id === 'QLF-017');
  const run = await runQLFixture(fixture);
  const evidence = evaluateFixtureRun(run);
  assert.equal(evidence.passed, true, evidence.failures.join('\n'));
  assert.equal(run.result.closure.closed_at_position, 'P5');
  assert.equal(run.result.reentry.renewed_frame.inherited_delta, run.result.reentryDelta.id);
  assert.deepEqual(run.result.reentryDelta.achieved_artifact_refs, ['artifact:A']);
  assert.ok(run.result.reentryDelta.established_material_refs.includes('evidence:E'));
  assert.ok(run.result.reentryDelta.unresolved_refs.includes('question:Q'));
});

test('no-tool candidate without closure evaluation cannot auto-close', async () => {
  const fixture = NEGATIVE_FIXTURES.find((item) => item.id === 'QLN-001');
  const run = await runQLFixture(fixture);
  const evidence = evaluateFixtureRun(run);
  assert.equal(evidence.passed, true, evidence.failures.join('\n'));
  assert.equal(run.result.status, RUN_STATUS.EXHAUSTED);
  assert.equal(run.events.some((event) => event.event_type === 'circuit_closed'), false);
});

test('P5 is not stop: inadequate determination reopens instead of closing', async () => {
  const fixture = NEGATIVE_FIXTURES.find((item) => item.id === 'QLN-003');
  const run = await runQLFixture(fixture);
  const evidence = evaluateFixtureRun(run);
  assert.equal(evidence.passed, true, evidence.failures.join('\n'));
  assert.equal(run.events.some((event) => event.event_type === 'circuit_reopened' && event.ql?.relation === 'R51'), true);
  assert.equal(run.events.some((event) => event.event_type === 'circuit_closed'), false);
});

test('six-step coercion is absent', async () => {
  const fixture = NEGATIVE_FIXTURES.find((item) => item.id === 'QLN-006');
  const run = await runQLFixture(fixture);
  const evidence = evaluateFixtureRun(run);
  assert.equal(evidence.passed, true, evidence.failures.join('\n'));
  assert.deepEqual(evidence.relations, ['R03', 'R34', 'R45']);
});

test('Direct Core exposes but does not fake later conjugation/depth operators', () => {
  const runtime = new QLDirectCoreRuntime({ policy: new ScriptedQLPolicy([]) });
  assert.throws(() => runtime.openConjugate(), UnsupportedQLOperatorError);
  assert.throws(() => runtime.openChild(), UnsupportedQLOperatorError);
});

test('foundation fixture gate passes every required #99 fixture without network or model API', async () => {
  const gate = await runFoundationGate();
  assert.equal(gate.deterministic, true);
  assert.equal(gate.network_required, false);
  assert.equal(gate.model_api_required, false);
  assert.equal(gate.fixture_count, FOUNDATION_FIXTURES.length + NEGATIVE_FIXTURES.length);
  assert.equal(gate.passed, true, JSON.stringify(gate.fixtures.filter((item) => !item.passed), null, 2));
});

test('A/B optics hold host/task/model/capability/environment conditions constant while runtime changes', async () => {
  const ab = await runABDemo();
  for (const [key, held] of Object.entries(ab.comparison.held_constant)) {
    assert.equal(held, true, `expected ${key} to be held constant`);
  }
  assert.equal(ab.comparison.changed.runtime.classic.id, 'classic');
  assert.equal(ab.comparison.changed.runtime.ql.id, 'ql-core');
  assert.ok(ab.comparison.event_counts.ql_semantic > 0);
  assert.ok(ab.comparison.event_counts.ql_host > 0);
  assert.ok(ab.ql.events.some((event) => event.channel === 'host'));
  assert.ok(ab.ql.events.some((event) => event.channel === 'runtime-semantic'));
});

test('stored A/B run record replays without live host/session', async () => {
  const ab = await runABDemo();
  const serialized = JSON.parse(JSON.stringify(ab.ql));
  const replay = replayRun(serialized);
  assert.equal(replay.run_id, serialized.run_id);
  assert.equal(replay.status, serialized.result.status);
  assert.ok(replay.events.length > 0);
  assert.match(formatReplay(serialized), /R03/);
});

test('foundation gate and A/B CLI entry points are executable', () => {
  const gate = spawnSync(process.execPath, ['fixtures/run-foundation-gate.mjs'], {
    cwd: new URL('..', import.meta.url),
    encoding: 'utf8'
  });
  assert.equal(gate.status, 0, gate.stderr || gate.stdout);
  const parsedGate = JSON.parse(gate.stdout);
  assert.equal(parsedGate.passed, true);

  const ab = spawnSync(process.execPath, ['optics/run-ab.mjs', '--json'], {
    cwd: new URL('..', import.meta.url),
    encoding: 'utf8'
  });
  assert.equal(ab.status, 0, ab.stderr || ab.stdout);
  const parsedAB = JSON.parse(ab.stdout);
  assert.equal(parsedAB.comparison.changed.runtime.ql.id, 'ql-core');
});
