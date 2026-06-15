# DSL

Surface syntax macros and expansion helpers.

Macros belong here, but they should only translate readable DSL forms into
ordinary data structures or builder calls. They should not perform IO,
validation, composition, or code generation.

Expected files:

- `syntax.lisp`
- `expand.lisp`

