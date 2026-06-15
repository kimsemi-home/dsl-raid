# Graph Pipeline

This layer converts Core IR into screen-facing data.

Expected files:

- `projection.ts`: choose what to show
- `view-model.ts`: build nodes, edges, labels, badges, inspector records
- `layout.ts`: call ELK/Dagre or cached layout
- `scene.ts`: convert view model to renderable scene shapes

The renderer should receive scene data and should not know FSM semantics.

