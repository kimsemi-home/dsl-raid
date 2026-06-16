(in-package #:dslraid.lang)

(defparameter *language-pipeline*
  '(("Lisp forms" "Native authoring SSOT and executable ontology surface.")
    ("Reader" "Common Lisp reads forms without turning Lisp into plain text.")
    ("Macro expansion" "Surface syntax expands into ordinary authoring data.")
    ("Language AST" "Authoring forms keep source-level diagnostic context.")
    ("Language conformance" "Duplicate and unknown authoring forms fail early.")
    ("Canonical IR" "Stable interchange data consumed by Rust runtime tooling.")
    ("IR conformance" "Cross-language semantic contract and diagnostics.")
    ("Projection" "View models, docs, traces, and backend outputs derive here.")
    ("Backend output" "Rust, Go, TypeScript, Mermaid, DOT, SVG, and docs.")))

(defparameter *language-contracts*
  '("Lisp forms are the native source of truth."
    "Authoring order is preserved as deterministic semantic presentation order."
    "Macros must not hide IO, conformance, projection, or backend codegen."
    "Backends consume Canonical IR, not raw Lisp forms."
    "Rust source can be generated runtime output, not authoring truth."
    "Backend target catalog is generated from Lisp data and checked by backendgen."
    "Generated language docs are checked by lisp-docgen."
    "Generated Lisp Canonical IR is checked by lisp-irgen and Rust validation."
    "Generated Rust backend code is checked by lisp-rustgen."))

(defun language-pipeline-catalog ()
  "Return the native authoring pipeline contract."
  (copy-list *language-pipeline*))

(defun language-contract-catalog ()
  "Return language-layer contracts for generated documentation."
  (copy-list *language-contracts*))
