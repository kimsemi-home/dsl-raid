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

Shared semantic graph tokens live in `lib/graph_tokens.dart` and are documented
in `../../docs/graph-ide-tokens.md`.

## Commands

```sh
flutter analyze
flutter test
flutter build web
```

From the repository root, validate the fixture contract with:

```sh
cargo run -p dslraid-cli -- schema validate schemas/dslraid-view.schema.json apps/viewer_flutter/assets/view_model_sample.json
```
