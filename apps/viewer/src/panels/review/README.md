# Review Panel

Review mode compares two projections or two canonical IR snapshots.

It should show:

- added, removed, and changed subjects
- diagnostics delta
- coverage delta
- artifact freshness delta
- compatibility delta
- public/private projection delta

The panel should drive selection by semantic subject IDs. It should not compute
semantic diffs itself; it consumes `dslraid diff` output or WASM diff results.
