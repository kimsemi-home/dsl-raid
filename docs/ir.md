# DSLRaid IR Design

## Core Rule

DSLRaid core IR is a typed executable architecture IR, not an entity-relation
knowledge graph.

The project has ten core companion data shapes:

1. `dslraid-core.schema.json`: semantic executable IR
2. `dslraid-assertion.schema.json`: assertion registry for validation SSOT
3. `dslraid-index.schema.json`: derived index graph for search and cross refs
4. `dslraid-view.schema.json`: derived view model for layout and rendering
5. `dslraid-trace.schema.json`: runtime event log
6. `dslraid-coverage.schema.json`: design/runtime coverage overlay
7. `dslraid-sourcemap.schema.json`: DSL/IR/generated/runtime mapping
8. `dslraid-lock.schema.json`: resolved IDs, hashes, derivation, artifacts
9. `dslraid-annotation.schema.json`: notes, review comments, and links
10. `dslraid-validation.schema.json`: validation proposition/assertion report

Only the core schema is authoritative for design meaning. The index, view,
trace, coverage, source-map, lock, annotation, assertion, and validation schemas
are separate data products and must not be folded back into core IR.

```text
typed executable core IR + assertion registry
  -> validation / analyzer
  -> composition / projection / derivation
  -> runtime trace import
  -> coverage overlay
  -> source map
  -> lock / artifact freshness
  -> validation proposition/assertion report
  -> derived index graph
  -> derived view model
  -> annotations
  -> renderer
```

## Core Top-Level Shape

```json
{
  "ir_version": "0.1.0",
  "project": {
    "id": "myhome-jarvis",
    "name": "myhome-jarvis"
  },
  "contexts": [],
  "requirements": [],
  "capabilities": [],
  "policies": [],
  "commands": [],
  "fsms": [],
  "compositions": [],
  "projections": [],
  "derivations": [],
  "artifacts": [],
  "diagnostics": []
}
```

The core IR owns typed objects. It does not own generic `atoms`, `relations`,
runtime events, coverage counters, canvas nodes, layout coordinates, or UI
selection state.

## What Schema Enforces

The core JSON Schema enforces object families, required fields, ID namespaces,
composition input shape, projection source shape, derivation rule shape, and
artifact provenance fields.

Examples:

- FSM IDs must use the `fsm:` namespace.
- composition IDs must use the `composition:` namespace.
- product composition inputs must be FSM refs.
- projections must point at an FSM, composition, or context.
- derivations must declare a source, rule, and artifact targets.
- artifacts can declare the derivation that generated them.
- capabilities, policies, commands, and requirements have typed namespaces.

## What Validation Enforces

Some semantic rules require the core validator/analyzer, not plain JSON Schema.

Required validation:

- `transition.from` must refer to a state in the same FSM or region.
- `transition.to` must refer to a state in the same FSM or region.
- `transition.on` must refer to an event in the same FSM or inherited scope.
- `transition.requires` must resolve to policies, capabilities, constraints, or
  other semantic subjects accepted by the analyzer.
- an FSM must have exactly one initial state unless explicitly marked partial
  later.
- terminal states must not have outgoing transitions.
- guard references must resolve to guards in the same FSM.
- action references must resolve to actions in the same FSM.
- action/event type rules must be checked by analyzer.
- `composition.inputs` must resolve to existing FSM objects.
- product composition state tuples must be derived from a product composition.
- derivation targets must resolve to artifacts.
- generated artifacts must point back to the derivation that produced them.
- lock file artifact hashes must match the current canonical IR hash when
  artifact verification is requested.
- runtime traces must be imported into trace files, not embedded in core IR.
- coverage must be produced as an overlay, not embedded in core IR.
- annotations must reference semantic subjects without changing executable
  meaning.

The important split:

```text
schema = shape and namespace contract
validator/analyzer = executable semantic contract
```

See [Validation Proposition Catalog](validation.md) for the first `V001` through
`V050` validation propositions.

## ID Model

Core top-level objects use semantic IDs.

Examples:

- `context:runtime`
- `requirement:runtime_runs_to_completion`
- `capability:runtime_execution`
- `policy:runtime_failure_policy`
- `command:runtime_start`
- `fsm:runtime`
- `composition:runscope`
- `view:runscope.conflicts`
- `derivation:runtime.codegen`
- `artifact:runtime_fsm.rs`
- `diagnostic:runtime.unreachable.completed`

FSM-local objects use local IDs:

- state: `running`
- event: `start_requested`
- guard: `workspace_ready`
- action: `allocate_workspace`
- transition: `idle_to_starting`

Normalizers may derive full semantic subjects for diagnostics and view models:

- `state:runtime.running`
- `transition:runtime.idle_to_starting`

Semantic ID, layout ID, `defined_at` source range, diagnostic subject, and UI
selection state must remain separate.

Core objects may also carry:

- `uid`: optional stable internal UID for long-lived refactor tracking
- `defined_at`: source URI and range
- `visibility`: `public`, `internal`, `private`, or `secret`
- `tags`: searchable semantic labels

## FSM

An FSM is a first-class executable object.

```json
{
  "id": "fsm:runtime",
  "name": "RuntimeFSM",
  "states": [
    { "id": "idle", "kind": "atomic", "initial": true },
    { "id": "starting", "kind": "atomic" },
    { "id": "running", "kind": "atomic" },
    { "id": "completed", "kind": "atomic", "terminal": true }
  ],
  "events": [
    { "id": "start_requested", "kind": "external" }
  ],
  "guards": [],
  "actions": [],
  "transitions": [
    {
      "id": "idle_to_starting",
      "from": "idle",
      "to": "starting",
      "on": "start_requested",
      "guards": [],
      "actions": []
    }
  ]
}
```

The important point: `transition` is a typed executable object. It is not
decomposed into `transition_from` and `transition_to` relations in core IR.

## State

States are local to an FSM or region.

```json
{
  "id": "running",
  "kind": "atomic",
  "initial": false,
  "terminal": false
}
```

Initial state kinds:

- `atomic`
- `compound`
- `parallel`
- `history`

Composite and parallel states may own regions later. Their region transitions
are still typed transitions, not generic relation records.

## Event, Guard, Action

Events are typed inputs or emitted facts.

```json
{
  "id": "start_requested",
  "kind": "external"
}
```

Guards are typed predicates.

```json
{
  "id": "workspace_ready",
  "kind": "predicate"
}
```

Actions are typed effects.

```json
{
  "id": "allocate_workspace",
  "kind": "command",
  "command": "command:workspace.allocate"
}
```

The analyzer owns type compatibility checks between events, guards, actions,
commands, emitted events, and transition sites.

## Traceability and Visibility

Traceability is a core feature, not a viewer add-on.

```json
{
  "id": "running",
  "kind": "atomic",
  "defined_at": {
    "uri": "lisp/runtime/runscope.lisp",
    "range": {
      "start_line": 24,
      "end_line": 26
    }
  },
  "visibility": "internal",
  "tags": ["tested", "generated"]
}
```

A click path should be able to move through:

```text
state
  -> defined_at source
  -> derivation
  -> generated artifact
  -> test artifact
  -> related policy/capability later
  -> diagnostic subjects
```

Visibility exists because IR can contain source paths, artifact paths, policy
names, capability names, and sensitive metadata. Public exports must redact or
reject `private` and `secret` data according to export policy.

## Composition

Composition is a first-class object.

```json
{
  "id": "composition:runscope",
  "name": "RunScopeFSM",
  "kind": "product",
  "inputs": ["fsm:runtime", "fsm:agent", "fsm:workspace"],
  "state_space": {
    "kind": "lazy",
    "max_materialized_states": 5000
  },
  "conflict_policy": {
    "nondeterminism": "diagnostic",
    "unreachable": "hide_by_default"
  }
}
```

Initial composition kinds:

- `union`
- `product`
- `synchronized_product`
- `refinement`

Composition results are derived IR. `state_tuple` objects belong to derived
composition/projection/index output, not hand-written core FSM definitions.

## Projection

A projection is a semantic view over core IR or a composition result.

```json
{
  "id": "view:runscope.conflicts",
  "kind": "projection",
  "source": "composition:runscope",
  "show": ["states", "transitions", "conflicts", "coverage"],
  "filters": {
    "unreachable": false
  }
}
```

A projection is not a member list. It is a domain-aware query over typed IR.

## Derivation and Artifacts

Derivations connect executable IR to generated outputs.

```json
{
  "id": "derivation:runtime.codegen",
  "source": "fsm:runtime",
  "rule": {
    "id": "fsm_codegen",
    "kind": "codegen",
    "generator": "dslraid-codegen",
    "version": "0.1.0"
  },
  "targets": [
    { "artifact": "artifact:runtime_fsm.rs", "role": "generated" }
  ]
}
```

Artifacts can point back to the derivation that produced them.

```json
{
  "id": "artifact:runtime_fsm.rs",
  "kind": "generated",
  "path": "generated/runtime_fsm.rs",
  "generated_by": "derivation:runtime.codegen"
}
```

This makes generated code/test/docs provenance explicit in core IR.

## Diagnostics

Diagnostics are first-class structured objects.

```json
{
  "id": "diagnostic:runtime.nondeterministic.running",
  "severity": "error",
  "code": "FSM001",
  "message": "Two transitions handle the same event from running.",
  "subjects": [
    "transition:runtime.t1",
    "transition:runtime.t2",
    "state:runtime.running"
  ],
  "suggestion": "Add mutually exclusive guards or merge transitions."
}
```

The viewer can use `subjects` to highlight the full cause chain. Diagnostics
must not be plain log strings.

## Index Graph

The index graph is derived from core IR.

Schema: [dslraid-index.schema.json](../schemas/dslraid-index.schema.json)

It may contain:

- atoms
- relations
- search records
- cross references

This is where `transition_from`, `transition_to`, `contains`, and
`generates`-style relations are allowed. They are rendering/search/navigation
indexes, not source-of-truth semantics.

## View Model

The view model is derived from a projection and optional layout.

Schema: [dslraid-view.schema.json](../schemas/dslraid-view.schema.json)

It may contain:

- canvas nodes
- routed edges
- layout coordinates
- badges
- labels
- inspector panel data

It must not contain core meaning changes. It also should not own UI selection
state; selection belongs to the viewer app shell.

## Storage Formats

Recommended files:

- `.dslraid.json`: canonical core IR
- `.dslraid.lock.json`: resolved IDs, hashes, derivation records
- `.dslraid.view.json`: optional cached layout/view data
- `.dslraid.annotations.json`: user notes, review notes, links, explanations

The canonical core IR must use stable ordering, stable semantic IDs, stable
hash inputs, and deterministic JSON serialization.

## Minimal RunScope Example

See [examples/runscope/runscope.raid.json](../examples/runscope/runscope.raid.json)
for the committed core fixture.
