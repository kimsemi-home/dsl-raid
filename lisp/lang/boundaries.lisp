(in-package #:dslraid.lang)

(defparameter *language-boundaries*
  '(("Expansion"
     "Lisp forms / AST"
     "Canonical IR-shaped data"
     "Common Lisp"
     "No IO, conformance execution, projection, or backend codegen.")
    ("Conformance"
     "Language AST and Canonical IR"
     "diagnostics"
     "Common Lisp and Rust"
     "Authoring checks happen before IR emission; semantic checks run on IR.")
    ("Projection"
     "Canonical IR"
     "view models and backend artifacts"
     "Rust tooling"
     "Rust, Go, TypeScript, Mermaid, DOT, SVG, and docs are derived outputs.")))

(defun language-boundary-catalog ()
  "Return product-level language pipeline boundaries."
  (copy-list *language-boundaries*))
