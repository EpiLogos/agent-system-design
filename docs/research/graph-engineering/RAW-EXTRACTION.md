# QL Software Factory — Graph Engineering Principles
## Nodes · Edges · Verification · Skill Stipulation

**Status:** cross-cutting principles companion — extracted 2026-08-12, generalised, and re-expressed in Factory terms.
**Source:** AI LABS — *Anthropic Just Fixed Graph Engineering's Greatest Flaw* (2026-07-29, https://youtu.be/H7t3uUp3HVw).
**Raw transcript:** `transcripts/04-graph-engineering.txt` (full, timestamped).
**Authority:** subordinate to `QL-SOFTWARE-FACTORY-CONSTITUTIONAL-INDEX.md` and to the numbered Wayfinder Maps. This document is **not a numbered map and claims no territory**. It fixes general graph-construction and skill-stipulation principles that the Run Map design (A · C · G · J), the Agentic Execution Body (E · F · H · I) and Project World (B · D · K · L) import where they concern parallel execution topology, verification gates and capability stipulation.

> **Joined question:** When work is fanned out across parallel nodes, how does the Factory keep the whole graph truthful — so that no node builds on a mistake, every node's output is attributable, the human can review a fleet rather than drowning in it, and verification is stipulated rather than hoped for?

---

# 0. Source, stance, extraction rule

## 0.1 What the source is

A survey of **graph engineering** by a software consultancy channel: the transition from *loop engineering* (one agent grinding a linear pipeline) to *graphs* (a task split into parallel nodes, each an agent with its own isolated context window, tied together by edges that move data between them). Its load-bearing claims:

1. Graphs buy speed (parallel legs) and per-node cost control (a different model per node), at the price of **much higher total token burn**.
2. Every graph shape rests on **verification**; without stipulated checks every downstream node builds on a mistake.
3. Verification quality is set by the **model on the judging node** — economising there is where the whole graph degrades, silently and untraceably.
4. The **builder is the worst reviewer of its own work**; a fresh context window is required for unbiased judgement.
5. Skills come in **three invocation kinds** (standalone / embedded / orchestrator), one **angle per skill**, chained by composition — and an orchestrator skill is the single entry point a graph prompt needs.
6. Anthropic's own practice: chain `code review` + `simplify` + `verify` + a design-check skill — **four directions**, not one.

## 0.2 Extraction rule

The source is **product-ecosystem-specific**: Claude Code / Codex, the skill-creator plugin, Haiku/Opus model tiers, Chrome headless shell. This document keeps the **general principles** behind those affordances and re-expresses them in project-owned, provider-neutral, QL-rooted Factory terms. Where the source names a product, this document names the principle the product implements. Where the Factory differs deliberately, §6 says so.

**Provenance convention:** each principle cites the transcript timestamp it is extracted from, so any claim here can be re-audited against the raw material. The video is evidence about graph practice — it is not Factory canon.

## 0.3 Determination statuses used in this document

```text
PRINCIPLE            a general rule extracted from the source, stated provider-neutrally
FACTORY MAPPING      where the principle lands in the Factory (imports into the numbered maps)
DELIBERATE DIVERGENCE where the Factory chooses otherwise than the source suggests (§6)
```

---

# 1. From loops to graphs — the general principle

## 1.1 What a loop is, and its limit

A loop hands an agent a goal; the agent works toward it, a verification step kicks in, and on pass the next step starts. Everything runs **in a straight line** — every step waits on the step before it, *even when the two have nothing to do with each other* (0:29–1:49).

**PRINCIPLE L1 — Shape follows dependency structure.** Serial waiting is only correct where the dependency is real. Where two pieces of work do not depend on each other, forcing them into one linear context is wasted time and wasted context — the pipeline imposes false seriality.

**FACTORY MAPPING:** the Factory's phase pipeline (plan → build → test → review → document → commit) is a **loop by default**. It is the correct default for a single coherent change. It is wrong when a task decomposes into genuinely independent work units — that is the signal to promote the loop into a graph (below).

## 1.2 What a graph is

A graph splits the main task into smaller parts; **each part gets its own agent with its own isolated context window**, and reports back. Edges control how data moves from one node to the next — "one agent's output lands with the right agent at the right point" (1:36–3:18).

**PRINCIPLE G1 — A node is one job, one context, one report.** Isolation of context is the property that makes everything else possible: parallel legs, per-node model assignment, and — critically — later review by a context that had no hand in the doing (§3.4).

**PRINCIPLE G2 — An edge is a typed handoff, not a pipe.** The edge carries the output *and* the gate state between nodes: what was produced, against what requirement, whether it passed. "One agent's output lands with the right agent at the right point" — the right point is determined by the gate, not by luck.

**PRINCIPLE G3 — Every node is connected.** A node not tied into the graph is orphan work: it can neither be reviewed nor traced. (0:57–3:18: the source's fan-out example — several agents review the same work, none waits on another, all reports feed one place at the end.)

**INVARIANT — One canonical topology per Run.** Exactly one canonical Run Map belongs to each Run (Map 1, §0.3). A graph in the Factory *is* the Run Map: nodes are positioned executions, edges are typed envelopes with gate semantics. There is no second graph floating beside the Run Map.

## 1.3 Why graphs exist

- **Speed:** several agents cover the work at once instead of one grinding through it (1:44–1:59).
- **Per-node cost:** you get to pick which model each node runs on — "you stop burning your most expensive model on the parts that never needed that much intelligence" (1:52–2:02).
- **Ground covered:** more of the task's surface is worked in parallel.

**PRINCIPLE G4 — Fan-out is earned by independence, not chosen for its own sake.** A graph's speed and coverage are real only when the work units are actually independent. Fanning out dependent work produces coordination overhead and merged-context loss — the opposite of the gain.

---

# 2. Shapes

## 2.1 The diamond (fan-out → fan-in)

One task at the top splits into several subagents running side by side; they narrow back into a single agent that pulls everything found into one answer (3:22–3:43). The source notes it previously mislabelled this shape a "loop" — it was actually a graph being looped.

**PRINCIPLE S1 — Synthesis nodes exist to compress.** The fan-in node exists so N reports become one reviewable surface. A diamond without a real synthesis node drowns the next consumer in unmerged output (see also §4.3).

## 2.2 Fan-in at a barrier

The shape for "one thing has to be judged from several angles at once": the same problem fans out to a set of agents, each looking **through a different lens**; **nothing moves forward until every one has reported back**; only then do fixes run (3:44–4:01).

**PRINCIPLE S2 — A barrier waits on all legs by construction.** Multi-angle judgement is only trustworthy if the gate is structurally unable to open on a partial report. "Nothing moves forward until every one of those agents has reported back" is a gate invariant, not an aspiration.

**FACTORY MAPPING:** barrier = the Factory's fan-in gate; all legs must have produced Evidence-bearing Claims (not "done" assertions) before the gate opens. This is the multi-angle counterpart of the single-angle gate in the phase pipeline — both are boundary conditions, not vibes (README QL-alignment hooks).

## 2.3 The general rule

**PRINCIPLE S3 — Shape is determined by the dependency relation of the work, and every shape rests on verification.**

```text
work units independent of each other        → fan-out (diamond when they merge into one synthesis)
one thing judged from several angles        → fan-in at a barrier (all legs must report)
genuinely sequential dependency             → edge chain (the loop's correct residue)
anything                                    → verification first (§3)
```

**FACTORY MAPPING (CURRENT DESIGN suggestion for `core/runmap` graph invariants):** the Run Map's graph invariants should encode at minimum — one source node; no orphan nodes (G3); barrier nodes declare their leg set and open only when all legs report Evidence (S2); synthesis nodes declare their report contract (S1). These belong to the Run Map design in Map 1's territory, not here; this document only fixes that they must exist.

---

# 3. Verification is the load-bearing layer

The source's central claim: "every one of these shapes rests on the same thing, and that's verification. If you don't set those checks up properly, every agent that comes after is just building on top of a mistake." (4:02–4:12)

## 3.1 Built-in verification is necessary, never sufficient

All agents verify whatever they write, whether asked or not — for code, they run the tests and catch errors that come back. **But that only catches major errors; it never checks how the code is written** (4:41–4:56).

**PRINCIPLE V1 — Run-verification proves the artifact runs; it does not prove the artifact is right.** "Does it execute?" and "does it satisfy the original requirement, written the way we stipulated?" are different questions. The first comes free with any agent; the second must be stipulated.

**FACTORY MAPPING:** the free check corresponds to run-level Evidence (tests pass). The stipulated check corresponds to standards-level Assessment against the governing Intent/Design Claims — and only an Assessment can carry the `intended · observed · verified` distinction the Factory treats as constitutional (Map 1, §0.3).

## 3.2 Stipulated verification is the only verification that scales into a graph

Why this matters *more* in a graph: **each agent only ever sees its own piece**. The check that matters is the one that lets a node "check that piece against the original requirements" even though it never sees the whole (5:35–6:30).

**PRINCIPLE V2 — A node's verification must reference the governing requirement, not the node's own memory of it.** In a graph, no node holds the whole task; the requirement it checks against must be carried to it (via the Run / governing Claims), so its check is against the original stipulation rather than against whatever it happened to build.

**FACTORY MAPPING:** this is the epistemics loop verbatim — a node asserts Claims against the governing Intent; Evidence attaches to Claims; an Assessment records standing under criteria; Recognition is a specialised Decision determination (Map 1, §0.3). The graph does not need a new mechanism; it needs the epistemics to be *stipulated per node*.

## 3.3 The judging node is where you never save tokens

The source's field story: a cheap model reviewing a UI returned a long list of issues — by finding count alone it looked excellent. The frontier model flagged fewer things, "right up until we read the reasoning": most of the cheap findings were false positives — things deliberately left that way, which the cheap reviewer couldn't infer from surrounding code. The cheap review hadn't saved anything, because **the review itself now needed reviewing** (6:29–7:07).

Inside a graph this compounds: a whole set of nodes check their own work with the same cheap-judge skill, agents burn time and tokens "fixing things that were never broken" — and because it happens across separate agents all at once, "you'd have no way of telling which one started it" (7:09–7:35).

**PRINCIPLE V3 — Model choice on the judging node decides the quality of the whole graph, not just the review.** The source: "The node that does the judging is the one place where saving tokens costs you everything."

**PRINCIPLE V4 — False-positive verification is a systemic cost, not a local one.** Every node that acts on a false finding spends time and tokens, and the cost is paid downstream. Cheap-judge savings are recovered at the expense of the entire fleet.

**PRINCIPLE V5 — Verification output must be attributable.** Findings carry which node/angle produced them, so a downstream fix cascade can be traced to its origin. Untraceable review noise is a graph-poisoning event (see also §4.3).

**FACTORY MAPPING (DELIBERATE DIVERGENCE surface):** the Factory already encodes V3–V5 in structure: model-stack configs (frontier / open-weights / top-speed / deepestseek) assign tiers per node type; Assessments carry author and standing; Trace is rebuilt from typed Events. The principles here *fix the policy*: **a judging/Assessment node is always assigned the strongest available model tier, and its findings are Claims/Evidence-bearing with provenance** — never a bare "issues list".

## 3.4 The builder is the worst reviewer of its own work

The source's most-used verification: *second opinion*. The agent that built the thing judges its own work off the same context it used to build it; a fresh session — one that has not seen any of that context — gives an unbiased review. The built-in advisor fails at this because it reads the current chat and **inherits the same context**. The second opinion starts a wholly separate session, takes a long time, and "the model matters here more than anywhere else, because the whole point is a smarter second read" (11:12–12:16).

**PRINCIPLE V6 — Review and build never share a context window.** A reviewer that inherits the builder's context inherits the builder's blind spots. This holds whether the reviewer is another session, another Agency, or another Agent entirely.

**PRINCIPLE V7 — The independent judge runs the strongest available model.** A second read is only worth doing if it is a *smarter* read. The point of independence is not procedure — it is a different, stronger judgement applied to the same artifact.

**PRINCIPLE V8 — Every node gets its work checked by something that had no hand in doing it.** The source's closing statement of the pattern (12:12–12:16): this is what gives every node in a graph a review path at all.

**FACTORY MAPPING:** the Factory's fresh-context audit discipline (never same-agent, distinct session) is this principle in practice; the source confirms it as the general law, not a local habit. In Agency terms: build and assess are distinct Agency roles enacted over the same Execution lineage; the assessor's Context does not include the builder's SessionSpace.

---

# 4. Graph economics, limits, and observability

## 4.1 Total burn beats per-node savings

**PRINCIPLE E1 — A graph is more expensive than a loop, in total, by construction.** Per-node cost falls (right model per node) while total token burn rises sharply — many contexts running at once instead of one (2:02–2:15). The source's operational rule: "If you are using graphs, expect your limits to hit way sooner than you're used to."

**PRINCIPLE E2 — Fan-out degree is budgeted against the total ceiling, not per-node price.** The binding constraint on a graph's width is the aggregate quota/token ceiling of the whole run. Budgeting must be done on the sum.

**FACTORY MAPPING (CURRENT DESIGN hook):** model-stack configs and quota-capped provisioning already exist in the Factory material; E1/E2 fix that **fan-out width is a Run-level budget decision** (how many parallel legs, at which tiers, under which cap) recorded on the Run Map — not a per-node ad-hoc choice.

## 4.2 Per-node model assignment

**PRINCIPLE E3 — Model per node follows the node's capability requirement; judging nodes are the single exception.** Cheap nodes run cheap tiers; execution nodes run what the function needs; **judging nodes always run the strongest tier (V3, V7)**. The exception is absolute — it is the one place where economising degrades the whole graph.

## 4.3 The review flood and the invisibility problem

With a fleet, two new failure modes appear (4:21–4:40):

1. **Amount of work:** everything comes back at once — a huge pile that is genuinely hard to review at the end.
2. **Invisibility:** "you can't see what happened. When something goes wrong, you've got no way of telling what caused it."

**PRINCIPLE E4 — Fan-in synthesis is the answer to the flood, and it must be a real node.** The diamond's synthesis (§2.1) exists precisely to compress N reports into one reviewable surface. A graph whose fan-in is nominal (reports merely concatenated) has not solved the flood; it has moved it.

**PRINCIPLE E5 — "Can't see what happened" is a design failure, not a fact of life.** A fleet is only untraceable if the system does not require per-node typing of events and per-edge recording of handoffs. The fix is structural: typed Event emission per node, Trace reconstruction per edge, and an outbox that survives telemetry loss.

**FACTORY MAPPING:** the Factory's observability envelope / telemetry / trace layers (Map 1, §2.1 `observability/`) are the structural answer to E5; this document fixes that **graph nodes and edges are constitutive Event sources** — the video's pain point is the Factory's Event/Trace requirement, not a nice-to-have.

---

# 5. Rules for building and stipulating skills

This is the section that governs **how skills are built and stipulated in execution**. In Factory terms a *skill* is a project-native Capability: a stipulated, tested, invocable procedure with five stipulated fields — **trigger, scope, measure, model, output contract**. A skill stipulation is incomplete if any of the five is missing.

## 5.1 The three invocation kinds (generalised from the source, 8:34–11:12)

| Kind | Fires | Depth | Factory analogue |
|---|---|---|---|
| **Standalone** | only when someone explicitly runs it | deep pass over something that *already exists* (finished output) | commissioning-time tool: deep reviews run by human or by request; never on unfinished work (token burn) |
| **Embedded** | as part of the workflow, unasked | rule-check on work in progress; blocks completion until checked | workflow-bound gate between phases: cannot finish until the check against the skill's rules passes |
| **Orchestrator** | when the graph prompt names it — the only thing the prompt names | runs other skills; one agent per skill, each in its own context window, in parallel; collects all findings into one report the fixing agents work from | composition Action/Agency: the single entry point; fan-out happens underneath it, never hand-rolled per node |

**PRINCIPLE K1 — A skill's kind is part of its stipulation.** "How and when this gets invoked" (the source's own framing) is a field of the skill, not a deployment accident. Ambiguity here is where verification either never fires or fires on unfinished work.

**PRINCIPLE K2 — Standalone skills are commissioned, not scheduled.** A deep pass on finished work must not auto-fire after every run — it would burn tokens reviewing work that is not finished yet (8:36–8:51).

**PRINCIPLE K3 — Embedded skills gate completion.** An embedded skill's contract is: implementation cannot finish until checked against the skill's rules (9:30–9:47). The gate is the point of the skill; a "suggestion" embedded skill is a standalone skill wearing the wrong label.

**PRINCIPLE K4 — One entry point per graph: the orchestrator skill.** The graph prompt says only "use that one skill"; every node loads it and the whole review fans out underneath it on its own (13:26–13:35). Composition is how multi-angle review is wired — never by listing N skills in the prompt and hoping the agent sequences them.

## 5.2 The stipulation contract — five fields

**R1 — Trigger (how and when it invokes).** One of: *standalone* (explicit commissioning only), *embedded* (workflow-bound gate, fires automatically at the stipulated phase), *orchestrator* (composition entry, named by the graph prompt). No other modes.

**R2 — Scope (one angle, named standard).** A skill reviews **one angle** with **its own way of measuring** — and checks against a **named standard document** (the design.md analogue: the file that holds every design decision for the product, 12:39–12:57), not a vibe. "Comprehensive" must be explicitly stipulated when a deep pass is wanted — a deep pass and a quick pass are different scopes (9:16–9:24).

**R3 — Measure (evidence shape).** What counts as pass/fail, and what the skill returns: a findings report in a form the fixing agents can consume (findings, angle, provenance, severity). A skill without a defined output contract produces prose the next node cannot act on.

**R4 — Model (minimum tier for judging).** A judging/verification skill declares a minimum model tier; the declaration is **enforced at the node**, not hoped for (§3.3). "Skills are only ever as good as the model you run them on" (6:29–6:32) is the standing invariant of this field.

**R5 — Tested creation.** When a verification skill is created, its references and scripts are **structured and tested as part of creation** (the source's skill-creator point, generalised: what comes back from a generated, tested skill is easier to trust than an ad-hoc prompt, 9:08–9:15). Ad-hoc prompts produce untested judgement; an untested judge is a V3/V4 liability.

## 5.3 Composition rules

**PRINCIPLE K5 — One angle per skill; chain for many angles.** "You can't stuff all review types into one skill, because that way the agent will have too many directions to review and will end up getting worse instead of better" (12:18–12:35). Multi-angle review is achieved by chaining skills, each covering one angle: the source's example — code review + simplify + verify + design-check = four directions (12:37–12:59).

**PRINCIPLE K6 — Verification is as heavy as needed, as light as possible.** Use the lightest sufficient verification machinery for the workflow's repeated passes (the source's headless-shell-over-full-browser point, 10:39–11:06). Verification itself is costed and runs inside the workflow, so its weight is a design decision, not an accident.

**FACTORY MAPPING (CURRENT DESIGN hook for `integrations/actions` + capability stipulation):** a skill stipulation (R1–R5) should surface as an `ActionManifest` extension — effects class, approval policy, exposure — plus the five stipulation fields. Embedded skills bind to phase gates in the pipeline; standalone skills expose as commissioning-time Actions; orchestrator skills expose as composition Actions. All five fields belong in the manifest so the stipulation is inspectable, not conversational.

---

# 6. Where our approach differs from the video (deliberate divergences)

The source is good practice evidence, not a template. These are the points where the Factory deliberately chooses otherwise:

| # | The video | The Factory | Why |
|---|---|---|---|
| D1 | Graph engineering lives inside Claude Code / Codex; skills are product plugins; verification is product-built | Project-owned, provider-neutral, QL-rooted: skills are project-native Capabilities; the Run Map is the canonical topology; GitHub/UI/Hermes are projections | No projection, index, or product may become an undeclared second source of truth (Map 1, §0.2) |
| D2 | The graph is the shipped artifact; failure tracking is solved by a vendor fix | One canonical Run Map per Run with graph invariants; typed Events + Trace + outbox as constitutive observability | "Can't see what happened" is a design failure (E5); observability is constitutional, not a patch |
| D3 | Verification is "something you set up yourself" (tooling) | Verification is epistemics: Claims / Evidence / Assessment / Recognition | `intended · observed · verified` are distinct epistemic relations; a tool call cannot manufacture "verified" (Map 1, §0.3) |
| D4 | Three skill kinds are usage modes | Three kinds are stipulation contracts (R1–R5) with enforceability | Stipulation must be inspectable and enforceable at the node, not hoped for |
| D5 | Judge quality by model tier (Haiku vs Opus) as a private cost call | Judging tier is a Run-level policy on the Run Map: judging nodes always take the strongest available tier | V3/V4 — false positives poison the whole graph; the policy must be structural |
| D6 | "Expect limits to hit sooner" — the graph is expensive, cope | Fan-out width is budgeted at Run level (E2); model-stack configs + quota caps are the answer | The graph's width is a designed budget, not a surprise bill |
| D7 | The pipeline's end is "fixed" (tests pass, review done) | Consequential results pass through Recognition; retained differences fold into future Context/Ground via RecursionArtifact | The Factory's loop closes one level up: the graph produces Candidates, and Recognition decides (Map 1, §0.3, AG5) |

---

# 7. How this dovetails with the Wayfinder Maps

The maps import these principles at their seams:

```text
Map 1 (A·C·G·J — semantic core)
    §2.1 runmap/graph invariants     ← G1–G3, S1–S3 (nodes, edges, shapes)
    epistemics (Claim/Evidence/Assessment/Recognition) ← V1–V5 (verification as epistemics)
    observability envelope/trace     ← E5 (attributability, traceability)

Agentic Execution Body (E·F·H·I)
    Agent/Agency/AgentSession/Execution ← G1 (context isolation), V6–V8 (fresh-context judge)
    Actions/Capability field         ← R1–R5 (skill stipulation contract)
    model-stack / ExecutionDemand    ← E1–E3, V3, V7 (per-node tiers; judging tier policy)

Project World (B·D·K·L)
    skill/Capability adoption        ← K1–K6, D1 (project-native, provider-neutral)
```

**Constitutional invariants preserved by this document:**

- A Run Map is the graph; there is exactly one per Run. No second graph exists beside it.
- Verification is epistemic (Claim/Evidence/Assessment), never a bare tool call.
- Judging nodes never economise on model tier; their output is attributable Evidence.
- Build and review never share a context window; the reviewer had no hand in the doing.
- Skills are stipulated contracts (trigger · scope · measure · model · output), one angle each, composed by orchestrator skills — never stuffed, never hoped for.
- Fan-out width is a Run-level budget under the total quota ceiling, recorded on the Run Map.
- Graphs remain QL-consonant: the source is evidence about graph practice; it does not define Factory semantics, and no principle here requires executable QL service availability.
