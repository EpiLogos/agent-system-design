import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import {
  DETERMINATION,
  SERIES1_SCHEMA,
  assertLiveManifest,
  compareHeldConstant,
  stableDigest
} from '../../comparison/series1/contract.mjs';
import { LiveRuntimeHost, Series1Workspace } from '../../comparison/series1/host.mjs';
import {
  normalizeEnvelope,
  parseJsonObject,
  SERIES1_PROVIDER,
  SERIES1_DEFAULT_MODEL,
  DEEPSEEK_BASE_URL
} from '../../comparison/series1/providers.mjs';
import { SERIES1_TASKS, setupTask, verifyTask } from '../../comparison/series1/tasks.mjs';

test('Series 1 refuses fixture-shaped evidence and requires human-review contract', () => {
  assert.throws(() => assertLiveManifest({
    schema: SERIES1_SCHEMA,
    provider_mode: 'fixture',
    fixture_provider: true,
    host: { id: 'pi', revision: 'x', real_framework_path: 'fixture' },
    model: { provider: 'fixture', id: 'fake' },
    conditions: ['classic','ql-direct','ql-deep'],
    held_constant: { task: true, start_state: true, model: true, capabilities: true, verification: true, budget: true },
    determination: DETERMINATION,
    review: { prompt: 'x', focus: ['x'] },
    records: [{}]
  }), /not evidence eligible/);
});

test('Series 1 uses provider-native DeepSeek candidate defaults', () => {
  assert.equal(SERIES1_PROVIDER, 'deepseek');
  assert.equal(SERIES1_DEFAULT_MODEL, 'deepseek-v4-flash');
  assert.equal(DEEPSEEK_BASE_URL, 'https://api.deepseek.com');
  assert.equal(DETERMINATION, 'pending-human-review');
});

test('held-constant comparison covers task, workspace, model, capabilities, verifier and budget', () => {
  const records = ['classic','ql-direct','ql-deep'].map((condition) => ({
    condition,
    task_digest: stableDigest({ prompt: 'same', success: ['same'] }),
    start_state_digest: 'same-start',
    capability_digest: stableDigest([{ id: 'read_file', args: { path: 'x' } }]),
    verification_protocol_digest: 'same-verifier',
    execution_budget_digest: 'same-budget',
    model: { provider: 'deepseek', id: 'deepseek-v4-flash', parameters: { temperature: 0 } }
  }));
  assert.deepEqual(compareHeldConstant(records), {
    task: true,
    start_state: true,
    model: true,
    capabilities: true,
    verification: true,
    budget: true
  });
  assert.equal(stableDigest({ a: { x: 1, y: 2 }, b: 3 }), stableDigest({ b: 3, a: { y: 2, x: 1 } }));
});

test('workspace capabilities cannot escape the isolated task root', async () => {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'series1-boundary-'));
  try {
    const workspace = new Series1Workspace(root);
    await fs.writeFile(path.join(root, 'ok.txt'), 'ok');
    assert.equal((await workspace.execute('read_file', { path: 'ok.txt' })).content, 'ok');
    await assert.rejects(() => workspace.execute('read_file', { path: '../escape.txt' }), /escapes Series 1 workspace/);
    await assert.rejects(() => workspace.execute('write_file', { path: '../../escape.txt', content: 'bad' }), /escapes Series 1 workspace/);
  } finally {
    await fs.rm(root, { recursive: true, force: true });
  }
});

test('live host observation retains complete model and capability IO', async () => {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'series1-trace-'));
  try {
    await fs.writeFile(path.join(root, 'fact.txt'), 'value');
    const workspace = new Series1Workspace(root);
    const provider = {
      id: 'test-live-provider',
      async complete() {
        return { content: 'done', capabilityCalls: [], usage: { input_tokens: 1, output_tokens: 1, total_tokens: 2 }, raw: { model: 'x' } };
      }
    };
    const host = new LiveRuntimeHost({ id: 'native', revision: 'x', realFrameworkPath: 'test', provider, workspace });
    const events = [];
    host.attachObserver({ emit: (event) => events.push(event) }, 'run-test');
    await host.callModel({ request: { input: 'prompt' } });
    await host.executeCapability({ name: 'read_file', args: { path: 'fact.txt' } });
    assert.equal(events.find((e) => e.event_type === 'model_requested').payload.input.prompt, '"prompt"');
    assert.equal(events.find((e) => e.event_type === 'model_returned').payload.output.content, 'done');
    assert.equal(events.find((e) => e.event_type === 'capability_returned').payload.result.content, 'value');
  } finally {
    await fs.rm(root, { recursive: true, force: true });
  }
});

test('live response parser enforces the portable turn envelope', () => {
  assert.deepEqual(normalizeEnvelope('{"content":"done","capabilityCalls":[]}').capabilityCalls, []);
  assert.equal(parseJsonObject('prefix {"choice":"P4"} suffix').choice, 'P4');
  assert.throws(() => parseJsonObject('not json'), /did not return a JSON object/);
});

test('benchmark v0.1 spans six distinct human-review task families with frozen protocols', async () => {
  const expected = new Map([
    ['S1-CODE-001', 'code'],
    ['S1-RESEARCH-001', 'local-research'],
    ['S1-EPISTEMIC-001', 'epistemic-understanding'],
    ['S1-SKILL-001', 'skill-agency'],
    ['S1-AGENCY-001', 'agency-recovery'],
    ['S1-RESTRAINT-001', 'bounded-restraint']
  ]);
  assert.equal(SERIES1_TASKS.length, expected.size);
  for (const task of SERIES1_TASKS) {
    assert.equal(task.category, expected.get(task.id));
    assert.ok(task.prompt.length > 20);
    assert.ok(task.successConditions.length >= 3);
    assert.ok(task.verificationProtocol.length >= 3);
    assert.ok(task.reviewFocus.length >= 6);
    assert.ok(task.reviewReference.length >= 1);
    assert.equal('quality' in task, false);

    const root = await fs.mkdtemp(path.join(os.tmpdir(), `series1-${task.id}-`));
    try {
      await setupTask(task, root);
      const names = await fs.readdir(root);
      assert.ok(names.length > 0);
      if (task.id === 'S1-RESEARCH-001' || task.id === 'S1-EPISTEMIC-001' || task.id === 'S1-RESTRAINT-001') {
        const snapshot = {};
        async function walk(dir = '.') {
          for (const entry of await fs.readdir(path.join(root, dir), { withFileTypes: true })) {
            const child = dir === '.' ? entry.name : `${dir}/${entry.name}`;
            if (entry.isDirectory()) await walk(child);
            else snapshot[child] = await fs.readFile(path.join(root, child), 'utf8');
          }
        }
        await walk();
        const verification = await verifyTask(task, root, { before: snapshot, after: { ...snapshot }, output: '' });
        assert.equal(verification.objective_checks_pass, true);
      }
    } finally {
      await fs.rm(root, { recursive: true, force: true });
    }
  }
});
