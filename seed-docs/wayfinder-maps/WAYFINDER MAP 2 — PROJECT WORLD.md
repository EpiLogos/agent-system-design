# WAYFINDER MAP 2 — PROJECT WORLD
## AIKit · Project Map · Project Bootstrap · Source Fidelity

**Namespaces:** `B.* · D.* · K.* · L.*`  
**Status:** replacement Wayfinder Map for whole-system reconciliation  
**Governing authority:** `QL-SOFTWARE-FACTORY-CONSTITUTIONAL-INDEX.md`  
**Subordinate sources:** Architecture Spec · Primitive Relations · Workcell Module Spec · Deep QL Integration Foundations · prior Project World Wayfinder Claim-set  
**Live-source Ground:** current `EpiLogos/ai-kit` source inspected independently of its documentation

---

# 0. Purpose and authority

This map owns the joined Project World question:

> **How does an arbitrary repository, source collection, or new software intention become a durable authored Project which human and agent can enter, understand, navigate, reproduce, and operate without repeatedly reconstructing its world by hand?**

It integrates four development threads:

```text
B · AIKit co-development
D · Project Map and Information Horizon
K · Project Bootstrap
L · Source Integration and reproducibility
```

It does **not** own the whole Factory ontology.

The Constitutional Index governs the current meaning of the suite. Earlier documents remain strong sources of architecture and implementation detail, but their formulations are read developmentally rather than flattened into simultaneous requirements.

This map therefore uses the following determination statuses:

```text
CONSTITUTIONAL
    governing cross-system invariant from the current corpus

CURRENT DESIGN
    sufficiently determined architecture for implementation,
    still subject to legitimate root/program reconciliation

OBSERVED
    directly evidenced in the inspected implementation/source

VERIFIED
    exercised by an appropriate real acceptance or integration test

RESEARCH CLAIM
    supported direction or source claim not yet verified as implementation fact

OPEN DECISION
    deliberately left to the owning map/human/product design

IMPORTED
    semantic identity or contract owned by another Wayfinder map
```

The prior Project World map is treated as a Claim-set. Its strongest architecture is retained; its unilateral cross-map or product-surface decisions are not.

---

# 1. Constitutional joined shape

The joined architecture remains:

```text
repository / directories / source / new intention
                    │
                    ▼
             PROJECT BOOTSTRAP
              a Factory Run
                    │
      evidence recovery before interrogation
                    │
                    ▼
                 PROJECT
          enduring authored identity
                    │
       ┌────────────┼────────────┐
       │            │            │
       ▼            ▼            ▼
 Project Canon  Project Map   Source Integrations
                    │
                    │
                    ▼
               AIKit indexes
               and resolves
                    │
                    ▼
                 CONTEXT
      ┌─────────────┼─────────────┐
      │             │             │
      ▼             ▼             ▼
Operative World  Information    Focus
                  Horizon
      │             │             │
      └─────────────┼─────────────┘
                    │
                    ▼
          Agent / Agency / human
                    │
                    ▼
              ExecutionDemand
                    │
                    ▼
                 Workcell
                    │
                    ▼
       MaterialisedExecutionWorld
```

The invariant boundaries are:

```text
Project > repository

AIKit
    indexes / resolves / projects operational resources
    does not own Project meaning

Project Map
    joins navigational and intelligence surfaces
    does not become a universal knowledge database

Context
    = Operative World
    + Information Horizon
    + Focus

available ≠ retrieved ≠ loaded

Action
    remains application/Project-owned
    AIKit indexes/resolves it
    Project Map navigates it

Workcell
    materialises ExecutionDemand
    AIKit does not become infrastructure scheduler

SourceIntegration
    declares and verifies a real upstream seam
    approximate local imitation is not integration

derived indexes
    remain reconstructable
    remain provider-owned

Project Bootstrap
    is a Factory Run
    performs evidence recovery before asking the human

QL / MEF
    remain first-class semantic/formal resources
    but ordinary Project operation does not depend on QL service availability
```

---

# 2. Ownership architecture

## 2.1 Product primitive ownership

| Concern | Canonical owner | Project World relation |
|---|---|---|
| `Project` | A / Factory Core | B/D/K/L materialise, index and enter it; do not redefine it |
| `Context` | shared Factory product primitive | AIKit contributes resolution structures; D contributes Information Horizon |
| `Run` / `RunMap` | A/C | K Bootstrap is one Run; D navigates Run/Evolution |
| `Agent` / `Agency` | I/shared | B indexes and resolves resources; Projects may profile/select them |
| `Action` | E / Project/Application | B indexes/resolves; D navigates |
| `Capability` | shared/B operating field | B resolves |
| `Candidate` | A/C | D may navigate Candidate relations; K may create first-run handoff |
| `Claim` / `Evidence` | A/G | K produces recovery Claims; D carries provenance |
| `ExecutionDemand` | shared A/F boundary | B may help resolve resources; F materialises it |
| `Workcell` | F | B indexes availability/offers only |
| `MaterialisedExecutionWorld` | F | Context may reference it in Operative World |
| `Ref` grammar | A/shared | all Project World objects consume it |
| `QLFormRef` / `QLAddress` / `LensRef` | H/shared QL contract | B/D expose and resolve them without inventing alternate identity |
| `SourceIntegration` | L | D/B/K consume its status and provenance |
| `ProjectMap` | D | canonical join/navigation contract |
| `ProjectBinding` | B | AIKit physical/workspace → Project resolution mechanism |
| `ProjectManifest` | Project World current design | small committed entry/reference projection of Project identity |
| `ContextDescriptor` | B / AIKit | operational input to AIKit resolution |
| `ResolvedView` | B / AIKit | capability-resolution output |
| `ContextResolution` | B | AIKit resolution envelope beneath canonical Context |
| `Generation` | B / AIKit | materialised capability/resource resolution |
| `InformationHorizon` | canonical Context facet | D assembles addressable providers; not a content store |

---

## 2.2 Three adjacent questions

The architecture remains legible if three questions are never collapsed:

```text
PROJECT MAP
"Where in this Project should I look?"

AIKIT
"What identities, powers, resources and sources
are available to this actor here?"

WORKCELL
"How can this deployment materially satisfy
the resolved execution demand?"
```

And above all three:

```text
CONTEXT
"What world can this actor meaningfully inhabit for this act?"
```

Context is therefore **not an AIKit object**.

---

# 3. Imported shared identities

This map imports rather than redefines:

```text
Ref
ProjectRef

RunRef
RunMapRef

AgentRef
AgencyRef

ActionRef
CapabilityRef

CandidateRef
ClaimRef

SourceIntegrationRef

ExecutionDemand
WorkcellRef
MaterialisedExecutionWorldRef

QLFormRef
QLAddress
LensRef
```

It additionally requires shared execution-surface identities from I/F/J where available:

```text
ExecutionRef
HarnessRef
ModelRef
```

Their exact grammar is **not specified here**.

Every map-local record points to these identities rather than creating a competing identifier system.

---

# 4. Canonical versus derived versus materialised versus projected

Project World must retain the Primitive Relations distinction.

## Canonical / authored

Examples:

```text
Project identity
recognised Project Canon
RunMap
Action identity
Agent identity
SourceIntegration declaration
Project Map declaration
```

## Derived

Examples:

```text
GitNexus graph
bkmr search index
AIKit catalogue index
Project Evolution view
frecency ranking
relevance ranking
Project Map provider cache
```

Derived state is rebuildable.

## Materialised

Examples:

```text
Context → AIKit Generation
ExecutionDemand → MaterialisedExecutionWorld
source revision → Checkout
Agent → AgentSession / Execution
```

## Projected

Examples:

```text
Project Map → UI/TUI/HTML view
RunMap → GitHub
HumanRequest → inbox/message surface
Action → UI/MCP/HTTP/CLI
Agent resources → harness projection
```

No useful projection becomes canonical merely because it is convenient.

---

# 5. Project as authored world

A Project remains:

> **An enduring authored identity around which source, intention, design, semantic knowledge, Actions, capabilities, history, contexts, and developmental work gather.**

It may contain:

```text
PROJECT
│
├── ProjectRef
│
├── source constituents
│   ├── repository / repositories
│   ├── local material
│   ├── external references
│   └── related Projects
│
├── Project Canon
│   ├── foundational intention
│   ├── recognised product / experiential intent
│   ├── recognised architecture / program design
│   ├── recognised Decisions
│   └── project language
│
├── Project Map
├── semantic wiki
├── Action Catalog reference
├── Project profiles / AIKit declarations
├── context-source declarations
├── SourceIntegrations
├── Run / Evolution history
├── current Ground
├── Agent / Agency resource relations
├── QL / MEF semantic resources where enabled
└── runtime descriptions / Workcell demand sources
```

Repository topology may change while Project identity survives.

---

# 6. Project metadata versus AIKit operating declarations

## 6.1 Semantic distinction

The following are distinct:

```text
Factory Project metadata / Project identity surfaces
                    ≠
AIKit operating / resolution configuration
```

The distinction is constitutional in meaning.

The exact filenames remain CURRENT DESIGN.

---

## 6.2 Factory-facing committed Project surfaces

Current design:

```text
.factory/
├── project.toml
├── project-map.toml
├── sources.toml
└── sources.lock.toml
```

**Determination status:** `CURRENT DESIGN — root Z ratification required`.

No governing source currently establishes `.factory/` as metaphysically privileged.

The semantic separation it represents **is** fixed.

A repo Ground pass must check for filesystem/package/tooling collision before Z ratifies the exact namespace.

---

## 6.3 ProjectManifest

`ProjectManifest` is deliberately small.

Conceptual form:

```toml
schema = 1

project = "<ProjectRef>"

project_map = ".factory/project-map.toml"
source_integrations = ".factory/sources.toml"
source_lock = ".factory/sources.lock.toml"

canon_roots = [
  "docs/constitution",
  "docs/design"
]

semantic_roots = [
  "wiki"
]
```

It may reference existing Project-owned surfaces.

It must not duplicate:

```text
source code
runtime commands
build commands
Project Canon content
Project Map provider definitions
SourceIntegration details
Action definitions
AIKit profile contents
```

The manifest is an **entry/reference surface**, not a universal project configuration file.

---

## 6.4 AIKit-owned project declarations

`.aikit/` remains AIKit operating space.

Current/target surfaces may include:

```text
.aikit/
├── profile.toml
├── profile.local.toml
└── project.toml
```

Exact schema remains an AIKit implementation task.

`.aikit/project.toml` may declare:

```text
AIKit ProjectRef binding
context-source bindings
profile/resource defaults
Capability Sets
Action source registrations
Agent-resource registrations
model/harness preferences
Workcell-resource visibility
integration-specific resolver configuration
```

It must **not** become Project Canon.

---

# 7. B — AIKit co-development

# 7.1 AIKit constitutional role

AIKit is the context-scoped index/resolver of actor-available:

```text
Projects / ProjectBindings
Profiles
Scopes
Session Spaces

Agents
Agencies

Capabilities
Capability Sets

Actions
Action Sets

agent resources
context sources

Models
Harnesses
Workcells / Host resources

Procedures
inbox items

usage / frecency
contextual relevance
fitness
preference
trust
availability

QL/MEF resources where enabled
```

Its defining question is:

> **What world of identities, powers, resources, knowledge sources and execution possibilities is available to this actor here and now?**

AIKit does not own the higher product meaning of Context.

---

# 7.2 Live AIKit Ground

The live source was rechecked during this regeneration.

As of this pass, `EpiLogos/ai-kit` still has `main` at:

```text
0ff819bb72763162b28491629ccc09ee93451808
```

with no later push visible since 5 August 2026.  

Observed crate structure remains:

```text
aikit-core
aikit-store
aikit-adapters
aikit-tui
aikit-cli
```

The source already implements important lower algebra:

```text
context-scoped deterministic capability resolution
scope precedence
Project directory/repository matching
Project/profile/session/task overlays
trust / quarantine
Capability cataloguing
Skill Sets
frecency/search foundations
immutable Generations
atomic materialisation
Procedures
inbox
client projections
tmux/cmux adapters
shared CLI/TUI application service
machine-readable CLI output
```

`aikit-core` explicitly treats the effective capability view as a resolution over roughly:

```text
user
+ host
+ Project scope chain
+ Session Space
+ task
+ targets
```

and produces explanation plus deterministic content identity.

Current `ContextDescriptor` already carries:

```text
context id
session id
project id
project root
task
isolation
platform
targets
mux
host
```

Current scope precedence is encoded as:

```text
global
→ host
→ project
→ project-local
→ session
→ task
→ one-shot

with managed policy outside ordinary override precedence
```

These are **OBSERVED implementation Ground**, not speculative design.

---

# 7.3 Current code-health evidence

The latest published CI run for current `main` is still failing. 

The previously inspected run showed:

```text
test every target       PASS
clippy -D warnings      FAIL
release build           skipped
diff check              skipped
```

Therefore:

```text
AIKit architecture/source presence      OBSERVED
large real test suite                   OBSERVED
tests on latest run                     VERIFIED PASS
full repository health gate             NOT VERIFIED / currently red
```

This blocks claims that current AIKit is a clean release baseline.

It does **not** block independent D/K/L design or source-independent implementation.

---

# 7.4 Project versus current AIKit `ProjectSpec`

Live AIKit currently has a `ProjectSpec` that primarily carries:

```text
schema
id
directory matchers
repository matchers
inherited/default Skill Sets
explicit Skill Sets
```

Its resolver maps physical/repository evidence to an operational project selection.

Semantically this is closer to:

```text
ProjectBinding
```

than to the Factory's canonical `Project`.

## Governing relation

```text
Project
    authored product primitive
    enduring identity

ProjectBinding
    AIKit operating mechanism
    maps physical/workspace evidence
    to Project identity / resolver declarations
```

This semantic correction is retained.

The implementation strategy is **migration, not casual rename**.

Required compatibility work:

```text
inspect every current ProjectSpec use
inspect persisted schema and JSON/output surfaces
inspect CLI vocabulary
inspect project discovery
inspect source/projection assumptions

then choose the least disruptive path among:
    internal rename + compatibility alias
    public alias/deprecation
    schema-stable semantic migration
    delayed code rename with corrected documentation/type boundary
```

Prohibited:

```text
ProjectSpec and ProjectBinding evolving into two parallel project models
```

---

# 7.5 Context resolution layers

Existing AIKit capability resolution should not be rewritten.

The current capability resolver remains responsible for its already-tested algebra:

```text
explicit enable / disable
scope precedence
managed denials
dependencies
conflicts
trust
platform/target availability
explanation
content identity
```

A Factory `ContextResolution` composes that existing result with other indexed resource classes.

Conceptually:

```text
ContextResolution
│
├── ContextDescriptor
├── ResolvedView
│
│   capability resolution
│
├── AgentResourceView
├── AgencyResourceView
├── ActionResourceView
├── ExecutionResourceView
│   ├── Model
│   ├── Harness
│   └── Workcell visibility
│
├── ContextSourceView
├── ProjectMapRef
├── InformationHorizonRef
├── QLResourceView
├── Focus refs
├── projection/materialisation status
└── explanation / provenance
```

`ContextResolution` is **not Context**.

It is an AIKit-produced description of resolved resources contributing principally to Context's Operative World and addressable Information Horizon.

---

# 7.6 Resource indexing without ontology collapse

AIKit should share index infrastructure where useful without flattening unlike things into one universal `Capsule`.

Target resource families:

```text
Capability Resources
    CapabilityRef

Action Resources
    ActionRef

Actor Resources
    AgentRef
    AgencyRef

Execution Resources
    ModelRef
    HarnessRef
    WorkcellRef

Context Sources
    Project Map providers
    bkmr sources
    external knowledge sources
    neighbouring Projects
    optional Bimba

QL / MEF Resources
    QLFormRef
    QLAddress
    LensRef
    executable QL-service capability
```

`Capsule` may remain AIKit packaging for the resource kinds it genuinely packages.

It is not automatically the semantic identity of every resource in the Factory.

---

# 8. Epi-Logos profile as first-class Project/Context resource

## 8.1 Generic Factory

A generic Project may resolve:

```text
generic AgentRefs
custom Project AgentRefs
AgencyRefs
Capability Sets
Action Sets
context sources
model/harness resources
Workcell resources
optional QL/MEF resources
```

It is not required to know the Epi-Logos identities.

---

## 8.2 Epi-Logos Project/Profile

The Epi-Logos profile activates first-class Agent resources:

```text
0/1  Epi-Logos orchestrator

#0   Anuttara
#1   Paramasiva
#2   Parāśakti
#3   Mahāmāyā
#4   Nara
#5   Epii
```

These are **enduring Agent identities**.

They are not universal Factory-stage names.

They are not reduced to Agency metadata.

They do not become different Agents when model, harness, capabilities, Workcell, session, or local Agency changes.

The Epi-Logos orchestrator remains the `0/1` meta-level reader/composer of the six.

---

## 8.3 Profile-scoped richer resources

The same Epi-Logos profile may additionally resolve:

```text
optional richer Agency identity forms
Epi-Logos Capability Sets
project-specific ontology sources
project-specific semantic sources
QLFormRefs
QLAddresses
all twelve LensRefs
optional executable QL/MEF service
optional Bimba / Neo4j information horizon
specialised theoretical / symbolic capabilities
```

None become dependencies of generic Project operation.

---

## 8.4 Required identity invariants

```text
Agent identity survives model change
Agent identity survives harness change
Agent identity survives Workcell change
Agent identity survives Agency change

Epi-Logos profile:
    orchestrator + six canonical AgentRefs available

generic project:
    those refs absent unless explicitly configured

Bimba unavailable:
    Bimba horizon/resource unavailable
    Project remains valid
    AIKit remains valid
    ordinary Actions remain valid

QL executable service unavailable:
    explicit QL operations unavailable
    Project identity remains valid
    Project Map remains valid
    Bootstrap remains valid
    AIKit ordinary resolution remains valid
    ordinary information retrieval remains valid
    ordinary Actions remain valid
```

---

# 9. Capability and Action resolution

## 9.1 Capability

Imported semantic meaning:

> any power an actor can be given the ability to use.

Examples include:

```text
skill
tool
CLI
script
MCP service
reasoning method
browser operation
source integration
Action
QL refraction capability
```

---

## 9.2 Action

Action identity is Project/Application-owned.

AIKit stores/indexes a descriptor sufficient for discovery and availability.

Project Map stores/navigates its relation to the rest of the Project.

The implementation remains where the Project/Application declares it.

```text
Project/Application
       │
       └── ActionRef + canonical implementation
                  │
         ┌────────┴─────────┐
         │                  │
         ▼                  ▼
    Project Map            AIKit
     navigation       index / resolve
```

AIKit must not generate a weaker shadow Action merely to expose it to agents.

---

## 9.3 Capability Set and Action Set

```text
CapabilitySet
    reusable composition of Capabilities

ActionSet
    reusable/scoped selection of ActionRefs
    over a Project/Application-owned Action Catalog
```

An Action Set does not transfer Action ownership to AIKit.

Set membership never launders:

```text
trust
permissions
availability
source fidelity
```

---

# 10. Learned ergonomics without authority collapse

AIKit may combine independent signals for ranking and suggestion:

```text
frecency
contextual relevance
fitness
preference
trust
availability
```

These must remain separately inspectable.

Hard constraints:

```text
managed policy
trust
explicit disable
dependency validity
interface compatibility
availability
source-fidelity requirements
```

Soft signals:

```text
frecency
contextual relevance
fitness
historical successful use
non-binding preference
```

A soft signal may rank or suggest.

It does not silently override a hard constraint.

---

# 11. Context — product primitive

## 11.1 Canonical form

```text
Context
=
Operative World
+
Information Horizon
+
Focus
```

Context is the nexus between Project and act.

---

## 11.2 Operative World

Answers:

> **What can act here and what can it use?**

May contain references to:

```text
Project
Profile / scoped dispositions

AgentRef
AgencyRef

CapabilityRefs
Capability Sets

ActionRefs
Action Sets

ModelRef
HarnessRef

permissions / trust

WorkcellRef
MaterialisedExecutionWorldRef

Session Space
Agent Session / Execution

QL / MEF resources
```

Workcell contributes material parts of the Operative World.

It does not own Context.

---

## 11.3 Information Horizon

Answers:

> **What can this actor discover or retrieve from here?**

May include:

```text
repository/source
Project Map
Canon
semantic wiki
GitNexus
bkmr
documentation
websites
papers
external/local material
other Projects
prior Runs
Candidate history
project-specific ontology sources
optional Bimba / Neo4j
QL/MEF semantic resources
```

The Information Horizon is **not a canonical content store**.

It is the addressable knowledge facet of Context assembled through provider-backed Project Map/source relations.

---

## 11.4 Focus

Answers:

> **What matters to this act?**

May point through imported Refs to:

```text
RunRef
RunMapRef / frontier
Decision ref
ClaimRef
CandidateRef
ActionRef
Artifact ref
current task
current QLAddress
current LensRef / refraction request
```

Focus narrows a potentially huge Information Horizon into a tractable retrieval problem.

---

# 12. Available ≠ Retrieved ≠ Loaded

These are execution-relative relations.

## Available

A source/provider item is addressable inside the current Information Horizon.

## Retrieved

Content was requested from a provider for the current act.

## Loaded

A representation of retrieved material entered a specific Execution/harness context.

There is no global boolean:

```text
source.loaded = true
```

Loading is always relative.

---

## 12.1 ContextLoad

`ContextLoad` is an exported Project World record.

Its exact shared identifier types come from I/J/A.

Required semantics:

```text
ContextLoad
├── execution: ExecutionRef
├── harness: HarnessRef
├── project: ProjectRef
├── focus: Ref[]
├── loaded sources
│   ├── source address/ref
│   ├── retrieved revision
│   ├── provider
│   ├── freshness
│   ├── representation
│   └── reason
├── retrieval refs
├── context transformations
└── optional budget/token accounting
```

A valid `ContextLoad` can therefore support:

```text
Information Horizon:
    24,311 available source entries

retrieved for this Decision:
    18

loaded into Execution X:
    7
```

---

# 13. D — Project Map

# 13.1 Definition

The Project Map is:

> **The canonical join/navigation contract over the Project's distinct intelligence and authority surfaces.**

It answers:

> **Where should I enter in order to understand or act on this Project?**

It does not answer all questions itself.

---

# 13.2 Joined providers

The Project Map may join:

```text
Source / Git

code-as-map headers

GitNexus / code intelligence

Intent / Design / Project Canon

semantic Markdown / Obsidian wiki

knowledge horizon / bkmr

Action Catalog

Run / Evolution history

Candidate relations

related Projects

websites / documentation / papers / local materials

optional Bimba / Neo4j

QL / MEF semantic/refraction resources
```

These remain separate providers/stores.

---

# 13.3 Project Map is not a graph-database mandate

The join may be implemented through:

```text
provider manifests
stable Refs
indexes
links
caches
relations
query federation
```

Nothing in this map requires all provider content to be copied into one graph database.

A future derived graph over ProjectMap entries is permitted.

It remains a derived view.

---

# 13.4 Project Map provider contract

Every provider exposes enough information to make its entries intelligible.

Conceptually:

```text
ProjectMapProvider
│
├── provider identity
├── provider kind
├── source identity
├── authority domains
├── current revision
├── freshness
├── health
├── indexing capability
├── retrieval capability
└── relation/link capability
```

A map entry contains at least:

```text
entry identity
subject Ref where applicable
provider/source
locator
authority domain
revision
freshness
health
retrievability
relations / links
```

Exact serialisation is D.01 implementation design.

---

# 13.5 Authority language

Project Map authority means:

> **For this question or authority domain, which source/provider is authoritative, and how current is our view of it?**

It does **not** mean:

> Project Map has become a general truth adjudicator.

Examples:

```text
"What code exists?"
    Source/Git

"What calls this function?"
    GitNexus view
    qualified by indexed Git revision

"What is this product intended to do?"
    recognised Project Canon

"What does this Project term mean?"
    recognised Canon / semantic wiki according to Project convention

"What canonical operation does the app expose?"
    application Action Catalog

"Why was this architectural path chosen?"
    Decision / Run history

"What is actually running now?"
    Workcell/runtime observations

"What Agent identities belong to this Project profile?"
    Project/Profile + AIKit resolution

"What external source informed this claim?"
    provenance / SourceIntegration / source provider
```

The Project Map routes and exposes authority.

It does not erase disagreement among sources.

---

# 13.6 Freshness

Providers expose a meaningful freshness relation.

Examples:

```text
Source/Git
    commit + dirty-worktree identity

GitNexus
    indexed source revision
    + index/tool schema revision

Canon
    source revision/content digest

semantic wiki
    source revision/content digest

Action Catalog
    owning source revision
    + Action schema/catalog revision

Run/Evolution
    canonical Run history revision/event position

bkmr
    bound source/index generation
    + provider version

Bimba
    graph/index revision where available

QL/MEF service
    service/kernel/form revision
```

Standard map-level freshness states:

```text
fresh
possibly-stale
stale
unavailable
unknown
```

`unknown` is not silently converted into `fresh`.

---

# 13.7 Stale provider handling

```text
provider source changed
        │
        ▼
map health detects mismatch
        │
        ▼
provider marked stale
        │
        ├── stale result is acceptable
        │      └── return with explicit revision/freshness
        │
        └── fresh result required
               └── refresh/reindex
                        │
              ┌─────────┴─────────┐
              ▼                   ▼
            fresh             failed
                               stale /
                               unavailable
```

Stale derived state cannot corrupt canonical source truth.

---

# 14. Core Project Map providers

## 14.1 Source / Git provider

Own question:

> What authored source/files/revisions actually exist?

Exposes:

```text
repository roots
source tree
current revision
dirty state
history links
file/module locations
```

It remains primary for source existence.

---

## 14.2 Code-as-map

Source remains directly navigable.

Small module/root headers may link:

```text
responsibility
primary interface
design
semantic wiki
```

They are entry apertures, not a second documentation system.

Preferred exploration remains:

```text
tree
→ search
→ source
→ imports/references
→ GitNexus where useful
→ design/wiki/Run history where useful
```

---

## 14.3 GitNexus provider

Role:

```text
structural code graph
symbols
dependencies
calls
context
impact
trace
change relationships
```

Project Map wraps it with:

```text
Project identity
stable Factory Refs
freshness
health
cross-links to Canon/wiki/Actions/Runs
```

GitNexus remains responsible for actual graph extraction/query.

No local approximate call-graph implementation satisfies this provider.

Implementation is blocked until `SourceIntegrationRef(GitNexus)` is VERIFIED.

---

## 14.4 Project Canon provider

Exposes recognised:

```text
foundational intention
product / experiential design
architecture
program design
recognised Decisions
project language
```

Project Canon remains selective.

Run-local analysis does not become canon merely because it is indexed.

---

## 14.5 Semantic Wiki provider

Target form remains plain Markdown / Obsidian compatible:

```text
wiki/
├── index.md
├── concepts/
├── modules/
├── flows/
├── decisions/
├── experiences/
└── glossary.md
```

It is:

```text
human-readable
agent-readable
Git-versionable
directly editable
cross-linkable
usable without Obsidian runtime
```

Bootstrap may create a sparse scaffold.

It must not fabricate a vast semantic web to claim completion.

---

## 14.6 Knowledge horizon / bkmr provider

bkmr remains an information-horizon adapter.

It may index/retrieve:

```text
external documentation
websites
papers
local document trees
notes
related repositories
related Projects
other declared knowledge sources
```

The ProjectMap/bkmr adapter must preserve:

```text
Project source binding
source identity
retrieval provenance
provider revision/version
freshness
```

There is no hidden globally mutable active Project database.

---

## 14.7 Action Catalog provider

Navigates:

```text
ActionRef
description
owning Project/Application
canonical source
available surfaces
schema/contract refs
relations to modules/concepts/Runs
```

Project Map does not own invocation semantics.

---

## 14.8 Run / Evolution provider

Answers:

> How did this Project become what it is?

It joins:

```text
RunRefs
RunMapRefs
Decisions
Candidates
recognition
recursion
reopenings / returns
source revisions
```

Project Evolution is a derived view over canonical developmental history.

Git history alone is insufficient.

---

## 14.9 Candidate navigation

Project Map may surface:

```text
CandidateRef
source state
RunRef
Claims
Evidence
MaterialisedExecutionWorldRef when live
recognition outcome
historical residue
```

Candidate semantics remain imported from A/C.

---

## 14.10 Bimba / Neo4j provider

Profile-gated.

For Epi-Logos:

```text
Project semantic field
        │
        ▼
optional Bimba horizon
```

Bimba failure yields:

```text
Bimba provider unavailable
```

not:

```text
Project unavailable
```

---

## 14.11 QL / MEF provider

Project World must keep QL/MEF first-class without creating a second object ontology.

The provider may expose:

```text
QLFormRefs relevant to the Project
QLAddresses attached to canonical Refs
all twelve LensRefs
available derived refractions
QL/MEF service health
version compatibility
links between Project concepts and QL resources
```

Canonical subject identity remains the ordinary Factory `Ref`.

Example:

```text
subject:
    ActionRef

reading:
    LensRef

result:
    derived refraction

identity:
    still the same ActionRef
```

---

# 15. QL / MEF continuity

## 15.1 Full manifold

Project World must leave the complete MEF manifold available:

```text
L0  L1  L2  L3  L4  L5
L0′ L1′ L2′ L3′ L4′ L5′
```

No subset becomes “the computational MEF”.

Current Factory-relevant roles include:

```text
L0
    investigative orientation

L1
    causal constitution

L2
    logical disposition

L3
    processual reading

L3′
    chronological reading

L4′
    knowledge work:
    Prompts → Traces → Challenges → Patterns → Discovery → Insight

L5
    articulation / Vāk
```

Other lenses remain first-class and discoverable rather than demoted.

---

## 15.2 Project semantics and QL

A Project may carry:

```text
QL-rooted or QL-consonant Canon
QLFormRefs
QLAddresses
LensRefs
refraction resources
semantic source relations
```

These deepen the Project's formal intelligibility.

They do not become prerequisites for the existence of `ProjectRef`.

---

## 15.3 Executable QL/MEF service

An executable QL/MEF service is optional in **runtime availability**, not optional in architectural addressability.

The Factory must have a stable seam for it.

It may eventually support operations equivalent in meaning to:

```text
locate
refract
relate
conjugate
mask
trace
synthesise
explain
```

Exact API names belong to H.

Project World consumes the H interface.

It does not define one.

---

## 15.4 Graceful absence invariant

If QL service is absent:

```text
Project identity                 works
Bootstrap                        works
Project Map                      works
AIKit ordinary resolution        works
ordinary information retrieval  works
Source Integration               works
ordinary Actions                 work
```

If QL service is present:

```text
QLAddress resolution
MEF refraction
QL-aware resource selection
Epi-Logos semantic depth
derived lens views
```

may deepen the same canonical objects.

---

# 16. Progressive retrieval

## 16.1 Retrieval request

Conceptually:

```text
RetrievalRequest
├── ProjectRef
├── Focus refs
├── question / retrieval purpose
├── authority-domain preference
├── optional preferred providers
├── freshness requirement
└── retrieval budget
```

Provider ranking may use:

```text
authority domain
Project Map topology
semantic relevance
GitNexus proximity
Run / Decision relations
explicit project preference
frecency
known source quality
```

Every returned item preserves provenance.

---

## 16.2 Focused Decision example

```text
Focus:
    Decision on session identity continuity

Information Horizon:
    8,000+ Project/source entries

retrieval planner chooses:
    recognised design Canon
    current source module
    GitNexus context
    prior Decision
    relevant harness source docs

retrieved:
    13

loaded into Execution:
    6
```

The un-loaded horizon remains addressable.

---

# 17. K — Project Bootstrap

# 17.1 Definition

Project Bootstrap is the Factory's developmental process for turning:

```text
mature external repository
sparse repository
fresh Project
existing Factory Project after long absence
```

into an intelligible and operable Project.

Bootstrap is itself a **Factory Run**.

It is not an installer.

---

# 17.2 Bootstrap destination

The destination is:

> **A visioned/operable Project which both human and agent can enter meaningfully.**

The result may include:

```text
recognised Bootstrap outcome
current Ground
Project Canon
Project Map
semantic wiki
AIKit ProjectBinding/profile/resources
SourceIntegration declarations
runtime understanding
Action surface understanding
Agent-resource understanding
Project developmental frontier
ordinary Run-ready state
```

---

# 17.3 Bootstrap is evidence-led

For an existing Project/repository, inspect before questioning.

Required evidence sweep includes:

```text
source / repository identity
Git history

tree / modules
package/build manifests
README/docs
ADRs / design material
product material
tests
CI

APIs
CLI
UI operations
domain methods
MCP/agent actions
Action-like operations

agent instructions
skills / capabilities
existing Agents / Agencies

runtime / deployment descriptions
Docker / Compose / scripts
host expectations

existing semantic wiki
Project Map-like material

GitHub issues / PRs where accessible
prior Run history where Factory-native

external references
SourceIntegration evidence

AIKit declarations
Factory declarations
QL / MEF resources where present
```

---

# 17.4 Evidence classes

Bootstrap classifies findings:

```text
OBSERVED
    directly evidenced by current source/runtime/configuration

DOCUMENTED
    asserted by existing project material

RECOGNISED
    current Project Canon / recognised Decision

INFERRED
    evidence-supported synthesis

CONFLICTING
    sources disagree materially

MISSING
    no adequate evidence

AUTHORIAL
    evidence cannot determine which future is intended
```

Human requests arise principally from `AUTHORIAL`.

`CONFLICTING` only becomes a HumanRequest if evidence cannot resolve it and the difference is consequentially authorial.

---

# 17.5 Human-question rule

Do not ask the human to provide facts already recoverable from:

```text
code
Git
tests
CI
runtime descriptors
documentation
Project Canon
Run history
Action Catalog
SourceIntegrations
```

Examples of prohibited unnecessary questions:

```text
Which package manager does this repository use?
What is the test command?
Does it use Docker?
Where is the entry point?
Which API operations exist?
What did ADR-12 decide?
```

Human attention stays primarily at:

```text
vision
product meaning
experiential intent
foundational architectural direction
genuine unresolved alternative
recognition
```

---

# 17.6 Bootstrap state

```text
entrance
   │
   ▼
Project identified / recovered
   │
   ▼
evidence inspected
   │
   ▼
Ground recovered
   │
   ▼
Intent / Design recovered where possible
   │
   ▼
authorial frontier
   │
   ├── no authorial decision
   │        │
   │        ▼
   │   continue autonomously
   │
   └── HumanRequest / Decision
            │
            ▼
project surfaces established
   │
   ▼
Project Map + AIKit + SourceIntegration
   │
   ▼
operative entry validated
   │
   ▼
Bootstrap recognition / recursion
   │
   ▼
ordinary Run-ready Project
```

---

# 17.7 Four Bootstrap entrances

## A. Mature external repository

Expected:

```text
heavy recovery
light questioning
source-derived architecture
existing Action discovery
existing runtime discovery
minimum necessary semantic scaffolding
```

## B. Sparse repository

Expected:

```text
strong code/runtime Ground recovery
clear separation:
    observed behaviour
    versus intended meaning
human authorship only where Project dream/architecture is not recoverable
```

## C. Fresh Project

Expected:

```text
Project identity
visioning
product/experiential Intent
foundational Design
Agent-Native Action design
source/runtime bootstrap
Project Map
AIKit declarations
```

Human involvement is naturally higher because less can be recovered.

## D. Existing Factory Project after absence

Expected:

```text
recover same ProjectRef
compare current evidence to retained Ground/Canon/history
refresh stale providers
inspect SourceIntegration drift
surface developmental frontier
resume ordinary work
```

No fresh-project questionnaire.

---

# 17.8 Bootstrap artifacts

Bootstrap may produce ordinary Factory Artifacts/Claims such as:

```text
Ground recovery Artifact
Intent recovery Artifact
Design recovery Artifact

source inspection Artifact
Action discovery Artifact
Capability/resource discovery Artifact
runtime discovery Artifact

Project Map plan
SourceIntegration plan

HumanRequests / Decisions where necessary

Bootstrap Recognition packet
```

Exact Artifact types are owned by A.

K must not invent a competing Artifact ontology.

---

# 17.9 Bootstrap committed/materialised surfaces

Bootstrap may establish or reconcile:

```text
ProjectManifest
Project Map declaration
SourceIntegration declarations / lock

Project Canon surfaces where missing
semantic wiki scaffold/content where justified

AIKit ProjectBinding
AIKit profiles
AIKit resource declarations

Action Catalog reference/adapters
runtime descriptions
context-source declarations
```

These are delegated to their actual owners.

Bootstrap orchestrates the developmental act.

It does not absorb their semantics.

---

# 17.10 Bootstrap Run Map terminology

Bootstrap is itself a Run.

Its map is therefore:

```text
Bootstrap Run Map
```

At completion, the Project has:

```text
recognised Bootstrap outcome
current Ground
Project developmental frontier
ordinary Run-ready state
```

The next developmental transformation may create:

```text
first ordinary Project Run / Wayfinder
```

Do **not** call every Project's post-Bootstrap artifact a `Root Wayfinder Map`.

`Root Wayfinder Map` is reserved for a genuine root/program-control artifact such as the Software Factory's Z-level program map.

---

# 17.11 Recognition / Recursion effect

Bootstrap Recognition can promote appropriate durable orientation.

Later ordinary Recognition/Recursion can change:

```text
Project Canon
current Ground
Project Map relations
semantic wiki
SourceIntegration state
learned resource observations
Evolution history
Project entry frontier
```

The next Project entry must see this retained difference.

This map consumes Recognition/Recursion semantics from A/C.

It does not redefine them.

---

# 18. Re-entry after long absence

A Project re-entry experience must answer, without requiring archaeology:

```text
What is this Project?
What was I trying to make?
What is recognised now?
What changed since I last meaningfully entered?
Which Runs/Decisions caused that?
What is the current developmental frontier?
Which sources/providers are stale?
Which SourceIntegrations drifted?
Which Agents/Actions/capabilities are currently available?
Where should I enter?
```

A conceptual `ProjectEntry` export contains references to:

```text
ProjectRef
current Ground
recognised Intent / Design
current frontier
active/recent RunRefs
Evolution delta
Project Map health
SourceIntegration health
AIKit ContextResolution
Agent-resource availability
recommended entry refs
```

The exact product surface belongs to M/Z.

---

# 19. L — Source Integration

# 19.1 Definition

A `SourceIntegration` is:

> **A durable declaration that the Project intentionally incorporates or depends upon a real external codebase, system, protocol, knowledge source, or tool through a named actual seam.**

It exists to prevent:

```text
"integrate X"
    ↓
agent invents something vaguely shaped like X
```

---

# 19.2 Severe fidelity chain

The required chain is:

```text
Declaration
    │
    ▼
Resolution / Pin
    │
    ▼
Actual source inspection
    │
    ▼
Real seam identified
    │
    ▼
adapter / dependency / mount / fork
    │
    ▼
real smoke / integration test
    │
    ▼
Evidence retained
    │
    ▼
VERIFIED integration
    │
    ▼
drift / upgrade lifecycle
```

Documentation alone never produces `VERIFIED`.

---

# 19.3 Integration modes

Permitted explicit modes:

```text
direct dependency
CLI adapter
protocol adapter
source mount
capability source
vendored fork
reference implementation
```

Additional modes require an architectural Decision if they materially differ.

---

# 19.4 SourceIntegration declaration

Conceptually records:

```text
SourceIntegrationRef

upstream identity

intended revision/range

integration mode

intended seam

licence / provenance expectation

local augmentation

dependent nodes

upgrade policy
```

---

# 19.5 Resolved source lock

Records actual resolution:

```text
SourceIntegrationRef

exact resolved revision/version
content/source identity
licence evidence

actual API / CLI / protocol / paths used

local adapter/fork revision

inspection evidence

smoke/integration-test evidence

compatibility result

drift baseline
```

Current design:

```text
.factory/sources.toml
.factory/sources.lock.toml
```

Filename/location subject to Z ratification.

Semantic split is retained.

---

# 19.6 Source status

Minimum status model:

```text
DECLARED
    intended source exists in architecture

PINNED
    exact source/revision resolved

INSPECTED
    actual source at that revision examined

SEAM-IDENTIFIED
    real reusable API/CLI/protocol/source path established

VERIFIED
    integration exercises real seam successfully

DRIFTED
    upstream or compatibility relation changed

BLOCKED
    dependent implementation cannot proceed honestly

SUPERSEDED
    recognised replacement exists
```

`OBSERVED` and `VERIFIED` remain epistemic labels over evidence, not merely lifecycle names.

---

# 19.7 Per-node blocking

Source fidelity blocks the **dependent implementation**, not unrelated Project World work.

Example:

```text
D.03 GitNexus provider
    requires source/gitnexus = VERIFIED

D.01 Project Map provider interface
    does not

Therefore:
    D.01 may build
    D.03 remains blocked
```

This prevents both:

```text
premature imitation
```

and:

```text
one unavailable upstream freezing the entire architecture
```

---

# 19.8 Source substitution

A weaker/local substitute for a named upstream requires an explicit architectural Decision.

It must state:

```text
why real upstream cannot be used
what semantics are being replaced
what fidelity is lost
why substitution is intentional
whether it is temporary or permanent
how convergence/upgrade would work
```

Without that Decision:

```text
local imitation ≠ integration
```

---

# 20. Current upstream ledger

This ledger distinguishes intended architecture from inspected evidence.

| Integration | Intended role | Current pass status | Implementation consequence |
|---|---|---|---|
| **AIKit · `EpiLogos/ai-kit`** | co-developed context/index/resolution substrate | `OBSERVED`; source rechecked at `0ff819…`; tests pass in latest CI but full CI red | architecture may integrate now; release/code-health claims blocked |
| **SSSF** | deterministic/agent runtime reference patterns | `INTENDED / source claim` | dependent source-level reuse blocked until inspection |
| **Pi** | preferred first harness; real CLI/RPC/SDK/extensions | `INTENDED / source claim` | Pi-specific implementation blocked until verified |
| **GitNexus** | Project Map structural code intelligence | `INTENDED / detailed source claim` | D.03 blocked until verified |
| **Matt Pocock skills** | Capability source / Wayfinder, Grilling, Prototype, related methods | `INTENDED` | capability import/overlay implementation requires pinned source inspection |
| **HumanLayer / Dexter material** | design/context discipline reference | `INTENDED / reference` | copied/adapted material must name exact source seam |
| **cmux** | session/workspace/application surface | AIKit adapter `OBSERVED`; upstream integration not fully reverified here | current adapter may be inspected further; upstream-specific new work blocked until pin |
| **tmux** | session/multiplexer surface | AIKit adapter `OBSERVED`; real current protocol compatibility not fully audited here | retain observed adapter; production compatibility tests required |
| **Hermes** | personal orchestrator/messaging projection | `INTENDED` | blocked pending source inspection |
| **bkmr** | Project knowledge-horizon provider | AIKit contrib shim `OBSERVED` in prior live inspection; upstream/core target seam not yet fully verified | D.05 production provider blocked pending L verification |
| **Agent-Native precedent/adapters** | design precedent / possible adapters | `REFERENCE / INTENDED` | E owns canonical standard; no universal framework dependency |
| **Neo4j / Bimba** | optional profile-scoped semantic horizon | architectural role determined; concrete connector versions unverified | profile-specific implementation only after verification |
| **Docker** | Workcell provider | F role determined; exact provider/API version not verified here | F-owned implementation gate |
| **Arrakis / selected MicroVM provider** | optional stronger Workcell isolation | `RESEARCH / INTENDED` | F provider blocked until exact upstream seam chosen and verified |

---

# 21. Reproducibility

Reproducibility exists at several layers.

## 21.1 Project reproducibility

A Project's authored world can be reconstructed from:

```text
ProjectRef
ProjectManifest
source constituents / Git revisions
Project Canon
Project Map declaration
semantic wiki
SourceIntegration declaration + lock
AIKit ProjectBinding/profile/resource declarations
Action Catalog references
runtime descriptions
Run/Decision history
```

Derived indexes are not required to preserve authored identity.

---

## 21.2 SourceIntegration reproducibility

Records:

```text
source
exact pin
licence/provenance
real seam
adapter/fork identity
local augmentation
verification
compatibility
upgrade relation
```

---

## 21.3 Execution reproducibility

Consumes cross-map evidence sufficient to reconstruct or explain an act:

```text
Project source revision / dirty state

RunRef / CandidateRef

Context resolution identity
AIKit Generation / resolution identity where relevant

Capability revisions
Action source revision

AgentRef
AgencyRef

ModelRef
HarnessRef
ExecutionRef

ExecutionDemand

WorkcellRef
MaterialisedExecutionWorldRef

SourceIntegrationRefs

ContextLoad
```

For nondeterministic agents, reproducibility means:

> **the grounded world, material conditions, source versions, selected resources and evidence trail can be reconstructed and inspected.**

It does not mean identical model tokens.

---

## 21.4 Deployment reproducibility

Owned principally by F.

Project World records enough references to connect:

```text
Project runtime description
ExecutionDemand
Workcell
provider selections
runtime/image versions
service bindings
persistence relationships
```

without importing provider-specific infrastructure into Project meaning.

---

# 22. Drift and upgrade

Drift detection is read-only:

```text
locked source
     │
     ▼
inspect current upstream
     │
     ▼
compare:
    revision
    seam
    schema/API
    licence
    compatibility
     │
     ▼
DriftReport
```

Possible outcomes:

```text
no drift
compatible upstream change
breaking seam change
licence/provenance change
compatibility unknown
source unavailable
```

Update is a separate action:

```text
DriftReport
    │
    ▼
UpgradePlan
    │
    ├── proposed new pin
    ├── upstream diff
    ├── adapter/fork effect
    ├── compatibility tests
    └── migration work
    │
    ▼
Decision / normal technical approval
    │
    ▼
apply
    │
    ▼
verify real seam
    │
    ▼
new lock
```

A drift check never follows `latest` automatically.

---

# 23. Product-experience exports to M/Z

B/D/K/L owns the information and state needed for product experience.

It does **not** fix final CLI words, page hierarchy, TUI permanence, or overall human-product constitution.

Required M/Z journeys:

```text
existing-repository adoption / Bootstrap

fresh Project creation

Project entry

long-absence re-entry

Project health

Project Map navigation

information-horizon inspection

source provenance drill-down

"why is this source here?"

Bootstrap authorial frontier

Agent-resource availability

Action/Capability availability

available / retrieved / loaded disclosure

SourceIntegration drift / blocked integration

optional QL/Bimba resource degradation
```

---

## 23.1 Existing repo Bootstrap intent state

The human should see:

```text
Repository identified

Recovered from evidence:
    source             ✓
    runtime            ✓
    tests              ✓
    product intent     mostly recovered
    architecture       conflicting in one place
    Action surface     discovered
    agent resources    partial

Authorial frontier:
    one unresolved consequential question

Factory work to establish:
    Project Map
    semantic scaffold
    AIKit declarations
    SourceIntegrations
    current Ground

[inspect evidence]
[enter authorial decision]
```

The product requirement is:

> **review an intelligent recovery, not complete a setup questionnaire.**

---

## 23.2 Fresh Project intent state

Must support movement from:

```text
vision
→ intended human experience
→ foundational design
→ Action surface
→ Project world
→ first ordinary Candidate/Run
```

The user should not be forced through infrastructure-first configuration.

---

## 23.3 Return-after-absence intent state

Required information:

```text
what the Project is
what changed
which changes were recognised
current Ground
current developmental frontier
important Decisions/Runs
Project Map health
source drift
Agent/resource availability
where to enter
```

Raw logs remain available but are not the default route to understanding.

---

## 23.4 Information Horizon intent state

Must visibly distinguish:

```text
AVAILABLE
    can be retrieved

RETRIEVED
    retrieved for this act

LOADED
    actually present in this Execution/harness context
```

Each item can drill into:

```text
source
provider
authority domain
revision
freshness
retrieval reason
load reason
```

---

## 23.5 Source provenance intent state

For any relevant source:

```text
Why is this here?

because:
    Project declared SourceIntegration X
    at pinned revision Y
    using seam Z
    for purpose P

current status:
    VERIFIED / stale / drifted / blocked
```

---

## 23.6 Agent-resource intent state

For Epi-Logos Project:

```text
Epi-Logos orchestrator      available
Anuttara                    available
Paramasiva                  available
Parāśakti                   available
Mahāmāyā                    available
Nara                        available
Epii                        available

Bimba                       unavailable
QL service                  available
```

For generic Project:

```text
only generic/custom Project Agents and Agencies
```

Final visual form belongs to M.

---

# 24. UI/CLI/headless invariant

The only architectural UI invariant owned here is:

```text
CLI
TUI
headless
browser/app
future richer Project/Run surfaces
        │
        ▼
share one application/service semantics
        │
        ▼
do not duplicate resolver/business logic
```

The current AIKit transient palette is useful `OBSERVED` Ground.

It is **not** constitutionalised as the permanent upper bound of Project/Run UX.

Likewise, current command wording is Ground, not immutable product vocabulary.

---

# 25. Open product-surface naming collision

Current AIKit uses top-level:

```text
aikit adopt
```

for foreign Agent Skill authority adoption.

Project Bootstrap/adoption is semantically distinct.

This map determines only that product UX must distinguish:

```text
foreign skill/capability authority adoption

versus

Project Bootstrap / adoption / entry
```

The exact CLI words, aliases, deprecations and migration belong to:

```text
OPEN PRODUCT-SURFACE DECISION → M/Z
```

B/D/K/L must not resolve the command vocabulary unilaterally.

---

# 26. Interface Ledger

| Interface | Producer → Consumer | Payload / relation | Boundary |
|---|---|---|---|
| `B.I01 ProjectBindingResolution` | AIKit → Context composition | `ProjectRef` + binding evidence | physical match does not define Project meaning |
| `B.I02 CapabilityResolution` | AIKit → Context | `ResolvedView` / `CapabilityRef`s | existing resolver algebra retained |
| `B.I03 ActorResourceIndex` | I/Project → AIKit | `AgentRef` / `AgencyRef` descriptors | AIKit indexes; I owns identity |
| `B.I04 ActionResourceIndex` | E/Project → AIKit | `ActionRef` descriptors | application owns Action |
| `B.I05 ExecutionResourceIndex` | I/F/providers → AIKit | model/harness/Workcell availability | index only |
| `B.I06 ContextSourceIndex` | D/providers → AIKit | addressable source descriptors | source truth remains provider-owned |
| `B.I07 QLResourceIndex` | H/Project → AIKit | `QLFormRef`, `QLAddress`, `LensRef` | same Factory subject Ref |
| `B.I08 ContextResolution` | AIKit → actor/harness | resolved resource envelope | not canonical Context |
| `B.I09 ContextProjection` | AIKit → target/harness | materialised capability/resource view | projection effect explicit |
| `D.I01 ProjectMap` | D → AIKit/human/agents/K | provider/index navigation | D owns join, not content |
| `D.I02 ProviderHealth` | provider → D | revision/freshness/health | unknown ≠ fresh |
| `D.I03 SourceNavigation` | Git → D | source tree/revision | Git owns source truth |
| `D.I04 CodeIntelligence` | GitNexus → D | context/impact/trace | GitNexus owns graph |
| `D.I05 CanonNavigation` | Project Canon → D | recognised intent/design refs | Canon owner unchanged |
| `D.I06 SemanticNavigation` | wiki → D | linked project-language refs | Markdown remains source |
| `D.I07 KnowledgeRetrieval` | bkmr/provider → D | provenance-bearing results | provider index not source truth |
| `D.I08 ActionNavigation` | E → D | `ActionRef` and relations | no Action copy |
| `D.I09 EvolutionNavigation` | A/C → D | Runs/Decisions/Candidates/recognition | Run history remains canonical elsewhere |
| `D.I10 QLRefractionNavigation` | H → D | QL/MEF derived readings | subject identity preserved |
| `D.I11 ContextLoad` | D/I/J → trace/evidence | execution-relative loaded-source relation | loading never global |
| `K.I01 BootstrapRun` | A/C → K | normal Run/RunMap | K does not invent second workflow ontology |
| `K.I02 BootstrapRecovery` | K → A/G | Ground/Intent/Design Claims/Artifacts | evidence-led |
| `K.I03 AuthorialFrontier` | K → HumanRequest/Decision | genuinely unresolved authorship | minimal questioning |
| `K.I04 ProjectSurfaceHandoff` | K → Project owners | manifests/canon/map/profile refs | delegated ownership |
| `K.I05 ActionDiscoveryHandoff` | K → E | recovered Action Claims | E validates/owns Action |
| `K.I06 RuntimeDiscoveryHandoff` | K → F | runtime descriptions / ExecutionDemand inputs | F owns material semantics |
| `K.I07 SourcePlanHandoff` | K → L | candidate integration requirements | L verifies source |
| `K.I08 BootstrapRecognition` | A/C → Project | retained Project difference | next entry observes it |
| `L.I01 SourceDeclaration` | Project → L | intended upstream/seam | durable declaration |
| `L.I02 SourceLock` | L → all consumers | pin/licence/seam/evidence | resolved source truth |
| `L.I03 SourceVerification` | L → development gate | VERIFIED/BLOCKED | per-dependent-node |
| `L.I04 DriftReport` | L → Project/Run | read-only difference | no automatic mutation |
| `L.I05 ReproducibilityEnvelope` | A/B/D/F/I/J/L → evidence | reconstruction refs | no ownership collapse |

---

# 27. Decision Ledger

| Decision | Result | Status |
|---|---|---|
| Project is larger than repository | retained | CONSTITUTIONAL |
| AIKit owns Project meaning | rejected | CONSTITUTIONAL |
| AIKit resolves/indexes/project resources | retained | CONSTITUTIONAL |
| Project Map becomes universal project DB | rejected | CONSTITUTIONAL |
| `Context = OperativeWorld + InformationHorizon + Focus` | retained | CONSTITUTIONAL |
| AIKit `ContextResolution` replaces Context | rejected | CONSTITUTIONAL |
| Available/retrieved/loaded collapse | rejected | CONSTITUTIONAL |
| Workcell scheduling logic belongs in AIKit | rejected | CONSTITUTIONAL |
| Action semantics copied into AIKit | rejected | CONSTITUTIONAL |
| ProjectBinding semantic correction | retained | CURRENT DESIGN |
| immediate public `ProjectSpec` rename | rejected as premature | CURRENT DESIGN |
| compatibility migration/alias audit first | retained | CURRENT DESIGN |
| `.factory/` semantic namespace separation | retained | CURRENT DESIGN |
| exact `.factory/` filesystem name | root ratification | OPEN DECISION → Z |
| `.aikit/` contains Project Canon | rejected | CONSTITUTIONAL |
| top-level `aikit adopt` must be renamed | returned to M/Z | OPEN PRODUCT DECISION |
| transient palette is permanent UI topology | rejected | OPEN PRODUCT DESIGN |
| one shared CLI/TUI/headless service semantics | retained | CONSTITUTIONAL/CURRENT IMPLEMENTATION |
| Epi-Logos Agent names = universal Factory stages | rejected | CONSTITUTIONAL refinement |
| Epi-Logos six Agent identities + orchestrator profile resources | restored | CONSTITUTIONAL |
| Bimba generic dependency | rejected | CONSTITUTIONAL |
| QL/MEF reduced to metadata provider | rejected | CONSTITUTIONAL |
| QL service mandatory for ordinary Project operation | rejected | CONSTITUTIONAL |
| all twelve MEF lenses remain first-class | retained | CONSTITUTIONAL |
| Project Map is general truth adjudicator | rejected | CONSTITUTIONAL |
| SourceIntegration may be satisfied by local approximation | rejected | CONSTITUTIONAL |
| source-fidelity blocker freezes unrelated work | rejected | CURRENT DESIGN |
| Bootstrap asks user before evidence recovery | rejected | CONSTITUTIONAL |
| post-Bootstrap map always called Root Wayfinder | rejected | TERMINOLOGY CORRECTION |
| Recognition/Recursion must alter future Project entry where durable difference was retained | retained | CONSTITUTIONAL |

---

# 28. Executable development nodes

# B.* — AIKit

## B.01 — AIKit implementation Ground and health baseline

**id:** `B.01`  
**purpose:** maintain an evidence-backed view of current AIKit before co-development.  
**owner/system:** AIKit co-development.  
**inputs:** live AIKit repository; CI; current SourceIntegration record.  
**outputs:** implementation inventory; current health evidence; source pin.  
**dependencies:** none.  
**interfaces:** `L.I01–L.I03`.  
**source basis:** Constitutional Index B thread; Architecture AIKit source discipline; inspected live source.  
**acceptance:** current crate/source capabilities classified `intended · observed · verified`; CI failure accurately represented; no README-only feature promoted.  
**test artifacts:** source inventory; latest CI evidence; version/pin record.  
**decisions already resolved:** current resolver/store/adapters are real Ground worth preserving.  
**prohibited hidden decisions:** rewriting architecture while performing source inventory; treating red CI as green.  
**determination status:** `CONSTITUTIONAL requirement + OBSERVED implementation Ground`.

---

## B.02 — ProjectBinding compatibility migration

**id:** `B.02`  
**purpose:** align current AIKit `ProjectSpec` semantics with canonical Project without unnecessary public churn.  
**owner/system:** AIKit.  
**inputs:** current `ProjectSpec` uses/schema/CLI; imported `ProjectRef`.  
**outputs:** migration ADR; canonical semantic `ProjectBinding`; compatibility aliases/migration as selected.  
**dependencies:** B.01; shared `ProjectRef`.  
**interfaces:** `B.I01`.  
**source basis:** live `ProjectSpec`; Primitive Relations Project identity.  
**acceptance:** existing saved bindings continue resolving; one logical Project-binding representation exists; no duplicate Project model.  
**test artifacts:** old serialized fixtures; new serialized fixtures; directory/repo matching goldens; ambiguity tests.  
**decisions already resolved:** ProjectBinding is an operating matcher, not Project.  
**prohibited hidden decisions:** destructive rename without compatibility audit; adding Canon/Project semantics into binding.  
**determination status:** `CURRENT DESIGN`.

---

## B.03 — AIKit Project resource declaration

**id:** `B.03`  
**purpose:** provide Project-scoped AIKit operating declarations distinct from Factory Project metadata.  
**owner/system:** AIKit.  
**inputs:** `ProjectRef`; Profiles; resource provider refs.  
**outputs:** parsed project resource declarations, potentially `.aikit/project.toml`.  
**dependencies:** B.02; resource interfaces.  
**interfaces:** B.I03–B.I07.  
**source basis:** Constitutional AIKit role; observed `.aikit` profile model; bkmr target-project design.  
**acceptance:** project resource configuration resolves deterministically; no Project Canon content required or duplicated.  
**test artifacts:** schema goldens; nested Project/profile precedence; invalid refs; missing optional providers.  
**decisions already resolved:** `.aikit` is AIKit operational state.  
**prohibited hidden decisions:** using AIKit declaration as canonical Project identity or Project Map store.  
**determination status:** `CURRENT DESIGN`.

---

## B.04 — Typed resource index plane

**id:** `B.04`  
**purpose:** extend AIKit from capability-only emphasis to typed actor/project resources without ontology flattening.  
**owner/system:** AIKit core/store.  
**inputs:** Capability, Action, Agent, Agency, Model, Harness, Workcell, ContextSource and QL resource descriptors.  
**outputs:** reconstructable typed indexes.  
**dependencies:** shared A/E/F/H/I contracts; B.03.  
**interfaces:** B.I03–B.I07.  
**source basis:** Constitutional Index AIKit full role; Deep QL Action/resource indexing.  
**acceptance:** source owner/provenance retained for every indexed item; index rebuild loses no authored meaning.  
**test artifacts:** mixed-resource fixture; provider disappearance; duplicate Ref conflict; rebuild test.  
**decisions already resolved:** not every resource is a Capsule.  
**prohibited hidden decisions:** generic untyped resource blob; AIKit-generated Action/Agent identity.  
**determination status:** `CURRENT DESIGN dependent on imported contracts`.

---

## B.05 — ContextResolution composition

**id:** `B.05`  
**purpose:** compose existing AIKit capability resolution with typed Project/actor/execution/source resource views.  
**owner/system:** AIKit.  
**inputs:** ContextDescriptor; ResolvedView; ProjectRef; Focus; typed resources; trust/availability; profiles/scopes.  
**outputs:** `ContextResolution`.  
**dependencies:** B.04; D.01; A/I Context interface.  
**interfaces:** B.I08.  
**source basis:** canonical Context relation + existing AIKit resolver.  
**acceptance:** existing capability resolution behavior remains unchanged; extended resources have deterministic provenance/explanation.  
**test artifacts:** scope/profile/resource goldens; identical-input determinism; missing optional provider.  
**decisions already resolved:** ContextResolution is beneath Context.  
**prohibited hidden decisions:** renaming Context to ResolvedContext globally; frecency overriding hard policy.  
**determination status:** `CURRENT DESIGN`.

---

## B.06 — Epi-Logos profile Agent-resource projection

**id:** `B.06`  
**purpose:** make canonical Epi-Logos Agent identities and resources first-class in Project/Context resolution.  
**owner/system:** AIKit profile integration consuming I.  
**inputs:** Epi-Logos Project/Profile; canonical AgentRefs; optional Agency/QL/Bimba resources.  
**outputs:** resolved orchestrator + six Agent resources and related profile-scoped resources.  
**dependencies:** I Agent schema; H QL resource contract; B.04.  
**interfaces:** B.I03, B.I07.  
**source basis:** Constitutional Index Epi-Logos profile.  
**acceptance:** Epi profile resolves exactly orchestrator + six canonical AgentRefs; generic fixture does not; model/harness changes preserve refs.  
**test artifacts:** Epi profile golden; generic profile golden; model-switch fixture; Bimba-unavailable fixture.  
**decisions already resolved:** named Agents are enduring identities, not stage labels or Agencies.  
**prohibited hidden decisions:** creating one Agent per model; universalising Epi ontology.  
**determination status:** `CONSTITUTIONAL semantics + CURRENT DESIGN integration`.

---

## B.07 — CapabilitySet / ActionSet harmonisation

**id:** `B.07`  
**purpose:** support Capability and Action collections without transferring ownership.  
**owner/system:** AIKit.  
**inputs:** existing SkillSet/CapabilitySet structures; Action Catalog.  
**outputs:** resolved CapabilitySet and ActionSet views.  
**dependencies:** E Action standard; B.04.  
**interfaces:** B.I02, B.I04.  
**source basis:** Primitive Relations + Deep QL.  
**acceptance:** current skill-only behavior remains compatible; ActionRefs remain Project-owned; member trust/availability remains individual.  
**test artifacts:** mixed sets; unavailable Action; untrusted Capability; legacy SkillSet migration.  
**decisions already resolved:** Actions inhabit wider power field but are not identical to Capability implementation types.  
**prohibited hidden decisions:** copying Action bodies into AIKit sets.  
**determination status:** `CURRENT DESIGN`.

---

## B.08 — Shared application-service/headless contracts

**id:** `B.08`  
**purpose:** expose Project/Context/resource operations through multiple surfaces over one semantic service.  
**owner/system:** AIKit application layer.  
**inputs:** B.02–B.07 services.  
**outputs:** stable service/API operations usable by CLI/TUI/headless/future UI.  
**dependencies:** B.05.  
**interfaces:** product M/Z.  
**source basis:** observed current shared CLI/TUI service plus constitutional dual UX.  
**acceptance:** semantically identical operation returns same refs/explanation regardless of surface.  
**test artifacts:** service/CLI JSON parity; TUI backend parity; headless schema snapshots.  
**decisions already resolved:** no frontend-specific resolver/business logic.  
**prohibited hidden decisions:** freezing current TUI topology; deciding final command wording.  
**determination status:** `CONSTITUTIONAL invariant + CURRENT DESIGN`.

---

## B.09 — Learned ergonomic resolution

**id:** `B.09`  
**purpose:** extend asset memory across Project World resources while preserving signal distinctions.  
**owner/system:** AIKit + J.  
**inputs:** UsageSignals, FitnessObservations, preferences, trust, availability, context relevance.  
**outputs:** inspectable ranking/suggestions.  
**dependencies:** J schemas; B.04.  
**interfaces:** search, resource resolution, product surfaces.  
**source basis:** Primitive Relations asset memory; Constitutional AIKit learned ease.  
**acceptance:** each signal independently inspectable; hard constraints never overridden by learned rank.  
**test artifacts:** ranking goldens; conflicting-signal fixture; unavailable-but-frequent fixture.  
**decisions already resolved:** no universal opaque AI score.  
**prohibited hidden decisions:** frecency = fitness; preference = trust.  
**determination status:** `CURRENT DESIGN dependent on J`.

---

# D.* — Project Map and Information Horizon

## D.01 — Project Map declaration and provider contracts

**id:** `D.01`  
**purpose:** establish canonical join/navigation contract and provider manifest.  
**owner/system:** Project Map.  
**inputs:** ProjectRef; provider declarations; imported Refs.  
**outputs:** ProjectMap declaration and typed provider registry.  
**dependencies:** A Ref grammar.  
**interfaces:** D.I01–D.I02.  
**source basis:** Constitutional Index + Primitive Relations.  
**acceptance:** each provider has identity/source/authority-domain/revision/freshness/health/retrieval metadata; provider content remains external.  
**test artifacts:** manifest goldens; duplicate provider; invalid Ref; unavailable provider.  
**decisions already resolved:** Project Map is an index/join.  
**prohibited hidden decisions:** universal graph-store requirement; new Ref grammar.  
**determination status:** `CONSTITUTIONAL boundary + CURRENT DESIGN schema`.

---

## D.02 — Git/source and code-as-map provider

**id:** `D.02`  
**purpose:** expose authored source state as the cheapest direct Project entry.  
**owner/system:** D adapter over Git/source.  
**inputs:** source constituents.  
**outputs:** map entries with revision/dirty/source location.  
**dependencies:** D.01.  
**interfaces:** D.I03.  
**source basis:** Architecture/Primitive Relations.  
**acceptance:** clean/dirty/revision states accurately distinguishable; module links optional and direct.  
**test artifacts:** Git fixtures; nested repo; renamed module; dirty worktree.  
**decisions already resolved:** source is authoritative for what source exists.  
**prohibited hidden decisions:** generated prose mirror of every file.  
**determination status:** `CURRENT DESIGN`.

---

## D.03 — Real GitNexus provider

**id:** `D.03`  
**purpose:** integrate structural code intelligence through actual GitNexus seam.  
**owner/system:** D adapter.  
**inputs:** ProjectRef; Git source revision; verified SourceIntegrationRef.  
**outputs:** structural context/impact/trace/change entries.  
**dependencies:** D.01; L.03 GitNexus verification.  
**interfaces:** D.I04.  
**source basis:** Architecture/Constitutional Project Map.  
**acceptance:** real GitNexus indexes canonical fixture and returns expected structure; stale revision detectable.  
**test artifacts:** real repository index; context/impact/trace tests; stale-index fixture.  
**decisions already resolved:** GitNexus owns code graph extraction.  
**prohibited hidden decisions:** local approximate call graph.  
**determination status:** `CURRENT DESIGN — SOURCE BLOCKED`.

---

## D.04 — Canon and semantic-wiki providers

**id:** `D.04`  
**purpose:** make recognised Project intention/design and project language navigable.  
**owner/system:** Project Map.  
**inputs:** Canon refs; Markdown wiki roots.  
**outputs:** provenance-bearing entries/relations.  
**dependencies:** D.01; A Canon interfaces.  
**interfaces:** D.I05–D.I06.  
**source basis:** Primitive Relations.  
**acceptance:** recognised/superseded material distinguished; broken wiki links visible.  
**test artifacts:** semantic wiki fixture; superseded Canon; missing-link tests.  
**decisions already resolved:** wiki participates in Project semantics but is not all Canon.  
**prohibited hidden decisions:** generated wiki material automatically recognised as truth.  
**determination status:** `CURRENT DESIGN`.

---

## D.05 — Information-horizon provider layer and bkmr adapter

**id:** `D.05`  
**purpose:** progressively retrieve heterogeneous Project knowledge.  
**owner/system:** D/B integration.  
**inputs:** ContextSource descriptors; verified provider SourceIntegrations.  
**outputs:** search/retrieval results with provenance/freshness.  
**dependencies:** D.01; B.03; L.03 bkmr for production bkmr adapter.  
**interfaces:** D.I07.  
**source basis:** Primitive Relations bkmr role; Constitutional Index.  
**acceptance:** multiple projects bind independent knowledge horizons; retrieval identifies source/provider/revision.  
**test artifacts:** project A/B isolation; offline text retrieval; provider unavailable; stale index.  
**decisions already resolved:** bkmr is one horizon provider, not Context.  
**prohibited hidden decisions:** hidden global active DB; knowledge index becomes source truth.  
**determination status:** `provider interface READY; bkmr implementation SOURCE BLOCKED`.

---

## D.06 — Action, Run, Evolution and Candidate navigation

**id:** `D.06`  
**purpose:** connect operational and developmental Project surfaces into navigation.  
**owner/system:** Project Map.  
**inputs:** ActionRefs, RunRefs, RunMapRefs, CandidateRefs, Decision/recognition refs.  
**outputs:** cross-linked Map entries and Evolution paths.  
**dependencies:** E Action API; A/C Run/Candidate API.  
**interfaces:** D.I08–D.I09.  
**source basis:** Constitutional Index.  
**acceptance:** current state can be traced to relevant Decisions/Runs/Candidates without Git archaeology; Action source remains external.  
**test artifacts:** branching Run fixture; recognised Candidate fixture; Action→module→Run fixture.  
**decisions already resolved:** Evolution is derived from developmental history.  
**prohibited hidden decisions:** Git commit history substituted for Run/Decision semantics.  
**determination status:** `CURRENT DESIGN dependent on A/C/E`.

---

## D.07 — QL/MEF and Bimba provider surfaces

**id:** `D.07`  
**purpose:** make QL/MEF and optional Bimba first-class navigable semantic resources without making them generic dependencies.  
**owner/system:** D consuming H/profile providers.  
**inputs:** QLFormRefs, QLAddresses, LensRefs, Bimba provider when enabled.  
**outputs:** ProjectMap entries and derived refraction links.  
**dependencies:** H; optional Bimba SourceIntegration.  
**interfaces:** D.I10.  
**source basis:** Deep QL + Constitutional profile semantics.  
**acceptance:** same canonical Ref retained through lens readings; all twelve lenses addressable; absent Bimba/QL service degrades only those resources.  
**test artifacts:** QL service on/off; Bimba on/off; Ref identity preservation; twelve-lens enumeration.  
**decisions already resolved:** MEF remains whole; Bimba profile-gated.  
**prohibited hidden decisions:** lens-created object identity; Bimba generic requirement.  
**determination status:** `CONSTITUTIONAL semantics; implementation partially OPEN/SOURCE BLOCKED`.

---

## D.08 — Authority/freshness/health service

**id:** `D.08`  
**purpose:** answer where authority for a question lies and whether the indexed view is current.  
**owner/system:** Project Map.  
**inputs:** provider health/freshness/authority-domain metadata.  
**outputs:** ProjectMapHealth and authority routing explanations.  
**dependencies:** D.01–D.07.  
**interfaces:** Project entry; K; M.  
**source basis:** Project Map authority/freshness discipline.  
**acceptance:** stale/unknown/unavailable never shown as fresh; source disagreements preserved.  
**test artifacts:** stale GitNexus; stale bkmr; unavailable Bimba; conflicting Canon/source claim.  
**decisions already resolved:** routing to authority is not general truth adjudication.  
**prohibited hidden decisions:** one universal confidence/authority scalar.  
**determination status:** `CURRENT DESIGN`.

---

## D.09 — Progressive retrieval and ContextLoad

**id:** `D.09`  
**purpose:** make available/retrieved/loaded relations explicit and traceable.  
**owner/system:** D retrieval + I/J execution trace integration.  
**inputs:** InformationHorizon, Focus, retrieval budget, ExecutionRef/HarnessRef.  
**outputs:** Retrieval records and ContextLoad.  
**dependencies:** D.05; I/J execution/telemetry schemas.  
**interfaces:** D.I11.  
**source basis:** constitutional progressive Context.  
**acceptance:** every loaded source traces to retrieval and provider revision; same source may be loaded in one Execution and not another.  
**test artifacts:** dual-execution fixture; retrieval budget; stale result; partial load; provider failure.  
**decisions already resolved:** loaded state is execution-relative.  
**prohibited hidden decisions:** global loaded flag; maximal prompt stuffing.  
**determination status:** `CURRENT DESIGN dependent on I/J`.

---

# K.* — Project Bootstrap

## K.01 — Bootstrap as ordinary Factory Run

**id:** `K.01`  
**purpose:** make Project genesis/adoption a normal durable Factory transformation.  
**owner/system:** K consuming A/C.  
**inputs:** repository/local directory/new intention/existing ProjectRef.  
**outputs:** Bootstrap RunRef/RunMapRef and entrance state.  
**dependencies:** A/C Run contracts.  
**interfaces:** K.I01.  
**source basis:** Constitutional Bootstrap invariant.  
**acceptance:** Bootstrap survives session/host boundaries like any Run.  
**test artifacts:** four entrance fixtures.  
**decisions already resolved:** no separate installer ontology.  
**prohibited hidden decisions:** local bootstrap state becomes canonical Run replacement.  
**determination status:** `CONSTITUTIONAL`.

---

## K.02 — Evidence-first Project inspector

**id:** `K.02`  
**purpose:** recover determinable Project reality before asking human questions.  
**owner/system:** Bootstrap.  
**inputs:** source/Git/docs/tests/runtime/Actions/resources/history/providers.  
**outputs:** source inspection and recovery evidence.  
**dependencies:** D.02; optional verified providers.  
**interfaces:** K.I02.  
**source basis:** Constitutional Index/Primitive Relations.  
**acceptance:** fixture-determinable facts recovered autonomously.  
**test artifacts:** mature/sparse/re-entry inspections; forbidden-question assertions.  
**decisions already resolved:** evidence before interrogation.  
**prohibited hidden decisions:** human asked to restate code facts.  
**determination status:** `CONSTITUTIONAL`.

---

## K.03 — Ground / Intent / Design recovery

**id:** `K.03`  
**purpose:** recover current Project orientation and distinguish actuality from intention.  
**owner/system:** K using A/G Claims/Artifacts.  
**inputs:** K.02 evidence.  
**outputs:** recovered Ground/Intent/Design Claims/Artifacts with evidence status.  
**dependencies:** A/G epistemic contracts.  
**interfaces:** K.I02.  
**source basis:** Primitive Relations Bootstrap.  
**acceptance:** OBSERVED behavior never silently promoted to intended behavior; contradictions explicit.  
**test artifacts:** conflicting docs/code; obsolete README; sparse repo.  
**decisions already resolved:** existing claims are evidence, not automatic current Canon.  
**prohibited hidden decisions:** inventing missing vision from code shape.  
**determination status:** `CONSTITUTIONAL method + CURRENT DESIGN implementation`.

---

## K.04 — Minimal authorial frontier

**id:** `K.04`  
**purpose:** isolate only genuinely consequential unresolved human authorship.  
**owner/system:** Bootstrap/A HumanRequest boundary.  
**inputs:** recovered Claims, contradictions, missing Intent/Design.  
**outputs:** Decision/HumanRequest refs.  
**dependencies:** K.03; A authority contracts.  
**interfaces:** K.I03.  
**source basis:** Commission/Recognition constitution.  
**acceptance:** expected semantic question set equals fixture authorial frontier; ordinary engineering questions absent.  
**test artifacts:** per-fixture expected-human-questions files.  
**decisions already resolved:** human altitude is vision/experience/foundational meaning/recognition.  
**prohibited hidden decisions:** ask-for-confirmation-everywhere workflow.  
**determination status:** `CONSTITUTIONAL`.

---

## K.05 — Project World surface establishment

**id:** `K.05`  
**purpose:** establish missing Project Map/source/AIKit/semantic surfaces while respecting ownership.  
**owner/system:** K orchestrating B/D/L/A owners.  
**inputs:** recognised/recovered Project orientation.  
**outputs:** ProjectManifest; ProjectMap declaration; SourceIntegration declarations; AIKit bindings/profile/resources; justified semantic scaffold.  
**dependencies:** B.02/B.03, D.01, L.01, A Project/Canon.  
**interfaces:** K.I04.  
**source basis:** joined Project World design.  
**acceptance:** minimal non-duplicative committed surfaces; each file has one owner.  
**test artifacts:** exact fixture trees; idempotence snapshots.  
**decisions already resolved:** K coordinates rather than owning downstream semantics.  
**prohibited hidden decisions:** one giant Factory metadata file.  
**determination status:** `CURRENT DESIGN`.

---

## K.06 — Action/runtime/resource/source discovery handoff

**id:** `K.06`  
**purpose:** hand discovered operational structure to its canonical subsystem.  
**owner/system:** K.  
**inputs:** evidence from source/runtime/project.  
**outputs:** Action discovery Claims to E; runtime demands/descriptions to F; AIKit resource declarations to B; source plans to L.  
**dependencies:** E/F/B/L interfaces.  
**interfaces:** K.I05–K.I07.  
**source basis:** Constitutional Bootstrap + Deep QL Agent-Native.  
**acceptance:** existing canonical operations/runtime seams referenced rather than reimplemented by Bootstrap.  
**test artifacts:** native Action project; legacy API project; Compose runtime; unknown upstream source.  
**decisions already resolved:** discovery is not ownership.  
**prohibited hidden decisions:** Bootstrap creates a duplicate Action framework or provider scheduler.  
**determination status:** `CURRENT DESIGN dependent on E/F`.

---

## K.07 — Bootstrap recognition and ordinary developmental frontier

**id:** `K.07`  
**purpose:** close Bootstrap coherently into normal Project life.  
**owner/system:** K consuming A/C Recognition/Recursion.  
**inputs:** operable Project; Bootstrap Claims/Evidence; human Decision where required.  
**outputs:** recognised Bootstrap outcome; current Ground; Project developmental frontier; ordinary Run-ready state.  
**dependencies:** A/C.  
**interfaces:** K.I08.  
**source basis:** Constitutional recursion.  
**acceptance:** next ordinary Run can orient without reconstructing Bootstrap; no misuse of Root Wayfinder terminology.  
**test artifacts:** bootstrap→first ordinary Run fixture.  
**decisions already resolved:** Bootstrap Run Map is Bootstrap's map; root program map is distinct.  
**prohibited hidden decisions:** issue-per-document-heading handoff.  
**determination status:** `CONSTITUTIONAL terminology + CURRENT DESIGN handoff`.

---

## K.08 — Re-bootstrap / long-absence reconciliation

**id:** `K.08`  
**purpose:** re-enter an existing Factory Project from retained identity/history and current evidence.  
**owner/system:** K/B/D/L consuming A/C history.  
**inputs:** existing ProjectRef; prior Ground/Canon/Run history; current source/provider/source-integration state.  
**outputs:** reconciliation delta; refreshed health; current Project entry/frontier.  
**dependencies:** K.01–K.07; D.08; L.04; B.05.  
**interfaces:** Project entry export.  
**source basis:** return-after-absence constitution.  
**acceptance:** same Project identity retained; stale derived indexes refreshable; no repeated foundational questionnaire.  
**test artifacts:** six-month-change fixture; deleted derived indexes; changed upstream; new recognised Decision.  
**decisions already resolved:** re-bootstrap is reconciliation, not Project recreation.  
**prohibited hidden decisions:** destructive reset of Canon/Profile/Project identity.  
**determination status:** `CURRENT DESIGN implementing CONSTITUTIONAL journey`.

---

# L.* — Source fidelity

## L.01 — SourceIntegration declaration and lock contracts

**id:** `L.01`  
**purpose:** create durable authored and resolved source-fidelity records.  
**owner/system:** L.  
**inputs:** SourceIntegrationRef; intended upstream role/seam.  
**outputs:** declaration and resolved-lock schemas.  
**dependencies:** shared Ref grammar.  
**interfaces:** L.I01–L.I02.  
**source basis:** Architecture/Primitive/Constitutional source-fidelity rule.  
**acceptance:** all supported integration modes; pin/licence/seam/augmentation/verification/upgrade fields representable.  
**test artifacts:** schema goldens for dependency/CLI/protocol/mount/capability/fork/reference.  
**decisions already resolved:** declaration ≠ lock.  
**prohibited hidden decisions:** floating unrecorded `latest`.  
**determination status:** `CURRENT DESIGN`.

---

## L.02 — Real source inspector

**id:** `L.02`  
**purpose:** replace documentation assumptions with actual upstream source evidence.  
**owner/system:** L.  
**inputs:** declaration + pinned source.  
**outputs:** inspection evidence; actual seam inventory; licence/provenance evidence.  
**dependencies:** L.01.  
**interfaces:** L.I02.  
**source basis:** source-code-is-evidence principle.  
**acceptance:** actual reusable paths/API/CLI/protocol exist at pin; discrepancies recorded.  
**test artifacts:** moved API; missing CLI; invalid docs claim; licence mismatch.  
**decisions already resolved:** documentation does not verify implementation.  
**prohibited hidden decisions:** guessing current upstream seam.  
**determination status:** `CONSTITUTIONAL discipline + CURRENT DESIGN`.

---

## L.03 — Real seam verifier

**id:** `L.03`  
**purpose:** prove the integration through the actual upstream system.  
**owner/system:** L + owning adapter.  
**inputs:** inspected seam and local adapter/dependency.  
**outputs:** verification Evidence; VERIFIED/BLOCKED status.  
**dependencies:** L.02.  
**interfaces:** L.I03.  
**source basis:** constitutional source-fidelity invariant.  
**acceptance:** representative real operation executes through upstream seam.  
**test artifacts:** real CLI/protocol/library smoke test; incompatible revision.  
**decisions already resolved:** mock-only integration does not satisfy verification.  
**prohibited hidden decisions:** substituting imitation to make test pass.  
**determination status:** `CONSTITUTIONAL`.

---

## L.04 — Drift and upgrade planning

**id:** `L.04`  
**purpose:** detect and manage source evolution without invisible mutation.  
**owner/system:** L.  
**inputs:** verified lock + current upstream state.  
**outputs:** DriftReport; UpgradePlan; migration evidence.  
**dependencies:** L.03.  
**interfaces:** L.I04.  
**source basis:** source reproducibility discipline.  
**acceptance:** drift check is read-only; incompatible change visible; old pin retained until accepted.  
**test artifacts:** compatible bump; breaking bump; licence change; source unavailable.  
**decisions already resolved:** upgrade is explicit.  
**prohibited hidden decisions:** auto-follow upstream main/latest.  
**determination status:** `CURRENT DESIGN`.

---

## L.05 — Cross-layer reproducibility envelope

**id:** `L.05`  
**purpose:** connect Project/source/Context/execution/materialisation evidence sufficiently for reconstruction.  
**owner/system:** L coordinating A/B/D/F/I/J.  
**inputs:** Project/source locks; ContextResolution; ContextLoad; ExecutionDemand; MaterialisedExecutionWorldRef; execution evidence.  
**outputs:** ReproducibilityEnvelope/ref set.  
**dependencies:** cross-map execution contracts.  
**interfaces:** L.I05.  
**source basis:** Architecture + Workcell four-layer reproducibility.  
**acceptance:** canonical fixture execution can be explained/rematerialised on another suitable Workcell without semantic Project change.  
**test artifacts:** second-Workcell recreation; missing source pin; unavailable provider.  
**decisions already resolved:** reproducibility ≠ identical physical machine or identical LLM tokens.  
**prohibited hidden decisions:** unrecorded local state treated as reproducible.  
**determination status:** `CURRENT DESIGN dependent on F/I/J`.

---

## L.06 — Source-fidelity development gate

**id:** `L.06`  
**purpose:** mechanically prevent implementation nodes from silently bypassing named upstream dependencies.  
**owner/system:** L + Run/development gates.  
**inputs:** node SourceIntegrationRef requirements; integration status.  
**outputs:** allow/block Evidence.  
**dependencies:** L.03.  
**interfaces:** A/C development gate.  
**source basis:** Constitutional source-fidelity rule.  
**acceptance:** only dependent nodes block; explicit substitution Decision is required for non-upstream replacement.  
**test artifacts:** verified source; unverified source; unrelated node; substitution Decision.  
**decisions already resolved:** planning can precede source verification; dependent implementation cannot.  
**prohibited hidden decisions:** broad global freeze; silent temporary imitation.  
**determination status:** `CONSTITUTIONAL behavior + CURRENT DESIGN gate`.

---

# 29. Required root fixtures contributed by Map 2

## Fixture 1 — External repo → Bootstrap → Project Map → first Run

```text
external repository
→ K evidence recovery
→ ProjectRef established/recovered
→ Project Map
→ AIKit ProjectBinding/profile
→ SourceIntegrations
→ recognised Bootstrap outcome
→ first ordinary Project Run
```

Acceptance:

```text
no unnecessary human reconstruction
all source-derived claims carry evidence
```

---

## Fixture 2 — Fresh Project → Intent/Design/Actions → first Candidate

Consumes A/E/C.

Acceptance:

```text
fresh Project authors real Intent/Design
Action surface designed
first ordinary Run creates CandidateRef
Project World is already navigable
```

---

## Fixture 3 — Long absence → Evolution/current frontier without archaeology

Acceptance:

```text
same ProjectRef
recognised current Ground
meaningful delta
relevant Runs/Decisions
provider/source health
entry recommendation
```

without requiring reading raw logs.

---

## Fixture 4 — Real pinned upstream seam

Use at least one production SourceIntegration.

Acceptance:

```text
declaration
pin
inspection
real seam
real integration test
Evidence
drift baseline
```

No local imitation.

---

## Fixture 5 — Sparse repo minimal questioning

Fixture intentionally has:

```text
working source
tests
runtime evidence
little product documentation
```

Acceptance:

```text
system discovers implementation facts
human asked only genuinely authorial product/foundational questions
```

---

## Fixture 6 — Stale derived providers

Deliberately stale:

```text
GitNexus
bkmr
another derived Project Map provider
```

Acceptance:

```text
staleness visible
canonical source remains correct
provider can be rebuilt
no derived cache overwrites source truth
```

---

## Fixture 7 — Available / Retrieved / Loaded

Acceptance:

```text
large Information Horizon
subset retrieved
smaller subset loaded
ContextLoad names ExecutionRef + HarnessRef
two Executions may have different loaded sets
```

---

## Fixture 8 — Epi-Logos Agent resources

Epi profile:

```text
0/1 Epi-Logos orchestrator
#0 Anuttara
#1 Paramasiva
#2 Parāśakti
#3 Mahāmāyā
#4 Nara
#5 Epii
```

Acceptance:

```text
all seven AgentRefs resolved

generic project:
    none implicitly injected

switch ModelRef:
    AgentRefs unchanged

switch HarnessRef:
    AgentRefs unchanged
```

---

## Fixture 9 — Bimba / QL absence

Acceptance:

```text
Bimba unavailable
QL executable service unavailable

still works:
    Project
    Project Map ordinary providers
    Bootstrap
    AIKit ordinary resolution
    ordinary retrieval
    SourceIntegration
    ordinary Actions

unavailable:
    only dependent Bimba/QL capabilities/refractions
```

---

## Fixture 10 — Recognition / Recursion retained difference

Start:

```text
Project entry at Ground₀
```

Complete Run with recognised change.

Next entry must show:

```text
Ground₁
updated Canon/map/history where appropriate
recognised Decision/Candidate relation
changed developmental frontier
```

Acceptance:

```text
durable recognised difference survives session/model/host restart
```

---

# 30. Additional acceptance suites

## AIKit resolution goldens

Prove:

```text
managed policy not overridable

global
< host
< project
< project-local
< session
< task
< one-shot

nested Project resolution deterministic

explicit disabled dependency not silently enabled

conflict fails

search/tag matching does not activate

trust independent of preference

identical inputs produce stable resolution identity

ProjectBinding ambiguity fails visibly
```

---

## AIKit materialisation tests

Prove:

```text
failed apply changes nothing
stale Generation base rejected
derived Generation rebuildable
projection root cannot escape
shared task isolation never pretended private
activation/materialisation effect explicit
```

---

## Project Map integrity

Prove:

```text
every entry has provider/source
authority-domain routing explainable
revision/freshness visible
stale index detected
provider cache rebuildable
broken relations visible
one provider cannot overwrite another provider's source identity
```

---

## Semantic wiki fixtures

Prove:

```text
plain Markdown readability
wiki-link resolution
code/design/Run relations
broken links
supersession links
direct editability
```

---

## GitNexus fixtures

After source verification:

```text
index canonical repo
symbol context
impact
trace
source changes
index becomes stale
refresh
new result
```

Real GitNexus required.

---

## bkmr fixtures

After source verification:

```text
Project A/B isolation
source provenance
text search
semantic availability reporting
no hidden current DB
stale index
provider unavailable
```

---

## QL/MEF fixtures

```text
all twelve LensRefs available

same subject Ref before/after refraction

QL service absent:
    ordinary Project works

QL service present:
    derived reading available

Epi profile:
    QL resources available according to profile

generic profile:
    no Epi-specific resource injection
```

---

## Source-fidelity fixtures

```text
unverified source blocks dependent node

unrelated node remains buildable

verified source passes

upstream seam changed
    verification fails

reference implementation explicitly marked reference

vendored fork records upstream base

licence drift visible

substitution requires Decision
```

---

# 31. Code-health and architectural taste gate

## Mechanical

Each applicable implementation slice passes:

```text
unit tests
integration tests
real external seam tests
type/compiler checks
lint
format/diff checks
migration tests
fixture goldens
source-fidelity gates
```

For current AIKit, its own advertised verification commands remain the appropriate baseline:

```text
cargo test --locked --workspace --all-targets --no-fail-fast
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo build --locked --workspace --release
git diff --check
```

A Project World release claim cannot bypass them merely because feature tests pass.

---

## Architectural taste

Reject implementation introducing:

```text
AIKit-owned Project meaning

AIKit-owned Run semantics

AIKit-owned Action business logic

AIKit-owned Workcell provider scheduling

AIKit ContextResolution treated as canonical Context

Project Map universal truth database

bkmr treated as Project Map

GitNexus treated as source truth

global mutable loaded-context flag

duplicated Project representations

generic Resource blob replacing Agent/Action/Capability identities

Epi-Logos Agent names demoted to local Agency tags

Epi-Logos Agent names universalised into generic Factory stages

Bimba generic dependency

two-object QL ontology alongside Factory Refs

subset of MEF presented as whole MEF

unverified upstream imitation

SourceIntegration with floating unknown source

Bootstrap questionnaire for recoverable code facts

provider-specific infrastructure in Project semantic manifests

derived cache treated as canonical

entire Information Horizon injected into every model
```

Prefer:

```text
clear ownership
stable imported Refs
small manifests
provider adapters
real upstream reuse
rebuildable indexes
precise provenance
deterministic resolution
direct source navigation
progressive retrieval
explicit degraded states
narrow interfaces
clean migration paths
```

---

# 32. Build programme

## Slice 0 — Establish trustworthy Ground

Parallel:

```text
B.01 live AIKit inventory / health

L.01 SourceIntegration schemas

D.01 Project Map provider contract

shared A/E/F/I/H Ref/interface reconciliation
```

B.01's current CI issue does not block L.01/D.01 design.

---

## Slice 1 — Project identity without representation collapse

Prove:

```text
ProjectRef
    │
    ├── small ProjectManifest
    │
    └── AIKit ProjectBinding
```

Physical matching changes without Project identity changing.

---

## Slice 2 — Source + Canon Project Map

Implement source-independent first:

```text
Git/source provider
code headers
Canon provider
semantic wiki provider
Map health
```

Proves Project Map without external provider dependencies.

---

## Slice 3 — Generic versus Epi-Logos Context resolution

Prove:

```text
generic Project
    generic/custom Agent resources only

Epi-Logos Project/Profile
    orchestrator + canonical six
    QL/MEF resources
    optional Bimba

same AIKit resolution architecture
```

---

## Slice 4 — Project re-entry

Implement the information/service export required for:

```text
return after six months
```

using canonical fixture Run history and Project Map health.

Final human surface remains M-owned.

---

## Slice 5 — First real SourceIntegration-backed provider

Choose an upstream after L verification.

A strong candidate is GitNexus because it proves:

```text
real upstream reuse
derived freshness
Project Map navigation
source/canon distinction
```

---

## Slice 6 — Progressive knowledge horizon

After bkmr or another provider is verified:

```text
large Available set
→ focused retrieval
→ smaller ContextLoad
```

---

## Slice 7 — Mature repository Bootstrap

Full path:

```text
repo
→ evidence
→ recovered Ground/Intent/Design
→ minimal authorial frontier
→ Project world surfaces
→ recognised Bootstrap
→ first ordinary Run
```

---

## Slice 8 — Sparse + fresh Bootstrap

Proves the distinction:

```text
recover what exists
versus
author what does not
```

---

## Slice 9 — Re-bootstrap and Recursion retention

Delete derived indexes, change source, advance one recognised Run and drift one upstream.

Return to Project.

The authored whole must remain intelligible.

---

# 33. Cross-map harmonisation export

# OWNED TERMS

Map 2 owns or is primary design owner for:

```text
ProjectBinding

ProjectManifest
    as small Project entry/reference surface

ProjectMap
ProjectMapProvider
ProjectMap entry/provider health/freshness concepts

Information-horizon provider integration

Available / Retrieved / Loaded relation

ContextLoad
    subject to I/J execution identity

Bootstrap process contract
Bootstrap evidence classes
Bootstrap reconciliation/idempotence

SourceIntegration declaration/lock lifecycle
Source-fidelity gate
source drift/upgrade relation

AIKit typed Project-resource indexing design
ContextResolution
```

---

# IMPORTED TERMS

```text
Ref / ProjectRef

RunRef / RunMapRef

AgentRef / AgencyRef

ActionRef
CapabilityRef

CandidateRef
ClaimRef

SourceIntegrationRef
    identity grammar imported even though L owns lifecycle/content

ExecutionDemand

WorkcellRef
MaterialisedExecutionWorldRef

ExecutionRef
ModelRef
HarnessRef

QLFormRef
QLAddress
LensRef

Decision / HumanRequest
Artifact / Evidence
Recognition / Recursion
Project Canon
```

No local grammar overrides these shared terms.

---

# EXPORTED INTERFACES

```text
B.I01 ProjectBindingResolution
B.I02 CapabilityResolution
B.I03 ActorResourceIndex
B.I04 ActionResourceIndex
B.I05 ExecutionResourceIndex
B.I06 ContextSourceIndex
B.I07 QLResourceIndex
B.I08 ContextResolution
B.I09 ContextProjection

D.I01 ProjectMap
D.I02 ProviderHealth
D.I03 SourceNavigation
D.I04 CodeIntelligence
D.I05 CanonNavigation
D.I06 SemanticNavigation
D.I07 KnowledgeRetrieval
D.I08 ActionNavigation
D.I09 EvolutionNavigation
D.I10 QLRefractionNavigation
D.I11 ContextLoad

K.I01 BootstrapRun
K.I02 BootstrapRecovery
K.I03 AuthorialFrontier
K.I04 ProjectSurfaceHandoff
K.I05 ActionDiscoveryHandoff
K.I06 RuntimeDiscoveryHandoff
K.I07 SourcePlanHandoff
K.I08 BootstrapRecognition

L.I01 SourceDeclaration
L.I02 SourceLock
L.I03 SourceVerification
L.I04 DriftReport
L.I05 ReproducibilityEnvelope
```

---

# CURRENT DESIGN

```text
.factory/project.toml
.factory/project-map.toml
.factory/sources.toml
.factory/sources.lock.toml

small ProjectManifest

.aikit/project.toml as target AIKit resource declaration surface

ProjectBinding semantic correction

typed AIKit resource indexes

ContextResolution envelope

ProjectMap provider contract

map-level freshness states

ContextLoad

SourceIntegration lifecycle/status model

Bootstrap evidence-status vocabulary

recommended vertical slice order
```

Filesystem naming remains subject to Z ratification.

---

# CONSTITUTIONAL DETERMINATIONS

```text
Project > repository

Project identity survives repository restructuring

Context = Operative World + Information Horizon + Focus

Context is not Generation / ContextDescriptor / ResolvedView / ContextResolution

AIKit indexes/resolves; it does not own Project meaning

Actions remain Project/Application-owned

Project Map joins; it does not replace provider stores

available ≠ retrieved ≠ loaded

Workcell materialises ExecutionDemand below Context

derived indexes are reconstructable

Project Bootstrap is a Factory Run

Bootstrap recovers evidence before questioning

human attention remains at genuine authorship/recognition

Epi-Logos profile:
    0/1 orchestrator
    #0 Anuttara
    #1 Paramasiva
    #2 Parāśakti
    #3 Mahāmāyā
    #4 Nara
    #5 Epii

those Agents are enduring identities, not universal Factory stages

Bimba is profile-gated

QL/MEF remains first-class

all twelve MEF lenses remain available

QL readings preserve canonical subject identity

ordinary Factory operation survives QL-service absence

SourceIntegrations use real upstream seams

documentation does not equal verification
```

---

# OPEN PRODUCT DECISIONS

```text
exact public CLI wording for:
    Project Bootstrap/adoption
    existing foreign-skill `aikit adopt`
    Project entry
    map/source/provenance commands

whether/how existing command names are aliased/deprecated

permanent product relationship among:
    current AIKit palette
    richer TUI
    browser Project surface
    cmux view
    Run/Project UI

exact Project-entry presentation

exact Bootstrap review presentation

exact available/retrieved/loaded visualization
```

Owner:

```text
M/Z
```

---

# OPEN SOCKETS

```text
shared Ref serialization

ProjectRef persistence API

Action descriptor/catalog API from E

Agent/Agency descriptor API from I

ExecutionRef / ModelRef / HarnessRef contracts from I

Run/Evolution query API from A/C

Recognition/Recursion event/export API from A/C

Workcell discovery/offer API from F

ExecutionDemand exact schema from A/F

MaterialisedExecutionWorldRef schema from F

QL service/resource API from H

QLFormRef / QLAddress / LensRef exact serialization

J UsageSignal / FitnessObservation / retrieval telemetry

Project Canon query/reference API

remote/multi-host AIKit resource sync

cross-host ContextResolution identity

Project Map cache storage strategy

source access/security policy

exact `.factory/` root namespace

exact Factory↔AIKit source-mount Git topology
```

---

# RESEARCH CLAIMS

```text
SSSF exact reusable source modules/current compatibility

Pi current RPC/SDK/extension seams

GitNexus current CLI/MCP command set and version behavior

current Matt Pocock skills structure/revisions

HumanLayer/Dexter exact reusable source material

Hermes current profiles/tasks/messaging seams

bkmr current semantic/hybrid API/version behavior

cmux current upstream automation interface

Bimba/Neo4j concrete provider contract

Agent-Native precedent/framework current APIs

Arrakis/MicroVM exact provider seam

deeper QL service operators

richer Agency identity forms

full operational roles of all twelve MEF lenses
```

These can inform planning.

They do not become verified implementation by appearing in the architecture corpus.

---

# SOURCE-INSPECTION BLOCKERS

The following dependent implementations are provisionally blocked:

```text
D.03 GitNexus production provider
    until GitNexus SourceIntegration VERIFIED

D.05 bkmr production provider
    until bkmr SourceIntegration VERIFIED

Pi-specific harness/resource projections
    until Pi SourceIntegration VERIFIED

Hermes-specific product projection
    until Hermes SourceIntegration VERIFIED

new cmux-upstream-specific behavior
    until current upstream seam pinned/verified

Bimba production provider
    until concrete connector/source integration verified

Docker/Arrakis implementation details
    owned by F and blocked at their specific upstream seam where applicable
```

Not blocked by those facts:

```text
D.01 generic Project Map architecture
D.02 Git/source provider
D.04 Canon/wiki provider
B ProjectBinding work
generic typed resource indexing
K evidence-recovery architecture
L source schemas/gates
generic Project/Context fixtures
```

---

# CROSS-MAP DEPENDENCIES

## A — Factory Core

Required:

```text
ProjectRef
Context relation
Claim/Evidence/Artifact interfaces
Decision/HumanRequest
Recognition/Recursion
Project Canon
```

Map 2 cannot be globally build-complete until these are stable.

---

## C — Run Map

Required:

```text
RunRef
RunMapRef
frontier/current-locus query
Evolution export
Bootstrap Run representation
ordinary first-Run handoff
```

---

## E — Agent-Native / Action

Required:

```text
ActionRef
ActionDescriptor
ActionCatalog
ActionSet semantics
surface/caller-lineage relations
legacy Action recovery contract
```

---

## F — Workcell

Required:

```text
ExecutionDemand
WorkcellRef
Workcell discovery/offers
MaterialisedExecutionWorldRef
reproducibility evidence
```

---

## G/H — Claims / QL / MEF

Required:

```text
QLFormRef
QLAddress
LensRef
refraction result contract
same-subject identity
QL service health
full MEF availability
```

---

## I — Agent / Agency / harness

Required:

```text
AgentRef
AgencyRef
ExecutionRef
ModelRef
HarnessRef
Agent resource descriptors
Epi-Logos canonical Agent identities
```

---

## J — learning / telemetry

Required:

```text
UsageSignal
FitnessObservation
retrieval observation
ContextLoad telemetry relation
preference/trust/availability distinction
```

---

## M/Z — product/root control

Required:

```text
exact CLI words
Project-entry UX
Bootstrap UX
persistent/rich UI topology
`.factory/` ratification
root cross-map naming
programme-level build order
```

---

# PROHIBITED REDEFINITIONS

Map 2 and its execution agents must not redefine:

```text
Project
as repository or AIKit binding

Context
as AIKit ContextDescriptor / Generation / prompt contents

Agent
as Model or Harness

Agency
as canonical Epi-Logos Agent identity

Action
as an AIKit-owned capability implementation

Candidate
as branch/container

RunMap
as Project Map or GitHub object

ExecutionDemand
as an AIKit provider plan

Workcell
as AIKit host state

QLAddress
as a new Project Map identifier

LensRef
as an alternative canonical object identity

SourceIntegration
as documentation link

Project Map
as source-of-truth database

Information Horizon
as bkmr database

Loaded
as global source property
```

---

# 34. Overlap classification and reconciliation

| Claim overlap | Classification | Resolution |
|---|---|---|
| Project vs AIKit current `ProjectSpec` | `DISTINCT HOMONYM / terminology collision` | canonical Project + AIKit ProjectBinding |
| Context vs AIKit `ContextDescriptor` | `COMPLEMENTARY VIEW` | descriptor is AIKit resolution input beneath Context |
| Context vs `ResolvedView` | `COMPLEMENTARY VIEW` | ResolvedView is capability facet |
| Context vs Generation | `COMPLEMENTARY VIEW` | Generation is materialised resolution |
| Project Map vs GitNexus | `COMPLEMENTARY VIEW` | GitNexus is one provider |
| Project Map vs bkmr | `COMPLEMENTARY VIEW` | bkmr is one information provider |
| Project Map vs Bimba | `COMPLEMENTARY VIEW` | optional semantic provider |
| Project Map as truth adjudicator | `COMPETING DESIGN` | rejected |
| Action vs Capability | `COMPLEMENTARY VIEW` | Action is domain operation in broader actor-power field |
| Action implementation in app vs AIKit Action copy | `COMPETING DESIGN` | app/project canonical; AIKit indexes |
| Agent vs model/harness | `DISTINCT CONCEPT` | identity survives execution composition |
| Epi-Logos names as Agents vs Factory stages | `SAME RELATIONAL HISTORY, DISTINCT CURRENT SEMANTICS` | profile Agent identities retained; stage mapping only historical/useful projection |
| six canonical Agents reduced to Agency metadata | `COMPETING DESIGN` | rejected |
| QL Forms vs Factory canonical object identity | `COMPLEMENTARY VIEW` | QL reads canonical Refs |
| MEF subset as “computational MEF” | `UNSUPPORTED INVENTION` | rejected; all twelve first-class |
| QL service optionality = QL unimportant | `COMPETING DESIGN` | service availability optional; architectural seam first-class |
| `.factory/` semantic separation | `SAME CONCEPT` | retained |
| `.factory/` exact name as invariant | `OPEN DECISION` | Z |
| `aikit adopt` rename | `OPEN DECISION` | M/Z |
| current transient palette as permanent UI constitution | `UNSUPPORTED INVENTION` | removed |
| shared application service for all surfaces | `SAME CONCEPT` | retained |
| Bootstrap as installer | `COMPETING DESIGN` | rejected |
| Bootstrap as Factory Run | `SAME CONCEPT` | retained |
| post-Bootstrap “Root Wayfinder” universally | `DISTINCT HOMONYM` | corrected to Bootstrap Run Map / ordinary Project Run |
| source documentation = integration evidence | `COMPETING DESIGN` | rejected |
| local imitation = named SourceIntegration | `COMPETING DESIGN` | rejected |
| source blocker freezes all Project World work | `UNSUPPORTED INVENTION` | rejected; block dependent nodes only |
| Recognition changes future Ground | `SAME CONCEPT` | retained and root-fixtured |

---

# 35. READY TO BUILD

The joined subsystem is **not globally READY TO BUILD** while shared A/E/F/I/H contracts remain unresolved.

The following independent foundations are ready to enter program design/implementation now:

```text
B.01
    AIKit implementation Ground / health inventory

B.02
    ProjectBinding compatibility analysis/migration design

D.01
    Project Map generic provider contracts

D.02
    Git/source provider

D.04
    Canon/wiki provider

L.01
    SourceIntegration declaration/lock schemas

L.02
    generic source-inspection framework

L.06
    dependency-local source-fidelity gate

K.02
    evidence-inspection framework over already available source providers

generic fixtures for:
    Project > repository
    Context availability/retrieval/loading
    derived-index rebuildability
    generic vs Epi profile resource expectations
```

Items dependent on shared identity grammar should use placeholders/import adapters, not invent competing contracts.

---

# 36. MUST RESOLVE FIRST

Before the **joined B/D/K/L subsystem as a whole** can be declared build-ready:

```text
1.
A:
exact shared Ref / ProjectRef / Project Canon interfaces

2.
C:
RunRef / RunMapRef / Bootstrap Run / Evolution query contracts

3.
E:
ActionRef / ActionDescriptor / ActionCatalog contracts

4.
F:
ExecutionDemand
Workcell discovery/offers
MaterialisedExecutionWorldRef

5.
I:
AgentRef / AgencyRef
ExecutionRef / ModelRef / HarnessRef
Epi-Logos canonical Agent resource descriptors

6.
H:
QLFormRef / QLAddress / LensRef
QL service/refraction boundary

7.
J:
ContextLoad/usage/fitness telemetry seam

8.
Z:
ratification of exact `.factory/` filesystem namespace

9.
M/Z:
product command names and permanent surface topology

10.
L:
verification of every named upstream before its dependent implementation
```

These are interface reconciliation tasks.

They are not invitations to reopen the already-settled Project World boundaries.

---

# 37. OPEN SOCKETS

```text
A Project persistence / Ref resolution
A Project Canon read/write/reference API

C Run/Evolution provider

E Action Catalog provider

F ExecutionDemand / Workcell offer provider

H QL/MEF provider

I Agent/Agency resource provider
I model/harness provider

J usage/fitness/retrieval telemetry

Project Map distributed cache

cross-host AIKit ContextResolution

remote Project resource synchronization

source-access/security policy

Project source-provider plugin discovery

Bimba/Neo4j concrete adapter

external documents/web source provider

Project “last meaningful entry” observation

Project-entry recommendation policy

SourceIntegration licence policy

root namespace/version migration
```

---

# 38. RESEARCH CLAIMS

The following remain active Claims rather than implementation requirements by formatting:

```text
exact reusable SSSF runtime modules

exact current Pi integration seam

exact current GitNexus CLI/MCP contract

exact latest Matt Pocock capability corpus

exact current HumanLayer/Dexter source seams

Hermes as preferred personal front door

cmux as long-term preferred Project surface

bkmr 7.x semantic/hybrid behavior

specific Bimba/Neo4j connection architecture

Arrakis as preferred MicroVM provider

specific full twelve-lens technological roles

deeper QL-native agent loop semantics

cross-application QL interoperability

richer sixfold Agency identity forms
```

They may guide experiments and SourceIntegration work.

They must not silently harden into code contracts before evidence.

---

# 39. INTEGRATION TESTS REQUIRED

The whole Project World requires the following integration proof set.

```text
01.
external repository
→ Bootstrap Run
→ evidence recovery
→ minimal human frontier
→ Project
→ Project Map
→ AIKit resolution
→ first ordinary Run

02.
fresh Project
→ authored Intent/Design
→ Agent-Native Action surface
→ Project World
→ first Candidate

03.
long absence
→ same ProjectRef
→ Evolution delta
→ current Ground/frontier
→ no archaeology requirement

04.
real SourceIntegration
→ pin
→ source inspection
→ actual seam
→ smoke/integration test
→ Evidence
→ drift handling

05.
sparse repository
→ implementation facts recovered autonomously
→ only genuine authorial questions reach human

06.
source changes
→ GitNexus/bkmr/other indexes stale
→ staleness visible
→ canonical source remains intact
→ indexes rebuild

07.
Context
→ Available set
→ Retrieved subset
→ ContextLoad subset
→ ExecutionRef/HarnessRef explicit

08.
Epi-Logos Project/Profile
→ 0/1 orchestrator
→ Anuttara
→ Paramasiva
→ Parāśakti
→ Mahāmāyā
→ Nara
→ Epii

generic Project
→ no implicit Epi-Logos identities

09.
same AgentRef
→ ModelRef A
→ ModelRef B
→ HarnessRef X
→ HarnessRef Y
Agent identity unchanged

10.
Bimba unavailable
QL service unavailable
→ ordinary Project/Bootstrap/Map/AIKit/Action operation survives

11.
QL service available
→ all twelve LensRefs addressable
→ refraction result keeps original subject Ref

12.
Action Catalog
→ AIKit index
→ Project Map navigation
→ invocation reaches original Project/Application operation
→ no AIKit business-logic copy

13.
ExecutionDemand
→ Workcell offer
→ MaterialisedExecutionWorld
→ Project meaning unchanged across provider substitution

14.
Recognition/Recursion
→ Project Canon/Ground/history difference retained
→ all sessions stop
→ Project re-entered
→ retained difference visible

15.
delete:
    AIKit derived indexes
    GitNexus graph
    bkmr index/cache
rebuild
→ Project identity and authored Canon unchanged

16.
unverified GitNexus integration
→ D.03 blocked
→ D.01/D.02/D.04 remain buildable

17.
current AIKit
→ full own health gate green
before release-level Factory integration is claimed

18.
CLI / TUI / headless / future UI
→ same underlying application/service operation
→ same Refs
→ same resolution semantics
```

---

# 40. Final joined statement

Project World exists so a Project does not have to be reconstructed from a directory every time an intelligence enters it.

```text
Project
    preserves authored identity and recognised orientation

Project Map
    preserves navigability across distinct sources of intelligence

Information Horizon
    keeps more knowledge addressable than any one execution should load

AIKit
    resolves the identities, powers, resources and sources
    available to the current actor

Agent / Agency
    inhabit that world without being reduced to model/harness state

QL / MEF
    remains a first-class formal and refractive field
    over the same canonical objects

Bootstrap
    recovers or authors the Project world once
    rather than making the human reconstruct it repeatedly

SourceIntegration
    keeps every borrowed technological power connected
    to the actual upstream system it claims to use

Workcell
    turns semantic execution demand into material computation
    without colonising Project meaning

Recognition / Recursion
    makes durable difference visible on the next entry
```

The negative architecture is equally important:

```text
repository is not Project

ProjectBinding is not Project

AIKit is not Project

ContextResolution is not Context

Generation is not Context

Project Map is not source truth

GitNexus is not source

bkmr is not the Information Horizon itself

Bimba is not a generic Project dependency

Agent is not Model

Agency is not a demotion of canonical Agent identity

Action descriptor is not Action implementation

Workcell binding is not semantic identity

Available is not Loaded

MEF reading is not a second object

QL service outage is not Project failure

documentation is not source verification

local imitation is not upstream integration

Bootstrap is not a questionnaire

Bootstrap Run Map is not automatically a Root Wayfinder Map
```

The resulting architecture allows generic software Projects and the fuller Epi-Logos Project to inhabit the same Factory substrate without either flattening the other.

A generic Project remains lightweight and operational.

An Epi-Logos Project can additionally resolve:

```text
0/1 orchestrator
six canonical Agent identities
richer Agencies
QL forms/addresses
the complete MEF manifold
project-specific ontology
specialised capabilities
optional Bimba semantic horizon
```

through the same Project, Context, AIKit, Project Map, Run, Action, Workcell and SourceIntegration relations.

That is the Project World this map commits the programme to build.