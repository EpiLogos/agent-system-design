# Agent System Design — Software Factory Reference

Reference material for building a **software factory** setup: transcripts, linked resources,
working codebases, and distilled design principles from four source videos (2026-07/08).
Collected 2026-08-11, graph-engineering video added 2026-08-12.

**Intended use:** design a QL-aligned software factory — deterministic code wrapping
non-deterministic agents, phased AI developer workflows with gate checks, out-of-the-loop
operation, model-stack (not single-model) execution.

---

## The three videos

| # | Video | Channel / guest | Date | What it gives you |
|---|-------|-----------------|------|-------------------|
| 1 | [Ex-NASA dev reveals his Agentic Engineering Workflow](https://youtu.be/xgkjtF89-44) | David Ondrej × Dexter Horthy (HumanLayer, coined "context engineering") | 2026-08-07 | Why benchmarks don't matter, program design before the agent cooks, the 4-month unattended software factory run, why pull requests are dead |
| 2 | [Engineers… Your Software Factory NEEDS Agent Sandboxes to SCALE](https://youtu.be/SEI_qIW4o2c) | IndyDevDan | 2026-08-10 | Factory In A Box: three-tier architecture (out-loop orchestrator → in-sandbox orchestrator → Pi ADW agents), disposable provisioning keys, best-of-N fan-out, blast radius = the box |
| 3 | [My Super Simple Software Factory](https://youtu.be/haUfb1ievTE) | IndyDevDan | 2026-08-03 | The SSSF skill: Observable / Customizable / Reusable, the core four (context, model, prompt, tool), phase gates, typed envelopes, in-distribution (Python + YAML + skills only) |
| 4 | [Anthropic Just Fixed Graph Engineering's Greatest Flaw](https://youtu.be/H7t3uUp3HVw) | AI LABS | 2026-07-29 | Graph engineering: node/edge anatomy, shapes (diamond, fan-in at a barrier), verification as the load-bearing layer, three skill kinds (standalone / embedded / orchestrator), fresh-context second opinion, model-per-node economics — distilled in `seed-docs/QL-SOFTWARE-FACTORY-GRAPH-ENGINEERING-PRINCIPLES.md` |

**Cross-references:** both IndyDevDan videos point at the "Forget Loop Engineering"
video ([VQy50fuxI34](https://youtu.be/VQy50fuxI34)) — the predecessor concept to the factory.
The AI LABS graph-engineering video (04) sits at the far end of the same lineage: it documents
the loop → graph transition the factory's phase pipeline must be able to promote into.

---

## Directory layout

```
Agent System Design/
├── README.md                                  ← this file
├── transcripts/
│   ├── 01-david-ondrej-dexter-horthy-agentic-workflow.txt   (full, timestamped, ~85K)
│   ├── 02-inkwell-agent-sandboxes-exe-dev.txt               (full, timestamped, ~51K)
│   ├── 03-super-simple-software-factory.txt                 (full, timestamped, ~42K)
│   └── 04-graph-engineering.txt                             (full, timestamped, ~19K)
├── sites/                                     ← fetched content of description link sites
│   ├── dex-factory-skill.md                   (email-gated landing page — see note)
│   ├── exe-dev.md                             (sandbox/VPS/devbox platform, Shelley web agent)
│   ├── openrouter-provisioning.md             (provisioning keys API — spend caps, revoke)
│   └── humanlayer.md                          (Dexter's company — human-in-the-loop ops)
├── seed-docs/                                 ← design docs + wayfinder maps
│   ├── wayfinder-maps/                        (Map 1 semantic core A·C·G·J, Agentic Execution
│   │                                           Body E·F·H·I, Map 2 Project World B·D·K·L)
│   └── QL-SOFTWARE-FACTORY-GRAPH-ENGINEERING-PRINCIPLES.md  ← distilled from video 04
├── super-simple-software-factory/             ← cloned repo (disler) — the SSSF skill
└── inkwell-agent-sandboxes-and-software-factory/ ← cloned repo (disler) — Factory In A Box
```

---

## The codebases

### `super-simple-software-factory` (~1.6M)
A Claude skill (`.claude/skills/sssf/`) that drops an entire factory into any codebase
with one `/install`. Core assets:
- **`SKILL.md`** — the factory playbook (observable / customizable / reusable)
- **`references/`** — config.md (the core four: context, model, prompt, tool), observability.md, handoff.md (typed envelopes)
- **`templates/adws/`** — the ADW (agent developer workflow) roster: `adw_scout`, `adw_plan_build`, `adw_build_test`, `adw_plan_build_test_quality`, `adw_quality`… each with prompt/build gates and shared `adw_modules/` (runner, gates, session, tracer, permissions, git_helper)
- **`scripts/`** — `make_config.py`, `make_adw.py`, `install.py`

### `inkwell-agent-sandboxes-and-software-factory` (~52M, Factory In A Box)
The factory + a blog app (Inkwell) + the sandbox mount system. Three nested tiers:
1. **Out-loop orchestrator** on your machine — `just sbx` command surface, mounts a throwaway exe.dev VM
2. **In-sandbox orchestrator** on each VM — drives the factory
3. **Pi coding agents** running plan → build → test → review → document inside

Notable assets:
- `adws/adw_sssf_config/` — **five model-stack configs**: `frontier`, `open-weights`, `top-speed`, `deepestseek` (all-DeepSeek), base — best-of-N against one prompt
- `adws/adw_document.py`, `adw_build_review.py` — extended roster beyond SSSF
- `.claude/commands/install` + `/prime` — agentic bootstrap; `just sbx manage doctor` preflight
- Disposable OpenRouter provisioning keys with hard spend caps, revoked at teardown

---

## Notes / honest gaps

- **`sites/dex-factory-skill.md`** — the page at davidondrej.com/dex-podcast is an **email-capture gate**; the actual Dexter Software Factory skill content is only delivered by subscribing. Page text is what we got: "FREE — What you get: Website" + newsletter consent form. Do not expect the skill from this fetch; the repo clones above are the substantive code.
- **PostHog / Skool / TAC links** in descriptions are sponsor/course pages — excluded.
- HumanLayer page fetched for completeness (Dexter's company; agent ops with human sign-off).

---

## QL alignment hooks (for the design pass)

The factory's phase model maps onto QL structure without forcing:
- **plan / build / test / review / document / commit** = a phase pipeline with deterministic gate checks between phases — natural analogue to position-driven phase transitions (each gate = a boundary condition, not a vibe)
- **core four (context, model, prompt, tool)** = the configurable degrees of freedom per agent — candidates for coordinate-driven configuration
- **typed envelopes / shared session dirs** = the handoff contract between phases — analogue of the envelope structure in QL messaging
- **out-loop vs in-loop** = the observing system vs the operating system split
- **graph engineering principles** (`seed-docs/QL-SOFTWARE-FACTORY-GRAPH-ENGINEERING-PRINCIPLES.md`,
  distilled from video 04) = how the phase pipeline promotes into parallel Run Map topology:
  node/edge semantics, fan-out shapes, verification as the load-bearing layer, and the
  five-field skill stipulation contract (trigger · scope · measure · model · output)

Actual QL schema design (coordinate assignment, envelope typing, gate semantics) is the next step — this directory is the raw material for it.
