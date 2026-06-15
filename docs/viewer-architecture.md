# Viewer Architecture

The DSLRaid viewer is a Graph IDE shell over projected IR. It is not a Canvas
drawing app. Canvas is only the central viewport; the product value lives in
projection, interaction, inspector, search, diagnostics, and runtime overlays.

## Pipeline

```text
Core IR
  -> Projection Engine
  -> ViewModel
  -> Layout Engine
  -> Canvas Scene
  -> Interaction Layer
  -> Inspector / Search / Timeline
```

Layer responsibilities:

- IR: semantic design data
- Projection: select what should be shown
- ViewModel: screen-facing nodes, edges, labels, badges, inspector records
- Layout: compute x/y coordinates and edge routes
- Canvas Scene: renderable shapes
- Renderer: draw shapes
- Interaction: click, drag, zoom, selection, focus
- Panels: inspector, search, diagnostics, minimap, runtime timeline
- Review mode: before/after, changed subjects, diagnostics delta, coverage
  delta, artifact freshness delta

Bad:

```text
IR object contains x/y, color, selected
```

Good:

```text
IR:
  state:runtime.running

ViewModel:
  subject: state:runtime.running
  x, y, width, height, badges

UI State:
  selected: state:runtime.running
  hovered: transition:runtime.idle_to_starting
```

## Recommended Structure

```text
apps/viewer/
  src/
    app/
      shell.ts
      routes.ts
    store/
      ir-store.ts
      view-store.ts
      selection-store.ts
      viewport-store.ts
      diagnostic-store.ts
      trace-store.ts
    graph/
      projection.ts
      view-model.ts
      layout.ts
      scene.ts
    canvas/
      renderer.ts
      layers.ts
      hit-test.ts
      camera.ts
      text.ts
    panels/
      inspector/
      search/
      diagnostics/
      minimap/
      timeline/
    wasm/
      dslraid-wasm.ts
```

## App Shell

Use React, Solid, or Svelte for the application shell and panels. The graph
viewport should be Canvas 2D for the MVP.

Recommended split:

- app shell and panels: React/Solid/Svelte
- central graph viewport: Canvas 2D
- validation/projection/composition hot paths: Rust/WASM
- initial layout: ELK or Dagre
- static docs/export: SVG

The Canvas viewport is not the accessibility surface. Every visible graph fact
should also be available through HTML panels such as inspector, search results,
diagnostics, and timeline.

## Canvas Choice

Start with Canvas 2D.

- MVP: Canvas 2D
- larger graphs: Canvas 2D plus spatial index
- very large graphs or animation-heavy views: WebGL
- static output: SVG export

Starting with WebGL is too expensive. Canvas 2D is realistic for thousands of
nodes when layout, hit testing, and rendering are separated.

## Logical Layers

Use logical layers even if the first implementation draws to one Canvas:

- background/grid
- edges
- edge labels
- nodes
- node labels
- badges
- selection
- hover
- diagnostics overlay

When performance requires it, split physical layers:

- `staticCanvas`: grid, edges, nodes
- `overlayCanvas`: hover, selection, tooltip
- `uiLayer`: HTML inspector, popups, command palette

## Camera

Zoom and pan belong in a camera model.

```ts
type Camera = {
  zoom: number;
  panX: number;
  panY: number;
};
```

Screen to world:

```ts
const worldX = (screenX - panX) / zoom;
const worldY = (screenY - panY) / zoom;
```

Render:

```ts
ctx.save();
ctx.translate(panX, panY);
ctx.scale(zoom, zoom);
drawWorld();
ctx.restore();
```

## Scene Model

Canvas should not render semantic IR directly. Convert the ViewModel to a scene.

```ts
type SceneNode = {
  id: string;
  subject: string;
  kind: "state" | "fsm" | "artifact" | "test";
  x: number;
  y: number;
  width: number;
  height: number;
  label: string;
  badges: string[];
};

type SceneEdge = {
  id: string;
  subject: string;
  from: string;
  to: string;
  points: Point[];
  label?: string;
};
```

The renderer should not need to know FSM semantics. It draws boxes, edges,
labels, and badges.

## Hit Testing

MVP hit testing:

- node: bounding box
- edge: segment distance
- label/badge: bounding box

Node hit:

```text
x <= worldX <= x + width
y <= worldY <= y + height
```

Edge hit:

```text
distance(mouse, segment) < threshold
```

For large graphs, use:

- spatial hash
- quadtree
- rbush

Do not scan every node on every mouse move once graphs are large.

## Panels

Canvas alone is not the product.

Left:

- project tree
- contexts
- FSMs
- views

Center:

- Canvas graph

Right:

- inspector
- selected object detail
- source/test/artifact trace

Bottom:

- diagnostics
- runtime trace timeline
- command log

## Interaction Model

Required:

- click: select
- double click: focus
- Shift click: multi-select
- Space drag: pan
- wheel: zoom
- Cmd+K: command/search
- F: focus selected
- Esc: clear selection

Useful:

- 1-hop / 2-hop focus
- show path to terminal
- show generated artifacts
- show untested transitions
- show runtime trace overlay
- review before/after mode

## Accessibility

Minimum expectations:

- keyboard navigation through selected subjects
- search result list for graph objects
- diagnostic list with linked subjects
- timeline list for trace events
- inspector text for the selected object
- ARIA labels on HTML panels and controls

Canvas should never be the only way to learn that a state, transition,
diagnostic, artifact, or coverage result exists.

## Stores

Recommended state separation:

- `irStore`: original core IR
- `projectionStore`: current view/filter
- `layoutStore`: coordinates and layout cache
- `viewportStore`: pan and zoom
- `selectionStore`: selected and hovered subjects
- `diagnosticStore`: diagnostics
- `traceStore`: runtime events and overlays

Zustand, Redux Toolkit, or a small direct store are all acceptable. The store
choice is less important than keeping IR, layout, viewport, selection, hover,
diagnostics, and traces separate.

## Performance Rules

- Do not calculate layout on every event.
- Do not calculate hit indexes during rendering.
- Use `requestAnimationFrame` for rendering.
- Redraw only overlay layers when hover changes.
- Cull nodes outside the viewport.
- Cache text measurement.
- Separate selection changes from IR changes.

## MVP

The realistic MVP:

1. Load JSON IR.
2. Generate projection.
3. Use Dagre or ELK for layout.
4. Render Canvas 2D.
5. Click node to open inspector.
6. Support zoom and pan.
7. Show diagnostics overlay.
8. Export SVG.
