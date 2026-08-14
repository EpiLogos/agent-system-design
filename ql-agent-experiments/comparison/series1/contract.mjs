import crypto from 'node:crypto';

export const SERIES1_SCHEMA = 'ql-series1-run/0.1';
export const CONDITIONS = Object.freeze(['classic', 'ql-direct', 'ql-deep']);
export const HOSTS = Object.freeze(['pi', 'pydantic-ai', 'native']);

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
  if (!manifest?.held_constant?.task) errors.push('task equality must be evidenced');
  if (!manifest?.held_constant?.start_state) errors.push('start-state equality must be evidenced');
  if (!manifest?.held_constant?.model) errors.push('model equality must be evidenced');
  if (!manifest?.held_constant?.capabilities) errors.push('capability equality must be evidenced');
  if (manifest?.fixture_provider === true) errors.push('fixture providers are not evidence eligible');
  if (manifest?.quality?.kind === 'semantic' && !manifest?.quality?.judge_model) {
    errors.push('semantic/chat evidence requires a configured judge_model');
  }
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
    capabilities: values((record) => record.capability_digest).size === 1
  };
}

export function classifyEffect({ classic, candidate }) {
  if (!Number.isFinite(classic?.quality_score) || !Number.isFinite(candidate?.quality_score)) return 'unscored';
  const qualityDelta = candidate.quality_score - classic.quality_score;
  const costRatio = classic.total_tokens > 0 ? candidate.total_tokens / classic.total_tokens : null;
  if (qualityDelta < -0.05) return 'degrades';
  if (qualityDelta > 0.05) return 'improves';
  if (costRatio !== null && costRatio > 1.5) return 'degrades-efficiency';
  return 'no-material-effect';
}
