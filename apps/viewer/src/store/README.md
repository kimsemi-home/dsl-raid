# Stores

Viewer state is split by meaning:

- `ir-store.ts`: original Core IR
- `view-store.ts`: projected view and filters
- `selection-store.ts`: selected and hovered subjects
- `viewport-store.ts`: pan and zoom
- `diagnostic-store.ts`: diagnostics
- `trace-store.ts`: runtime traces and coverage overlays

Do not store layout coordinates in Core IR. Do not store semantic IR changes in
selection or viewport state.

