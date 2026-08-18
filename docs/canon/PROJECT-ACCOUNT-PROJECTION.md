# Software Factory — Project Account Projection

**Status:** integration note subordinate to the Factory Constitutional Index

## 1. Purpose

The Software Factory develops an enduring Project from authored intention through design, Runs, evidence, candidate reality, Recognition, and Return.

O:I can present selected readings of that Project without becoming the Project's canonical owner and without requiring the Factory to introduce an `Account` object.

The relation is:

```text
Project
    ├── authored intent / positions
    ├── vision / design
    ├── diagrams / architecture
    ├── implementation
    ├── Wiki / knowledge
    ├── Runs / RunMap
    ├── Claims / Evidence / Candidates
    └── current reality
         │
         │ provenance-aware authored reading
         ▼
    WorldPresentation
         │
         │ selective ratified O:I Projection
         ▼
    Explore / O:I desktop / standalone HTML / agent reading
```

The Project remains canonically owned by its native Project authority.

## 2. Why this relation exists

Fast agentic development can preserve the technical nouns of a request while losing the human reason those nouns mattered.

The Factory therefore keeps developmental continuity available across several different kinds of evidence:

```text
human-crafted intent / experience
        0
        ↓
positions / vision
        ↓
design
        ↓
experiential + conceptual diagrams
        ↓
architecture
        ↓
implementation
        ↓
tests / Runs / evidence / current reality
        1
        ↓
encounter / Recognition / Return
        ↓
renewed understanding, design, or development
```

A rich Project account makes this continuity inspectable. It must not collapse the distinct authorities that constitute it.

## 3. Account is a reading, not Factory canon

Do not introduce a universal Factory `Account` identity merely because rich presentations are useful.

A Project can support several legitimate readings:

- public overview;
- developer account;
- research account;
- design account;
- release account;
- evidence or Recognition account.

Each reading can select different material and audience boundaries while retaining provenance to the same Project and its native objects.

The reading is derivative. The Project, Run, Claim, Evidence, Candidate, Commission, Recognition, and other Factory semantics remain native objects or relations where the Factory defines them.

## 4. Current implementation boundary

Current Factory code already has concrete identity and Run seams which a Project reading can cite.

`factory/src/core/run/model.rs` currently defines a `Project` with a `ProjectRef` and revision. A `Run` retains its own `RunRef`, `ProjectRef`, revision, lifecycle, destination, write authority, and `RunMap`.

`factory/src/core/identity/state.rs` provides revisioned identity records and explicit mutation checks.

These implementation facts establish current identity/revision and Run topology mechanics. They do not by themselves define the whole human meaning of Project.

The constitutional and architecture corpus remains prior for that meaning.

## 5. Provenance standing

A Project account should preserve the standing of each module it presents.

At minimum distinguish:

```text
AUTHORED HUMAN POSITION
PRODUCT / PROJECT INTENT
DESIGN COMMITMENT
ARCHITECTURE CONTRACT
IMPLEMENTATION FACT
RUN / TEST / EVIDENCE RESULT
CURRENT DEVELOPMENT STATE
INFERENCE / INTERPRETATION
```

This distinction matters because the sources answer different questions.

Vision tells us what is meant. It does not prove current behaviour.

Current code tells us what is real now. It does not retroactively author why the Project exists.

Runs and evidence tell us what happened under specific conditions. They do not automatically settle the Project's intended direction.

Returned reality can create pressure to revise implementation, design, intent, or ground. The revision remains explicit.

## 6. Project Canon and ordinary Projects

The account capability must not become an adoption gate.

A lightly documented Project can still exist as ordinary source and receive a useful account. A mature Factory Project can expose much richer material through the same presentation craft.

```text
ordinary Project
    directory / repository / prose / code
        ↓ optional deeper authoring
vision / design / diagrams / architecture / evidence
        ↓ optional account reading
WorldPresentation / HTML / Explore
```

The existence of a Project account therefore says nothing about whether every Factory primitive is present.

## 7. Run and evidence readings

Runs are developmental continuity, not merely terminal sessions.

A Project account can present selected Run material through source-aware modules such as:

- Run history;
- RunMap or topology reading;
- Claim/Evidence relation;
- Candidate comparison;
- current Build state;
- Recognition decision;
- returned difference.

These modules must point back to native Factory refs/revisions when available rather than copying Run or Evidence state into a presentation-owned substitute.

## 8. Recognition and Return

The deepest value of the account is not documentation after the fact. It is that encounter with a coherent reading can participate in Return.

```text
Project reality
    ↓ coherent account / candidate presentation
human encounter
    ↓
Recognition, rejection, qualification, or new understanding
    ↓
returned difference
    ├── implementation change
    ├── design change
    ├── intent change
    └── renewed ground
```

O:I Projection can host the presentation and revision history of the reading. It does not own the decision that changes Factory Project source.

The Project's own authoring and Recognition authority remains responsible for accepted return.

## 9. O:I Projection handoff

When a Project reading is prepared for O:I, use the existing O:I Projection and WorldPresentation architecture.

```text
Factory Project source revision F12
        ↓ selected reading
WorldPresentation W1
        ↓ ratification
O:I Projection P1
        ↓ representation refinement
O:I Projection P2
        ├── source still F12
        └── editor provenance retained
```

A presentation refinement does not mutate the Factory Project.

If the refinement should change the Project itself, return it through the Project's native development/review/Recognition path.

## 10. Important terminology collision

Factory currently contains a Rust `ProjectionIdentity` in `factory/src/core/identity/projection.rs`.

That type resolves an external provider identity back to an optional canonical Factory `Ref`:

```text
provider + external_id
        ↓
canonical_ref
```

It is **not** the O:I `oi.projection/v1` publication/presence object.

Do not merge these semantics because they share the word “Projection”. A Project account may cite Factory canonical refs and then be wrapped by an O:I Projection, but the two contracts perform different jobs.

## 11. HTML presentation

The canonical standalone HTML account format is a renderer of a Project reading, not Project source.

```text
Project source / Wiki / Runs / evidence
        ↓
structured account reading
        ↓
HTML renderer
```

A standalone HTML artifact can provide deep prose, diagrams, source links, timelines, comparisons, mockups, review controls, local notes, and machine-readable provenance.

The same structured reading should also remain available directly to Agents. Agents must not need to scrape HTML to recover Project refs, Claims, Evidence, relations, or provenance.

## 12. QL-informed, not QL-imposed

Factory's QL corpus can inform the completeness of an authored reading where useful.

That does not require visible QL terminology.

A software Project may naturally expose headings such as:

```text
Why this exists
The product
Experience
Design
System
Current frontier
```

Another Project can use a different structure. The account succeeds when the represented Project becomes more whole and intelligible, not when a visible sixfold is manufactured.

## 13. Coordination with current development

Two current development branches deepen the Project meaning this integration consumes:

- PR #152 restores the Factory whole as human-authored developmental continuity from intention through Candidate, evidence, Recognition, and Return;
- PR #153 adds visual product understanding of human authorship, development, candidate reality, and returned difference.

Those PRs are current development state until accepted. This integration does not promote them to merged implementation truth.

The live `main` Rust seams remain the authority for current executable Project/Run identity and topology behaviour.

## 14. Ownership summary

The relation preserves four owners:

```text
Factory
    Project development semantics, Runs, evidence, Recognition / Return

AIKit
    reusable product-understanding and account-authoring procedure

O:I
    Projection, WorldPresentation, Explore, presentation revision

native Project source
    canonical authored material and implementation
```

The account connects these worlds without replacing any of them.
