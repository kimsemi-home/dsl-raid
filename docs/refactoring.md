# Refactoring Guide

This guide defines the first-pass refactoring audit order once DSLRaid has
implementation code. It complements the ADRs: renderer and UI internals can move
freely, while Core IR semantics, composition rules, diagnostic codes, stable ID
rules, visibility rules, and codegen contracts require an ADR.

## Audit Order

Start with the layer boundaries before optimizing details:

```text
Core IR
  -> Analyzer / Projection
  -> ViewModel / Layout
  -> Renderer / UI
```

If these four layers are clean, Canvas, WASM, CLI, GitHub Actions, and codegen
can evolve independently.

### 1. Separate IR, View, Layout, and UI State

Core IR is semantic truth. It must not contain UI or layout state.

Allowed split:

```text
Core IR       = semantic executable model
ViewModel     = screen-facing nodes, edges, badges, labels, inspector rows
LayoutCache   = coordinates, edge routes, layout engine metadata
UI Store      = selected, hovered, filters, viewport
```

Refactor immediately if Core IR contains `x`, `y`, `color`, `selected`,
`expanded`, `collapsed`, `hover`, viewport, or panel state.

### 2. Remove Atom and Relation Overuse

The core should not become a generic knowledge graph.

Bad core shape:

```text
atom transition
relation transition_from
relation transition_to
```

Good core shape:

```text
fsms[]
  states[]
  events[]
  transitions[]
    from
    to
    on
    guards[]
    actions[]
```

`atoms` and `relations` belong in the derived index layer for search, cross
references, debugging, and navigation. FSMs, transitions, compositions,
projections, derivations, artifacts, diagnostics, policies, capabilities, and
commands remain typed Core IR objects.

### 3. Separate Analyzer and Renderer

The analyzer knows FSM and architecture semantics. The renderer knows only scene
shapes.

Analyzer API shape:

```text
FSM / Core IR -> diagnostics / projection / composition result
```

Renderer API shape:

```text
SceneNode / SceneEdge -> Canvas, SVG, or WebGL draw
```

Move logic out of the renderer if you see branches like:

```text
if node.kind == "terminal_state" then ...
if transition.guard exists then ...
```

The ViewModel should translate those semantics into style intents, badges,
labels, and inspector data before rendering.

### 4. Make Composition Lazy

Full product composition is a scale risk.

Avoid:

```text
compose() materializes every product state and transition
```

Prefer separate execution paths:

```text
compose()
composeReachable()
composeFocused(subject, depth)
composeDiagnosticsOnly()
```

Large views should materialize reachable subsets, selected neighborhoods,
conflicts, coverage overlays, or artifact-relevant paths. Full materialization
should be explicit and bounded.

### 5. Promote Diagnostics to First-Class Objects

String logs cannot power clickable architecture diagnostics.

Diagnostic records should contain:

```json
{
  "code": "FSM001",
  "severity": "error",
  "subjects": ["transition:t1", "state:runtime.running"],
  "message": "Transition t1 points to an unknown state.",
  "suggestion": "Add the state or fix the transition target."
}
```

Diagnostics must be deterministic, addressable, and linked to semantic
subjects so the viewer can highlight every cause.

### 6. Stabilize IDs

Check for fragile ID rules:

- file rename changes an object ID
- display name changes an object ID
- list reorder creates noisy output diffs
- generated IDs depend on hash-map iteration

Preferred model:

```json
{
  "id": "state:runtime.running",
  "uid": "01JZK000000000000000000000",
  "name": "running"
}
```

Use semantic IDs for readable references and optional stable UIDs for long-lived
rename and move tracking.

### 7. Add Source Maps Early

Source maps are hard to retrofit. Every meaningful core object should carry
definition provenance when possible:

```json
{
  "defined_at": {
    "uri": "lisp/runtime/runscope.lisp",
    "range": {
      "start_line": 10,
      "end_line": 18
    }
  }
}
```

Generated artifacts need reverse mappings from generated ranges to source
subjects, plus runtime event IDs when traces are imported.

### 8. Split Viewer Stores

A single giant UI store will make hover and selection changes expensive.

Recommended stores:

```text
irStore
projectionStore
layoutStore
selectionStore
viewportStore
diagnosticsStore
traceStore
```

Hover should not cause Core IR or the entire app shell to re-render.

### 9. Split Canvas Drawing Layers

Refactor a monolithic draw function into stable pieces:

```text
drawGrid
drawEdges
drawNodes
drawLabels
drawBadges
drawSelection
drawDiagnostics
```

When needed, split physical layers:

```text
staticCanvas
overlayCanvas
htmlOverlay
```

The logical split should exist before the physical split.

### 10. Separate Hit Testing from Rendering

Do not perform click detection inside draw routines.

Preferred flow:

```text
Scene -> HitIndex
Mouse -> world coordinates -> HitIndex query
```

Initial hit testing can use bounding boxes and segment distance checks. Later
implementations can use a spatial hash, quadtree, or rbush-style index.

### 11. Keep Layout Cache Out of Core IR

Layout is derived data.

```text
.dslraid.json       core executable IR
.dslraid.view.json  layout and view cache
```

Layout cache keys should include:

```text
ir_hash + projection_id + layout_engine_version
```

### 12. Enforce Deterministic Output

Golden tests depend on stable output.

Audit for:

- hash-map iteration in output generation
- nondeterministic diagnostic ordering
- unstable state or transition ordering
- unstable JSON field ordering
- unstable SVG IDs or element order
- unstable generated artifact hashes

Sort and canonicalize before emitting any public or golden output.

### 13. Keep CLI as an I/O Boundary

The CLI should call stable library APIs and handle files, flags, exit codes, and
human-readable output.

Good boundary:

```text
validate(input) -> ValidationReport
compose(input, options) -> CompositionResult
project(input, options) -> ProjectionResult
render(view_model, options) -> RenderResult
```

Avoid embedding validation, projection, rendering, or composition meaning in CLI
argument handlers.

### 14. Expand Golden Fixtures Beyond Happy Paths

Normal examples are not enough. Add fixtures for:

- missing initial state
- unknown transition target
- terminal state with outgoing transition
- unreachable state
- duplicate event transition
- nondeterministic guard
- composition explosion or bounded materialization
- private visibility leak
- generated artifact mismatch
- stale generated artifact lock mismatch
- provider capability mismatch

Golden outputs should compare diagnostics, projections, view models, SVG, diff
summaries, trace imports, and generated artifact hashes.

### 15. Test Public and Private Projection

Open source projects need safe sharing.

`dslraid project --visibility public` must remove or redact:

- private local paths
- secret-bearing metadata
- private policies
- private runtime traces
- token-like capability names or artifact contents

CI should include a fixture that fails when public projection leaks private or
secret data.

### 16. Keep WASM Boundaries Clean

WASM should expose semantic engine operations:

```text
validate
compose
project
layout-prep
diff
trace import
coverage overlay
```

JavaScript owns:

```text
file loading
stores
canvas rendering
interaction
panels
keyboard shortcuts
```

Do not let WASM depend on viewer selection, hover, panel state, or viewport
state.

### 17. Keep Package Names Aligned with Layers

Use package names that make architectural drift obvious:

```text
dslraid-core
dslraid-analyzer
dslraid-projection
dslraid-layout
dslraid-render
dslraid-cli
dslraid-wasm
dslraid-viewer
```

Package boundaries should mirror the data boundaries, not temporary
implementation convenience.

### 18. Classify ADR-Required Changes Before Refactoring

Do not fold these into ordinary cleanup:

- IR schema changes
- composition semantics
- transition, guard, or action semantics
- diagnostic code changes
- diagnostic severity policy
- stable ID policy
- visibility policy
- provenance and trust-boundary policy
- lock file, artifact freshness, or compatibility contracts
- codegen output contract

These need an ADR or accepted design issue before implementation.

## Refactoring Rule

Refactor freely inside renderer, viewer UX, layout caching, hit testing,
inspector composition, examples, and internal helper functions. Treat Core IR,
composition, validation meaning, visibility, diagnostic codes, diagnostic
severity, provenance, stable IDs, source maps, lock/compat contracts, and
codegen contracts as public design surfaces.
