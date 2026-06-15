# Operational Product Contracts

DSLRaid should be designed as an operable open source product, not only as a
library or viewer. The CLI, file formats, CI behavior, examples, and governance
need stable contracts early.

## Product Boundary

Preferred positioning:

> DSLRaid is an executable architecture IR browser for FSM-heavy systems.

Do not describe DSLRaid as a Mermaid replacement. Mermaid, DOT, SVG, JSON, and
Markdown are export targets with explicit lossiness contracts.

## File Format Strategy

Use separate files for separate responsibilities:

```text
*.dslraid.json              canonical executable Core IR
*.dslraid.assertions.json   assertion registry for validation SSOT
*.dslraid.lock.json         resolved IDs, hashes, derivation, artifact records
*.dslraid.view.json         layout/cache/view data
*.dslraid.annotations.json  user notes, review notes, links, explanations
*.dslraid.trace.json        imported runtime events
*.dslraid.coverage.json     coverage overlay
*.dslraid.sourcemap.json    DSL/IR/generated/runtime mappings
*.dslraid.validation.json   validation proposition report
```

The lock file answers:

- which IR hash generated this code?
- which schema version produced this golden?
- which derivation generated this artifact?
- is this generated artifact stale?
- did the provider or protocol compatibility surface change?

## Stale Artifact Detection

Generated code, docs, tests, SVG, and exported graphs should carry enough
metadata to detect stale outputs.

```text
current IR hash != artifact input hash -> artifact is stale
```

Required command:

```bash
dslraid artifact verify
```

CI should eventually fail when generated artifacts are stale unless the job is a
pure documentation preview.

## Provider, Runtime, Protocol, Capability, and Constraint Model

FSM alone is not enough for agent runtimes and tooling integrations. DSLRaid
needs typed objects for:

- `provider`: Codex, Claude Code, OpenClaw, GitHub Actions, OpenAPI service
- `runtime`: local CLI runtime, hosted agent runtime, CI workflow runtime
- `protocol`: exec-jsonl, stream-json, hooks, workspace files, OpenAPI
- `capability`: reset, streaming, hooks, local workspace, tool execution
- `constraint`: network disabled, local workspace required, protocol version

Example:

```text
provider ClaudeCode
  supports stream-json
  supports hooks
  uses CLAUDE.md
  requires local workspace

provider Codex
  supports exec-jsonl
  supports reset
  uses AGENTS.md
```

These objects should connect to FSMs, transitions, commands, guards, actions,
policies, and compatibility checks.

## Policy as IR

Security and permission rules are executable architecture, not diagram notes.

Policies should be attachable to FSMs, states, transitions, commands,
capabilities, artifacts, derivations, and exports.

Example transition intent:

```json
{
  "id": "running_to_completed",
  "from": "running",
  "to": "completed",
  "requires": ["policy:no_secret_leak", "policy:artifact_signed"]
}
```

The analyzer owns enforcement. The renderer only receives badges, style tokens,
and diagnostics.

## Runtime Event Mapping

Runtime traces need mapping rules from external event names to semantic
subjects.

```text
runtime_event.kind = "thread.completed"
  maps_to transition:runtime.running_to_completed
```

This mapping enables trace overlays:

- transition executed
- state reached
- transition failed
- failure rate increased
- runtime violated design

Mappings may come from source maps, provider profiles, trace import profiles, or
explicit configuration.

## Importer Strategy

Importers should increase adoption without making imported formats canonical.

Early useful importers:

- `import mermaid`: documentation migration, lossy
- `import dot`: graph migration, lossy
- `trace import jsonl`: runtime overlay
- `import github-actions`: CI workflow FSM
- `import openapi`: capability and protocol surface
- `import linear`: requirement and issue references

Imported data should carry provenance and trust classification.

## Export Contract

Exports must declare whether they are lossy:

| Export | Purpose | Lossy |
| --- | --- | --- |
| Mermaid | documentation diagrams | yes |
| DOT | dense graph layout | yes |
| SVG | static sharing | yes |
| JSON | tooling interchange | no, when exporting canonical IR |
| Markdown | README/ADR/docs | yes |

Export commands should record format, generator, input hash, output hash, and
lossiness in the lock file.

## Semantic Diff

DSLRaid diff is a semantic review product, not a text diff.

It should report:

```text
+ state retrying
+ transition failed -> retrying
- transition running -> failed
warning: terminal path changed
warning: untested transition added
```

Diff should also compare diagnostics, coverage, derivations, artifact freshness,
capability compatibility, and visibility leaks.

## Review Mode

The viewer should eventually support PR review mode:

- before / after
- added / removed / changed
- diagnostics delta
- coverage delta
- stale artifact delta
- compatibility delta
- public/private projection delta

This mode is a likely killer UX for DSLRaid.

## Compatibility Gate

Compatibility checks validate IR, provider capability, protocol version, and
generated artifact version together.

Required command:

```bash
dslraid compat check
```

Example:

```text
Codex provider requires protocol >= 0.3
RuntimeFSM uses reset transition
Current provider has no reset capability
```

## Trust Boundary and Provenance

Every object should be classified by origin:

- trusted source
- generated source
- external imported source
- runtime trace
- user annotation

Example:

```json
{
  "provenance": {
    "kind": "generated",
    "generator": "dslraid 0.1.0",
    "input_hash": "sha256:..."
  }
}
```

Public projection must treat untrusted and secret-bearing data carefully.

## Annotation Layer

User-facing notes, review comments, links, and explanations should not become
Core IR semantics.

```text
Core IR      = executable meaning
Annotations  = notes, explanations, links, review context
```

Annotations are stored separately in `*.dslraid.annotations.json` and reference
semantic subjects.

## Query Language

Large systems need query, not only search.

Initial commands:

```bash
dslraid query 'kind=transition and tested=false'
dslraid query 'kind in [state,transition] and generated=false'
dslraid query 'kind=transition and requires~=policy:no_secret_leak or terminal=true'
```

Useful queries:

- find untested transitions
- find states without source
- find artifacts generated from policy X
- find runtime traces that violate design
- find private objects included in public projection

The first implementation is a small expression language over indexed IR items.
It supports `and`, `or`, `=`, `!=`, `~=`, `^=`, `$=`, numeric comparisons,
`in [...]`, `exists`, and `missing`. A dedicated DSL can come later.

## Diagnostic Severity Policy

Severity must be stable:

```text
error    = merge block
warning  = review needed
info     = contextual information
hint     = improvement suggestion
```

CI should support:

```bash
dslraid validate --deny warning
dslraid validate --allow info
```

Diagnostic code changes require an ADR.

## Fixture Projects

Open source adoption needs examples that feel useful:

```text
examples/
  simple-fsm/
  agent-runtime/
  ci-pipeline/
  policy-redaction/
  composition-conflict/
  runtime-trace-overlay/
```

Each fixture should include canonical IR, expected diagnostics, expected
projection, and later rendered/exported outputs.

## Benchmarks

Analyzer benchmarks matter before Canvas benchmarks.

Benchmark tiers:

- 100 states
- 1,000 states
- 10,000 states
- lazy composition
- focused projection
- diagnostics-only composition

CI should run light benchmarks. Nightly or scheduled workflows can run heavy
benchmarks.

## Fuzzing and Property Tests

IR parsing and validation should be fuzzed.

Targets:

- invalid JSON
- unknown refs
- cyclic hierarchy
- huge composition
- unusual Unicode in display text
- IDs near namespace boundaries
- deeply nested metadata

Rust options include property tests first and `cargo fuzz` later.

## Deterministic Layout Policy

Layout is hard to keep perfectly deterministic, so the policy must be explicit:

- default layout is deterministic
- interactive layout is user-controlled
- cached layout is explicit
- SVG golden tests pin layout engine, layout version, and seed

Never store layout coordinates in Core IR.

## Accessibility

Canvas must not be the only way to inspect the architecture.

Minimum requirements:

- keyboard navigation
- selected object text panel
- search result list
- diagnostic list
- timeline list
- ARIA on HTML panels

Every visible Canvas fact should also be reachable from an inspector, list, or
search result.

## Extension Points Before Plugins

Do not build a public plugin runtime before the core is stable. Define internal
extension points first:

- `Emitter`
- `Importer`
- `AnalyzerPass`
- `ProjectionPass`
- `RendererTarget`

External plugins can come after the IR, diagnostic, visibility, source-map, and
codegen contracts are stable.

## Governance Files

The public repository should keep these files current:

- `LICENSE`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- `docs/roadmap.md`
- `docs/adr/`

IR changes, diagnostic code changes, visibility policy changes, and codegen
contract changes go through ADRs.

## Stable CLI Names

Early command names should be stable:

```bash
dslraid init
dslraid normalize
dslraid validate
dslraid compose
dslraid project
dslraid render
dslraid diff
dslraid trace import
dslraid artifact verify
dslraid compat check
dslraid query
dslraid import
dslraid export
dslraid migrate
```

## First Build Order

The next practical build order remains:

1. Core IR v0.1
2. semantic validator
3. projection model
4. golden fixture suite

The viewer comes after these are real.
