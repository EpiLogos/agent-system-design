# Factory Build surface

React/TypeScript implementation for Factory #143–#145. It deliberately has two layers:

1. an SSSF source-fidelity execution optic (`SessionCards`, `TraceWaterfall`, `SpanDetail`, `sssf.ts`); and
2. the Factory semantic envelope (`BuildSurface`) where Project → Run → frontier → Candidate/Claim/Evidence/HumanRequest/Recognition remains primary and execution becomes drill-down.

The package uses the O:I semantic CSS variable names and supplies fallbacks for standalone development. When hosted by O:I, import `@epilogos/oi-design-system/tokens.css` in the host rather than copying a second token authority into Factory.

The package does **not** own SessionSpace, Surface, HarnessComposition, Workcell bindings, Actuation root/metagency/Return, or native harness trajectories. Those enter the read model as stable external refs/provenance.

## Verify

```bash
cd factory-ui
npm install --no-audit --no-fund
npm run verify
```

See `docs/SSSF-SOURCE-FIDELITY.md` for the exact upstream pin, licence and parity map.
