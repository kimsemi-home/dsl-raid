# IR Conformance

IR conformance checks verify that expanded authoring data can become valid
Canonical IR.

Language conformance lives under `../lang/` and runs before IR expansion.
Rust-side IR conformance runs after emission and validates the canonical
interchange format.

Both layers should use stable diagnostic codes where possible.
