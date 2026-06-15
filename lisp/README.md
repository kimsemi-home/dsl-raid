# DSLRaid Common Lisp Layer

The Common Lisp layer is for SSOT authoring, DSL expansion, normalization, and
deterministic emitters.

It must not hide validation, composition, IO, or code generation inside macro
expansion. Macros should expand readable declarations into ordinary data or
data-construction calls.

See [../docs/lisp-dsl.md](../docs/lisp-dsl.md).

## Smoke Test

```bash
sbcl --noinform --non-interactive \
  --eval '(require :asdf)' \
  --eval '(asdf:load-asd (merge-pathnames "lisp/dslraid.asd" (uiop:getcwd)))' \
  --eval '(asdf:load-system :dslraid)' \
  --load lisp/tests/golden.lisp \
  --eval '(dslraid::run-golden-smoke)'
```

The `fsm` macro expands to a `build-fsm` function call. Validation and JSON
emission are explicit functions, not hidden macro-expansion side effects.
