# Deep QL runtime profile

This directory implements the required Deep QL profile above the frozen Direct Core foundation. It does not modify the Foundation source or the completed Pi, Pydantic AI or Native host implementations.

## Foundation pin

- merged Foundation branch point: `7690069846eb6fc89f6aa78dcf7aab886ac7c737`
- frozen executable runtime candidate: `11c7be5767735c25f49139906b59398bcdf3bf42`

Direct Core semantics remain invariant: real Circuit state; 4+2; all `Rij`; `π 0/1` / `ρ 1/0`; difference before interpretation; explicit P5 determination; `R51/R52/R53/R54/R50` as continuation of the same open process; `QLClosure` as the only positive closure; `ReentryDelta`/P0+ only after closure; Run != Closure.

## Required Deep profile

- `operators.js` implements fresh-context whole/current-position conjugation (§24) and P4-oriented recursive depth (§25).
- `operator-session.js` makes those operators observable through portable conjugate/child/reopening lifecycle events without changing the frozen host contract.
- `conformance/` exposes the 61 stable required QLC IDs as substantive executable assertions plus portable schemas.
- `typing-corpus/` materialises the shared 100-case semantic benchmark with stable identities, benchmark provenance and optional real human witnesses.
- `render/` renders Direct, Conjugate and child Circuit state without treating carrier prose as QL meaning.
- `comparison/` compares full portable semantic signatures rather than relation strings alone.
- `convergence/` proves the common profile across Pi, Pydantic AI and Native and records the representative product-review runs.

## Review policy

`../TYPING-CORPUS-REVIEW-CLARIFICATION-08-14-2026.md` governs corpus review. The 100 cases are an executable benchmark, not a 100/100 manual annotation gate. Human witnesses may be added selectively. Human acceptance is product-level: inspect the runtime behaviour, traces, operator sessions, comparisons and representative review cases.

## Commands

From this directory:

```text
npm test
npm run conformance
npm run corpus
npm run corpus:ndjson
```

`npm run readiness` and `npm run review` exercise the three completed host profiles and therefore run on the PR merge ref/current integrated tree where Pi, Pydantic AI and Native are present. CI publishes the `review-bundle.json` output as the product-review artifact.

## Experiment Readiness

#104, #105, #106, #107 and #109 are complete. The verified convergence profile provides:

- frozen Direct Core shallow equivalence on all three hosts;
- observable whole/current conjugation and recursive depth;
- 61/61 required QLC conformance;
- the shared 100-case benchmark replayed across all three profiles;
- one renderer and semantic comparator;
- 8 representative product-review scenarios × 3 hosts = 24 recorded runs.

#108 remains an independent, non-blocking research track. Research-only structures stay under `extensions/` and `research/`; they are namespaced, ignorable by Core consumers and do not silently become control semantics.
