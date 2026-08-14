import fs from 'node:fs/promises';
import path from 'node:path';
import { spawn } from 'node:child_process';

async function write(root, relativePath, content) {
  const file = path.join(root, relativePath);
  await fs.mkdir(path.dirname(file), { recursive: true });
  await fs.writeFile(file, content, 'utf8');
}

async function read(root, relativePath) {
  return fs.readFile(path.join(root, relativePath), 'utf8');
}

function run(root, command, args) {
  return new Promise((resolve) => {
    const child = spawn(command, args, { cwd: root, stdio: ['ignore', 'pipe', 'pipe'] });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => { stdout += chunk; });
    child.stderr.on('data', (chunk) => { stderr += chunk; });
    child.on('close', (code) => resolve({ ok: code === 0, code, stdout, stderr }));
  });
}

async function nodeTests(root, ...files) {
  return run(root, process.execPath, ['--test', ...files]);
}

const noSetup = async () => {};
const semantic = (rubric) => ({ kind: 'semantic', rubric });
const artifact = () => ({ kind: 'artifact' });

export const SERIES1_TASKS = Object.freeze([
  {
    id: 'S1-CHAT-001',
    category: 'chat',
    anti_overengineering: true,
    prompt: 'In two short paragraphs, explain why a failing unit test is evidence about software behaviour rather than automatically evidence that the intended requirement is wrong.',
    successConditions: [
      'Answer the distinction directly.',
      'Do not overcomplicate the response.',
      'Distinguish implementation behaviour from requirement validity.'
    ],
    setup: noSetup,
    quality: semantic('Score correctness, directness, clarity, and unnecessary complexity. A concise correct answer should beat a ceremonially elaborate one.')
  },
  {
    id: 'S1-CHAT-002',
    category: 'chat',
    prompt: 'A user asks: “make the export safer, but do not change the public API and do not add a dependency.” State what you would need to preserve and the first concrete thing you would inspect before editing.',
    successConditions: [
      'Preserve the public API.',
      'Add no dependency.',
      'Identify a concrete first inspection step.',
      'Do not invent repository facts.'
    ],
    setup: noSetup,
    quality: semantic('Score intent preservation, handling of ambiguity, concrete next action, restraint, and absence of invented facts.')
  },
  {
    id: 'S1-CODE-001',
    category: 'code',
    anti_overengineering: true,
    prompt: 'Fix the implementation so `add(2, 3)` returns 5. Make the smallest appropriate change and leave the exported API unchanged. Run the tests.',
    successConditions: ['All tests pass.', 'The export remains named add.', 'Avoid unrelated changes.'],
    quality: artifact(),
    async setup(root) {
      await write(root, 'math.js', "export function add(a, b) {\n  return a - b;\n}\n");
      await write(root, 'math.test.js', "import test from 'node:test';\nimport assert from 'node:assert/strict';\nimport { add } from './math.js';\ntest('add',()=>assert.equal(add(2,3),5));\n");
      await write(root, 'package.json', '{"type":"module"}\n');
    },
    async evaluate(root) {
      const tests = await nodeTests(root, 'math.test.js');
      const source = await read(root, 'math.js');
      const api = /export\s+function\s+add\s*\(/.test(source);
      const small = source.split('\n').length <= 6;
      return { quality_score: (Number(tests.ok) * 0.8) + (Number(api) * 0.15) + (Number(small) * 0.05), tests, checks: { api, small } };
    }
  },
  {
    id: 'S1-CODE-002',
    category: 'code',
    prompt: 'Fix `slugify` so it trims leading/trailing whitespace, lowercases text, collapses any run of non-alphanumeric characters to one hyphen, and never leaves a leading or trailing hyphen. Preserve the function signature and run the tests.',
    successConditions: ['All slugify tests pass.', 'No dependency is added.', 'Signature remains slugify(value).'],
    quality: artifact(),
    async setup(root) {
      await write(root, 'slug.js', "export function slugify(value) {\n  return value.toLowerCase().replace(' ', '-');\n}\n");
      await write(root, 'slug.test.js', "import test from 'node:test';\nimport assert from 'node:assert/strict';\nimport { slugify } from './slug.js';\ntest('slugify',()=>{\n assert.equal(slugify('  Hello,   World!  '),'hello-world');\n assert.equal(slugify('A__B---C'),'a-b-c');\n assert.equal(slugify('***x***'),'x');\n});\n");
      await write(root, 'package.json', '{"type":"module"}\n');
    },
    async evaluate(root) {
      const tests = await nodeTests(root, 'slug.test.js');
      const source = await read(root, 'slug.js');
      const dependencyFree = !source.includes(' from ') && !source.includes('require(');
      return { quality_score: (Number(tests.ok) * 0.9) + (Number(dependencyFree) * 0.1), tests, checks: { dependencyFree } };
    }
  },
  {
    id: 'S1-CODE-003',
    category: 'code',
    prompt: 'Implement `summariseOrders` using the existing modules. It must return `{count,total,average}` for valid numeric order amounts, ignore malformed amounts, and keep the existing `parseAmount` and `summariseOrders` exports. Run the tests.',
    successConditions: ['All tests pass.', 'Malformed amounts are ignored.', 'Existing exports remain available.', 'Do not add dependencies.'],
    quality: artifact(),
    async setup(root) {
      await write(root, 'parse.js', "export function parseAmount(order) {\n  const value = Number(order?.amount);\n  return Number.isFinite(value) ? value : null;\n}\n");
      await write(root, 'summary.js', "import { parseAmount } from './parse.js';\nexport function summariseOrders(orders) {\n  // TODO\n  return { count: 0, total: 0, average: 0 };\n}\nexport { parseAmount };\n");
      await write(root, 'summary.test.js', "import test from 'node:test';\nimport assert from 'node:assert/strict';\nimport { summariseOrders, parseAmount } from './summary.js';\ntest('summary',()=>{\n assert.equal(parseAmount({amount:'2.5'}),2.5);\n assert.deepEqual(summariseOrders([{amount:'2.5'},{amount:3},{amount:'bad'},{}]),{count:2,total:5.5,average:2.75});\n assert.deepEqual(summariseOrders([]),{count:0,total:0,average:0});\n});\n");
      await write(root, 'package.json', '{"type":"module"}\n');
    },
    async evaluate(root) {
      const tests = await nodeTests(root, 'summary.test.js');
      const summary = await read(root, 'summary.js');
      const preserved = /export\s+function\s+summariseOrders/.test(summary) && /export\s*\{\s*parseAmount\s*\}/.test(summary);
      return { quality_score: (Number(tests.ok) * 0.9) + (Number(preserved) * 0.1), tests, checks: { preserved } };
    }
  },
  {
    id: 'S1-CODE-004',
    category: 'code',
    prompt: 'The cache tests expose stale data after `set`. Fix the cache. Do not change the exported names or test files. Inspect before editing and run the tests after the change.',
    successConditions: ['All cache tests pass.', 'Exports remain createCache and normaliseKey.', 'The stale-data cause is actually fixed.'],
    quality: artifact(),
    async setup(root) {
      await write(root, 'keys.js', "export const normaliseKey = (key) => String(key).trim().toLowerCase();\n");
      await write(root, 'cache.js', "import { normaliseKey } from './keys.js';\nexport function createCache() {\n const values = new Map();\n let last = null;\n return {\n  get(key) { const k=normaliseKey(key); if(last?.key===k) return last.value; const value=values.get(k); last={key:k,value}; return value; },\n  set(key,value) { values.set(normaliseKey(key),value); },\n };\n}\nexport { normaliseKey };\n");
      await write(root, 'cache.test.js', "import test from 'node:test';\nimport assert from 'node:assert/strict';\nimport { createCache, normaliseKey } from './cache.js';\ntest('normalises',()=>assert.equal(normaliseKey(' X '),'x'));\ntest('set invalidates stale last read',()=>{ const c=createCache(); c.set('A',1); assert.equal(c.get('a'),1); c.set(' A ',2); assert.equal(c.get('a'),2); });\n");
      await write(root, 'package.json', '{"type":"module"}\n');
    },
    async evaluate(root) {
      const tests = await nodeTests(root, 'cache.test.js');
      const source = await read(root, 'cache.js');
      const preserved = source.includes('createCache') && source.includes('normaliseKey');
      return { quality_score: (Number(tests.ok) * 0.9) + (Number(preserved) * 0.1), tests, checks: { preserved } };
    }
  },
  {
    id: 'S1-CODE-005',
    category: 'code',
    prompt: 'Harden `safeJson` without changing its API: it must return `{ok:true,value}` for valid JSON and `{ok:false,error}` for invalid JSON, where error is a short string rather than an Error object. Do not throw for invalid input. Run the tests.',
    successConditions: ['All tests pass.', 'safeJson remains the only required API.', 'Invalid JSON does not throw.', 'No dependency is added.'],
    quality: artifact(),
    async setup(root) {
      await write(root, 'safe-json.js', "export function safeJson(text) {\n  return { ok: true, value: JSON.parse(text) };\n}\n");
      await write(root, 'safe-json.test.js', "import test from 'node:test';\nimport assert from 'node:assert/strict';\nimport { safeJson } from './safe-json.js';\ntest('valid',()=>assert.deepEqual(safeJson('{\"x\":1}'),{ok:true,value:{x:1}}));\ntest('invalid',()=>{ const r=safeJson('{x'); assert.equal(r.ok,false); assert.equal(typeof r.error,'string'); assert.ok(r.error.length>0 && r.error.length<200); });\n");
      await write(root, 'package.json', '{"type":"module"}\n');
    },
    async evaluate(root) {
      const tests = await nodeTests(root, 'safe-json.test.js');
      return { quality_score: Number(tests.ok), tests, checks: {} };
    }
  },
  {
    id: 'S1-CODE-006',
    category: 'code',
    prompt: 'Implement `renderGreeting` so it uses `formatName` from `name.js`, returns `Hello, NAME!`, and falls back to `friend` when the formatted name is empty. Keep both modules and exports intact. Run the tests; if your first implementation fails, use the failure as evidence and correct it.',
    successConditions: ['All tests pass.', 'renderGreeting imports and uses formatName.', 'Existing exports remain intact.'],
    quality: artifact(),
    async setup(root) {
      await write(root, 'name.js', "export function formatName(value) { return String(value ?? '').trim(); }\n");
      await write(root, 'greeting.js', "export function renderGreeting(name) {\n  return `Hi ${name}`;\n}\n");
      await write(root, 'greeting.test.js', "import test from 'node:test';\nimport assert from 'node:assert/strict';\nimport { renderGreeting } from './greeting.js';\ntest('name',()=>assert.equal(renderGreeting('  Ada  '),'Hello, Ada!'));\ntest('fallback',()=>assert.equal(renderGreeting('   '),'Hello, friend!'));\n");
      await write(root, 'package.json', '{"type":"module"}\n');
    },
    async evaluate(root) {
      const tests = await nodeTests(root, 'greeting.test.js');
      const source = await read(root, 'greeting.js');
      const usesHelper = /from\s+['"]\.\/name\.js['"]/.test(source) && /formatName\s*\(/.test(source);
      return { quality_score: (Number(tests.ok) * 0.9) + (Number(usesHelper) * 0.1), tests, checks: { usesHelper } };
    }
  }
]);

export function getTask(id) {
  const task = SERIES1_TASKS.find((entry) => entry.id === id);
  if (!task) throw new Error(`Unknown Series 1 task '${id}'.`);
  return task;
}

export async function setupTask(task, root) {
  await fs.mkdir(root, { recursive: true });
  await task.setup(root);
}

export async function evaluateTask(task, root) {
  if (task.quality.kind === 'artifact') return task.evaluate(root);
  return { quality_score: null, rubric: task.quality.rubric, semantic: true };
}
