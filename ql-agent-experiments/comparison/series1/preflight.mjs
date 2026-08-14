import { HOSTS } from './contract.mjs';
import { providerForHost } from './providers.mjs';
import { SERIES1_TASKS } from './tasks.mjs';

function cli() {
  const values = process.argv.slice(2);
  const live = values.includes('--live');
  const hostIndex = values.indexOf('--host');
  const host = hostIndex >= 0 ? values[hostIndex + 1] : null;
  return { live, hosts: host ? [host] : [...HOSTS] };
}

function checkConfiguration() {
  const model = process.env.QL_SERIES1_MODEL ?? null;
  const provider = process.env.QL_SERIES1_PROVIDER ?? 'openai';
  const piModel = process.env.QL_SERIES1_PI_MODEL ?? model;
  const piProvider = process.env.QL_SERIES1_PI_PROVIDER ?? 'openai';
  const pydanticModel = process.env.QL_SERIES1_PYDANTIC_MODEL ?? (model ? `${provider}:${model}` : null);
  const judgeModel = process.env.QL_SERIES1_JUDGE_MODEL ?? null;
  const semanticTasks = SERIES1_TASKS.filter((task) => task.quality.kind === 'semantic').map((task) => task.id);
  const errors = [];
  if (!model) errors.push('QL_SERIES1_MODEL is missing');
  if (piModel && model && piModel !== model) errors.push('Pi model override differs from QL_SERIES1_MODEL; matched-model evidence would be invalid');
  if (piProvider !== provider) errors.push('Pi provider override differs from QL_SERIES1_PROVIDER; matched-provider evidence would be invalid');
  if (pydanticModel && model && pydanticModel !== `${provider}:${model}`) errors.push(`Pydantic model must be ${provider}:${model} for matched evidence`);
  if (!process.env.QL_SERIES1_API_KEY && provider === 'openai') errors.push('QL_SERIES1_API_KEY is missing');
  if (semanticTasks.length && !judgeModel) errors.push(`QL_SERIES1_JUDGE_MODEL is missing; semantic tasks ${semanticTasks.join(', ')} cannot be scored`);
  if (judgeModel && model && judgeModel === model) errors.push('QL_SERIES1_JUDGE_MODEL must differ from QL_SERIES1_MODEL');
  return { provider, model, judgeModel, semanticTasks, errors };
}

async function main() {
  const { live, hosts } = cli();
  for (const host of hosts) {
    if (!HOSTS.includes(host)) throw new Error(`Unknown host '${host}'.`);
  }
  const config = checkConfiguration();
  const result = {
    schema: 'ql-series1-preflight/0.1',
    live_requested: live,
    hosts,
    model: config.model,
    provider: config.provider,
    judge_model: config.judgeModel,
    structural: {
      task_count: SERIES1_TASKS.length,
      semantic_task_count: config.semanticTasks.length,
      artifact_task_count: SERIES1_TASKS.length - config.semanticTasks.length,
      conditions: ['classic', 'ql-direct', 'ql-deep'],
      fixture_fallback: false
    },
    configuration_errors: config.errors,
    host_checks: []
  };

  if (live) {
    for (const host of hosts) {
      try {
        const provider = providerForHost(host);
        await provider.assertReady();
        result.host_checks.push({ host, ready: true, provider_path: provider.id });
      } catch (error) {
        result.host_checks.push({ host, ready: false, error: error.message });
      }
    }
  }

  const hostFailures = result.host_checks.filter((entry) => !entry.ready);
  result.evidence_ready = live && config.errors.length === 0 && hostFailures.length === 0;
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
  if (live && !result.evidence_ready) process.exitCode = 1;
}

main().catch((error) => {
  console.error(error.stack ?? error.message ?? String(error));
  process.exitCode = 1;
});
