import crypto from 'node:crypto';

export const SERIES1_SCHEMA = 'ql-series1-run/0.2';
export const CONDITIONS = Object.freeze(['classic', 'ql-direct', 'ql-deep']);
export const HOSTS = Object.freeze(['pi', 'pydantic-ai', 'native']);
export const DETERMINATION = 'pending-human-review';

function canonicalize(value) {
  if (Array.isArray(value)) return value.map(canonicalize);
  if (value && typeof value === 'object') {
    return Object.fromEntries(Object.keys(value).sort().map((key) => [key, canonicalize(value[key])]));
  }
  return value;
}

export function stableDigest(value) {
  return crypto.createHash('sha256').update(JSON.stringify(canonicalize(value))).digest('hex');
}

export function assertLiveManifest(manifest) {
  const errors = [];
  if (manifest?.schema !== SERIES1_SCHEMA) errors.push(`schema must be ${SERIES1_SCHEMA}`);
  if (manifest?.provider_mode !== 'live') errors.push('provider_mode must be live');
  if (!HOSTS.includes(manifest?.host?.id)) errors.push('host.id must name pi, pydantic-ai, or native');
  if (!manifest?.host?.real_framework_path) errors.push('host.real_framework_path is required');
  if (!manifest?.host?.revision) errors.push('host.revision is required');
  if (!manifest?.model?.id || !manifest?.model?.provider) errors.push('concrete model provider/id is required');
  if (!Array.isArray(manifest?.conditions) || !CONDITIONS.every((condition) => manifest.conditions.includes(condition))) {
    errors.push('classic, ql-direct, and ql-deep conditions are required');
  }
  if (!manifest?.held_constant?.task) errors.push('task/prompt equality must be evidenced');
  if (!manifest?.held_constant?.start_state) errors.push('start-state equality must be evidenced');
  if (!manifest?.held_constant?.model) errors.push('model equality must be evidenced');
  if (!manifest?.held_constant?.capabilities) errors.push('capability equality must be evidenced');
  if (!manifest?.held_constant?.verification) errors.push('verification-protocol equality must be evidenced');
  if (!manifest?.held_constant?.budget) errors.push('execution-budget equality must be evidenced');
  if (manifest?.fixture_provider === true) errors.push('fixture providers are not evidence eligible');
  if (manifest?.determination !== DETERMINATION) errors.push(`determination must begin as ${DETERMINATION}`);
  if (!manifest?.review?.prompt || !Array.isArray(manifest?.review?.focus)) errors.push('human review prompt/focus must be retained');
  if (!Array.isArray(manifest?.records) || manifest.records.length === 0) errors.push('run records are required');
  if (errors.length) {
    const error = new Error(`Series 1 manifest is not evidence eligible:\n- ${errors.join('\n- ')}`);
    error.code = 'SERIES1_NOT_ELIGIBLE';
    error.errors = errors;
    throw error;
  }
  return manifest;
}

export function compareHeldConstant(records) {
  const values = (selector) => new Set(records.map(selector));
  return {
    task: values((record) => record.task_digest).size === 1,
    start_state: values((record) => record.start_state_digest).size === 1,
    model: values((record) => `${record.model.provider}:${record.model.id}:${JSON.stringify(canonicalize(record.model.parameters ?? {}))}`).size === 1,
    capabilities: values((record) => record.capability_digest).size === 1,
    verification: values((record) => record.verification_protocol_digest).size === 1,
    budget: values((record) => record.execution_budget_digest).size === 1
  };
}
