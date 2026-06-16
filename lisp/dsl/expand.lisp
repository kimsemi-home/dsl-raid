(in-package #:dslraid.dsl)

(defun build-fsm (name forms)
  "Convert human-friendly DSL forms into a plain DSLRaid IR FSM object.

The macro-facing function remains small: form parsing and expansion are owned
by the language layer, while conformance, IO, projection, and backend codegen
stay explicit outside macro expansion."
  (let* ((ast (dslraid.lang:parse-fsm-form name forms))
         (diagnostics (dslraid.lang:validate-fsm-ast ast)))
    (when diagnostics
      (error "DSLRaid language conformance failed: ~S" diagnostics))
    (dslraid.lang:expand-fsm-ast ast)))
