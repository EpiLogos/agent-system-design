import type { FactoryBuildView } from './types'
import sssfFixture from '../fixtures/sssf-session.json'
import dshFixture from '../fixtures/dsh-trajectory.json'
import thinFixture from '../fixtures/thin-trajectory.json'
import { mapSssfTrace, type SssfBinding, type SssfEvent, type SssfSessionDetail } from './sssf'

const binding: SssfBinding = {
  projectRef: 'project:01K2N6P7H8J9K0M1N2P3Q4R5S6',
  runRef: 'run:01K2N6P7H8J9K0M1N2P3Q4R5S7',
  executionRef: 'execution:01K2N6P7H8J9K0M1N2P3Q4R5S8',
  harnessRef: 'sssf@de31374882e7a4e3e5b7bb9bd09e69dc2f779356',
  agentRefs: {
    planner: 'agent:01K2N6P7H8J9K0M1N2P3Q4R5S9',
    builder: 'agent:01K2N6P7H8J9K0M1N2P3Q4R5SA',
  },
  agencyRefs: {
    planner: 'agency:01K2N6P7H8J9K0M1N2P3Q4R5SB',
    builder: 'agency:01K2N6P7H8J9K0M1N2P3Q4R5SC',
  },
  agentSessionRefs: {
    planner: 'agent-session:01K2N6P7H8J9K0M1N2P3Q4R5SD',
    builder: 'agent-session:01K2N6P7H8J9K0M1N2P3Q4R5SE',
  },
  nativeTrajectoryRef: 'sssf:demo-2026-08-17',
}

export const sssfParityTrace = mapSssfTrace(
  { session: sssfFixture.session, phases: sssfFixture.phases, agents: sssfFixture.agents } as SssfSessionDetail,
  sssfFixture.events as SssfEvent[],
  binding,
)

export const factoryBuildFixture: FactoryBuildView = {
  project: { projectRef: binding.projectRef, label: 'Factory GUI / Build Surface' },
  run: { runRef: binding.runRef, runMapRef: `${binding.runRef}/map`, label: 'Port SSSF execution UX and integrate Factory semantics', status: 'running' },
  frontier: {
    subjectRef: 'decision:01K2N6P7H8J9K0M1N2P3Q4R5SQ',
    title: 'Prove the semantic envelope across heterogeneous trajectories',
    mode: 'verification',
    summary: 'The SSSF-derived execution optic is present. The live question is whether portable Factory trace remains primary while DSH-native depth and a thin harness degrade honestly.',
    closureState: 'open',
    gateState: 'evidence-required',
  },
  claims: [
    { claimRef: 'claim:01K2N6P7H8J9K0M1N2P3Q4R5ST', statement: 'The execution view preserves the pinned SSSF run → waterfall → phase → tool-detail interaction.', status: 'supported', evidenceRefs: ['evidence:01K2N6P7H8J9K0M1N2P3Q4R5SX'] },
    { claimRef: 'claim:01K2N6P7H8J9K0M1N2P3Q4R5SV', statement: 'DeepSeek Harness native trajectory can remain linked without becoming Factory event identity.', status: 'supported', evidenceRefs: ['evidence:01K2N6P7H8J9K0M1N2P3Q4R5SY'] },
    { claimRef: 'claim:01K2N6P7H8J9K0M1N2P3Q4R5SW', statement: 'A thin portable-only harness remains usable without fabricated body/session/process detail.', status: 'supported', evidenceRefs: ['evidence:01K2N6P7H8J9K0M1N2P3Q4R5SZ'] },
  ],
  evidence: [
    { evidenceRef: 'evidence:01K2N6P7H8J9K0M1N2P3Q4R5SX', label: 'Pinned source/fixture geometry + tool detail comparison', producingExecutionRef: binding.executionRef },
    { evidenceRef: 'evidence:01K2N6P7H8J9K0M1N2P3Q4R5SY', label: 'Portable/native correlation fixture', nativeRef: 'dsh-session-event:evt-42', producingExecutionRef: dshFixture.executionRef },
    { evidenceRef: 'evidence:01K2N6P7H8J9K0M1N2P3Q4R5SZ', label: 'Missing-native-field degradation fixture', producingExecutionRef: thinFixture.executionRef },
  ],
  candidates: [
    { candidateRef: 'candidate:01K2N6P7H8J9K0M1N2P3Q4R5SF', revision: 1, label: 'Source-fidelity React port', status: 'ready', producingExecutionRefs: [binding.executionRef], claimRefs: ['claim:01K2N6P7H8J9K0M1N2P3Q4R5ST'], evidenceRefs: ['evidence:01K2N6P7H8J9K0M1N2P3Q4R5SX'], tradeoffs: ['React port avoids a permanent Vue island while retaining the upstream interaction grammar.'] },
    { candidateRef: 'candidate:01K2N6P7H8J9K0M1N2P3Q4R5SG', revision: 1, label: 'Factory semantic + multi-harness envelope', status: 'developing', producingExecutionRefs: [dshFixture.executionRef, thinFixture.executionRef], claimRefs: ['claim:01K2N6P7H8J9K0M1N2P3Q4R5SV', 'claim:01K2N6P7H8J9K0M1N2P3Q4R5SW'], evidenceRefs: ['evidence:01K2N6P7H8J9K0M1N2P3Q4R5SY', 'evidence:01K2N6P7H8J9K0M1N2P3Q4R5SZ'], tradeoffs: ['Portable trace is intentionally smaller than target-native trajectories.'] },
  ],
  humanRequests: [],
  agencies: [
    { agencyRef: 'agency:01K2N6P7H8J9K0M1N2P3Q4R5SR', agentRef: 'agent:01K2N6P7H8J9K0M1N2P3Q4R5SS', label: 'Situated root agency', position: 'root', rootScopeRef: 'actuation:root-scope/demo', metagencyGrantRefs: ['actuation:metagency/demo'], actuationRef: 'actuation:demo', returnRef: 'actuation:return/demo', returnState: 'returned-difference' },
    { agencyRef: binding.agencyRefs.builder!, agentRef: binding.agentRefs.builder!, label: 'Build agency', position: 'participating', actuationRef: 'actuation:demo' },
  ],
  executions: [
    { executionRef: binding.executionRef, agencyRef: binding.agencyRefs.builder, agentRef: binding.agentRefs.builder, status: 'success', harnessRef: binding.harnessRef, agentSessionRef: binding.agentSessionRefs?.builder, nativeTrajectoryRef: binding.nativeTrajectoryRef },
    { executionRef: dshFixture.executionRef, agencyRef: dshFixture.agencyRef, agentRef: dshFixture.agentRef, status: 'success', harnessRef: dshFixture.harnessRef, harnessCompositionRef: dshFixture.harnessCompositionRef, agentSessionRef: dshFixture.agentSessionRef, sessionSpaceRef: dshFixture.sessionSpaceRef, surfaceRefs: dshFixture.surfaceRefs, workcellBindingRefs: dshFixture.workcellBindingRefs, nativeTrajectoryRef: dshFixture.nativeTrajectory.ref },
    { executionRef: thinFixture.executionRef, agencyRef: thinFixture.agencyRef, agentRef: thinFixture.agentRef, status: 'success', harnessRef: thinFixture.harnessRef },
  ],
  trajectories: [sssfParityTrace, dshFixture, thinFixture] as FactoryBuildView['trajectories'],
  actions: [
    { actionRef: 'action:01K2N6P7H8J9K0M1N2P3Q4R5T0', label: 'Recognise', subjectKinds: ['candidate'] },
    { actionRef: 'action:01K2N6P7H8J9K0M1N2P3Q4R5T1', label: 'Return for repair', subjectKinds: ['candidate'] },
    { actionRef: 'action:01K2N6P7H8J9K0M1N2P3Q4R5T2', label: 'Request evidence', subjectKinds: ['candidate', 'run'] },
    { actionRef: 'action:01K2N6P7H8J9K0M1N2P3Q4R5T3', label: 'Re-enter frontier', subjectKinds: ['run'] },
  ],
}
