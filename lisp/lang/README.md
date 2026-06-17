# Language Layer

`dslraid.lang` is the boundary between readable Lisp forms and Canonical IR.

The stages are explicit:

```text
Lisp forms
  -> AST
  -> language conformance
  -> expanded IR objects
  -> canonical JSON emission
```

The product-facing concepts are Expansion, Conformance, and Projection. Passes
may exist internally, but they are not the language surface.

`dsl/` should stay small and macro-facing. `lang/` owns testable parsing,
authoring-time diagnostics, and AST-to-IR expansion.
