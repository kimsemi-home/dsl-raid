(in-package #:dslraid.lang)

(defparameter *authoring-diagnostic-codes*
  '((:key :duplicate-state
     :code "LANG001"
     :severity "error"
     :scope "state authoring forms"
     :summary "Duplicate state form.")
    (:key :duplicate-event
     :code "LANG002"
     :severity "error"
     :scope "event authoring forms"
     :summary "Duplicate event form.")
    (:key :duplicate-transition
     :code "LANG003"
     :severity "error"
     :scope "transition authoring forms"
     :summary "Duplicate transition form.")
    (:key :duplicate-guard
     :code "LANG023"
     :severity "error"
     :scope "guard authoring forms"
     :summary "Duplicate guard form.")
    (:key :duplicate-action
     :code "LANG024"
     :severity "error"
     :scope "action authoring forms"
     :summary "Duplicate action form.")
    (:key :unknown-form
     :code "LANG004"
     :severity "error"
     :scope "FSM authoring forms"
     :summary "Unknown FSM authoring form.")
    (:key :malformed-form
     :code "LANG013"
     :severity "error"
     :scope "FSM authoring forms"
     :summary "FSM authoring form must be a list.")))
