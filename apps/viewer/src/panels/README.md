# Panels

Panels are the IDE part of the viewer.

Panels include:

- inspector
- search
- diagnostics
- minimap
- timeline
- review

Panels consume selected subjects, diagnostics, traces, and source/artifact
links. They should not mutate Core IR semantics.

Panels are also the main accessibility surface for the Canvas graph. Anything
drawn on Canvas should be discoverable through inspector, search, diagnostics,
timeline, or review lists.
