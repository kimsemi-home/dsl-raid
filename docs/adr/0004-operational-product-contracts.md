# ADR 0004: Operational Product Contracts

## Status

Accepted.

## Context

DSLRaid needs to work as an open source product that can be installed, reviewed
in CI, adopted by other projects, and trusted with generated artifacts. A viewer
and typed IR are not enough without file format, freshness, compatibility,
provenance, and governance contracts.

## Decision

DSLRaid will treat the following as first-class product contracts:

- canonical IR, assertion, lock, view, annotation, trace, coverage, validation,
  and source-map files
- stale generated artifact detection
- provider/runtime/protocol/capability/constraint modeling
- policy references on executable IR subjects
- runtime event mapping
- import/export contracts with explicit lossiness
- semantic diff and review mode
- compatibility checks
- provenance and trust classification
- query over projected/indexed architecture data
- diagnostic severity policy
- benchmark and fuzzing strategy
- deterministic layout policy
- accessibility requirements for the viewer
- internal extension points before public plugins

## File Family

```text
*.dslraid.json              canonical Core IR
*.dslraid.assertions.json   assertion registry
*.dslraid.lock.json         hashes, resolved refs, derivations, artifacts
*.dslraid.view.json         layout and view cache
*.dslraid.annotations.json  user annotations
*.dslraid.trace.json        runtime trace
*.dslraid.coverage.json     coverage overlay
*.dslraid.sourcemap.json    source/generated/runtime mapping
*.dslraid.validation.json   validation proposition/assertion report
```

Only `*.dslraid.json` is canonical design meaning. The rest are derived,
operational, imported, or user-facing companion products.

## CLI Contract

Stable command names:

```bash
dslraid init
dslraid normalize
dslraid validate
dslraid compose
dslraid project
dslraid render
dslraid diff
dslraid trace import
dslraid artifact verify
dslraid compat check
dslraid query
dslraid import
dslraid export
dslraid migrate
```

## Artifact Freshness

Generated artifacts must record the IR hash and derivation that produced them.

```text
IR hash != artifact input hash -> stale artifact
disk content hash != artifact content hash -> stale artifact
```

`dslraid artifact verify` is the public gate. `dslraid artifact lock update`
is the deterministic regeneration tool. CI should include verification once
codegen exists.

## Severity Policy

Diagnostics use stable severity:

- `error`: merge block
- `warning`: review needed
- `info`: contextual information
- `hint`: improvement suggestion

Diagnostic code changes require an ADR.

## Import and Export

Imports carry provenance. Imported formats are not canonical unless normalized
into DSLRaid Core IR and accepted by validation.

Exports must declare whether they are lossy:

- Mermaid: lossy
- DOT: lossy
- SVG: lossy
- Markdown: lossy
- canonical JSON: lossless

## Compatibility

`dslraid compat check` validates that IR requirements, provider capability,
protocol version, generated artifact metadata, and runtime constraints agree.

## Provenance and Trust

Objects should be classified by origin:

- trusted source
- generated source
- external imported source
- runtime trace
- user annotation

Public projection must redact or reject private, secret-bearing, or untrusted
data according to visibility policy.

## Accessibility

Canvas is not the accessibility surface. Every Canvas-visible fact must also be
reachable from HTML panels such as inspector, search results, diagnostics, and
timeline lists.

## Consequences

- The project gains more file formats, but each has a clear role.
- CI can detect stale generated outputs.
- Provider/runtime adoption becomes possible without overloading FSM semantics.
- Public sharing becomes safer because visibility, provenance, and annotations
  have explicit boundaries.
- Plugin implementation is deferred, but extension points can be designed early.
