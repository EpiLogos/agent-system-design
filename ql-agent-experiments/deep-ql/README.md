# Deep QL workbench

This directory implements #104–#108 **above** the frozen Direct Core foundation. It does not modify the host/runtime contract or the Foundation source.

## Foundation pin

- merged branch point: `7690069846eb6fc89f6aa78dcf7aab886ac7c737`
- frozen executable runtime candidate: `11c7be5767735c25f49139906b59398bcdf3bf42`

Direct Core semantics remain invariant: real Circuit state; 4+2; all `Rij`; `π 0/1` / `ρ 1/0`; difference before interpretation; explicit P5 determination; `R51/R52/R53/R54/R50` as continuation of the same open process; `QLClosure` as the only positive closure; `ReentryDelta`/P0+ only after closure; Run != Closure.

## Required Deep profile

`operators.js` implements fresh-context conjugation (§24) and P4-oriented recursive depth (§25). `conformance/` exposes the 61 stable required QLC IDs and portable schemas. `typing-corpus/` materialises the 100-act semantic corpus. `render/` and `comparison/` provide replay rendering and cross-runtime trace comparison.

## Status boundary

The required machine-executable profile passes locally, but **#107 is not closed** because §42 requires human review/adjudication of the 100-act corpus. The corpus is prepared and replayable with every record explicitly marked `pending-human-review`. No Series 1 / #109 readiness claim is made until that condition is satisfied.

Research-only structures are isolated under `extensions/` and `research/`; they are ignorable by Core consumers and do not block the required profile.
