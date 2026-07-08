# dslraid_viewer

Flutter shadcn Graph IDE shell for DSLRaid projections.

This is a parallel pilot, not a replacement for `apps/viewer`. It keeps the
same UI contract:

```text
Core IR -> Projection -> ViewModel -> Layout -> Scene -> Interaction -> Panels
```

The first slice renders a source/projection panel, a CustomPainter graph
viewport, an inspector panel, and diagnostics from
`assets/view_model_sample.json`. That asset follows
`schemas/dslraid-view.schema.json` and is loaded through
`lib/view_model_loader.dart`, keeping the shell ready for a generated projection
fixture later.

The project panel also derives compact status signals for the loaded contract,
source hash, layout engine, review load, coverage tags, codegen freshness, and
trace linkage. These signals stay app-shell state until generated projections
need stable schema-level status ids or workflow ownership.

Shared semantic graph tokens live in `lib/graph_tokens.dart` and are documented
in `../../docs/graph-ide-tokens.md`. Meta Astryx token references live in
`lib/astryx_tokens.dart`; they map the public Astryx neutral theme into Flutter
constants used by the shadcn theme and graph tone resolver.

## Commands

The local and CI baseline is Flutter `3.44.2` on the stable channel.

```sh
flutter analyze
flutter test
flutter build web
```

The GitHub Pages deployment serves this pilot at
`https://kimsemi-home.github.io/dsl-raid/flutter/`, so the deployment build uses:

```sh
flutter build web --base-href /dsl-raid/flutter/
```

From the repository root, validate the fixture contract with:

```sh
cargo run -p dslraid-cli -- schema validate schemas/dslraid-view.schema.json apps/viewer_flutter/assets/view_model_sample.json
```
