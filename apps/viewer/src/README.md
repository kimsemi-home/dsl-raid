# Viewer Source

This tree is intentionally split by responsibility:

- `app/`: shell and routes
- `store/`: state stores
- `graph/`: projection, view model, layout, scene conversion
- `canvas/`: rendering, layers, hit testing, camera, text measurement
- `panels/`: inspector, search, diagnostics, minimap, timeline, review
- `wasm/`: bindings to `dslraid-wasm`

Keep Core IR, ViewModel, layout, viewport, selection, hover, diagnostics, and
trace state separate.
