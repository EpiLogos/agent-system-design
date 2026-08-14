import { HOSTS } from './contract.mjs';
import { providerForHost } from './providers.mjs';
import { SERIES1_TASKS, getTask } from './tasks.mjs';

function cli() {
  const values = process.argv.slice(2);
  const live = values.includes('--live');
  const hostIndex = values.indexOf('--host');
  const taskIndex = values.indexOf('--task');
  const host = hostIndex >= 0 ? values[hostIndex + 1] : null;
  const task = taskIndex >= 0 ? values[taskIndex + 1] : null;
  return { live, hosts: host ? [host] : [...HOSTS], task };
}

function checkConfiguration(selectedTask) {
  const model = process.env.QL_SERIES1_MODEL ?? null;
  const provider = process.env.QL_SERIES1_PROVIDER ?? 'openai';
  const piModel = process.env.QL_SERIES1_PI_MODEL ?? model;
  const piProvider = process.env.QL_SERIES1_PI_PROVIDER ?? 'openai';
  const pydanticModel = process.env.QL_SERIES1_PYDANTIC_MODEL ?? (model ? `${provider}:${model}` : null);
  const judgeModel = process.env.QL_SERIES1_JUDGE_MODEL ?? null;
  const semanticTasks = SERIES1_TASKS.filter((task) => task.quality.kind === 'semantic').map((task) => task.id);
  const semanticRequired = selectedTask ? getTask(selectedTask).quality.kind === 'semantic' : semanticTasks.length > 0;
  const errors = [];
  if (!model) errors.push('QL_SERIES1_MODEL is missing');
  if (piModel && model && piModel !== model) errors.push('Pi model override differs from QL_SERIES1_MODEL; matched-model evidence would be invalid');
  if (piProvider !== provider) errors.push('Pi provider override differs from QL_SERIES1_PROVIDER; matched-provider evidence would be invalid');
  if (pydanticModel && model && pydanticModel !== `${provider}:${model}`) errors.push(`Pydantic model must be ${provider}:${model} for matched evidence`);
  if (!process.env.QL_SERIES1_API_KEY && provider === 'openai') errors.push('QL_SERIES1_API_KEY is missing');
  if (semanticRequired && !judgeModel) errors.push(`QL_SERIES1_JUDGE_MODEL is missing; ${selectedTask ? `selected semantic task ${selectedTask}` : `semantic tasks ${semanticTasks.join(', ')}`} cannot be scored`);
  if (judgeModel && model && judgeModel === model) errors.push('QL_SERIES1_JUDGE_MODEL must differ from QL_SERIES1_MODEL');
  return { provider, model, judgeModel, semanticTasks, semanticRequired, errors };
}

async function main() {
  const { live, hosts, task } = cli();
  for (const host of hosts) {
    if (!HOSTS.includes(host)) throw new Error(`Unknown host '${host}'.`);
  }
  if (task) getTask(task);
  const config = checkConfiguration(task);
  const result = {
    schema: 'ql-series1-preflight/0.2',
    live_requested: live,
    hosts,
    selected_task: task,
    model: config.model,
    provider: config.provider,
    judge_model: config.judgeModel,
    structural: {
      task_count: SERIES1_TASKS.length,
      semantic_task_count: config.semanticTasks.length,
      artifact_task_count: SERIES1_TASKS.length - config.semanticTasks.length,
      semantic_judge_required_for_this_preflight: config.semanticRequired,
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
