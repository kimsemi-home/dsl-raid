# Common Lisp DSL Guide

Common Lisp is DSLRaid's native language layer. Lisp forms are the authoring
SSOT for native DSLRaid projects; Canonical IR is the deterministic interchange
product emitted from those forms.

Common Lisp should not become a hidden runtime where conformance, composition,
IO, and backend code generation happen implicitly.

## Core Principles

1. Treat Lisp forms as native authoring SSOT.
2. Keep DSL syntax separate from Canonical IR.
3. Use macros for surface syntax and expansion only.
4. Implement conformance checks with ordinary functions.
5. Do not hide backend code generation inside macro expansion.
6. Generate every derived output deterministically from Canonical IR.

Short rule:

```text
Lisp forms own native authoring meaning.
Expansion emits Canonical IR.
Conformance and Projection derive everything else.
```

## Recommended Structure

```text
lisp/
  packages.lisp
  ir/
    model.lisp
    ids.lisp
  dsl/
    syntax.lisp
    expand.lisp
  expansion/
    normalize.lisp
  conformance/
    validation.lisp
  projection/
    compose.lisp
    project.lisp
    diagnostics.lisp
  emit/
    json.lisp
    go.lisp
    rust.lisp
    markdown.lisp
  tests/
    golden.lisp
```

## Macro Policy

Good macro use:

```lisp
(defdsl-fsm runtime
  (:state idle :initial t)
  (:state starting)
  (:state running)
  (:state completed :terminal t)
  (:event start-requested)
  (:transition idle->starting
    :from idle
    :to starting
    :on start-requested))
```

Macro responsibility:

```text
readable DSL -> ordinary data structure
```

Macro expansion should produce data construction, for example:

```lisp
(make-fsm
 :id "fsm:runtime"
 :states (...)
 :transitions (...))
```

Macros must not:

- write files
- run conformance checks
- mutate global registries with DSL results
- execute external commands
- perform complex composition
- perform backend code generation

## Data-First Macro Pattern

Prefer macros that quote the surface form and pass it to a function.

```lisp
(defmacro fsm (name &body forms)
  `(build-fsm ',name ',forms))

(defun build-fsm (name forms)
  ;; parse the authoring form into ordinary data
  )
```

This keeps the real behavior testable without macroexpansion gymnastics.

## Expansion

DSL syntax can be convenient. Canonical IR must be strict.

```text
Lisp form
  idle -> starting
  (:transition ...)
  (:state ...)

expanded authoring data / Canonical IR
  stable IDs
  explicit fields
  sorted order
```

Normalization responsibilities:

- derive stable semantic IDs
- expand shorthand syntax
- make implicit fields explicit
- sort lists deterministically
- preserve `defined_at` source information when available
- preserve enough structure for language conformance diagnostics

Expansion must be ordinary, testable Lisp functions. Macro expansion can call
builders, but it should not hide IO, conformance, projection, or backend
generation.

## Conformance and Projection

The language pipeline should make each product boundary explicit.

```lisp
(defun compile-project (forms)
  (let* ((expanded (expand-project forms))
         (canonical (emit-canonical-ir expanded))
         (diagnostics (check-conformance expanded canonical)))
    (values canonical diagnostics)))
```

Conformance should exist at two levels:

- language conformance before Canonical IR emission
- IR conformance after Canonical IR emission

Projection should consume Canonical IR and produce specific outputs. Backend
targets such as Rust, Go, TypeScript, Mermaid, DOT, SVG, docs, and future WASM
must not consume raw Lisp forms directly.

## Emitter Registry

Use an explicit registry for output targets.

```lisp
(defvar *emitters* (make-hash-table :test 'equal))

(defun register-emitter (name fn)
  (setf (gethash name *emitters*) fn))

(register-emitter :go #'emit-go)
(register-emitter :rust #'emit-rust)
(register-emitter :json #'emit-json)
```

Registration should happen intentionally during system load, not as a side
effect of user DSL forms.

## Declarative Codegen

Prefer template or AST-style emitters over ad hoc string concatenation.

```lisp
(emit-go-fsm
 :name "RuntimeFSM"
 :states '("Idle" "Starting" "Running" "Completed"))
```

All emitters must use:

- stable ordering
- fixed newlines
- fixed indentation
- fixed field order
- deterministic hashes when hashes are produced

Never emit by iterating directly over a hash table. Sort first.

## Pitfalls to Avoid

- IO inside macros
- excessive `gensym` use that makes debugging impossible
- hidden global mutation
- read-time eval abuse
- using package symbols directly as stable IDs
- hash table iteration as output order
- backend codegen hidden in macro expansion
- conformance hidden in macro expansion

## DSLRaid Language Split

Common Lisp should own:

- SSOT DSL authoring
- macro expansion
- language conformance
- Canonical IR emission
- golden test helpers
- policy/doc authoring helpers

Rust/WASM should own:

- Canonical IR structs and schema validation
- IR conformance
- projection and backend orchestration
- Rust/Go/TypeScript/Mermaid/DOT/SVG codegen targets
- browser runtime support
- large graph analysis
- composition performance hot paths when needed
- viewer bridge APIs
- Canvas/WebGL-adjacent runtime data

The boundary is Canonical IR. Rust source can be a generated artifact, not the
authoring source of truth.
