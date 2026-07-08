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

The pilot is also published inside the GitHub Pages artifact at
`https://kimsemi-home.github.io/dsl-raid/flutter/`. The TypeScript Canvas
viewer remains the root Pages app; the Flutter build is a subpath deployment so
agents and reviewers can inspect the shadcn-style shell without changing the
production viewer entry point.

Astryx usage: Meta Astryx is currently React and StyleX based, so the Flutter
pilot uses it as an agent-readable token and component reference rather than a
runtime package. `apps/viewer_flutter/lib/astryx_tokens.dart` mirrors the
public Astryx neutral theme into Dart constants, and
`apps/viewer_flutter/lib/graph_tokens.dart` maps those constants into the
shadcn Flutter theme plus DSLRaid graph semantic tones.

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
- Astryx neutral token bridge feeding the shadcn theme and graph semantics;
- JSON ViewModel asset loading through `DslraidViewModel.fromJson`;
- derived status signals for contract, source, layout, review, coverage,
  codegen freshness, and trace linkage;
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
flutter build web --base-href /dsl-raid/flutter/
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

CI now analyzes, tests, and builds the Flutter pilot on pull requests and main.
The generated CI and Pages workflows pin Flutter `3.44.2` on the stable channel
so the hosted pilot stays tied to the same local latest-stable toolchain and
ViewModel fixture contract.

The current Pages workflow builds both web viewers. It runs the TypeScript
viewer build first, then builds the Flutter pilot with the Pages subpath
`base-href`, copies `apps/viewer_flutter/build/web` into
`apps/viewer/dist/flutter`, and uploads the combined `apps/viewer/dist`
artifact.
