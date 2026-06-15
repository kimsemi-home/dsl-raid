# DSLRaid

Composable DSL Runtime & Interactive Architecture Explorer.

DSLRaid is an executable architecture browser. It turns domain DSLs, finite
state machines, DDD models, policies, commands, events, capabilities, generated
code, and tests into one explorable executable architecture model.

The project is not a diagram-first Mermaid or PlantUML replacement. Its center
is a typed executable IR that can be analyzed, composed, matched, projected,
rendered, diffed, and used for code generation.

## Positioning

DSLRaid is for teams that want architecture to be executable instead of
decorative:

- IR first, diagram second
- Executable SSOT
- FSM composition
- Interactive architecture exploration
- Source, test, doc, and generated artifact traceability
- Code generation from the same model used by the viewer
- Operational checks for stale artifacts, compatibility, semantic diff, and
  public/private projection safety

## Core Idea

The core model is typed executable IR:

- `project`
- `context`
- `fsm`
- `state`
- `event`
- `transition`
- `guard`
- `action`
- `composition`
- `projection`
- `derivation`
- `artifact`

Transitions are structured executable objects, not generic relations. Views,
docs, tests, code, and render indexes are derived from the typed IR.

The UI is an architecture IDE over that executable model. A user should be able
to click from a project or context to a runtime FSM, from the FSM to a state or
transition, and from there to source definitions, tests, generated code, and
related policies.

## Stack

- Common Lisp for authoring DSLs and SSOT-friendly macros
- Rust for canonical IR, JSON Schema validation, FSM analysis, projection,
  code generation, and CLI workflows
- TypeScript for the web application shell
- Canvas 2D for the interactive graph viewport
- WASM, WebGL, ELK, and Graphviz remain later expansion points

## Repository Map

This repository now has a working MVP implementation around the original
design contracts.

- `crates/dslraid-core`: typed Core IR structs, JSON loading, stable hashes,
  and JSON Schema validation
- `crates/dslraid-analyzer`: FSM and traceability semantic validation reports
- `crates/dslraid-codegen`: projection, SVG render, Mermaid/DOT export, and
  Rust/Go/TypeScript FSM codegen
- `crates/dslraid-cli`: executable CLI over the core/analyzer/codegen crates
- `apps/viewer`: TypeScript Canvas viewer with search, hit-testing, inspector,
  and diagnostics panels
- `lisp`: Common Lisp data-first DSL macros, normalization, validation, and
  deterministic JSON emitters

- [Architecture](docs/architecture.md)
- [IR Design](docs/ir.md)
- [Validation Proposition Catalog](docs/validation.md)
- [Traceability and Runtime Evidence](docs/traceability-runtime.md)
- [Viewer Architecture](docs/viewer-architecture.md)
- [Viewer Rendering Guide](docs/viewer-rendering.md)
- [Refactoring Guide](docs/refactoring.md)
- [Operational Product Contracts](docs/operational-contracts.md)
- [Common Lisp DSL Guide](docs/lisp-dsl.md)
- [CI Strategy](docs/ci.md)
- [Roadmap](docs/roadmap.md)
- [Open Source Strategy](docs/open-source.md)
- [Layer Boundaries ADR](docs/adr/0001-layer-boundaries.md)
- [Product Scope ADR](docs/adr/0002-product-scope-and-risk-boundaries.md)
- [Traceability ADR](docs/adr/0003-traceability-runtime-and-diff.md)
- [Operational Contracts ADR](docs/adr/0004-operational-product-contracts.md)
- [Core IR schema](schemas/dslraid-core.schema.json)
- [Assertion registry schema](schemas/dslraid-assertion.schema.json)
- [Index graph schema](schemas/dslraid-index.schema.json)
- [View model schema](schemas/dslraid-view.schema.json)
- [Runtime trace schema](schemas/dslraid-trace.schema.json)
- [Coverage overlay schema](schemas/dslraid-coverage.schema.json)
- [Source map schema](schemas/dslraid-sourcemap.schema.json)
- [Lock file schema](schemas/dslraid-lock.schema.json)
- [Annotation schema](schemas/dslraid-annotation.schema.json)
- [Validation report schema](schemas/dslraid-validation.schema.json)
- [RunScope example fixture](examples/runscope/runscope.raid.json)

## Development Principle

Start with a small, stable executable IR kernel. Let renderers, code
generators, graph indexes, and language integrations grow around it as
replaceable adapters.

The implemented product contract is CLI-first:

```bash
cargo run -p dslraid-cli -- init .dslraid.json
cargo run -p dslraid-cli -- normalize examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- validate examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- validate examples/runscope/runscope.raid.json --format json
cargo run -p dslraid-cli -- schema validate schemas/dslraid-core.schema.json examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- migrate examples/runscope/runscope.raid.json --from 0.1.0 --to 0.1.0
cargo run -p dslraid-cli -- project examples/runscope/runscope.raid.json --projection view:runtime
cargo run -p dslraid-cli -- render examples/runscope/runscope.raid.json --format svg
cargo run -p dslraid-cli -- codegen examples/runscope/runscope.raid.json --target rust
cargo run -p dslraid-cli -- export mermaid examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- diff base.json head.json
cargo run -p dslraid-cli -- query examples/runscope/runscope.raid.json 'kind=transition and tested=false'
cargo run -p dslraid-cli -- query examples/runscope/runscope.raid.json 'kind in [state,transition] and generated=false'
cargo run -p dslraid-cli -- compose examples/runscope/runscope.raid.json --materialize reachable --format json
cargo run -p dslraid-cli -- trace import examples/runscope/run-002.trace.jsonl --design-ir examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- trace check examples/runscope/run-001.trace.json --design-ir examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- coverage build --trace examples/runscope/run-001.trace.json --design-ir examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- coverage check examples/runscope/run-001.coverage.json --design-ir examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- artifact verify examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- compat check examples/runscope/runscope.raid.json
cargo run -p dslraid-cli -- quality
```

MVP implementations now exist for no-op version migration, richer IR queries,
lazy reachable composition materialization, runtime trace import/check, and
trace-derived coverage overlays. Planned but not yet implemented as full
product features: non-trivial migrations, synchronized product semantics,
coverage-aware viewer overlays, WASM packaging, and WebGL rendering.

## Viewer

```bash
cd apps/viewer
npm ci
npm run dev
npm run lint
npm run build
```

The viewer loads `public/examples/runscope.raid.json` and
`public/examples/run-001.coverage.json` when the Pages workflow renders example
assets, and falls back to an embedded RunScope sample during local development.
