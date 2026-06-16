# Conformance

Conformance checks verify that expanded authoring data can become valid
Canonical IR.

Language conformance runs before JSON emission and can point at Lisp authoring
forms. Rust-side IR conformance runs after emission and validates the canonical
interchange format.

Both layers should use stable diagnostic codes where possible.
