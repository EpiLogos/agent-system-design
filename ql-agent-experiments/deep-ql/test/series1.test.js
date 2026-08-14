import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { assertLiveManifest, classifyEffect, compareHeldConstant, stableDigest } from '../../comparison/series1/contract.mjs';
import { Series1Workspace } from '../../comparison/series1/host.mjs';
import {
  normalizeEnvelope,
  parseJsonObject,
  SERIES1_PROVIDER,
  SERIES1_DEFAULT_MODEL,
  SERIES1_DEFAULT_JUDGE_MODEL,
  DEEPSEEK_BASE_URL
} from '../../comparison/series1/providers.mjs';
import { SERIES1_TASKS, evaluateTask, setupTask } from '../../comparison/series1/tasks.mjs';

test('Series 1 refuses fixture-shaped evidence', () => {
  assert.throws(() => assertLiveManifest({
    schema: 'ql-series1-run/0.1',
    provider_mode: 'fixture',
    fixture_provider: true,
    host: { id: 'pi', revision: 'x', real_framework_path: 'fixture' },
    model: { provider: 'fixture', id: 'fake' },
    conditions: ['classic','ql-direct','ql-deep'],
    held_constant: { task: true, start_state: true, model: true, capabilities: true },
    quality: { kind: 'artifact' }
  }), /not evidence eligible/);
});

test('Series 1 uses provider-native DeepSeek defaults', () => {
  assert.equal(SERIES1_PROVIDER, 'deepseek');
  assert.equal(SERIES1_DEFAULT_MODEL, 'deepseek-v4-flash');
  assert.equal(SERIES1_DEFAULT_JUDGE_MODEL, 'deepseek-v4-pro');
  assert.equal(DEEPSEEK_BASE_URL, 'https://api.deepseek.com');
});

test('held-constant comparison uses stable nested digests', () => {
  const records = ['classic','ql-direct','ql-deep'].map((condition) => ({
    condition,
    task_digest: stableDigest({ b: { y: 2, x: 1 }, a: 0 }),
    start_state_digest: 'same-start',
    capability_digest: stableDigest([{ id: 'read_file', args: { path: 'x' } }]),
    model: { provider: 'deepseek', id: 'deepseek-v4-flash', parameters: { temperature: 0 } }
  }));
  assert.deepEqual(compareHeldConstant(records), { task: true, start_state: true, model: true, capabilities: true });
  assert.equal(stableDigest({ a: { x: 1, y: 2 }, b: 3 }), stableDigest({ b: 3, a: { y: 2, x: 1 } }));
});

test('effect classification charges equal-quality QL for large token overhead', () => {
  assert.equal(classifyEffect({ classic: { quality_score: 1, total_tokens: 100 }, candidate: { quality_score: 1, total_tokens: 170 } }), 'degrades-efficiency');
  assert.equal(classifyEffect({ classic: { quality_score: 0.5, total_tokens: 100 }, candidate: { quality_score: 0.8, total_tokens: 300 } }), 'improves');
  assert.equal(classifyEffect({ classic: { quality_score: 0.9, total_tokens: 100 }, candidate: { quality_score: 0.7, total_tokens: 50 } }), 'degrades');
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

test('live response parser enforces the portable turn envelope', () => {
  assert.deepEqual(normalizeEnvelope('{"content":"done","capabilityCalls":[]}').capabilityCalls, []);
  assert.equal(parseJsonObject('prefix {"score":0.75} suffix').score, 0.75);
  assert.throws(() => parseJsonObject('not json'), /did not return a JSON object/);
});

test('task corpus includes semantic, trivial and multi-file artifact work with deterministic evaluators', async () => {
  assert.equal(SERIES1_TASKS.length, 8);
  assert.ok(SERIES1_TASKS.some((task) => task.category === 'chat'));
  assert.ok(SERIES1_TASKS.some((task) => task.anti_overengineering));
  const codeTasks = SERIES1_TASKS.filter((task) => task.quality.kind === 'artifact');
  assert.equal(codeTasks.length, 6);
  for (const task of codeTasks) {
    const root = await fs.mkdtemp(path.join(os.tmpdir(), `series1-${task.id}-`));
    try {
      await setupTask(task, root);
      const before = await evaluateTask(task, root);
      assert.ok(Number.isFinite(before.quality_score));
      assert.ok(before.quality_score >= 0 && before.quality_score <= 1);
    } finally {
      await fs.rm(root, { recursive: true, force: true });
    }
  }
});
