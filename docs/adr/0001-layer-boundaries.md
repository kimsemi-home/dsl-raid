# ADR 0001: Layer Boundaries

## Status

Accepted.

## Context

DSLRaid can easily collapse into a diagram application if semantic IR, analysis,
layout, rendering, and app shell state are mixed. The project needs explicit
boundaries before implementation starts.

## Decision

DSLRaid is an IR compiler, analyzer, projection engine, and viewer. The core
model is typed executable IR.

The layers are separate:

- `dslraid-core`: pure IR, IDs, schema, validation
- `dslraid-analyzer`: reachability, determinism, composition, projection,
  diagnostics
- `dslraid-layout`: graph conversion, ELK adapter, cached layout
- `dslraid-render`: view model, canvas model, SVG export
- `dslraid-wasm`: browser bindings
- `apps/viewer`: search, inspector, shortcuts, file loading, UX state
- `cli/dslraid`: validate, compose, project, render

The core IR must not include:

- coordinates
- color
- selected state
- expanded/collapsed UI state
- Canvas/SVG/WebGL details
- layout cache details

These concepts must remain distinct:

1. semantic ID
2. layout ID
3. source location
4. diagnostic subject
5. UI selection state

## ADR-Required Changes

These changes require an ADR or accepted design issue before implementation:

- Core IR field changes
- ID stability rule changes
- FSM composition semantic changes
- transition/guard/action semantic changes
- diagnostic code changes
- diagnostic severity policy changes
- public/private visibility changes
- provenance and trust-boundary changes
- lock file, artifact freshness, or compatibility contract changes
- generated code contract changes

## Safe Autonomous Refactoring

These changes may be done without an ADR when public behavior is preserved:

- UI layout
- color, icon, and panel composition
- zoom and pan UX
- minimap
- search UX
- inspector panel layout
- layout cache implementation
- SVG export implementation
- Canvas/WebGL optimization
- keyboard shortcuts
- example projects
- internal function extraction
- test fixture additions
- CLI output formatting improvements
- friendlier error messages

Short rule:

```text
Renderer and UI can move freely.
Core IR, composition, validation meaning, visibility, diagnostic codes,
diagnostic severity, provenance, stable IDs, lock/compat contracts, and codegen
contracts require an ADR.
```

## Consequences

The project gets more files and boundaries earlier, but the core meaning layer
stays stable as rendering, layout, and app UX evolve.
