# DSLRaid Architecture

## Product Definition

DSLRaid is a composable DSL runtime and interactive architecture explorer. The
product category is closer to an executable architecture browser than a diagram
tool.

The core workflow is:

```text
SSOT / DSL sources
  -> typed executable IR
  -> analyzer / composer / matcher
  -> source maps / runtime traces / coverage overlays
  -> lock records / artifact freshness / compatibility checks
  -> codegen / docs / tests / derived graph index
  -> interactive renderer
```

The executable IR is the source of truth. The graph index is generated from it.

## Design Goals

- Make architecture executable and inspectable.
- Treat contexts, requirements, policies, capabilities, commands, FSMs, states,
  transitions, events, compositions, projections, derivations, and artifacts as
  typed objects.
- Preserve traceability from executable IR to source files, tests, docs, and
  generated code.
- Detect stale generated artifacts through lock-file hashes.
- Model provider, runtime, protocol, capability, constraint, and policy
  compatibility without overloading FSM structure.
- Support finite state machine composition as a first-class capability.
- Keep the core usable without a server or hosted SaaS.
- Let open source contributors work on independent layers without needing to
  understand the entire system.
- Make the IR stable enough that external tools can build against it.

## Non-Goals

- A diagram-only syntax renderer.
- A Mermaid replacement.
- full UML, ERD, sequence diagram, or arbitrary graph authoring in the MVP.
- A generic knowledge graph as the core model.
- A custom graph database.
- UI state stored in core IR.
- layout coordinates stored in core IR.
- runtime traces or coverage counters stored in core IR.
- annotations stored in core IR.
- round-trip editing from ViewModel back to DSL in the MVP.
- A cloud-first Backstage replacement.
- A custom graph layout engine before the executable IR is proven.
- A Common Lisp-only or Rust-only project. Each language has a clear boundary.

## Boundary Contract

DSLRaid should be treated as an IR compiler, analyzer, projection engine, and
viewer. These layers must stay separate from the beginning.

```text
core meaning layer
  != analysis layer
  != layout layer
  != render/view layer
  != app shell
```

Layer responsibilities:

- Core IR: pure executable data. No `x`, `y`, `color`, `expanded`, or
  `selected`.
- Analyzer: reads IR and produces diagnostics, projections, composition
  results, and validation output. It must not know Canvas, SVG, or WebGL.
- Layout: takes projected render graph/view data and computes geometry.
- ViewModel: screen-ready objects such as node boxes, edge routes, labels, and
  badges.
- Renderer: draws Canvas/SVG/WebGL. It must not know FSM semantics.
- Viewer app shell: file loading, search, inspector panels, shortcuts, UX state,
  and UI selection.

These five concepts must not be mixed:

1. semantic ID
2. layout ID
3. source location
4. diagnostic subject
5. UI selection state

Bad core IR:

```json
{
  "id": "state:runtime.running",
  "x": 120,
  "y": 80,
  "selected": true,
  "color": "red"
}
```

Good separation:

```json
{
  "id": "state:runtime.running",
  "kind": "state",
  "name": "running"
}
```

```json
{
  "subject": "state:runtime.running",
  "x": 120,
  "y": 80,
  "width": 160,
  "height": 48,
  "badges": ["tested", "generated"]
}
```

## Layered Architecture

### 1. Authoring Layer

Inputs may come from:

- Common Lisp DSL files
- canonical JSON IR files
- future YAML/TOML authoring formats
- code annotations
- generated build metadata

Mermaid, DOT, PlantUML, or Structurizr import can be considered later. Early
Mermaid/Graphviz support should be export-only.

Common Lisp is the best initial authoring layer because it supports expressive
macros, executable SSOT, and direct integration with existing Lisp-based domain
models.

### 2. Core IR Layer

The IR is the product kernel. It represents executable architecture as nested
typed objects.

Core object families:

- project
- context
- requirement
- capability
- provider later
- runtime later
- protocol later
- constraint later
- policy
- command
- FSM
- state
- event
- transition
- guard
- action
- composition
- projection
- derivation
- artifact
- diagnostic

Required IR properties:

- stable semantic IDs for top-level objects
- stable local IDs inside FSMs and regions
- deterministic serialization
- source path, derivation, and artifact traceability
- lock-file artifact freshness
- versioned schema
- explicit composition semantics
- lossless round trip for native DSLRaid sources

The IR should not flatten transitions into generic relation records. A
transition owns its `from`, `to`, `on`, guard, action, and emitted event fields.

### 3. Analysis Layer

The analyzer validates executable IR and computes diagnostics.

FSM analysis:

- missing initial state
- multiple initial states
- unreachable states
- dead states
- terminal state validity
- transition references unknown state
- transition references unknown event
- ambiguous transition handling
- event/action consistency

Architecture analysis:

- derivation target conflict
- source path missing or unresolved
- generated artifact target duplicated
- unsupported codegen target
- projection references unknown root
- broken requirement/FSM/policy/capability trace
- public projection includes private or secret data
- generated artifact is stale relative to lock file input hash
- provider/runtime/protocol compatibility is unsatisfied

Validation boundaries:

- JSON Schema checks syntax, shape, enum values, and ID namespaces.
- Analyzer checks executable semantics.
- Validation proposition reports identify which stable rules passed, failed,
  warned, or did not apply.
- Golden tests lock behavior.

### 4. Composition and Projection Layer

Composition is the strategic differentiator. DSLRaid should support multiple
FSMs and architecture models that can be combined, matched, projected, and
inspected.

Composition must default to lazy, materialized-on-demand state spaces. The
viewer should project reachable, conflicting, covered, or artifact-relevant
subsets instead of eagerly showing the full Cartesian product.

Core operators:

- `union`: combine compatible typed IR objects
- `product`: create state tuples across FSMs
- `synchronized_product`: compose FSMs with event synchronization
- `project`: create a semantic projection over typed IR
- `match`: check whether a runtime trace conforms to an FSM later
- `diff`: compare two IR snapshots

Example:

```text
RunScope
  owns RuntimeFSM
  owns AgentFSM
  owns WorkspaceFSM

compose RunScopeExecution
  mode product
  inputs RuntimeFSM AgentFSM WorkspaceFSM
```

### 5. Derivation and Codegen Layer

Derivations connect executable IR to generated outputs.

Initial targets:

- Rust FSM skeletons
- Go FSM skeletons
- TypeScript definitions
- Markdown architecture docs
- golden tests
- Mermaid and Graphviz exports

Codegen rules:

- deterministic output
- golden tests for generated files
- generated files include source IR IDs
- generated files are not the canonical source

### 6. Derived Graph Index Layer

The graph model is useful as an index, not as core IR.

The indexer can convert typed IR into:

- index atoms
- index relations
- source lookup records
- derivation and artifact lookup records
- search records
- hit-test handles
- layout hints

This index is disposable and reproducible. It may be emitted for debugging or
cache use, but the first public workflow should go through `dslraid project`.

```bash
dslraid project examples/runscope/runscope.raid.json --projection view:runtime
```

### 7. Layout Layer

Layout is an adapter.

Initial approach:

- use ELK for hierarchical architecture and FSM layouts
- use Graphviz DOT for compatibility and quick exports
- store layout output as cached layout data, never as canonical truth

Later:

- incremental layout for large graphs
- domain-specific FSM layout constraints
- stable layout under small edits

### 8. View Model Layer

The view model is the boundary between meaning and pixels.

It may contain:

- `subject`: semantic ID of the IR object being represented
- node boxes
- edge routes
- labels
- badges
- collapsed/expanded render groups
- diagnostic decorations
- source/derivation/artifact decorations

It must not redefine FSM semantics. If meaning changes, the core IR or analyzer
must change first.

The first viewer should follow:

```text
Core IR -> ViewModel -> Canvas
```

See [Viewer Rendering Guide](viewer-rendering.md) for Canvas coordinates,
layers, hit testing, projection UX, and inspector rules.

When implementation code exists, use the [Refactoring Guide](refactoring.md) as
the audit sequence for keeping Core IR, analyzer/projection, ViewModel/layout,
and renderer/UI separated.

### 9. Rendering Layer

The renderer presents executable IR through generated projections and indexes.

Expected UI regions:

- model tree
- graph viewport
- selected object inspector
- composition/projection selector
- source/test/generated target panel
- analysis diagnostics panel

Rendering strategy:

- Canvas first for MVP simplicity
- WebGL for large graph edges, highlights, and massive viewports
- DOM or Canvas labels depending on scale
- spatial index for hit testing
- explicit selection model independent from rendering

### 10. Viewer App Shell

The viewer app owns product UX:

- file loading
- search
- inspector panels
- shortcuts
- local UI state
- selection state
- projection switching
- command palette later

The app shell consumes view models and diagnostics. It should not perform FSM
analysis itself.

### 11. Language Server Layer

The language server comes after the IR stabilizes.

Capabilities:

- completion for states, events, commands, and capabilities
- diagnostics from the analyzer
- go to definition
- find references
- rename state/event/command
- preview selected projection

## Language Boundaries

### Common Lisp

Owns:

- expressive DSL authoring
- SSOT macro layer
- data-first DSL expansion
- IR normalization and deterministic emitters
- existing Lisp project integration
- optional REPL-driven model inspection

Does not own:

- browser runtime
- large graph rendering
- canonical analyzer implementation long term
- hidden validation, composition, IO, or codegen inside macro expansion

See [Common Lisp DSL Guide](lisp-dsl.md) for macro rules, pass structure, and
deterministic emitter guidance.

### Rust

Owns:

- canonical IR data model
- schema validation
- FSM analysis and composition
- deterministic serialization
- code generation
- derived graph index generation
- WASM exports

### WASM

Owns:

- browser-side analysis
- composition
- validation
- index generation
- diffing

### JavaScript / TypeScript

Owns:

- web app shell
- editor/viewer state
- plugin API
- renderer orchestration
- browser integration

### Canvas / WebGL

Owns:

- pan and zoom
- drawing generated render nodes and edges
- hit testing support
- selection and hover rendering
- large graph performance

## Proposed Repository Structure

```text
.
├── crates/
│   ├── dslraid-core/
│   │   ├── ir/
│   │   ├── ids/
│   │   ├── schema/
│   │   └── validation/
│   ├── dslraid-analyzer/
│   │   ├── reachability/
│   │   ├── determinism/
│   │   ├── composition/
│   │   ├── projection/
│   │   └── diagnostics/
│   ├── dslraid-layout/
│   │   ├── graph/
│   │   ├── elk_adapter/
│   │   └── cached_layout/
│   ├── dslraid-render/
│   │   ├── view_model/
│   │   ├── canvas_model/
│   │   └── svg_export/
│   └── dslraid-wasm/
│       └── bindings/
├── apps/
│   └── viewer/
│       └── src/
│           ├── app/
│           ├── store/
│           ├── graph/
│           ├── canvas/
│           ├── panels/
│           └── wasm/
├── cli/
│   └── dslraid/
│       ├── validate
│       ├── compose
│       ├── project
│       └── render
├── docs/
├── examples/
│   ├── runscope/
│   └── ddd-order-system/
├── schemas/
│   ├── dslraid-core.schema.json
│   ├── dslraid-assertion.schema.json
│   ├── dslraid-index.schema.json
│   ├── dslraid-view.schema.json
│   ├── dslraid-trace.schema.json
│   ├── dslraid-coverage.schema.json
│   ├── dslraid-sourcemap.schema.json
│   ├── dslraid-lock.schema.json
│   ├── dslraid-annotation.schema.json
│   └── dslraid-validation.schema.json
├── lisp/
│   ├── packages.lisp
│   ├── ir/
│   ├── dsl/
│   ├── passes/
│   ├── emit/
│   └── tests/
├── tests/
│   ├── fixtures/
│   └── golden/
│       ├── validate/
│       ├── compose/
│       ├── project/
│       ├── render/
│       └── diagnostics/
└── tools/
```

## Public CLI Shape

```bash
dslraid init
dslraid normalize examples/runscope/runscope.raid.json
dslraid validate examples/runscope/runscope.raid.json
dslraid migrate --from 0.1.0 --to 0.2.0 examples/runscope/runscope.raid.json
dslraid compose examples/runscope/runscope.raid.json --composition composition:runscope
dslraid project examples/runscope/runscope.raid.json --projection view:runtime
dslraid trace import logs/run-123.jsonl
dslraid diff base.json head.json
dslraid render examples/runscope/runscope.raid.json --format svg
dslraid artifact verify
dslraid compat check
dslraid query 'kind=transition and tested=false'
dslraid serve examples/runscope/runscope.raid.json
dslraid codegen examples/runscope/runscope.raid.json --target rust
```

The first product contract is the CLI pipeline:

```text
dslraid init
dslraid normalize
dslraid validate
dslraid migrate
dslraid compose
dslraid project
dslraid trace import
dslraid diff
dslraid render
dslraid artifact verify
dslraid compat check
dslraid query
dslraid import
dslraid export
```

The viewer should consume this pipeline rather than inventing parallel logic.

## Test Strategy

Golden tests are the main refactoring safety net.

```text
IR input
  -> expected diagnostics
  -> expected projection
  -> expected view model
  -> expected generated artifact hash
```

Golden directories:

- `tests/golden/validate/`
- `tests/golden/compose/`
- `tests/golden/project/`
- `tests/golden/render/`
- `tests/golden/diagnostics/`
- `tests/golden/artifacts/`
- `tests/golden/compat/`

## MVP Slice

The first useful open source release should do only this:

1. Read canonical typed core JSON IR.
2. Validate FSM structure, derivations, and artifacts.
3. Generate a derived graph index.
4. Render an interactive browser with click-to-inspect.
5. Show source/test/generated artifact provenance.
6. Export Mermaid or Graphviz.
7. Include one excellent `RunScope` example.

This proves the executable architecture browser concept without overbuilding
the compiler front end.
