# QL / MEF Runtime Refraction — Intent Note

Status: canonical intent clarification for deep QL integration.

The Factory architecture already distinguishes QL canon, executable QL kernel, MEF semantic service, and ordinary software. This note makes one design intent explicit for agent/runtime work.

## Technical primitives first, semantic refraction second

Major software primitives should exist operationally without requiring MEF in order to exist. Their design may be informed by raw QL form and laws, but MEF does not define them by renaming technical objects.

Instead, the QL/MEF kernel should increasingly become an epistemic-functional map of the underlying technical system: it projects and refracts Projects, Runs, Agents, Claims, closures and other primitives to disclose relations already present in their operation.

This preserves the direction:

```text
QL canon and formal laws
    -> inform executable architecture

ordinary technical primitives and traces
    -> remain independently usable

QL / MEF kernel
    -> projects and refracts those primitives
    -> discloses semantic, epistemic and functional relations
```

## Agent loop alignment

The current agent-loop work exposes a particularly clean correspondence.

A Run is the chronological execution/observation unit. Closure is the semantic attainment of sufficient determinacy relative to an operative whole. A closure may require evidence or effects produced by one or more Runs; a Run may terminate without closure.

The natural MEF refractions are:

```text
L3   Processual     closure-bearing recurrence as concrescence
L3'  Chronological actual traversal and Run history
L1   Causal         causal constitution and causal disclosure at closure
L4'  Knowledge Work prompts, traces, challenges, patterns, discovery, insight and verification
```

The L3 alignment is warranted by the existing processual definition of concrescence and satisfaction/perishing: inherited actuality and novelty are integrated toward determinacy; once satisfaction is attained, the achieved occasion becomes datum for what follows. In agent terms, this maps naturally onto open recurrence, candidate determination, positive closure, retained difference and possible re-entry.

This is a semantic refraction, not a requirement to encode Whiteheadian or MEF vocabulary in the generic runtime API.

## Integration direction

A successful QL-native agent is expected to fit naturally into the QL/MEF stack and may eventually be projected through the executable kernel as semantically loaded meta-intelligence. That integration is desirable, but it is not a dependency in either direction:

- the QL Agent experiments can establish recurrence and closure semantics without an MEF service;
- the QL/MEF kernel can continue to formalise QL and refraction operations without depending on a finished QL agent;
- when both mature, the kernel should be able to refract the agent architecture and its traces without redefining their underlying technical ontology.

The governing principle remains: **every major primitive should admit MEF refraction without requiring MEF in order to exist.**
