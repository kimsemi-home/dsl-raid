# Flutter Viewer Strategy

DSLRaid now has a parallel Flutter viewer shell at `apps/viewer_flutter`. The
existing TypeScript Canvas viewer remains the primary shipped web viewer while
the Flutter shell proves whether a shadcn-style Graph IDE can consume the same
projected ViewModel contract without changing Core IR semantics.

## Decision

Preferred direction: keep the TypeScript viewer as the production shell for now,
and grow the Flutter shell as a contract-compatible pilot.

Fallback: if Flutter CustomPainter cannot match the graph interaction and
performance needs, keep Flutter for read-only review/status surfaces and leave
the high-density graph IDE in the TypeScript Canvas viewer.

## Contract

The Flutter shell consumes the same screen-facing contract as the current
viewer:

```text
Core IR -> Projection -> ViewModel -> Layout -> Scene -> Interaction -> Panels
```

The boundary stays at `schemas/dslraid-view.schema.json`:

- `nodes` and `edges` drive the graph viewport.
- `inspector_panels` drive the right-side details.
- `source`, `layout`, style tone, and diagnostic metadata stay visible to
  agents.
- Core IR stays immutable from UI code.

The current Flutter pilot reads `apps/viewer_flutter/assets/view_model_sample.json`
through `lib/view_model_loader.dart`. The asset intentionally follows
`schemas/dslraid-view.schema.json` directly instead of embedding a Dart-only
fixture, so the next step can swap in a generated projection fixture without
changing the shell widgets.

See [ViewModel UI Contract](viewmodel-ui-contract.md) for the agent-readable
field and surface mapping, and [Graph IDE Tokens](graph-ide-tokens.md) for the
shared TypeScript/Flutter token baseline.

## First Slice

The first Flutter slice intentionally mirrors the current viewer's information
architecture rather than replacing it:

- left projection/source panel;
- central CustomPainter graph viewport;
- right inspector panel;
- bottom diagnostics panel;
- `shadcn_ui` app/theme/card/badge/button primitives;
- JSON ViewModel asset loading through `DslraidViewModel.fromJson`;
- widget test proving the shell, shadcn root, and graph viewport mount.

## Migration Risks

- Graph density: CustomPainter is fine for a pilot, but larger graphs need the
  existing spatial-index and hit-test lessons ported deliberately.
- Canvas parity: pan, zoom, hit-test, focus, and search should migrate one
  behavior at a time from `apps/viewer/src/interaction`.
- Contract drift: Flutter models must stay generated from or checked against
  `schemas/dslraid-view.schema.json` before this becomes more than a pilot.
- Platform scope: web and macOS are enough for the architecture IDE shell until
  there is a mobile-specific product reason.

## Test Strategy

Local Flutter checks:

```sh
cd apps/viewer_flutter
flutter analyze
flutter test
flutter build web
```

Schema fixture check:

```sh
cargo run -p dslraid-cli -- schema validate schemas/dslraid-view.schema.json apps/viewer_flutter/assets/view_model_sample.json
```

Keep the existing viewer checks unchanged:

```sh
npm --prefix apps/viewer test
npm --prefix apps/viewer run build
```

Add Flutter checks to CI only after the shell either promotes
`assets/view_model_sample.json` as a stable ViewModel fixture or replaces it
with a generated fixture from `examples/runscope`.
