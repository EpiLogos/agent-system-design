# Persistent Agency and Material Hosting — Cross-Product Alignment

**Status:** `DESIGN AMENDMENT — subordinate to the canonical Factory architecture`  
**Date:** 2026-08-15  
**Coordinates:** RC-010 ExecutionDemand/materialisation, RC-014 ExecutionDisposition/Execution Intelligence, AIKit V2 composable runtime environments, Workcell next Wayfinder.

## 1. Determination

Modern agent systems increasingly package several different responsibilities behind one gateway, daemon, runtime or host process. Factory must preserve the semantic distinctions even when a particular implementation does not.

The relevant ownership relation is:

```text
Factory
  authors/develops why an agency arrangement should exist,
  what act it serves, and what evidence closes the act
        ↓
AIKit
  resolves Agent / Agency / Harness / session / capabilities
  and the Surfaces through which that agency is encountered
        ↓
Workcell
  resolves processes / services / bindings / storage / network
  and keeps the material arrangement alive and observable
```

This diagram is an ownership relation, not a mandatory linear runtime pipeline.

The enduring Agent, its situated Agency, effective Harness/HarnessComposition, AgentSession/Execution, communication Surface, Workcell material world and provider process are all distinct.

## 2. Authored persistence and reachability requirements

A Project/Run may legitimately require properties such as:

- the resolved agency remains continuously available for a bounded span;
- an authenticated interactive access path exists;
- a human conversation Surface exists;
- an automation/event ingress Surface exists;
- execution state survives or can be recovered according to an explicit retention contract;
- loss/restart of a communication adapter is detected and repaired;
- the material host can be replaced without rewriting authored semantic identity where the task permits provider substitution.

These are meaningful requirements, but Factory must express them at the right level.

Factory may author the **need** for persistence, reachability, access modes, recovery or independence. It should not prescribe `Docker`, `systemd`, a hostname, a port, a gateway process, a Workcell provider or a target-native Surface implementation unless that concrete choice is itself part of Project intent/design.

## 3. ExecutionDisposition relation

`ExecutionDisposition` remains the situated resolution product for a semantic act.

Where relevant it may retain opaque, attributable references to:

```text
Agent / Agency
model / Harness
HarnessComposition ref/revision/fingerprint
AgentSession / SessionSpace relation
resolved Surface set or Surface-binding evidence
ExecutionDemand
MaterialisedExecutionWorld / Workcell relation
```

The exact cross-repository serialization remains an implementation contract. The semantic law is that those refs explain the body and material environment that carried the act without transferring ownership into Factory.

Factory does not need to import AIKit Component/Surface ontology or Workcell provider/binding ontology merely to preserve provenance.

## 4. Surface and material binding are distinct

A communication/encounter Surface belongs to the operational runtime composition resolved by AIKit or the target application/harness.

Examples:

```text
CLI
TUI
GUI / conversation view
messaging application
HTTP/API
webhook / event trigger
editor/application integration
```

A material binding belongs to Workcell and describes how a service is physically reachable/maintained:

```text
process/service lifecycle
logical endpoint
socket / HTTP / WebSocket / stdio transport
network exposure scope
credentials binding
health/readiness
persistent storage
provider/host provenance
```

One effective Agent/Harness/session may expose several Surfaces. One Surface may be rebound to another material endpoint. Neither event automatically changes Agent identity.

## 5. Gateway-shaped systems

`Gateway` is not a Factory primitive and should not become one merely because Hermes, OpenClaw or another harness uses that term.

Such systems are useful integration evidence because they often demonstrate:

- a persistent agent/session host;
- independent communication adapters;
- multiple human/software access methods;
- service health/restart;
- authenticated remote control;
- durable runtime state.

The semantic interpretation remains target-specific.

AIKit may model the Harness/Component/Surface relation. Workcell may model the material process/service/binding relation. Factory records enough provenance to explain which resolved arrangement carried an Execution.

There is no requirement for a universal `GatewayProtocol`.

## 6. Identity and lifecycle laws

Implementations and fixtures must preserve at least these distinctions:

```text
restart communication adapter
    != new Agent

restart gateway/service process
    != new Agent
    != necessarily new AgentSession

replace AgentSession
    != new Agent

change HarnessComposition
    != new Agent

rebind Surface endpoint
    != new Surface where target semantics preserve the same Surface relation

move material host local -> remote Workcell
    != new Project / Run / Candidate / Agent

restart Workcell Control Service
    != new material world when reconciliation proves the same recoverable world
```

The converse also matters: a stable semantic ref must not be used to hide a real session replacement, runtime-body change, lost state or failed recovery. Effective state/provenance must remain inspectable.

## 7. ExecutionDemand boundary

`ExecutionDemand` remains provider-neutral. A persistent-agent workload may require ordinary material affordances such as:

- long-lived execution;
- durable writable state;
- supervised restart/reconciliation;
- health/readiness observation;
- authenticated interactive endpoint;
- event/webhook ingress;
- streaming connectivity;
- exposure policy.

That does not require `AgentGateway`, `Hermes`, `OpenClaw`, `Docker`, `systemd`, port numbers or hostnames in the canonical semantic demand.

Higher-level authored requirements may resolve through AIKit into a Workcell demand/binding need while preserving the original Project/Run/Agent identity.

## 8. Verification consequences

Cross-product acceptance should prove:

1. one semantic Agent can be enacted through several communication Surfaces without identity multiplication;
2. loss of one Surface can be reported while the Agent and other Surfaces remain operative;
3. communication-adapter restart and AgentSession replacement remain distinguishable;
4. the same provider-neutral persistent-hosting demand can materialise on different Workcells where offers satisfy it;
5. moving local -> reference-server changes provider/material provenance but not authored identity;
6. provider/process/endpoint/gateway IDs cannot masquerade as Agent, Project, Run, Candidate, Action, Capability or Surface refs;
7. target-native gateway protocols remain target-owned and opaque to Factory;
8. ExecutionDisposition/Evidence can explain which effective runtime body and material world carried an execution without importing either product's private ontology.

## 9. Coordinated implementation

- AIKit V2-J / #53 owns the Agent/Harness/HarnessComposition/Surface resolution and disclosure side.
- Workcell #19–#25 owns the native Workcell surface, zero-setup local base, Control Service, persistent service bindings, SDK/conformance, reference server and gateway-management material integrations.
- Factory RC-010/#11 and RC-014/#15 retain the semantic boundaries above and should consume only the minimum stable cross-repository refs/evidence needed for explanation and verification.

This amendment adds no new Factory runtime service and does not make Workcell or AIKit a Factory implementation dependency.
