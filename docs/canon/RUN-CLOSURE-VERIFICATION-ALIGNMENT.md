# Run, Closure, Gate, and Verification Alignment

**Status:** canonical clarification subordinate to the Constitutional Index and read with the Primitive Relations specification.  
**Purpose:** align the shared developmental primitives before AIKit verification/provider implementation and QL runtime deepening.

---

## 1. Determination

`Run`, `Closure`, and `Gate` are distinct semantic primitives/relations and must not be collapsed by implementation convenience.

```text
Run
    chronological execution and observation of an intended transformation

Closure
    sufficient determinacy relative to the operative whole and the condition that opened it

Gate
    rule or assessment controlling whether a requested transition is permitted
```

A Run may end without Closure.

Closure may require effects, Evidence, Assessments, Decisions, or Recognition accumulated across one or many Runs.

A Gate may require Closure, a narrower verified condition, a human determination, or a composite of these. Passing a Gate therefore does not redefine Closure, and a provider workflow finishing does not define either one.

This clarifies rather than replaces the existing constitutional meaning of Run as a durable intended Project transformation and Gate as an evidence-sensitive transition control.

---

## 2. Runs are the common carrier across products

The presence of shared primitives across Factory, AIKit, Workcell, agent runtimes, GitHub projections, and QL/MEF integrations is intentional architecture.

Different products may implement different representations, stores, read models, and provider bindings while retaining the same semantic identity.

A CI workflow, verification sequence, release sequence, review sequence, or deployment check therefore does not require a new top-level Pipeline ontology merely because one provider expresses it as a graph of jobs.

At the shared level it can be represented as a Run, nested Run, or a provider projection of Run execution:

```text
Run
  ├── Action / ActionInvocation
  ├── Execution
  ├── Event / Trace
  ├── Artifact
  ├── Claim
  ├── Evidence
  ├── Assessment
  ├── Decision
  └── nested / related Run
```

Provider-specific workflow/job/step concepts remain legitimate provider vocabulary. They do not become constitutional replacements for the shared carrier.

---

## 3. Opening, span, and closure

The general process grammar is:

```text
OPENING / INSTIGATION
        ↓
DIRECTED SPAN OF BECOMING
        ↓
ACTUAL RUN CHRONOLOGY
        ↓
EFFECTS / EVIDENCE / ASSESSMENTS
        ↓
CANDIDATE DETERMINATION
        ↓
CLOSURE or DISCLOSED INSUFFICIENCY
        ↓
RETURN / RE-ENTRY / NEXT SPAN
```

No new universal `Instigator` object is required at this point. Existing primitives may open a span: HumanRequest, intent, ActionInvocation, Decision, Event, Run opening, Claim, or another Project condition.

The important shared relation is that something **opens** or **initiates** a directed process whose closure conditions can later be judged.

Likewise, an ActionInvocation can have a local completion condition while participating in a larger Run that remains open. Closure is therefore naturally nested.

```text
tool Execution completes
    ↓
ActionInvocation locally completes
    ↓
verification leg completes
    ↓
verification Run may close
    ↓
implementation Run may remain open
    ↓
Project concern may remain open
```

---

## 4. Closure is semantic/processual, not merely temporal

Run chronology answers:

> **What actually happened, in what order, through which executions and returns?**

Closure answers:

> **Has the becoming that was opened become sufficiently determinate relative to its operative whole and initiating intent?**

This makes termination, determination, and closure non-identical.

A process can stop because a tool errored, a provider cancelled execution, a session ended, an agent lost context, or a workflow exhausted its graph. None of these conditions by themselves imply semantic Closure.

Similarly, a candidate determination can appear while the process remains open because the determination itself discloses an insufficiency.

---

## 5. Causal disclosure

The closure-side process may be described as **causal disclosure**:

> the causal, evidential, contextual, and authority conditions warranting a determination become sufficiently explicit, current, and properly related to the subject being determined.

Verification is one strong instance of causal disclosure, but not the only one.

For engineering work:

```text
Completion Claim
        ↓
requires current qualifying Evidence
        ↓
Verification-oriented Run(s)
        ↓
Results / Evidence / Assessments
        ↓
causal disclosure of whether completion conditions hold
        ↓
Closure determination
```

A strong invariant is:

> **A consequential completion Claim must be grounded in current Evidence whose Subject/state matches the state being claimed complete.**

The exact evidence burden remains Project- and context-sensitive rather than globally maximal.

---

## 6. Verification and CI are specialisations, not governing workflow ontology

Verification is a feature of the engineering environment and a family of evidence-producing operations.

A Project may use almost none of the maximal assurance field, or may use focused tests, type/static checks, broad suites, independent review, compatibility matrices, merge-group checks, release verification, protected deployment, and runtime smoke checks.

The architecture does not mandate one sequence.

Instead:

```text
Project verification requirement
        ↓
resolved verification demand
        ↓
verification-oriented Run(s)
        ↓
Evidence / Assessment
        ↓
Closure input and/or Gate condition
```

GitHub Actions and other CI systems materialise and enforce some of these Runs. AIKit may discover and resolve them. Factory retains developmental meaning. No one of these products becomes the universal workflow owner.

---

## 7. Gate relation

A Gate remains an operational/developmental transition rule.

Examples:

```text
Gate:
    may this Candidate enter Application?

Gate:
    may this pull request enter the protected branch?

Gate:
    may this release artifact enter deployment?
```

A Gate evaluates Claims, Evidence, Assessments, Decisions, Recognition, or Closure state according to the transition being attempted.

It may be deterministic, agent-assessed, human-recognised, provider-enforced, or composite.

Provider enforcement is a materialisation of the Gate relation, not its semantic source unless the Project explicitly treats that provider configuration as authoritative source.

---

## 8. QL / MEF alignment

This technical architecture does not depend on MEF, but it admits a natural non-coercive refraction:

```text
L3   Processual
     the closure-bearing becoming / concrescence / meaningful span

L3′  Chronological
     the actual Run history, sequence, ancestry, branch and return

L1   Causal
     causal constitution and causal disclosure of the conditions
     warranting the determination

L4′  Scientific
     observation, test, verification, correction and evidence work
```

This alignment helps distinguish process meaning from chronological trace and empirical verification without redefining any technical primitive as a lens.

---

## 9. QL runtime consequence

For a QL-native Loop Runtime, `P5` is a candidate determination, not Closure itself.

The same still-open becoming may continue through return relations when P5 discloses insufficiency. Positive QL Closure is the condition by which the present recurrence is sufficiently determined; retained difference may then condition re-entry / `P0+`.

Architecturally this favours:

```text
shared carrier / host mechanics
        +
selectable Loop Logic
```

rather than duplicating entire loops.

QL changes recurrence, determination, closure, return, and re-entry semantics. It does not merely replace one stop predicate with another.

---

## 10. Implementation consequence

Across repositories, prefer shared semantic contracts and references before shared implementation packages.

The same `Run`, `Evidence`, `Assessment`, `Gate`, `Closure`, `Action`, `Execution`, `Agent/Agency`, and Project references may have Rust, Python, TypeScript, JSON, provider-native, or runtime-specific materialisations while preserving one meaning.

This enables the wider architecture to compose:

```text
Project intent
    ↓
Run
    ↓
AIKit-resolved actor/world/capabilities/verification
    ↓
Workcell / provider materialisation
    ↓
Execution + Evidence + Assessment
    ↓
Closure / Gate / return
    ↓
Project history and changed ground
```

without requiring every product to own every primitive or duplicate every process.
