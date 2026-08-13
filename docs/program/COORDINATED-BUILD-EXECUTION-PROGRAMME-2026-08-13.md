# Coordinated Software Factory Build Execution Programme

**Status:** planning / execution-map draft — no broad implementation authorised.

This branch records the coordination pass over the existing Factory programme.

## Governing ticket maps

- Cross-repository contracts: #113 interoperability; #114 verification / Closure / Gate.
- QL runtime: preserve existing #94–#110 and close #100 exactly before #101–#104 begin.
- Standalone QL/MEF: #115 Wayfinder with #116–#122.
- AIKit V2: `EpiLogos/ai-kit#23` with V2-A→H at #24–#31.
- Central: preserve Wayfinder #1; new #23 provides the remote verification floor before the existing #7+ graph resumes.

## Ownership amendments

Factory #44 is now the Factory↔AIKit integration span; Factory #77 is the Factory↔QL/MEF integration span; Factory #89 tracks migration acceptance while the Rust ProjectBinding migration belongs to AIKit V2-A; Factory #37 now follows #114 verification semantics.

## First tranche after approval

1. Finish the evidence delta and truthful Foundation Freeze on existing QL runtime PR #112.
2. Execute Factory root/source-ground tickets and establish the real implementation workspace through #16, then build #113/#114.
3. Add Central #23 to current PR #22, obtain exact-head CI evidence, merge, then resume #7/#8/#11/#12 in parallel.
4. Ratify QL/MEF PR #111 and run #116→#118 without waiting for runtime #100; wait only at #120.
5. Ratify AIKit PR #22 and start #24 only after its consumed #113 contract subset is pinned.

Programme-level Closure is reached when these maps are ratified and the next instruction can be simply: **execute the programme**.