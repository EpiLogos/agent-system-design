import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';

function parseJsonObject(text) {
  const value = String(text ?? '').trim();
  try {
    return JSON.parse(value);
  } catch {}
  const start = value.indexOf('{');
  const end = value.lastIndexOf('}');
  if (start >= 0 && end > start) return JSON.parse(value.slice(start, end + 1));
  throw new Error(`Model did not return a JSON object: ${value.slice(0, 300)}`);
}

function normalizeEnvelope(raw) {
  const parsed = typeof raw === 'string' ? parseJsonObject(raw) : raw;
  return {
    content: parsed?.content ?? '',
    capabilityCalls: Array.isArray(parsed?.capabilityCalls) ? parsed.capabilityCalls.map((call, index) => ({
      id: call.id ?? `call-${index + 1}`,
      name: call.name,
      args: call.args ?? {}
    })) : [],
    usage: parsed?.usage ?? null,
    raw: parsed?.raw ?? null
  };
}

export const LIVE_RESPONSE_SYSTEM = `You are an execution model inside a controlled agent-loop experiment.
Return exactly one JSON object and no prose outside it:
{"content":"assistant text","capabilityCalls":[{"id":"optional","name":"capability_name","args":{}}]}
Use capabilityCalls only when exterior work is needed. If no capability is needed, return an empty array.`;

export class NativeOpenAICompatibleProvider {
  constructor({ baseUrl = process.env.QL_SERIES1_BASE_URL ?? 'https://api.openai.com/v1', apiKey = process.env.QL_SERIES1_API_KEY, model = process.env.QL_SERIES1_MODEL } = {}) {
    this.id = 'native-openai-compatible';
    this.baseUrl = baseUrl.replace(/\/$/, '');
    this.apiKey = apiKey;
    this.model = model;
  }

  assertReady() {
    if (!this.apiKey) throw new Error('QL_SERIES1_API_KEY is required for live Native runs.');
    if (!this.model) throw new Error('QL_SERIES1_MODEL is required for live Native runs.');
  }

  async complete({ system = LIVE_RESPONSE_SYSTEM, prompt, temperature = 0, signal } = {}) {
    this.assertReady();
    const response = await fetch(`${this.baseUrl}/chat/completions`, {
      method: 'POST',
      headers: { 'content-type': 'application/json', authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({
        model: this.model,
        temperature,
        messages: [
          { role: 'system', content: system },
          { role: 'user', content: prompt }
        ]
      }),
      signal
    });
    if (!response.ok) throw new Error(`Native model HTTP ${response.status}: ${await response.text()}`);
    const body = await response.json();
    const text = body?.choices?.[0]?.message?.content ?? '';
    const envelope = normalizeEnvelope(text);
    envelope.usage = {
      input_tokens: body?.usage?.prompt_tokens ?? 0,
      output_tokens: body?.usage?.completion_tokens ?? 0,
      total_tokens: body?.usage?.total_tokens ?? 0
    };
    envelope.raw = { finish_reason: body?.choices?.[0]?.finish_reason ?? null };
    return envelope;
  }
}

export class PiAIProvider {
  constructor({ provider = process.env.QL_SERIES1_PI_PROVIDER ?? 'openai', model = process.env.QL_SERIES1_PI_MODEL ?? process.env.QL_SERIES1_MODEL } = {}) {
    this.id = 'pi-ai';
    this.provider = provider;
    this.model = model;
    this.models = null;
  }

  async #load() {
    if (this.models) return;
    let module;
    try {
      module = await import('@earendil-works/pi-ai/providers/all');
    } catch (error) {
      throw new Error(`Real Pi provider unavailable. Install @earendil-works/pi-ai@0.84.1. ${error.message}`);
    }
    this.models = module.builtinModels();
  }

  async assertReady() {
    await this.#load();
    if (!this.model) throw new Error('QL_SERIES1_PI_MODEL or QL_SERIES1_MODEL is required.');
    const model = this.models.getModel(this.provider, this.model);
    if (!model) throw new Error(`Pi model '${this.provider}:${this.model}' is not present in the pinned Pi catalog.`);
    const auth = await this.models.getAuth(model);
    if (!auth) throw new Error(`Pi provider '${this.provider}' has no live credentials.`);
  }

  async complete({ system = LIVE_RESPONSE_SYSTEM, prompt, signal } = {}) {
    await this.#load();
    const model = this.models.getModel(this.provider, this.model);
    if (!model) throw new Error(`Unknown Pi model '${this.provider}:${this.model}'.`);
    const context = {
      systemPrompt: system,
      messages: [{ role: 'user', content: prompt, timestamp: Date.now() }],
      tools: []
    };
    const response = await this.models.complete(model, context, { signal });
    const text = (response.content ?? []).filter((block) => block.type === 'text').map((block) => block.text).join('\n');
    const envelope = normalizeEnvelope(text);
    envelope.usage = {
      input_tokens: response?.usage?.input ?? 0,
      output_tokens: response?.usage?.output ?? 0,
      total_tokens: response?.usage?.totalTokens ?? ((response?.usage?.input ?? 0) + (response?.usage?.output ?? 0)),
      cost: response?.usage?.cost?.total ?? null
    };
    envelope.raw = { model: response?.model ?? this.model, provider: this.provider, stopReason: response?.stopReason ?? null };
    return envelope;
  }
}

function runPythonBridge(payload, { signal } = {}) {
  const bridge = fileURLToPath(new URL('./pydantic_bridge.py', import.meta.url));
  return new Promise((resolve, reject) => {
    const child = spawn(process.env.PYTHON ?? 'python3', [bridge], { stdio: ['pipe', 'pipe', 'pipe'], env: process.env });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => { stdout += chunk; });
    child.stderr.on('data', (chunk) => { stderr += chunk; });
    child.on('error', reject);
    child.on('close', (code) => {
      if (code !== 0) return reject(new Error(`Pydantic bridge exited ${code}: ${stderr || stdout}`));
      try { resolve(JSON.parse(stdout)); } catch (error) { reject(new Error(`Invalid Pydantic bridge output: ${stdout}\n${error.message}`)); }
    });
    if (signal) {
      if (signal.aborted) child.kill('SIGTERM');
      signal.addEventListener('abort', () => child.kill('SIGTERM'), { once: true });
    }
    child.stdin.end(JSON.stringify(payload));
  });
}

export class PydanticAIProvider {
  constructor({ model = process.env.QL_SERIES1_PYDANTIC_MODEL ?? `openai:${process.env.QL_SERIES1_MODEL ?? ''}` } = {}) {
    this.id = 'pydantic-ai';
    this.model = model;
  }

  async assertReady() {
    const result = await runPythonBridge({ operation: 'preflight', model: this.model });
    if (!result.ready) throw new Error(result.error ?? 'Pydantic AI bridge is not ready.');
  }

  async complete({ system = LIVE_RESPONSE_SYSTEM, prompt, signal } = {}) {
    const result = await runPythonBridge({ operation: 'complete', model: this.model, system, prompt }, { signal });
    const envelope = normalizeEnvelope(result.output);
    envelope.usage = result.usage ?? null;
    envelope.raw = { model: result.model_name ?? this.model, framework: 'pydantic-ai' };
    return envelope;
  }
}

export function providerForHost(hostId, options = {}) {
  if (hostId === 'pi') return new PiAIProvider(options.pi);
  if (hostId === 'pydantic-ai') return new PydanticAIProvider(options.pydantic);
  if (hostId === 'native') return new NativeOpenAICompatibleProvider(options.native);
  throw new Error(`Unknown live host '${hostId}'.`);
}

export { normalizeEnvelope, parseJsonObject };
