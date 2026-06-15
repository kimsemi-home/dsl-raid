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

## Planned Stack

- Common Lisp for authoring DSLs and SSOT-friendly macros
- Rust for canonical IR, analysis, composition, code generation, and WASM
- WASM for running the core engine in the browser
- JavaScript/TypeScript for the web application shell and plugin surface
- Canvas/WebGL for scalable interactive visualization
- ELK or Graphviz for early layout, with a native incremental layout engine
  later only if needed

## Repository Map

This repository is currently in design-first bootstrap mode.

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

The first product contract is CLI-first:

```bash
dslraid init
dslraid normalize .dslraid.json
dslraid validate .dslraid.json
dslraid validate .dslraid.json --format json
dslraid migrate --from 0.1.0 --to 0.2.0 .dslraid.json
dslraid compose .dslraid.json --composition composition:runscope
dslraid project .dslraid.json --projection view:runscope.conflicts
dslraid trace import logs/run-123.jsonl
dslraid diff base.json head.json
dslraid render .dslraid.json --format svg
dslraid artifact verify
dslraid compat check
dslraid query 'kind=transition and tested=false'
dslraid export mermaid .dslraid.json
```
