import fs from 'node:fs/promises';

function fence(value, language = 'json') {
  const text = typeof value === 'string' ? value : JSON.stringify(value, null, 2);
  return `~~~${language}\n${text}\n~~~`;
}

function changedFiles(before = {}, after = {}) {
  const names = [...new Set([...Object.keys(before), ...Object.keys(after)])].sort();
  return names.filter((name) => before[name] !== after[name]);
}

function renderEvent(event, index) {
  const title = `${index + 1}. ${event.channel ?? 'event'} :: ${event.event_type ?? event.type ?? 'unknown'}`;
  return `#### ${title}\n\n${fence(event)}\n`;
}

function renderRecord(record) {
  const changed = changedFiles(record.starting_workspace, record.final_workspace);
  const parts = [
    `## ${record.condition} — repetition ${record.repetition}`,
    '',
    `Runtime: \`${record.runtime.id}@${record.runtime.version}\`  `,
    `Execution: \`${record.execution_status}\`  `,
    `Semantic state: \`${record.semantic_status}\`  `,
    `Objective verification: \`${record.verification?.objective_checks_pass}\`  `,
    `Model calls: **${record.model_calls}** · Capability calls: **${record.capability_calls}** · Tokens: **${record.total_tokens}** · Elapsed: **${Math.round(record.elapsed_ms)} ms**`,
    '',
    '### Final outcome',
    '',
    fence(record.outcome, 'text'),
    '',
    '### Objective verification evidence',
    '',
    fence(record.verification),
    '',
    `### Workspace changes (${changed.length})`,
    '',
    changed.length ? changed.map((name) => `- \`${name}\``).join('\n') : '_No workspace files changed._',
    ''
  ];

  for (const name of changed) {
    parts.push(`#### ${name} — before`, '', fence(record.starting_workspace?.[name] ?? '<absent>', 'text'), '');
    parts.push(`#### ${name} — after`, '', fence(record.final_workspace?.[name] ?? '<absent>', 'text'), '');
  }

  parts.push('### Complete chronological trace', '');
  for (const [index, event] of (record.record?.events ?? []).entries()) {
    parts.push(renderEvent(event, index));
  }
  return parts.join('\n');
}

async function main() {
  const file = process.argv[2];
  if (!file) throw new Error('Usage: node render-review.mjs <series1-run.json>');
  const manifest = JSON.parse(await fs.readFile(file, 'utf8'));
  const lines = [
    '# Series 1 v0.1 — human review bundle',
    '',
    `Benchmark: \`${manifest.benchmark}\`  `,
    `Host: \`${manifest.host.id}\`  `,
    `Host path: \`${manifest.host.real_framework_path}\`  `,
    `Model: \`${manifest.model.provider}:${manifest.model.id}\`  `,
    `Determination: **${manifest.determination}**`,
    '',
    '## Held-constant check',
    '',
    fence(manifest.held_constant),
    '',
    '## Exact benchmark prompt',
    '',
    fence(manifest.review.prompt, 'text'),
    '',
    '## Success / constraint text',
    '',
    manifest.review.success_conditions.map((item) => `- ${item}`).join('\n'),
    '',
    '## Verification protocol',
    '',
    manifest.review.verification_protocol.map((item) => `- ${item}`).join('\n'),
    '',
    '## Human review focus',
    '',
    manifest.review.focus.map((item) => `- ${item}`).join('\n'),
    '',
    '## Human-only source/reference anchors',
    '',
    '_These anchors are included in the review bundle only. They were not sent to the candidate model._',
    '',
    manifest.review.human_reference.map((item) => `- ${item}`).join('\n'),
    '',
    '## Process summary (not a winner calculation)',
    '',
    fence(manifest.process_summary),
    '',
    ...manifest.records.map(renderRecord),
    '',
    '---',
    '',
    'Human determination remains open. Record observations first; introduce scalar/blinded evals only in a later benchmark phase.'
  ];
  process.stdout.write(`${lines.join('\n')}\n`);
}

main().catch((error) => {
  console.error(error.stack ?? error.message ?? String(error));
  process.exitCode = 1;
});
