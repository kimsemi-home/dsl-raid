# ViewModel UI Contract

This is the agent-readable UI snapshot contract shared by the TypeScript Canvas
viewer and the Flutter `shadcn_ui` pilot. Renderer code may decide layout,
camera, selection, and interaction state, but it must not invent or mutate Core
IR meaning.

Authoritative schema: [`schemas/dslraid-view.schema.json`](../schemas/dslraid-view.schema.json).

## Boundary

```text
Core IR -> Projection -> ViewModel -> Layout -> Scene -> Interaction -> Panels
```

The ViewModel is screen-facing data. It can cache layout and rendering metadata,
but the Core IR remains the only source of design meaning.

## Required Shape

| Field | Role | Consumers |
| --- | --- | --- |
| `view_version` | Version of the projected UI contract. | Fixture checks, migration gates. |
| `source.core_ir` | Repo-relative Core IR input path or source id. | Project/source panel, trace links. |
| `source.projection` | Semantic projection id such as `view:runtime`. | Toolbar, project panel, test fixtures. |
| `source.hash` | Optional source or projection hash. | Freshness and cache checks. |
| `layout.engine` | Layout source: `elk`, `graphviz`, `manual`, `cached`, or `none`. | Viewer badges and layout debugging. |
| `nodes[]` | Graph scene nodes with layout id, semantic subject, bounds, label, badges, and style. | Graph viewport, project list, search, minimap. |
| `edges[]` | Routed graph scene edges with layout id, semantic subject, endpoints, route, optional label, and style. | Graph viewport, path review, trace display. |
| `inspector_panels[]` | Subject-keyed panel sections and rows. | Inspector, review surface, agent summaries. |

## Style Tokens

`style.tone` is currently the only semantic visual state in the schema:

- `default`: normal projected view state.
- `success`: verified, covered, generated, deployed, or otherwise healthy.
- `warning`: stale, incomplete, uncovered, flaky, or needs review.
- `danger`: failed, blocked, unsafe, or release-risk state.
- `muted`: out-of-scope, filtered, disabled, or less relevant state.

`style.emphasis` is reserved for density and contrast. Current renderers may
ignore it until there is a concrete selection/focus/accessibility need.

## Diagnostics

Current decision: diagnostics are not first-class in
`schemas/dslraid-view.schema.json`. The Flutter pilot derives diagnostic rows
from `warning` and `danger` node/edge tones so the sample fixture stays strict
against the current schema.

Migration candidate: add a top-level `diagnostics[]` array only when generated
projection output needs stable diagnostic ids, severity, source locations, or
review workflow state that cannot be represented by node/edge style tone.

## Surface Mapping

| Surface | ViewModel source | Notes |
| --- | --- | --- |
| Graph viewport | `nodes[]`, `edges[]`, `style.tone`, `badges`. | Draw only layout/render state. Never write Core IR. |
| Inspector | `inspector_panels[]`. | Rows are already display-ready and should stay subject-addressable. |
| Project/source panel | `source`, `layout`, `nodes[].subject`. | Shows where the projection came from. |
| Search | `nodes[].label`, `nodes[].subject`, `edges[].label`, `inspector_panels`. | Search index can be local viewer state. |
| Minimap | `nodes[].bounds`, `edges[].route`, `style.tone`. | Minimap should not require a different schema. |
| Timeline/trace | `subject` fields plus future trace/coverage overlays. | Runtime overlays should reference semantic subjects. |
| Diagnostics | Derived from `warning`/`danger` tones for now. | First-class diagnostics require a schema migration. |
| Review/diff | `subject`, `source.hash`, style tone, badges. | Review comments belong to annotations, not this view cache. |

## Derived Status Signals

The Flutter pilot now exposes a renderer-local `ViewStatusSignal` list from the
schema-shaped ViewModel. This keeps the current schema strict while making the
agent-facing shell explicit about contract and freshness state:

| Signal | Derived from | Tone rule |
| --- | --- | --- |
| `Contract` | `view_version` | `success` when the loaded fixture is schema-shaped. |
| `Source` | `source.hash`, `source.projection` | `success` with a hash, `warning` when only projection metadata is present. |
| `Layout` | `layout.engine`, `layout.version` | `warning` only for `none`; otherwise renderer metadata. |
| `Review` | Derived diagnostics from warning/danger nodes and edges. | `danger` for blocked items, `warning` for review items, `success` when clear. |
| `Coverage` | Node badges such as `coverage`, `covered`, `tested`, `uncovered`. | `success` for coverage tags, `warning` for explicit gaps, `muted` when absent. |
| `Codegen` | Node labels and badges such as `generated` and `stale-check`. | `warning` for stale tags, `success` for generated/fresh tags, `muted` when absent. |
| `Trace` | Edge labels or subjects containing trace references. | `success` when linked, `warning` when linked trace edges are review/risk toned. |

These signals are app-shell state, not schema fields. Promote them into
`schemas/dslraid-view.schema.json` only if generated projections need stable
status ids, source locations, or workflow ownership.

## State Ownership

The ViewModel owns:

- projected scene geometry;
- subject references;
- display labels and badges;
- inspector content;
- style tone and emphasis;
- layout engine metadata.

The app shell owns:

- selected node or edge;
- hover/focus;
- camera pan and zoom;
- search query;
- open/closed panels;
- keyboard shortcuts;
- local interaction history.

## Validation

The Flutter sample fixture must stay schema-valid:

```sh
cargo run -p dslraid-cli -- schema validate schemas/dslraid-view.schema.json apps/viewer_flutter/assets/view_model_sample.json
```

The Flutter pilot must continue proving the shell can consume the contract:

```sh
cd apps/viewer_flutter
flutter analyze
flutter test
flutter build web
```
