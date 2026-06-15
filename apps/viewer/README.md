# DSLRaid Viewer

The viewer is a Graph IDE shell over DSLRaid projections.

Pipeline:

```text
Core IR -> Projection -> ViewModel -> Layout -> Canvas Scene -> Interaction -> Panels
```

The viewer must not mutate Core IR semantics. Selection, hover, viewport,
layout, and rendering state belong to viewer stores and view models.

Review mode should eventually compare before/after projections, diagnostics
delta, coverage delta, and stale artifact delta. Accessibility should be
provided through HTML panels, not by relying on Canvas alone.

See [../../docs/viewer-architecture.md](../../docs/viewer-architecture.md) and
[../../docs/viewer-rendering.md](../../docs/viewer-rendering.md).
