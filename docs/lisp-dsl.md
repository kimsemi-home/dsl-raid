# Common Lisp DSL Guide

Common Lisp is a good fit for DSLRaid's SSOT, DSL, and compiler layer. It
should not become the hidden runtime where validation, composition, IO, and
code generation happen implicitly.

## Core Principles

1. Keep data IR separate from DSL syntax.
2. Use macros only for surface syntax.
3. Implement semantic validation with ordinary functions.
4. Do not hide code generation inside macro expansion.
5. Generate every derived output deterministically.

Short rule:

```text
Lisp macros make declarations pleasant.
Typed IR and ordinary compiler passes own meaning.
```

## Recommended Structure

```text
lisp/
  packages.lisp
  ir/
    model.lisp
    ids.lisp
    validation.lisp
  dsl/
    syntax.lisp
    expand.lisp
  passes/
    normalize.lisp
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
- run validation
- mutate global registries with DSL results
- execute external commands
- perform complex composition
- perform code generation

## Data-First Macro Pattern

Prefer macros that quote the surface form and pass it to a function.

```lisp
(defmacro fsm (name &body forms)
  `(build-fsm ',name ',forms))

(defun build-fsm (name forms)
  ;; parse, normalize, and validate through ordinary functions
  )
```

This keeps the real behavior testable without macroexpansion gymnastics.

## Normalize Pass

DSL syntax can be convenient. Core IR must be strict.

```text
DSL
  idle -> starting
  (:transition ...)
  (:state ...)

normalized IR
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
- reject or diagnose ambiguous DSL constructs

## Compiler Pass Pipeline

The compiler should be a normal function pipeline.

```lisp
(defun compile-project (project)
  (attach-diagnostics
   (build-projections
    (compose-fsms
     (validate-project
      (resolve-refs
       (normalize-project project)))))))
```

A threading macro is fine if it stays cosmetic. The passes should remain normal
functions that are individually testable.

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
- codegen hidden in macro expansion
- validation hidden in macro expansion

## DSLRaid Language Split

Common Lisp should own:

- SSOT DSL authoring
- IR normalization
- deterministic emitters
- code generation
- golden test helpers
- policy/doc generation

Rust/WASM should own:

- browser runtime support
- large graph analysis
- composition performance hot paths when needed
- viewer bridge APIs
- Canvas/WebGL-adjacent runtime data

The boundary is the typed core IR.

