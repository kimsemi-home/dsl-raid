# DSLRaid Common Lisp Layer

The Common Lisp layer is the language and executable ontology surface for
DSLRaid. Lisp forms are the authoring SSOT; canonical IR is the deterministic
interchange product emitted from those forms.

It must not hide conformance, composition, IO, or backend code generation
inside macro expansion. Macros should expand readable declarations into
ordinary data or data-construction calls.

See [../docs/lisp-dsl.md](../docs/lisp-dsl.md).

The executable authoring path is:

```text
Lisp forms -> language AST -> conformance -> expanded IR -> canonical JSON
```

## Smoke Test

```bash
sbcl --noinform --non-interactive \
  --eval '(require :asdf)' \
  --eval '(asdf:load-asd (merge-pathnames "lisp/dslraid.asd" (uiop:getcwd)))' \
  --eval '(asdf:load-system :dslraid)' \
  --load lisp/tests/golden.lisp \
  --eval '(dslraid::run-golden-smoke)'
```

The `fsm` macro expands to a `build-fsm` function call. `build-fsm` delegates
to `dslraid.lang` for AST parsing and expansion. Conformance and JSON emission
are explicit functions, not hidden macro-expansion side effects.
