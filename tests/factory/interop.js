const fs = require('fs');
const path = require('path');

const fixturePath = process.argv[2] || path.resolve(__dirname, '../../contracts/factory/fixtures/interop-v1.json');
const schemaPath = process.argv[3] || path.resolve(__dirname, '../../contracts/factory/interop.schema.json');
const qlSchemaPath = process.argv[4] || path.resolve(__dirname, '../../contracts/factory/ql-mef-composition.schema.json');
const document = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));
const schema = JSON.parse(fs.readFileSync(schemaPath, 'utf8'));
const qlSchema = JSON.parse(fs.readFileSync(qlSchemaPath, 'utf8'));
const fail = (message) => { throw new Error(message); };
const requireTrue = (condition, message) => { if (!condition) fail(message); };
const canonical = /^factory:[a-z][a-z0-9-]*:[A-Za-z0-9][A-Za-z0-9._-]*$/;
const kind = (name) => new RegExp(`^factory:${name}:[A-Za-z0-9][A-Za-z0-9._-]*$`);

function assertOwners(node, location = '#') {
  if (!node || typeof node !== 'object' || Array.isArray(node)) return;
  if (node.properties) {
    for (const [name, child] of Object.entries(node.properties)) {
      requireTrue(typeof child['x-semantic-owner'] === 'string' && child['x-semantic-owner'].length > 0, `${location}/${name} owner`);
      assertOwners(child, `${location}/${name}`);
    }
  }
  if (node.$defs) for (const [name, child] of Object.entries(node.$defs)) assertOwners(child, `${location}/$defs/${name}`);
  if (node.items && typeof node.items === 'object') assertOwners(node.items, `${location}/items`);
}

function subjectKey(value) { return `${value.subjectRef}|${value.stateRef}|${value.revision}`; }

function validateQlComposition(value) {
  requireTrue(schema.properties.qlComposition.$ref === qlSchema.$id, 'parent QL schema reference drift');
  requireTrue(schema.properties.qlComposition['x-semantic-owner'] === 'Standalone QL/MEF module', 'QL semantic owner drift');
  requireTrue(!Object.prototype.hasOwnProperty.call(schema.$defs, 'qlComposition'), 'Factory must not duplicate QL composition definition');
  requireTrue(value && typeof value === 'object' && !Array.isArray(value), 'QL composition object');
  const allowed = new Set(Object.keys(qlSchema.properties));
  requireTrue(Object.keys(value).every((key) => allowed.has(key)), 'QL composition additional property');
  requireTrue(typeof value.targetRef === 'string' && value.targetRef.length > 0, 'QL targetRef');
  const coordinateKeys = ['qlFormRef', 'qlAddress', 'lensRef', 'sublensRef'];
  requireTrue(coordinateKeys.some((key) => Object.prototype.hasOwnProperty.call(value, key)), 'QL composition requires a coordinate');
  for (const key of coordinateKeys) {
    if (!Object.prototype.hasOwnProperty.call(value, key)) continue;
    const pattern = qlSchema.properties[key].pattern;
    requireTrue(typeof pattern === 'string' && new RegExp(pattern).test(value[key]), `invalid ${key}`);
  }
}

function rejectAnti(item) {
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
      fail(`unknown anti fixture ${item.id}`);
  }
}

function validate(doc) {
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
  requireTrue(c.projectionProvenance.every((p) => p.canonicalRef === c.projectBinding.projectRef), 'projection identity drift');
  requireTrue(new Set(c.projectionProvenance.map((p) => p.providerRef)).size >= 2, 'provider change not exercised');
  validateQlComposition(c.qlComposition);
  requireTrue(canonical.test(c.qlComposition.targetRef), 'QL target composition');
  const expectedAnti = new Set(['action-as-capability-identity-collapse','binding-as-ref','model-as-agent','session-as-agent','provider-as-project','stale-subject-state-evidence']);
  requireTrue(doc.antiFixtures.length === expectedAnti.size && doc.antiFixtures.every((item) => expectedAnti.has(item.id)), 'anti fixture completeness');
  for (const item of doc.antiFixtures) {
    let rejected = false;
    try { rejectAnti(item); } catch (_) { rejected = true; }
    requireTrue(rejected && item.mustReject === true, `anti fixture accepted: ${item.id}`);
  }
}

const roundTripped = JSON.parse(JSON.stringify(document));
validate(roundTripped);
requireTrue(JSON.stringify(roundTripped) === JSON.stringify(document), 'round-trip drift');
console.log('Node interop PASS');
