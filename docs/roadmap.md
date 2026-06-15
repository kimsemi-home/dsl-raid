# DSLRaid Roadmap

## Milestone 0: Bootstrap

Goal: make the project understandable and contributable.

- publish positioning and architecture docs
- choose license
- create contribution guide
- define initial core, assertion, index, view, trace, coverage, source map,
  lock, annotation, and validation JSON schemas
- add one hand-written RunScope fixture
- add fixture validation in CI
- record MVP non-goals and risk boundaries in ADRs

Exit criteria:

- a new contributor can read the README and understand what DSLRaid is
- `runscope.raid.json` validates against the core schema
- the core/assertion/index/view/trace/coverage/source-map/lock/annotation/
  validation schemas and fixture are committed to the repository

## Milestone 1: IR Kernel

Goal: make the executable IR real.

- create `dslraid-core` Rust crate
- implement `Project`, `Context`, `Fsm`, `State`, `Event`, `Transition`,
  `Requirement`, `Capability`, `Policy`, `Command`, `Composition`,
  `Projection`, `Derivation`, and `Artifact`
- implement deterministic JSON serialization
- implement lock-file writer for resolved refs, hashes, derivations, artifacts,
  and golden records
- implement IR version reader and no-op migration scaffold
- implement core schema validation
- implement assertion registry reader
- add golden fixtures
- add diagnostics data model
- add validation proposition report model

Exit criteria:

- Rust can read, validate, and write the RunScope fixture without changing it
- `dslraid migrate --from 0.1.0 --to 0.1.0` is deterministic as a no-op
- `dslraid artifact verify` compares fixture artifact input hashes against the
  lock file
- `dslraid validate --format json` can emit a validation report with
  proposition IDs, assertion IDs, diagnostic codes, evidence, and suggestions

## Milestone 1.5: Operational Contracts

Goal: make DSLRaid usable in CI before the viewer exists.

- implement `dslraid normalize`
- implement `dslraid artifact verify`
- implement diagnostic severity gates such as `--deny warning`
- implement `dslraid compat check` scaffold
- define import/export lossiness records
- define annotation file loading without changing Core IR meaning
- add public/private projection leak fixture

Exit criteria:

- CI can fail on stale artifacts, warning-denied diagnostics, and public
  projection leaks

## Milestone 2: FSM Analyzer

Goal: prove DSLRaid is more than a renderer.

- create `dslraid-analyzer`
- implement missing initial state checks
- implement multiple initial state checks
- implement reachability
- implement dead state detection
- implement terminal state checks
- implement transition state/event reference checks
- implement human-readable error messages with known candidate lists
- implement nondeterminism diagnostics
- implement structured diagnostic subjects and suggestions
- support `hint`, `info`, `warning`, and `error` severity policy
- implement V001-V050 proposition registry incrementally
- implement assertion-to-diagnostic rendering for FSM assertions
- expose diagnostics through CLI

Exit criteria:

- `dslraid validate examples/runscope/runscope.raid.json` produces deterministic
  diagnostics

## Milestone 3: Projected View Model

Goal: project typed IR into a screen-ready model without turning the graph into
core IR.

- create `dslraid project`
- produce view model subjects, node boxes, edge routes, labels, and badges
- preserve references back to FSM/state/transition IDs
- include requirement, policy, capability, command, source map, and coverage
  references
- include derivation and artifact lookup records
- include projection-aware output

Exit criteria:

- `dslraid project examples/runscope/runscope.raid.json --projection view:runtime`
  emits deterministic view model JSON

## Milestone 4: Canvas Rendering MVP

Goal: render the projected view model before building a polished app shell.

- scaffold `apps/viewer/src` boundaries
- render Canvas from view model
- implement world/screen coordinate conversion
- add zoom and pan
- add node bounding-box hit testing
- add selection and hover layers
- add basic SVG export

Exit criteria:

- `dslraid render examples/runscope/runscope.raid.json --format svg` emits a
  deterministic SVG

## Milestone 5: Inspector MVP

Goal: make clicking objects valuable.

- create web app shell
- add project tree, graph viewport, inspector, diagnostics, and timeline regions
- load canonical JSON IR
- render project/context/FSM graph
- click states and transitions to inspect details
- show source/test/generated artifact provenance
- show incoming/outgoing transitions
- show diagnostics for selected subjects

Exit criteria:

- the RunScope example can be explored interactively in a browser

## Milestone 6: Lisp Authoring

Goal: support executable SSOT authoring.

- create Common Lisp DSL package
- keep data IR separate from DSL syntax
- keep macros limited to surface syntax
- implement normalize, resolve, validate, project, and diagnostics as ordinary
  functions
- add deterministic JSON emitter
- export canonical typed core JSON IR
- preserve source paths
- add RunScope Lisp source example
- validate exported IR with the Rust CLI

Exit criteria:

- a Lisp-defined FSM becomes the same IR consumed by the analyzer, indexer, and
  viewer

## Milestone 7: Composition and Union

Goal: make multi-FSM architecture exploration unique.

- implement union composition
- implement lazy reachable product FSM composition MVP
- implement synchronized product composition
- implement projection views
- avoid eager full Cartesian product materialization by default
- add RuntimeFSM + AgentFSM + WorkspaceFSM example

Exit criteria:

- the viewer can switch between individual FSMs and a composed execution view

## Milestone 8: Diagnostics Overlay

Goal: make diagnostics visible in projections and the viewer.

- render diagnostic badges
- highlight all diagnostic subjects
- show suggestions in inspector
- filter by severity
- add golden tests for diagnostic view models

Exit criteria:

- selecting a diagnostic highlights every related state/transition subject

## Milestone 9: Runtime Trace and Coverage

Goal: compare designed behavior to observed behavior.

- implement `dslraid trace import logs/run-123.jsonl`
- produce runtime trace JSON and validate it against the trace schema
- implement `dslraid trace check` for basic transition conformance
- produce trace-derived coverage overlay JSON
- implement `dslraid coverage build`
- implement `dslraid coverage check`
- show executed and unexecuted transitions in the viewer
- show transition failure rates
- connect runtime events through source maps
- map runtime event kinds to semantic transition and state subjects

Exit criteria:

- the RunScope trace example marks covered and uncovered transitions

## Milestone 10: Diff and PR Review

Goal: make DSLRaid useful in code review.

- implement `dslraid diff base.json head.json` MVP with text, JSON, and
  Markdown output
- summarize added/removed/changed states and transitions
- detect broken policy/capability traces
- detect untested transitions
- detect terminal path deltas
- detect stale artifact deltas
- detect compatibility deltas
- produce deterministic markdown suitable for PR comments

Exit criteria:

- a changed FSM fixture produces a stable review summary

## Milestone 11: Export and Generated Docs

Goal: make DSLRaid output useful outside the app.

- export SVG
- export Mermaid or Graphviz as derived output
- export JSON
- export Markdown traceability docs
- record lossy/lossless export contract
- implement `dslraid doc generate`
- preserve deterministic output hashes

Exit criteria:

- export golden tests lock SVG and generated artifact hashes

## Milestone 12: Codegen

Goal: prove that the same IR can generate implementation artifacts.

- generate Rust FSM skeletons
- generate Go FSM skeletons
- generate TypeScript type definitions
- generate Markdown architecture docs
- add golden tests

Exit criteria:

- generated files are deterministic and traceable to source IR IDs

## Milestone 13: Scale and Incrementality

Goal: make large systems practical.

- introduce stable layout caches
- add partial recomputation
- add spatial index for hit testing
- add progressive graph expansion
- add light analyzer benchmarks to CI
- add heavy scheduled benchmarks
- add parser/validator fuzzing or property tests
- benchmark 10,000 states and 50,000 transitions

Exit criteria:

- one changed state does not require recomputing every derived view

## MVP Scope Lock

Support:

- atomic FSM
- optional hierarchical FSM shape without full behavior
- one lazy product composition
- reachability
- nondeterminism diagnostics
- source/test/artifact traceability
- runtime trace import
- public/private projection
- stale artifact verification
- compatibility check scaffold
- static SVG or Canvas rendering

Do not support in MVP:

- full UML
- arbitrary graph authoring
- realtime collaboration
- custom layout engine
- plugin ecosystem
- full LSP

## Milestone 14: Language Server

Goal: make DSLRaid usable inside editors.

- implement diagnostics over open files
- completion for states, events, commands, and capabilities
- go to definition
- find references
- rename states, events, and transitions
- preview selected projection

Exit criteria:

- VS Code or another LSP client can edit a DSLRaid source with feedback from
  the analyzer
