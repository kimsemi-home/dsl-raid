(in-package #:dslraid.lang)

(defun validate-fsm-ast (ast)
  "Return language conformance diagnostics before Canonical IR expansion."
  (append
   (duplicate-form-diagnostics ast :state "state" "LANG001")
   (duplicate-form-diagnostics ast :event "event" "LANG002")
   (duplicate-form-diagnostics ast :transition "transition" "LANG003")
   (unknown-form-diagnostics ast)))
