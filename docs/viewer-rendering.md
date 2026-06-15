# Viewer Rendering Guide

## Implementation Order

Do not start with a polished app. First open the pipeline:

```text
Core IR -> ViewModel -> Canvas
```

For the full Graph IDE shell structure, see
[Viewer Architecture](viewer-architecture.md).

Recommended build order:

1. Core IR JSON
2. `validate`
3. project view model
4. render Canvas
5. inspector
6. compose/union
7. diagnostics overlay
8. export

## State Boundaries

Keep viewer state separate:

- `ir`: canonical core meaning
- `viewModel`: screen-ready nodes, edges, labels, badges, inspector data
- `layout`: node boxes and edge routes
- `viewport`: zoom and pan
- `selection`: selected semantic subject
- `hover`: hovered semantic subject

Never store layout coordinates or UI state in core IR.

Recommended files:

- `.dslraid.json`: meaning
- `.dslraid.view.json`: layout/cache

## Coordinate Spaces

Use world space for model geometry:

- node `x`, `y`, `width`, `height`
- edge points
- layout coordinates

Use screen space for interaction:

- zoom
- pan
- mouse position

Canvas render transform:

```js
ctx.save();
ctx.translate(panX, panY);
ctx.scale(zoom, zoom);
drawWorld();
ctx.restore();
```

Mouse to world transform:

```js
const worldX = (mouseX - panX) / zoom;
const worldY = (mouseY - panY) / zoom;
```

## Logical Layers

Use logical layers even if the first implementation draws to one Canvas:

- background
- grid
- edges
- nodes
- labels
- selection
- hover
- diagnostics

When performance requires it, split physical canvases:

- static canvas: grid and edges
- dynamic canvas: hover and selection

## Hit Testing

Start with bounding boxes.

Node hit:

```text
x <= mouseX <= x + width
y <= mouseY <= y + height
```

Edge hit can come later:

- draw a transparent thick path for interaction, or
- calculate point-to-segment distance

```text
distance(mouse, segment) < threshold
```

For large graphs, add a spatial index:

- quadtree
- rbush
- spatial hash

## Visual Grammar

Consistency matters more than decoration.

- FSM: large container
- state: rounded box
- terminal state: thick border
- transition: edge plus small label
- event: small pill
- guard: diamond or badge
- action: small tag
- diagnostic: warning badge
- artifact: document icon
- test: check icon
- source: code icon

## Inspector

The inspector is the main product value of the viewer.

State inspector:

- name
- parent FSM
- incoming transitions
- outgoing transitions
- `defined_at`
- tests
- generated artifacts
- diagnostics

Transition inspector:

- from
- to
- event
- guards
- actions
- source range
- related tests

## Projection UX

Large systems cannot be shown all at once. Projection controls should exist
from the first viewer.

Show toggles:

- unreachable
- generated artifacts
- tests
- source files
- diagnostics
- guards/actions
- only selected neighborhood

Focus mode:

- selected node 1-hop
- selected node 2-hop

Useful projections:

- happy path
- conflicts only
- unreachable only
- runtime x agent only
- edges without coverage
- path to a generated artifact

## Performance Rules

Initial targets:

- under 1,000 nodes: Canvas 2D is enough
- 1,000 to 10,000 nodes: Canvas plus spatial index
- 10,000+ nodes: consider WebGL

Do not recompute everything per frame.

- layout calculation is not rendering
- hit index calculation is not rendering
- rendering should run through `requestAnimationFrame`

## Autonomous Implementation Rule

Agents may freely improve UI/UX, rendering performance, canvas layering,
keyboard shortcuts, inspector layout, visual styling, and example diagrams.

Agents must not change Core IR semantics, composition rules, stable IDs,
diagnostic codes, visibility rules, or codegen contracts without an ADR.
