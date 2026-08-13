export const RUN_STATUS = Object.freeze({
  COMPLETED: 'completed',
  FAILED: 'failed',
  CANCELLED: 'cancelled',
  EXHAUSTED: 'exhausted'
});

export class RuntimeContractError extends Error {
  constructor(message, details = {}) {
    super(message);
    this.name = 'RuntimeContractError';
    this.details = details;
  }
}

export function assertRuntime(runtime) {
  if (!runtime || typeof runtime.run !== 'function') {
    throw new RuntimeContractError('LoopRuntime must expose run(request, host, observer, signal).');
  }
  if (typeof runtime.id !== 'string' || runtime.id.length === 0) {
    throw new RuntimeContractError('LoopRuntime must expose a non-empty id.');
  }
  if (typeof runtime.version !== 'string' || runtime.version.length === 0) {
    throw new RuntimeContractError('LoopRuntime must expose a non-empty version.');
  }
  return runtime;
}

export function assertHost(host) {
  const required = [
    'callModel',
    'executeCapability',
    'receiveExternalInput',
    'readContext'
  ];
  for (const method of required) {
    if (typeof host?.[method] !== 'function') {
      throw new RuntimeContractError(`RuntimeHost must expose ${method}().`);
    }
  }
  return host;
}

export function assertObserver(observer) {
  if (!observer || typeof observer.emit !== 'function') {
    throw new RuntimeContractError('RuntimeObserver must expose emit(event).');
  }
  return observer;
}

export function normalizeRunRequest(request = {}) {
  const id = request.id ?? request.taskId ?? 'task';
  return Object.freeze({
    ...request,
    id,
    taskId: request.taskId ?? id,
    input: request.input ?? null,
    successConditions: Array.isArray(request.successConditions)
      ? [...request.successConditions]
      : [],
    maxSteps: Number.isInteger(request.maxSteps) && request.maxSteps > 0
      ? request.maxSteps
      : 64
  });
}

export function isAbortRequested(signal) {
  return Boolean(signal?.aborted);
}

export class RuntimeRegistry {
  #runtimes = new Map();

  register(runtime) {
    assertRuntime(runtime);
    if (this.#runtimes.has(runtime.id)) {
      throw new RuntimeContractError(`Runtime '${runtime.id}' is already registered.`);
    }
    this.#runtimes.set(runtime.id, runtime);
    return this;
  }

  get(runtimeId) {
    const runtime = this.#runtimes.get(runtimeId);
    if (!runtime) {
      throw new RuntimeContractError(`Unknown runtime '${runtimeId}'.`);
    }
    return runtime;
  }

  list() {
    return [...this.#runtimes.values()].map(({ id, version }) => ({ id, version }));
  }

  async run(runtimeId, request, host, observer, signal) {
    const runtime = this.get(runtimeId);
    assertHost(host);
    assertObserver(observer);
    return runtime.run(normalizeRunRequest(request), host, observer, signal);
  }
}
