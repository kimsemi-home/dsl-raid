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

`dsl/` should stay small and macro-facing. `lang/` owns testable parsing,
authoring-time diagnostics, and AST-to-IR expansion.
