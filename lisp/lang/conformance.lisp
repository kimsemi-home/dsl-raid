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
   (unknown-form-diagnostics ast)))
