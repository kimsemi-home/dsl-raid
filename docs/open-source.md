# Open Source Strategy

## Project Category

DSLRaid should be presented as an executable architecture browser, not as a
diagramming library and not as a generic knowledge graph.

Comparable categories:

- Structurizr and C4 tooling
- Backstage architecture and catalog experiences
- statechart tooling
- Enterprise Architect-style model browsers
- code generation frameworks

The differentiator is the combination of:

- typed executable IR
- executable SSOT
- FSM composition
- interactive exploration
- code generation
- source/test/generated artifact traceability

## Licensing

Recommended default:

- code: Apache-2.0
- docs/examples: CC-BY-4.0 if separate documentation terms are desired
- MIT only if maximum simplicity is preferred
- dual MIT/Apache-2.0 only if Rust ecosystem convention becomes more important

Avoid copyleft licenses unless the goal is to prevent proprietary embedding.
DSLRaid is likely to be adopted faster if companies can embed the viewer and
CLI in internal developer platforms.

Mermaid compatibility should be export-oriented. Do not position DSLRaid as a
Mermaid replacement, do not use Mermaid as product identity, and do not copy
Mermaid parser or renderer code without following its license notices.

## Governance

Start with a benevolent maintainer model:

- one project owner
- lightweight RFCs for IR changes
- maintainers per area after repeated contributions
- public roadmap, public issues, public design notes

Require RFCs for:

- IR schema changes
- new core object kinds
- transition semantics
- composition semantics
- projection semantics
- derivation and artifact semantics
- public CLI changes
- plugin API changes
- diagnostic severity policy changes
- visibility, provenance, and trust-boundary changes

Do not require RFCs for:

- bug fixes
- documentation improvements
- example additions
- internal refactors that preserve public behavior
- renderer improvements based on existing derived indexes

## Contribution Areas

Make contribution boundaries explicit:

- typed IR schema and fixtures
- Rust IR kernel
- Rust analyzer
- FSM composition engine
- Common Lisp authoring DSL
- derived graph indexer
- web viewer
- graph layout adapters
- code generators
- examples
- documentation
- editor/LSP support

This helps contributors join without needing to understand the whole stack.

## Compatibility Policy

The executable IR is the contract.

- The CLI may evolve quickly before `1.0`.
- The web UI may evolve quickly before `1.0`.
- The derived graph index may evolve quickly before `1.0`.
- The core IR should be conservative from the beginning.
- Breaking IR changes require migration notes.
- Fixtures must cover every public object kind and composition mode.

## Plugin Strategy

DSLRaid should be extensible, but plugins should come after the core model is
stable.

Plugin categories:

- importers
- code generators
- analyzers
- composition strategies
- renderers
- source resolvers
- artifact resolvers

Plugin API rule:

- plugins consume and produce typed IR, diagnostics, derivations, artifacts, or
  derived indexes
- plugins do not own the canonical model

Before public plugins, keep internal extension points explicit:

- `Emitter`
- `Importer`
- `AnalyzerPass`
- `ProjectionPass`
- `RendererTarget`

## Documentation Strategy

Docs should be example-driven.

Required examples:

- minimal FSM
- hierarchical FSM
- parallel FSM
- DDD aggregate with commands and events
- RunScope composite FSM
- source/test/generated artifact provenance
- codegen example
- provider/runtime/capability compatibility
- public/private projection and redaction
- runtime trace overlay
- composition conflict

Each example should include:

- source
- canonical typed IR
- expected diagnostics
- rendered screenshot later
- generated output later

## Initial Issue Labels

Suggested labels:

- `area:ir`
- `area:lisp`
- `area:rust`
- `area:web`
- `area:renderer`
- `area:index`
- `area:layout`
- `area:codegen`
- `area:docs`
- `area:examples`
- `kind:bug`
- `kind:feature`
- `kind:rfc`
- `good first issue`
- `help wanted`

## Release Strategy

Early releases should be honest and narrow.

### `0.1`

- core/assertion/index/view/trace/coverage/source-map/lock/annotation/validation
  schemas
- Rust IR crate
- fixture validation
- basic CLI validate command

### `0.2`

- FSM analyzer
- diagnostics
- Mermaid/Graphviz export

### `0.3`

- derived graph index
- viewer MVP
- object inspector
- source/test/generated artifact traceability

### `0.4`

- Common Lisp authoring DSL
- source path preservation

### `0.5`

- codegen targets
- golden tests

### `0.6`

- composition operators
- projected views
- IR diff

### `0.7`

- runtime trace import
- coverage overlays
- generated docs
- public/private projection
- PR review summaries

### `0.8`

- artifact freshness verification
- compatibility checks
- query command
- importer/exporter contracts
- review mode design

## Community Message

The short pitch:

> DSLRaid turns architecture DSLs into typed executable IR that can be analyzed,
> composed, explored, and used to generate code.

The contributor pitch:

> If you care about compilers, state machines, graph visualization, developer
> tools, or architecture-as-code, there is a clean boundary where you can help.
