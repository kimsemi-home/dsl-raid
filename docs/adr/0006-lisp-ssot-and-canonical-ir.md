# ADR 0006: Lisp SSOT and Canonical IR

## Status

Accepted.

## Context

DSLRaid uses Rust for durable tooling, analysis, code generation, and runtime
interfaces. It also uses Common Lisp for expressive authoring. If the project
centers a generic pass framework too early, Lisp becomes a thin syntax layer
and DSLRaid loses the main reason to use Lisp: macro expansion over executable
authoring forms.

The authoring source of truth is not a pass registry. For native DSLRaid
projects, the authoring source of truth is the Lisp form.

## Decision

DSLRaid's native authoring pipeline is:

```text
Common Lisp form
  -> reader
  -> macro expansion
  -> expanded authoring data
  -> Canonical IR
  -> IR conformance
  -> projection
  -> backend output
```

The stable interchange product is Canonical IR. Rust tools consume Canonical IR.
Rust source code may also be a generated backend artifact from Canonical IR.

The key product concepts are:

- Expansion: Lisp forms become deterministic IR-shaped data.
- Conformance: expanded data and Canonical IR satisfy stable rules.
- Projection: Canonical IR is transformed into view models, docs, code, traces,
  and other outputs.

`Pass` may exist as an internal implementation detail. It is not the product
center and should not be stabilized ahead of the Lisp authoring contract.

## Boundaries

Lisp owns:

- authoring forms
- macro expansion
- authoring-time diagnostics
- canonical IR emission
- deterministic language-level golden tests

Rust owns:

- Canonical IR structs and schema validation
- IR conformance reports
- CLI workflows
- projection, render, export, and codegen backends
- runtime/WASM/viewer-adjacent tooling

Backends must consume Canonical IR, not raw Lisp forms. This keeps Rust, Go,
TypeScript, Mermaid, DOT, SVG, docs, and future WASM outputs derived from one
interchange model.

## Consequences

- The Lisp DSL is not merely parser syntax; it is the executable ontology
  authoring layer.
- IR schema stability remains critical because every backend depends on it.
- Language conformance can catch authoring errors before IR emission.
- IR conformance remains the cross-language contract.
- Rust code generation is a backend, not the primary source of truth.
- Public plugin/pass APIs should wait until Expansion, Conformance, and
  Projection contracts are more stable.
