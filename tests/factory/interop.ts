import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type RecordValue = { [key: string]: any };
const here = path.dirname(fileURLToPath(import.meta.url));
const fixturePath = process.argv[2] || path.resolve(here, '../../contracts/factory/fixtures/interop-v1.json');
const schemaPath = process.argv[3] || path.resolve(here, '../../contracts/factory/interop.schema.json');
const document: RecordValue = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));
const schema: RecordValue = JSON.parse(fs.readFileSync(schemaPath, 'utf8'));
const requireTrue = (condition: boolean, message: string): void => { if (!condition) throw new Error(message); };
const canonical = /^factory:[a-z][a-z0-9-]*:[A-Za-z0-9][A-Za-z0-9._-]*$/;
const kind = (name: string): RegExp => new RegExp(`^factory:${name}:[A-Za-z0-9][A-Za-z0-9._-]*$`);

function assertOwners(node: RecordValue, location = '#'): void {
  if (!node || typeof node !== 'object' || Array.isArray(node)) return;
  if (node.properties) {
    for (const [name, child] of Object.entries(node.properties as Record<string, RecordValue>)) {
      requireTrue(typeof child['x-semantic-owner'] === 'string' && child['x-semantic-owner'].length > 0, `${location}/${name} owner`);
      assertOwners(child, `${location}/${name}`);
    }
  }
  if (node.$defs) for (const [name, child] of Object.entries(node.$defs as Record<string, RecordValue>)) assertOwners(child, `${location}/$defs/${name}`);
  if (node.items && typeof node.items === 'object') assertOwners(node.items, `${location}/items`);
}

const subjectKey = (value: RecordValue): string => `${value.subjectRef}|${value.stateRef}|${value.revision}`;

function rejectAnti(item: RecordValue): void {
  const value = item.value || {};
  switch (item.id) {
    case 'action-as-capability-identity-collapse':
      requireTrue(kind('action').test(value.actionRef), 'ActionRef');
      requireTrue(kind('capability').test(value.capabilityRef), 'CapabilityRef');
      requireTrue(value.actionRef !== value.capabilityRef, 'Action Capability collapse');
      return;
    case 'binding-as-ref':
      requireTrue(typeof value.bindingId === 'string' && value.bindingId.startsWith('binding:'), 'Binding cannot be canonical Ref');
      return;
    case 'model-as-agent':
    case 'session-as-agent':
      requireTrue(kind('agent').test(value.agentRef), 'Agent identity');
      return;
    case 'provider-as-project':
      requireTrue(kind('project').test(value.projectRef), 'Project identity');
      return;
    case 'stale-subject-state-evidence':
      requireTrue(subjectKey(value.currentSubjectState) === subjectKey(value.evidenceSubjectState), 'stale Evidence');
      return;
    default:
      throw new Error(`unknown anti fixture ${item.id}`);
  }
}

function validate(doc: RecordValue): void {
  requireTrue(doc.fixtureVersion === 'factory.interop-fixtures/v1', 'fixture version');
  const c = doc.contract;
  requireTrue(c.contractVersion === 'factory.interop/v1', 'contract version');
  requireTrue(JSON.stringify(Object.keys(c).sort()) === JSON.stringify([...schema.required].sort()), 'complete contract surface');
  assertOwners(schema);
  requireTrue(canonical.test(c.identityEnvelope.ref), 'canonical identity');
  requireTrue(kind('action').test(c.actionDescriptor.actionRef), 'Action identity');
  requireTrue(kind('capability').test(c.capabilityDescriptor.capabilityRef), 'Capability identity');
  requireTrue(c.actionDescriptor.actionRef !== c.capabilityDescriptor.capabilityRef, 'Action/Capability drift');
  requireTrue(kind('project').test(c.projectBinding.projectRef), 'Project identity');
  requireTrue(kind('run').test(c.runEnvelope.runRef), 'Run identity');
  requireTrue(kind('execution').test(c.executionEnvelope.executionRef), 'Execution identity');
  requireTrue(kind('agent').test(c.executionEnvelope.agentRef), 'Agent identity');
  requireTrue(!Object.keys(c.executionDemand).some((key) => key.toLowerCase().includes('provider')), 'provider leaked into ExecutionDemand');
  requireTrue(c.binding.bindingId.startsWith('binding:') && !c.binding.bindingId.startsWith('factory:'), 'Binding-as-Ref');
  const current = subjectKey(c.identityEnvelope.subjectState);
  for (const keyName of ['evidenceEnvelope', 'assessmentEnvelope', 'closureEnvelope', 'gateDecisionEnvelope']) requireTrue(subjectKey(c[keyName].subjectState) === current, `${keyName} subject-state`);
  requireTrue(c.projectionProvenance.length >= 2, 'projection corpus');
  requireTrue(c.projectionProvenance.every((p: RecordValue) => p.canonicalRef === c.projectBinding.projectRef), 'projection identity drift');
  requireTrue(new Set(c.projectionProvenance.map((p: RecordValue) => p.providerRef)).size >= 2, 'provider change not exercised');
  requireTrue(canonical.test(c.qlComposition.targetRef), 'QL target composition');
  const expectedAnti = new Set(['action-as-capability-identity-collapse','binding-as-ref','model-as-agent','session-as-agent','provider-as-project','stale-subject-state-evidence']);
  requireTrue(doc.antiFixtures.length === expectedAnti.size && doc.antiFixtures.every((item: RecordValue) => expectedAnti.has(item.id)), 'anti fixture completeness');
  for (const item of doc.antiFixtures) {
    let rejected = false;
    try { rejectAnti(item); } catch { rejected = true; }
    requireTrue(rejected && item.mustReject === true, `anti fixture accepted: ${item.id}`);
  }
}

const encoded = JSON.stringify(document);
const roundTripped: RecordValue = JSON.parse(encoded) as Json;
validate(roundTripped);
requireTrue(JSON.stringify(roundTripped) === encoded, 'round-trip drift');
console.log('TypeScript interop PASS');
