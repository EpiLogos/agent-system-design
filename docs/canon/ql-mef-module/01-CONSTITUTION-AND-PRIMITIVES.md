# Part I — Constitutional relation

## 1. Four distinct layers

The design preserves four non-identical layers.

### 1.1 QL canon

The formal bimba: sixfold positions, 4+2 structure, direct/conjugate faces, relation structures, contextual recursion, lens manifold, harmonic and other formal relations insofar as they belong to canon.

The canon is not a software package version.

Its articulation may become clearer over time without being semantically replaced by a kernel release.

### 1.2 Executable QL kernel

The versioned software formalisation of portions of QL that are computationally exact enough to implement.

The kernel may progressively support:

- sixfold positions;
- `4+2` distinction;
- direct/conjugate face relations;
- QL addresses;
- validated relation fields;
- typed transitions;
- contextual recursion;
- projection/return structures;
- formal masks/relations where canonically established;
- harmonic operators where computational semantics become exact.

A kernel version is an implementation revision, not a revision of QL canon.

### 1.3 MEF semantic layer

The complete twelve-lens manifold and machinery for refracting subjects through one or more lenses while retaining the subject's identity.

All twelve lenses remain structurally first-class.

A software system may discover particularly useful lens roles, but no application may redefine MEF around its own object model.

### 1.4 Software field

AIKit, Factory, agents, Projects, applications, and research runtimes are clients/adapters.

They may use QL architecturally even without a live provider, and they may request explicit operations when a provider exists.

Software experiments can disclose useful computational interpretations of QL. They do not determine QL canon by convenience.

---

## 2. Module identity

The QL/MEF module is not:

- the Software Factory;
- AIKit;
- an agent harness;
- the QL-native Loop Runtime;
- a Skill collection;
- an ontology migration tool;
- a tag generator;
- a mandatory scheduler;
- a metaphysical dependency required before ordinary code can run.

It is:

> **a versioned library/service/provider which gives software a precise way to reference QL forms and lenses, request formal or semantic readings, and receive attributable results without transferring ownership of the subject into the QL system.**

---

## 3. Alignment, not translation

A Factory `Claim` remains the same Claim before and after refraction.

An AIKit `Capability` remains the same Capability.

An `Agent` remains the same Agent.

A `Project` remains the same Project.

The QL/MEF module returns a reading **of** the subject.

```text
Subject Ref
    │
    ▼
QL / MEF operation
    │
    ▼
derived reading
    │
    └── still points to Subject Ref
```

The module must not require applications to rename their primitives as `P0`, `L4`, `Earth`, `Logos`, or any other QL/MEF label in order to interoperate.

---

# Part II — Operational parity

## 4. Operational parity

A claimed QL integration is meaningful only when the QL relation has observable consequence in software behaviour, interpretation, or evidence.

Weak parity:

```text
record.ql_position = 5
record.meaning = "return"
```

with no effect on the system.

Strong parity may include:

- a returned difference changing the next recurrence in a QL runtime;
- a lens refraction causing a distinct evidence/assessment view;
- a conjugate operator constructing an independent fresh-context reading;
- a QL relation becoming a durable provenance-bearing Claim;
- a QL address altering a valid operator path;
- a lens-aware capability explicitly requested and its effects compared to baseline.

Operational parity does not require every QL relation to become executable immediately.

It requires honesty about what is merely descriptive versus what has computational consequence.

---

## 5. Parity evidence contract

Whenever a feature is advertised as **QL-operational**, its design/evaluation should identify:

```text
baseline behaviour
QL-enabled behaviour
observable difference
trace/evidence
canonical QL form/lens relation
provider/version
```

This allows QL-inspired design, passive metadata, semantic refraction, and genuinely QL-governed runtime semantics to remain distinguishable.

---

# Part III — Shared semantic references

## 6. Shared primitive intent across repositories

QL/MEF primitives may have language-specific representations in different repositories while preserving one semantic contract.

The module should define stable JSON/schema-level intent before demanding one shared implementation library across Rust, Python, TypeScript, or experimental runtimes.

Initial shared semantic references should include:

```text
QLFormRef
QLAddress
LensRef
QLTarget
QLReadingRef
QLProviderRef
```

Additional result types are introduced below.

---

## 7. QLFormRef

`QLFormRef` identifies a canonical or module-supported QL form without copying the form's full definition into every client.

Examples might refer to:

```text
sixfold
4+2
0/1 : 1/0
direct/conjugate face
relation field
contextual recursion
```

Exact identifiers require a later registry specification.

A `QLFormRef` is a semantic reference, not proof that the current provider implements every operation over that form.

---

## 8. QLAddress

`QLAddress` locates a position/orientation inside a supported QL frame.

Conceptually:

```text
QLAddress {
    position: P0 | P1 | P2 | P3 | P4 | P5
    face: direct | conjugate
    depth?: integer
    frame?: QLFormRef
}
```

The exact shape may deepen as nested frames and richer canon become executable.

The fundamental constraint is that `P0…P5` are **raw QL addresses before any one software vocabulary**.

A client must not equate:

```text
P1 = read
P2 = tool call
P3 = architecture
P4 = test
P5 = final answer
```

Such activities may occupy those positions under a particular refraction. They do not define the addresses.

---

## 9. LensRef

`LensRef` identifies one canonical MEF lens.

Initial canonical registry:

```text
L0   Quaternal
L0′  Archetypal-Numerical

L1   Causal
L1′  Phenomenal

L2   Logical
L2′  Alchemical-Elemental

L3   Processual
L3′  Chronological

L4   Phenomenological
L4′  Scientific

L5   Para Vāk
L5′  Divine Logos
```

The twelve organise as three Klein squares:

```text
Square A — Articulation [0 + 5]
    L0 · L0′ · L5 · L5′

Square B — Encounter [1 + 4]
    L1 · L1′ · L4 · L4′

Square C — Becoming [2 + 3]
    L2 · L2′ · L3 · L3′
```

The module's lens registry must preserve these identities and their canonical internal sixfold structures.

---

## 10. QLTarget

A `QLTarget` points at something independently owned which is being read through QL/MEF.

Conceptually:

```text
QLTarget {
    subject: Ref
    subject_type?: string
    frame?: Ref | QLFormRef
    context_refs?: Ref[]
}
```

`Ref` belongs to the client ecosystem, not necessarily the QL module.

The module treats the subject reference as opaque identity plus whatever structured input the caller deliberately provides.

---

## 11. QLReading

A `QLReading` is a derived result, not canonical replacement truth.

Conceptually:

```text
QLReading {
    id
    target: QLTarget
    operation
    ql_form?: QLFormRef
    address?: QLAddress
    lens?: LensRef
    reading
    confidence_or_status?: ...
    evidence_refs?: Ref[]
    warnings?: ...
    provenance: QLProvenance
}
```

A semantic reading may be expressed as structured relations, Claims, or prose depending on provider capability.

The subject identity is always retained.

---

## 12. QLRelationReading

For operations over two or more subjects:

```text
QLRelationReading {
    id
    subjects: Ref[]
    frame?: QLFormRef
    relation
    addresses?: QLAddress[]
    lenses?: LensRef[]
    evidence_refs?: Ref[]
    provenance
}
```

The relation is a derived account between existing subjects.

---

## 13. QLSynthesis

`QLSynthesis` integrates several prior readings while preserving their provenance.

```text
QLSynthesis {
    id
    input_readings: QLReadingRef[]
    synthesis
    retained_differences?: ...
    unresolved?: ...
    provenance
}
```

Synthesis must not erase disagreement among source readings merely to produce one smooth answer.

---

## 14. Provenance

Every explicit QL/MEF result must record enough provenance to answer:

```text
which provider produced this?
which provider version?
which QL canon/form references were claimed?
which MEF lens registry revision?
which operation?
which input refs/revisions?
which model was used, if semantic inference used one?
which deterministic/stochastic mode?
which warnings or unsupported semantics applied?
```

Old readings remain intelligible after provider upgrades because their provider/version remains attached.

---
