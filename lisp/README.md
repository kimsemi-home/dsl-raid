# DSLRaid Common Lisp Layer

The Common Lisp layer is for SSOT authoring, DSL expansion, normalization, and
deterministic emitters.

It must not hide validation, composition, IO, or code generation inside macro
expansion. Macros should expand readable declarations into ordinary data or
data-construction calls.

See [../docs/lisp-dsl.md](../docs/lisp-dsl.md).

