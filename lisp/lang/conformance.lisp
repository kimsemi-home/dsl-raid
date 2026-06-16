(in-package #:dslraid.lang)

(defun validate-fsm-ast (ast)
  "Return language conformance diagnostics before Canonical IR expansion."
  (append
   (duplicate-form-diagnostics ast :state "state"
                               (language-code :duplicate-state))
   (duplicate-form-diagnostics ast :event "event"
                               (language-code :duplicate-event))
   (duplicate-form-diagnostics ast :transition "transition"
                               (language-code :duplicate-transition))
   (malformed-form-diagnostics ast)
   (unknown-form-diagnostics ast)
   (primary-id-diagnostics ast)
   (keyword-argument-diagnostics ast)
   (transition-required-diagnostics ast)
   (transition-reference-diagnostics ast)))
