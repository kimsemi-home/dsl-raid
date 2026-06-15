# ADR 0005: Observable Surface Area

## Status

Accepted.

## Context

DSLRaid will grow by adding typed IR, validation propositions, projections,
runtime traces, queries, codegen, viewer state, and public/private contracts.
The hard problem is not the total number of files. The hard problem is how many
files a human or agent must hold in mind at once.

Large files hide conceptual boundaries. Small files without a visible entry
point scatter the design. DSLRaid needs a folder shape that keeps both the
public workflow and the domain concepts easy to find.

## Decision

Optimize for observable surface area, not file count.

Default source files should stay below roughly 100 lines. A file may exceed
that only when splitting would make the reader open more files for one concept.

Workflow modules should use this shape:

```text
commands/<workflow>/
  mod.rs              # public entry points people call first
  model.rs            # local concepts and typed options
  parse.rs            # syntax to model
  evaluate.rs         # predicates, checks, execution rules
  value.rs            # small scalar helpers
  tests.rs            # workflow-level regression examples
  <domain>/
    mod.rs            # domain entry
    root.rs           # project/context/requirement-level concepts
    fsm.rs            # FSM-level entry
    fsm_states.rs     # state/event indexing
    fsm_behavior.rs   # guard/action/transition indexing
```

The top-level workflow entry should expose only the commands other modules need:

```text
run(...)
values(...)
item_map(...)
```

Internal files should match concepts a reader naturally searches for, such as
FSM states, transition behavior, derivations, diagnostics, policies, artifacts,
or runtime traces.

## Reader Layers

Structure folders by reader intent:

```text
public entry       command API, docs, examples
workflow model     local data concepts and options
syntax/parser      human expression to typed model
domain indexing    objects people inspect directly
evaluation         matching, validation, comparison rules
integration        quality gates, CLI routing, CI
```

`main.rs` should be a router, not a feature home. A developer changing query
matching should not need to read artifact freshness, coverage overlay, diff
rendering, or CLI argument wiring.

## Consequences

The repository will contain more files, but each task should start from one
obvious entry file and descend into one or two concept files.

This improves:

- code review focus
- LLM/agent context selection
- domain onboarding
- independent testing
- future split into crates or packages

It also means refactoring should move in vertical slices. First carve out a
workflow folder, then split it by reader concepts, then add tests at the local
boundary.
