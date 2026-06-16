# Emit

Deterministic emitters for Canonical IR JSON and language-adjacent generated
metadata.

Expected files:

- `json.lisp`
- future backend adapters only when they consume Canonical IR

Never emit output directly from hash table iteration. Sort first.

Rust, Go, TypeScript, Mermaid, DOT, SVG, docs, and WASM outputs are backend
targets derived from Canonical IR. Rust source can be generated output.
