# WASM Bindings

Bindings to `dslraid-wasm` belong here.

WASM may provide validation, projection, composition, trace import, coverage,
and diff operations. The app shell should call those APIs instead of
reimplementing Core IR semantics in TypeScript.

WASM should not depend on selected subjects, hovered subjects, panel expansion,
viewport coordinates, or other viewer-only state.
