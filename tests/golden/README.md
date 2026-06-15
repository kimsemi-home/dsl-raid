# Golden Tests

Golden tests lock DSLRaid behavior across validation, composition, projection,
rendering, and diagnostics.

Expected layout:

```text
tests/golden/
  validate/
  assertion-registry/
  validation-report/
  compose/
  project/
  render/
  trace/
  diff/
  diagnostics/
  mutation/
  artifacts/
  compat/
```

Future commands:

```bash
cargo run -p dslraid-cli -- golden check tests/golden
cargo run -p dslraid-cli -- golden update tests/golden
```

Golden files should compare deterministic outputs only. Core IR, projected view
models, SVG exports, diagnostics, and generated artifact hashes should all be
stable under repeat runs.

Validation report golden files should compare proposition IDs and statuses, not
incidental wording.

Mutation fixtures should intentionally break FSMs:

- missing initial state
- unreachable state
- duplicate transition
- nondeterministic guard
- unknown event
- terminal outgoing transition
- cycle without exit
- composition explosion or bounded materialization
- private visibility leak
- generated artifact mismatch
- stale generated artifact lock mismatch
- missing provider capability

Error messages should be human-readable. Prefer:

```text
FSM001: transition idle_to_running points to unknown state "running".
Known states: idle, starting.
```

over:

```text
invalid transition
```
