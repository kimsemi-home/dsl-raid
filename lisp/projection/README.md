# Projection

Projection turns Canonical IR into a purpose-specific derived product.

The Lisp layer may provide authoring helpers for projection declarations, but
projection outputs should consume Canonical IR rather than raw DSL forms.

Examples:

- view projection declarations
- documentation projection declarations
- codegen derivation declarations
- policy-focused projection declarations

Backend targets such as Rust, Go, TypeScript, SVG, Mermaid, DOT, docs, and
future WASM are projections from Canonical IR.
