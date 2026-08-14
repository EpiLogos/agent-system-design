const fs = require('node:fs');
const path = require('node:path');

type J = any;
const root: string = process.env.FACTORY_INTEROP_ROOT || path.resolve(__dirname, '../..');
const load = (rel: string): J => JSON.parse(fs.readFileSync(path.join(root, rel), 'utf8'));
const assert = (ok: boolean, msg: string): void => { if (!ok) throw new Error(msg); };
const canonical = /^[a-z][a-z0-9-]*:[0-7][0-9A-HJKMNP-TV-Z]{25}$/;
const kindRef = (kind: string, value: unknown): boolean => typeof value === 'string' && new RegExp(`^${kind}:[0-7][0-9A-HJKMNP-TV-Z]{25}$`).test(value);
const subjectKey = (s: J): string => `${s.subjectRef}|${s.subjectRevision}|${s.stateRef}`;

function checkOwners(node: J, where = '#'): void {
  if (!node || typeof node !== 'object' || Array.isArray(node)) return;
  if (node.properties) for (const [name, raw] of Object.entries(node.properties as Record<string, J>)) {
    const child: J = raw;
    assert(typeof child['x-semantic-owner'] === 'string' && child['x-semantic-owner'].trim().length > 0, `${where}/${name} missing one semantic owner`);
    checkOwners(child, `${where}/${name}`);
  }
  if (node.$defs) for (const [name, child] of Object.entries(node.$defs as Record<string, J>)) checkOwners(child, `${where}/$defs/${name}`);
  if (node.items && typeof node.items === 'object') checkOwners(node.items, `${where}/items`);
  if (node.anyOf) (node.anyOf as J[]).forEach((child, i) => checkOwners(child, `${where}/anyOf/${i}`));
}

function rejectsAnti(item: J): boolean {
  const v = item.value || {};
  switch (item.id) {
    case 'action-as-capability-identity-collapse': return kindRef('action', v.actionRef) && !kindRef('capability', v.capabilityRef);
    case 'binding-as-ref': return !!v.binding && Object.prototype.hasOwnProperty.call(v.binding, 'bindingRef') && canonical.test(v.binding.bindingRef);
    case 'model-as-agent':
    case 'session-as-agent': return !kindRef('agent', v.agentRef);
    case 'provider-as-project': return !kindRef('project', v.projectRef);
    case 'stale-subject-state-evidence': return subjectKey(v.currentSubjectState) !== subjectKey(v.evidenceSubjectState);
    default: return false;
  }
}

const schemaSet: J = load('contracts/factory/interop/schema-set.json');
const fixtureSet: J = load('contracts/factory/fixtures/interop/fixture-set.json');
assert(schemaSet.schemaSetVersion === 'factory.interop/v1', 'schema set version');
assert(fixtureSet.fixtureSetVersion === 'factory.interop-fixtures/v1' && fixtureSet.contractVersion === 'factory.interop/v1', 'fixture versions');
const schemas = new Map<string, J>(schemaSet.schemas.map((entry: J) => [entry.instanceKey, entry]));
const sections = new Map<string, J>(fixtureSet.sections.map((entry: J) => [entry.instanceKey, entry]));
assert(schemas.size === sections.size && [...schemas.keys()].every((key) => sections.has(key)), 'schema/fixture manifest drift');
for (const [key, entry] of schemas) {
  const schema: J = load(entry.path), fixture: J = load(sections.get(key).path);
  assert(schema.$id === entry.id, `${key} schema id drift`); checkOwners(schema);
  const encoded = JSON.stringify(fixture); assert(JSON.stringify(JSON.parse(encoded)) === encoded, `${key} round-trip drift`);
  assert(fixture.contractVersion === 'factory.interop/v1', `${key} contract version`);
}

const identity = load(sections.get('identityCore').path), run = load(sections.get('runProjection').path), execution = load(sections.get('execution').path);
const action = load(sections.get('action').path), capability = load(sections.get('capability').path), context = load(sections.get('context').path);
const evidence = load(sections.get('evidenceAssessment').path), closure = load(sections.get('closureGate').path), demand = load(sections.get('demand').path);
const offer = load(sections.get('workcellOffer').path), binding = load(sections.get('binding').path), world = load(sections.get('materialisedWorld').path), ql = load(sections.get('ql').path);

assert(canonical.test(identity.identityEnvelope.ref) && !identity.identityEnvelope.ref.startsWith('factory:'), 'canonical identity grammar');
assert(identity.identityEnvelope.revision === identity.identityEnvelope.subjectState.subjectRevision && identity.identityEnvelope.ref === identity.identityEnvelope.subjectState.subjectRef, 'identity subject/revision drift');
assert(kindRef('project', identity.projectBinding.projectRef) && !String(identity.projectBinding.sourceRef).startsWith('project:'), 'ProjectBinding/source boundary');
assert(kindRef('run', run.runEnvelope.runRef) && run.runEnvelope.runMapAddress === `${run.runEnvelope.runRef}/map`, 'Run/RunMap address');
assert(run.runEnvelope.projectRef === identity.projectBinding.projectRef, 'Run ProjectRef');
assert(run.projectionProvenance.length >= 2 && new Set(run.projectionProvenance.map((p: J) => p.providerRef)).size >= 2 && new Set(run.projectionProvenance.map((p: J) => p.externalId)).size >= 2, 'projection change not exercised');
assert(run.projectionProvenance.every((p: J) => p.canonicalRef === identity.projectBinding.projectRef && p.projectedRevision === run.runEnvelope.runRevision), 'projection identity/revision drift');
const ex = execution.executionEnvelope;
assert(kindRef('execution', ex.executionRef) && kindRef('agent', ex.agentRef) && kindRef('agency', ex.agencyRef) && kindRef('agent-session', ex.agentSessionRef), 'execution actor refs');
assert(ex.runRef === run.runEnvelope.runRef && !ex.agentRef.startsWith('model:') && !ex.agentRef.startsWith('agent-session:'), 'execution identity collapse/drift');
assert(kindRef('action', action.actionDescriptor.actionRef) && kindRef('capability', capability.capabilityDescriptor.capabilityRef) && action.actionDescriptor.actionRef !== capability.capabilityDescriptor.capabilityRef, 'Action/Capability distinction');
assert(action.actionDescriptor.ownerProjectRef === identity.projectBinding.projectRef && ex.capabilityRefs.includes(capability.capabilityDescriptor.capabilityRef), 'Action/Capability ownership/projection drift');
const cx = context.contextResolution, available = new Set<string>(cx.availableRefs), retrieved = new Set<string>(cx.retrievedRefs), loaded = new Set<string>(cx.loadedRefs);
assert([...loaded].every((r) => retrieved.has(r)) && [...retrieved].every((r) => available.has(r)), 'Context availability/retrieval/load drift');
assert(cx.projectRef === identity.projectBinding.projectRef && ex.contextResolutionRef === cx.contextResolutionRef && ex.generationRef === cx.generationRef, 'Context/generation identity drift');
assert(context.generationProvenance.actionRefs.includes(action.actionDescriptor.actionRef) && context.generationProvenance.capabilityRefs.includes(capability.capabilityDescriptor.capabilityRef), 'Generation content drift');
const current = subjectKey(identity.identityEnvelope.subjectState);
[evidence.evidenceEnvelope.subjectState, evidence.assessmentEnvelope.subjectState, closure.closureEnvelope.subjectState, closure.gateDecisionEnvelope.subjectState].forEach((s: J) => assert(subjectKey(s) === current, 'verification subject-state mismatch'));
assert(evidence.assessmentEnvelope.evidenceRefs.includes(evidence.evidenceEnvelope.evidenceRef) && closure.closureEnvelope.evidenceRefs.includes(evidence.evidenceEnvelope.evidenceRef) && closure.closureEnvelope.assessmentRefs.includes(evidence.assessmentEnvelope.assessmentRef), 'verification relation drift');
assert(closure.gateDecisionEnvelope.closureRef === closure.closureEnvelope.closureRef, 'Gate/Closure drift');
assert(evidence.assessmentEnvelope.producerExecutionRef !== evidence.evidenceEnvelope.producerExecutionRef && evidence.assessmentEnvelope.independentFromExecutionRefs.includes(evidence.evidenceEnvelope.producerExecutionRef), 'independent assessment lineage');
assert(!Object.keys(demand.executionDemand).some((key) => key.toLowerCase().includes('provider')), 'provider leaked into ExecutionDemand');
assert(demand.executionDemand.projectRef === identity.projectBinding.projectRef && demand.executionDemand.runRef === run.runEnvelope.runRef, 'ExecutionDemand identity drift');
assert(demand.candidateMaterialisationDemand.executionDemandRef === demand.executionDemand.demandRef && demand.candidateMaterialisationDemand.candidateRef === identity.identityEnvelope.ref, 'Candidate materialisation demand drift');
assert(demand.executionDemand.requiredAffordances.every((a: string) => offer.workcellOffer.affordances.includes(a)), 'required affordance unavailable');
assert(binding.binding.workcellRef === offer.workcellOffer.workcellRef && !('ref' in binding.binding) && !('bindingRef' in binding.binding) && !canonical.test(binding.binding.bindingKey), 'Binding identity boundary');
assert(world.materialisedExecutionWorld.executionDemandRef === demand.executionDemand.demandRef && world.materialisedExecutionWorld.candidateRef === identity.identityEnvelope.ref && world.materialisedExecutionWorld.workcellRef === offer.workcellOffer.workcellRef && world.materialisedExecutionWorld.bindingKeys.includes(binding.binding.bindingKey), 'Materialised world relation drift');
assert(canonical.test(ql.qlComposition.targetRef), 'QL target Ref'); ['qlFormRef','qlAddress','lensRef','qlTarget'].forEach((key) => assert(typeof ql.qlComposition[key] === 'string' && ql.qlComposition[key].length > 0, `${key} missing`));

const anti = load(fixtureSet.antiFixturesPath); const expectedAnti = new Set(['action-as-capability-identity-collapse','binding-as-ref','model-as-agent','session-as-agent','provider-as-project','stale-subject-state-evidence']);
assert(anti.fixtureVersion === 'factory.interop-fixtures/v1' && anti.antiFixtures.length === expectedAnti.size && anti.antiFixtures.every((item: J) => expectedAnti.has(item.id)), 'anti-fixture set incomplete');
anti.antiFixtures.forEach((item: J) => assert(item.mustReject === true && rejectsAnti(item), `anti fixture unexpectedly accepted: ${item.id}`));
console.log(`TypeScript CR-001 interop PASS (${schemas.size} schema sections, ${anti.antiFixtures.length} anti-fixtures)`);
