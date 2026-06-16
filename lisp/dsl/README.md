# DSL Surface

Surface syntax macros and macro-facing builder entry points.

Macros belong here, but they should only translate readable DSL forms into
ordinary data structures or builder calls. They should not perform IO,
conformance, composition, projection, or backend code generation.

Expected files:

- `syntax.lisp`
- `expand.lisp`

AST parsing, language conformance, and AST-to-IR expansion belong to
`../lang/`.
