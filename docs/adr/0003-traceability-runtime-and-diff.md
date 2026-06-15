# ADR 0003: Traceability, Runtime Evidence, and Diff

## Status

Accepted.

## Context

DSLRaid's strongest value is not visualization. It is verifiable traceability
across design, policy, generated artifacts, tests, runtime telemetry, and PR
review.

The key comparisons are:

- designed transition
- tested transition
- generated/deployed artifact
- actually observed runtime transition

## Decision

Design IR, runtime traces, coverage overlays, and source maps are separate data
products.

```text
Design IR
  != Runtime Trace
  != Coverage Overlay
  != Source Map
  != View Model
```

Schemas:

- `dslraid-core.schema.json`: design meaning
- `dslraid-assertion.schema.json`: assertion registry
- `dslraid-trace.schema.json`: runtime event log
- `dslraid-coverage.schema.json`: design/runtime coverage overlay
- `dslraid-sourcemap.schema.json`: DSL/IR/generated/runtime mapping
- `dslraid-view.schema.json`: rendered view model
- `dslraid-index.schema.json`: search/cross-reference index
- `dslraid-lock.schema.json`: resolved hashes, derivations, artifacts
- `dslraid-annotation.schema.json`: notes, links, review context
- `dslraid-validation.schema.json`: proposition/assertion validation report

## Required CLI Surface

The CLI must treat these as first-class operations:

```bash
dslraid validate .dslraid.json
dslraid migrate --from 0.1.0 --to 0.2.0 .dslraid.json
dslraid trace import logs/run-123.jsonl
dslraid project .dslraid.json --visibility public
dslraid compose .dslraid.json --materialize reachable
dslraid compose .dslraid.json --focus state:runtime.running
dslraid compose .dslraid.json --diagnostics-only
dslraid diff base.json head.json
dslraid artifact verify
dslraid compat check
dslraid doc generate .dslraid.json
dslraid export mermaid .dslraid.json
dslraid export dot .dslraid.json
dslraid export svg .dslraid.json
```

## Determinism

All generated outputs must be deterministic:

- JSON field order
- array order
- generated code
- SVG output
- diagnostic order
- layout cache key
- source map ordering
- coverage output ordering

## Versioning

Core IR requires explicit migrations between schema versions.

```bash
dslraid migrate --from 0.1.0 --to 0.2.0 .dslraid.json
```

The project should not promise round-trip editing from ViewModel to DSL in the
MVP. The supported direction is:

```text
DSL -> Core IR -> Projection -> View
```

View edits should be deferred until LSP/edit operations exist.

## PR Review

Diff is a first-class product feature.

The PR summary should eventually answer:

- Which FSMs changed?
- Were states or transitions added/removed?
- Did a new terminal path appear?
- Was a policy/capability edge disconnected?
- Did an untested transition appear?
- Did runtime coverage improve or regress?

## Ecosystem Policy

Keep the core closed and stable before opening a plugin ecosystem.

Stabilize first:

- core kinds
- FSM semantics
- diagnostics
- derivation contract
- trace and coverage contracts

Open plugin APIs later.
