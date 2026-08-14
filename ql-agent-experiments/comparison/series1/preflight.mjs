import { HOSTS } from './contract.mjs';
import {
  providerForHost,
  SERIES1_PROVIDER,
  series1JudgeModelId,
  series1ModelId
} from './providers.mjs';
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
  const provider = process.env.QL_SERIES1_PROVIDER ?? SERIES1_PROVIDER;
  const model = series1ModelId();
  const judgeModel = series1JudgeModelId();
  const semanticTasks = SERIES1_TASKS.filter((task) => task.quality.kind === 'semantic').map((task) => task.id);
  const semanticRequired = selectedTask ? getTask(selectedTask).quality.kind === 'semantic' : semanticTasks.length > 0;
  const errors = [];

  if (provider !== SERIES1_PROVIDER) {
    errors.push(`Series 1 is currently stipulated to provider '${SERIES1_PROVIDER}', not '${provider}'`);
  }
  if (!model) errors.push('A concrete Series 1 candidate model is required');
  if (!process.env.DEEPSEEK_API_KEY) errors.push('DEEPSEEK_API_KEY is missing');
  if (semanticRequired && !judgeModel) {
    errors.push(`A distinct judge model is required; ${selectedTask ? `selected semantic task ${selectedTask}` : `semantic tasks ${semanticTasks.join(', ')}`} cannot be scored`);
  }
  if (semanticRequired && judgeModel === model) {
    errors.push('The blinded semantic judge model must differ from the candidate model');
  }

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
    schema: 'ql-series1-preflight/0.3',
    live_requested: live,
    hosts,
    selected_task: task,
    provider: config.provider,
    model: config.model,
    judge_model: config.judgeModel,
    credential_contract: 'DEEPSEEK_API_KEY',
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
