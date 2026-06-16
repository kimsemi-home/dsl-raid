# ADR 0002: Product Scope and Risk Boundaries

## Status

Accepted.

## Context

DSLRaid can fail by becoming too broad. The most dangerous paths are becoming a
Mermaid replacement, mixing core IR with index/view data, eagerly materializing
large FSM products, or treating JSON Schema as the semantic validator.

## Decision

DSLRaid is an executable architecture and FSM IR browser. It is not a general
diagramming tool.

The MVP is CLI-first:

- `dslraid validate`
- `dslraid compose`
- `dslraid project`
- `dslraid render`

The viewer is built on top of those contracts.

## Explicit Non-Goals for MVP

- Mermaid replacement
- full UML
- arbitrary graph editor
- sequence diagram tool
- ERD tool
- realtime collaboration
- custom layout engine
- plugin ecosystem
- full LSP

Mermaid/Graphviz export is allowed as output. Mermaid should not be used as a
product identity or copied into the implementation without following its
license.

## Composition Policy

FSM composition defaults to lazy, materialized-on-demand state spaces.

Allowed MVP composition behavior:

- product composition for a small number of FSMs
- reachable subset projection
- conflict-focused projection
- coverage-focused projection
- artifact trace projection

Avoid eagerly materializing the full state product by default.

## Validation Contract

The three validation layers are distinct:

- JSON Schema = syntax and shape contract
- Analyzer = semantic contract
- Validation propositions = stable rule catalog
- Assertions = executable validation rule SSOT
- Golden tests = behavior contract

JSON Schema should check required fields, primitive types, enum values, and ID
namespaces. It should not try to prove all FSM semantics.

Analyzer checks include:

- transition target state exists
- exactly one initial state
- terminal state has no outgoing transition
- composition input resolves to an FSM
- nondeterminism exists
- guard/action/event compatibility

The initial validation catalog is documented as `V001` through `V050` in
[../validation.md](../validation.md).

## DSL Rollout

Do not freeze a broad human DSL too early. The native DSLRaid authoring target
is Common Lisp forms that expand into Canonical IR.

Phases:

1. JSON core IR
2. Common Lisp form expansion that emits Canonical IR
3. language conformance over expanded authoring data
4. LSP/editor support

JSON Core IR is the initial bootstrap and interchange format. It is not the
long-term native authoring SSOT for Lisp-first projects.

## Security and Visibility

Core IR can contain source paths, artifact paths, policy names, capability
names, and other sensitive metadata. Every public export path must support
visibility and redaction.

Required policies:

- public IR excludes secret-bearing metadata
- private IR can keep local paths and richer metadata
- secret-bearing artifact content is not embedded in IR
- user annotations are stored separately from Core IR
- generated artifact freshness is checked against lock-file hashes
- paths are sanitizable
- metadata visibility is explicit

For projects such as `myhome-jarvis`, private raw data, tokens, financial data,
and secret-bearing artifacts must not be committed.

## Storage Formats

Canonical IR must be deterministic.

Recommended files:

- `.dslraid.json`: canonical core IR
- `.dslraid.lock.json`: resolved IDs, hashes, derivation records
- `.dslraid.view.json`: optional cached layout/view data
- `.dslraid.annotations.json`: user notes, review notes, links, explanations

Requirements:

- stable ordering
- stable semantic IDs
- optional stable internal UIDs
- stable hashes
- canonical JSON serialization

## License Direction

Recommended license:

- code: Apache-2.0
- docs/examples: CC-BY-4.0 if the project wants separate documentation terms

Apache-2.0 is preferred because it includes a patent grant and is friendly to
company adoption.
