import fs from 'node:fs/promises';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { LIVE_RESPONSE_SYSTEM } from './providers.mjs';

const clone = (value) => value === undefined ? undefined : structuredClone(value);

function ensureInside(root, relativePath = '.') {
  const resolved = path.resolve(root, relativePath);
  const prefix = `${path.resolve(root)}${path.sep}`;
  if (resolved !== path.resolve(root) && !resolved.startsWith(prefix)) {
    throw new Error(`Path escapes Series 1 workspace: ${relativePath}`);
  }
  return resolved;
}

async function runNodeTests(root, args = []) {
  const testArgs = ['--test', ...args];
  return new Promise((resolve) => {
    const child = spawn(process.execPath, testArgs, { cwd: root, stdio: ['ignore', 'pipe', 'pipe'] });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => { stdout += chunk; });
    child.stderr.on('data', (chunk) => { stderr += chunk; });
    child.on('close', (code) => resolve({ ok: code === 0, exit_code: code, stdout, stderr }));
  });
}

export class Series1Workspace {
  constructor(root) {
    this.root = path.resolve(root);
  }

  list() {
    return ['list_files', 'read_file', 'write_file', 'run_tests'];
  }

  async execute(name, args = {}) {
    if (name === 'list_files') {
      const dir = ensureInside(this.root, args.path ?? '.');
      const entries = await fs.readdir(dir, { withFileTypes: true });
      return { ok: true, entries: entries.map((entry) => ({ name: entry.name, type: entry.isDirectory() ? 'directory' : 'file' })) };
    }
    if (name === 'read_file') {
      const file = ensureInside(this.root, args.path);
      return { ok: true, path: args.path, content: await fs.readFile(file, 'utf8') };
    }
    if (name === 'write_file') {
      const file = ensureInside(this.root, args.path);
      await fs.mkdir(path.dirname(file), { recursive: true });
      await fs.writeFile(file, String(args.content ?? ''), 'utf8');
      return { ok: true, path: args.path, bytes: Buffer.byteLength(String(args.content ?? '')) };
    }
    if (name === 'run_tests') {
      const allowed = Array.isArray(args.files) ? args.files.map((file) => ensureInside(this.root, file).slice(this.root.length + 1)) : [];
      return runNodeTests(this.root, allowed);
    }
    throw new Error(`Unknown Series 1 capability '${name}'.`);
  }
}

function historyPrompt(history, request, capabilities) {
  return JSON.stringify({
    task: request.input,
    success_conditions: request.successConditions,
    capabilities,
    history
  }, null, 2);
}

export class LiveRuntimeHost {
  constructor({ id, revision, realFrameworkPath, provider, workspace }) {
    this.id = id;
    this.revision = revision;
    this.realFrameworkPath = realFrameworkPath;
    this.provider = provider;
    this.workspace = workspace;
    this.observer = null;
    this.runId = null;
    this.sequence = 0;
    this.usage = { model_calls: 0, input_tokens: 0, output_tokens: 0, total_tokens: 0, model_cost: 0 };
  }

  attachObserver(observer, runId) {
    this.observer = observer;
    this.runId = runId;
  }

  emit(eventType, payload) {
    this.observer?.emit({
      channel: 'host',
      event_id: `${this.runId}:${this.id}:live:${this.sequence}`,
      event_type: eventType,
      run_id: this.runId,
      sequence: this.sequence++,
      host: this.id,
      payload: clone(payload)
    });
  }

  absorbUsage(usage) {
    this.usage.model_calls += 1;
    this.usage.input_tokens += usage?.input_tokens ?? 0;
    this.usage.output_tokens += usage?.output_tokens ?? 0;
    this.usage.total_tokens += usage?.total_tokens ?? ((usage?.input_tokens ?? 0) + (usage?.output_tokens ?? 0));
    this.usage.model_cost += Number(usage?.cost ?? 0) || 0;
  }

  async callModel(payload = {}) {
    let system = LIVE_RESPONSE_SYSTEM;
    let prompt;

    if (payload.series1Control) {
      system = payload.series1Control.system;
      prompt = payload.series1Control.prompt;
    } else if (payload.history) {
      prompt = historyPrompt(payload.history, payload.request, this.workspace.list());
    } else if (payload.qlAct) {
      system = `${LIVE_RESPONSE_SYSTEM}\nFor this QL act, perform only the stated intent. Do not invent a capability call unless the act itself asks for one.`;
      prompt = JSON.stringify({
        task: payload.request?.input,
        success_conditions: payload.request?.successConditions,
        ql_act: payload.qlAct
      }, null, 2);
    } else {
      prompt = JSON.stringify(payload.request?.input ?? payload, null, 2);
    }

    this.emit('model_requested', { provider: this.provider.id, purpose: payload.series1Control?.purpose ?? (payload.qlAct ? 'ql-act' : 'classic-turn') });
    const result = await this.provider.complete({ system, prompt, signal: payload.signal });
    this.absorbUsage(result.usage);
    this.emit('model_returned', { provider: this.provider.id, usage: result.usage, capability_call_count: result.capabilityCalls.length });
    return result;
  }

  async executeCapability({ name, args } = {}) {
    this.emit('capability_requested', { name, args: clone(args ?? {}) });
    try {
      const result = await this.workspace.execute(name, args ?? {});
      this.emit('capability_returned', { name, ok: result?.ok !== false });
      return result;
    } catch (error) {
      this.emit('capability_returned', { name, ok: false, error: error.message });
      throw error;
    }
  }

  async receiveExternalInput() { return null; }

  async readContext({ kind, input } = {}) {
    if (kind === 'environment' || kind === 'artifact') {
      return { root: this.workspace.root, input: clone(input), capabilities: this.workspace.list() };
    }
    return { kind, input: clone(input), capabilities: this.workspace.list() };
  }

  snapshotUsage() { return clone(this.usage); }
}

export function createLiveHost({ hostId, provider, workspace }) {
  const table = {
    pi: {
      revision: 'earendil-works/pi@9d2ec7ffabe927bfad2214c1cee25b6632a78dcf',
      path: '@earendil-works/pi-ai@0.84.1'
    },
    'pydantic-ai': {
      revision: 'pydantic/pydantic-ai@00db3a4b391eb9a46f3d6e704070bcf725121f75',
      path: 'pydantic_ai.Agent via pinned-source Python environment'
    },
    native: {
      revision: 'native-series1-host-v1',
      path: 'repo-owned OpenAI-compatible HTTP transport'
    }
  };
  const config = table[hostId];
  if (!config) throw new Error(`Unknown Series 1 host '${hostId}'.`);
  return new LiveRuntimeHost({ id: hostId, revision: config.revision, realFrameworkPath: config.path, provider, workspace });
}
