# Traceability and Runtime Evidence

DSLRaid should optimize for verifiable traceability, not just visualization.

The value chain is:

```text
requirement
  -> FSM
  -> policy / capability / command
  -> generated code
  -> test
  -> deployed artifact
  -> runtime event
  -> coverage overlay
```

The viewer should eventually compare these on one screen:

- designed transition
- tested transition
- generated artifact
- deployed artifact
- observed runtime transition

## Runtime Trace Import

Runtime traces are separate from design IR.

```bash
dslraid trace import logs/run-123.jsonl
```

Trace import should produce data matching
[dslraid-trace.schema.json](../schemas/dslraid-trace.schema.json).

Useful overlays:

- edge executed
- state reached
- transition failure rate
- transition never observed
- deployed artifact version

## Runtime Event Mapping

Trace overlay requires a mapping from provider/runtime event names to semantic
IR subjects.

```text
runtime_event.kind = "thread.completed"
  -> transition:runtime.running_to_completed
```

Mappings may come from source maps, provider profiles, trace import profiles, or
explicit configuration. The imported trace stays separate from Core IR.

## Coverage Overlay

Coverage overlays compare design IR to runtime traces.

Schema: [dslraid-coverage.schema.json](../schemas/dslraid-coverage.schema.json)

Examples:

- designed transition was observed
- designed transition was never observed
- transition has high failure rate
- generated artifact was deployed
- terminal failure state was reached

## Source Map

Source maps connect authoring, generated code, and runtime evidence.

Schema: [dslraid-sourcemap.schema.json](../schemas/dslraid-sourcemap.schema.json)

Mapping chain:

```text
DSL range
  -> IR object ID
  -> generated file range
  -> runtime event ID
```

This is what makes DSLRaid feel like an architecture IDE instead of a static
diagram.

## Public Projection

Open source usage needs public/private projection from day one.

```bash
dslraid project .dslraid.json --visibility public
```

Public projection must remove or redact:

- secret-bearing metadata
- local-only paths
- token names
- financial or personal data
- internal-only policies

## Diff

PR review is a primary use case.

```bash
dslraid diff base.json head.json
```

Good diff output should answer:

```text
FSM changed:
+ 2 states
+ 3 transitions
- 1 policy edge
warning: 1 untested transition
warning: 1 stale generated artifact
warning: provider capability changed
```

Diff should be deterministic and usable both in CLI output and PR comments.
The current CLI supports text, JSON, and Markdown output, including state and
transition additions/removals/changes, untested added transitions, terminal path
changes, and transition policy requirement changes.

Review mode should compare before/after projections, diagnostics delta,
coverage delta, artifact freshness delta, and compatibility delta.

## Artifact Freshness

Generated code, tests, docs, and exports should be verified against the lock
file.

```bash
dslraid artifact verify
```

If an artifact's recorded input hash differs from the current canonical IR hash,
the artifact is stale.
The current CLI infers the sibling lock file by default, validates it against
the lock schema, and reports stale/missing artifact records in text or JSON.

## Generated Docs

Docs should be generated from IR.

```bash
dslraid doc generate .dslraid.json
```

Generated docs should include:

- FSM overview
- state table
- transition table
- diagnostics
- coverage
- artifact trace
