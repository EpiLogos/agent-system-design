# Workcell Repository Handoff

**Date:** 2026-08-14  
**Implementation repository:** `EpiLogos/Workcell`

The Factory root programme remains authoritative for shared semantic dependencies and Factory integration gates. Workcell product code is developed in `EpiLogos/Workcell`.

Workcell product programme:

- #1 product Wayfinder
- #2 public contract + deterministic reference provider
- #3 provider replaceability/degradation/lifecycle conformance
- #4 workspace materialisation provider
- #5 container provider, gated by Factory #24
- #6 cross-repository conformance, gated by Factory #113
- #7 Candidate material-reality integration

Factory #11 owns shared execution/materialisation meaning. Factory #57-#61 remain integration/acceptance gates; they are not the Workcell code location. Factory #83 enforces the boundary, and #113 owns the language-neutral fixture floor.

Factory #57 is still blocked by its declared dependencies. Workcell #2 may proceed because it implements only Workcell-owned behavior and treats external semantic refs as opaque. It must not invent unfinished Factory fields.
